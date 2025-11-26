//! Transfer progress display for uploads and downloads
//!
//! This module provides progress screens for file transfers,
//! integrating with the Zmodem protocol's TransferProgress trait.

use std::time::{Duration, Instant};

use impulse_protocol::zmodem::error::ZmodemError;
use impulse_protocol::zmodem::file::ZmodemFileInfo;
use impulse_protocol::zmodem::progress::{BatchStats, FileProgress, TransferProgress};
use impulse_protocol::zmodem::send::TransferStats;

/// Transfer direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferDirection {
    /// Upload (receiving file from user)
    Upload,
    /// Download (sending file to user)
    Download,
}

/// Transfer progress status
#[derive(Debug, Clone, Copy)]
pub enum TransferStatus {
    /// Waiting for transfer to start
    Waiting,

    /// Initializing protocol
    Initializing,

    /// Transfer in progress
    InProgress {
        /// Bytes transferred so far
        bytes_transferred: u64,
        /// Total bytes to transfer
        total_bytes: u64,
    },

    /// Transfer complete
    Complete,

    /// Transfer failed
    Failed,

    /// Transfer cancelled by user
    Cancelled,
}

/// Transfer progress screen
///
/// Displays transfer progress with progress bar, speed, and ETA.
/// Implements the TransferProgress trait for Zmodem integration.
pub struct TransferProgressScreen {
    /// Current transfer status
    pub status: TransferStatus,

    /// Transfer direction
    pub direction: TransferDirection,

    /// Filename being transferred
    pub filename: String,

    /// Start time of transfer
    pub start_time: Option<Instant>,

    /// Last update time for rate calculation
    last_update: Option<Instant>,

    /// Bytes at last update for rate calculation
    last_bytes: u64,

    /// Smoothed transfer rate (bytes/sec)
    smoothed_rate: f64,

    /// Progress bar width in characters
    pub bar_width: usize,

    /// Whether to use ANSI colors
    pub use_ansi: bool,

    /// Last error message
    pub last_error: Option<String>,
}

impl TransferProgressScreen {
    /// Create a new transfer progress screen
    pub fn new(filename: String, direction: TransferDirection) -> Self {
        Self {
            status: TransferStatus::Waiting,
            direction,
            filename,
            start_time: None,
            last_update: None,
            last_bytes: 0,
            smoothed_rate: 0.0,
            bar_width: 40,
            use_ansi: true,
            last_error: None,
        }
    }

    /// Create an upload progress screen
    pub fn upload(filename: String) -> Self {
        Self::new(filename, TransferDirection::Upload)
    }

    /// Create a download progress screen
    pub fn download(filename: String) -> Self {
        Self::new(filename, TransferDirection::Download)
    }

    /// Set whether to use ANSI colors
    pub fn with_ansi(mut self, use_ansi: bool) -> Self {
        self.use_ansi = use_ansi;
        self
    }

    /// Set progress bar width
    pub fn with_bar_width(mut self, width: usize) -> Self {
        self.bar_width = width;
        self
    }

