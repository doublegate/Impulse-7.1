//! Error types for the menu system

use std::path::PathBuf;
use thiserror::Error;

/// Errors that can occur during menu parsing
#[derive(Debug, Error)]
pub enum MenuParseError {
    /// Failed to read menu file
    #[error("Failed to read menu file {path}: {source}")]
    FileRead {
        path: PathBuf,
        source: std::io::Error,
    },

    /// Failed to parse TOML
    #[error("Failed to parse menu TOML from {path}: {source}")]
    TomlParse {
        path: PathBuf,
        source: toml::de::Error,
    },

    /// Invalid menu structure
    #[error("Invalid menu structure in {path}: {message}")]
    InvalidStructure { path: PathBuf, message: String },
}

/// Validation errors for menu definitions
#[derive(Debug, Error)]
pub enum ValidationError {
    /// Menu name is empty
    #[error("Menu name cannot be empty")]
    EmptyMenuName,

    /// Menu title is empty
    #[error("Menu title cannot be empty")]
    EmptyMenuTitle,

    /// Option has empty key
    #[error("Menu option has empty key")]
    EmptyOptionKey,

    /// Option has empty command
    #[error("Menu option has empty command")]
    EmptyOptionCommand,

    /// Option has empty description
    #[error("Menu option has empty description")]
    EmptyOptionDescription,

    /// Duplicate option keys
    #[error("Duplicate option key: {key}")]
    DuplicateOptionKey { key: String },

    /// Invalid security level range
    #[error("Invalid security level range: min={min}, max={max}")]
    InvalidSecurityRange { min: u8, max: u8 },

    /// Referenced menu not found
    #[error("Referenced menu not found: {menu}")]
    MenuNotFound { menu: String },
}

/// Errors that can occur during menu loading
#[derive(Debug, Error)]
pub enum MenuLoadError {
    /// Parse error
    #[error("Parse error: {0}")]
    Parse(#[from] MenuParseError),

    /// Validation error
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),

    /// Multiple validation errors
    #[error("Multiple validation errors: {errors:?}")]
    MultipleValidation { errors: Vec<ValidationError> },

    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// Errors that can occur during navigation
#[derive(Debug, Error)]
pub enum NavigationError {
    /// Menu not found
    #[error("Menu not found: {menu}")]
    MenuNotFound { menu: String },

    /// Cannot go back from main menu
    #[error("Cannot go back from main menu")]
    CannotGoBack,

    /// Empty menu stack
    #[error("Menu stack is empty")]
    EmptyStack,
}

/// Errors that can occur during command routing
#[derive(Debug, Error)]
pub enum CommandError {
    /// Unknown command
    #[error("Unknown command: {command}")]
    UnknownCommand { command: String },

    /// Command execution failed
    #[error("Command execution failed: {source}")]
    ExecutionFailed {
        #[from]
        source: anyhow::Error,
    },

    /// Insufficient privileges
    #[error("Insufficient privileges to execute command: {command}")]
    InsufficientPrivileges { command: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menu_parse_error_display() {
        let err = MenuParseError::FileRead {
            path: PathBuf::from("/test/menu.toml"),
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "file not found"),
        };
        assert!(err.to_string().contains("/test/menu.toml"));
    }

    #[test]
    fn test_validation_error_display() {
        let err = ValidationError::DuplicateOptionKey {
            key: "F".to_string(),
        };
        assert!(err.to_string().contains("F"));
    }

    #[test]
    fn test_navigation_error_display() {
        let err = NavigationError::MenuNotFound {
            menu: "nonexistent".to_string(),
        };
        assert!(err.to_string().contains("nonexistent"));
    }

    #[test]
    fn test_command_error_display() {
        let err = CommandError::UnknownCommand {
            command: "invalid".to_string(),
        };
        assert!(err.to_string().contains("invalid"));
    }
}
