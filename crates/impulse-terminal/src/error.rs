//! Error types for terminal operations

use std::io;
use thiserror::Error;

/// Result type alias for terminal operations
pub type Result<T> = std::result::Result<T, TerminalError>;

/// Errors that can occur during terminal operations
#[derive(Error, Debug)]
pub enum TerminalError {
    /// I/O error occurred
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    /// Invalid ANSI sequence
    #[error("Invalid ANSI sequence: {0}")]
    InvalidSequence(String),

    /// Invalid color code
    #[error("Invalid color code: {0}")]
    InvalidColor(u8),

    /// Unsupported terminal capability
    #[error("Unsupported terminal capability: {0}")]
    Unsupported(String),

    /// Parse error
    #[error("Parse error: {0}")]
    Parse(String),
}
