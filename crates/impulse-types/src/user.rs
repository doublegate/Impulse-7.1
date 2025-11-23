//! User account data types
//!
//! This module defines the core user account structures for the BBS system,
//! including user credentials, statistics, and access control.

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};

/// User security level
///
/// Controls what areas and features a user can access.
/// Higher levels have more privileges.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub enum SecurityLevel {
    /// Locked out - no access
    Locked = 0,
    /// New user - limited access
    #[default]
    NewUser = 10,
    /// Validated user - normal access
    Validated = 20,
    /// Privileged user - enhanced access
    Privileged = 50,
    /// Assistant SysOp - administrative access
    AssistantSysOp = 100,
    /// System Operator - full access
    SysOp = 255,
}

/// User statistics
///
/// Tracks user activity on the BBS system.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct UserStats {
    /// Number of times the user has called the BBS
    pub calls: u32,
    /// Number of files uploaded
    pub uploads: u32,
    /// Number of files downloaded
    pub downloads: u32,
    /// Total upload kilobytes
    pub upload_kb: u64,
    /// Total download kilobytes
    pub download_kb: u64,
    /// Number of messages posted
    pub messages_posted: u32,
    /// Time spent online (minutes)
    pub time_online: u32,
}

/// User account record
///
/// Represents a complete user account with credentials, permissions,
/// and activity statistics.
///
/// # Examples
///
/// ```
/// use impulse_types::user::{User, SecurityLevel};
///
/// let user = User {
///     id: 1,
///     name: "JohnDoe".to_string(),
///     password_hash: "hashed_password".to_string(),
///     security_level: SecurityLevel::Validated,
///     real_name: Some("John Doe".to_string()),
///     location: Some("New York".to_string()),
///     stats: Default::default(),
///     registration_date: chrono::Utc::now(),
///     last_login: Some(chrono::Utc::now()),
///     email: Some("john@example.com".to_string()),
///     phone: None,
///     birthday: None,
///     notes: None,
/// };
///
/// // Validate the user record
/// assert!(user.validate().is_ok());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Unique user ID
    pub id: u32,

    /// Username (handle) - must be 1-30 characters
    pub name: String,

    /// Hashed password (using Argon2id)
    pub password_hash: String,

    /// User's security/access level
    pub security_level: SecurityLevel,

    /// Real name (optional)
    pub real_name: Option<String>,

    /// Location/city (optional)
    pub location: Option<String>,

    /// User activity statistics
    pub stats: UserStats,

    /// Date and time of account registration
    #[serde(with = "chrono::serde::ts_seconds")]
    pub registration_date: chrono::DateTime<chrono::Utc>,

    /// Date and time of last login
    #[serde(default, with = "chrono::serde::ts_seconds_option")]
    pub last_login: Option<chrono::DateTime<chrono::Utc>>,

    /// Email address (optional)
    pub email: Option<String>,

    /// Phone number (optional)
    pub phone: Option<String>,

    /// Birthday (optional)
    pub birthday: Option<chrono::NaiveDate>,

    /// SysOp notes about the user (optional)
    pub notes: Option<String>,
}

impl User {
    /// Validate the user record
    ///
    /// Ensures all required fields meet the constraints for a valid user account.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Validation`] if:
    /// - Username is empty or longer than 30 characters
    /// - Username contains invalid characters
    /// - Password hash is empty
    /// - Email format is invalid (if provided)
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::user::{User, SecurityLevel};
    /// use chrono::Utc;
    ///
    /// let mut user = User {
    ///     id: 1,
    ///     name: "ValidUser".to_string(),
    ///     password_hash: "hashed".to_string(),
    ///     security_level: SecurityLevel::Validated,
    ///     real_name: None,
    ///     location: None,
    ///     stats: Default::default(),
    ///     registration_date: Utc::now(),
    ///     last_login: None,
    ///     email: None,
    ///     phone: None,
    ///     birthday: None,
    ///     notes: None,
    /// };
    ///
    /// assert!(user.validate().is_ok());
    ///
    /// // Invalid username
    /// user.name = "".to_string();
    /// assert!(user.validate().is_err());
    /// ```
    pub fn validate(&self) -> Result<()> {
        // Validate username length
        if self.name.is_empty() {
            return Err(Error::Validation("Username cannot be empty".to_string()));
        }

        if self.name.len() > 30 {
            return Err(Error::Validation(
                "Username must be 30 characters or less".to_string(),
            ));
        }

        // Validate username characters (alphanumeric, underscore, dash, space)
        if !self
            .name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == ' ')
        {
            return Err(Error::Validation(
                "Username contains invalid characters".to_string(),
            ));
        }

