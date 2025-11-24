//! User flags and permissions
//!
//! This module defines user flags from the original Pascal `uflags` type,
//! which controls user permissions, restrictions, and preferences.

use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    /// User flags (Pascal: `uflags` set type)
    ///
    /// Original Pascal definition (RECORDS.PAS lines 125-128):
    /// ```pascal
    /// uflags = (rlogon, rchat, rvalidate, rbackspace, ramsg, rpostan, rpost,
    ///           remail, rvoting, rmsg, spcsr, onekey, avatar, pause, novice,
    ///           ansi, color, alert, smw, nomail, fnodlratio, fnopostratio,
    ///           fnofilepts, fnodeletion);
    /// ```
    ///
    /// These flags control user permissions (restrictions starting with 'r'),
    /// preferences (onekey, pause, etc.), and display settings (ansi, color, avatar).
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::user_flags::UserFlags;
    ///
    /// // Create flags for a new user with basic permissions
    /// let mut flags = UserFlags::ANSI | UserFlags::COLOR | UserFlags::PAUSE;
    ///
    /// // Check if user can post messages
    /// assert!(!flags.contains(UserFlags::RESTRICTED_POST));
    ///
    /// // Enable one-key mode
    /// flags.insert(UserFlags::ONE_KEY);
    /// assert!(flags.contains(UserFlags::ONE_KEY));
    ///
    /// // Restrict user from posting
    /// flags.insert(UserFlags::RESTRICTED_POST);
    /// assert!(flags.contains(UserFlags::RESTRICTED_POST));
    /// ```
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct UserFlags: u32 {
        // Restriction flags (24 total flags, so u32 is sufficient)

        /// Restricted from logging on
        const RESTRICTED_LOGON      = 0b0000_0000_0000_0000_0000_0000_0000_0001;

        /// Restricted from chat
        const RESTRICTED_CHAT       = 0b0000_0000_0000_0000_0000_0000_0000_0010;

        /// Restricted from validation
        const RESTRICTED_VALIDATE   = 0b0000_0000_0000_0000_0000_0000_0000_0100;

        /// Restricted from backspace
        const RESTRICTED_BACKSPACE  = 0b0000_0000_0000_0000_0000_0000_0000_1000;

        /// Restricted from auto-messages
        const RESTRICTED_AUTOMSG    = 0b0000_0000_0000_0000_0000_0000_0001_0000;

        /// Restricted from posting anonymously
        const RESTRICTED_POST_ANON  = 0b0000_0000_0000_0000_0000_0000_0010_0000;

        /// Restricted from posting messages
        const RESTRICTED_POST       = 0b0000_0000_0000_0000_0000_0000_0100_0000;

        /// Restricted from email
        const RESTRICTED_EMAIL      = 0b0000_0000_0000_0000_0000_0000_1000_0000;

        /// Restricted from voting
        const RESTRICTED_VOTING     = 0b0000_0000_0000_0000_0000_0001_0000_0000;

        /// Restricted from reading messages
        const RESTRICTED_MSG        = 0b0000_0000_0000_0000_0000_0010_0000_0000;

        // Display and preference flags

        /// Special cursor (Pascal: spcsr)
        const SPECIAL_CURSOR        = 0b0000_0000_0000_0000_0000_0100_0000_0000;

        /// One-key command mode (no Enter required)
        const ONE_KEY               = 0b0000_0000_0000_0000_0000_1000_0000_0000;

        /// Avatar graphics enabled
        const AVATAR                = 0b0000_0000_0000_0000_0001_0000_0000_0000;

        /// Pause at end of screen
        const PAUSE                 = 0b0000_0000_0000_0000_0010_0000_0000_0000;

        /// Novice mode (extra help)
        const NOVICE                = 0b0000_0000_0000_0000_0100_0000_0000_0000;

        /// ANSI graphics enabled
        const ANSI                  = 0b0000_0000_0000_0000_1000_0000_0000_0000;

        /// Color enabled
        const COLOR                 = 0b0000_0000_0000_0001_0000_0000_0000_0000;

        /// Alert on new messages
        const ALERT                 = 0b0000_0000_0000_0010_0000_0000_0000_0000;

        /// Show "Message Waiting" notification (Pascal: smw)
        const SHOW_MSG_WAITING      = 0b0000_0000_0000_0100_0000_0000_0000_0000;

        /// No email notifications
        const NO_MAIL               = 0b0000_0000_0000_1000_0000_0000_0000_0000;

        // Enforcement flags

        /// Force upload/download ratio (Pascal: fnodlratio)
        const FORCE_NO_DL_RATIO     = 0b0000_0000_0001_0000_0000_0000_0000_0000;

        /// Force post/read ratio (Pascal: fnopostratio)
        const FORCE_NO_POST_RATIO   = 0b0000_0000_0010_0000_0000_0000_0000_0000;

        /// Force file points (Pascal: fnofilepts)
        const FORCE_NO_FILE_PTS     = 0b0000_0000_0100_0000_0000_0000_0000_0000;

        /// Force no deletion (Pascal: fnodeletion)
        const FORCE_NO_DELETION     = 0b0000_0000_1000_0000_0000_0000_0000_0000;
    }
}

