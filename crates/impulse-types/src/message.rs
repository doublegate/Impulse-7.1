//! Message board data types
//!
//! This module defines the core message structures for the BBS message system,
//! supporting threaded discussions across multiple message areas.

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};

/// Message record
///
/// Represents a single message in a message area, with support for threading
/// and read status tracking.
///
/// # Examples
///
/// ```
/// use impulse_types::message::Message;
/// use chrono::Utc;
///
/// let message = Message {
///     id: 1,
///     from: "Alice".to_string(),
///     to: "Bob".to_string(),
///     subject: "Hello!".to_string(),
///     body: "This is a test message.".to_string(),
///     date: Utc::now(),
///     area_id: 1,
///     parent_id: None,
///     is_read: false,
///     is_private: false,
///     is_deleted: false,
/// };
///
/// assert!(message.validate().is_ok());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Unique message ID
    pub id: u32,

    /// Sender username
    pub from: String,

    /// Recipient username (or "All" for public messages)
    pub to: String,

    /// Message subject - must be 1-72 characters
    pub subject: String,

    /// Message body text
    pub body: String,

    /// Date and time the message was posted
    #[serde(with = "chrono::serde::ts_seconds")]
    pub date: chrono::DateTime<chrono::Utc>,

    /// Message area/conference ID
    pub area_id: u32,

    /// Parent message ID for threaded discussions (None for root messages)
    pub parent_id: Option<u32>,

    /// Whether the recipient has read this message
    pub is_read: bool,

    /// Whether this is a private message
    pub is_private: bool,

    /// Whether this message has been deleted (soft delete)
    pub is_deleted: bool,
}

impl Message {
    /// Validate the message record
    ///
    /// Ensures all required fields meet the constraints for a valid message.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Validation`] if:
    /// - From field is empty or longer than 30 characters
    /// - To field is empty or longer than 30 characters
    /// - Subject is empty or longer than 72 characters
    /// - Body is empty or exceeds maximum length (64KB)
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::message::Message;
    /// use chrono::Utc;
    ///
    /// let mut message = Message {
    ///     id: 1,
    ///     from: "Alice".to_string(),
    ///     to: "Bob".to_string(),
    ///     subject: "Test".to_string(),
    ///     body: "Valid message".to_string(),
    ///     date: Utc::now(),
    ///     area_id: 1,
    ///     parent_id: None,
    ///     is_read: false,
    ///     is_private: false,
    ///     is_deleted: false,
    /// };
    ///
    /// assert!(message.validate().is_ok());
    ///
    /// // Invalid subject (too long)
    /// message.subject = "a".repeat(73);
    /// assert!(message.validate().is_err());
    /// ```
    pub fn validate(&self) -> Result<()> {
        // Validate 'from' field
        if self.from.is_empty() {
            return Err(Error::Validation("From field cannot be empty".to_string()));
        }
        if self.from.len() > 30 {
            return Err(Error::Validation(
                "From field must be 30 characters or less".to_string(),
            ));
        }

        // Validate 'to' field
        if self.to.is_empty() {
            return Err(Error::Validation("To field cannot be empty".to_string()));
        }
        if self.to.len() > 30 {
            return Err(Error::Validation(
                "To field must be 30 characters or less".to_string(),
            ));
        }

        // Validate subject
        if self.subject.is_empty() {
            return Err(Error::Validation("Subject cannot be empty".to_string()));
        }
        if self.subject.len() > 72 {
            return Err(Error::Validation(
                "Subject must be 72 characters or less".to_string(),
            ));
        }

        // Validate body
        if self.body.is_empty() {
            return Err(Error::Validation(
                "Message body cannot be empty".to_string(),
            ));
        }
        // Maximum message size: 64KB
        if self.body.len() > 65536 {
            return Err(Error::Validation(
                "Message body too large (max 64KB)".to_string(),
            ));
        }

        Ok(())
    }

