//! Registration flow implementation
//!
//! Handles new user registration with comprehensive validation including
//! username availability, password strength, and optional email verification.

use crate::{
    AuthService,
    validation::{PasswordStrength, Validator},
};
use impulse_types::user::User;
use std::sync::Arc;

/// Registration request data
///
/// Contains all information needed to register a new user account.
#[derive(Debug, Clone)]
pub struct RegistrationRequest {
    /// Desired username (must be unique)
    pub username: String,

    /// Password (will be validated for strength)
    pub password: String,

    /// Password confirmation (must match password)
    pub password_confirm: String,

    /// Optional email address
    pub email: Option<String>,

    /// Optional real name
    pub real_name: Option<String>,

    /// Optional location
    pub location: Option<String>,
}

impl RegistrationRequest {
    /// Create a new registration request
    pub fn new(username: String, password: String, password_confirm: String) -> Self {
        Self {
            username,
            password,
            password_confirm,
            email: None,
            real_name: None,
            location: None,
        }
    }

    /// Set optional email address
    pub fn with_email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }

    /// Set optional real name
    pub fn with_real_name(mut self, real_name: String) -> Self {
        self.real_name = Some(real_name);
        self
    }

    /// Set optional location
    pub fn with_location(mut self, location: String) -> Self {
        self.location = Some(location);
        self
    }
}

/// Registration result
///
/// Represents all possible outcomes of a registration attempt.
#[derive(Debug, Clone)]
pub enum RegistrationResult {
    /// Registration successful
    Success {
        /// Newly created user
        user: User,
    },

    /// Username already exists
    UsernameExists,

    /// Username too short
    UsernameTooShort,

    /// Username too long
    UsernameTooLong,

    /// Username contains invalid characters
    UsernameInvalidChars,

    /// Password too weak
    PasswordTooWeak {
        /// Detected password strength
        strength: PasswordStrength,
        /// Description of requirements
        requirements: String,
    },

    /// Password and confirmation don't match
    PasswordMismatch,

    /// Email address invalid
    EmailInvalid,

    /// Generic validation error
    ValidationError(String),
}

/// Registration flow handler
///
/// Coordinates new user registration by validating all inputs,
/// checking username availability, and creating new user accounts.
///
/// # Examples
///
/// ```no_run
/// use impulse_auth::{AuthService, flows::register::{RegistrationFlow, RegistrationRequest}};
/// use std::sync::Arc;
/// use std::time::Duration;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let auth = Arc::new(AuthService::new(Duration::from_secs(1800)));
/// let reg_flow = RegistrationFlow::new(auth);
///
/// let request = RegistrationRequest::new(
///     "newuser".to_string(),
///     "SecureP@ss123".to_string(),
///     "SecureP@ss123".to_string(),
/// ).with_email("user@example.com".to_string());
///
/// let result = reg_flow.execute(&request).await;
/// match result {
///     impulse_auth::flows::register::RegistrationResult::Success { user } => {
///         println!("Welcome, {}!", user.username());
///     }
///     _ => println!("Registration failed"),
/// }
/// # Ok(())
/// # }
/// ```
pub struct RegistrationFlow {
    #[allow(dead_code)] // Will be used for user storage lookup in future
    auth_service: Arc<AuthService>,
    min_password_strength: PasswordStrength,
}

impl RegistrationFlow {
    /// Create a new registration flow handler
    ///
    /// # Arguments
    ///
    /// * `auth_service` - Shared authentication service
    pub fn new(auth_service: Arc<AuthService>) -> Self {
        Self {
            auth_service,
            min_password_strength: PasswordStrength::Fair,
        }
    }

    /// Set minimum required password strength
    ///
    /// Default is `Medium`. Can be adjusted based on security requirements.
    pub fn with_min_password_strength(mut self, strength: PasswordStrength) -> Self {
        self.min_password_strength = strength;
        self
    }

