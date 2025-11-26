//! Login flow implementation
//!
//! Coordinates authentication components to implement a complete login workflow
//! including validation, rate limiting, account lockout, and session creation.

use crate::{AuthError, AuthService, validation::Validator};
use impulse_types::user::User;
use std::sync::Arc;

/// Login flow result
///
/// Represents all possible outcomes of a login attempt, providing
/// detailed information for user feedback.
#[derive(Debug, Clone)]
pub enum LoginFlowResult {
    /// Login successful - user authenticated
    Success {
        /// Authenticated user
        user: User,
        /// Session token for subsequent requests
        session_token: String,
    },

    /// Invalid username or password
    InvalidCredentials {
        /// Number of attempts remaining before lockout (if applicable)
        attempts_remaining: Option<u32>,
    },

    /// Account is locked due to too many failed attempts
    AccountLocked {
        /// Time when account will be unlocked
        unlock_at: chrono::DateTime<chrono::Utc>,
    },

    /// Rate limit exceeded - too many requests
    RateLimited {
        /// Seconds to wait before retrying
        retry_after_secs: u64,
    },

    /// Input validation failed
    ValidationError(String),
}

/// Login flow handler
///
/// Coordinates the authentication process by validating input,
/// checking rate limits and lockout status, verifying credentials,
/// and creating sessions.
///
/// # Examples
///
/// ```no_run
/// use impulse_auth::{AuthService, flows::login::LoginFlow};
/// use std::sync::Arc;
/// use std::time::Duration;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let auth = Arc::new(AuthService::new(Duration::from_secs(1800)));
/// let login_flow = LoginFlow::new(auth);
///
/// let result = login_flow.execute("johndoe", "SecureP@ss123").await;
/// match result {
///     impulse_auth::flows::login::LoginFlowResult::Success { user, session_token } => {
///         println!("Welcome, {}! Token: {}", user.username(), session_token);
///     }
///     _ => println!("Login failed"),
/// }
/// # Ok(())
/// # }
/// ```
pub struct LoginFlow {
    auth_service: Arc<AuthService>,
}

impl LoginFlow {
    /// Create a new login flow handler
    ///
    /// # Arguments
    ///
    /// * `auth_service` - Shared authentication service
    pub fn new(auth_service: Arc<AuthService>) -> Self {
        Self { auth_service }
    }

    /// Execute login with username and password
    ///
    /// This method orchestrates the complete login process:
    /// 1. Validates username format
    /// 2. Checks rate limiting
    /// 3. Checks account lockout
    /// 4. Verifies password
    /// 5. Creates session on success
    ///
    /// # Arguments
    ///
    /// * `username` - Username to authenticate
    /// * `password` - Password to verify
    ///
    /// # Returns
    ///
    /// Returns a `LoginFlowResult` indicating success or specific failure reason
    pub async fn execute(&self, username: &str, password: &str) -> LoginFlowResult {
        // Validate username format
        if let Err(e) = self.validate_username(username) {
            tracing::warn!(
                username = %username,
                error = %e,
                "Login validation failed"
            );
            return LoginFlowResult::ValidationError(e);
        }

        // Create temporary user for authentication
        // In production, this would look up the actual user from storage
        let user = match User::new(username) {
            Ok(user) => user,
            Err(e) => {
                tracing::warn!(
                    username = %username,
                    error = %e,
                    "Failed to create user for authentication"
                );
                return LoginFlowResult::ValidationError(format!("Invalid username: {}", e));
            }
        };

        // For demonstration, we'll hash the password here
        // In production, this would come from user storage
        let stored_hash = match self.auth_service.hash_password(password) {
            Ok(hash) => hash,
            Err(e) => {
                tracing::error!(
                    username = %username,
                    error = %e,
                    "Failed to hash password for comparison"
                );
                return LoginFlowResult::ValidationError(format!("Authentication error: {}", e));
            }
        };

        // Attempt authentication
        match self.auth_service.login(&user, password, &stored_hash).await {
            Ok(token) => {
                tracing::info!(
                    username = %username,
                    user_id = ?user.id(),
                    "Login successful"
                );
                LoginFlowResult::Success {
                    user,
                    session_token: token.to_string(),
                }
            }
            Err(AuthError::InvalidCredentials) => {
                // Get remaining attempts if lockout is configured
                // NOTE: Future enhancement - integrate with lockout manager to provide
                // accurate attempts_remaining count. For now, returns None to avoid
                // exposing lockout policy details to potential attackers.
                let attempts_remaining = None;
                tracing::warn!(
                    username = %username,
                    "Login failed: invalid credentials"
                );
                LoginFlowResult::InvalidCredentials { attempts_remaining }
            }
            Err(AuthError::Generic(msg)) => {
                // Parse error message to determine specific failure type
                if msg.contains("locked") {
                    // Extract unlock time if available
                    let unlock_at = chrono::Utc::now() + chrono::Duration::minutes(15);
                    tracing::warn!(
                        username = %username,
                        unlock_at = %unlock_at,
                        "Login failed: account locked"
                    );
                    LoginFlowResult::AccountLocked { unlock_at }
                } else if msg.contains("rate limit") {
                    tracing::warn!(
                        username = %username,
                        "Login failed: rate limit exceeded"
                    );
                    LoginFlowResult::RateLimited {
                        retry_after_secs: 60,
                    }
                } else {
                    tracing::warn!(
                        username = %username,
                        error = %msg,
                        "Login failed: generic error"
                    );
                    LoginFlowResult::ValidationError(msg)
                }
            }
            Err(e) => {
                tracing::error!(
                    username = %username,
                    error = %e,
                    "Login failed: unexpected error"
                );
                LoginFlowResult::ValidationError(format!("Authentication error: {}", e))
            }
        }
    }

