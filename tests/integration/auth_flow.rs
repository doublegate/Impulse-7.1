//! Integration tests for authentication workflows
//!
//! This module tests the complete authentication flow including:
//! - User registration (mocked)
//! - Login with valid credentials
//! - Login with invalid credentials
//! - Session validation
//! - Session expiration
//! - Password change flow
//! - Logout

use impulse_auth::{
    flows::AuthService,
    lockout::AccountLockout,
    rate_limit::RateLimiter,
    validation,
    PasswordHasher, SessionToken,
};
use impulse_types::user::User;
use std::time::Duration;

/// Setup a test authentication service with default configuration
fn setup_auth_service() -> AuthService {
    AuthService::new_with_protection(
        Duration::from_secs(1800), // 30 min session timeout
        RateLimiter::new_default(),
        AccountLockout::new_default(),
    )
}

#[tokio::test]
async fn test_complete_login_flow() {
    // Setup
    let mut auth = setup_auth_service();
    let hasher = PasswordHasher::new();

    // Create test user credentials
    let username = "testuser";
    let password = "SecureP@ss123";
    let password_hash = hasher.hash_password(password).expect("Failed to hash password");

    // Attempt login with valid credentials
    let result = auth
        .authenticate(username, password, &password_hash)
        .await;

    assert!(result.is_ok(), "Login should succeed with valid credentials");
    let token = result.unwrap();

    // Verify session is valid
    let session_check = auth.validate_session(&token).await;
    assert!(
        session_check.is_ok(),
        "Session should be valid immediately after login"
    );

    // Logout
    let logout_result = auth.logout(&token).await;
    assert!(logout_result.is_ok(), "Logout should succeed");

    // Verify session is no longer valid
    let session_check_after = auth.validate_session(&token).await;
    assert!(
        session_check_after.is_err(),
        "Session should be invalid after logout"
    );
}

#[tokio::test]
async fn test_login_with_invalid_credentials() {
    // Setup
    let mut auth = setup_auth_service();
    let hasher = PasswordHasher::new();

    let username = "testuser";
    let correct_password = "SecureP@ss123";
    let wrong_password = "WrongPassword";
    let password_hash = hasher
        .hash_password(correct_password)
        .expect("Failed to hash password");

    // Attempt login with wrong password
    let result = auth
        .authenticate(username, wrong_password, &password_hash)
        .await;

    assert!(
        result.is_err(),
        "Login should fail with invalid credentials"
    );
}

