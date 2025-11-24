//! Security level types and utilities
//!
//! This module defines the `SecurityLevel` type which represents a user's access
//! level in the BBS system. Security levels range from 0 (minimum) to 255 (SysOp).

use serde::{Deserialize, Serialize};
use std::fmt;

/// Security level (0-255, higher = more access)
///
/// The BBS uses security levels to control access to features, file areas,
/// message boards, and administrative functions. Each feature or resource
/// can specify a minimum security level required for access.
///
/// # Standard Levels
///
/// - **0** (MIN): Lowest possible level, typically unused
/// - **10** (NEW_USER): Default for new users who just registered
/// - **50** (VALIDATED): Users who have been validated by SysOp
/// - **100** (PRIVILEGED): Trusted users with extra permissions
/// - **200** (COSYSOP): Co-System Operators
/// - **255** (SYSOP): Full System Operator privileges
///
/// # Examples
///
/// ```
/// use impulse_types::security::SecurityLevel;
///
/// // Create security levels
/// let new_user = SecurityLevel::NEW_USER;
/// let sysop = SecurityLevel::SYSOP;
///
/// // Check access
/// assert!(sysop.can_access(new_user));
/// assert!(!new_user.can_access(sysop));
///
/// // Check SysOp status
/// assert!(sysop.is_sysop());
/// assert!(!new_user.is_sysop());
///
/// // Create custom level
/// let moderator = SecurityLevel::new(75);
/// assert!(moderator.can_access(new_user));
/// assert!(!moderator.is_sysop());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SecurityLevel(u8);

impl SecurityLevel {
    /// Minimum security level (0)
    pub const MIN: SecurityLevel = SecurityLevel(0);

    /// New user level (10) - Default for new registrations
    pub const NEW_USER: SecurityLevel = SecurityLevel(10);

    /// Validated user level (50) - Users validated by SysOp
    pub const VALIDATED: SecurityLevel = SecurityLevel(50);

    /// Privileged user level (100) - Trusted users
    pub const PRIVILEGED: SecurityLevel = SecurityLevel(100);

    /// Co-SysOp level (200) - Assistant system operators
    pub const COSYSOP: SecurityLevel = SecurityLevel(200);

    /// SysOp level (255) - Full system operator
    pub const SYSOP: SecurityLevel = SecurityLevel(255);

    /// Create a new security level
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::security::SecurityLevel;
    ///
    /// let level = SecurityLevel::new(75);
    /// assert_eq!(level.value(), 75);
    /// ```
    #[must_use]
    pub const fn new(level: u8) -> Self {
        SecurityLevel(level)
    }

    /// Get the numeric security level value
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::security::SecurityLevel;
    ///
    /// let level = SecurityLevel::NEW_USER;
    /// assert_eq!(level.value(), 10);
    /// ```
    #[must_use]
    pub const fn value(self) -> u8 {
        self.0
    }

    /// Check if this security level can access a resource requiring the given level
    ///
    /// Returns `true` if this level is greater than or equal to the required level.
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::security::SecurityLevel;
    ///
    /// let user_level = SecurityLevel::VALIDATED;
    /// let required_level = SecurityLevel::NEW_USER;
    ///
    /// assert!(user_level.can_access(required_level));
    /// ```
    #[must_use]
    pub fn can_access(self, required: SecurityLevel) -> bool {
        self.0 >= required.0
    }

    /// Check if this is a SysOp level (255)
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::security::SecurityLevel;
    ///
    /// assert!(SecurityLevel::SYSOP.is_sysop());
    /// assert!(!SecurityLevel::COSYSOP.is_sysop());
    /// ```
    #[must_use]
    pub fn is_sysop(self) -> bool {
        self.0 == 255
    }

    /// Check if this is a Co-SysOp or SysOp level (>= 200)
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::security::SecurityLevel;
    ///
    /// assert!(SecurityLevel::SYSOP.is_operator());
    /// assert!(SecurityLevel::COSYSOP.is_operator());
    /// assert!(!SecurityLevel::PRIVILEGED.is_operator());
    /// ```
    #[must_use]
    pub fn is_operator(self) -> bool {
        self.0 >= 200
    }

