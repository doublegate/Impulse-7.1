//! Log archival and compression system
//!
//! This module provides automatic compression and archival of old log files,
//! with configurable retention periods and storage management.

use anyhow::{Context, Result};
use flate2::Compression;
use flate2::write::GzEncoder;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use tokio::task;

/// Configuration for log archival
#[derive(Debug, Clone)]
pub struct ArchivalConfig {
    /// Directory for archived logs
    pub archive_dir: PathBuf,
    /// Retention period in days
    pub retention_days: u32,
    /// Compression level (0-9, higher = better compression but slower)
    pub compression_level: u32,
    /// Enable automatic archival
    pub enabled: bool,
}

impl Default for ArchivalConfig {
    fn default() -> Self {
        Self {
            archive_dir: PathBuf::from("logs/archive"),
            retention_days: 90,
            compression_level: 6,
            enabled: true,
        }
    }
}

impl ArchivalConfig {
    /// Create a new archival configuration
    ///
    /// # Examples
    ///
    /// ```rust
    /// use impulse_logging::ArchivalConfig;
    /// use std::path::PathBuf;
    ///
    /// let config = ArchivalConfig::new(
    ///     PathBuf::from("logs/archive"),
    ///     90,
    ///     6
    /// );
    /// ```
    pub fn new(archive_dir: PathBuf, retention_days: u32, compression_level: u32) -> Self {
        Self {
            archive_dir,
            retention_days,
            compression_level: compression_level.min(9), // Cap at 9
            enabled: true,
        }
    }

    /// Disable automatic archival
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Default::default()
        }
    }
}

/// Manager for log archival operations
///
/// Handles compression and long-term storage of log files.
pub struct ArchiveManager {
    config: ArchivalConfig,
}

impl ArchiveManager {
    /// Create a new archive manager
    ///
    /// # Examples
    ///
    /// ```rust
    /// use impulse_logging::{ArchiveManager, ArchivalConfig};
    ///
    /// let config = ArchivalConfig::default();
    /// let manager = ArchiveManager::new(config);
    /// ```
    pub fn new(config: ArchivalConfig) -> Self {
        Self { config }
    }

    /// Compress a log file using gzip
    ///
    /// # Arguments
    ///
    /// * `source` - Path to the log file to compress
    /// * `dest` - Optional destination path (defaults to source + .gz)
    ///
    /// # Returns
    ///
    /// The path to the compressed file
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use impulse_logging::{ArchiveManager, ArchivalConfig};
    /// use std::path::Path;
    ///
    /// # async fn example() -> anyhow::Result<()> {
    /// let manager = ArchiveManager::new(ArchivalConfig::default());
    /// let compressed = manager.compress_file(
    ///     Path::new("logs/old.log"),
    ///     None
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn compress_file(&self, source: &Path, dest: Option<&Path>) -> Result<PathBuf> {
        let source = source.to_owned();
        let dest = dest.map(|p| p.to_owned());
        let compression_level = self.config.compression_level;

        task::spawn_blocking(move || {
            Self::compress_file_sync(&source, dest.as_deref(), compression_level)
        })
        .await?
    }

    /// Synchronous version of compress_file (internal)
    fn compress_file_sync(
        source: &Path,
        dest: Option<&Path>,
        compression_level: u32,
    ) -> Result<PathBuf> {
        let dest_path = if let Some(d) = dest {
            d.to_owned()
        } else {
            let mut p = source.to_owned();
            p.set_extension(format!(
                "{}.gz",
                source.extension().and_then(|e| e.to_str()).unwrap_or("")
            ));
            p
        };

        // Create parent directory if needed
        if let Some(parent) = dest_path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create archive directory: {:?}", parent))?;
        }

        // Open source file
        let mut source_file = File::open(source)
            .with_context(|| format!("Failed to open source file: {:?}", source))?;

        // Create compressed file
        let dest_file = File::create(&dest_path)
            .with_context(|| format!("Failed to create compressed file: {:?}", dest_path))?;

        let mut encoder = GzEncoder::new(dest_file, Compression::new(compression_level));

        // Copy and compress
        io::copy(&mut source_file, &mut encoder).context("Failed to compress log file")?;

        encoder.finish().context("Failed to finish compression")?;

        tracing::info!(source = ?source, dest = ?dest_path, "Compressed log file");

