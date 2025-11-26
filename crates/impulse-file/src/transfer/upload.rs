//! Upload integration with file areas
//!
//! This module provides the upload manager that coordinates between
//! the Zmodem protocol and file area processing, including FILE_ID.DIZ extraction.

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use impulse_protocol::zmodem::{ReceiverConfig, TransferProgress, ZmodemReceiver};
use tokio::io::{AsyncRead, AsyncWrite};

use super::{Protocol, TransferConfig, TransferStatus};
use crate::diz;
use crate::error::Result;
use crate::types::FileArea;

/// Result of a completed upload operation
#[derive(Debug, Clone)]
pub struct UploadResult {
    /// Path where file was saved
    pub file_path: PathBuf,

    /// Original filename
    pub filename: String,

    /// Total bytes received
    pub bytes_received: u64,

    /// Transfer duration
    pub duration: Duration,

    /// Average transfer speed (bytes/sec)
    pub speed_bps: u64,

    /// Whether this was a resumed transfer
    pub was_resumed: bool,

    /// Position where resume started (if resumed)
    pub resume_offset: u64,

    /// Extracted FILE_ID.DIZ description (if found)
    pub description: Option<String>,

    /// Final status
    pub status: TransferStatus,
}

impl UploadResult {
    /// Create a successful upload result
    pub fn success(
        file_path: PathBuf,
        filename: String,
        bytes_received: u64,
        duration: Duration,
        was_resumed: bool,
        resume_offset: u64,
        description: Option<String>,
    ) -> Self {
        let speed_bps = if duration.as_secs() > 0 {
            bytes_received / duration.as_secs()
        } else {
            bytes_received
        };

        Self {
            file_path,
            filename,
            bytes_received,
            duration,
            speed_bps,
            was_resumed,
            resume_offset,
            description,
            status: TransferStatus::Completed,
        }
    }

    /// Create a failed upload result
    pub fn failed(filename: String) -> Self {
        Self {
            file_path: PathBuf::new(),
            filename,
            bytes_received: 0,
            duration: Duration::ZERO,
            speed_bps: 0,
            was_resumed: false,
            resume_offset: 0,
            description: None,
            status: TransferStatus::Failed,
        }
    }
}

/// Upload manager for coordinating file transfers
///
/// This struct manages the upload process, connecting the protocol
/// to file area storage and FILE_ID.DIZ extraction.
pub struct UploadManager<S> {
    /// The underlying stream for protocol communication
    stream: S,

    /// Transfer configuration
    config: TransferConfig,

    /// Current transfer status
    status: TransferStatus,

    /// Start time of current transfer
    start_time: Option<Instant>,

    /// Target file area for uploads
    target_area: Option<FileArea>,

    /// Upload directory path
    upload_dir: PathBuf,
}

impl<S: AsyncRead + AsyncWrite + Unpin> UploadManager<S> {
    /// Create a new upload manager
    pub fn new(stream: S, config: TransferConfig, upload_dir: PathBuf) -> Self {
        Self {
            stream,
            config,
            status: TransferStatus::Idle,
            start_time: None,
            target_area: None,
            upload_dir,
        }
    }

    /// Set the target file area for uploads
    pub fn set_target_area(&mut self, area: FileArea) {
        self.target_area = Some(area);
    }

    /// Get the current transfer status
    pub fn status(&self) -> TransferStatus {
        self.status
    }

    /// Receive a single file using the configured protocol
    ///
    /// # Arguments
    ///
    /// * `progress` - Progress callback
    ///
    /// # Returns
    ///
    /// Result containing upload information including extracted description
    pub async fn receive_file<P: TransferProgress>(
        &mut self,
        progress: &mut P,
    ) -> Result<UploadResult> {
        self.status = TransferStatus::Initializing;
        self.start_time = Some(Instant::now());

        match self.config.protocol {
            Protocol::Zmodem => self.receive_zmodem(progress).await,
            Protocol::Xmodem | Protocol::Ymodem | Protocol::YmodemG => {
                // Placeholder for other protocols
                Ok(UploadResult::failed("unknown".to_string()))
            }
        }
    }

    /// Receive using Zmodem protocol
    async fn receive_zmodem<P: TransferProgress>(
        &mut self,
        _progress: &mut P,
    ) -> Result<UploadResult> {
        // Create Zmodem receiver config from transfer config
        let receiver_config = ReceiverConfig {
            buffer_size: self.config.buffer_size,
            timeout_ms: self.config.timeout_ms,
            max_retries: self.config.max_retries,
            use_crc32: self.config.use_crc32,
            escape_control: self.config.escape_control,
            escape_8bit: self.config.escape_8bit,
            allow_resume: self.config.enable_resume,
            overwrite_existing: self.config.overwrite_existing,
        };

        // We need to take ownership of stream temporarily
        let stream = std::mem::replace(&mut self.stream, unsafe {
            // This is a placeholder - in production, use proper stream handling
            std::mem::zeroed()
        });

        let mut receiver = ZmodemReceiver::new(stream, receiver_config);

        // Initialize the protocol
        self.status = TransferStatus::Initializing;
        let _params = match receiver.init().await {
            Ok(params) => params,
            Err(_) => {
                self.status = TransferStatus::Failed;
                return Ok(UploadResult::failed("unknown".to_string()));
            }
        };

        // Receive files
        self.status = TransferStatus::InProgress;
        let received_files = match receiver.receive_files(&self.upload_dir).await {
            Ok(files) => files,
            Err(_) => {
                self.status = TransferStatus::Failed;
                return Ok(UploadResult::failed("unknown".to_string()));
            }
        };

        // Finish the session
        if receiver.finish().await.is_err() {
            self.status = TransferStatus::Failed;
            return Ok(UploadResult::failed("unknown".to_string()));
        }

        // Process the first received file (for single file upload)
        if let Some(received) = received_files.into_iter().next() {
            self.status = TransferStatus::Completed;
            let duration = self.start_time.map(|t| t.elapsed()).unwrap_or_default();

            // Try to extract FILE_ID.DIZ
            let description = self.extract_description(&received.saved_path).await;

            Ok(UploadResult::success(
                received.saved_path,
                received.file_info.name.clone(),
                received.stats.bytes_received,
                duration,
                received.stats.was_resumed,
                received.stats.resume_position,
                description,
            ))
        } else {
            self.status = TransferStatus::Failed;
            Ok(UploadResult::failed("unknown".to_string()))
        }
    }

