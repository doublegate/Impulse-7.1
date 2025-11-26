//! Error types for session management

use thiserror::Error;

/// Result type alias for session operations
pub type Result<T> = std::result::Result<T, SessionError>;

/// Errors that can occur during session management
#[derive(Error, Debug)]
pub enum SessionError {
    /// Session not found
    #[error("Session not found: {0}")]
    NotFound(String),

    /// Session already exists
    #[error("Session already exists: {0}")]
    AlreadyExists(String),

    /// Session has expired
    #[error("Session expired: {0}")]
    Expired(String),

    /// Maximum sessions per user exceeded
    #[error("Maximum sessions per user exceeded (limit: {limit})")]
    TooManySessions { limit: usize },

    /// Session is not authenticated
    #[error("Session not authenticated")]
    NotAuthenticated,

    /// Invalid session state transition
    #[error("Invalid state transition from {from} to {to}")]
    InvalidStateTransition { from: String, to: String },

    /// Internal error
    #[error("Internal session error: {0}")]
    Internal(String),
}
