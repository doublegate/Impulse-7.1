//! Ymodem protocol implementation.
//!
//! This module provides an implementation of the Ymodem file transfer protocol,
//! which extends Xmodem-1K with batch file support and file metadata.
//!
//! # Overview
//!
//! Ymodem builds on Xmodem-1K to provide:
//! - **Batch transfers**: Multiple files in a single session
//! - **File metadata**: Name, size, modification time, mode
//! - **Block 0**: Special initial block containing file information
//! - **CRC-16**: Always uses CRC-16 for error detection (like Xmodem-1K)
//!
//! # Protocol Structure
//!
//! Each file transfer begins with:
//! 1. **Block 0**: 128 bytes containing file metadata
//!    - Format: `filename\0size mod_time mode\0`
//!    - Filename is null-terminated
//!    - Metadata fields are space-separated, collectively null-terminated
//! 2. **Data blocks**: Xmodem-1K blocks (1024 bytes each) with file data
//! 3. **EOT**: End of transmission for this file
//!
//! Multiple files can be sent in sequence. An empty block 0 (all zeros)
//! signals the end of the batch.
//!
//! # Examples
//!
//! ## Creating file metadata
//!
//! ```
//! use impulse_protocol::ymodem::FileMetadata;
//!
//! let metadata = FileMetadata::with_size("document.pdf", 12345);
//! let block0 = metadata.encode();
//! ```
//!
//! ## Creating a batch
//!
//! ```
//! use impulse_protocol::ymodem::{YmodemBatch, FileMetadata, BatchFile};
//!
//! let mut batch = YmodemBatch::new();
//! batch.add_metadata(FileMetadata::with_size("file1.txt", 1000));
//! batch.add_metadata(FileMetadata::with_size("file2.txt", 2000));
//! assert_eq!(batch.len(), 2);
//! ```

pub mod batch;
pub mod metadata;
pub mod streaming;

// Re-export commonly used types
pub use batch::{BatchFile, YmodemBatch};
pub use metadata::FileMetadata;
pub use streaming::{BatchStats, StreamingConfig, YmodemGReceiver, YmodemGSender};

/// Ymodem always uses 1024-byte blocks (same as Xmodem-1K).
pub const BLOCK_SIZE: usize = 1024;

/// Block 0 is always 128 bytes (metadata block).
pub const BLOCK0_SIZE: usize = 128;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(BLOCK_SIZE, 1024);
        assert_eq!(BLOCK0_SIZE, 128);
    }

    #[test]
    fn test_file_metadata_creation() {
        let metadata = FileMetadata::new("test.txt");
        assert_eq!(metadata.name, "test.txt");
    }

    #[test]
    fn test_batch_creation() {
        let batch = YmodemBatch::new();
        assert!(batch.is_empty());
    }

    #[test]
    fn test_metadata_encoding() {
        let metadata = FileMetadata::with_size("test.txt", 12345);
        let encoded = metadata.encode();
        assert_eq!(encoded.len(), BLOCK0_SIZE);
    }

    #[test]
    fn test_end_of_batch() {
        let eob = FileMetadata::end_of_batch();
        assert_eq!(eob.len(), BLOCK0_SIZE);
        assert!(eob.iter().all(|&b| b == 0));
    }

    #[test]
    fn test_batch_file_creation() {
        let metadata = FileMetadata::new("file.dat");
        let file = BatchFile::new(metadata);
        assert_eq!(file.name(), "file.dat");
    }
}