        Ok(dest_path)
    }

    /// Archive old log files from a directory
    ///
    /// Compresses log files older than 24 hours and moves them to the archive directory.
    ///
    /// # Arguments
    ///
    /// * `log_dir` - Directory containing log files
    ///
    /// # Returns
    ///
    /// The number of files archived
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use impulse_logging::{ArchiveManager, ArchivalConfig};
    /// use std::path::Path;
    ///
    /// # async fn example() -> anyhow::Result<()> {
    /// let manager = ArchiveManager::new(ArchivalConfig::default());
    /// let count = manager.archive_old_logs(Path::new("logs")).await?;
    /// println!("Archived {} files", count);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn archive_old_logs(&self, log_dir: &Path) -> Result<usize> {
        if !self.config.enabled {
            return Ok(0);
        }

        let mut archived_count = 0;
        let cutoff_time = SystemTime::now() - Duration::from_secs(24 * 60 * 60); // 24 hours ago

        // Find old log files
        for entry in std::fs::read_dir(log_dir)
            .with_context(|| format!("Failed to read log directory: {:?}", log_dir))?
        {
            let entry = entry?;
            let path = entry.path();

            if !path.is_file() {
                continue;
            }

            // Skip already compressed files
            if path.extension().and_then(|e| e.to_str()) == Some("gz") {
                continue;
            }

            // Check if file is a log file
            let is_log_file = path
                .file_name()
                .and_then(|n| n.to_str())
                .map(|name| name.ends_with(".log") || name.contains(".log."))
                .unwrap_or(false);

            if !is_log_file {
                continue;
            }

            // Check file age
            let metadata = entry.metadata()?;
            let modified = metadata.modified()?;

            if modified < cutoff_time {
                // Compress and move to archive
                let archived_name = path.file_name().unwrap();
                let archive_path = self.config.archive_dir.join(archived_name);

                self.compress_file(&path, Some(&archive_path)).await?;

                // Remove original file
                std::fs::remove_file(&path)
                    .with_context(|| format!("Failed to remove original log file: {:?}", path))?;

                archived_count += 1;
            }
        }

        Ok(archived_count)
    }

    /// Clean up archived logs older than retention period
    ///
    /// # Returns
    ///
    /// The number of files deleted
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use impulse_logging::{ArchiveManager, ArchivalConfig};
    ///
    /// # async fn example() -> anyhow::Result<()> {
    /// let manager = ArchiveManager::new(ArchivalConfig::default());
    /// let count = manager.cleanup_old_archives().await?;
    /// println!("Deleted {} old archives", count);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn cleanup_old_archives(&self) -> Result<usize> {
        if !self.config.enabled {
            return Ok(0);
        }

        let mut deleted_count = 0;
        let retention_secs = self.config.retention_days as u64 * 24 * 60 * 60;
        let cutoff_time = SystemTime::now() - Duration::from_secs(retention_secs);

        // Ensure archive directory exists
        if !self.config.archive_dir.exists() {
            return Ok(0);
        }

        // Find old archived files
        for entry in std::fs::read_dir(&self.config.archive_dir).with_context(|| {
            format!(
                "Failed to read archive directory: {:?}",
                self.config.archive_dir
            )
        })? {
            let entry = entry?;
            let path = entry.path();

            if !path.is_file() {
                continue;
            }

            let metadata = entry.metadata()?;
            let modified = metadata.modified()?;

            if modified < cutoff_time {
                tracing::info!(
                    ?path,
                    retention_days = self.config.retention_days,
                    "Deleting old archive"
                );
                std::fs::remove_file(&path)
                    .with_context(|| format!("Failed to remove old archive: {:?}", path))?;
                deleted_count += 1;
            }
        }

        Ok(deleted_count)
    }

    /// Run archival maintenance (archive old logs and cleanup)
    ///
    /// This is a convenience method that runs both archival and cleanup operations.
    ///
    /// # Arguments
    ///
    /// * `log_dir` - Directory containing active log files
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use impulse_logging::{ArchiveManager, ArchivalConfig};
    /// use std::path::Path;
    ///
    /// # async fn example() -> anyhow::Result<()> {
    /// let manager = ArchiveManager::new(ArchivalConfig::default());
    /// manager.run_maintenance(Path::new("logs")).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn run_maintenance(&self, log_dir: &Path) -> Result<()> {
        if !self.config.enabled {
            return Ok(());
        }

        let archived = self.archive_old_logs(log_dir).await?;
        let cleaned = self.cleanup_old_archives().await?;

        tracing::info!(
            archived_files = archived,
            deleted_archives = cleaned,
            "Completed log archival maintenance"
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_archival_config_default() {
        let config = ArchivalConfig::default();
        assert_eq!(config.retention_days, 90);
        assert_eq!(config.compression_level, 6);
        assert!(config.enabled);
    }

    #[test]
    fn test_archival_config_new() {
        let config = ArchivalConfig::new(PathBuf::from("/tmp/archive"), 30, 9);
        assert_eq!(config.archive_dir, PathBuf::from("/tmp/archive"));
        assert_eq!(config.retention_days, 30);
        assert_eq!(config.compression_level, 9);
        assert!(config.enabled);
    }

    #[test]
    fn test_archival_config_disabled() {
        let config = ArchivalConfig::disabled();
        assert!(!config.enabled);
    }

    #[tokio::test]
    async fn test_compress_file() {
        let temp_dir = TempDir::new().unwrap();
        let source = temp_dir.path().join("test.log");
        let content = b"This is a test log file with some content to compress";

        std::fs::write(&source, content).unwrap();

        let config = ArchivalConfig::default();
        let manager = ArchiveManager::new(config);

        let compressed = manager.compress_file(&source, None).await.unwrap();

        assert!(compressed.exists());
        assert!(compressed.to_str().unwrap().ends_with(".gz"));

        // Compressed file should be smaller (for this small test file it might not be,
        // but we can at least verify it exists and has content)
        let compressed_size = std::fs::metadata(&compressed).unwrap().len();
        assert!(compressed_size > 0);
    }

    #[tokio::test]
    async fn test_archive_manager_disabled() {
        let temp_dir = TempDir::new().unwrap();
        let config = ArchivalConfig::disabled();
        let manager = ArchiveManager::new(config);

        let count = manager.archive_old_logs(temp_dir.path()).await.unwrap();
        assert_eq!(count, 0);

        let count = manager.cleanup_old_archives().await.unwrap();
        assert_eq!(count, 0);
    }

    #[tokio::test]
    async fn test_compression_level_capped() {
        let config = ArchivalConfig::new(PathBuf::from("/tmp"), 30, 15); // Over max
        assert_eq!(config.compression_level, 9); // Should be capped
    }
}
