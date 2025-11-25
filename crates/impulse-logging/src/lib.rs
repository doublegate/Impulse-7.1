//! Structured logging infrastructure for Impulse BBS
//!
//! This crate provides a comprehensive logging system built on the `tracing` ecosystem,
//! featuring file rotation, log archival, security audit logging, and error reporting.
//!
//! # Features
//!
//! - **Structured Logging**: JSON and human-readable formats
//! - **File Rotation**: Size-based and time-based rotation policies
//! - **Log Archival**: Automatic compression and retention management
//! - **Audit Logging**: Tamper-evident security event tracking
//! - **Error Reporting**: Structured error formatting with context
//! - **Multi-Output**: File, stdout, stderr, and syslog support
//!
//! # Quick Start
//!
//! ```rust
//! use impulse_logging::{LoggerBuilder, LogLevel, LogOutput, RotationPolicy};
//! use anyhow::Result;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     // Initialize logging with default settings
//!     LoggerBuilder::new()
//!         .with_level(LogLevel::Info)
//!         .with_output(LogOutput::File("logs/bbs.log".into()))
//!         .with_rotation(RotationPolicy::Daily)
//!         .with_max_files(30)
//!         .build()?;
//!
//!     // Use throughout application
//!     tracing::info!(user_id = 42, "User logged in");
//!     tracing::error!(error = "auth failed", "Authentication failed");
//!
//!     Ok(())
//! }
//! ```
//!
//! # Architecture
//!
//! The logging system provides several key components:
//!
//! - **Subscriber Configuration**: [`LoggerBuilder`] for tracing subscriber setup
//! - **File Rotation**: [`RotationPolicy`] and [`RotationManager`] for log file management
//! - **Log Archival**: [`ArchiveManager`] for compression and retention
//! - **Security Auditing**: [`AuditLogger`] for tamper-evident event tracking
//! - **Error Reporting**: [`ErrorReporter`] for structured error formatting
//!
//! # Integration
//!
//! See the type documentation for integration examples with impulse-auth,
//! impulse-user, and impulse-config crates.

mod archival;
mod audit;
mod error;
mod rotation;
mod subscriber;

pub use archival::{ArchivalConfig, ArchiveManager};
pub use audit::{AuditEvent, AuditEventType, AuditLogger};
pub use error::{ErrorContext, ErrorReporter, ErrorSeverity};
pub use rotation::{RotationManager, RotationPolicy, RotationTrigger};
pub use subscriber::{LogFormat, LogLevel, LogOutput, LoggerBuilder};

/// Result type alias using anyhow::Error
pub type Result<T> = anyhow::Result<T>;

/// Initialize logging with sensible defaults for console output
///
/// This is a convenience function for quick setup during development.
/// For production, use [`LoggerBuilder`] for more control.
///
/// # Examples
///
/// ```rust
/// use impulse_logging::init_console_logging;
///
/// # fn main() -> anyhow::Result<()> {
/// init_console_logging()?;
/// tracing::info!("Logging initialized");
/// # Ok(())
/// # }
/// ```
pub fn init_console_logging() -> Result<()> {
    LoggerBuilder::new()
        .with_level(LogLevel::Info)
        .with_output(LogOutput::Stdout)
        .with_format(LogFormat::Human)
        .build()
}

/// Initialize logging with sensible defaults for file output
///
/// Creates a log file with daily rotation and 30-day retention.
///
/// # Examples
///
/// ```rust,no_run
/// use impulse_logging::init_file_logging;
///
/// # fn main() -> anyhow::Result<()> {
/// init_file_logging("logs/bbs.log")?;
/// tracing::info!("Logging to file initialized");
/// # Ok(())
/// # }
/// ```
pub fn init_file_logging(path: &str) -> Result<()> {
    LoggerBuilder::new()
        .with_level(LogLevel::Info)
        .with_output(LogOutput::File(path.into()))
        .with_rotation(RotationPolicy::Daily)
        .with_max_files(30)
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_console_logging() {
        // Initialize should succeed
        let result = init_console_logging();
        assert!(result.is_ok(), "Console logging init failed: {:?}", result);
    }

    #[test]
    fn test_logger_builder_defaults() {
        let builder = LoggerBuilder::new();
        assert_eq!(builder.level(), LogLevel::Info);
        assert_eq!(builder.format(), LogFormat::Human);
    }

    #[test]
    fn test_logger_builder_with_level() {
        let builder = LoggerBuilder::new().with_level(LogLevel::Debug);
        assert_eq!(builder.level(), LogLevel::Debug);
    }

    #[test]
    fn test_logger_builder_with_format() {
        let builder = LoggerBuilder::new().with_format(LogFormat::Json);
        assert_eq!(builder.format(), LogFormat::Json);
    }

    #[test]
    fn test_log_level_ordering() {
        assert!(LogLevel::Trace < LogLevel::Debug);
        assert!(LogLevel::Debug < LogLevel::Info);
        assert!(LogLevel::Info < LogLevel::Warn);
        assert!(LogLevel::Warn < LogLevel::Error);
    }
}
