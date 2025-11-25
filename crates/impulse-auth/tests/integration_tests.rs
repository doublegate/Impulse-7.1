//! Integration tests for complete authentication flows
//!
//! These tests verify that all authentication components work together
//! correctly in real-world scenarios including login, registration,
//! logout, rate limiting, and account lockout.

use impulse_auth::{
    AuthService,
    flows::{
        login::{LoginFlow, LoginFlowResult},
        logout::{LogoutFlow, LogoutResult},
        register::{RegistrationFlow, RegistrationRequest, RegistrationResult},
    },
    lockout::AccountLockout,
    rate_limit::RateLimiter,
    validation::PasswordStrength,
};
use impulse_types::user::User;
use std::sync::Arc;
use std::time::Duration;

#[tokio::test]
async fn test_complete_login_flow() {
    // Setup auth service
    let auth = Arc::new(AuthService::new(Duration::from_secs(1800)));
    let login_flow = LoginFlow::new(Arc::clone(&auth));

    // Login with valid credentials
    let result = login_flow.execute("testuser", "ValidPassword123!").await;

    match result {
        LoginFlowResult::Success {
            user,
            session_token,
        } => {
            assert_eq!(user.username(), "testuser");
            assert!(!session_token.is_empty());
            // Session token string should be hex (64 characters)
            assert!(session_token.len() >= 32);
        }
        other => panic!("Expected login success, got {:?}", other),
    }
}

#[tokio::test]
async fn test_failed_login_rate_limiting() {
    // Setup auth service with rate limiting
    let rate_limiter = RateLimiter::new(3, Duration::from_secs(60));

    let lockout = AccountLockout::new_default();

    let auth = Arc::new(AuthService::new_with_protection(
        Duration::from_secs(1800),
        rate_limiter,
        lockout,
    ));

    // Create a user with known password
    let user = User::new("testuser").unwrap();
    let correct_password = "CorrectPassword123!";
    let hash = auth.hash_password(correct_password).unwrap();

    // First attempt with wrong password
    let result1 = auth.login(&user, "WrongPassword1", &hash).await;
    assert!(result1.is_err());

    // Second attempt with wrong password
    let result2 = auth.login(&user, "WrongPassword2", &hash).await;
    assert!(result2.is_err());

    // Third attempt with wrong password
    let result3 = auth.login(&user, "WrongPassword3", &hash).await;
    assert!(result3.is_err());

    // Fourth attempt should be rate limited
    let result4 = auth.login(&user, "WrongPassword4", &hash).await;
    match result4 {
        Err(e) => {
            let msg = e.to_string();
            assert!(
                msg.contains("Rate limit") || msg.contains("too many"),
                "Expected rate limit error, got: {}",
                msg
            );
        }
        Ok(_) => panic!("Expected rate limit error, but login succeeded"),
    }
}

#[tokio::test]
async fn test_account_lockout_flow() {
    // Setup auth service with lockout
    let rate_limiter = RateLimiter::new_default();
    let lockout = AccountLockout::new(3, Duration::from_secs(5));

    let auth = Arc::new(AuthService::new_with_protection(
        Duration::from_secs(1800),
        rate_limiter,
        lockout,
    ));

    let user = User::new("testuser").unwrap();
    let correct_password = "CorrectPassword123!";
    let hash = auth.hash_password(correct_password).unwrap();

    // Fail login multiple times
    for i in 0..3 {
        let result = auth
            .login(&user, &format!("WrongPassword{}", i), &hash)
            .await;
        assert!(result.is_err(), "Attempt {} should fail", i);
    }

    // Account should now be locked
    let result = auth.login(&user, correct_password, &hash).await;
    match result {
        Err(e) => {
            let msg = e.to_string();
            assert!(
                msg.contains("locked") || msg.contains("too many"),
                "Expected lockout error, got: {}",
                msg
            );
        }
        Ok(_) => panic!("Expected lockout error, but login succeeded"),
    }

    // Wait for lockout to expire
    tokio::time::sleep(Duration::from_secs(6)).await;

    // Should be able to login now
    let result = auth.login(&user, correct_password, &hash).await;
    assert!(result.is_ok(), "Login should succeed after lockout expires");
}

