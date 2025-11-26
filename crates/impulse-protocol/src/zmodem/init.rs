//! Session initialization for Zmodem protocol.
//!
//! This module handles the ZRQINIT (request init) and ZRINIT (receiver init)
//! frames used to negotiate session parameters.

use super::error::Result;
use super::frame::{FrameEncoding, FrameType, ZmodemFrame};

/// ZRINIT capability flags (receiver capabilities).
///
/// These flags are sent by the receiver in the ZRINIT frame to indicate
/// what features it supports.
/// Receiver can send and receive full duplex
pub const CANFDX: u8 = 0x01;

/// Receiver can receive overlapped I/O (streaming)
pub const CANOVIO: u8 = 0x02;

/// Receiver can send a break signal
pub const CANBRK: u8 = 0x04;

/// Receiver can decrypt data
pub const CANCRY: u8 = 0x08;

/// Receiver can decompress with LZW
pub const CANLZW: u8 = 0x10;

/// Receiver can use 32-bit CRC (FC)
pub const CANFC32: u8 = 0x20;

/// Receiver wants control characters escaped
pub const ESCCTL: u8 = 0x40;

/// Receiver wants 8th bit escaped
pub const ESC8: u8 = 0x80;

/// Zmodem session initialization parameters.
///
/// Represents the capabilities and preferences of a Zmodem receiver.
///
/// # Examples
///
/// ```
/// use impulse_protocol::zmodem::init::ZmodemInit;
///
/// let init = ZmodemInit {
///     escape_ctrl: true,
///     escape_8bit: false,
///     use_crc32: true,
///     buffer_size: 1024,
/// };
///
/// let frame = init.to_zrinit();
/// assert_eq!(frame.frame_type, impulse_protocol::zmodem::FrameType::ZRINIT);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZmodemInit {
    /// Escape control characters
    pub escape_ctrl: bool,

    /// Escape 8th bit (high-bit characters)
    pub escape_8bit: bool,

    /// Use 32-bit CRC instead of 16-bit
    pub use_crc32: bool,

    /// Receive buffer size in bytes
    pub buffer_size: u16,
}

impl ZmodemInit {
    /// Create a new initialization with default settings.
    ///
    /// Defaults:
    /// - escape_ctrl: false (no control character escaping)
    /// - escape_8bit: false (no 8th bit escaping)
    /// - use_crc32: true (prefer 32-bit CRC)
    /// - buffer_size: 8192 bytes
    pub fn new() -> Self {
        Self {
            escape_ctrl: false,
            escape_8bit: false,
            use_crc32: true,
            buffer_size: 8192,
        }
    }

    /// Create initialization with conservative settings.
    ///
    /// Conservative settings:
    /// - escape_ctrl: true (escape all control characters)
    /// - escape_8bit: true (escape high-bit characters)
    /// - use_crc32: true (32-bit CRC)
    /// - buffer_size: 1024 bytes
    ///
    /// Use this for unreliable or 7-bit connections.
    pub fn conservative() -> Self {
        Self {
            escape_ctrl: true,
            escape_8bit: true,
            use_crc32: true,
            buffer_size: 1024,
        }
    }

    /// Create initialization with aggressive settings for performance.
    ///
    /// Aggressive settings:
    /// - escape_ctrl: false (no escaping)
    /// - escape_8bit: false (no escaping)
    /// - use_crc32: true (32-bit CRC for reliability)
    /// - buffer_size: 16384 bytes (16 KB)
    ///
    /// Use this for reliable 8-bit connections.
    pub fn aggressive() -> Self {
        Self {
            escape_ctrl: false,
            escape_8bit: false,
            use_crc32: true,
            buffer_size: 16384,
        }
    }

    /// Create a ZRQINIT frame (request receiver initialization).
    ///
    /// Sent by the sender to request the receiver's capabilities.
    /// Uses hex encoding for maximum compatibility.
    ///
    /// # Returns
    ///
    /// ZRQINIT frame ready to be serialized and sent
    pub fn create_zrqinit() -> ZmodemFrame {
        ZmodemFrame::with_defaults(FrameType::ZRQINIT, FrameEncoding::Hex)
    }