    /// Start the transfer
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
        self.last_update = Some(Instant::now());
        self.status = TransferStatus::Initializing;
    }

    /// Update progress
    pub fn update_progress(&mut self, bytes_transferred: u64, total_bytes: u64) {
        let now = Instant::now();

        // Calculate instantaneous rate
        if let Some(last) = self.last_update {
            let elapsed = now.duration_since(last).as_secs_f64();
            if elapsed > 0.1 {
                // Update at most every 100ms
                let bytes_delta = bytes_transferred.saturating_sub(self.last_bytes);
                let instant_rate = bytes_delta as f64 / elapsed;

                // Exponential smoothing for rate (alpha = 0.3)
                self.smoothed_rate = 0.3 * instant_rate + 0.7 * self.smoothed_rate;

                self.last_update = Some(now);
                self.last_bytes = bytes_transferred;
            }
        } else {
            self.last_update = Some(now);
            self.last_bytes = bytes_transferred;
        }

        self.status = TransferStatus::InProgress {
            bytes_transferred,
            total_bytes,
        };
    }

    /// Mark as complete
    pub fn mark_complete(&mut self) {
        self.status = TransferStatus::Complete;
    }

    /// Mark as failed
    pub fn mark_failed(&mut self) {
        self.status = TransferStatus::Failed;
    }

    /// Mark as cancelled
    pub fn mark_cancelled(&mut self) {
        self.status = TransferStatus::Cancelled;
    }

    /// Get elapsed time
    pub fn elapsed(&self) -> Duration {
        self.start_time
            .map(|t| t.elapsed())
            .unwrap_or(Duration::ZERO)
    }

    /// Get current transfer rate (bytes/sec)
    pub fn current_rate(&self) -> f64 {
        self.smoothed_rate
    }

    /// Calculate ETA (estimated time remaining)
    fn calculate_eta(&self, bytes_transferred: u64, total_bytes: u64) -> Option<Duration> {
        if self.smoothed_rate > 0.0 && bytes_transferred < total_bytes {
            let remaining = total_bytes - bytes_transferred;
            let seconds = remaining as f64 / self.smoothed_rate;
            Some(Duration::from_secs_f64(seconds))
        } else {
            None
        }
    }

    /// Format bytes in human-readable form
    fn format_bytes(bytes: u64) -> String {
        if bytes >= 1024 * 1024 * 1024 {
            format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
        } else if bytes >= 1024 * 1024 {
            format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
        } else if bytes >= 1024 {
            format!("{:.2} KB", bytes as f64 / 1024.0)
        } else {
            format!("{} B", bytes)
        }
    }

    /// Format duration in human-readable form
    fn format_duration(duration: Duration) -> String {
        let secs = duration.as_secs();
        if secs >= 3600 {
            format!("{}:{:02}:{:02}", secs / 3600, (secs % 3600) / 60, secs % 60)
        } else if secs >= 60 {
            format!("{}:{:02}", secs / 60, secs % 60)
        } else {
            format!("{}s", secs)
        }
    }

    /// Render progress bar
    fn render_progress_bar(&self, percent: u32) -> String {
        let filled = (percent as usize * self.bar_width) / 100;
        let empty = self.bar_width.saturating_sub(filled);

        if self.use_ansi {
            format!(
                "\x1b[32m{}\x1b[0m\x1b[90m{}\x1b[0m",
                "=".repeat(filled),
                "-".repeat(empty)
            )
        } else {
            format!("{}{}", "=".repeat(filled), " ".repeat(empty))
        }
    }

    /// Render progress display
    pub fn render(&self) -> String {
        let mut output = String::new();

        // Header
        let direction_str = match self.direction {
            TransferDirection::Upload => "Uploading",
            TransferDirection::Download => "Downloading",
        };

        if self.use_ansi {
            output.push_str(&format!(
                "\x1b[1;36m{}: {}\x1b[0m\n\n",
                direction_str, self.filename
            ));
        } else {
            output.push_str(&format!("{}: {}\n\n", direction_str, self.filename));
        }

        match self.status {
            TransferStatus::Waiting => {
                output.push_str("Waiting for transfer to start...\n");
            }
            TransferStatus::Initializing => {
                output.push_str("Initializing protocol...\n");
            }
            TransferStatus::InProgress {
                bytes_transferred,
                total_bytes,
            } => {
                let percent = if total_bytes > 0 {
                    ((bytes_transferred as f64 / total_bytes as f64) * 100.0) as u32
                } else {
                    0
                };

                // Progress bar
                let bar = self.render_progress_bar(percent);
                output.push_str(&format!("[{}] {:>3}%\n", bar, percent));

                // Bytes transferred
                output.push_str(&format!(
                    "{} / {}\n",
                    Self::format_bytes(bytes_transferred),
                    Self::format_bytes(total_bytes)
                ));

                // Speed
                let speed_str = Self::format_bytes(self.smoothed_rate as u64);
                output.push_str(&format!("Speed: {}/s\n", speed_str));

                // Time and ETA
                let elapsed = Self::format_duration(self.elapsed());
                if let Some(eta) = self.calculate_eta(bytes_transferred, total_bytes) {
                    let eta_str = Self::format_duration(eta);
                    output.push_str(&format!("Elapsed: {} | ETA: {}\n", elapsed, eta_str));
                } else {
                    output.push_str(&format!("Elapsed: {}\n", elapsed));
                }
            }
            TransferStatus::Complete => {
                let elapsed = Self::format_duration(self.elapsed());
                if self.use_ansi {
                    output.push_str(&format!(
                        "\x1b[1;32mTransfer complete!\x1b[0m ({})\n",
                        elapsed
                    ));
                } else {
                    output.push_str(&format!("Transfer complete! ({})\n", elapsed));
                }
            }
            TransferStatus::Failed => {
                if self.use_ansi {
                    output.push_str("\x1b[1;31mTransfer failed.\x1b[0m\n");
                } else {
                    output.push_str("Transfer failed.\n");
                }
                if let Some(ref error) = self.last_error {
                    output.push_str(&format!("Error: {}\n", error));
                }
            }
            TransferStatus::Cancelled => {
                if self.use_ansi {
                    output.push_str("\x1b[1;33mTransfer cancelled.\x1b[0m\n");
                } else {
                    output.push_str("Transfer cancelled.\n");
                }
            }
        }

        output
    }
}

