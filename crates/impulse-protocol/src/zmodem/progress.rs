//! Progress tracking for Zmodem file transfers.
//!
//! This module provides traits and types for tracking file transfer progress,
//! allowing applications to display progress bars, status messages, or other
//! UI feedback during transfers.

use super::file::ZmodemFileInfo;
use super::send::TransferStats;
use std::path::PathBuf;

/// Batch transfer statistics.
///
/// Tracks overall progress across multiple file transfers.
///
/// # Examples
///
/// ```
/// use impulse_protocol::zmodem::progress::BatchStats;
///
/// let mut stats = BatchStats::new(5, 1048576);
/// stats.files_completed = 2;
/// stats.bytes_sent = 524288;
///
/// assert_eq!(stats.files_percent_complete(), 40.0);
/// assert_eq!(stats.bytes_percent_complete(), 50.0);
/// ```
#[derive(Debug, Clone)]
pub struct BatchStats {
    /// Total number of files in batch
    pub total_files: usize,

    /// Number of files completed
    pub files_completed: usize,

    /// Total bytes across all files
    pub total_bytes: u64,

    /// Bytes sent across all files
    pub bytes_sent: u64,

    /// Total retransmissions across all files
    pub total_retries: u32,
}

impl BatchStats {
    /// Create new batch statistics.
    ///
    /// # Arguments
    ///
    /// * `total_files` - Total number of files to transfer
    /// * `total_bytes` - Total bytes across all files
    pub fn new(total_files: usize, total_bytes: u64) -> Self {
        Self {
            total_files,
            files_completed: 0,
            total_bytes,
            bytes_sent: 0,
            total_retries: 0,
        }
    }

    /// Get percentage of files completed.
    pub fn files_percent_complete(&self) -> f64 {
        if self.total_files == 0 {
            100.0
        } else {
            (self.files_completed as f64 / self.total_files as f64) * 100.0
        }
    }

    /// Get percentage of bytes completed.
    pub fn bytes_percent_complete(&self) -> f64 {
        if self.total_bytes == 0 {
            100.0
        } else {
            (self.bytes_sent as f64 / self.total_bytes as f64) * 100.0
        }
    }

    /// Check if batch is complete.
    pub fn is_complete(&self) -> bool {
        self.files_completed >= self.total_files
    }
}

/// Progress information for current file.
///
/// Contains details about the file currently being transferred.
///
/// # Examples
///
/// ```
/// use impulse_protocol::zmodem::progress::FileProgress;
/// use std::path::PathBuf;
///
/// let progress = FileProgress {
///     file_index: 2,
///     total_files: 5,
///     file_path: PathBuf::from("document.pdf"),
///     file_name: "document.pdf".to_string(),
///     bytes_sent: 524288,
///     bytes_total: 1048576,
///     retries: 1,
/// };
///
/// assert_eq!(progress.percent_complete(), 50.0);
/// ```
#[derive(Debug, Clone)]
pub struct FileProgress {
    /// Index of current file (0-based)
    pub file_index: usize,

    /// Total number of files
    pub total_files: usize,

    /// Path to current file
    pub file_path: PathBuf,

    /// Name of current file
    pub file_name: String,

    /// Bytes sent for current file
    pub bytes_sent: u64,

    /// Total bytes for current file
    pub bytes_total: u64,

    /// Retries for current file
    pub retries: u32,
}

impl FileProgress {
    /// Get percentage complete for current file.
    pub fn percent_complete(&self) -> f64 {
        if self.bytes_total == 0 {
            100.0
        } else {
            (self.bytes_sent as f64 / self.bytes_total as f64) * 100.0
        }
    }
}

