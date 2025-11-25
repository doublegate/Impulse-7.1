//! Message base trait definitions

use crate::error::Result;
use crate::types::{
    FullMessage, MessageBaseStats, MessageHeader, MessageThread, NewMessage, SearchCriteria,
};
use async_trait::async_trait;

/// Message base interface for reading and writing messages
///
/// This trait defines the operations for reading and writing messages to a message base,
/// supporting various formats (JAM, Hudson, etc.).
#[async_trait]
pub trait MessageBase: Send + Sync {
    /// Read a complete message by message number
    ///
    /// # Arguments
    /// * `msg_num` - The message number to read
    ///
    /// # Returns
    /// The complete message with body and kludges
    ///
    /// # Errors
    /// Returns `MessageError::MessageNotFound` if the message doesn't exist
    async fn read_message(&self, msg_num: u32) -> Result<FullMessage>;

    /// Get the total number of messages in the base
    async fn message_count(&self) -> Result<u32>;

    /// Search for messages matching criteria
    ///
    /// # Arguments
    /// * `criteria` - The search criteria
    ///
    /// # Returns
    /// A list of message numbers matching the criteria
    async fn search(&self, criteria: &SearchCriteria) -> Result<Vec<u32>>;

    /// Get thread information for a message
    ///
    /// # Arguments
    /// * `msg_num` - The message number
    ///
    /// # Returns
    /// Thread information including parent, children, and depth
    async fn get_thread(&self, msg_num: u32) -> Result<MessageThread>;

    /// List message headers in a range
    ///
    /// # Arguments
    /// * `start` - Starting message number (1-based)
    /// * `count` - Number of messages to retrieve
    ///
    /// # Returns
    /// A list of message headers (may be less than count if at end of base)
    async fn list_messages(&self, start: u32, count: u32) -> Result<Vec<MessageHeader>>;

    /// Get message base statistics
    async fn get_stats(&self) -> Result<MessageBaseStats>;

    /// Mark a message as read
    ///
    /// # Arguments
    /// * `msg_num` - The message number
    async fn mark_read(&mut self, msg_num: u32) -> Result<()>;

    /// Check if a message exists
    ///
    /// # Arguments
    /// * `msg_num` - The message number
    async fn message_exists(&self, msg_num: u32) -> Result<bool>;

    /// Get the message number range
    ///
    /// # Returns
    /// A tuple of (first_msg, last_msg)
    async fn get_message_range(&self) -> Result<(u32, u32)>;

    /// Post a new message
    ///
    /// # Arguments
    /// * `message` - The new message to post
    ///
    /// # Returns
    /// The message number assigned to the new message
    ///
    /// # Errors
    /// Returns error if validation fails or write operation fails
    async fn post_message(&mut self, message: NewMessage) -> Result<u32>;

    /// Reply to an existing message
    ///
    /// # Arguments
    /// * `parent_msg_num` - The message number to reply to
    /// * `message` - The reply message (reply_to will be set automatically)
    ///
    /// # Returns
    /// The message number assigned to the reply
    ///
    /// # Errors
    /// Returns error if parent doesn't exist, validation fails, or write operation fails
    async fn reply_to_message(&mut self, parent_msg_num: u32, message: NewMessage) -> Result<u32>;
}
