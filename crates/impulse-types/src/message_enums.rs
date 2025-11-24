//! Message board enumeration types
//!
//! This module defines enumeration types for the message system from the
//! original Pascal RECORDS.PAS file.

use serde::{Deserialize, Serialize};

/// Anonymous message type (Pascal: `anontyp`)
///
/// Original Pascal definition (RECORDS.PAS lines 130-131):
/// ```pascal
/// anontyp = (anon_none, anon_yes, anon_forced, anon_dear_abby, anon_any_name);
/// ```
///
/// Controls whether messages in a board can be posted anonymously.
///
/// # Examples
///
/// ```
/// use impulse_types::message_enums::AnonymousType;
///
/// let anon_type = AnonymousType::Allowed;
/// assert!(anon_type.is_allowed());
///
/// let forced = AnonymousType::Forced;
/// assert!(forced.is_anonymous());
/// ```
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AnonymousType {
    /// No anonymous posting allowed
    #[default]
    NotAllowed = 0,

    /// Anonymous posting allowed (user choice)
    Allowed = 1,

    /// All posts are anonymous (forced)
    Forced = 2,

    /// "Dear Abby" style (posts show as "Dear Abby")
    DearAbby = 3,

    /// User can enter any name
    AnyName = 4,
}

impl AnonymousType {
    /// Create from Pascal byte value
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::message_enums::AnonymousType;
    ///
    /// let anon_type = AnonymousType::from_pascal_byte(1);
    /// assert_eq!(anon_type, AnonymousType::Allowed);
    /// ```
    pub fn from_pascal_byte(value: u8) -> Self {
        match value {
            0 => AnonymousType::NotAllowed,
            1 => AnonymousType::Allowed,
            2 => AnonymousType::Forced,
            3 => AnonymousType::DearAbby,
            4 => AnonymousType::AnyName,
            _ => AnonymousType::NotAllowed, // Default for invalid values
        }
    }

    /// Convert to Pascal byte value
    pub fn to_pascal_byte(self) -> u8 {
        self as u8
    }

    /// Check if anonymous posting is allowed
    pub fn is_allowed(self) -> bool {
        !matches!(self, AnonymousType::NotAllowed)
    }

    /// Check if all posts are anonymous
    pub fn is_anonymous(self) -> bool {
        matches!(
            self,
            AnonymousType::Forced | AnonymousType::DearAbby | AnonymousType::AnyName
        )
    }

    /// Check if user has choice
    pub fn has_user_choice(self) -> bool {
        matches!(self, AnonymousType::Allowed | AnonymousType::AnyName)
    }
}

/// Message index status (Pascal: `msgindexstatr`)
///
/// Original Pascal definition (RECORDS.PAS lines 133-134):
/// ```pascal
/// msgindexstatr = (mis_open, mis_closed, mis_inval, mis_dead);
/// ```
///
/// Status flags for message index records.
///
/// # Examples
///
/// ```
/// use impulse_types::message_enums::MessageIndexStatus;
///
/// let status = MessageIndexStatus::Open;
/// assert!(status.is_valid());
///
/// let closed = MessageIndexStatus::Closed;
/// assert!(!closed.is_open());
/// ```
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum MessageIndexStatus {
    /// Message is open/active
    #[default]
    Open = 0,

    /// Message is closed (no more replies)
    Closed = 1,

    /// Message is invalid
    Invalid = 2,

    /// Message is dead/deleted
    Dead = 3,
}

impl MessageIndexStatus {
    /// Create from Pascal byte value
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::message_enums::MessageIndexStatus;
    ///
    /// let status = MessageIndexStatus::from_pascal_byte(0);
    /// assert_eq!(status, MessageIndexStatus::Open);
    /// ```
    pub fn from_pascal_byte(value: u8) -> Self {
        match value {
            0 => MessageIndexStatus::Open,
            1 => MessageIndexStatus::Closed,
            2 => MessageIndexStatus::Invalid,
            3 => MessageIndexStatus::Dead,
            _ => MessageIndexStatus::Invalid, // Default for invalid values
        }
    }

    /// Convert to Pascal byte value
    pub fn to_pascal_byte(self) -> u8 {
        self as u8
    }

    /// Check if message is open for replies
    pub fn is_open(self) -> bool {
        matches!(self, MessageIndexStatus::Open)
    }

