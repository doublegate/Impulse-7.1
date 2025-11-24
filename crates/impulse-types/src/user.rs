//! Modern user account types
//!
//! This module provides the modern User API layer, separate from the Pascal binary
//! format (PascalUserRec). It includes user identification, validation, and conversion
//! between modern and Pascal formats.

use crate::pascal_user::PascalUserRec;
use crate::security::SecurityLevel;
use crate::user_flags::UserFlags;
use crate::user_prefs::UserPreferences;
use crate::user_stats::UserStats;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;

/// User ID (UUID v4)
///
/// Modern UUID-based user identification, separate from Pascal's 16-bit user index.
/// This allows for distributed systems and prevents ID reuse issues.
///
/// # Examples
///
/// ```
/// use impulse_types::user::UserId;
///
/// // Generate new user ID
/// let id = UserId::new();
///
/// // Check identity
/// let id2 = UserId::new();
/// assert_ne!(id, id2);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserId(Uuid);

impl UserId {
    /// Generate a new random user ID (UUID v4)
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::user::UserId;
    ///
    /// let id = UserId::new();
    /// assert!(!id.as_uuid().is_nil());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        UserId(Uuid::new_v4())
    }

    /// Create a user ID from an existing UUID
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::user::UserId;
    /// use uuid::Uuid;
    ///
    /// let uuid = Uuid::new_v4();
    /// let id = UserId::from_uuid(uuid);
    /// assert_eq!(id.as_uuid(), &uuid);
    /// ```
    #[must_use]
    pub const fn from_uuid(uuid: Uuid) -> Self {
        UserId(uuid)
    }

    /// Get the underlying UUID
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::user::UserId;
    ///
    /// let id = UserId::new();
    /// let uuid = id.as_uuid();
    /// assert!(!uuid.is_nil());
    /// ```
    #[must_use]
    pub const fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}

/// Modern user type
///
/// Represents a user account with modern Rust idioms and clean separation from
/// the Pascal binary format. Provides validation, conversion, and business logic.
///
/// # Architecture
///
/// - **Modern Layer:** User (this type) - Clean API, validation, business logic
/// - **Pascal Layer:** PascalUserRec - Binary compatibility with .DAT files
/// - **Conversion:** Bidirectional conversion methods maintain compatibility
///
/// # Examples
///
/// ```
/// use impulse_types::user::User;
/// use impulse_types::security::SecurityLevel;
///
/// // Create new user
/// let user = User::new("JohnDoe").unwrap();
/// assert_eq!(user.username(), "JohnDoe");
/// assert_eq!(user.security_level(), SecurityLevel::NEW_USER);
///
/// // Validate username
/// assert!(User::validate_username("ValidUser").is_ok());
/// assert!(User::validate_username("").is_err());
/// assert!(User::validate_username("Invalid@User").is_err());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Unique user ID (UUID v4)
    id: UserId,

    /// Username (handle) - 1-30 characters, alphanumeric/underscore/dash only
    username: String,

    /// Real name (optional)
    pub real_name: Option<String>,

    /// Email address (optional, validated format)
    pub email: Option<String>,

    /// Security level for general access
    security_level: SecurityLevel,

    /// Security level for file downloads
    pub download_security: SecurityLevel,

    /// User permission and preference flags
    pub flags: UserFlags,

    /// Activity statistics
    pub stats: UserStats,

    /// Terminal and UI preferences
    pub preferences: UserPreferences,

    /// Account creation timestamp
    pub created_at: SystemTime,

    /// Last login timestamp
    pub last_login: Option<SystemTime>,

    /// Account active (can log in)
    pub is_active: bool,

    /// Account locked (suspended)
    pub is_locked: bool,

    /// SysOp notes about this user
    pub sysop_note: Option<String>,
}

