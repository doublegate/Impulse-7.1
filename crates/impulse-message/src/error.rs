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

    /// Validation error
    #[error("Validation error: {0}")]
    Validation(String),

    /// Message body too long
    #[error("Message body exceeds maximum length of {max} bytes (got {actual})")]
    BodyTooLong {
        /// Maximum allowed length
        max: usize,
        /// Actual length
        actual: usize,
    },

    /// Subject too long
    #[error("Subject exceeds maximum length of {max} characters (got {actual})")]
    SubjectTooLong {
        /// Maximum allowed length
        max: usize,
        /// Actual length
        actual: usize,
    },

    /// Subject too short
    #[error("Subject is too short (minimum {min} characters)")]
    SubjectTooShort {
        /// Minimum required length
        min: usize,
    },

    /// Body too short
    #[error("Message body is too short (minimum {min} characters)")]
    BodyTooShort {
        /// Minimum required length
        min: usize,
    },

    /// Required field missing
    #[error("Required field missing: {0}")]
    RequiredFieldMissing(String),

    /// Write error
    #[error("Write error: {0}")]
    WriteError(String),

    /// Atomic operation failed
    #[error("Atomic operation failed: {0}")]
    AtomicFailed(String),

    /// Index update failed
    #[error("Index update failed: {0}")]
    IndexUpdateFailed(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Deserialization error
    #[error("Deserialization error: {0}")]
    Deserialization(String),
}

impl From<binrw::Error> for MessageError {
    fn from(err: binrw::Error) -> Self {
        MessageError::BinRead(err.to_string())
    }
}
