//! QWK-specific error types

use std::io;
use thiserror::Error;

/// Errors specific to QWK packet operations
#[derive(Debug, Error)]
pub enum QwkError {
    /// I/O error during QWK operations
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    /// ZIP compression/decompression error
    #[error("ZIP error: {0}")]
    Zip(#[from] zip::result::ZipError),

    /// Invalid QWK packet format
    #[error("Invalid QWK packet format: {0}")]
    InvalidFormat(String),

    /// Invalid message block size
    #[error("Invalid message block size: expected 128 bytes, got {0}")]
    InvalidBlockSize(usize),

    /// Missing required file in QWK packet
    #[error("Missing required file in QWK packet: {0}")]
    MissingFile(String),

    /// Invalid conference number
    #[error("Invalid conference number: {0}")]
    InvalidConference(u16),

    /// Message header parsing error
    #[error("Message header parsing error: {0}")]
    HeaderParse(String),

    /// Reply packet parsing error
    #[error("Reply packet parsing error: {0}")]
    ReplyParse(String),

    /// Encoding error
    #[error("Encoding error: {0}")]
    Encoding(String),
}

/// QWK-specific result type
pub type Result<T> = std::result::Result<T, QwkError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = QwkError::InvalidFormat("bad header".to_string());
        assert_eq!(err.to_string(), "Invalid QWK packet format: bad header");

        let err = QwkError::InvalidBlockSize(64);
        assert_eq!(
            err.to_string(),
            "Invalid message block size: expected 128 bytes, got 64"
        );

        let err = QwkError::MissingFile("CONTROL.DAT".to_string());
        assert_eq!(
            err.to_string(),
            "Missing required file in QWK packet: CONTROL.DAT"
        );
    }

    #[test]
    fn test_error_from_io() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let qwk_err: QwkError = io_err.into();
        assert!(matches!(qwk_err, QwkError::Io(_)));
    }
}
