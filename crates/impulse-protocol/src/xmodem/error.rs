//! Error types for Xmodem protocol operations.

use std::io;
use thiserror::Error;

/// Errors that can occur during Xmodem protocol operations.
#[derive(Debug, Error)]
pub enum XmodemError {
    /// Checksum mismatch detected.
    #[error("Checksum mismatch: expected {expected:02x}, got {actual:02x}")]
    ChecksumMismatch {
        /// Expected checksum value.
        expected: u8,
        /// Actual checksum value received.
        actual: u8,
    },

    /// CRC-16 mismatch detected.
    #[error("CRC mismatch: expected {expected:04x}, got {actual:04x}")]
    CrcMismatch {
        /// Expected CRC value.
        expected: u16,
        /// Actual CRC value received.
        actual: u16,
    },

    /// Invalid block number received.
    #[error("Invalid block number: expected {expected}, got {actual}")]
    InvalidBlockNumber {
        /// Expected block number.
        expected: u8,
        /// Actual block number received.
        actual: u8,
    },

    /// Block number complement mismatch.
    #[error("Block number complement mismatch: block={block}, complement={complement}")]
    ComplementMismatch {
        /// Block number.
        block: u8,
        /// Complement of block number.
        complement: u8,
    },

    /// Invalid block header byte received.
    #[error("Invalid block header: {0:02x}")]
    InvalidBlockHeader(u8),

    /// Timeout waiting for response from remote.
    #[error("Timeout waiting for response")]
    Timeout,

    /// Transfer cancelled by remote end.
    #[error("Transfer cancelled by remote")]
    Cancelled,

    /// Unexpected end of data stream.
    #[error("Unexpected end of data")]
    UnexpectedEof,

    /// I/O error occurred.
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    /// Maximum retries exceeded.
    #[error("Maximum retries exceeded (tried {attempts} times)")]
    MaxRetriesExceeded {
        /// Number of retry attempts made.
        attempts: usize,
    },

    /// Synchronization error - lost sync with sender.
    #[error("Synchronization lost with remote")]
    SyncError,

    /// Invalid variant configuration.
    #[error("Invalid variant: {0}")]
    InvalidVariant(String),
}

/// Result type for Xmodem operations.
pub type Result<T> = std::result::Result<T, XmodemError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = XmodemError::ChecksumMismatch {
            expected: 0x42,
            actual: 0x24,
        };
        assert_eq!(err.to_string(), "Checksum mismatch: expected 42, got 24");

        let err = XmodemError::CrcMismatch {
            expected: 0x1234,
            actual: 0x5678,
        };
        assert_eq!(err.to_string(), "CRC mismatch: expected 1234, got 5678");

        let err = XmodemError::InvalidBlockNumber {
            expected: 1,
            actual: 3,
        };
        assert_eq!(err.to_string(), "Invalid block number: expected 1, got 3");

        let err = XmodemError::ComplementMismatch {
            block: 5,
            complement: 250,
        };
        assert!(err.to_string().contains("5"));
        assert!(err.to_string().contains("250"));
    }

    #[test]
    fn test_timeout_error() {
        let err = XmodemError::Timeout;
        assert_eq!(err.to_string(), "Timeout waiting for response");
    }

    #[test]
    fn test_cancelled_error() {
        let err = XmodemError::Cancelled;
        assert_eq!(err.to_string(), "Transfer cancelled by remote");
    }

    #[test]
    fn test_max_retries_error() {
        let err = XmodemError::MaxRetriesExceeded { attempts: 10 };
        assert!(err.to_string().contains("10"));
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = io::Error::new(io::ErrorKind::UnexpectedEof, "test error");
        let err: XmodemError = io_err.into();
        assert!(matches!(err, XmodemError::Io(_)));
    }

    #[test]
    fn test_sync_error() {
        let err = XmodemError::SyncError;
        assert!(err.to_string().contains("Synchronization"));
    }

    #[test]
    fn test_invalid_block_header() {
        let err = XmodemError::InvalidBlockHeader(0xFF);
        assert!(err.to_string().contains("ff"));
    }
}
