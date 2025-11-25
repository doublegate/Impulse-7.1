//! Input validation for authentication
//!
//! This module provides validation utilities for usernames, passwords, and other
//! authentication inputs to ensure security and data integrity.
//!
//! # Examples
//!
//! ```
//! use impulse_auth::validation::Validator;
//!
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Validate username
//! Validator::validate_username("johndoe")?;
//!
//! // Check password strength
//! let strength = Validator::password_strength("MySecureP@ssw0rd")?;
//! println!("Password strength: {:?}", strength);
//!
//! // Validate email
//! Validator::validate_email("user@example.com")?;
//! # Ok(())
//! # }
//! ```

use thiserror::Error;

/// Validation errors
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ValidationError {
    /// Username validation failed
    #[error("Invalid username: {0}")]
    InvalidUsername(String),

    /// Password validation failed
    #[error("Invalid password: {0}")]
    InvalidPassword(String),

    /// Email validation failed
    #[error("Invalid email: {0}")]
    InvalidEmail(String),

    /// Generic validation error
    #[error("Validation error: {0}")]
    Generic(String),
}

/// Password strength levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PasswordStrength {
    /// Very weak password (fails minimum requirements)
    VeryWeak,
    /// Weak password (meets minimum but lacks complexity)
    Weak,
    /// Fair password (adequate complexity)
    Fair,
    /// Strong password (good complexity)
    Strong,
    /// Very strong password (excellent complexity)
    VeryStrong,
}

impl PasswordStrength {
    /// Check if password strength is acceptable
    pub fn is_acceptable(&self) -> bool {
        *self >= PasswordStrength::Fair
    }

    /// Get a user-friendly description
    pub fn description(&self) -> &'static str {
        match self {
            Self::VeryWeak => "Very weak - does not meet requirements",
            Self::Weak => "Weak - meets minimum but lacks complexity",
            Self::Fair => "Fair - adequate complexity",
            Self::Strong => "Strong - good complexity",
            Self::VeryStrong => "Very strong - excellent complexity",
        }
    }
}

/// Password validation configuration
#[derive(Debug, Clone)]
pub struct PasswordConfig {
    /// Minimum password length
    pub min_length: usize,
    /// Maximum password length
    pub max_length: usize,
    /// Require at least one uppercase letter
    pub require_uppercase: bool,
    /// Require at least one lowercase letter
    pub require_lowercase: bool,
    /// Require at least one digit
    pub require_digit: bool,
    /// Require at least one special character
    pub require_special: bool,
    /// Check against common passwords
    pub check_common: bool,
}

impl Default for PasswordConfig {
    fn default() -> Self {
        Self {
            min_length: 8,
            max_length: 128,
            require_uppercase: true,
            require_lowercase: true,
            require_digit: true,
            require_special: true,
            check_common: true,
        }
    }
}

impl PasswordConfig {
    /// Create a strict configuration
    ///
    /// 12+ characters, all complexity requirements
    pub fn strict() -> Self {
        Self {
            min_length: 12,
            max_length: 128,
            require_uppercase: true,
            require_lowercase: true,
            require_digit: true,
            require_special: true,
            check_common: true,
        }
    }

    /// Create a lenient configuration
    ///
    /// 6+ characters, no complexity requirements
    pub fn lenient() -> Self {
        Self {
            min_length: 6,
            max_length: 128,
            require_uppercase: false,
            require_lowercase: false,
            require_digit: false,
            require_special: false,
            check_common: false,
        }
    }
}

/// Validator for authentication inputs
pub struct Validator;

impl Validator {
    /// Validate a username
    ///
    /// Rules:
    /// - 3-20 characters
    /// - Alphanumeric characters and underscore only
    /// - Must start with a letter
    /// - Case-insensitive (will be stored as lowercase)
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::InvalidUsername` if validation fails
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_auth::validation::Validator;
    ///
    /// // Valid usernames
    /// assert!(Validator::validate_username("johndoe").is_ok());
    /// assert!(Validator::validate_username("user_123").is_ok());
    ///
    /// // Invalid usernames
    /// assert!(Validator::validate_username("ab").is_err()); // Too short
    /// assert!(Validator::validate_username("123user").is_err()); // Starts with digit
    /// assert!(Validator::validate_username("user@name").is_err()); // Invalid char
    /// ```
    pub fn validate_username(username: &str) -> Result<(), ValidationError> {
        // Length check
        if username.len() < 3 {
            return Err(ValidationError::InvalidUsername(
                "Username must be at least 3 characters".to_string(),
            ));
        }

        if username.len() > 20 {
            return Err(ValidationError::InvalidUsername(
                "Username must be at most 20 characters".to_string(),
            ));
        }

        // Must start with a letter
        if !username.chars().next().unwrap().is_ascii_alphabetic() {
            return Err(ValidationError::InvalidUsername(
                "Username must start with a letter".to_string(),
            ));
        }

        // Only alphanumeric and underscore
        if !username
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_')
        {
            return Err(ValidationError::InvalidUsername(
                "Username can only contain letters, numbers, and underscores".to_string(),
            ));
        }

        Ok(())
    }

