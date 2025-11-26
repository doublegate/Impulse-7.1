//! Zmodem protocol implementation.
//!
//! This module provides a complete implementation of the Zmodem file transfer protocol,
//! including frame handling, CRC calculation, and data escaping.
//!
//! # Overview
//!
//! Zmodem is a file transfer protocol that provides reliable data transfer with:
//! - Multiple CRC options (CRC-16 and CRC-32)
//! - Automatic crash recovery
//! - Streaming protocol with minimal turnaround delays
//! - Full duplex operation
//!
//! # Protocol Structure
//!
//! The protocol uses frames for control and data transfer:
//! - Frames contain a type, encoding, flags, and optional data
//! - Three encoding types: Hex (human readable), Binary with CRC-16, Binary with CRC-32
//! - ZDLE escaping ensures transparency over various communication channels
//!
//! # Modules
//!
//! - `error` - Error types for Zmodem operations
//! - `frame` - Frame structure and serialization
//! - `crc16` - CRC-16/XMODEM checksum calculation
//! - `crc32` - CRC-32/ZMODEM checksum calculation
//! - `escape` - ZDLE encoding and decoding
//!
//! # Examples
//!
//! ## Creating and serializing a frame
//!
//! ```
//! use impulse_protocol::zmodem::{FrameType, FrameEncoding, ZmodemFrame};
//!
//! let frame = ZmodemFrame::with_defaults(FrameType::ZRINIT, FrameEncoding::Hex);
//! let serialized = frame.serialize();
//! ```
//!
//! ## Calculating checksums
//!
//! ```
//! use impulse_protocol::zmodem::{crc16, crc32};
//!
//! let data = b"Hello, Zmodem!";
//! let crc16_value = crc16::calculate(data);
//! let crc32_value = crc32::calculate(data);
//! ```
//!
//! ## Encoding and decoding data
//!
//! ```
//! use impulse_protocol::zmodem::escape;
//!
//! let original = b"Data with \x11 XON";
//! let encoded = escape::encode(original);
//! let decoded = escape::decode(&encoded).unwrap();
//! assert_eq!(decoded, original);
//! ```

pub mod crc16;
pub mod crc32;
pub mod error;
pub mod escape;
pub mod file;
pub mod frame;
pub mod init;
pub mod negotiate;
pub mod parser;
pub mod progress;
pub mod receive;
pub mod recovery;
pub mod send;
pub mod state;

// Re-export commonly used types
pub use error::{Result, ZmodemError};
pub use file::ZmodemFileInfo;
pub use frame::{FrameEncoding, FrameType, ZmodemFrame};
pub use init::ZmodemInit;
pub use negotiate::{CrcType, EscapeMode, NegotiatedParams};
pub use parser::{CrcType as ParserCrcType, FrameParser};
pub use progress::{BatchStats, ConsoleProgress, FileProgress, NoOpProgress, TransferProgress};
pub use receive::{ReceiveStats, ReceivedFile, ReceiverConfig, ZmodemReceiver};
pub use recovery::{RecoveryManager, ResumeInfo, TransferDirection, TransferState};
pub use send::{SenderConfig, TransferStats, ZmodemSender};
pub use state::{ZmodemState, ZmodemStateMachine};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_protocol_flow() {
        // Create a ZRINIT frame
        let frame = ZmodemFrame::with_defaults(FrameType::ZRINIT, FrameEncoding::Bin32);

        // Serialize it
        let serialized = frame.serialize();
        assert!(!serialized.is_empty());

        // Verify we can calculate CRCs
        let test_data = b"Test data";
        let crc16_value = crc16::calculate(test_data);
        let crc32_value = crc32::calculate(test_data);

        assert_ne!(crc16_value, 0);
        assert_ne!(crc32_value, 0);

        // Verify encoding/decoding
        let encoded = escape::encode(test_data);
        let decoded = escape::decode(&encoded).unwrap();
        assert_eq!(decoded, test_data);
    }

    #[test]
    fn test_frame_type_coverage() {
        // Ensure all frame types are accessible
        let types = [
            FrameType::ZRQINIT,
            FrameType::ZRINIT,
            FrameType::ZSINIT,
            FrameType::ZACK,
            FrameType::ZFILE,
            FrameType::ZSKIP,
            FrameType::ZNAK,
            FrameType::ZABORT,
            FrameType::ZFIN,
            FrameType::ZRPOS,
            FrameType::ZDATA,
            FrameType::ZEOF,
            FrameType::ZFERR,
            FrameType::ZCRC,
            FrameType::ZCHALLENGE,
            FrameType::ZCOMPL,
            FrameType::ZCAN,
            FrameType::ZFREECNT,
            FrameType::ZCOMMAND,
            FrameType::ZSTDERR,
        ];

        for frame_type in types {
            let byte = frame_type.to_u8();
            let parsed = FrameType::from_u8(byte).unwrap();
            assert_eq!(parsed, frame_type);
        }
    }

    #[test]
    fn test_encoding_types() {
        let encodings = [
            FrameEncoding::Hex,
            FrameEncoding::Bin16,
            FrameEncoding::Bin32,
        ];

        for encoding in encodings {
            let byte = encoding.to_u8();
            let parsed = FrameEncoding::from_u8(byte).unwrap();
            assert_eq!(parsed, encoding);
        }
    }

    #[test]
    fn test_error_types() {
        // Verify error types are accessible and work
        let err = ZmodemError::InvalidFrameType(99);
        assert!(err.to_string().contains("99"));

        let err = ZmodemError::Timeout;
        assert!(err.to_string().contains("Timeout"));

        let err = ZmodemError::CrcMismatch {
            expected: 0x1234,
            actual: 0x5678,
        };
        assert!(err.to_string().contains("1234"));
        assert!(err.to_string().contains("5678"));
    }
}