/// Progress callback trait for file transfers.
///
/// Implement this trait to receive progress updates during Zmodem file transfers.
///
/// # Examples
///
/// ```
/// use impulse_protocol::zmodem::progress::{TransferProgress, FileProgress, BatchStats};
/// use impulse_protocol::zmodem::{ZmodemFileInfo, ZmodemError};
/// use impulse_protocol::zmodem::send::TransferStats;
///
/// struct ConsoleProgress;
///
/// impl TransferProgress for ConsoleProgress {
///     fn on_file_start(&mut self, file_info: &ZmodemFileInfo) {
///         println!("Starting: {}", file_info.name);
///     }
///
///     fn on_progress(&mut self, progress: &FileProgress) {
///         println!("Progress: {:.1}%", progress.percent_complete());
///     }
///
///     fn on_file_complete(&mut self, file_info: &ZmodemFileInfo, stats: &TransferStats) {
///         println!("Complete: {} ({} bytes)", file_info.name, stats.bytes_sent);
///     }
///
///     fn on_error(&mut self, error: &ZmodemError) {
///         eprintln!("Error: {}", error);
///     }
///
///     fn on_complete(&mut self, batch_stats: &BatchStats) {
///         println!("Batch complete: {} files, {} bytes",
///             batch_stats.files_completed, batch_stats.bytes_sent);
///     }
/// }
/// ```
pub trait TransferProgress: Send + Sync {
    /// Called when a file transfer starts.
    ///
    /// # Arguments
    ///
    /// * `file_info` - Information about the file being transferred
    fn on_file_start(&mut self, file_info: &ZmodemFileInfo);

    /// Called periodically during file transfer.
    ///
    /// # Arguments
    ///
    /// * `progress` - Current progress information
    fn on_progress(&mut self, progress: &FileProgress);

    /// Called when a file transfer completes successfully.
    ///
    /// # Arguments
    ///
    /// * `file_info` - Information about the completed file
    /// * `stats` - Transfer statistics
    fn on_file_complete(&mut self, file_info: &ZmodemFileInfo, stats: &TransferStats);

    /// Called when an error occurs.
    ///
    /// # Arguments
    ///
    /// * `error` - The error that occurred
    fn on_error(&mut self, error: &super::error::ZmodemError);

    /// Called when batch transfer completes.
    ///
    /// # Arguments
    ///
    /// * `batch_stats` - Overall batch statistics
    fn on_complete(&mut self, batch_stats: &BatchStats);
}

/// No-op progress callback.
///
/// Use this when you don't need progress tracking.
///
/// # Examples
///
/// ```
/// use impulse_protocol::zmodem::progress::NoOpProgress;
///
/// let progress = NoOpProgress;
/// // Use with sender...
/// ```
#[derive(Debug, Clone, Copy)]
pub struct NoOpProgress;

impl TransferProgress for NoOpProgress {
    fn on_file_start(&mut self, _file_info: &ZmodemFileInfo) {}
    fn on_progress(&mut self, _progress: &FileProgress) {}
    fn on_file_complete(&mut self, _file_info: &ZmodemFileInfo, _stats: &TransferStats) {}
    fn on_error(&mut self, _error: &super::error::ZmodemError) {}
    fn on_complete(&mut self, _batch_stats: &BatchStats) {}
}

/// Simple console progress callback.
///
/// Prints progress updates to stdout.
///
/// # Examples
///
/// ```
/// use impulse_protocol::zmodem::progress::ConsoleProgress;
///
/// let progress = ConsoleProgress::new();
/// // Use with sender...
/// ```
#[derive(Debug)]
pub struct ConsoleProgress {
    verbose: bool,
}

impl ConsoleProgress {
    /// Create new console progress tracker.
    pub fn new() -> Self {
        Self { verbose: false }
    }

    /// Create verbose console progress tracker.
    pub fn verbose() -> Self {
        Self { verbose: true }
    }
}

impl Default for ConsoleProgress {
    fn default() -> Self {
        Self::new()
    }
}

impl TransferProgress for ConsoleProgress {
    fn on_file_start(&mut self, file_info: &ZmodemFileInfo) {
        println!("Sending: {} ({} bytes)", file_info.name, file_info.size);
    }

    fn on_progress(&mut self, progress: &FileProgress) {
        if self.verbose {
            println!(
                "  [{}/{}] {}: {:.1}% ({}/{} bytes)",
                progress.file_index + 1,
                progress.total_files,
                progress.file_name,
                progress.percent_complete(),
                progress.bytes_sent,
                progress.bytes_total
            );
        }
    }

    fn on_file_complete(&mut self, file_info: &ZmodemFileInfo, stats: &TransferStats) {
        println!(
            "Complete: {} - {} bytes in {:.2}s ({:.0} bytes/sec, {} retries)",
            file_info.name,
            stats.bytes_sent,
            stats.elapsed().as_secs_f64(),
            stats.bytes_per_second(),
            stats.retries
        );
    }

