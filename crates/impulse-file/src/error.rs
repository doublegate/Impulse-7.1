//! Error types for file area management

use thiserror::Error;

/// File area error type
#[derive(Debug, Error)]
pub enum FileError {
    /// File area not found
    #[error("File area {0} not found")]
    AreaNotFound(u32),

    /// File not found
    #[error("File {0} not found")]
    FileNotFound(u64),

    /// Permission denied
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    /// Invalid search criteria
    #[error("Invalid search criteria: {0}")]
    InvalidCriteria(String),

    /// File I/O error
    #[error("File I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// ZIP archive error
    #[error("ZIP archive error: {0}")]
    Zip(#[from] zip::result::ZipError),

    /// Invalid path
    #[error("Invalid path: {0}")]
    InvalidPath(String),

    /// Area path not configured
    #[error("Area path not configured for area {0}")]
    PathNotConfigured(u32),

    /// File too large
    #[error("File size {0} bytes exceeds limit of {1} bytes")]
    FileTooLarge(u64, u64),

    /// Duplicate file detected
    #[error("Duplicate file detected (hash: {0})")]
    DuplicateFile(String),

    /// Upload quota exceeded
    #[error("Upload quota exceeded: {0}")]
    QuotaExceeded(String),

    /// Extension not allowed
    #[error("File extension '.{0}' is not allowed")]
    ExtensionNotAllowed(String),

    /// Virus detected
    #[error("Virus detected: {0}")]
    VirusDetected(String),

    /// Upload not allowed in area
    #[error("Uploads not allowed in area {0}")]
    UploadNotAllowed(u32),

    /// Virus scanner unavailable
    #[error("Virus scanner unavailable: {0}")]
    ScannerUnavailable(String),

    /// Archive extraction error
    #[error("Failed to extract archive: {0}")]
    ArchiveError(String),
}

/// Result type for file area operations
pub type Result<T> = std::result::Result<T, FileError>;

impl From<impulse_types::Error> for FileError {
    fn from(err: impulse_types::Error) -> Self {
        match err {
            impulse_types::Error::Validation(msg) => FileError::InvalidPath(msg),
            _ => FileError::InvalidPath(format!("Type validation error: {}", err)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = FileError::AreaNotFound(42);
        assert_eq!(err.to_string(), "File area 42 not found");

        let err = FileError::FileNotFound(123);
        assert_eq!(err.to_string(), "File 123 not found");

        let err = FileError::PermissionDenied("Access denied".to_string());
        assert_eq!(err.to_string(), "Permission denied: Access denied");
    }
}
