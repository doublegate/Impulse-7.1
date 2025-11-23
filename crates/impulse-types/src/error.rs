//! Error types for Impulse 7.1 BBS
//!
//! This module provides a unified error handling framework using `thiserror`
//! for consistent error reporting across all crates in the workspace.

use thiserror::Error;

/// Unified error type for Impulse 7.1 BBS operations
///
/// This error type covers all expected failure modes across the entire BBS system,
/// from validation errors to I/O failures, database errors, and network issues.
///
/// # Examples
///
/// ```
/// use impulse_types::error::{Error, Result};
///
/// fn validate_username(name: &str) -> Result<()> {
///     if name.is_empty() {
///         return Err(Error::Validation("Username cannot be empty".to_string()));
///     }
///     if name.len() > 30 {
///         return Err(Error::Validation("Username too long (max 30 chars)".to_string()));
///     }
///     Ok(())
/// }
/// ```
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// Validation error - invalid input data
    #[error("Validation failed: {0}")]
    Validation(String),

    /// I/O error - file system operations
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Database error - storage layer failures
    #[error("Database error: {0}")]
    Database(String),

    /// Authentication error - login/access control failures
    #[error("Authentication failed: {0}")]
    Authentication(String),

    /// Configuration error - invalid or missing configuration
    #[error("Configuration error: {0}")]
    Config(String),

    /// Network error - protocol and connection failures
    #[error("Network error: {0}")]
    Network(String),

    /// Session error - session management failures
    #[error("Session error: {0}")]
    Session(String),

    /// Protocol error - protocol parsing/handling failures
    #[error("Protocol error: {0}")]
    Protocol(String),

    /// File area error - file management failures
    #[error("File area error: {0}")]
    FileArea(String),

    /// Message area error - message base failures
    #[error("Message area error: {0}")]
    MessageArea(String),

    /// User management error - user record failures
    #[error("User management error: {0}")]
    UserManagement(String),

    /// Permission error - access denied
    #[error("Permission denied: {0}")]
    Permission(String),

    /// Resource not found
    #[error("Not found: {0}")]
    NotFound(String),

    /// Resource already exists
    #[error("Already exists: {0}")]
    AlreadyExists(String),

    /// Operation timed out
    #[error("Operation timed out: {0}")]
    Timeout(String),

    /// Internal error - unexpected system state
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result type alias using the unified Error type
///
/// This provides a convenient shorthand for functions that may fail with
/// any of the error variants defined in [`Error`].
///
/// # Examples
///
/// ```
/// use impulse_types::error::Result;
///
/// fn do_something() -> Result<String> {
///     Ok("success".to_string())
/// }
/// ```
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_error() {
        let err = Error::Validation("test error".to_string());
        assert!(err.to_string().contains("Validation failed"));
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err: Error = io_err.into();
        assert!(err.to_string().contains("I/O error"));
    }

    #[test]
    fn test_result_type() {
        fn success() -> Result<i32> {
            Ok(42)
        }

        fn failure() -> Result<i32> {
            Err(Error::Validation("bad input".to_string()))
        }

        assert!(success().is_ok());
        assert!(failure().is_err());
    }
}
