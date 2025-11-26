//! Error types for telnet protocol operations

use std::io;
use thiserror::Error;

/// Result type alias for telnet operations
pub type Result<T> = std::result::Result<T, TelnetError>;

/// Errors that can occur during telnet operations
#[derive(Error, Debug)]
pub enum TelnetError {
    /// I/O error occurred
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    /// Connection closed by remote host
    #[error("Connection closed by remote host")]
    ConnectionClosed,

    /// Invalid telnet command received
    #[error("Invalid IAC command: {0}")]
    InvalidCommand(u8),

    /// Protocol violation
    #[error("Protocol violation: {0}")]
    ProtocolViolation(String),

    /// Connection timeout
    #[error("Connection timeout")]
    Timeout,

    /// Invalid UTF-8 data received
    #[error("Invalid UTF-8 data")]
    InvalidUtf8(#[from] std::string::FromUtf8Error),

    /// Buffer overflow
    #[error("Buffer overflow: maximum size {max} exceeded")]
    BufferOverflow { max: usize },
}