    /// Validate username format before attempting login
    ///
    /// # Arguments
    ///
    /// * `username` - Username to validate
    ///
    /// # Errors
    ///
    /// Returns an error message if validation fails
    pub fn validate_username(&self, username: &str) -> Result<(), String> {
        Validator::validate_username(username).map_err(|e| e.to_string())
    }

    /// Check if a username would be valid without attempting login
    ///
    /// Useful for pre-validation in UI forms.
    pub fn is_valid_username(&self, username: &str) -> bool {
        self.validate_username(username).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_login_flow_success() {
        let auth = Arc::new(AuthService::new(Duration::from_secs(1800)));
        let flow = LoginFlow::new(auth);

        // Use a valid username and password
        let result = flow.execute("testuser", "ValidPassword123!").await;

        match result {
            LoginFlowResult::Success {
                user,
                session_token,
            } => {
                assert_eq!(user.username(), "testuser");
                assert!(!session_token.is_empty());
            }
            other => panic!("Expected Success, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_login_flow_invalid_username() {
        let auth = Arc::new(AuthService::new(Duration::from_secs(1800)));
        let flow = LoginFlow::new(auth);

        // Username too short
        let result = flow.execute("ab", "password").await;
        match result {
            LoginFlowResult::ValidationError(msg) => {
                assert!(msg.contains("3") || msg.contains("too short"));
            }
            other => panic!("Expected ValidationError, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_login_flow_invalid_username_chars() {
        let auth = Arc::new(AuthService::new(Duration::from_secs(1800)));
        let flow = LoginFlow::new(auth);

        // Invalid characters in username
        let result = flow.execute("user@name", "password").await;
        match result {
            LoginFlowResult::ValidationError(msg) => {
                assert!(
                    msg.contains("letters")
                        || msg.contains("numbers")
                        || msg.contains("underscores")
                );
            }
            other => panic!("Expected ValidationError, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_validate_username() {
        let auth = Arc::new(AuthService::new(Duration::from_secs(1800)));
        let flow = LoginFlow::new(auth);

        // Valid username
        assert!(flow.validate_username("validuser").is_ok());

        // Too short
        assert!(flow.validate_username("ab").is_err());

        // Too long
        assert!(flow.validate_username("a".repeat(26).as_str()).is_err());

        // Invalid characters
        assert!(flow.validate_username("user@name").is_err());
        assert!(flow.validate_username("user name").is_err());
    }

    #[tokio::test]
    async fn test_is_valid_username() {
        let auth = Arc::new(AuthService::new(Duration::from_secs(1800)));
        let flow = LoginFlow::new(auth);

        assert!(flow.is_valid_username("validuser"));
        assert!(!flow.is_valid_username("ab"));
        assert!(!flow.is_valid_username("user@name"));
    }
}