    /// Validate password with default configuration
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::InvalidPassword` if validation fails
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_auth::validation::Validator;
    ///
    /// // Valid password
    /// assert!(Validator::validate_password("MyP@ssw0rd").is_ok());
    ///
    /// // Invalid password
    /// assert!(Validator::validate_password("weak").is_err());
    /// ```
    pub fn validate_password(password: &str) -> Result<(), ValidationError> {
        Self::validate_password_with_config(password, &PasswordConfig::default())
    }

    /// Validate password with custom configuration
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::InvalidPassword` if validation fails
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_auth::validation::{Validator, PasswordConfig};
    ///
    /// let config = PasswordConfig::strict();
    /// assert!(Validator::validate_password_with_config("MyVerySecureP@ssw0rd!", &config).is_ok());
    /// ```
    pub fn validate_password_with_config(
        password: &str,
        config: &PasswordConfig,
    ) -> Result<(), ValidationError> {
        // Length check
        if password.len() < config.min_length {
            return Err(ValidationError::InvalidPassword(format!(
                "Password must be at least {} characters",
                config.min_length
            )));
        }

        if password.len() > config.max_length {
            return Err(ValidationError::InvalidPassword(format!(
                "Password must be at most {} characters",
                config.max_length
            )));
        }

        // Complexity checks
        if config.require_uppercase && !password.chars().any(|c| c.is_ascii_uppercase()) {
            return Err(ValidationError::InvalidPassword(
                "Password must contain at least one uppercase letter".to_string(),
            ));
        }

        if config.require_lowercase && !password.chars().any(|c| c.is_ascii_lowercase()) {
            return Err(ValidationError::InvalidPassword(
                "Password must contain at least one lowercase letter".to_string(),
            ));
        }

        if config.require_digit && !password.chars().any(|c| c.is_ascii_digit()) {
            return Err(ValidationError::InvalidPassword(
                "Password must contain at least one digit".to_string(),
            ));
        }

        if config.require_special && !password.chars().any(|c| !c.is_alphanumeric()) {
            return Err(ValidationError::InvalidPassword(
                "Password must contain at least one special character".to_string(),
            ));
        }

        // Check common passwords
        if config.check_common && Self::is_common_password(password) {
            return Err(ValidationError::InvalidPassword(
                "This password is too common, please choose a different one".to_string(),
            ));
        }

        Ok(())
    }

    /// Calculate password strength
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::InvalidPassword` if password fails basic validation
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_auth::validation::{Validator, PasswordStrength};
    ///
    /// let strength = Validator::password_strength("MyP@ssw0rd123").unwrap();
    /// assert!(strength.is_acceptable());
    /// ```
    pub fn password_strength(password: &str) -> Result<PasswordStrength, ValidationError> {
        // Basic validation first
        if password.len() < 8 {
            return Ok(PasswordStrength::VeryWeak);
        }

        let mut score: i32 = 0;

        // Length scoring
        if password.len() >= 8 {
            score += 1;
        }
        if password.len() >= 12 {
            score += 1;
        }
        if password.len() >= 16 {
            score += 1;
        }

        // Complexity scoring
        if password.chars().any(|c| c.is_ascii_lowercase()) {
            score += 1;
        }
        if password.chars().any(|c| c.is_ascii_uppercase()) {
            score += 1;
        }
        if password.chars().any(|c| c.is_ascii_digit()) {
            score += 1;
        }
        if password.chars().any(|c| !c.is_alphanumeric()) {
            score += 1;
        }

        // Diversity penalty for repetitive patterns
        let unique_chars = password.chars().collect::<std::collections::HashSet<_>>();
        if unique_chars.len() < password.len() / 2 {
            score = score.saturating_sub(1);
        }

        // Common password penalty
        if Self::is_common_password(password) {
            score = score.saturating_sub(2);
        }

        let strength = match score {
            0..=2 => PasswordStrength::VeryWeak,
            3..=4 => PasswordStrength::Weak,
            5..=6 => PasswordStrength::Fair,
            7..=8 => PasswordStrength::Strong,
            _ => PasswordStrength::VeryStrong,
        };

        Ok(strength)
    }

