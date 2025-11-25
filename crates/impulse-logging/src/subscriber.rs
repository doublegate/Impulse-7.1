//! Tracing subscriber configuration and initialization
//!
//! This module provides the [`LoggerBuilder`] API for configuring and initializing
//! the logging system with various output targets, formats, and filtering options.

use anyhow::{Context, Result};
use std::path::PathBuf;
use tracing::Level;
use tracing_subscriber::{
    EnvFilter,
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

/// Log level configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    /// Trace-level logging (most verbose)
    Trace,
    /// Debug-level logging
    Debug,
    /// Info-level logging (default)
    Info,
    /// Warning-level logging
    Warn,
    /// Error-level logging (least verbose)
    Error,
}

impl LogLevel {
    /// Convert to tracing::Level
    pub fn as_tracing_level(&self) -> Level {
        match self {
            LogLevel::Trace => Level::TRACE,
            LogLevel::Debug => Level::DEBUG,
            LogLevel::Info => Level::INFO,
            LogLevel::Warn => Level::WARN,
            LogLevel::Error => Level::ERROR,
        }
    }

    /// Convert to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Trace => "trace",
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
        }
    }
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Log output format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogFormat {
    /// Human-readable format with colors (default for console)
    Human,
    /// JSON format (default for files)
    Json,
    /// Compact format (minimal output)
    Compact,
}

/// Log output destination
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogOutput {
    /// Standard output
    Stdout,
    /// Standard error
    Stderr,
    /// File output with path
    File(PathBuf),
}

/// Builder for configuring and initializing the logging system
///
/// # Examples
///
/// ```rust
/// use impulse_logging::{LoggerBuilder, LogLevel, LogOutput, LogFormat};
///
/// # fn main() -> anyhow::Result<()> {
/// let logger = LoggerBuilder::new()
///     .with_level(LogLevel::Debug)
///     .with_format(LogFormat::Json)
///     .with_output(LogOutput::Stdout)
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct LoggerBuilder {
    level: LogLevel,
    format: LogFormat,
    output: LogOutput,
    rotation_policy: Option<super::RotationPolicy>,
    max_files: usize,
    show_target: bool,
    show_thread_ids: bool,
    show_line_numbers: bool,
}

impl Default for LoggerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl LoggerBuilder {
    /// Create a new logger builder with default settings
    ///
    /// Defaults:
    /// - Level: Info
    /// - Format: Human
    /// - Output: Stdout
    /// - Max files: 10
    /// - Show target: true
    /// - Show thread IDs: false
    /// - Show line numbers: true
    pub fn new() -> Self {
        Self {
            level: LogLevel::Info,
            format: LogFormat::Human,
            output: LogOutput::Stdout,
            rotation_policy: None,
            max_files: 10,
            show_target: true,
            show_thread_ids: false,
            show_line_numbers: true,
        }
    }

    /// Set the log level
    ///
    /// # Examples
    ///
    /// ```rust
    /// use impulse_logging::{LoggerBuilder, LogLevel};
    ///
    /// let builder = LoggerBuilder::new().with_level(LogLevel::Debug);
    /// ```
    pub fn with_level(mut self, level: LogLevel) -> Self {
        self.level = level;
        self
    }

    /// Set the log format
    ///
    /// # Examples
    ///
    /// ```rust
    /// use impulse_logging::{LoggerBuilder, LogFormat};
    ///
    /// let builder = LoggerBuilder::new().with_format(LogFormat::Json);
    /// ```
    pub fn with_format(mut self, format: LogFormat) -> Self {
        self.format = format;
        self
    }

    /// Set the log output destination
    ///
    /// # Examples
    ///
    /// ```rust
    /// use impulse_logging::{LoggerBuilder, LogOutput};
    ///
    /// let builder = LoggerBuilder::new()
    ///     .with_output(LogOutput::File("logs/bbs.log".into()));
    /// ```
    pub fn with_output(mut self, output: LogOutput) -> Self {
        self.output = output;
        self
    }

    /// Set the file rotation policy
    ///
    /// Only applicable when output is a file.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use impulse_logging::{LoggerBuilder, LogOutput, RotationPolicy};
    ///
    /// let builder = LoggerBuilder::new()
    ///     .with_output(LogOutput::File("logs/bbs.log".into()))
    ///     .with_rotation(RotationPolicy::Daily);
    /// ```
    pub fn with_rotation(mut self, policy: super::RotationPolicy) -> Self {
        self.rotation_policy = Some(policy);
        self
    }