/// Implementation of TransferProgress for progress screen
impl TransferProgress for TransferProgressScreen {
    fn on_file_start(&mut self, file_info: &ZmodemFileInfo) {
        self.filename = file_info.name.clone();
        self.start();
    }

    fn on_progress(&mut self, progress: &FileProgress) {
        self.update_progress(progress.bytes_sent, progress.bytes_total);
    }

    fn on_file_complete(&mut self, _file_info: &ZmodemFileInfo, _stats: &TransferStats) {
        self.mark_complete();
    }

    fn on_error(&mut self, error: &ZmodemError) {
        self.last_error = Some(error.to_string());
        self.mark_failed();
    }

    fn on_complete(&mut self, _batch_stats: &BatchStats) {
        // Batch complete - nothing additional to do for single file progress
    }
}

// Legacy type alias for backwards compatibility
/// Upload progress status (legacy alias)
pub type UploadStatus = TransferStatus;

/// Upload progress screen (legacy wrapper)
pub type UploadProgressScreen = TransferProgressScreen;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_progress_screen_new() {
        let screen = TransferProgressScreen::upload("test.zip".to_string());
        assert_eq!(screen.filename, "test.zip");
        assert!(matches!(screen.status, TransferStatus::Waiting));
        assert_eq!(screen.direction, TransferDirection::Upload);
    }

    #[test]
    fn test_progress_screen_download() {
        let screen = TransferProgressScreen::download("game.zip".to_string());
        assert_eq!(screen.direction, TransferDirection::Download);
    }

    #[test]
    fn test_progress_screen_start() {
        let mut screen = TransferProgressScreen::upload("test.zip".to_string());
        screen.start();
        assert!(matches!(screen.status, TransferStatus::Initializing));
        assert!(screen.start_time.is_some());
    }

    #[test]
    fn test_progress_screen_update() {
        let mut screen = TransferProgressScreen::upload("test.zip".to_string());
        screen.start();
        screen.update_progress(500, 1000);

        match screen.status {
            TransferStatus::InProgress {
                bytes_transferred,
                total_bytes,
            } => {
                assert_eq!(bytes_transferred, 500);
                assert_eq!(total_bytes, 1000);
            }
            _ => panic!("Expected InProgress status"),
        }
    }

    #[test]
    fn test_progress_screen_complete() {
        let mut screen = TransferProgressScreen::upload("test.zip".to_string());
        screen.mark_complete();
        assert!(matches!(screen.status, TransferStatus::Complete));
    }

    #[test]
    fn test_progress_screen_failed() {
        let mut screen = TransferProgressScreen::upload("test.zip".to_string());
        screen.mark_failed();
        assert!(matches!(screen.status, TransferStatus::Failed));
    }

    #[test]
    fn test_progress_screen_cancelled() {
        let mut screen = TransferProgressScreen::upload("test.zip".to_string());
        screen.mark_cancelled();
        assert!(matches!(screen.status, TransferStatus::Cancelled));
    }

    #[test]
    fn test_progress_screen_render_waiting() {
        let screen = TransferProgressScreen::upload("test.zip".to_string());
        let output = screen.render();
        assert!(output.contains("test.zip"));
        assert!(output.contains("Waiting"));
    }

    #[test]
    fn test_progress_screen_render_progress() {
        let mut screen = TransferProgressScreen::upload("test.zip".to_string());
        screen.use_ansi = false;
        screen.start();
        screen.update_progress(500, 1000);

        let output = screen.render();

        assert!(output.contains("test.zip"));
        assert!(output.contains("50%"));
    }

    #[test]
    fn test_progress_screen_render_complete() {
        let mut screen = TransferProgressScreen::upload("test.zip".to_string());
        screen.use_ansi = false;
        screen.mark_complete();

        let output = screen.render();
        assert!(output.contains("complete"));
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(TransferProgressScreen::format_bytes(500), "500 B");
        assert_eq!(TransferProgressScreen::format_bytes(1024), "1.00 KB");
        assert_eq!(TransferProgressScreen::format_bytes(1536), "1.50 KB");
        assert_eq!(TransferProgressScreen::format_bytes(1048576), "1.00 MB");
        assert_eq!(TransferProgressScreen::format_bytes(1073741824), "1.00 GB");
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(
            TransferProgressScreen::format_duration(Duration::from_secs(30)),
            "30s"
        );
        assert_eq!(
            TransferProgressScreen::format_duration(Duration::from_secs(90)),
            "1:30"
        );
        assert_eq!(
            TransferProgressScreen::format_duration(Duration::from_secs(3661)),
            "1:01:01"
        );
    }

    #[test]
    fn test_progress_bar_render() {
        let mut screen = TransferProgressScreen::upload("test.zip".to_string());
        screen.use_ansi = false;
        screen.bar_width = 10;

        let bar = screen.render_progress_bar(50);
        assert_eq!(bar.len(), 10);
        assert!(bar.starts_with("====="));
    }

    #[test]
    fn test_transfer_progress_trait() {
        let mut screen = TransferProgressScreen::upload("".to_string());

        let file_info = ZmodemFileInfo::new("newfile.zip", 1000);

        screen.on_file_start(&file_info);
        assert_eq!(screen.filename, "newfile.zip");
        assert!(matches!(screen.status, TransferStatus::Initializing));

        let progress_info = FileProgress {
            file_index: 0,
            total_files: 1,
            file_path: PathBuf::from("newfile.zip"),
            file_name: "newfile.zip".to_string(),
            bytes_sent: 500,
            bytes_total: 1000,
            retries: 0,
        };

        screen.on_progress(&progress_info);
        match screen.status {
            TransferStatus::InProgress {
                bytes_transferred, ..
            } => {
                assert_eq!(bytes_transferred, 500);
            }
            _ => panic!("Expected InProgress"),
        }

        let stats = TransferStats::new(1000);
        screen.on_file_complete(&file_info, &stats);
        assert!(matches!(screen.status, TransferStatus::Complete));
    }

    #[test]
    fn test_transfer_progress_error() {
        let mut screen = TransferProgressScreen::upload("test.zip".to_string());

        screen.on_error(&ZmodemError::Timeout);
        assert!(matches!(screen.status, TransferStatus::Failed));
        assert!(screen.last_error.is_some());
    }
}
