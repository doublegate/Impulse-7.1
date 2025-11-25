//! Error types for message operations

use thiserror::Error;

/// Result type for message operations
pub type Result<T> = std::result::Result<T, MessageError>;

/// Errors that can occur during message operations
#[derive(Debug, Error)]
pub enum MessageError {
    /// Message not found
    #[error("Message {0} not found")]
    MessageNotFound(u32),

    /// Message area not found
    #[error("Message area '{0}' not found")]
    AreaNotFound(String),

    /// Corrupted message data
    #[error("Corrupted message data: {0}")]
    CorruptMessage(String),

    /// Invalid message header
    #[error("Invalid message header: {0}")]
    InvalidHeader(String),

    /// Invalid message format
    #[error("Invalid message format: {0}")]
    InvalidFormat(String),

    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Binary parsing error
    #[error("Binary parsing error: {0}")]
    BinRead(String),

    /// UTF-8 encoding error
    #[error("UTF-8 encoding error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    /// Invalid thread structure
    #[error("Invalid thread structure: {0}")]
    InvalidThread(String),

    /// Search error
    #[error("Search error: {0}")]
    SearchError(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Cache error
    #[error("Cache error: {0}")]
    Cache(String),
}

impl From<binrw::Error> for MessageError {
    fn from(err: binrw::Error) -> Self {
        MessageError::BinRead(err.to_string())
    }
}
