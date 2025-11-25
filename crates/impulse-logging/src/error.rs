//! Enhanced error reporting and formatting
//!
//! This module provides utilities for structured error reporting with context
//! and user-friendly error messages.

use std::error::Error as StdError;
use std::fmt;

/// Error context for enhanced error reporting
///
/// Provides additional context information for errors, including stack traces
/// and user-friendly descriptions.
#[derive(Debug, Clone)]
pub struct ErrorContext {
    /// User-friendly error message
    pub message: String,
    /// Technical details (for logs)
    pub details: Option<String>,
    /// Error code (for programmatic handling)
    pub code: Option<String>,
    /// Severity level
    pub severity: ErrorSeverity,
}

impl ErrorContext {
    /// Create a new error context
    ///
    /// # Examples
    ///
    /// ```rust
    /// use impulse_logging::ErrorContext;
    ///
    /// let ctx = ErrorContext::new("Failed to connect to database");
    /// ```
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            details: None,
            code: None,
            severity: ErrorSeverity::Error,
        }
    }

    /// Add technical details
    ///
    /// # Examples
    ///
    /// ```rust
    /// use impulse_logging::ErrorContext;
    ///
    /// let ctx = ErrorContext::new("Connection failed")
    ///     .with_details("Timeout after 30 seconds");
    /// ```
    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }

    /// Add an error code
    ///
    /// # Examples
    ///
    /// ```rust
    /// use impulse_logging::ErrorContext;
    ///
    /// let ctx = ErrorContext::new("Invalid input")
    ///     .with_code("INPUT_001");
    /// ```
    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    /// Set the severity level
    ///
    /// # Examples
    ///
    /// ```rust
    /// use impulse_logging::{ErrorContext, ErrorSeverity};
    ///
    /// let ctx = ErrorContext::new("Minor issue")
    ///     .with_severity(ErrorSeverity::Warning);
    /// ```
    pub fn with_severity(mut self, severity: ErrorSeverity) -> Self {
        self.severity = severity;
        self
    }
}

impl fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)?;
        if let Some(code) = &self.code {
            write!(f, " [{}]", code)?;
        }
        if let Some(details) = &self.details {
            write!(f, ": {}", details)?;
        }
        Ok(())
    }
}

/// Error severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorSeverity {
    /// Informational (not really an error)
    Info,
    /// Warning (potential issue)
    Warning,
    /// Error (operation failed)
    Error,
    /// Critical (system instability)
    Critical,
}

impl fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ErrorSeverity::Info => "INFO",
            ErrorSeverity::Warning => "WARNING",
            ErrorSeverity::Error => "ERROR",
            ErrorSeverity::Critical => "CRITICAL",
        };
        write!(f, "{}", s)
    }
}

/// Error reporter
///
/// Provides structured error reporting with context preservation.
pub struct ErrorReporter;

impl ErrorReporter {
    /// Report an error with context
    ///
    /// # Examples
    ///
    /// ```rust
    /// use impulse_logging::{ErrorReporter, ErrorContext};
    ///
    /// let ctx = ErrorContext::new("Database connection failed");
    /// ErrorReporter::report(&ctx);
    /// ```
    pub fn report(context: &ErrorContext) {
        match context.severity {
            ErrorSeverity::Info => {
                tracing::info!(
                    message = %context.message,
                    code = context.code.as_deref(),
                    details = context.details.as_deref(),
                    "ERROR_REPORT"
                );
            }
            ErrorSeverity::Warning => {
                tracing::warn!(
                    message = %context.message,
                    code = context.code.as_deref(),
                    details = context.details.as_deref(),
                    "ERROR_REPORT"
                );
            }
            ErrorSeverity::Error => {
                tracing::error!(
                    message = %context.message,
                    code = context.code.as_deref(),
                    details = context.details.as_deref(),
                    "ERROR_REPORT"
                );
            }
            ErrorSeverity::Critical => {
                tracing::error!(
                    message = %context.message,
                    code = context.code.as_deref(),
                    details = context.details.as_deref(),
                    severity = "CRITICAL",
                    "ERROR_REPORT"
                );
            }
        }
    }

    /// Report an error from a standard Error trait object
    ///
    /// # Examples
    ///
    /// ```rust
    /// use impulse_logging::ErrorReporter;
    /// use std::io;
    ///
    /// fn example() -> Result<(), io::Error> {
    ///     let err = io::Error::new(io::ErrorKind::NotFound, "File not found");
    ///     ErrorReporter::report_error(&err);
    ///     Ok(())
    /// }
    /// ```
    pub fn report_error(error: &dyn StdError) {
        let mut context = ErrorContext::new(error.to_string());

        // Add source chain if available
        if let Some(source) = error.source() {
            let details = Self::format_error_chain(source);
            context = context.with_details(details);
        }

        Self::report(&context);
    }

    /// Format an error chain into a string
    fn format_error_chain(error: &dyn StdError) -> String {
        let mut chain = Vec::new();
        let mut current = Some(error);

        while let Some(err) = current {
            chain.push(err.to_string());
            current = err.source();
        }

        chain.join(" -> ")
    }