    /// Extract FILE_ID.DIZ from an uploaded file
    async fn extract_description(&self, file_path: &Path) -> Option<String> {
        // Use the auto-detecting extraction function
        diz::extract_file_id_diz(file_path).await.ok().flatten()
    }

    /// Receive multiple files in batch mode
    ///
    /// # Arguments
    ///
    /// * `progress` - Progress callback
    ///
    /// # Returns
    ///
    /// Vector of upload results for each file received
    pub async fn receive_batch<P: TransferProgress>(
        &mut self,
        progress: &mut P,
    ) -> Vec<UploadResult> {
        self.status = TransferStatus::Initializing;
        self.start_time = Some(Instant::now());

        match self.config.protocol {
            Protocol::Zmodem => self.receive_batch_zmodem(progress).await,
            _ => vec![UploadResult::failed("unsupported".to_string())],
        }
    }

    /// Receive batch using Zmodem protocol
    async fn receive_batch_zmodem<P: TransferProgress>(
        &mut self,
        _progress: &mut P,
    ) -> Vec<UploadResult> {
        let receiver_config = ReceiverConfig {
            buffer_size: self.config.buffer_size,
            timeout_ms: self.config.timeout_ms,
            max_retries: self.config.max_retries,
            use_crc32: self.config.use_crc32,
            escape_control: self.config.escape_control,
            escape_8bit: self.config.escape_8bit,
            allow_resume: self.config.enable_resume,
            overwrite_existing: self.config.overwrite_existing,
        };

        let stream = std::mem::replace(&mut self.stream, unsafe { std::mem::zeroed() });

        let mut receiver = ZmodemReceiver::new(stream, receiver_config);

        // Initialize
        self.status = TransferStatus::Initializing;
        if receiver.init().await.is_err() {
            self.status = TransferStatus::Failed;
            return vec![UploadResult::failed("init_failed".to_string())];
        }

        // Receive all files
        self.status = TransferStatus::InProgress;
        let received_files = match receiver.receive_files(&self.upload_dir).await {
            Ok(files) => files,
            Err(_) => {
                self.status = TransferStatus::Failed;
                return vec![UploadResult::failed("receive_failed".to_string())];
            }
        };

        // Finish session
        let _ = receiver.finish().await;

        let duration = self.start_time.map(|t| t.elapsed()).unwrap_or_default();

        // Process each received file
        let mut results = Vec::with_capacity(received_files.len());
        for received in received_files {
            let description = self.extract_description(&received.saved_path).await;

            results.push(UploadResult::success(
                received.saved_path,
                received.file_info.name.clone(),
                received.stats.bytes_received,
                duration,
                received.stats.was_resumed,
                received.stats.resume_position,
                description,
            ));
        }

        self.status = TransferStatus::Completed;
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use std::path::PathBuf;

    #[test]
    fn test_upload_result_success() {
        let result = UploadResult::success(
            PathBuf::from("/uploads/test.zip"),
            "test.zip".to_string(),
            1024 * 1024, // 1048576 bytes
            Duration::from_secs(10),
            false,
            0,
            Some("A test file".to_string()),
        );

        assert_eq!(result.bytes_received, 1024 * 1024);
        // 1048576 / 10 = 104857 (integer division)
        assert_eq!(result.speed_bps, 104857);
        assert!(!result.was_resumed);
        assert_eq!(result.description, Some("A test file".to_string()));
        assert_eq!(result.status, TransferStatus::Completed);
    }

    #[test]
    fn test_upload_result_failed() {
        let result = UploadResult::failed("test.zip".to_string());

        assert_eq!(result.bytes_received, 0);
        assert!(result.description.is_none());
        assert_eq!(result.status, TransferStatus::Failed);
    }

    #[test]
    fn test_upload_manager_creation() {
        let stream = Cursor::new(Vec::new());
        let config = TransferConfig::default();
        let manager = UploadManager::new(stream, config, PathBuf::from("/uploads"));

        assert_eq!(manager.status(), TransferStatus::Idle);
    }

    #[test]
    fn test_upload_manager_set_area() {
        let stream = Cursor::new(Vec::new());
        let config = TransferConfig::default();
        let mut manager = UploadManager::new(stream, config, PathBuf::from("/uploads"));

        let area = FileArea::new(1, "Games".to_string(), "Game files".to_string());
        manager.set_target_area(area);

        assert!(manager.target_area.is_some());
        assert_eq!(manager.target_area.as_ref().unwrap().name, "Games");
    }
}