    /// Check if password is in the common passwords list
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_auth::validation::Validator;
    ///
    /// assert!(Validator::is_common_password("password"));
    /// assert!(Validator::is_common_password("123456"));
    /// assert!(!Validator::is_common_password("MyP@ssw0rd123"));
    /// ```
    pub fn is_common_password(password: &str) -> bool {
        // Top 100 most common passwords (subset for demonstration)
        const COMMON_PASSWORDS: &[&str] = &[
            "password",
            "123456",
            "12345678",
            "qwerty",
            "abc123",
            "monkey",
            "1234567",
            "letmein",
            "trustno1",
            "dragon",
            "baseball",
            "111111",
            "iloveyou",
            "master",
            "sunshine",
            "ashley",
            "bailey",
            "passw0rd",
            "shadow",
            "123123",
            "654321",
            "superman",
            "qazwsx",
            "michael",
            "football",
            "password1",
            "admin",
            "welcome",
            "test",
            "guest",
            "root",
            "default",
        ];

        let password_lower = password.to_lowercase();
        COMMON_PASSWORDS
            .iter()
            .any(|&common| password_lower == common)
    }

    /// Validate email address format
    ///
    /// Simple validation (RFC 5322 compliant validation is very complex).
    /// Checks for basic format: local@domain.tld
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::InvalidEmail` if validation fails
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_auth::validation::Validator;
    ///
    /// // Valid emails
    /// assert!(Validator::validate_email("user@example.com").is_ok());
    /// assert!(Validator::validate_email("john.doe@example.co.uk").is_ok());
    ///
    /// // Invalid emails
    /// assert!(Validator::validate_email("invalid").is_err());
    /// assert!(Validator::validate_email("@example.com").is_err());
    /// ```
    pub fn validate_email(email: &str) -> Result<(), ValidationError> {
        // Basic email validation
        if email.is_empty() {
            return Err(ValidationError::InvalidEmail(
                "Email cannot be empty".to_string(),
            ));
        }

        // Must contain exactly one @
        let at_count = email.chars().filter(|&c| c == '@').count();
        if at_count != 1 {
            return Err(ValidationError::InvalidEmail(
                "Email must contain exactly one @ symbol".to_string(),
            ));
        }

        let parts: Vec<&str> = email.split('@').collect();
        let (local, domain) = (parts[0], parts[1]);

        // Local part validation
        if local.is_empty() {
            return Err(ValidationError::InvalidEmail(
                "Email local part cannot be empty".to_string(),
            ));
        }

        if local.len() > 64 {
            return Err(ValidationError::InvalidEmail(
                "Email local part is too long (max 64 characters)".to_string(),
            ));
        }

        // Domain part validation
        if domain.is_empty() {
            return Err(ValidationError::InvalidEmail(
                "Email domain cannot be empty".to_string(),
            ));
        }

        if !domain.contains('.') {
            return Err(ValidationError::InvalidEmail(
                "Email domain must contain at least one dot".to_string(),
            ));
        }

        // Domain must not start or end with dot
        if domain.starts_with('.') || domain.ends_with('.') {
            return Err(ValidationError::InvalidEmail(
                "Email domain cannot start or end with a dot".to_string(),
            ));
        }

        // Basic character validation
        let valid_local_chars = |c: char| c.is_alphanumeric() || "._-+".contains(c);
        if !local.chars().all(valid_local_chars) {
            return Err(ValidationError::InvalidEmail(
                "Email local part contains invalid characters".to_string(),
            ));
        }

        let valid_domain_chars = |c: char| c.is_alphanumeric() || ".-".contains(c);
        if !domain.chars().all(valid_domain_chars) {
            return Err(ValidationError::InvalidEmail(
                "Email domain contains invalid characters".to_string(),
            ));
        }

        Ok(())
    }

