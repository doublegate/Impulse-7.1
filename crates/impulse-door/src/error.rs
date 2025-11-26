//! Error types for the door game interface.
//!
//! This module provides comprehensive error handling for all door-related operations,
//! including dropfile generation, door execution, and session management.

use std::path::PathBuf;

/// Result type alias for door operations.
pub type Result<T> = std::result::Result<T, DoorError>;

/// Errors that can occur during door operations.
#[derive(Debug, thiserror::Error)]
pub enum DoorError {
    /// Door not found in the configured door list.
    #[error("Door not found: {0}")]
    DoorNotFound(String),

    /// DOSBox executable not found at the specified path.
    #[error("DOSBox not found at path: {0}")]
    DosBoxNotFound(PathBuf),

    /// Failed to create a dropfile.
    #[error("Failed to create dropfile: {0}")]
    DropfileCreation(String),

    /// Door execution failed.
    #[error("Door execution failed: {0}")]
    ExecutionFailed(String),

    /// Standard I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Configuration error.
    #[error("Configuration error: {0}")]
    Config(String),

    /// Process execution timeout.
    #[error("Process timeout after {0} seconds")]
    Timeout(u64),

    /// User's time limit expired.
    #[error("User time expired")]
    TimeExpired,

    /// Invalid door configuration.
    #[error("Invalid door configuration: {0}")]
    InvalidConfig(String),

    /// Door executable not found.
    #[error("Door executable not found: {0}")]
    ExecutableNotFound(PathBuf),

    /// Door directory not found.
    #[error("Door directory not found: {0}")]
    DirectoryNotFound(PathBuf),

    /// Failed to parse TOML configuration.
    #[error("Failed to parse door configuration: {0}")]
    TomlParse(#[from] toml::de::Error),

    /// Failed to serialize TOML configuration.
    #[error("Failed to serialize door configuration: {0}")]
    TomlSerialize(#[from] toml::ser::Error),

    /// Insufficient security level to access door.
    #[error("Insufficient security level: required {required}, user has {actual}")]
    InsufficientSecurity { required: u8, actual: u8 },

    /// Node directory already in use.
    #[error("Node directory {0} is already in use")]
    NodeInUse(u16),

    /// Failed to acquire node lock.
    #[error("Failed to acquire node lock: {0}")]
    NodeLockFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_door_not_found_error() {
        let error = DoorError::DoorNotFound("test-door".to_string());
        assert_eq!(error.to_string(), "Door not found: test-door");
    }

    #[test]
    fn test_dosbox_not_found_error() {
        let path = PathBuf::from("/usr/bin/dosbox");
        let error = DoorError::DosBoxNotFound(path.clone());
        assert_eq!(
            error.to_string(),
            format!("DOSBox not found at path: {}", path.display())
        );
    }

    #[test]
    fn test_dropfile_creation_error() {
        let error = DoorError::DropfileCreation("Invalid format".to_string());
        assert_eq!(
            error.to_string(),
            "Failed to create dropfile: Invalid format"
        );
    }

    #[test]
    fn test_execution_failed_error() {
        let error = DoorError::ExecutionFailed("Process exited with code 1".to_string());
        assert_eq!(
            error.to_string(),
            "Door execution failed: Process exited with code 1"
        );
    }

    #[test]
    fn test_config_error() {
        let error = DoorError::Config("Missing field 'name'".to_string());
        assert_eq!(
            error.to_string(),
            "Configuration error: Missing field 'name'"
        );
    }

    #[test]
    fn test_timeout_error() {
        let error = DoorError::Timeout(300);
        assert_eq!(error.to_string(), "Process timeout after 300 seconds");
    }

    #[test]
    fn test_time_expired_error() {
        let error = DoorError::TimeExpired;
        assert_eq!(error.to_string(), "User time expired");
    }

    #[test]
    fn test_invalid_config_error() {
        let error = DoorError::InvalidConfig("Empty door name".to_string());
        assert_eq!(
            error.to_string(),
            "Invalid door configuration: Empty door name"
        );
    }

    #[test]
    fn test_executable_not_found_error() {
        let path = PathBuf::from("/doors/game.exe");
        let error = DoorError::ExecutableNotFound(path.clone());
        assert_eq!(
            error.to_string(),
            format!("Door executable not found: {}", path.display())
        );
    }

    #[test]
    fn test_directory_not_found_error() {
        let path = PathBuf::from("/doors/game");
        let error = DoorError::DirectoryNotFound(path.clone());
        assert_eq!(
            error.to_string(),
            format!("Door directory not found: {}", path.display())
        );
    }

    #[test]
    fn test_insufficient_security_error() {
        let error = DoorError::InsufficientSecurity {
            required: 100,
            actual: 50,
        };
        assert_eq!(
            error.to_string(),
            "Insufficient security level: required 100, user has 50"
        );
    }

    #[test]
    fn test_node_in_use_error() {
        let error = DoorError::NodeInUse(5);
        assert_eq!(error.to_string(), "Node directory 5 is already in use");
    }

    #[test]
    fn test_node_lock_failed_error() {
        let error = DoorError::NodeLockFailed("Permission denied".to_string());
        assert_eq!(
            error.to_string(),
            "Failed to acquire node lock: Permission denied"
        );
    }

    #[test]
    fn test_error_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<DoorError>();
    }

    #[test]
    fn test_io_error_conversion() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let door_error: DoorError = io_error.into();
        assert!(matches!(door_error, DoorError::Io(_)));
    }
}