    /// Report an anyhow::Error with full context
    ///
    /// # Examples
    ///
    /// ```rust
    /// use impulse_logging::ErrorReporter;
    /// use anyhow::Context;
    ///
    /// fn example() -> anyhow::Result<()> {
    ///     let result: Result<(), std::io::Error> = Err(std::io::Error::new(
    ///         std::io::ErrorKind::NotFound,
    ///         "File not found"
    ///     ));
    ///
    ///     let err = result.context("Failed to read config file").unwrap_err();
    ///     ErrorReporter::report_anyhow(&err);
    ///     Ok(())
    /// }
    /// ```
    pub fn report_anyhow(error: &anyhow::Error) {
        let message = error.to_string();
        let chain: Vec<String> = error.chain().skip(1).map(|e| e.to_string()).collect();

        let context = if chain.is_empty() {
            ErrorContext::new(message)
        } else {
            ErrorContext::new(message).with_details(chain.join(" -> "))
        };

        Self::report(&context);
    }

    /// Create a user-friendly error message
    ///
    /// Converts technical errors into messages suitable for end users.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use impulse_logging::ErrorReporter;
    /// use std::io;
    ///
    /// let err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    /// let message = ErrorReporter::user_friendly_message(&err);
    /// assert!(message.contains("not found"));
    /// ```
    pub fn user_friendly_message(error: &dyn StdError) -> String {
        let error_str = error.to_string().to_lowercase();

        if error_str.contains("not found") {
            "The requested resource was not found.".to_string()
        } else if error_str.contains("permission") || error_str.contains("access denied") {
            "Permission denied. Please check your access rights.".to_string()
        } else if error_str.contains("timeout") || error_str.contains("timed out") {
            "The operation timed out. Please try again.".to_string()
        } else if error_str.contains("connection") {
            "Connection failed. Please check your network connection.".to_string()
        } else if error_str.contains("parse") || error_str.contains("invalid") {
            "Invalid input or format. Please check your data.".to_string()
        } else {
            format!("An error occurred: {}", error)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_error_context_new() {
        let ctx = ErrorContext::new("Test error");
        assert_eq!(ctx.message, "Test error");
        assert!(ctx.details.is_none());
        assert!(ctx.code.is_none());
        assert_eq!(ctx.severity, ErrorSeverity::Error);
    }

    #[test]
    fn test_error_context_with_details() {
        let ctx = ErrorContext::new("Test error").with_details("Additional information");

        assert_eq!(ctx.details, Some("Additional information".to_string()));
    }

    #[test]
    fn test_error_context_with_code() {
        let ctx = ErrorContext::new("Test error").with_code("ERR_001");

        assert_eq!(ctx.code, Some("ERR_001".to_string()));
    }

    #[test]
    fn test_error_context_with_severity() {
        let ctx = ErrorContext::new("Test error").with_severity(ErrorSeverity::Critical);

        assert_eq!(ctx.severity, ErrorSeverity::Critical);
    }

    #[test]
    fn test_error_context_display() {
        let ctx = ErrorContext::new("Test error")
            .with_code("ERR_001")
            .with_details("Extra info");

        let display = ctx.to_string();
        assert!(display.contains("Test error"));
        assert!(display.contains("ERR_001"));
        assert!(display.contains("Extra info"));
    }

    #[test]
    fn test_error_severity_ordering() {
        assert!(ErrorSeverity::Info < ErrorSeverity::Warning);
        assert!(ErrorSeverity::Warning < ErrorSeverity::Error);
        assert!(ErrorSeverity::Error < ErrorSeverity::Critical);
    }

    #[test]
    fn test_error_severity_display() {
        assert_eq!(ErrorSeverity::Info.to_string(), "INFO");
        assert_eq!(ErrorSeverity::Warning.to_string(), "WARNING");
        assert_eq!(ErrorSeverity::Error.to_string(), "ERROR");
        assert_eq!(ErrorSeverity::Critical.to_string(), "CRITICAL");
    }

    #[test]
    fn test_error_reporter_report() {
        let ctx = ErrorContext::new("Test error");
        ErrorReporter::report(&ctx);
        // Should not panic
    }

    #[test]
    fn test_error_reporter_report_error() {
        let err = io::Error::new(io::ErrorKind::NotFound, "File not found");
        ErrorReporter::report_error(&err);
        // Should not panic
    }

    #[test]
    fn test_error_reporter_user_friendly_message() {
        let err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let message = ErrorReporter::user_friendly_message(&err);
        assert!(message.contains("not found"));

        let err = io::Error::new(io::ErrorKind::PermissionDenied, "permission denied");
        let message = ErrorReporter::user_friendly_message(&err);
        assert!(message.contains("Permission"));

        let err = io::Error::new(io::ErrorKind::TimedOut, "connection timeout");
        let message = ErrorReporter::user_friendly_message(&err);
        assert!(message.contains("timed out"));
    }

    #[test]
    fn test_error_context_builder_chaining() {
        let ctx = ErrorContext::new("Test error")
            .with_code("ERR_001")
            .with_details("Details")
            .with_severity(ErrorSeverity::Warning);

        assert_eq!(ctx.message, "Test error");
        assert_eq!(ctx.code, Some("ERR_001".to_string()));
        assert_eq!(ctx.details, Some("Details".to_string()));
        assert_eq!(ctx.severity, ErrorSeverity::Warning);
    }
}