    /// Convert initialization parameters to a ZRINIT frame.
    ///
    /// Creates a ZRINIT frame with flags set according to the initialization
    /// parameters. Uses hex encoding for maximum compatibility during init.
    ///
    /// # Returns
    ///
    /// ZRINIT frame ready to be serialized and sent
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_protocol::zmodem::init::ZmodemInit;
    ///
    /// let init = ZmodemInit::new();
    /// let frame = init.to_zrinit();
    /// let serialized = frame.serialize();
    /// ```
    pub fn to_zrinit(&self) -> ZmodemFrame {
        let mut flags = [0u8; 4];

        // ZF0: Capability flags
        let mut zf0 = 0u8;
        zf0 |= CANFDX; // We always support full duplex
        zf0 |= CANOVIO; // We always support overlapped I/O

        if self.use_crc32 {
            zf0 |= CANFC32;
        }
        if self.escape_ctrl {
            zf0 |= ESCCTL;
        }
        if self.escape_8bit {
            zf0 |= ESC8;
        }

        flags[0] = zf0;

        // ZF1, ZF2: Buffer size (little-endian)
        let buffer_bytes = self.buffer_size.to_le_bytes();
        flags[1] = buffer_bytes[0];
        flags[2] = buffer_bytes[1];

        // ZF3: Reserved (0)
        flags[3] = 0;

        ZmodemFrame::new(FrameType::ZRINIT, FrameEncoding::Hex, flags, None)
    }

    /// Parse a ZRINIT frame to extract initialization parameters.
    ///
    /// # Arguments
    ///
    /// * `frame` - ZRINIT frame to parse
    ///
    /// # Returns
    ///
    /// Initialization parameters extracted from the frame
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_protocol::zmodem::init::ZmodemInit;
    ///
    /// let init = ZmodemInit::new();
    /// let frame = init.to_zrinit();
    /// let parsed = ZmodemInit::from_zrinit(&frame).unwrap();
    /// assert_eq!(parsed, init);
    /// ```
    pub fn from_zrinit(frame: &ZmodemFrame) -> Result<Self> {
        let zf0 = frame.flags[0];
        let zf1 = frame.flags[1];
        let zf2 = frame.flags[2];

        let escape_ctrl = (zf0 & ESCCTL) != 0;
        let escape_8bit = (zf0 & ESC8) != 0;
        let use_crc32 = (zf0 & CANFC32) != 0;

        let buffer_size = u16::from_le_bytes([zf1, zf2]);

        Ok(Self {
            escape_ctrl,
            escape_8bit,
            use_crc32,
            buffer_size,
        })
    }

    /// Check if receiver supports full duplex operation.
    pub fn can_full_duplex(&self) -> bool {
        true // We always support full duplex
    }

    /// Check if receiver supports overlapped I/O.
    pub fn can_overlap_io(&self) -> bool {
        true // We always support overlapped I/O
    }
}