    /// Set the maximum number of rotated log files to keep
    ///
    /// # Examples
    ///
    /// ```rust
    /// use impulse_logging::LoggerBuilder;
    ///
    /// let builder = LoggerBuilder::new().with_max_files(30);
    /// ```
    pub fn with_max_files(mut self, max_files: usize) -> Self {
        self.max_files = max_files;
        self
    }

    /// Enable or disable showing the target module path
    ///
    /// # Examples
    ///
    /// ```rust
    /// use impulse_logging::LoggerBuilder;
    ///
    /// let builder = LoggerBuilder::new().with_show_target(false);
    /// ```
    pub fn with_show_target(mut self, show: bool) -> Self {
        self.show_target = show;
        self
    }

    /// Enable or disable showing thread IDs
    ///
    /// # Examples
    ///
    /// ```rust
    /// use impulse_logging::LoggerBuilder;
    ///
    /// let builder = LoggerBuilder::new().with_show_thread_ids(true);
    /// ```
    pub fn with_show_thread_ids(mut self, show: bool) -> Self {
        self.show_thread_ids = show;
        self
    }

    /// Enable or disable showing line numbers
    ///
    /// # Examples
    ///
    /// ```rust
    /// use impulse_logging::LoggerBuilder;
    ///
    /// let builder = LoggerBuilder::new().with_show_line_numbers(false);
    /// ```
    pub fn with_show_line_numbers(mut self, show: bool) -> Self {
        self.show_line_numbers = show;
        self
    }

    /// Get the configured log level
    pub fn level(&self) -> LogLevel {
        self.level
    }

    /// Get the configured log format
    pub fn format(&self) -> LogFormat {
        self.format
    }

    /// Get the configured log output
    pub fn output(&self) -> &LogOutput {
        &self.output
    }

