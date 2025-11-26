//! Protocol implementations for Impulse BBS.
//!
//! This crate provides implementations of various file transfer and communication protocols
//! used in BBS systems.
//!
//! # Protocols
//!
//! ## Zmodem
//!
//! A robust file transfer protocol with error correction and resume capabilities.
//! See the [`zmodem`] module for details.
//!
//! ## Xmodem
//!
//! A simple and reliable file transfer protocol with three variants:
//! - Xmodem (Checksum): 128-byte blocks with simple checksum
//! - Xmodem-CRC: 128-byte blocks with CRC-16 error detection
//! - Xmodem-1K: 1024-byte blocks with CRC-16 error detection
//!
//! See the [`xmodem`] module for details.
//!
//! ## Ymodem
//!
//! An enhanced version of Xmodem-1K with batch file support:
//! - Multiple files in a single session
//! - File metadata (name, size, modification time)
//! - Always uses 1024-byte blocks and CRC-16
//!
//! See the [`ymodem`] module for details.
//!
//! # Examples
//!
//! ## Zmodem
//!
//! ```
//! use impulse_protocol::zmodem::{FrameType, FrameEncoding, ZmodemFrame};
//!
//! let frame = ZmodemFrame::with_defaults(FrameType::ZRINIT, FrameEncoding::Hex);
//! let serialized = frame.serialize();
//! ```
//!
//! ## Xmodem
//!
//! ```
//! use impulse_protocol::xmodem::{XmodemBlock, XmodemVariant};
//!
//! let data = vec![0x42; 128];
//! let block = XmodemBlock::new(1, data, XmodemVariant::Crc).unwrap();
//! let packet = block.serialize();
//! ```
//!
//! ## Ymodem
//!
//! ```
//! use impulse_protocol::ymodem::{FileMetadata, YmodemBatch};
//!
//! let mut batch = YmodemBatch::new();
//! batch.add_metadata(FileMetadata::with_size("file.txt", 12345));
//! ```

pub mod detection;
pub mod selection;
pub mod xmodem;
pub mod ymodem;
pub mod zmodem;

// Re-export commonly used types
pub use detection::{DetectedProtocol, ProtocolDetector};
pub use selection::{FileProtocol, ProtocolPreferences};