    /// Promote to the next standard level, if possible
    ///
    /// Returns the new level, or None if already at maximum or between standard levels.
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::security::SecurityLevel;
    ///
    /// let mut level = SecurityLevel::NEW_USER;
    /// if let Some(promoted) = level.promote() {
    ///     level = promoted;
    /// }
    /// assert_eq!(level, SecurityLevel::VALIDATED);
    /// ```
    #[must_use]
    pub fn promote(self) -> Option<SecurityLevel> {
        match self.0 {
            0..=9 => Some(SecurityLevel::NEW_USER),
            10..=49 => Some(SecurityLevel::VALIDATED),
            50..=99 => Some(SecurityLevel::PRIVILEGED),
            100..=199 => Some(SecurityLevel::COSYSOP),
            200..=254 => Some(SecurityLevel::SYSOP),
            255 => None, // Already at max
        }
    }

    /// Demote to the previous standard level, if possible
    ///
    /// Returns the new level, or None if already at minimum or between standard levels.
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::security::SecurityLevel;
    ///
    /// let mut level = SecurityLevel::VALIDATED;
    /// if let Some(demoted) = level.demote() {
    ///     level = demoted;
    /// }
    /// assert_eq!(level, SecurityLevel::NEW_USER);
    /// ```
    #[must_use]
    pub fn demote(self) -> Option<SecurityLevel> {
        match self.0 {
            0..=9 => None, // Already at min
            10..=49 => Some(SecurityLevel::MIN),
            50..=99 => Some(SecurityLevel::NEW_USER),
            100..=199 => Some(SecurityLevel::VALIDATED),
            200..=254 => Some(SecurityLevel::PRIVILEGED),
            255 => Some(SecurityLevel::COSYSOP),
        }
    }
}

impl Default for SecurityLevel {
    /// Default security level is NEW_USER (10)
    fn default() -> Self {
        SecurityLevel::NEW_USER
    }
}

impl fmt::Display for SecurityLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self.0 {
            0 => "Minimum",
            10 => "New User",
            50 => "Validated",
            100 => "Privileged",
            200 => "Co-SysOp",
            255 => "SysOp",
            n => return write!(f, "Level {}", n),
        };
        write!(f, "{} ({})", name, self.0)
    }
}

impl From<u8> for SecurityLevel {
    fn from(value: u8) -> Self {
        SecurityLevel::new(value)
    }
}

