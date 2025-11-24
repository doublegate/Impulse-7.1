//! Board flags for message and file areas
//!
//! This module defines flag types for message boards, file boards, and
//! conferences from the original Pascal RECORDS.PAS file.

use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    /// Message board flags (Pascal: `mbflags` set type)
    ///
    /// Original Pascal definition (RECORDS.PAS lines 262-263):
    /// ```pascal
    /// mbflags = (mb_fileboard, mb_auto, mb_nouser, mb_anon, mb_net,
    ///            mb_public, mb_private);
    /// ```
    ///
    /// Controls message board behavior and access.
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::board_flags::MessageBoardFlags;
    ///
    /// let mut flags = MessageBoardFlags::PUBLIC;
    /// flags.insert(MessageBoardFlags::ANONYMOUS);
    ///
    /// assert!(flags.is_public());
    /// assert!(flags.allows_anonymous());
    /// ```
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct MessageBoardFlags: u8 {
        /// Board is associated with a file area
        const FILE_BOARD    = 0b0000_0001;

        /// Auto-add messages to this board
        const AUTO          = 0b0000_0010;

        /// No user can post (SysOp only)
        const NO_USER       = 0b0000_0100;

        /// Anonymous posting allowed
        const ANONYMOUS     = 0b0000_1000;

        /// Board is networked (FidoNet/etc)
        const NETWORKED     = 0b0001_0000;

        /// Public board (all users can see)
        const PUBLIC        = 0b0010_0000;

        /// Private board (restricted access)
        const PRIVATE       = 0b0100_0000;
    }
}

impl Default for MessageBoardFlags {
    fn default() -> Self {
        MessageBoardFlags::PUBLIC
    }
}

impl MessageBoardFlags {
    /// Create from Pascal byte
    pub fn from_pascal_byte(byte: u8) -> Self {
        MessageBoardFlags::from_bits_truncate(byte)
    }

    /// Convert to Pascal byte
    pub fn to_pascal_byte(self) -> u8 {
        self.bits()
    }

    /// Check if board is public
    pub fn is_public(self) -> bool {
        self.contains(MessageBoardFlags::PUBLIC)
    }

    /// Check if board is private
    pub fn is_private(self) -> bool {
        self.contains(MessageBoardFlags::PRIVATE)
    }

    /// Check if anonymous posting is allowed
    pub fn allows_anonymous(self) -> bool {
        self.contains(MessageBoardFlags::ANONYMOUS)
    }

    /// Check if board is networked
    pub fn is_networked(self) -> bool {
        self.contains(MessageBoardFlags::NETWORKED)
    }

    /// Check if users can post
    pub fn users_can_post(self) -> bool {
        !self.contains(MessageBoardFlags::NO_USER)
    }
}

bitflags! {
    /// File board flags (Pascal: `fbflags` set type)
    ///
    /// Original Pascal definition (RECORDS.PAS lines 265-267):
    /// ```pascal
    /// fbflags = (fb_slow, fb_fast, fb_offline, fb_unlisted,
    ///            fb_priv, fb_msgpath, fb_autoload, fb_auto, fb_free);
    /// ```
    ///
    /// Controls file board behavior and access restrictions.
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::board_flags::FileBoardFlags;
    ///
    /// let flags = FileBoardFlags::FAST | FileBoardFlags::FREE;
    ///
    /// assert!(flags.is_fast());
    /// assert!(flags.is_free());
    /// assert!(!flags.is_private());
    /// ```
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct FileBoardFlags: u16 {
        /// Slow download area (modem speed restricted)
        const SLOW          = 0b0000_0000_0000_0001;

        /// Fast download area (no speed limit)
        const FAST          = 0b0000_0000_0000_0010;

        /// Offline storage (tape/CD-ROM)
        const OFFLINE       = 0b0000_0000_0000_0100;

        /// Unlisted (hidden from file area lists)
        const UNLISTED      = 0b0000_0000_0000_1000;

        /// Private area (restricted access)
        const PRIVATE       = 0b0000_0000_0001_0000;

        /// Message path area (associated with message board)
        const MSG_PATH      = 0b0000_0000_0010_0000;

        /// Auto-load files on entry
        const AUTO_LOAD     = 0b0000_0000_0100_0000;

        /// Auto-add files to this area
        const AUTO          = 0b0000_0000_1000_0000;

        /// Free download area (no ratio/credit check)
        const FREE          = 0b0000_0001_0000_0000;
    }
}

impl Default for FileBoardFlags {
    fn default() -> Self {
        FileBoardFlags::FAST
    }
}

impl FileBoardFlags {
    /// Create from Pascal word (u16)
    pub fn from_pascal_word(word: u16) -> Self {
        FileBoardFlags::from_bits_truncate(word)
    }

    /// Convert to Pascal word (u16)
    pub fn to_pascal_word(self) -> u16 {
        self.bits()
    }

    /// Check if area is fast
    pub fn is_fast(self) -> bool {
        self.contains(FileBoardFlags::FAST)
    }

    /// Check if area is slow
    pub fn is_slow(self) -> bool {
        self.contains(FileBoardFlags::SLOW)
    }

    /// Check if area is offline
    pub fn is_offline(self) -> bool {
        self.contains(FileBoardFlags::OFFLINE)
    }

    /// Check if area is private
    pub fn is_private(self) -> bool {
        self.contains(FileBoardFlags::PRIVATE)
    }

    /// Check if area is free (no ratio/credits)
    pub fn is_free(self) -> bool {
        self.contains(FileBoardFlags::FREE)
    }

    /// Check if area is unlisted
    pub fn is_unlisted(self) -> bool {
        self.contains(FileBoardFlags::UNLISTED)
    }
}