impl Default for UserFlags {
    /// Default flags for a new user
    ///
    /// Enables basic features: ANSI, Color, Pause
    /// Does not enable restrictions or advanced features
    fn default() -> Self {
        UserFlags::ANSI | UserFlags::COLOR | UserFlags::PAUSE
    }
}

impl UserFlags {
    /// Create flags from Pascal byte array
    ///
    /// Pascal stores `set of uflags` as a packed bit array.
    /// This converts from the Pascal binary format.
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::user_flags::UserFlags;
    ///
    /// // Pascal byte array (3 bytes for 24 flags)
    /// // ANSI is bit 15 (byte 1, bit 7), COLOR is bit 16 (byte 2, bit 0)
    /// let pascal_bytes = [0b0000_0000, 0b1000_0000, 0b0000_0001];
    /// let flags = UserFlags::from_pascal_bytes(&pascal_bytes);
    ///
    /// assert!(flags.contains(UserFlags::ANSI));
    /// assert!(flags.contains(UserFlags::COLOR));
    /// ```
    pub fn from_pascal_bytes(bytes: &[u8]) -> Self {
        // Pascal set is stored as bit array, one bit per flag
        // Convert to u32 for bitflags
        let mut bits: u32 = 0;

        for (byte_idx, &byte) in bytes.iter().take(3).enumerate() {
            for bit_idx in 0..8 {
                if byte & (1 << bit_idx) != 0 {
                    let flag_index = byte_idx * 8 + bit_idx;
                    if flag_index < 24 {
                        // Map to corresponding bit in u32
                        bits |= 1 << flag_index;
                    }
                }
            }
        }

        UserFlags::from_bits_truncate(bits)
    }

    /// Convert to Pascal byte array
    ///
    /// Converts to Pascal binary format (3 bytes for 24 flags).
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::user_flags::UserFlags;
    ///
    /// let flags = UserFlags::ANSI | UserFlags::COLOR | UserFlags::PAUSE;
    /// let bytes = flags.to_pascal_bytes();
    ///
    /// // Should have ANSI (bit 15), COLOR (bit 16), PAUSE (bit 13) set
    /// assert_eq!(bytes.len(), 3);
    /// ```
    pub fn to_pascal_bytes(&self) -> [u8; 3] {
        let bits = self.bits();
        let mut bytes = [0u8; 3];

        for flag_index in 0..24 {
            if bits & (1 << flag_index) != 0 {
                let byte_idx = flag_index / 8;
                let bit_idx = flag_index % 8;
                bytes[byte_idx] |= 1 << bit_idx;
            }
        }

        bytes
    }