impl From<SecurityLevel> for u8 {
    fn from(level: SecurityLevel) -> Self {
        level.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_level_constants() {
        assert_eq!(SecurityLevel::MIN.value(), 0);
        assert_eq!(SecurityLevel::NEW_USER.value(), 10);
        assert_eq!(SecurityLevel::VALIDATED.value(), 50);
        assert_eq!(SecurityLevel::PRIVILEGED.value(), 100);
        assert_eq!(SecurityLevel::COSYSOP.value(), 200);
        assert_eq!(SecurityLevel::SYSOP.value(), 255);
    }

    #[test]
    fn test_security_level_ordering() {
        assert!(SecurityLevel::SYSOP > SecurityLevel::COSYSOP);
        assert!(SecurityLevel::COSYSOP > SecurityLevel::PRIVILEGED);
        assert!(SecurityLevel::PRIVILEGED > SecurityLevel::VALIDATED);
        assert!(SecurityLevel::VALIDATED > SecurityLevel::NEW_USER);
        assert!(SecurityLevel::NEW_USER > SecurityLevel::MIN);
    }

    #[test]
    fn test_can_access() {
        let sysop = SecurityLevel::SYSOP;
        let new_user = SecurityLevel::NEW_USER;

        // SysOp can access everything
        assert!(sysop.can_access(SecurityLevel::MIN));
        assert!(sysop.can_access(SecurityLevel::NEW_USER));
        assert!(sysop.can_access(SecurityLevel::SYSOP));

        // New user can only access new user and below
        assert!(new_user.can_access(SecurityLevel::MIN));
        assert!(new_user.can_access(SecurityLevel::NEW_USER));
        assert!(!new_user.can_access(SecurityLevel::VALIDATED));
        assert!(!new_user.can_access(SecurityLevel::SYSOP));
    }

    #[test]
    fn test_is_sysop() {
        assert!(SecurityLevel::SYSOP.is_sysop());
        assert!(!SecurityLevel::COSYSOP.is_sysop());
        assert!(!SecurityLevel::new(254).is_sysop());
    }

    #[test]
    fn test_is_operator() {
        assert!(SecurityLevel::SYSOP.is_operator());
        assert!(SecurityLevel::COSYSOP.is_operator());
        assert!(SecurityLevel::new(200).is_operator());
        assert!(!SecurityLevel::new(199).is_operator());
        assert!(!SecurityLevel::PRIVILEGED.is_operator());
    }

    #[test]
    fn test_promote() {
        assert_eq!(SecurityLevel::MIN.promote(), Some(SecurityLevel::NEW_USER));
        assert_eq!(
            SecurityLevel::NEW_USER.promote(),
            Some(SecurityLevel::VALIDATED)
        );
        assert_eq!(
            SecurityLevel::VALIDATED.promote(),
            Some(SecurityLevel::PRIVILEGED)
        );
        assert_eq!(
            SecurityLevel::PRIVILEGED.promote(),
            Some(SecurityLevel::COSYSOP)
        );
        assert_eq!(SecurityLevel::COSYSOP.promote(), Some(SecurityLevel::SYSOP));
        assert_eq!(SecurityLevel::SYSOP.promote(), None);
    }

    #[test]
    fn test_demote() {
        assert_eq!(SecurityLevel::MIN.demote(), None);
        assert_eq!(SecurityLevel::NEW_USER.demote(), Some(SecurityLevel::MIN));
        assert_eq!(
            SecurityLevel::VALIDATED.demote(),
            Some(SecurityLevel::NEW_USER)
        );
        assert_eq!(
            SecurityLevel::PRIVILEGED.demote(),
            Some(SecurityLevel::VALIDATED)
        );
        assert_eq!(
            SecurityLevel::COSYSOP.demote(),
            Some(SecurityLevel::PRIVILEGED)
        );
        assert_eq!(SecurityLevel::SYSOP.demote(), Some(SecurityLevel::COSYSOP));
    }

    #[test]
    fn test_default() {
        assert_eq!(SecurityLevel::default(), SecurityLevel::NEW_USER);
    }

    #[test]
    fn test_display() {
        assert_eq!(SecurityLevel::MIN.to_string(), "Minimum (0)");
        assert_eq!(SecurityLevel::NEW_USER.to_string(), "New User (10)");
        assert_eq!(SecurityLevel::VALIDATED.to_string(), "Validated (50)");
        assert_eq!(SecurityLevel::PRIVILEGED.to_string(), "Privileged (100)");
        assert_eq!(SecurityLevel::COSYSOP.to_string(), "Co-SysOp (200)");
        assert_eq!(SecurityLevel::SYSOP.to_string(), "SysOp (255)");
        assert_eq!(SecurityLevel::new(75).to_string(), "Level 75");
    }

    #[test]
    fn test_from_u8() {
        let level: SecurityLevel = 100u8.into();
        assert_eq!(level, SecurityLevel::PRIVILEGED);
    }

    #[test]
    fn test_into_u8() {
        let value: u8 = SecurityLevel::VALIDATED.into();
        assert_eq!(value, 50);
    }

    #[test]
    fn test_serialization() {
        let level = SecurityLevel::VALIDATED;
        let json = serde_json::to_string(&level).unwrap();
        assert_eq!(json, "50");

        let deserialized: SecurityLevel = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, level);
    }
}
