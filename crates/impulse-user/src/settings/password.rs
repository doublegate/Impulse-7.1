//! Password validation and strength checking
//!
//! This module provides password validation logic and strength assessment.

use serde::{Deserialize, Serialize};

/// Password strength levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PasswordStrength {
    /// Weak password (< 6 chars)
    Weak,
    /// Fair password (6-7 chars, basic)
    Fair,
    /// Good password (8+ chars, mixed case)
    Good,
    /// Strong password (8+ chars, mixed case + numbers)
    Strong,
    /// Very strong password (12+ chars, mixed case + numbers + symbols)
    VeryStrong,
}

impl PasswordStrength {
    /// Get a description of the strength level
    #[must_use]
    pub fn description(&self) -> &'static str {
        match self {
            PasswordStrength::Weak => "Weak - Too short",
            PasswordStrength::Fair => "Fair - Add more characters",
            PasswordStrength::Good => "Good - Consider adding numbers",
            PasswordStrength::Strong => "Strong - Consider adding symbols",
            PasswordStrength::VeryStrong => "Very Strong",
        }
    }
}

/// Validate a password and return its strength
///
/// # Examples
///
/// ```
/// use impulse_user::settings::password::{validate_password, PasswordStrength};
///
/// assert_eq!(validate_password("test"), Ok(PasswordStrength::Weak));
/// assert_eq!(validate_password("TestPass123!"), Ok(PasswordStrength::VeryStrong));
/// assert!(validate_password("").is_err());
/// ```
pub fn validate_password(password: &str) -> Result<PasswordStrength, String> {
    if password.is_empty() {
        return Err("Password cannot be empty".to_string());
    }

    if password.len() < 4 {
        return Err("Password must be at least 4 characters".to_string());
    }

    let has_upper = password.chars().any(|c| c.is_uppercase());
    let has_lower = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_symbol = password.chars().any(|c| !c.is_alphanumeric());

    let strength = if password.len() >= 12 && has_upper && has_lower && has_digit && has_symbol {
        PasswordStrength::VeryStrong
    } else if password.len() >= 8 && has_upper && has_lower && has_digit {
        PasswordStrength::Strong
    } else if password.len() >= 8 && has_upper && has_lower {
        PasswordStrength::Good
    } else if password.len() >= 6 {
        PasswordStrength::Fair
    } else {
        PasswordStrength::Weak
    };

    Ok(strength)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_password_empty() {
        assert!(validate_password("").is_err());
    }

    #[test]
    fn test_validate_password_too_short() {
        assert!(validate_password("123").is_err());
    }

    #[test]
    fn test_validate_password_weak() {
        assert_eq!(validate_password("test"), Ok(PasswordStrength::Weak));
        assert_eq!(validate_password("12345"), Ok(PasswordStrength::Weak));
    }

    #[test]
    fn test_validate_password_fair() {
        assert_eq!(validate_password("test12"), Ok(PasswordStrength::Fair));
        assert_eq!(validate_password("testpwd"), Ok(PasswordStrength::Fair));
    }

    #[test]
    fn test_validate_password_good() {
        assert_eq!(validate_password("TestPass"), Ok(PasswordStrength::Good));
    }

    #[test]
    fn test_validate_password_strong() {
        assert_eq!(
            validate_password("TestPass123"),
            Ok(PasswordStrength::Strong)
        );
    }

    #[test]
    fn test_validate_password_very_strong() {
        assert_eq!(
            validate_password("TestPass123!"),
            Ok(PasswordStrength::VeryStrong)
        );
        assert_eq!(
            validate_password("MyP@ssw0rd2024!"),
            Ok(PasswordStrength::VeryStrong)
        );
    }

    #[test]
    fn test_strength_description() {
        assert_eq!(PasswordStrength::Weak.description(), "Weak - Too short");
        assert_eq!(PasswordStrength::VeryStrong.description(), "Very Strong");
    }
}