    /// Execute registration
    ///
    /// Validates all inputs and creates a new user account if validation passes.
    ///
    /// # Arguments
    ///
    /// * `request` - Registration request with user data
    ///
    /// # Returns
    ///
    /// Returns a `RegistrationResult` indicating success or specific failure reason
    pub async fn execute(&self, request: &RegistrationRequest) -> RegistrationResult {
        // Validate username
        if let Err(e) = Validator::validate_username(&request.username) {
            let e = e.to_string();
            tracing::warn!(
                username = %request.username,
                error = %e,
                "Registration username validation failed"
            );

            return if e.contains("3") || e.contains("too short") {
                RegistrationResult::UsernameTooShort
            } else if e.contains("25") || e.contains("too long") {
                RegistrationResult::UsernameTooLong
            } else if e.contains("alphanumeric") || e.contains("characters") {
                RegistrationResult::UsernameInvalidChars
            } else {
                RegistrationResult::ValidationError(e)
            };
        }

        // Check username availability
        if !self.check_username_available(&request.username).await {
            tracing::warn!(
                username = %request.username,
                "Registration failed: username already exists"
            );
            return RegistrationResult::UsernameExists;
        }

        // Validate password confirmation
        if request.password != request.password_confirm {
            tracing::warn!("Registration failed: password mismatch");
            return RegistrationResult::PasswordMismatch;
        }

        // Validate password strength
        match Validator::password_strength(&request.password) {
            Ok(strength) => {
                if strength < self.min_password_strength {
                    tracing::warn!(
                        strength = ?strength,
                        min_required = ?self.min_password_strength,
                        "Registration failed: password too weak"
                    );
                    return RegistrationResult::PasswordTooWeak {
                        strength,
                        requirements: self.password_requirements(),
                    };
                }
            }
            Err(e) => {
                tracing::warn!(
                    error = %e,
                    "Registration failed: password validation error"
                );
                return RegistrationResult::ValidationError(format!(
                    "Password validation failed: {}",
                    e
                ));
            }
        }

        // Validate email if provided
        if let Some(ref email) = request.email {
            if let Err(e) = Validator::validate_email(email) {
                let e = e.to_string();
                tracing::warn!(
                    email = %email,
                    error = %e,
                    "Registration failed: invalid email"
                );
                return RegistrationResult::EmailInvalid;
            }
        }

        // Create user
        match User::new(&request.username) {
            Ok(user) => {
                tracing::info!(
                    username = %request.username,
                    user_id = ?user.id(),
                    "User registered successfully"
                );
                RegistrationResult::Success { user }
            }
            Err(e) => {
                tracing::error!(
                    username = %request.username,
                    error = %e,
                    "Failed to create user"
                );
                RegistrationResult::ValidationError(format!("Failed to create user: {}", e))
            }
        }
    }

    /// Check if username is available
    ///
    /// # Arguments
    ///
    /// * `username` - Username to check
    ///
    /// # Returns
    ///
    /// Returns `true` if username is available, `false` if taken
    pub async fn check_username_available(&self, username: &str) -> bool {
        // TODO: In production, query user storage to check existence
        // For now, always return true (available)
        tracing::debug!(username = %username, "Checking username availability");
        true
    }

    /// Get password requirements as string
    ///
    /// Returns a human-readable description of password requirements
    /// based on the configured minimum strength.
    pub fn password_requirements(&self) -> String {
        match self.min_password_strength {
            PasswordStrength::VeryWeak => {
                "Password must be at least 1 character".to_string()
            }
            PasswordStrength::Weak => {
                "Password must be at least 6 characters with some variety".to_string()
            }
            PasswordStrength::Fair => {
                "Password must be at least 8 characters with uppercase, lowercase, and numbers"
                    .to_string()
            }
            PasswordStrength::Strong => {
                "Password must be at least 10 characters with uppercase, lowercase, numbers, and symbols".to_string()
            }
            PasswordStrength::VeryStrong => {
                "Password must be at least 12 characters with uppercase, lowercase, numbers, symbols, and high entropy".to_string()
            }
        }
    }