    /// Validate a password matches its confirmation
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::InvalidPassword` if passwords don't match
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_auth::validation::Validator;
    ///
    /// assert!(Validator::validate_password_match("password", "password").is_ok());
    /// assert!(Validator::validate_password_match("password", "different").is_err());
    /// ```
    pub fn validate_password_match(
        password: &str,
        confirmation: &str,
    ) -> Result<(), ValidationError> {
        if password != confirmation {
            return Err(ValidationError::InvalidPassword(
                "Passwords do not match".to_string(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Username validation tests
    #[test]
    fn test_valid_usernames() {
        assert!(Validator::validate_username("johndoe").is_ok());
        assert!(Validator::validate_username("user_123").is_ok());
        assert!(Validator::validate_username("Alice").is_ok());
        assert!(Validator::validate_username("test_user_99").is_ok());
    }

    #[test]
    fn test_invalid_usernames() {
        assert!(Validator::validate_username("ab").is_err()); // Too short
        assert!(Validator::validate_username("thisusernameiswaytoolong").is_err()); // Too long
        assert!(Validator::validate_username("123user").is_err()); // Starts with digit
        assert!(Validator::validate_username("user@name").is_err()); // Invalid char
        assert!(Validator::validate_username("user name").is_err()); // Space
        assert!(Validator::validate_username("user-name").is_err()); // Hyphen
    }

    // Password validation tests
    #[test]
    fn test_valid_passwords() {
        assert!(Validator::validate_password("MyP@ssw0rd").is_ok());
        assert!(Validator::validate_password("Str0ng!Pass").is_ok());
        assert!(Validator::validate_password("C0mpl3x#Pwd").is_ok());
    }

    #[test]
    fn test_invalid_passwords() {
        assert!(Validator::validate_password("short").is_err()); // Too short
        assert!(Validator::validate_password("nouppercase1!").is_err()); // No uppercase
        assert!(Validator::validate_password("NOLOWERCASE1!").is_err()); // No lowercase
        assert!(Validator::validate_password("NoDigits!").is_err()); // No digit
        assert!(Validator::validate_password("NoSpecial1").is_err()); // No special
        assert!(Validator::validate_password("password").is_err()); // Common password
    }

    #[test]
    fn test_password_config_strict() {
        let config = PasswordConfig::strict();
        assert!(Validator::validate_password_with_config("short", &config).is_err());
        assert!(Validator::validate_password_with_config("MyVeryS3cur3P@ss!", &config).is_ok());
    }

    #[test]
    fn test_password_config_lenient() {
        let config = PasswordConfig::lenient();
        assert!(Validator::validate_password_with_config("simple", &config).is_ok());
    }

    // Password strength tests
    #[test]
    fn test_password_strength() {
        // Very weak - too short
        assert_eq!(
            Validator::password_strength("weak").unwrap(),
            PasswordStrength::VeryWeak
        );

        // Weak - common password
        let strength = Validator::password_strength("password").unwrap();
        assert!(strength <= PasswordStrength::Weak);

        // Fair - decent complexity
        let strength = Validator::password_strength("S3cur3d!").unwrap();
        assert!(strength >= PasswordStrength::Weak && strength <= PasswordStrength::Fair);

        // Strong - good length and complexity
        let strength = Validator::password_strength("MySecur3!P@ss").unwrap();
        assert!(strength >= PasswordStrength::Fair);

        // Very strong - excellent complexity and length
        let strength = Validator::password_strength("MyV3ry!Str0ng#P@ssword2024").unwrap();
        assert!(strength >= PasswordStrength::Strong);
    }

    #[test]
    fn test_password_strength_acceptable() {
        assert!(!PasswordStrength::VeryWeak.is_acceptable());
        assert!(!PasswordStrength::Weak.is_acceptable());
        assert!(PasswordStrength::Fair.is_acceptable());
        assert!(PasswordStrength::Strong.is_acceptable());
        assert!(PasswordStrength::VeryStrong.is_acceptable());
    }

    // Common password tests
    #[test]
    fn test_common_passwords() {
        assert!(Validator::is_common_password("password"));
        assert!(Validator::is_common_password("123456"));
        assert!(Validator::is_common_password("qwerty"));
        assert!(!Validator::is_common_password("MyP@ssw0rd123"));
    }

    // Email validation tests
    #[test]
    fn test_valid_emails() {
        assert!(Validator::validate_email("user@example.com").is_ok());
        assert!(Validator::validate_email("john.doe@example.co.uk").is_ok());
        assert!(Validator::validate_email("alice+spam@test.org").is_ok());
        assert!(Validator::validate_email("user_123@domain.com").is_ok());
    }

    #[test]
    fn test_invalid_emails() {
        assert!(Validator::validate_email("").is_err()); // Empty
        assert!(Validator::validate_email("invalid").is_err()); // No @
        assert!(Validator::validate_email("@example.com").is_err()); // No local
        assert!(Validator::validate_email("user@").is_err()); // No domain
        assert!(Validator::validate_email("user@@example.com").is_err()); // Double @
        assert!(Validator::validate_email("user@domain").is_err()); // No TLD
        assert!(Validator::validate_email("user@.domain.com").is_err()); // Starts with dot
    }

    // Password match tests
    #[test]
    fn test_password_match() {
        assert!(Validator::validate_password_match("password", "password").is_ok());
        assert!(Validator::validate_password_match("password", "different").is_err());
    }
}
