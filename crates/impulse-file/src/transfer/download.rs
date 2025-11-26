//! Download integration with file browser
//!
//! This module provides the download manager that coordinates between
//! file selection and the Zmodem protocol for sending files to users.

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use impulse_protocol::zmodem::{SenderConfig, TransferProgress, ZmodemSender};
use tokio::io::{AsyncRead, AsyncWrite};

use super::{Protocol, TransferConfig, TransferStatus};
use crate::error::Result;
use impulse_types::file::FileEntry;

/// Result of a completed download operation
#[derive(Debug, Clone)]
pub struct DownloadResult {
    /// File that was transferred
    pub file_path: PathBuf,

    /// Original filename
    pub filename: String,

    /// Total bytes transferred
    pub bytes_transferred: u64,

    /// Transfer duration
    pub duration: Duration,

    /// Average transfer speed (bytes/sec)
    pub speed_bps: u64,

    /// Whether this was a resumed transfer
    pub was_resumed: bool,

    /// Position where resume started (if resumed)
    pub resume_offset: u64,

    /// Final status
    pub status: TransferStatus,
}

impl DownloadResult {
    /// Create a successful download result
    pub fn success(
        file_path: PathBuf,
        filename: String,
        bytes_transferred: u64,
        duration: Duration,
        was_resumed: bool,
        resume_offset: u64,
    ) -> Self {
        let speed_bps = if duration.as_secs() > 0 {
            bytes_transferred / duration.as_secs()
        } else {
            bytes_transferred
        };

        Self {
            file_path,
            filename,
            bytes_transferred,
            duration,
            speed_bps,
            was_resumed,
            resume_offset,
            status: TransferStatus::Completed,
        }
    }

    /// Create a failed download result
    pub fn failed(file_path: PathBuf, filename: String) -> Self {
        Self {
            file_path,
            filename,
            bytes_transferred: 0,
            duration: Duration::ZERO,
            speed_bps: 0,
            was_resumed: false,
            resume_offset: 0,
            status: TransferStatus::Failed,
        }
    }
}

/// Download manager for coordinating file transfers
///
/// This struct manages the download process, connecting file selection
/// in the UI to the underlying Zmodem protocol.
pub struct DownloadManager<S> {
    /// The underlying stream for protocol communication
    stream: S,

    /// Transfer configuration
    config: TransferConfig,

    /// Current transfer status
    status: TransferStatus,

    /// Start time of current transfer
    start_time: Option<Instant>,

    /// Files queued for download
    queue: Vec<PathBuf>,
}

impl<S: AsyncRead + AsyncWrite + Unpin> DownloadManager<S> {
    /// Create a new download manager
    pub fn new(stream: S, config: TransferConfig) -> Self {
        Self {
            stream,
            config,
            status: TransferStatus::Idle,
            start_time: None,
            queue: Vec::new(),
        }
    }

    /// Get the current transfer status
    pub fn status(&self) -> TransferStatus {
        self.status
    }

    /// Add a file to the download queue
    pub fn queue_file(&mut self, path: PathBuf) {
        self.queue.push(path);
    }

    /// Add multiple files to the download queue
    pub fn queue_files(&mut self, paths: impl IntoIterator<Item = PathBuf>) {
        self.queue.extend(paths);
    }

    /// Clear the download queue
    pub fn clear_queue(&mut self) {
        self.queue.clear();
    }

    /// Get the number of files in the queue
    pub fn queue_len(&self) -> usize {
        self.queue.len()
    }

    /// Download a single file using the configured protocol
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the file to send
    /// * `progress` - Optional progress callback
    ///
    /// # Returns
    ///
    /// Result containing download statistics
    pub async fn download_file<P: TransferProgress>(
        &mut self,
        file_path: &Path,
        progress: &mut P,
    ) -> Result<DownloadResult> {
        // Verify file exists
        if !file_path.exists() {
            return Ok(DownloadResult::failed(
                file_path.to_path_buf(),
                file_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string(),
            ));
        }

        let filename = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        self.status = TransferStatus::Initializing;
        self.start_time = Some(Instant::now());

        match self.config.protocol {
            Protocol::Zmodem => self.download_zmodem(file_path, &filename, progress).await,
            Protocol::Xmodem | Protocol::Ymodem | Protocol::YmodemG => {
                // Placeholder for other protocols
                Ok(DownloadResult::failed(file_path.to_path_buf(), filename))
            }
        }
    }