    /// Build and initialize the logging system
    ///
    /// This consumes the builder and initializes the global tracing subscriber.
    /// Can only be called once per process.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The subscriber has already been initialized
    /// - File output path is invalid or cannot be created
    /// - Rotation policy is specified for non-file output
    ///
    /// # Examples
    ///
    /// ```rust
    /// use impulse_logging::{LoggerBuilder, LogLevel};
    ///
    /// # fn main() -> anyhow::Result<()> {
    /// LoggerBuilder::new()
    ///     .with_level(LogLevel::Info)
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn build(self) -> Result<()> {
        // Create environment filter
        let env_filter = EnvFilter::new(self.level.as_str())
            .add_directive("impulse=trace".parse()?)
            .add_directive("hyper=info".parse()?)
            .add_directive("tokio=info".parse()?);

        // Create the appropriate layer based on output and format
        match self.output {
            LogOutput::Stdout => {
                let base_layer = fmt::layer()
                    .with_writer(std::io::stdout)
                    .with_target(self.show_target)
                    .with_thread_ids(self.show_thread_ids)
                    .with_line_number(self.show_line_numbers)
                    .with_span_events(FmtSpan::CLOSE);

                match self.format {
                    LogFormat::Human => {
                        tracing_subscriber::registry()
                            .with(env_filter)
                            .with(base_layer)
                            .try_init()
                            .context("Failed to initialize tracing subscriber")?;
                    }
                    LogFormat::Json => {
                        tracing_subscriber::registry()
                            .with(env_filter)
                            .with(base_layer.json())
                            .try_init()
                            .context("Failed to initialize tracing subscriber")?;
                    }
                    LogFormat::Compact => {
                        tracing_subscriber::registry()
                            .with(env_filter)
                            .with(base_layer.compact())
                            .try_init()
                            .context("Failed to initialize tracing subscriber")?;
                    }
                }
            }
            LogOutput::Stderr => {
                let base_layer = fmt::layer()
                    .with_writer(std::io::stderr)
                    .with_target(self.show_target)
                    .with_thread_ids(self.show_thread_ids)
                    .with_line_number(self.show_line_numbers)
                    .with_span_events(FmtSpan::CLOSE);

                match self.format {
                    LogFormat::Human => {
                        tracing_subscriber::registry()
                            .with(env_filter)
                            .with(base_layer)
                            .try_init()
                            .context("Failed to initialize tracing subscriber")?;
                    }
                    LogFormat::Json => {
                        tracing_subscriber::registry()
                            .with(env_filter)
                            .with(base_layer.json())
                            .try_init()
                            .context("Failed to initialize tracing subscriber")?;
                    }
                    LogFormat::Compact => {
                        tracing_subscriber::registry()
                            .with(env_filter)
                            .with(base_layer.compact())
                            .try_init()
                            .context("Failed to initialize tracing subscriber")?;
                    }
                }
            }
            LogOutput::File(ref path) => {
                // Create parent directory if it doesn't exist
                if let Some(parent) = path.parent() {
                    std::fs::create_dir_all(parent)
                        .with_context(|| format!("Failed to create log directory: {:?}", parent))?;
                }

                // Set up file appender with rotation if configured
                let file_appender = if let Some(rotation) = self.rotation_policy {
                    rotation.create_appender(path)?
                } else {
                    // Default: no rotation, just append to file
                    let file = std::fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(path)
                        .with_context(|| format!("Failed to open log file: {:?}", path))?;
                    tracing_appender::non_blocking(file).0
                };

                let base_layer = fmt::layer()
                    .with_writer(file_appender)
                    .with_target(self.show_target)
                    .with_thread_ids(self.show_thread_ids)
                    .with_line_number(self.show_line_numbers)
                    .with_span_events(FmtSpan::CLOSE);

                match self.format {
                    LogFormat::Human => {
                        tracing_subscriber::registry()
                            .with(env_filter)
                            .with(base_layer)
                            .try_init()
                            .context("Failed to initialize tracing subscriber")?;
                    }
                    LogFormat::Json => {
                        tracing_subscriber::registry()
                            .with(env_filter)
                            .with(base_layer.json())
                            .try_init()
                            .context("Failed to initialize tracing subscriber")?;
                    }
                    LogFormat::Compact => {
                        tracing_subscriber::registry()
                            .with(env_filter)
                            .with(base_layer.compact())
                            .try_init()
                            .context("Failed to initialize tracing subscriber")?;
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_ordering() {
        assert!(LogLevel::Trace < LogLevel::Debug);
        assert!(LogLevel::Debug < LogLevel::Info);
        assert!(LogLevel::Info < LogLevel::Warn);
        assert!(LogLevel::Warn < LogLevel::Error);
    }

    #[test]
    fn test_log_level_display() {
        assert_eq!(LogLevel::Trace.to_string(), "trace");
        assert_eq!(LogLevel::Debug.to_string(), "debug");
        assert_eq!(LogLevel::Info.to_string(), "info");
        assert_eq!(LogLevel::Warn.to_string(), "warn");
        assert_eq!(LogLevel::Error.to_string(), "error");
    }

    #[test]
    fn test_log_level_as_tracing_level() {
        assert_eq!(LogLevel::Trace.as_tracing_level(), Level::TRACE);
        assert_eq!(LogLevel::Debug.as_tracing_level(), Level::DEBUG);
        assert_eq!(LogLevel::Info.as_tracing_level(), Level::INFO);
        assert_eq!(LogLevel::Warn.as_tracing_level(), Level::WARN);
        assert_eq!(LogLevel::Error.as_tracing_level(), Level::ERROR);
    }

    #[test]
    fn test_logger_builder_defaults() {
        let builder = LoggerBuilder::new();
        assert_eq!(builder.level(), LogLevel::Info);
        assert_eq!(builder.format(), LogFormat::Human);
        assert_eq!(builder.output(), &LogOutput::Stdout);
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
    fn test_logger_builder_with_output() {
        let builder = LoggerBuilder::new().with_output(LogOutput::File(PathBuf::from("test.log")));
        assert_eq!(
            builder.output(),
            &LogOutput::File(PathBuf::from("test.log"))
        );
    }

    #[test]
    fn test_logger_builder_with_max_files() {
        let builder = LoggerBuilder::new().with_max_files(30);
        assert_eq!(builder.max_files, 30);
    }

    #[test]
    fn test_logger_builder_chaining() {
        let builder = LoggerBuilder::new()
            .with_level(LogLevel::Debug)
            .with_format(LogFormat::Json)
            .with_output(LogOutput::Stderr)
            .with_max_files(20)
            .with_show_target(false)
            .with_show_thread_ids(true);

        assert_eq!(builder.level(), LogLevel::Debug);
        assert_eq!(builder.format(), LogFormat::Json);
        assert_eq!(builder.output(), &LogOutput::Stderr);
        assert_eq!(builder.max_files, 20);
        assert!(!builder.show_target);
        assert!(builder.show_thread_ids);
    }
}
