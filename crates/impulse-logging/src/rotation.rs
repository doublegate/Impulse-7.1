//! File rotation policies and management
//!
//! This module provides file rotation capabilities including size-based and time-based
//! rotation policies.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tracing_appender::{
    non_blocking::NonBlocking,
    rolling::{RollingFileAppender, Rotation as TracingRotation},
};

/// File rotation policy
///
/// Determines when log files should be rotated to a new file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotationPolicy {
    /// Rotate log files hourly
    Hourly,
    /// Rotate log files daily (recommended for most use cases)
    Daily,
    /// Rotate log files when they reach a specific size in bytes
    Size(u64),
    /// No rotation (single file)
    Never,
}

impl RotationPolicy {
    /// Create a file appender with this rotation policy
    ///
    /// # Arguments
    ///
    /// * `path` - The log file path
    ///
    /// # Returns
    ///
    /// A non-blocking file appender configured with the rotation policy
    pub(crate) fn create_appender(&self, path: &Path) -> Result<NonBlocking> {
        let parent = path
            .parent()
            .context("Log path must have a parent directory")?;
        let file_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .context("Log path must have a valid file name")?;

        let appender = match self {
            RotationPolicy::Hourly => {
                RollingFileAppender::new(TracingRotation::HOURLY, parent, file_name)
            }
            RotationPolicy::Daily => {
                RollingFileAppender::new(TracingRotation::DAILY, parent, file_name)
            }
            RotationPolicy::Never => {
                RollingFileAppender::new(TracingRotation::NEVER, parent, file_name)
            }
            RotationPolicy::Size(max_bytes) => {
                // NOTE: Size-based rotation not yet implemented.
                // The tracing-appender crate doesn't provide built-in size-based rotation.
                // Custom implementation would require:
                // 1. File size monitoring thread
                // 2. Atomic log rotation mechanism
                // 3. Proper handling of concurrent writes
                // Current workaround: Falls back to daily rotation with a warning.
                // This ensures logs rotate regularly and don't grow unbounded.
                tracing::warn!(
                    max_bytes = %max_bytes,
                    "Size-based rotation not yet implemented, using daily rotation as fallback"
                );
                RollingFileAppender::new(TracingRotation::DAILY, parent, file_name)
            }
        };

        let (non_blocking, _guard) = tracing_appender::non_blocking(appender);

        // Store the guard to prevent premature cleanup
        // In production, this should be managed by the application
        std::mem::forget(_guard);

        Ok(non_blocking)
    }

    /// Get a human-readable description of the rotation policy
    pub fn description(&self) -> String {
        match self {
            RotationPolicy::Hourly => "Hourly rotation".to_string(),
            RotationPolicy::Daily => "Daily rotation".to_string(),
            RotationPolicy::Size(bytes) => format!("Size-based rotation ({} bytes)", bytes),
            RotationPolicy::Never => "No rotation".to_string(),
        }
    }
}

/// Trigger for log rotation
///
/// Represents the condition that triggered a log rotation event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotationTrigger {
    /// Rotation triggered by time boundary (hour/day)
    Time,
    /// Rotation triggered by file size limit
    Size,
    /// Manual rotation requested
    Manual,
}

/// Manager for log file rotation and cleanup
///
/// Handles cleanup of old rotated log files based on retention policies.
pub struct RotationManager {
    log_dir: PathBuf,
    max_files: usize,
}

impl RotationManager {
    /// Create a new rotation manager
    ///
    /// # Arguments
    ///
    /// * `log_dir` - Directory containing log files
    /// * `max_files` - Maximum number of rotated files to keep
    ///
    /// # Examples
    ///
    /// ```rust
    /// use impulse_logging::RotationManager;
    /// use std::path::Path;
    ///
    /// let manager = RotationManager::new(Path::new("logs"), 30);
    /// ```
    pub fn new(log_dir: impl Into<PathBuf>, max_files: usize) -> Self {
        Self {
            log_dir: log_dir.into(),
            max_files,
        }
    }

    /// Clean up old rotated log files
    ///
    /// Removes the oldest files if the count exceeds `max_files`.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The log directory cannot be read
    /// - Files cannot be deleted
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use impulse_logging::RotationManager;
    /// use std::path::Path;
    ///
    /// # fn main() -> anyhow::Result<()> {
    /// let manager = RotationManager::new(Path::new("logs"), 30);
    /// manager.cleanup_old_files()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn cleanup_old_files(&self) -> Result<()> {
        let mut log_files = Vec::new();