#[tokio::test]
async fn test_registration_flow() {
    // Setup auth service
    let auth = Arc::new(AuthService::new(Duration::from_secs(1800)));
    let reg_flow = RegistrationFlow::new(Arc::clone(&auth));

    // Register with valid data
    let request = RegistrationRequest::new(
        "newuser".to_string(),
        "SecureP@ss123".to_string(),
        "SecureP@ss123".to_string(),
    )
    .with_email("user@example.com".to_string());

    let result = reg_flow.execute(&request).await;
    match result {
        RegistrationResult::Success { user } => {
            assert_eq!(user.username(), "newuser");
        }
        other => panic!("Expected registration success, got {:?}", other),
    }

    // Try to register same username (would fail in real system with storage)
    // For now, this will succeed since we don't have persistent storage
    let request2 = RegistrationRequest::new(
        "newuser".to_string(),
        "AnotherP@ss456".to_string(),
        "AnotherP@ss456".to_string(),
    );

    let result2 = reg_flow.execute(&request2).await;
    // In production with storage, this would be UsernameExists
    match result2 {
        RegistrationResult::Success { .. } => {
            // Expected for now (no storage)
        }
        RegistrationResult::UsernameExists => {
            // Expected in production
        }
        other => panic!("Unexpected result: {:?}", other),
    }
}

#[tokio::test]
async fn test_registration_weak_password() {
    // Setup auth service
    let auth = Arc::new(AuthService::new(Duration::from_secs(1800)));
    let reg_flow =
        RegistrationFlow::new(Arc::clone(&auth)).with_min_password_strength(PasswordStrength::Fair);

    // Register with weak password
    let request = RegistrationRequest::new(
        "testuser".to_string(),
        "weak".to_string(),
        "weak".to_string(),
    );

    let result = reg_flow.execute(&request).await;
    match result {
        RegistrationResult::PasswordTooWeak { strength, .. } => {
            assert!(strength < PasswordStrength::Fair);
        }
        other => panic!("Expected PasswordTooWeak, got {:?}", other),
    }
}

#[tokio::test]
async fn test_session_timeout() {
    // Setup auth service with short timeout
    let auth = Arc::new(AuthService::new(Duration::from_millis(100)));
    let user = User::new("testuser").unwrap();
    let password = "TestPassword123!";
    let hash = auth.hash_password(password).unwrap();

    // Login and get session
    let token = auth.login(&user, password, &hash).await.unwrap();

    // Verify session valid
    assert!(auth.validate_session(&token).await.is_ok());

    // Wait for timeout
    tokio::time::sleep(Duration::from_millis(150)).await;

    // Verify session expired
    assert!(auth.validate_session(&token).await.is_err());
}

#[tokio::test]
async fn test_session_refresh() {
    // Setup auth service
    let auth = Arc::new(AuthService::new(Duration::from_secs(1800)));
    let user = User::new("testuser").unwrap();
    let password = "TestPassword123!";
    let hash = auth.hash_password(password).unwrap();

    // Login and get session
    let token = auth.login(&user, password, &hash).await.unwrap();

    // Get initial session info
    let sessions = auth.get_user_sessions(user.id()).await;
    assert_eq!(sessions.len(), 1);
    let initial_activity = sessions[0].last_activity();

    // Wait a bit
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Refresh session
    assert!(auth.validate_session(&token).await.is_ok());

    // Verify last_activity updated
    let sessions_after = auth.get_user_sessions(user.id()).await;
    assert_eq!(sessions_after.len(), 1);
    assert!(sessions_after[0].last_activity() > initial_activity);
}

#[tokio::test]
async fn test_logout_flow() {
    // Setup auth service
    let auth = Arc::new(AuthService::new(Duration::from_secs(1800)));
    let logout_flow = LogoutFlow::new(Arc::clone(&auth));

    let user = User::new("testuser").unwrap();
    let password = "TestPassword123!";
    let hash = auth.hash_password(password).unwrap();

    // Login and get session
    let token = auth.login(&user, password, &hash).await.unwrap();

    // Verify session valid
    assert!(auth.validate_session(&token).await.is_ok());

    // Logout
    let result = logout_flow.execute(token.as_str()).await;
    assert_eq!(result, LogoutResult::Success);

    // Verify session invalid
    assert!(auth.validate_session(&token).await.is_err());
}