bitflags! {
    /// Conference flags (Pascal: `cfflags` set type)
    ///
    /// Original Pascal definition (RECORDS.PAS lines 269-270):
    /// ```pascal
    /// cfflags = (cf_nojoin, cf_auto, cf_public, cf_private,
    ///            cf_networked, cf_netmail);
    /// ```
    ///
    /// Controls conference behavior and access.
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::board_flags::ConferenceFlags;
    ///
    /// let flags = ConferenceFlags::PUBLIC | ConferenceFlags::AUTO;
    ///
    /// assert!(flags.is_public());
    /// assert!(flags.auto_join());
    /// assert!(flags.can_join());
    /// ```
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct ConferenceFlags: u8 {
        /// Users cannot join this conference
        const NO_JOIN       = 0b0000_0001;

        /// Auto-join users to this conference
        const AUTO          = 0b0000_0010;

        /// Public conference (all can see)
        const PUBLIC        = 0b0000_0100;

        /// Private conference (restricted)
        const PRIVATE       = 0b0000_1000;

        /// Conference is networked
        const NETWORKED     = 0b0001_0000;

        /// Conference supports netmail
        const NETMAIL       = 0b0010_0000;
    }
}

impl Default for ConferenceFlags {
    fn default() -> Self {
        ConferenceFlags::PUBLIC
    }
}

impl ConferenceFlags {
    /// Create from Pascal byte
    pub fn from_pascal_byte(byte: u8) -> Self {
        ConferenceFlags::from_bits_truncate(byte)
    }

    /// Convert to Pascal byte
    pub fn to_pascal_byte(self) -> u8 {
        self.bits()
    }

    /// Check if users can join
    pub fn can_join(self) -> bool {
        !self.contains(ConferenceFlags::NO_JOIN)
    }

    /// Check if conference auto-joins users
    pub fn auto_join(self) -> bool {
        self.contains(ConferenceFlags::AUTO)
    }

    /// Check if conference is public
    pub fn is_public(self) -> bool {
        self.contains(ConferenceFlags::PUBLIC)
    }

    /// Check if conference is private
    pub fn is_private(self) -> bool {
        self.contains(ConferenceFlags::PRIVATE)
    }

    /// Check if conference is networked
    pub fn is_networked(self) -> bool {
        self.contains(ConferenceFlags::NETWORKED)
    }

    /// Check if conference supports netmail
    pub fn has_netmail(self) -> bool {
        self.contains(ConferenceFlags::NETMAIL)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // MessageBoardFlags tests
    #[test]
    fn test_message_board_flags_default() {
        let flags = MessageBoardFlags::default();
        assert!(flags.is_public());
    }

    #[test]
    fn test_message_board_flags_pascal_conversion() {
        let flags = MessageBoardFlags::PUBLIC | MessageBoardFlags::ANONYMOUS;
        let byte = flags.to_pascal_byte();
        let restored = MessageBoardFlags::from_pascal_byte(byte);
        assert_eq!(flags, restored);
    }

    #[test]
    fn test_message_board_users_can_post() {
        let mut flags = MessageBoardFlags::PUBLIC;
        assert!(flags.users_can_post());

        flags.insert(MessageBoardFlags::NO_USER);
        assert!(!flags.users_can_post());
    }

    #[test]
    fn test_message_board_serialization() {
        let flags = MessageBoardFlags::PUBLIC | MessageBoardFlags::NETWORKED;
        let json = serde_json::to_string(&flags).unwrap();
        let restored: MessageBoardFlags = serde_json::from_str(&json).unwrap();
        assert_eq!(flags, restored);
    }

    // FileBoardFlags tests
    #[test]
    fn test_file_board_flags_default() {
        let flags = FileBoardFlags::default();
        assert!(flags.is_fast());
    }

    #[test]
    fn test_file_board_flags_pascal_conversion() {
        let flags = FileBoardFlags::FAST | FileBoardFlags::FREE;
        let word = flags.to_pascal_word();
        let restored = FileBoardFlags::from_pascal_word(word);
        assert_eq!(flags, restored);
    }

    #[test]
    fn test_file_board_speed_flags() {
        let slow = FileBoardFlags::SLOW;
        assert!(slow.is_slow());
        assert!(!slow.is_fast());

        let fast = FileBoardFlags::FAST;
        assert!(fast.is_fast());
        assert!(!fast.is_slow());
    }

    #[test]
    fn test_file_board_serialization() {
        let flags = FileBoardFlags::FAST | FileBoardFlags::FREE;
        let json = serde_json::to_string(&flags).unwrap();
        let restored: FileBoardFlags = serde_json::from_str(&json).unwrap();
        assert_eq!(flags, restored);
    }

    // ConferenceFlags tests
    #[test]
    fn test_conference_flags_default() {
        let flags = ConferenceFlags::default();
        assert!(flags.is_public());
    }

    #[test]
    fn test_conference_flags_pascal_conversion() {
        let flags = ConferenceFlags::PUBLIC | ConferenceFlags::AUTO;
        let byte = flags.to_pascal_byte();
        let restored = ConferenceFlags::from_pascal_byte(byte);
        assert_eq!(flags, restored);
    }

    #[test]
    fn test_conference_can_join() {
        let mut flags = ConferenceFlags::PUBLIC;
        assert!(flags.can_join());

        flags.insert(ConferenceFlags::NO_JOIN);
        assert!(!flags.can_join());
    }

    #[test]
    fn test_conference_serialization() {
        let flags = ConferenceFlags::PUBLIC | ConferenceFlags::NETWORKED;
        let json = serde_json::to_string(&flags).unwrap();
        let restored: ConferenceFlags = serde_json::from_str(&json).unwrap();
        assert_eq!(flags, restored);
    }
}