        // Validate password hash exists
        if self.password_hash.is_empty() {
            return Err(Error::Validation(
                "Password hash cannot be empty".to_string(),
            ));
        }

        // Validate email format if provided
        if let Some(email) = &self.email {
            if !email.is_empty() && !email.contains('@') {
                return Err(Error::Validation("Invalid email format".to_string()));
            }
        }

        Ok(())
    }

    /// Check if the user has at least the specified security level
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::user::{User, SecurityLevel};
    ///
    /// # let user = User {
    /// #     id: 1,
    /// #     name: "test".to_string(),
    /// #     password_hash: "hash".to_string(),
    /// #     security_level: SecurityLevel::Validated,
    /// #     real_name: None,
    /// #     location: None,
    /// #     stats: Default::default(),
    /// #     registration_date: chrono::Utc::now(),
    /// #     last_login: None,
    /// #     email: None,
    /// #     phone: None,
    /// #     birthday: None,
    /// #     notes: None,
    /// # };
    /// assert!(user.has_security_level(SecurityLevel::NewUser));
    /// assert!(user.has_security_level(SecurityLevel::Validated));
    /// assert!(!user.has_security_level(SecurityLevel::SysOp));
    /// ```
    pub fn has_security_level(&self, required: SecurityLevel) -> bool {
        self.security_level >= required
    }

    /// Check if the user is a SysOp
    pub fn is_sysop(&self) -> bool {
        self.security_level == SecurityLevel::SysOp
    }

    /// Record a successful login
    pub fn record_login(&mut self) {
        self.last_login = Some(chrono::Utc::now());
        self.stats.calls += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_user() -> User {
        User {
            id: 1,
            name: "TestUser".to_string(),
            password_hash: "hashed_password".to_string(),
            security_level: SecurityLevel::Validated,
            real_name: Some("Test User".to_string()),
            location: Some("Test City".to_string()),
            stats: Default::default(),
            registration_date: chrono::Utc::now(),
            last_login: None,
            email: Some("test@example.com".to_string()),
            phone: None,
            birthday: None,
            notes: None,
        }
    }

    #[test]
    fn test_valid_user() {
        let user = create_test_user();
        assert!(user.validate().is_ok());
    }

    #[test]
    fn test_empty_username() {
        let mut user = create_test_user();
        user.name = String::new();
        assert!(user.validate().is_err());
    }

    #[test]
    fn test_username_too_long() {
        let mut user = create_test_user();
        user.name = "a".repeat(31);
        assert!(user.validate().is_err());
    }

    #[test]
    fn test_invalid_username_chars() {
        let mut user = create_test_user();
        user.name = "test@user".to_string();
        assert!(user.validate().is_err());
    }

    #[test]
    fn test_empty_password_hash() {
        let mut user = create_test_user();
        user.password_hash = String::new();
        assert!(user.validate().is_err());
    }

    #[test]
    fn test_invalid_email() {
        let mut user = create_test_user();
        user.email = Some("not_an_email".to_string());
        assert!(user.validate().is_err());
    }

    #[test]
    fn test_security_level_comparison() {
        let user = create_test_user();
        assert!(user.has_security_level(SecurityLevel::NewUser));
        assert!(user.has_security_level(SecurityLevel::Validated));
        assert!(!user.has_security_level(SecurityLevel::SysOp));
    }

    #[test]
    fn test_is_sysop() {
        let mut user = create_test_user();
        assert!(!user.is_sysop());

        user.security_level = SecurityLevel::SysOp;
        assert!(user.is_sysop());
    }

    #[test]
    fn test_record_login() {
        let mut user = create_test_user();
        assert_eq!(user.stats.calls, 0);
        assert!(user.last_login.is_none());

        user.record_login();
        assert_eq!(user.stats.calls, 1);
        assert!(user.last_login.is_some());
    }

    #[test]
    fn test_security_level_ordering() {
        assert!(SecurityLevel::SysOp > SecurityLevel::NewUser);
        assert!(SecurityLevel::Validated > SecurityLevel::Locked);
        assert!(SecurityLevel::Privileged < SecurityLevel::SysOp);
    }
}