#[tokio::test]
async fn test_logout_invalid_session() {
    // Setup auth service
    let auth = Arc::new(AuthService::new(Duration::from_secs(1800)));
    let logout_flow = LogoutFlow::new(Arc::clone(&auth));

    // Try to logout non-existent session
    let result = logout_flow.execute("invalid_token_12345").await;
    assert_eq!(result, LogoutResult::InvalidSession);
}

#[tokio::test]
async fn test_multiple_sessions_per_user() {
    // Setup auth service
    let auth = Arc::new(AuthService::new(Duration::from_secs(1800)));
    let user = User::new("testuser").unwrap();
    let password = "TestPassword123!";
    let hash = auth.hash_password(password).unwrap();

    // Create multiple sessions
    let token1 = auth.login(&user, password, &hash).await.unwrap();
    let token2 = auth.login(&user, password, &hash).await.unwrap();
    let token3 = auth.login(&user, password, &hash).await.unwrap();

    // Verify all sessions are valid
    assert!(auth.validate_session(&token1).await.is_ok());
    assert!(auth.validate_session(&token2).await.is_ok());
    assert!(auth.validate_session(&token3).await.is_ok());

    // Verify session count
    let sessions = auth.get_user_sessions(user.id()).await;
    assert_eq!(sessions.len(), 3);

    // Logout one session
    assert!(auth.logout(&token1).await);

    // Verify only that session is invalid
    assert!(auth.validate_session(&token1).await.is_err());
    assert!(auth.validate_session(&token2).await.is_ok());
    assert!(auth.validate_session(&token3).await.is_ok());

    // Verify session count
    let sessions = auth.get_user_sessions(user.id()).await;
    assert_eq!(sessions.len(), 2);
}

#[tokio::test]
async fn test_logout_all_sessions() {
    // Setup auth service
    let auth = Arc::new(AuthService::new(Duration::from_secs(1800)));
    let logout_flow = LogoutFlow::new(Arc::clone(&auth));

    let user = User::new("testuser").unwrap();
    let password = "TestPassword123!";
    let hash = auth.hash_password(password).unwrap();

    // Create multiple sessions
    let token1 = auth.login(&user, password, &hash).await.unwrap();
    let token2 = auth.login(&user, password, &hash).await.unwrap();
    let token3 = auth.login(&user, password, &hash).await.unwrap();

    // Verify all sessions are valid
    assert!(auth.validate_session(&token1).await.is_ok());
    assert!(auth.validate_session(&token2).await.is_ok());
    assert!(auth.validate_session(&token3).await.is_ok());

    // Logout all sessions
    let count = logout_flow.logout_all_sessions(&user.id()).await;
    assert_eq!(count, 3);

    // Verify all sessions are invalid
    assert!(auth.validate_session(&token1).await.is_err());
    assert!(auth.validate_session(&token2).await.is_err());
    assert!(auth.validate_session(&token3).await.is_err());
}

#[tokio::test]
async fn test_complete_user_lifecycle() {
    // Setup auth service
    let auth = Arc::new(AuthService::new(Duration::from_secs(1800)));
    let reg_flow = RegistrationFlow::new(Arc::clone(&auth));
    let login_flow = LoginFlow::new(Arc::clone(&auth));
    let logout_flow = LogoutFlow::new(Arc::clone(&auth));

    // 1. Register a new user
    let request = RegistrationRequest::new(
        "lifecycleuser".to_string(),
        "SecureP@ss123".to_string(),
        "SecureP@ss123".to_string(),
    )
    .with_email("lifecycle@example.com".to_string());

    let reg_result = reg_flow.execute(&request).await;
    match reg_result {
        RegistrationResult::Success { user } => {
            assert_eq!(user.username(), "lifecycleuser");
        }
        other => panic!("Expected registration success, got {:?}", other),
    }

    // 2. Login with the new account
    let login_result = login_flow.execute("lifecycleuser", "SecureP@ss123").await;
    let session_token = match login_result {
        LoginFlowResult::Success { session_token, .. } => session_token,
        other => panic!("Expected login success, got {:?}", other),
    };

    // 3. Verify session is active
    assert!(logout_flow.is_session_valid(&session_token).await);

    // 4. Logout
    let logout_result = logout_flow.execute(&session_token).await;
    assert_eq!(logout_result, LogoutResult::Success);

    // 5. Verify session is no longer active
    assert!(!logout_flow.is_session_valid(&session_token).await);
}
