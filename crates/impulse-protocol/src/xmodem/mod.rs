//! Xmodem protocol implementation.
//!
//! This module provides a complete implementation of the Xmodem file transfer protocol
//! and its variants (Xmodem-CRC and Xmodem-1K).
//!
//! # Overview
//!
//! Xmodem is a simple file transfer protocol that provides reliable data transfer with:
//! - Block-based transmission (128 or 1024 bytes per block)
//! - Error detection (simple checksum or CRC-16)
//! - Automatic retry on errors
//! - Stop-and-wait flow control
//!
//! # Protocol Variants
//!
//! Three variants are supported:
//!
//! - **Xmodem (Checksum)**: Original protocol with 128-byte blocks and simple checksum
//! - **Xmodem-CRC**: Enhanced version with 128-byte blocks and CRC-16 error detection
//! - **Xmodem-1K**: High-performance version with 1024-byte blocks and CRC-16
//!
//! # Protocol Structure
//!
//! Each block consists of:
//! - 1 byte: Start-of-header (SOH for 128-byte, STX for 1024-byte)
//! - 1 byte: Block number (0-255, wrapping)
//! - 1 byte: Block number complement (255 - block number)
//! - N bytes: Data (128 or 1024 bytes, padded with SUB if needed)
//! - 1 or 2 bytes: Checksum (1 byte) or CRC-16 (2 bytes)
//!
//! # Control Bytes
//!
//! - `SOH (0x01)`: Start of 128-byte block
//! - `STX (0x02)`: Start of 1024-byte block
//! - `EOT (0x04)`: End of transmission
//! - `ACK (0x06)`: Acknowledge (positive response)
//! - `NAK (0x15)`: Not acknowledge (negative response, request retransmit)
//! - `CAN (0x18)`: Cancel transfer
//! - `'C' (0x43)`: Request CRC mode (sent by receiver to initiate CRC mode)
//!
//! # Examples
//!
//! ## Creating and serializing a block
//!
//! ```
//! use impulse_protocol::xmodem::{XmodemBlock, XmodemVariant};
//!
//! let data = vec![0x42; 128];
//! let block = XmodemBlock::new(1, data, XmodemVariant::Crc).unwrap();
//! let packet = block.serialize();
//! ```
//!
//! ## Calculating checksums
//!
//! ```
//! use impulse_protocol::xmodem::{checksum, crc};
//!
//! let data = b"Hello, Xmodem!";
//! let checksum_value = checksum::calculate(data);
//! let crc_value = crc::calculate(data);
//! ```

pub mod block;
pub mod checksum;
pub mod crc;
pub mod error;
pub mod receive;
pub mod send;
pub mod variants;

// Re-export commonly used types
pub use block::XmodemBlock;
pub use error::{Result, XmodemError};
pub use receive::{ReceiveStats, ReceiverConfig, XmodemReceiver};
pub use send::{SendStats, SenderConfig, XmodemSender};
pub use variants::XmodemVariant;

/// Start of 128-byte block header.
pub const SOH: u8 = 0x01;

/// Start of 1024-byte block header.
pub const STX: u8 = 0x02;

/// End of transmission.
pub const EOT: u8 = 0x04;

/// Acknowledge (positive response).
pub const ACK: u8 = 0x06;

/// Not acknowledge (negative response, request retransmit).
pub const NAK: u8 = 0x15;

/// Cancel transfer.
pub const CAN: u8 = 0x18;

/// Padding byte for partial blocks.
pub const SUB: u8 = 0x1A;

/// CRC mode request (sent by receiver).
pub const CRC_MODE: u8 = b'C';

/// Maximum retry attempts for a single block.
pub const MAX_RETRIES: usize = 10;

/// Timeout for waiting for response (in milliseconds).
pub const TIMEOUT_MS: u64 = 10000;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(SOH, 0x01);
        assert_eq!(STX, 0x02);
        assert_eq!(EOT, 0x04);
        assert_eq!(ACK, 0x06);
        assert_eq!(NAK, 0x15);
        assert_eq!(CAN, 0x18);
        assert_eq!(SUB, 0x1A);
        assert_eq!(CRC_MODE, b'C');
    }

    #[test]
    fn test_block_creation() {
        let data = vec![0x42; 128];
        let block = XmodemBlock::new(1, data, XmodemVariant::Checksum).unwrap();
        assert_eq!(block.block_num, 1);
    }

    #[test]
    fn test_variant_block_sizes() {
        assert_eq!(XmodemVariant::Checksum.block_size(), 128);
        assert_eq!(XmodemVariant::Crc.block_size(), 128);
        assert_eq!(XmodemVariant::OneK.block_size(), 1024);
    }

    #[test]
    fn test_checksum_calculation() {
        let data = b"Test";
        let checksum_val = checksum::calculate(data);
        assert_ne!(checksum_val, 0);
    }

    #[test]
    fn test_crc_calculation() {
        let data = b"Test";
        let crc_val = crc::calculate(data);
        assert_ne!(crc_val, 0);
    }

    #[test]
    fn test_error_types() {
        let err = XmodemError::Timeout;
        assert!(err.to_string().contains("Timeout"));

        let err = XmodemError::Cancelled;
        assert!(err.to_string().contains("cancelled"));
    }

    #[test]
    fn test_max_retries() {
        // Verify the constant is set to the expected value
        assert_eq!(MAX_RETRIES, 10);
    }

    #[test]
    fn test_timeout() {
        // Verify the timeout is set to the expected value (10 seconds)
        assert_eq!(TIMEOUT_MS, 10000);
    }

    #[test]
    fn test_round_trip_serialization() {
        for variant in [
            XmodemVariant::Checksum,
            XmodemVariant::Crc,
            XmodemVariant::OneK,
        ] {
            let data = vec![0x55; variant.block_size()];
            let block = XmodemBlock::new(42, data.clone(), variant).unwrap();
            let packet = block.serialize();
            let deserialized = XmodemBlock::deserialize(&packet, variant.uses_crc()).unwrap();

            assert_eq!(deserialized.block_num, 42);
            assert_eq!(deserialized.data, data);
            assert_eq!(deserialized.variant, variant);
        }
    }
}