    /// Check if message is valid
    pub fn is_valid(self) -> bool {
        !matches!(self, MessageIndexStatus::Invalid | MessageIndexStatus::Dead)
    }

    /// Check if message is deleted
    pub fn is_deleted(self) -> bool {
        matches!(self, MessageIndexStatus::Dead)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // AnonymousType tests
    #[test]
    fn test_anonymous_type_default() {
        assert_eq!(AnonymousType::default(), AnonymousType::NotAllowed);
    }

    #[test]
    fn test_anonymous_type_pascal_conversion() {
        for i in 0..=4 {
            let anon_type = AnonymousType::from_pascal_byte(i);
            assert_eq!(anon_type.to_pascal_byte(), i);
        }
    }

    #[test]
    fn test_anonymous_type_invalid_byte() {
        let anon_type = AnonymousType::from_pascal_byte(255);
        assert_eq!(anon_type, AnonymousType::NotAllowed);
    }

    #[test]
    fn test_anonymous_type_is_allowed() {
        assert!(!AnonymousType::NotAllowed.is_allowed());
        assert!(AnonymousType::Allowed.is_allowed());
        assert!(AnonymousType::Forced.is_allowed());
        assert!(AnonymousType::DearAbby.is_allowed());
        assert!(AnonymousType::AnyName.is_allowed());
    }

    #[test]
    fn test_anonymous_type_is_anonymous() {
        assert!(!AnonymousType::NotAllowed.is_anonymous());
        assert!(!AnonymousType::Allowed.is_anonymous());
        assert!(AnonymousType::Forced.is_anonymous());
        assert!(AnonymousType::DearAbby.is_anonymous());
        assert!(AnonymousType::AnyName.is_anonymous());
    }

    #[test]
    fn test_anonymous_type_has_user_choice() {
        assert!(!AnonymousType::NotAllowed.has_user_choice());
        assert!(AnonymousType::Allowed.has_user_choice());
        assert!(!AnonymousType::Forced.has_user_choice());
        assert!(!AnonymousType::DearAbby.has_user_choice());
        assert!(AnonymousType::AnyName.has_user_choice());
    }

    #[test]
    fn test_anonymous_type_serialization() {
        let anon_type = AnonymousType::Allowed;
        let json = serde_json::to_string(&anon_type).unwrap();
        let restored: AnonymousType = serde_json::from_str(&json).unwrap();
        assert_eq!(anon_type, restored);
    }

    // MessageIndexStatus tests
    #[test]
    fn test_message_index_status_default() {
        assert_eq!(MessageIndexStatus::default(), MessageIndexStatus::Open);
    }

    #[test]
    fn test_message_index_status_pascal_conversion() {
        for i in 0..=3 {
            let status = MessageIndexStatus::from_pascal_byte(i);
            assert_eq!(status.to_pascal_byte(), i);
        }
    }

    #[test]
    fn test_message_index_status_invalid_byte() {
        let status = MessageIndexStatus::from_pascal_byte(255);
        assert_eq!(status, MessageIndexStatus::Invalid);
    }

    #[test]
    fn test_message_index_status_is_open() {
        assert!(MessageIndexStatus::Open.is_open());
        assert!(!MessageIndexStatus::Closed.is_open());
        assert!(!MessageIndexStatus::Invalid.is_open());
        assert!(!MessageIndexStatus::Dead.is_open());
    }

    #[test]
    fn test_message_index_status_is_valid() {
        assert!(MessageIndexStatus::Open.is_valid());
        assert!(MessageIndexStatus::Closed.is_valid());
        assert!(!MessageIndexStatus::Invalid.is_valid());
        assert!(!MessageIndexStatus::Dead.is_valid());
    }

    #[test]
    fn test_message_index_status_is_deleted() {
        assert!(!MessageIndexStatus::Open.is_deleted());
        assert!(!MessageIndexStatus::Closed.is_deleted());
        assert!(!MessageIndexStatus::Invalid.is_deleted());
        assert!(MessageIndexStatus::Dead.is_deleted());
    }

    #[test]
    fn test_message_index_status_serialization() {
        let status = MessageIndexStatus::Closed;
        let json = serde_json::to_string(&status).unwrap();
        let restored: MessageIndexStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(status, restored);
    }
}
