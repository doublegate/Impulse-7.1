//! File transfer protocol integration
//!
//! This module provides integration between the file system and transfer protocols
//! (Zmodem, etc.), coordinating downloads, uploads, and statistics tracking.
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
//! │  File Browser   │───▶│ TransferManager  │───▶│ ZmodemSender    │
//! │  (user selects) │    │ (coordinates)    │    │ (protocol)      │
//! └─────────────────┘    └──────────────────┘    └─────────────────┘
//!                               │
//!                               ▼
//!                        ┌──────────────────┐
//!                        │ TransferProgress │
//!                        │ (UI callback)    │
//!                        └──────────────────┘
//! ```
//!
//! # Examples
//!
//! ## Initiating a download
//!
//! ```no_run
//! use impulse_file::transfer::{DownloadManager, TransferConfig, Protocol};
//! use std::path::Path;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // This would normally use an actual stream
//! // let manager = DownloadManager::new(stream, TransferConfig::default());
//! // manager.download_file(Path::new("/files/game.zip"), &mut progress).await?;
//! # Ok(())
//! # }
//! ```

mod download;
mod stats;
mod upload;

pub use download::{DownloadManager, DownloadResult, prepare_download};
pub use stats::{DownloadStats, TransferStats};
pub use upload::{UploadManager, UploadResult};

/// Supported file transfer protocols
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Protocol {
    /// Zmodem protocol (default, most reliable)
    #[default]
    Zmodem,

    /// Xmodem protocol (legacy, 128-byte blocks)
    Xmodem,

    /// Ymodem protocol (batch mode Xmodem)
    Ymodem,

    /// Ymodem-G protocol (streaming, no error correction)
    YmodemG,
}

impl Protocol {
    /// Get the protocol name for display
    pub fn name(&self) -> &'static str {
        match self {
            Protocol::Zmodem => "Zmodem",
            Protocol::Xmodem => "Xmodem",
            Protocol::Ymodem => "Ymodem",
            Protocol::YmodemG => "Ymodem-G",
        }
    }

    /// Check if this protocol supports crash recovery
    pub fn supports_resume(&self) -> bool {
        matches!(self, Protocol::Zmodem)
    }

    /// Check if this protocol supports batch transfers
    pub fn supports_batch(&self) -> bool {
        matches!(
            self,
            Protocol::Zmodem | Protocol::Ymodem | Protocol::YmodemG
        )
    }
}

/// Configuration for file transfers
#[derive(Debug, Clone)]
pub struct TransferConfig {
    /// Protocol to use for transfers
    pub protocol: Protocol,

    /// Buffer size for transfers (bytes)
    pub buffer_size: usize,

    /// Timeout for protocol operations (milliseconds)
    pub timeout_ms: u64,

    /// Maximum retries on error
    pub max_retries: u32,

    /// Enable crash recovery/resume
    pub enable_resume: bool,

    /// Enable CRC-32 (vs CRC-16)
    pub use_crc32: bool,

    /// Escape control characters
    pub escape_control: bool,

    /// Escape 8-bit characters
    pub escape_8bit: bool,

    /// Overwrite existing files on receive
    pub overwrite_existing: bool,
}

impl Default for TransferConfig {
    fn default() -> Self {
        Self {
            protocol: Protocol::Zmodem,
            buffer_size: 8192,
            timeout_ms: 30_000,
            max_retries: 10,
            enable_resume: true,
            use_crc32: true,
            escape_control: true,
            escape_8bit: false,
            overwrite_existing: false,
        }
    }
}

impl TransferConfig {
    /// Create a new transfer configuration with defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the protocol
    pub fn with_protocol(mut self, protocol: Protocol) -> Self {
        self.protocol = protocol;
        self
    }

    /// Set the buffer size
    pub fn with_buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = size;
        self
    }

    /// Set the timeout
    pub fn with_timeout(mut self, timeout_ms: u64) -> Self {
        self.timeout_ms = timeout_ms;
        self
    }

    /// Enable or disable resume
    pub fn with_resume(mut self, enable: bool) -> Self {
        self.enable_resume = enable;
        self
    }
}

/// Transfer direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferDirection {
    /// Sending file to remote (download from BBS)
    Send,

    /// Receiving file from remote (upload to BBS)
    Receive,
}

/// Overall transfer status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferStatus {
    /// Transfer not started
    Idle,

    /// Initializing protocol handshake
    Initializing,

    /// Transferring data
    InProgress,

    /// Transfer completed successfully
    Completed,

    /// Transfer failed
    Failed,

    /// Transfer cancelled by user
    Cancelled,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_default() {
        let protocol = Protocol::default();
        assert_eq!(protocol, Protocol::Zmodem);
    }

    #[test]
    fn test_protocol_name() {
        assert_eq!(Protocol::Zmodem.name(), "Zmodem");
        assert_eq!(Protocol::Xmodem.name(), "Xmodem");
        assert_eq!(Protocol::Ymodem.name(), "Ymodem");
        assert_eq!(Protocol::YmodemG.name(), "Ymodem-G");
    }

    #[test]
    fn test_protocol_supports_resume() {
        assert!(Protocol::Zmodem.supports_resume());
        assert!(!Protocol::Xmodem.supports_resume());
        assert!(!Protocol::Ymodem.supports_resume());
        assert!(!Protocol::YmodemG.supports_resume());
    }

    #[test]
    fn test_protocol_supports_batch() {
        assert!(Protocol::Zmodem.supports_batch());
        assert!(!Protocol::Xmodem.supports_batch());
        assert!(Protocol::Ymodem.supports_batch());
        assert!(Protocol::YmodemG.supports_batch());
    }

    #[test]
    fn test_transfer_config_default() {
        let config = TransferConfig::default();
        assert_eq!(config.protocol, Protocol::Zmodem);
        assert_eq!(config.buffer_size, 8192);
        assert_eq!(config.timeout_ms, 30_000);
        assert_eq!(config.max_retries, 10);
        assert!(config.enable_resume);
        assert!(config.use_crc32);
    }

    #[test]
    fn test_transfer_config_builder() {
        let config = TransferConfig::new()
            .with_protocol(Protocol::Ymodem)
            .with_buffer_size(4096)
            .with_timeout(60_000)
            .with_resume(false);

        assert_eq!(config.protocol, Protocol::Ymodem);
        assert_eq!(config.buffer_size, 4096);
        assert_eq!(config.timeout_ms, 60_000);
        assert!(!config.enable_resume);
    }

    #[test]
    fn test_transfer_direction() {
        assert_eq!(TransferDirection::Send, TransferDirection::Send);
        assert_ne!(TransferDirection::Send, TransferDirection::Receive);
    }

    #[test]
    fn test_transfer_status() {
        let status = TransferStatus::Idle;
        assert_eq!(status, TransferStatus::Idle);
        assert_ne!(status, TransferStatus::InProgress);
    }
}