    /// Check if user has any restrictions
    ///
    /// Returns true if any restriction flags are set.
    pub fn has_any_restrictions(&self) -> bool {
        const RESTRICTION_MASK: u32 = 0b0000_0000_0000_0000_0000_0011_1111_1111;
        self.bits() & RESTRICTION_MASK != 0
    }

    /// Check if user can log on
    pub fn can_logon(&self) -> bool {
        !self.contains(UserFlags::RESTRICTED_LOGON)
    }

    /// Check if user can post messages
    pub fn can_post(&self) -> bool {
        !self.contains(UserFlags::RESTRICTED_POST)
    }

    /// Check if user can read messages
    pub fn can_read_messages(&self) -> bool {
        !self.contains(UserFlags::RESTRICTED_MSG)
    }

    /// Check if user can use chat
    pub fn can_chat(&self) -> bool {
        !self.contains(UserFlags::RESTRICTED_CHAT)
    }

    /// Check if user can send email
    pub fn can_email(&self) -> bool {
        !self.contains(UserFlags::RESTRICTED_EMAIL)
    }

    /// Check if user wants ANSI graphics
    pub fn wants_ansi(&self) -> bool {
        self.contains(UserFlags::ANSI)
    }

    /// Check if user wants color
    pub fn wants_color(&self) -> bool {
        self.contains(UserFlags::COLOR)
    }

    /// Check if user wants screen pauses
    pub fn wants_pause(&self) -> bool {
        self.contains(UserFlags::PAUSE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_flags() {
        let flags = UserFlags::default();
        assert!(flags.contains(UserFlags::ANSI));
        assert!(flags.contains(UserFlags::COLOR));
        assert!(flags.contains(UserFlags::PAUSE));
        assert!(!flags.contains(UserFlags::RESTRICTED_POST));
    }

    #[test]
    fn test_restriction_check() {
        let mut flags = UserFlags::default();
        assert!(!flags.has_any_restrictions());

        flags.insert(UserFlags::RESTRICTED_POST);
        assert!(flags.has_any_restrictions());
    }

    #[test]
    fn test_can_post() {
        let mut flags = UserFlags::default();
        assert!(flags.can_post());

        flags.insert(UserFlags::RESTRICTED_POST);
        assert!(!flags.can_post());
    }

    #[test]
    fn test_pascal_conversion() {
        let flags = UserFlags::ANSI | UserFlags::COLOR | UserFlags::PAUSE;
        let bytes = flags.to_pascal_bytes();
        let restored = UserFlags::from_pascal_bytes(&bytes);

        assert_eq!(flags, restored);
    }

    #[test]
    fn test_empty_flags() {
        let flags = UserFlags::empty();
        assert!(!flags.wants_ansi());
        assert!(!flags.wants_color());
        assert!(!flags.has_any_restrictions());
    }

    #[test]
    fn test_all_flags() {
        let flags = UserFlags::all();
        assert!(flags.wants_ansi());
        assert!(flags.wants_color());
        assert!(flags.has_any_restrictions());
    }

    #[test]
    fn test_serialization() {
        let flags = UserFlags::ANSI | UserFlags::COLOR;

        // JSON serialization
        let json = serde_json::to_string(&flags).unwrap();
        let restored: UserFlags = serde_json::from_str(&json).unwrap();
        assert_eq!(flags, restored);
    }

    #[test]
    fn test_permission_checks() {
        let flags = UserFlags::default();

        assert!(flags.can_logon());
        assert!(flags.can_post());
        assert!(flags.can_read_messages());
        assert!(flags.can_chat());
        assert!(flags.can_email());
    }

    #[test]
    fn test_restricted_flags() {
        let flags =
            UserFlags::RESTRICTED_LOGON | UserFlags::RESTRICTED_POST | UserFlags::RESTRICTED_MSG;

        assert!(!flags.can_logon());
        assert!(!flags.can_post());
        assert!(!flags.can_read_messages());
        assert!(flags.has_any_restrictions());
    }
}
