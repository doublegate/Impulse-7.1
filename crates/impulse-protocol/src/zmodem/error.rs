//! Error types for Zmodem protocol operations.

use std::io;
use thiserror::Error;

/// Errors that can occur during Zmodem protocol operations.
#[derive(Debug, Error)]
pub enum ZmodemError {
    /// CRC checksum mismatch detected.
    #[error("CRC mismatch: expected {expected:08x}, got {actual:08x}")]
    CrcMismatch {
        /// Expected CRC value.
        expected: u32,
        /// Actual CRC value received.
        actual: u32,
    },

    /// Invalid frame type byte received.
    #[error("Invalid frame type: {0}")]
    InvalidFrameType(u8),

    /// Invalid frame encoding type.
    #[error("Invalid frame encoding: {0}")]
    InvalidFrameEncoding(u8),

    /// Timeout waiting for response from remote.
    #[error("Timeout waiting for response")]
    Timeout,

    /// Transfer cancelled by remote end.
    #[error("Transfer cancelled by remote")]
    Cancelled,

    /// Invalid ZDLE escape sequence encountered.
    #[error("Invalid ZDLE escape sequence")]
    InvalidEscape,

    /// Unexpected end of data stream.
    #[error("Unexpected end of data")]
    UnexpectedEof,

    /// Invalid frame format.
    #[error("Invalid frame format: {0}")]
    InvalidFrame(String),

    /// I/O error occurred.
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    /// Maximum retries exceeded.
    #[error("Maximum retries exceeded")]
    MaxRetriesExceeded,
}

/// Result type for Zmodem operations.
pub type Result<T> = std::result::Result<T, ZmodemError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = ZmodemError::CrcMismatch {
            expected: 0x12345678,
            actual: 0x87654321,
        };
        assert_eq!(
            err.to_string(),
            "CRC mismatch: expected 12345678, got 87654321"
        );

        let err = ZmodemError::InvalidFrameType(99);
        assert_eq!(err.to_string(), "Invalid frame type: 99");

        let err = ZmodemError::Timeout;
        assert_eq!(err.to_string(), "Timeout waiting for response");

        let err = ZmodemError::Cancelled;
        assert_eq!(err.to_string(), "Transfer cancelled by remote");

        let err = ZmodemError::InvalidEscape;
        assert_eq!(err.to_string(), "Invalid ZDLE escape sequence");
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = io::Error::new(io::ErrorKind::UnexpectedEof, "test error");
        let err: ZmodemError = io_err.into();
        assert!(matches!(err, ZmodemError::Io(_)));
    }
}