impl Default for ZmodemInit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_new() {
        let init = ZmodemInit::new();
        assert!(!init.escape_ctrl);
        assert!(!init.escape_8bit);
        assert!(init.use_crc32);
        assert_eq!(init.buffer_size, 8192);
    }

    #[test]
    fn test_init_conservative() {
        let init = ZmodemInit::conservative();
        assert!(init.escape_ctrl);
        assert!(init.escape_8bit);
        assert!(init.use_crc32);
        assert_eq!(init.buffer_size, 1024);
    }

    #[test]
    fn test_init_aggressive() {
        let init = ZmodemInit::aggressive();
        assert!(!init.escape_ctrl);
        assert!(!init.escape_8bit);
        assert!(init.use_crc32);
        assert_eq!(init.buffer_size, 16384);
    }

    #[test]
    fn test_create_zrqinit() {
        let frame = ZmodemInit::create_zrqinit();
        assert_eq!(frame.frame_type, FrameType::ZRQINIT);
        assert_eq!(frame.encoding, FrameEncoding::Hex);
    }

    #[test]
    fn test_to_zrinit() {
        let init = ZmodemInit::new();
        let frame = init.to_zrinit();

        assert_eq!(frame.frame_type, FrameType::ZRINIT);
        assert_eq!(frame.encoding, FrameEncoding::Hex);

        // Check flags
        let zf0 = frame.flags[0];
        assert_eq!(zf0 & CANFDX, CANFDX);
        assert_eq!(zf0 & CANOVIO, CANOVIO);
        assert_eq!(zf0 & CANFC32, CANFC32);
        assert_eq!(zf0 & ESCCTL, 0);
        assert_eq!(zf0 & ESC8, 0);
    }

    #[test]
    fn test_to_zrinit_with_escape() {
        let init = ZmodemInit {
            escape_ctrl: true,
            escape_8bit: true,
            use_crc32: false,
            buffer_size: 2048,
        };

        let frame = init.to_zrinit();
        let zf0 = frame.flags[0];

        assert_eq!(zf0 & ESCCTL, ESCCTL);
        assert_eq!(zf0 & ESC8, ESC8);
        assert_eq!(zf0 & CANFC32, 0);
    }

    #[test]
    fn test_from_zrinit() {
        let init = ZmodemInit {
            escape_ctrl: true,
            escape_8bit: false,
            use_crc32: true,
            buffer_size: 4096,
        };

        let frame = init.to_zrinit();
        let parsed = ZmodemInit::from_zrinit(&frame).unwrap();

        assert_eq!(parsed, init);
    }

    #[test]
    fn test_roundtrip() {
        let original = ZmodemInit::new();
        let frame = original.to_zrinit();
        let parsed = ZmodemInit::from_zrinit(&frame).unwrap();

        assert_eq!(parsed, original);
    }

    #[test]
    fn test_buffer_size_encoding() {
        let init = ZmodemInit {
            escape_ctrl: false,
            escape_8bit: false,
            use_crc32: true,
            buffer_size: 0x1234,
        };

        let frame = init.to_zrinit();
        let parsed = ZmodemInit::from_zrinit(&frame).unwrap();

        assert_eq!(parsed.buffer_size, 0x1234);
    }

    #[test]
    fn test_can_full_duplex() {
        let init = ZmodemInit::new();
        assert!(init.can_full_duplex());
    }

    #[test]
    fn test_can_overlap_io() {
        let init = ZmodemInit::new();
        assert!(init.can_overlap_io());
    }

    #[test]
    fn test_capability_flags() {
        // Test all capability flag constants
        assert_eq!(CANFDX, 0x01);
        assert_eq!(CANOVIO, 0x02);
        assert_eq!(CANBRK, 0x04);
        assert_eq!(CANCRY, 0x08);
        assert_eq!(CANLZW, 0x10);
        assert_eq!(CANFC32, 0x20);
        assert_eq!(ESCCTL, 0x40);
        assert_eq!(ESC8, 0x80);
    }

    #[test]
    fn test_multiple_flags() {
        let init = ZmodemInit {
            escape_ctrl: true,
            escape_8bit: true,
            use_crc32: true,
            buffer_size: 1024,
        };

        let frame = init.to_zrinit();
        let zf0 = frame.flags[0];

        // All flags should be set
        assert_ne!(zf0 & CANFDX, 0);
        assert_ne!(zf0 & CANOVIO, 0);
        assert_ne!(zf0 & CANFC32, 0);
        assert_ne!(zf0 & ESCCTL, 0);
        assert_ne!(zf0 & ESC8, 0);
    }

    #[test]
    fn test_no_flags() {
        let init = ZmodemInit {
            escape_ctrl: false,
            escape_8bit: false,
            use_crc32: false,
            buffer_size: 0,
        };

        let frame = init.to_zrinit();
        let zf0 = frame.flags[0];

        // Only mandatory flags should be set
        assert_ne!(zf0 & CANFDX, 0);
        assert_ne!(zf0 & CANOVIO, 0);
        assert_eq!(zf0 & CANFC32, 0);
        assert_eq!(zf0 & ESCCTL, 0);
        assert_eq!(zf0 & ESC8, 0);
    }

    #[test]
    fn test_zrinit_serialization() {
        let init = ZmodemInit::new();
        let frame = init.to_zrinit();
        let serialized = frame.serialize();

        // Should be valid serialized frame
        assert!(!serialized.is_empty());
        assert_eq!(serialized[0], super::super::escape::ZPAD);
    }
}