    fn on_error(&mut self, error: &super::error::ZmodemError) {
        eprintln!("Error: {}", error);
    }

    fn on_complete(&mut self, batch_stats: &BatchStats) {
        println!(
            "\nBatch complete: {} files, {} bytes, {} retries",
            batch_stats.files_completed, batch_stats.bytes_sent, batch_stats.total_retries
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_stats_new() {
        let stats = BatchStats::new(5, 10000);
        assert_eq!(stats.total_files, 5);
        assert_eq!(stats.files_completed, 0);
        assert_eq!(stats.total_bytes, 10000);
        assert_eq!(stats.bytes_sent, 0);
        assert_eq!(stats.total_retries, 0);
    }

    #[test]
    fn test_batch_stats_files_percent() {
        let mut stats = BatchStats::new(4, 10000);
        assert_eq!(stats.files_percent_complete(), 0.0);

        stats.files_completed = 2;
        assert_eq!(stats.files_percent_complete(), 50.0);

        stats.files_completed = 4;
        assert_eq!(stats.files_percent_complete(), 100.0);
    }

    #[test]
    fn test_batch_stats_bytes_percent() {
        let mut stats = BatchStats::new(4, 10000);
        assert_eq!(stats.bytes_percent_complete(), 0.0);

        stats.bytes_sent = 5000;
        assert_eq!(stats.bytes_percent_complete(), 50.0);

        stats.bytes_sent = 10000;
        assert_eq!(stats.bytes_percent_complete(), 100.0);
    }

    #[test]
    fn test_batch_stats_zero_files() {
        let stats = BatchStats::new(0, 10000);
        assert_eq!(stats.files_percent_complete(), 100.0);
    }

    #[test]
    fn test_batch_stats_zero_bytes() {
        let stats = BatchStats::new(5, 0);
        assert_eq!(stats.bytes_percent_complete(), 100.0);
    }

    #[test]
    fn test_batch_stats_is_complete() {
        let mut stats = BatchStats::new(3, 10000);
        assert!(!stats.is_complete());

        stats.files_completed = 2;
        assert!(!stats.is_complete());

        stats.files_completed = 3;
        assert!(stats.is_complete());

        stats.files_completed = 4;
        assert!(stats.is_complete());
    }

    #[test]
    fn test_file_progress_percent_complete() {
        let progress = FileProgress {
            file_index: 0,
            total_files: 1,
            file_path: PathBuf::from("test.txt"),
            file_name: "test.txt".to_string(),
            bytes_sent: 500,
            bytes_total: 1000,
            retries: 0,
        };

        assert_eq!(progress.percent_complete(), 50.0);
    }

    #[test]
    fn test_file_progress_zero_size() {
        let progress = FileProgress {
            file_index: 0,
            total_files: 1,
            file_path: PathBuf::from("empty.txt"),
            file_name: "empty.txt".to_string(),
            bytes_sent: 0,
            bytes_total: 0,
            retries: 0,
        };

        assert_eq!(progress.percent_complete(), 100.0);
    }

    #[test]
    fn test_noop_progress() {
        let mut progress = NoOpProgress;
        let file_info = ZmodemFileInfo::new("test.txt", 1024);
        let stats = TransferStats::new(1024);
        let batch_stats = BatchStats::new(1, 1024);
        let file_progress = FileProgress {
            file_index: 0,
            total_files: 1,
            file_path: PathBuf::from("test.txt"),
            file_name: "test.txt".to_string(),
            bytes_sent: 512,
            bytes_total: 1024,
            retries: 0,
        };

        // Should not panic
        progress.on_file_start(&file_info);
        progress.on_progress(&file_progress);
        progress.on_file_complete(&file_info, &stats);
        progress.on_error(&super::super::error::ZmodemError::Timeout);
        progress.on_complete(&batch_stats);
    }

    #[test]
    fn test_console_progress_creation() {
        let progress = ConsoleProgress::new();
        assert!(!progress.verbose);

        let verbose_progress = ConsoleProgress::verbose();
        assert!(verbose_progress.verbose);

        let default_progress = ConsoleProgress::default();
        assert!(!default_progress.verbose);
    }
}
