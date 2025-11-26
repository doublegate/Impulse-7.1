//! Error types for the administration interface

use thiserror::Error;

/// Errors that can occur during administrative operations
#[derive(Debug, Error)]
pub enum AdminError {
    /// Access denied - insufficient permissions
    #[error("Access denied: {0}")]
    AccessDenied(String),

    /// User not found
    #[error("User not found: {0}")]
    UserNotFound(i32),

    /// File area not found
    #[error("File area not found: {0}")]
    FileAreaNotFound(i32),

    /// Session not found
    #[error("Session not found: {0}")]
    SessionNotFound(uuid::Uuid),

    /// Invalid security level
    #[error("Invalid security level: {0}")]
    InvalidSecurityLevel(u8),

    /// Invalid input
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Audit log failure
    #[error("Failed to log audit entry: {0}")]
    AuditLogFailed(String),

    /// Broadcast failed
    #[error("Broadcast failed: {0}")]
    BroadcastFailed(String),

    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Unexpected error
    #[error("Unexpected error: {0}")]
    Unexpected(String),
}

/// Result type for administrative operations
pub type AdminResult<T> = Result<T, AdminError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_denied_error() {
        let err = AdminError::AccessDenied("test".to_string());
        assert_eq!(err.to_string(), "Access denied: test");
    }

    #[test]
    fn test_user_not_found_error() {
        let err = AdminError::UserNotFound(42);
        assert_eq!(err.to_string(), "User not found: 42");
    }

    #[test]
    fn test_file_area_not_found_error() {
        let err = AdminError::FileAreaNotFound(99);
        assert_eq!(err.to_string(), "File area not found: 99");
    }

    #[test]
    fn test_session_not_found_error() {
        let session_id = uuid::Uuid::new_v4();
        let err = AdminError::SessionNotFound(session_id);
        assert!(err.to_string().contains("Session not found"));
    }

    #[test]
    fn test_invalid_security_level() {
        let err = AdminError::InvalidSecurityLevel(99);
        assert_eq!(err.to_string(), "Invalid security level: 99");
    }

    #[test]
    fn test_invalid_input() {
        let err = AdminError::InvalidInput("empty username".to_string());
        assert_eq!(err.to_string(), "Invalid input: empty username");
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let admin_err: AdminError = io_err.into();
        assert!(admin_err.to_string().contains("I/O error"));
    }
}