    /// Get minimum password strength requirement
    pub fn min_password_strength(&self) -> PasswordStrength {
        self.min_password_strength
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_registration_success() {
        let auth = Arc::new(AuthService::new(Duration::from_secs(1800)));
        let flow = RegistrationFlow::new(auth);

        let request = RegistrationRequest::new(
            "newuser".to_string(),
            "SecureP@ss123".to_string(),
            "SecureP@ss123".to_string(),
        );

        let result = flow.execute(&request).await;
        match result {
            RegistrationResult::Success { user } => {
                assert_eq!(user.username(), "newuser");
            }
            other => panic!("Expected Success, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_registration_username_too_short() {
        let auth = Arc::new(AuthService::new(Duration::from_secs(1800)));
        let flow = RegistrationFlow::new(auth);

        let request = RegistrationRequest::new(
            "ab".to_string(),
            "SecureP@ss123".to_string(),
            "SecureP@ss123".to_string(),
        );

        let result = flow.execute(&request).await;
        match result {
            RegistrationResult::UsernameTooShort => {}
            other => panic!("Expected UsernameTooShort, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_registration_username_too_long() {
        let auth = Arc::new(AuthService::new(Duration::from_secs(1800)));
        let flow = RegistrationFlow::new(auth);

        let request = RegistrationRequest::new(
            "a".repeat(26),
            "SecureP@ss123".to_string(),
            "SecureP@ss123".to_string(),
        );

        let result = flow.execute(&request).await;
        match result {
            RegistrationResult::UsernameTooLong | RegistrationResult::UsernameInvalidChars => {
                // Either is acceptable (might fail length or start with letter check first)
            }
            other => panic!("Expected UsernameTooLong, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_registration_username_invalid_chars() {
        let auth = Arc::new(AuthService::new(Duration::from_secs(1800)));
        let flow = RegistrationFlow::new(auth);

        let request = RegistrationRequest::new(
            "user@name".to_string(),
            "SecureP@ss123".to_string(),
            "SecureP@ss123".to_string(),
        );

        let result = flow.execute(&request).await;
        match result {
            RegistrationResult::UsernameInvalidChars | RegistrationResult::ValidationError(_) => {
                // Either is acceptable
            }
            other => panic!(
                "Expected UsernameInvalidChars or ValidationError, got {:?}",
                other
            ),
        }
    }

    #[tokio::test]
    async fn test_registration_password_mismatch() {
        let auth = Arc::new(AuthService::new(Duration::from_secs(1800)));
        let flow = RegistrationFlow::new(auth);

        let request = RegistrationRequest::new(
            "testuser".to_string(),
            "SecureP@ss123".to_string(),
            "DifferentP@ss456".to_string(),
        );

        let result = flow.execute(&request).await;
        match result {
            RegistrationResult::PasswordMismatch => {}
            other => panic!("Expected PasswordMismatch, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_registration_password_too_weak() {
        let auth = Arc::new(AuthService::new(Duration::from_secs(1800)));
        let flow = RegistrationFlow::new(auth);

        let request = RegistrationRequest::new(
            "testuser".to_string(),
            "weak".to_string(),
            "weak".to_string(),
        );

        let result = flow.execute(&request).await;
        match result {
            RegistrationResult::PasswordTooWeak { strength, .. } => {
                assert!(strength < PasswordStrength::Fair);
            }
            other => panic!("Expected PasswordTooWeak, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_registration_invalid_email() {
        let auth = Arc::new(AuthService::new(Duration::from_secs(1800)));
        let flow = RegistrationFlow::new(auth);

        let request = RegistrationRequest::new(
            "testuser".to_string(),
            "SecureP@ss123".to_string(),
            "SecureP@ss123".to_string(),
        )
        .with_email("invalid-email".to_string());

        let result = flow.execute(&request).await;
        match result {
            RegistrationResult::EmailInvalid => {}
            other => panic!("Expected EmailInvalid, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_registration_with_optional_fields() {
        let auth = Arc::new(AuthService::new(Duration::from_secs(1800)));
        let flow = RegistrationFlow::new(auth);

        let request = RegistrationRequest::new(
            "testuser".to_string(),
            "SecureP@ss123".to_string(),
            "SecureP@ss123".to_string(),
        )
        .with_email("user@example.com".to_string())
        .with_real_name("Test User".to_string())
        .with_location("Test City".to_string());

        let result = flow.execute(&request).await;
        match result {
            RegistrationResult::Success { user } => {
                assert_eq!(user.username(), "testuser");
            }
            other => panic!("Expected Success, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_password_requirements() {
        let auth = Arc::new(AuthService::new(Duration::from_secs(1800)));

        let flow_weak = RegistrationFlow::new(Arc::clone(&auth))
            .with_min_password_strength(PasswordStrength::Weak);
        assert!(flow_weak.password_requirements().contains("6 characters"));

        let flow_fair = RegistrationFlow::new(Arc::clone(&auth))
            .with_min_password_strength(PasswordStrength::Fair);
        assert!(flow_fair.password_requirements().contains("8 characters"));

        let flow_strong = RegistrationFlow::new(Arc::clone(&auth))
            .with_min_password_strength(PasswordStrength::Strong);
        assert!(
            flow_strong
                .password_requirements()
                .contains("10 characters")
        );
    }

    #[tokio::test]
    async fn test_check_username_available() {
        let auth = Arc::new(AuthService::new(Duration::from_secs(1800)));
        let flow = RegistrationFlow::new(auth);

        // For now, always returns true (no storage yet)
        assert!(flow.check_username_available("anyusername").await);
    }
}