impl User {
    /// Create a new user with default settings
    ///
    /// # Arguments
    ///
    /// * `username` - Username (1-30 chars, alphanumeric/underscore/dash)
    ///
    /// # Errors
    ///
    /// Returns error if username is invalid (empty, too long, or invalid characters).
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::user::User;
    ///
    /// let user = User::new("JohnDoe").unwrap();
    /// assert_eq!(user.username(), "JohnDoe");
    /// assert!(user.is_active);
    /// assert!(!user.is_locked);
    /// ```
    pub fn new(username: impl AsRef<str>) -> Result<Self, String> {
        let username = username.as_ref().to_string();
        Self::validate_username(&username)?;

        Ok(User {
            id: UserId::new(),
            username,
            real_name: None,
            email: None,
            security_level: SecurityLevel::NEW_USER,
            download_security: SecurityLevel::NEW_USER,
            flags: UserFlags::default(),
            stats: UserStats::default(),
            preferences: UserPreferences::default(),
            created_at: SystemTime::now(),
            last_login: None,
            is_active: true,
            is_locked: false,
            sysop_note: None,
        })
    }

    /// Validate a username
    ///
    /// # Rules
    ///
    /// - Length: 1-30 characters
    /// - Characters: alphanumeric, underscore, dash only
    /// - No leading/trailing whitespace
    ///
    /// # Errors
    ///
    /// Returns error describing why username is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::user::User;
    ///
    /// assert!(User::validate_username("ValidUser").is_ok());
    /// assert!(User::validate_username("User_123").is_ok());
    /// assert!(User::validate_username("").is_err());
    /// assert!(User::validate_username("x".repeat(31).as_str()).is_err());
    /// assert!(User::validate_username("Invalid User").is_err());
    /// ```
    pub fn validate_username(username: &str) -> Result<(), String> {
        if username.is_empty() {
            return Err("Username cannot be empty".to_string());
        }

        if username.len() > 30 {
            return Err("Username must be 30 characters or less".to_string());
        }

        // Trim check
        if username.trim() != username {
            return Err("Username cannot have leading/trailing whitespace".to_string());
        }

        // Character validation
        if !username
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            return Err(
                "Username can only contain alphanumeric characters, underscore, and dash"
                    .to_string(),
            );
        }

