//! Reply functionality for messages

use crate::error::{MessageError, Result};
use crate::quote::quote_message;
use crate::types::{FullMessage, NewMessage};

/// Reply builder for creating threaded replies
pub struct ReplyBuilder {
    /// Original message
    original: FullMessage,
    /// Quote original message
    quote_original: bool,
    /// Include attribution header
    include_attribution: bool,
}

impl ReplyBuilder {
    /// Create a new reply builder
    ///
    /// # Arguments
    /// * `original` - The message to reply to
    pub fn new(original: FullMessage) -> Self {
        Self {
            original,
            quote_original: true,
            include_attribution: true,
        }
    }

    /// Set whether to quote the original message
    pub fn quote_original(mut self, quote: bool) -> Self {
        self.quote_original = quote;
        self
    }

    /// Set whether to include attribution header
    pub fn include_attribution(mut self, include: bool) -> Self {
        self.include_attribution = include;
        self
    }

    /// Build a reply message
    ///
    /// # Arguments
    /// * `from` - The sender of the reply
    /// * `body` - The reply body text (will be prepended to quoted text)
    ///
    /// # Returns
    /// A new message configured as a reply
    pub fn build(self, from: impl Into<String>, body: impl Into<String>) -> NewMessage {
        let mut reply_body = body.into();

        // Add quoted original if requested
        if self.quote_original {
            reply_body.push_str("\n\n");
            let quoted = quote_message(
                &self.original.body,
                Some(&self.original.header.from),
                self.include_attribution,
            );
            reply_body.push_str(&quoted);
        }

        // Create reply message
        let subject = if self.original.header.subject.starts_with("Re: ") {
            self.original.header.subject.clone()
        } else {
            format!("Re: {}", self.original.header.subject)
        };

        NewMessage {
            from: from.into(),
            to: self.original.header.from.clone(),
            subject,
            body: reply_body,
            is_private: self.original.header.is_private,
            reply_to: Some(self.original.header.msg_num),
            area: "general".to_string(), // Default area
        }
    }
}

/// Thread manager for maintaining message threading
pub struct ThreadManager;

impl ThreadManager {
    /// Validate that a reply is valid
    ///
    /// # Arguments
    /// * `parent_msg_num` - The parent message number
    /// * `reply` - The reply message
    ///
    /// # Errors
    /// Returns error if reply structure is invalid
    pub fn validate_reply(parent_msg_num: u32, reply: &NewMessage) -> Result<()> {
        // Check that reply_to matches parent
        if let Some(reply_to) = reply.reply_to {
            if reply_to != parent_msg_num {
                return Err(MessageError::InvalidThread(
                    "Reply parent mismatch".to_string(),
                ));
            }
        } else {
            return Err(MessageError::InvalidThread(
                "Reply missing parent_id".to_string(),
            ));
        }

        // Check that subject starts with "Re: " for replies
        if !reply.subject.starts_with("Re: ") {
            return Err(MessageError::InvalidThread(
                "Reply subject should start with 'Re: '".to_string(),
            ));
        }

        Ok(())
    }

    /// Calculate thread depth
    ///
    /// # Arguments
    /// * `parent_depth` - Depth of parent message
    ///
    /// # Returns
    /// The depth for the new reply
    pub fn calculate_depth(parent_depth: u32) -> u32 {
        parent_depth + 1
    }

    /// Build thread path
    ///
    /// # Arguments
    /// * `parent_path` - Path of parent message
    /// * `msg_num` - The new message number
    ///
    /// # Returns
    /// The full path for the new reply
    pub fn build_path(parent_path: &[u32], msg_num: u32) -> Vec<u32> {
        let mut path = parent_path.to_vec();
        path.push(msg_num);
        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::MessageHeader;
    use chrono::Utc;

    fn create_test_message() -> FullMessage {
        FullMessage {
            header: MessageHeader {
                msg_num: 1,
                from: "Alice".to_string(),
                to: "Bob".to_string(),
                subject: "Test Subject".to_string(),
                date: Utc::now(),
                is_read: false,
                is_private: false,
                reply_to: None,
                reply_count: 0,
            },
            body: "This is the original message body.".to_string(),
            kludges: vec![],
        }
    }

    #[test]
    fn test_reply_builder_basic() {
        let original = create_test_message();
        let reply = ReplyBuilder::new(original)
            .quote_original(false)
            .build("Bob", "This is my reply.");

        assert_eq!(reply.from, "Bob");
        assert_eq!(reply.to, "Alice");
        assert_eq!(reply.subject, "Re: Test Subject");
        assert_eq!(reply.body, "This is my reply.");
        assert_eq!(reply.reply_to, Some(1));
    }

    #[test]
    fn test_reply_builder_with_quote() {
        let original = create_test_message();
        let reply = ReplyBuilder::new(original)
            .quote_original(true)
            .build("Bob", "This is my reply.");

        assert!(reply.body.contains("This is my reply."));
        assert!(reply.body.contains("> This is the original message body."));
    }

    #[test]
    fn test_reply_builder_with_attribution() {
        let original = create_test_message();
        let reply = ReplyBuilder::new(original)
            .quote_original(true)
            .include_attribution(true)
            .build("Bob", "This is my reply.");

        assert!(reply.body.contains("Alice wrote:"));
    }

    #[test]
    fn test_reply_builder_no_attribution() {
        let original = create_test_message();
        let reply = ReplyBuilder::new(original)
            .quote_original(true)
            .include_attribution(false)
            .build("Bob", "This is my reply.");

        assert!(!reply.body.contains("wrote:"));
    }

    #[test]
    fn test_reply_subject_no_duplicate_re() {
        let mut original = create_test_message();
        original.header.subject = "Re: Test Subject".to_string();

        let reply = ReplyBuilder::new(original).build("Bob", "Reply");

        assert_eq!(reply.subject, "Re: Test Subject");
        assert!(!reply.subject.contains("Re: Re: "));
    }

    #[test]
    fn test_validate_reply_valid() {
        let original = create_test_message();
        let reply = ReplyBuilder::new(original).build("Bob", "Reply");

        assert!(ThreadManager::validate_reply(1, &reply).is_ok());
    }

    #[test]
    fn test_validate_reply_missing_parent() {
        let mut reply = NewMessage::new("Bob", "Alice", "Re: Test");
        reply.reply_to = None;

        let result = ThreadManager::validate_reply(1, &reply);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_reply_wrong_parent() {
        let mut reply = NewMessage::new("Bob", "Alice", "Re: Test");
        reply.reply_to = Some(2);

        let result = ThreadManager::validate_reply(1, &reply);
        assert!(result.is_err());
    }

    #[test]
    fn test_calculate_depth() {
        assert_eq!(ThreadManager::calculate_depth(0), 1);
        assert_eq!(ThreadManager::calculate_depth(1), 2);
        assert_eq!(ThreadManager::calculate_depth(5), 6);
    }

    #[test]
    fn test_build_path() {
        let parent_path = vec![1, 2, 3];
        let path = ThreadManager::build_path(&parent_path, 4);

        assert_eq!(path, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_build_path_root() {
        let parent_path = vec![1];
        let path = ThreadManager::build_path(&parent_path, 2);

        assert_eq!(path, vec![1, 2]);
    }
}