    /// Check if this is a public message (addressed to "All")
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::message::Message;
    /// use chrono::Utc;
    ///
    /// let mut message = Message {
    ///     id: 1,
    ///     from: "Alice".to_string(),
    ///     to: "All".to_string(),
    ///     subject: "Announcement".to_string(),
    ///     body: "Hello everyone!".to_string(),
    ///     date: Utc::now(),
    ///     area_id: 1,
    ///     parent_id: None,
    ///     is_read: false,
    ///     is_private: false,
    ///     is_deleted: false,
    /// };
    ///
    /// assert!(message.is_public());
    ///
    /// message.to = "Bob".to_string();
    /// assert!(!message.is_public());
    /// ```
    pub fn is_public(&self) -> bool {
        self.to.eq_ignore_ascii_case("All")
    }

    /// Check if this message is a reply (has a parent)
    pub fn is_reply(&self) -> bool {
        self.parent_id.is_some()
    }

    /// Mark the message as read
    pub fn mark_read(&mut self) {
        self.is_read = true;
    }

    /// Mark the message as unread
    pub fn mark_unread(&mut self) {
        self.is_read = false;
    }

    /// Soft delete the message
    pub fn delete(&mut self) {
        self.is_deleted = true;
    }

    /// Undelete the message
    pub fn undelete(&mut self) {
        self.is_deleted = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_message() -> Message {
        Message {
            id: 1,
            from: "Alice".to_string(),
            to: "Bob".to_string(),
            subject: "Test Message".to_string(),
            body: "This is a test message body.".to_string(),
            date: chrono::Utc::now(),
            area_id: 1,
            parent_id: None,
            is_read: false,
            is_private: false,
            is_deleted: false,
        }
    }

    #[test]
    fn test_valid_message() {
        let message = create_test_message();
        assert!(message.validate().is_ok());
    }

    #[test]
    fn test_empty_from() {
        let mut message = create_test_message();
        message.from = String::new();
        assert!(message.validate().is_err());
    }

    #[test]
    fn test_from_too_long() {
        let mut message = create_test_message();
        message.from = "a".repeat(31);
        assert!(message.validate().is_err());
    }

    #[test]
    fn test_empty_to() {
        let mut message = create_test_message();
        message.to = String::new();
        assert!(message.validate().is_err());
    }

    #[test]
    fn test_to_too_long() {
        let mut message = create_test_message();
        message.to = "a".repeat(31);
        assert!(message.validate().is_err());
    }

    #[test]
    fn test_empty_subject() {
        let mut message = create_test_message();
        message.subject = String::new();
        assert!(message.validate().is_err());
    }

    #[test]
    fn test_subject_too_long() {
        let mut message = create_test_message();
        message.subject = "a".repeat(73);
        assert!(message.validate().is_err());
    }

    #[test]
    fn test_empty_body() {
        let mut message = create_test_message();
        message.body = String::new();
        assert!(message.validate().is_err());
    }

    #[test]
    fn test_body_too_long() {
        let mut message = create_test_message();
        message.body = "a".repeat(65537);
        assert!(message.validate().is_err());
    }

    #[test]
    fn test_is_public() {
        let mut message = create_test_message();
        message.to = "All".to_string();
        assert!(message.is_public());

        message.to = "all".to_string();
        assert!(message.is_public());

        message.to = "Bob".to_string();
        assert!(!message.is_public());
    }

    #[test]
    fn test_is_reply() {
        let mut message = create_test_message();
        assert!(!message.is_reply());

        message.parent_id = Some(10);
        assert!(message.is_reply());
    }

    #[test]
    fn test_mark_read_unread() {
        let mut message = create_test_message();
        assert!(!message.is_read);

        message.mark_read();
        assert!(message.is_read);

        message.mark_unread();
        assert!(!message.is_read);
    }

    #[test]
    fn test_delete_undelete() {
        let mut message = create_test_message();
        assert!(!message.is_deleted);

        message.delete();
        assert!(message.is_deleted);

        message.undelete();
        assert!(!message.is_deleted);
    }
}
