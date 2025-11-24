//! Configuration error types

use std::path::PathBuf;
use thiserror::Error;

/// Configuration operation errors
#[derive(Debug, Error)]
pub enum ConfigError {
    /// Failed to load configuration from file or environment
    #[error("Failed to load configuration: {0}")]
    LoadError(String),

    /// Configuration validation failed
    #[error("Validation failed: {0}")]
    ValidationError(String),

    /// Failed to save configuration to file
    #[error("Failed to save configuration: {0}")]
    SaveError(String),

    /// Required path does not exist
    #[error("Path does not exist: {}", .0.display())]
    PathNotFound(PathBuf),

    /// Network port is already in use
    #[error("Port {0} is already in use or unavailable")]
    PortInUse(u16),

    /// Invalid configuration value
    #[error("Invalid configuration value: {0}")]
    InvalidValue(String),

    /// I/O error
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// TOML serialization error
    #[error("TOML serialization error: {0}")]
    TomlSerError(#[from] toml::ser::Error),

    /// TOML deserialization error
    #[error("TOML deserialization error: {0}")]
    TomlDeError(#[from] toml::de::Error),

    /// Figment error (configuration loading/merging, boxed to reduce size)
    #[error("Configuration loading error: {0}")]
    FigmentError(Box<figment::Error>),

    /// Error from BbsConfig validation
    #[error("BBS configuration error: {0}")]
    BbsError(#[from] impulse_types::Error),
}

/// Result type for configuration operations
pub type Result<T> = std::result::Result<T, ConfigError>;

// Manual From implementation for boxed figment::Error
impl From<figment::Error> for ConfigError {
    fn from(err: figment::Error) -> Self {
        ConfigError::FigmentError(Box::new(err))
    }
}