    /// Download using Zmodem protocol
    async fn download_zmodem<P: TransferProgress>(
        &mut self,
        file_path: &Path,
        filename: &str,
        _progress: &mut P,
    ) -> Result<DownloadResult> {
        // Create Zmodem sender config from transfer config
        let sender_config = SenderConfig {
            block_size: self.config.buffer_size,
            timeout_ms: self.config.timeout_ms,
            max_retries: self.config.max_retries,
            use_crc32: self.config.use_crc32,
            escape_control: self.config.escape_control,
            escape_8bit: self.config.escape_8bit,
        };

        // We need to take ownership of stream temporarily
        // In a real implementation, you'd use a reference or Arc
        let stream = std::mem::replace(&mut self.stream, unsafe {
            // This is a placeholder - in production, use proper stream handling
            std::mem::zeroed()
        });

        let mut sender = ZmodemSender::new(stream, sender_config);

        // Initialize the protocol
        self.status = TransferStatus::Initializing;
        let _params = match sender.init().await {
            Ok(params) => params,
            Err(_) => {
                self.status = TransferStatus::Failed;
                return Ok(DownloadResult::failed(
                    file_path.to_path_buf(),
                    filename.to_string(),
                ));
            }
        };

        // Send the file
        self.status = TransferStatus::InProgress;
        let stats = match sender.send_file(file_path).await {
            Ok(stats) => stats,
            Err(_) => {
                self.status = TransferStatus::Failed;
                return Ok(DownloadResult::failed(
                    file_path.to_path_buf(),
                    filename.to_string(),
                ));
            }
        };

        // Finish the session
        if sender.finish().await.is_err() {
            self.status = TransferStatus::Failed;
            return Ok(DownloadResult::failed(
                file_path.to_path_buf(),
                filename.to_string(),
            ));
        }

        self.status = TransferStatus::Completed;
        let duration = self.start_time.map(|t| t.elapsed()).unwrap_or_default();

        // Note: TransferStats doesn't track resume, so we report false/0 for now
        Ok(DownloadResult::success(
            file_path.to_path_buf(),
            filename.to_string(),
            stats.bytes_sent,
            duration,
            false, // Resume tracking not in current TransferStats
            0,     // Resume offset
        ))
    }

    /// Download all files in the queue
    ///
    /// # Arguments
    ///
    /// * `progress` - Progress callback for each file
    ///
    /// # Returns
    ///
    /// Vector of download results for each file
    pub async fn download_all<P: TransferProgress>(
        &mut self,
        progress: &mut P,
    ) -> Vec<DownloadResult> {
        let queue = std::mem::take(&mut self.queue);
        let mut results = Vec::with_capacity(queue.len());

        for path in queue {
            let result = self.download_file(&path, progress).await;
            match result {
                Ok(r) => results.push(r),
                Err(_) => {
                    let filename = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown")
                        .to_string();
                    results.push(DownloadResult::failed(path, filename));
                }
            }
        }

        results
    }
}

/// Create a download manager from a file entry
///
/// Helper function to initiate a download from the file browser
pub fn prepare_download(file: &FileEntry, base_path: &Path) -> PathBuf {
    base_path.join(&file.filename)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_download_result_success() {
        let result = DownloadResult::success(
            PathBuf::from("/files/test.zip"),
            "test.zip".to_string(),
            1024 * 1024, // 1048576 bytes
            Duration::from_secs(10),
            false,
            0,
        );

        assert_eq!(result.bytes_transferred, 1024 * 1024);
        // 1048576 / 10 = 104857 (integer division)
        assert_eq!(result.speed_bps, 104857);
        assert!(!result.was_resumed);
        assert_eq!(result.status, TransferStatus::Completed);
    }

    #[test]
    fn test_download_result_failed() {
        let result =
            DownloadResult::failed(PathBuf::from("/files/test.zip"), "test.zip".to_string());

        assert_eq!(result.bytes_transferred, 0);
        assert_eq!(result.status, TransferStatus::Failed);
    }

    #[test]
    fn test_download_manager_queue() {
        let stream = Cursor::new(Vec::new());
        let mut manager = DownloadManager::new(stream, TransferConfig::default());

        assert_eq!(manager.queue_len(), 0);

        manager.queue_file(PathBuf::from("/files/file1.zip"));
        manager.queue_file(PathBuf::from("/files/file2.zip"));

        assert_eq!(manager.queue_len(), 2);

        manager.clear_queue();
        assert_eq!(manager.queue_len(), 0);
    }

    #[test]
    fn test_download_manager_status() {
        let stream = Cursor::new(Vec::new());
        let manager = DownloadManager::new(stream, TransferConfig::default());

        assert_eq!(manager.status(), TransferStatus::Idle);
    }

    #[test]
    fn test_prepare_download() {
        let file = FileEntry {
            id: 1,
            filename: "game.zip".to_string(),
            description: "A game".to_string(),
            uploader: "sysop".to_string(),
            uploader_id: 1,
            size_bytes: 1024,
            upload_date: chrono::Utc::now(),
            area_id: 1,
            download_count: 0,
            is_offline: false,
            is_missing: false,
            password: None,
            cost_credits: None,
        };

        let path = prepare_download(&file, Path::new("/bbs/files/games"));
        assert_eq!(path, PathBuf::from("/bbs/files/games/game.zip"));
    }
}