        Ok(())
    }

    /// Validate an email address (basic validation)
    ///
    /// # Rules
    ///
    /// - Must contain '@' symbol
    /// - Must have characters before and after '@'
    /// - Maximum 255 characters
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::user::User;
    ///
    /// assert!(User::validate_email("user@example.com").is_ok());
    /// assert!(User::validate_email("invalid").is_err());
    /// assert!(User::validate_email("@example.com").is_err());
    /// assert!(User::validate_email("user@").is_err());
    /// ```
    pub fn validate_email(email: &str) -> Result<(), String> {
        if email.is_empty() {
            return Err("Email cannot be empty".to_string());
        }

        if email.len() > 255 {
            return Err("Email must be 255 characters or less".to_string());
        }

        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() != 2 {
            return Err("Email must contain exactly one '@' symbol".to_string());
        }

        if parts[0].is_empty() || parts[1].is_empty() {
            return Err("Email must have characters before and after '@'".to_string());
        }

        Ok(())
    }

    /// Get user ID
    #[must_use]
    pub const fn id(&self) -> UserId {
        self.id
    }

    /// Get username
    #[must_use]
    pub fn username(&self) -> &str {
        &self.username
    }

    /// Set username (with validation)
    ///
    /// # Errors
    ///
    /// Returns error if new username is invalid.
    pub fn set_username(&mut self, username: impl AsRef<str>) -> Result<(), String> {
        let username = username.as_ref().to_string();
        Self::validate_username(&username)?;
        self.username = username;
        Ok(())
    }

    /// Get security level
    #[must_use]
    pub const fn security_level(&self) -> SecurityLevel {
        self.security_level
    }

    /// Set security level
    pub fn set_security_level(&mut self, level: SecurityLevel) {
        self.security_level = level;
    }

    /// Set email (with validation)
    ///
    /// # Errors
    ///
    /// Returns error if email format is invalid.
    pub fn set_email(&mut self, email: impl AsRef<str>) -> Result<(), String> {
        let email = email.as_ref();
        if !email.is_empty() {
            Self::validate_email(email)?;
            self.email = Some(email.to_string());
        } else {
            self.email = None;
        }
        Ok(())
    }

    /// Check if user can access a resource requiring the given security level
    #[must_use]
    pub fn can_access(&self, required: SecurityLevel) -> bool {
        self.security_level.can_access(required)
    }

    /// Check if user is a SysOp
    #[must_use]
    pub fn is_sysop(&self) -> bool {
        self.security_level.is_sysop()
    }

    /// Check if user is an operator (Co-SysOp or SysOp)
    #[must_use]
    pub fn is_operator(&self) -> bool {
        self.security_level.is_operator()
    }

    /// Record a login
    pub fn record_login(&mut self) {
        self.last_login = Some(SystemTime::now());
        self.stats.record_login();
    }

    /// Lock the account (suspend access)
    pub fn lock(&mut self) {
        self.is_locked = true;
    }

    /// Unlock the account (restore access)
    pub fn unlock(&mut self) {
        self.is_locked = false;
    }

    /// Deactivate the account (soft delete)
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    /// Reactivate the account
    pub fn reactivate(&mut self) {
        self.is_active = true;
    }

    /// Convert to Pascal user record for binary serialization
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::user::User;
    ///
    /// let user = User::new("TestUser").unwrap();
    /// let pascal_rec = user.to_pascal();
    /// assert_eq!(pascal_rec.name.to_string(), "TestUser");
    /// ```
    #[must_use]
    pub fn to_pascal(&self) -> PascalUserRec {
        use crate::pascal_user::PascalString;

        PascalUserRec {
            name: PascalString::from_string(&self.username),
            realname: self
                .real_name
                .as_ref()
                .map(PascalString::from_string)
                .unwrap_or_default(),
            pw: PascalString::from_string(""), // Password handled separately
            ph: self
                .email
                .as_ref()
                .map(PascalString::from_string)
                .unwrap_or_default(),
            bday: PascalString::default(), // Birthday not stored in modern format yet
            firston: PascalString::default(), // First login timestamp conversion needed
            x1xs: [0; 2],
            laston: PascalString::default(), // Last login timestamp conversion needed
            x2xs: [0; 2],
            street: PascalString::default(),
            citystate: PascalString::default(),
            zipcode: PascalString::default(),
            unused: [0; 31],
            autosig: PascalString::default(),
            unused2: [0; 41],
            note: PascalString::default(),
            prompt: 0,
            lockedout: self.is_locked,
            deleted: !self.is_active,
            lockedfile: PascalString::default(),
            novotes: 0,
            yesvotes: 0,
            ac: self.flags,
            fflag: Default::default(),
            ar: Default::default(),
            zzqscan: [0; 64],
            xqxxx: [0; 64],
            zzqscn: [false; 64],
            zzdlnscn: Default::default(),
            unused3: [0; 20],
            sex: 0,
            ttimeon: self.stats.total_time_minutes as i32,
            x1xx: 0,
            uk: self.stats.upload_kb as i32,
            x2xx: 0,
            dk: self.stats.download_kb as i32,
            x3xx: 0,
            uploads: self.stats.uploads as i16,
            downloads: self.stats.downloads as i16,
            loggedon: self.stats.logins as i16,
            tltoday: self.stats.time_left_today,
            msgpost: self.stats.posts as i16,
            emailsent: self.stats.emails_sent as i16,
            feedback: self.stats.feedback_sent as i16,
            forusr: 0,
            filepoints: self.stats.file_points,
            waiting: 0,
            linelen: self.preferences.line_length,
            pagelen: self.preferences.page_length,
            ontoday: self.stats.logins_today,
            illegal: self.stats.illegal_attempts,
            sl: self.security_level.value(),
            dsl: self.download_security.value(),
            cols: Default::default(), // User colors not implemented yet
            lastmsg: 0,
            lastfil: 0,
            credit: 0,
            x4xx: 0,
            timebank: 0,
            boardsysop: [0; 5],
            trapactivity: false,
            trapseperate: false, // Note: Pascal has typo "trapseperate"
            timebankadd: 0,
            mpointer: 0,
            chatauto: false,
            chatseperate: false, // Note: Pascal has typo "chatseperate"
            userstartmenu: PascalString::default(),
            slogseperate: false, // Note: Pascal has typo "slogseperate"
            clsmsg: if self.preferences.clear_screen { 1 } else { 2 },
            flistopt: 0,
            msgorder: 0,
            avadjust: 1,
        }
    }

    /// Convert from Pascal user record
    ///
    /// # Errors
    ///
    /// Returns error if Pascal data is invalid (empty username, etc.).
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::user::User;
    /// use impulse_types::pascal_user::PascalUserRec;
    ///
    /// let pascal_rec = PascalUserRec::default();
    /// // This will fail because default has empty username
    /// assert!(User::from_pascal(&pascal_rec).is_err());
    /// ```
    pub fn from_pascal(rec: &PascalUserRec) -> Result<Self, String> {
        let username = rec.name.to_string();
        Self::validate_username(&username)?;

        let real_name = rec.realname.to_string();
        let real_name = if real_name.is_empty() {
            None
        } else {
            Some(real_name)
        };

        let email = rec.ph.to_string();
        let email = if email.is_empty() { None } else { Some(email) };

        Ok(User {
            id: UserId::new(), // Generate new UUID
            username,
            real_name,
            email,
            security_level: SecurityLevel::new(rec.sl),
            download_security: SecurityLevel::new(rec.dsl),
            flags: rec.ac, // UserFlags already converted by binrw
            stats: UserStats {
                total_time_minutes: rec.ttimeon as u32,
                uploads: rec.uploads as u16,
                downloads: rec.downloads as u16,
                upload_kb: rec.uk as u32,
                download_kb: rec.dk as u32,
                posts: rec.msgpost as u16,
                emails_sent: rec.emailsent as u16,
                feedback_sent: rec.feedback as u16,
                logins: rec.loggedon as u16,
                file_points: rec.filepoints,
                time_left_today: rec.tltoday,
                logins_today: rec.ontoday,
                illegal_attempts: rec.illegal,
            },
            preferences: UserPreferences {
                line_length: rec.linelen,
                page_length: rec.pagelen,
                ..Default::default()
            },
            created_at: SystemTime::now(), // Timestamp conversion needed
            last_login: None,              // Timestamp conversion needed
            is_active: !rec.deleted,
            is_locked: rec.lockedout,
            sysop_note: {
                let note = rec.note.to_string();
                if note.is_empty() { None } else { Some(note) }
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_id_new() {
        let id1 = UserId::new();
        let id2 = UserId::new();
        assert_ne!(id1, id2);
        assert!(!id1.as_uuid().is_nil());
    }

    #[test]
    fn test_user_id_from_uuid() {
        let uuid = Uuid::new_v4();
        let id = UserId::from_uuid(uuid);
        assert_eq!(id.as_uuid(), &uuid);
    }

    #[test]
    fn test_user_new() {
        let user = User::new("TestUser").unwrap();
        assert_eq!(user.username(), "TestUser");
        assert_eq!(user.security_level(), SecurityLevel::NEW_USER);
        assert!(user.is_active);
        assert!(!user.is_locked);
    }

    #[test]
    fn test_validate_username_valid() {
        assert!(User::validate_username("ValidUser").is_ok());
        assert!(User::validate_username("User_123").is_ok());
        assert!(User::validate_username("Test-User").is_ok());
        assert!(User::validate_username("a").is_ok());
        assert!(User::validate_username("x".repeat(30).as_str()).is_ok());
    }

    #[test]
    fn test_validate_username_empty() {
        assert!(User::validate_username("").is_err());
    }

    #[test]
    fn test_validate_username_too_long() {
        assert!(User::validate_username(&"x".repeat(31)).is_err());
    }

    #[test]
    fn test_validate_username_invalid_chars() {
        assert!(User::validate_username("Invalid User").is_err()); // Space
        assert!(User::validate_username("Invalid@User").is_err()); // @
        assert!(User::validate_username("User!").is_err()); // !
    }

    #[test]
    fn test_validate_username_whitespace() {
        assert!(User::validate_username(" User").is_err());
        assert!(User::validate_username("User ").is_err());
        assert!(User::validate_username(" User ").is_err());
    }

    #[test]
    fn test_validate_email_valid() {
        assert!(User::validate_email("user@example.com").is_ok());
        assert!(User::validate_email("test.user@domain.co.uk").is_ok());
        assert!(User::validate_email("a@b.c").is_ok());
    }

    #[test]
    fn test_validate_email_invalid() {
        assert!(User::validate_email("").is_err());
        assert!(User::validate_email("invalid").is_err());
        assert!(User::validate_email("@example.com").is_err());
        assert!(User::validate_email("user@").is_err());
        assert!(User::validate_email("user@@example.com").is_err());
        assert!(User::validate_email(&"x".repeat(256)).is_err());
    }

    #[test]
    fn test_set_username() {
        let mut user = User::new("OldName").unwrap();
        assert!(user.set_username("NewName").is_ok());
        assert_eq!(user.username(), "NewName");
        assert!(user.set_username("Invalid User").is_err());
        assert_eq!(user.username(), "NewName"); // Unchanged
    }

    #[test]
    fn test_set_email() {
        let mut user = User::new("TestUser").unwrap();
        assert!(user.set_email("user@example.com").is_ok());
        assert_eq!(user.email.as_deref(), Some("user@example.com"));
        assert!(user.set_email("invalid").is_err());
        assert_eq!(user.email.as_deref(), Some("user@example.com")); // Unchanged
    }

    #[test]
    fn test_security_level_operations() {
        let mut user = User::new("TestUser").unwrap();
        assert_eq!(user.security_level(), SecurityLevel::NEW_USER);

        user.set_security_level(SecurityLevel::VALIDATED);
        assert_eq!(user.security_level(), SecurityLevel::VALIDATED);

        assert!(user.can_access(SecurityLevel::NEW_USER));
        assert!(user.can_access(SecurityLevel::VALIDATED));
        assert!(!user.can_access(SecurityLevel::SYSOP));
    }

    #[test]
    fn test_is_sysop() {
        let mut user = User::new("TestUser").unwrap();
        assert!(!user.is_sysop());

        user.set_security_level(SecurityLevel::SYSOP);
        assert!(user.is_sysop());
    }

    #[test]
    fn test_is_operator() {
        let mut user = User::new("TestUser").unwrap();
        assert!(!user.is_operator());

        user.set_security_level(SecurityLevel::COSYSOP);
        assert!(user.is_operator());

        user.set_security_level(SecurityLevel::SYSOP);
        assert!(user.is_operator());
    }

    #[test]
    fn test_record_login() {
        let mut user = User::new("TestUser").unwrap();
        assert!(user.last_login.is_none());
        assert_eq!(user.stats.logins, 0);

        user.record_login();
        assert!(user.last_login.is_some());
        assert_eq!(user.stats.logins, 1);
    }

    #[test]
    fn test_lock_unlock() {
        let mut user = User::new("TestUser").unwrap();
        assert!(!user.is_locked);

        user.lock();
        assert!(user.is_locked);

        user.unlock();
        assert!(!user.is_locked);
    }

    #[test]
    fn test_activate_deactivate() {
        let mut user = User::new("TestUser").unwrap();
        assert!(user.is_active);

        user.deactivate();
        assert!(!user.is_active);

        user.reactivate();
        assert!(user.is_active);
    }

    #[test]
    fn test_to_pascal() {
        let user = User::new("TestUser").unwrap();
        let pascal = user.to_pascal();
        assert_eq!(pascal.name.to_string(), "TestUser");
        assert_eq!(pascal.sl, SecurityLevel::NEW_USER.value());
    }

    #[test]
    fn test_from_pascal_invalid_username() {
        let pascal = PascalUserRec::default(); // Default has empty username
        assert!(User::from_pascal(&pascal).is_err());
    }
}