#[tokio::test]
async fn test_session_validation() {
    // Setup
    let mut auth = setup_auth_service();
    let hasher = PasswordHasher::new();

    let username = "testuser";
    let password = "SecureP@ss123";
    let password_hash = hasher.hash_password(password).expect("Failed to hash password");

    // Login
    let token = auth
        .authenticate(username, password, &password_hash)
        .await
        .expect("Login should succeed");

    // Validate session multiple times
    for _ in 0..5 {
        let result = auth.validate_session(&token).await;
        assert!(result.is_ok(), "Session should remain valid");
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

#[tokio::test]
async fn test_session_expiration() {
    // Setup with very short timeout for testing
    let mut auth = AuthService::new_with_protection(
        Duration::from_millis(500), // 500ms timeout
        RateLimiter::new_default(),
        AccountLockout::new_default(),
    );
    let hasher = PasswordHasher::new();

    let username = "testuser";
    let password = "SecureP@ss123";
    let password_hash = hasher.hash_password(password).expect("Failed to hash password");

    // Login
    let token = auth
        .authenticate(username, password, &password_hash)
        .await
        .expect("Login should succeed");

    // Session should be valid immediately
    assert!(auth.validate_session(&token).await.is_ok());

    // Wait for expiration
    tokio::time::sleep(Duration::from_millis(600)).await;

    // Session should now be expired
    let result = auth.validate_session(&token).await;
    assert!(
        result.is_err(),
        "Session should be expired after timeout"
    );
}

#[tokio::test]
async fn test_password_change_flow() {
    // Setup
    let hasher = PasswordHasher::new();

    let old_password = "OldP@ss123";
    let new_password = "NewP@ss456";

    // Hash both passwords
    let old_hash = hasher
        .hash_password(old_password)
        .expect("Failed to hash old password");
    let new_hash = hasher
        .hash_password(new_password)
        .expect("Failed to hash new password");

    // Verify old password works with old hash
    assert!(
        hasher.verify_password(old_password, &old_hash).is_ok(),
        "Old password should verify against old hash"
    );

    // Verify old password doesn't work with new hash
    assert!(
        hasher.verify_password(old_password, &new_hash).is_err(),
        "Old password should not verify against new hash"
    );

    // Verify new password works with new hash
    assert!(
        hasher.verify_password(new_password, &new_hash).is_ok(),
        "New password should verify against new hash"
    );
}

#[tokio::test]
async fn test_username_validation() {
    // Valid usernames
    assert!(validation::validate_username("user123").is_ok());
    assert!(validation::validate_username("alice").is_ok());
    assert!(validation::validate_username("user_name").is_ok());

    // Invalid usernames
    assert!(validation::validate_username("").is_err(), "Empty username");
    assert!(
        validation::validate_username("ab").is_err(),
        "Too short (< 3 chars)"
    );
    assert!(
        validation::validate_username("a".repeat(21).as_str()).is_err(),
        "Too long (> 20 chars)"
    );
    assert!(
        validation::validate_username("user@name").is_err(),
        "Invalid character"
    );
    assert!(
        validation::validate_username("user name").is_err(),
        "Contains space"
    );
}

#[tokio::test]
async fn test_password_validation() {
    // Valid passwords
    assert!(validation::validate_password("SecureP@ss123").is_ok());
    assert!(validation::validate_password("MyP@ssw0rd!").is_ok());

    // Invalid passwords
    assert!(
        validation::validate_password("").is_err(),
        "Empty password"
    );
    assert!(
        validation::validate_password("short1!").is_err(),
        "Too short (< 8 chars)"
    );
    assert!(
        validation::validate_password("nodigits!").is_err(),
        "Missing digit"
    );
    assert!(
        validation::validate_password("NoSpecial1").is_err(),
        "Missing special char"
    );
    assert!(
        validation::validate_password("nouppercase1!").is_err(),
        "Missing uppercase"
    );
    assert!(
        validation::validate_password("NOLOWERCASE1!").is_err(),
        "Missing lowercase"
    );
}

#[tokio::test]
async fn test_email_validation() {
    // Valid emails
    assert!(validation::validate_email("user@example.com").is_ok());
    assert!(validation::validate_email("alice.smith@company.co.uk").is_ok());
    assert!(validation::validate_email("user+tag@domain.org").is_ok());

    // Invalid emails
    assert!(validation::validate_email("").is_err(), "Empty email");
    assert!(
        validation::validate_email("notanemail").is_err(),
        "Missing @"
    );
    assert!(
        validation::validate_email("@example.com").is_err(),
        "Missing local part"
    );
    assert!(
        validation::validate_email("user@").is_err(),
        "Missing domain"
    );
    assert!(
        validation::validate_email("user@domain").is_err(),
        "Missing TLD"
    );
}

#[tokio::test]
async fn test_rate_limiting() {
    // Setup
    let mut auth = setup_auth_service();
    let hasher = PasswordHasher::new();

    let username = "testuser";
    let password = "SecureP@ss123";
    let wrong_password = "WrongPassword";
    let password_hash = hasher.hash_password(password).expect("Failed to hash password");

    // Attempt multiple failed logins
    for i in 0..3 {
        let result = auth
            .authenticate(username, wrong_password, &password_hash)
            .await;
        assert!(
            result.is_err(),
            "Attempt {} should fail with wrong password",
            i + 1
        );
    }

    // Note: Rate limiting behavior depends on RateLimiter configuration
    // This test verifies the flow doesn't panic under repeated failures
}

#[tokio::test]
async fn test_concurrent_sessions() {
    // Setup
    let mut auth = setup_auth_service();
    let hasher = PasswordHasher::new();

    let username = "testuser";
    let password = "SecureP@ss123";
    let password_hash = hasher.hash_password(password).expect("Failed to hash password");

    // Create multiple sessions for the same user
    let mut tokens = Vec::new();
    for _ in 0..3 {
        let token = auth
            .authenticate(username, password, &password_hash)
            .await
            .expect("Login should succeed");
        tokens.push(token);
    }

    // All sessions should be valid
    for (i, token) in tokens.iter().enumerate() {
        let result = auth.validate_session(token).await;
        assert!(result.is_ok(), "Session {} should be valid", i + 1);
    }

    // Logout from one session shouldn't affect others
    auth.logout(&tokens[0])
        .await
        .expect("Logout should succeed");

    assert!(
        auth.validate_session(&tokens[0]).await.is_err(),
        "Logged out session should be invalid"
    );
    assert!(
        auth.validate_session(&tokens[1]).await.is_ok(),
        "Other sessions should remain valid"
    );
    assert!(
        auth.validate_session(&tokens[2]).await.is_ok(),
        "Other sessions should remain valid"
    );
}

#[tokio::test]
async fn test_invalid_session_token() {
    // Setup
    let mut auth = setup_auth_service();

    // Try to validate a random token that was never issued
    let fake_token = SessionToken::new();
    let result = auth.validate_session(&fake_token).await;

    assert!(
        result.is_err(),
        "Validation should fail for non-existent token"
    );
}