        // Find all log files in the directory
        for entry in std::fs::read_dir(&self.log_dir)
            .with_context(|| format!("Failed to read log directory: {:?}", self.log_dir))?
        {
            let entry = entry?;
            let path = entry.path();

            if path.is_file()
                && let Some(name) = path.file_name().and_then(|n| n.to_str())
                && (name.ends_with(".log") || name.contains(".log."))
            {
                // Look for rotated log files (typically have date suffixes)
                let metadata = entry.metadata()?;
                let modified = metadata.modified()?;
                log_files.push((path, modified));
            }
        }

        // Sort by modification time (oldest first)
        log_files.sort_by_key(|(_, time)| *time);

        // Remove oldest files if we exceed max_files
        let to_remove = log_files.len().saturating_sub(self.max_files);
        for (path, _) in log_files.iter().take(to_remove) {
            tracing::info!(?path, "Removing old log file");
            std::fs::remove_file(path)
                .with_context(|| format!("Failed to remove log file: {:?}", path))?;
        }

        Ok(())
    }

    /// Get the number of log files currently in the directory
    ///
    /// # Errors
    ///
    /// Returns an error if the log directory cannot be read
    pub fn count_log_files(&self) -> Result<usize> {
        let count = std::fs::read_dir(&self.log_dir)
            .with_context(|| format!("Failed to read log directory: {:?}", self.log_dir))?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.path().is_file()
                    && entry
                        .path()
                        .file_name()
                        .and_then(|n| n.to_str())
                        .map(|name| name.ends_with(".log") || name.contains(".log."))
                        .unwrap_or(false)
            })
            .count();

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_rotation_policy_description() {
        assert_eq!(RotationPolicy::Hourly.description(), "Hourly rotation");
        assert_eq!(RotationPolicy::Daily.description(), "Daily rotation");
        assert_eq!(
            RotationPolicy::Size(10_000_000).description(),
            "Size-based rotation (10000000 bytes)"
        );
        assert_eq!(RotationPolicy::Never.description(), "No rotation");
    }

    #[test]
    fn test_rotation_manager_new() {
        let temp_dir = TempDir::new().unwrap();
        let manager = RotationManager::new(temp_dir.path(), 30);
        assert_eq!(manager.max_files, 30);
    }

    #[test]
    fn test_rotation_manager_count_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let manager = RotationManager::new(temp_dir.path(), 30);
        assert_eq!(manager.count_log_files().unwrap(), 0);
    }

    #[test]
    fn test_rotation_manager_count_with_files() {
        let temp_dir = TempDir::new().unwrap();
        let log_dir = temp_dir.path();

        // Create some log files
        std::fs::write(log_dir.join("app.log"), b"log1").unwrap();
        std::fs::write(log_dir.join("app.log.2025-01-01"), b"log2").unwrap();
        std::fs::write(log_dir.join("app.log.2025-01-02"), b"log3").unwrap();
        std::fs::write(log_dir.join("other.txt"), b"not a log").unwrap();

        let manager = RotationManager::new(log_dir, 30);
        assert_eq!(manager.count_log_files().unwrap(), 3); // Only .log files
    }

    #[test]
    fn test_rotation_manager_cleanup() {
        let temp_dir = TempDir::new().unwrap();
        let log_dir = temp_dir.path();

        // Create more files than max_files
        for i in 0..10 {
            let filename = format!("app.log.2025-01-{:02}", i + 1);
            std::fs::write(log_dir.join(filename), format!("log{}", i)).unwrap();
            // Add small delay to ensure different modification times
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        let manager = RotationManager::new(log_dir, 5);
        assert_eq!(manager.count_log_files().unwrap(), 10);

        manager.cleanup_old_files().unwrap();
        assert_eq!(manager.count_log_files().unwrap(), 5);
    }

    #[test]
    fn test_rotation_policy_equality() {
        assert_eq!(RotationPolicy::Hourly, RotationPolicy::Hourly);
        assert_eq!(RotationPolicy::Daily, RotationPolicy::Daily);
        assert_eq!(RotationPolicy::Size(1000), RotationPolicy::Size(1000));
        assert_ne!(RotationPolicy::Size(1000), RotationPolicy::Size(2000));
        assert_ne!(RotationPolicy::Hourly, RotationPolicy::Daily);
    }

    #[test]
    fn test_rotation_trigger_values() {
        assert_eq!(RotationTrigger::Time, RotationTrigger::Time);
        assert_eq!(RotationTrigger::Size, RotationTrigger::Size);
        assert_eq!(RotationTrigger::Manual, RotationTrigger::Manual);
        assert_ne!(RotationTrigger::Time, RotationTrigger::Size);
    }
}
