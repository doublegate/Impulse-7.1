//! Zmodem frame types and structures.
//!
//! This module defines the frame format used in Zmodem protocol communication.

use super::crc16;
use super::crc32;
use super::error::{Result, ZmodemError};
use super::escape::{self, ZDLE};

/// Zmodem frame type identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FrameType {
    /// Request receiver init (sent by sender)
    ZRQINIT = 0,
    /// Receiver init (sent by receiver)
    ZRINIT = 1,
    /// Sender init (sent by sender)
    ZSINIT = 2,
    /// Acknowledgment
    ZACK = 3,
    /// File header information
    ZFILE = 4,
    /// Skip this file
    ZSKIP = 5,
    /// Negative acknowledgment (last packet was garbled)
    ZNAK = 6,
    /// Abort batch transfer
    ZABORT = 7,
    /// Finish session
    ZFIN = 8,
    /// Resume file transfer at this position
    ZRPOS = 9,
    /// Data packet(s) follow
    ZDATA = 10,
    /// End of file
    ZEOF = 11,
    /// Fatal read or write error detected
    ZFERR = 12,
    /// Request for CRC of file
    ZCRC = 13,
    /// Challenge - security check
    ZCHALLENGE = 14,
    /// Command complete
    ZCOMPL = 15,
    /// Cancel - other end has cancelled
    ZCAN = 16,
    /// Request free space on filesystem
    ZFREECNT = 17,
    /// Command follows
    ZCOMMAND = 18,
    /// Output to stderr
    ZSTDERR = 19,
}

impl FrameType {
    /// Convert a byte to a frame type.
    ///
    /// # Errors
    ///
    /// Returns `ZmodemError::InvalidFrameType` if the byte doesn't correspond to a valid frame type.
    pub fn from_u8(byte: u8) -> Result<Self> {
        match byte {
            0 => Ok(FrameType::ZRQINIT),
            1 => Ok(FrameType::ZRINIT),
            2 => Ok(FrameType::ZSINIT),
            3 => Ok(FrameType::ZACK),
            4 => Ok(FrameType::ZFILE),
            5 => Ok(FrameType::ZSKIP),
            6 => Ok(FrameType::ZNAK),
            7 => Ok(FrameType::ZABORT),
            8 => Ok(FrameType::ZFIN),
            9 => Ok(FrameType::ZRPOS),
            10 => Ok(FrameType::ZDATA),
            11 => Ok(FrameType::ZEOF),
            12 => Ok(FrameType::ZFERR),
            13 => Ok(FrameType::ZCRC),
            14 => Ok(FrameType::ZCHALLENGE),
            15 => Ok(FrameType::ZCOMPL),
            16 => Ok(FrameType::ZCAN),
            17 => Ok(FrameType::ZFREECNT),
            18 => Ok(FrameType::ZCOMMAND),
            19 => Ok(FrameType::ZSTDERR),
            _ => Err(ZmodemError::InvalidFrameType(byte)),
        }
    }

    /// Convert frame type to byte.
    pub fn to_u8(self) -> u8 {
        self as u8
    }
}

/// Frame encoding type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameEncoding {
    /// Hex encoding (human readable, used for initialization)
    Hex,
    /// Binary encoding with CRC-16
    Bin16,
    /// Binary encoding with CRC-32
    Bin32,
}

impl FrameEncoding {
    /// Get the encoding type byte.
    pub fn to_u8(self) -> u8 {
        match self {
            FrameEncoding::Hex => b'B',   // Hex frame follows
            FrameEncoding::Bin16 => b'A', // Binary frame with CRC-16
            FrameEncoding::Bin32 => b'C', // Binary frame with CRC-32
        }
    }

    /// Parse encoding type from byte.
    pub fn from_u8(byte: u8) -> Result<Self> {
        match byte {
            b'B' => Ok(FrameEncoding::Hex),
            b'A' => Ok(FrameEncoding::Bin16),
            b'C' => Ok(FrameEncoding::Bin32),
            _ => Err(ZmodemError::InvalidFrameEncoding(byte)),
        }
    }
}

/// Zmodem frame structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZmodemFrame {
    /// Frame type identifier
    pub frame_type: FrameType,
    /// Encoding type
    pub encoding: FrameEncoding,
    /// Flag bytes (ZF0-ZF3)
    pub flags: [u8; 4],
    /// Optional data payload
    pub data: Option<Vec<u8>>,
}

impl ZmodemFrame {
    /// Create a new Zmodem frame.
    ///
    /// # Arguments
    ///
    /// * `frame_type` - The type of frame
    /// * `encoding` - The encoding to use
    /// * `flags` - The flag bytes
    /// * `data` - Optional data payload
    pub fn new(
        frame_type: FrameType,
        encoding: FrameEncoding,
        flags: [u8; 4],
        data: Option<Vec<u8>>,
    ) -> Self {
        Self {
            frame_type,
            encoding,
            flags,
            data,
        }
    }

    /// Create a new frame with default flags.
    pub fn with_defaults(frame_type: FrameType, encoding: FrameEncoding) -> Self {
        Self::new(frame_type, encoding, [0; 4], None)
    }

    /// Serialize the frame to bytes.
    ///
    /// # Returns
    ///
    /// Vector containing the serialized frame
    pub fn serialize(&self) -> Vec<u8> {
        match self.encoding {
            FrameEncoding::Hex => self.serialize_hex(),
            FrameEncoding::Bin16 => self.serialize_bin16(),
            FrameEncoding::Bin32 => self.serialize_bin32(),
        }
    }

    /// Serialize frame in hex encoding.
    fn serialize_hex(&self) -> Vec<u8> {
        // ZPAD ZPAD ZDLE encoding_type
        let mut result = vec![escape::ZPAD, escape::ZPAD, ZDLE, self.encoding.to_u8()];

        // Frame type + flags in hex
        let header = [
            self.frame_type.to_u8(),
            self.flags[0],
            self.flags[1],
            self.flags[2],
            self.flags[3],
        ];

        for byte in header {
            result.push(hex_digit(byte >> 4));
            result.push(hex_digit(byte & 0x0F));
        }

        // CRC-16 in hex
        let crc = crc16::calculate(&header);
        let crc_bytes = crc.to_be_bytes();
        for byte in crc_bytes {
            result.push(hex_digit(byte >> 4));
            result.push(hex_digit(byte & 0x0F));
        }

        // Line terminator
        result.push(0x0D); // CR
        result.push(0x8A); // LF | 0x80

        result
    }

    /// Serialize frame in binary CRC-16 encoding.
    fn serialize_bin16(&self) -> Vec<u8> {
        // ZPAD ZDLE encoding_type
        let mut result = vec![escape::ZPAD, ZDLE, self.encoding.to_u8()];

        // Build header
        let header = [
            self.frame_type.to_u8(),
            self.flags[0],
            self.flags[1],
            self.flags[2],
            self.flags[3],
        ];

        // Encode header
        result.extend_from_slice(&escape::encode(&header));

        // Calculate and encode CRC-16
        let crc = crc16::calculate(&header);
        let crc_bytes = crc.to_be_bytes();
        result.extend_from_slice(&escape::encode(&crc_bytes));

        result
    }

    /// Serialize frame in binary CRC-32 encoding.
    fn serialize_bin32(&self) -> Vec<u8> {
        // ZPAD ZDLE encoding_type
        let mut result = vec![escape::ZPAD, ZDLE, self.encoding.to_u8()];

        // Build header
        let header = [
            self.frame_type.to_u8(),
            self.flags[0],
            self.flags[1],
            self.flags[2],
            self.flags[3],
        ];

        // Encode header
        result.extend_from_slice(&escape::encode(&header));

        // Calculate and encode CRC-32
        let crc = crc32::calculate(&header);
        let crc_bytes = crc.to_le_bytes();
        result.extend_from_slice(&escape::encode(&crc_bytes));

        result
    }

    /// Get the flags as a 32-bit value.
    pub fn flags_as_u32(&self) -> u32 {
        u32::from_le_bytes(self.flags)
    }

    /// Set flags from a 32-bit value.
    pub fn set_flags_from_u32(&mut self, value: u32) {
        self.flags = value.to_le_bytes();
    }
}

/// Convert a nibble (0-15) to a hex digit character.
fn hex_digit(nibble: u8) -> u8 {
    match nibble & 0x0F {
        n @ 0..=9 => b'0' + n,
        n @ 10..=15 => b'a' + (n - 10),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_type_conversions() {
        assert_eq!(FrameType::ZRQINIT.to_u8(), 0);
        assert_eq!(FrameType::ZRINIT.to_u8(), 1);
        assert_eq!(FrameType::ZFIN.to_u8(), 8);
        assert_eq!(FrameType::ZDATA.to_u8(), 10);

        assert_eq!(FrameType::from_u8(0).unwrap(), FrameType::ZRQINIT);
        assert_eq!(FrameType::from_u8(1).unwrap(), FrameType::ZRINIT);
        assert_eq!(FrameType::from_u8(19).unwrap(), FrameType::ZSTDERR);

        assert!(FrameType::from_u8(255).is_err());
    }

    #[test]
    fn test_frame_encoding_conversions() {
        assert_eq!(FrameEncoding::Hex.to_u8(), b'B');
        assert_eq!(FrameEncoding::Bin16.to_u8(), b'A');
        assert_eq!(FrameEncoding::Bin32.to_u8(), b'C');

        assert_eq!(FrameEncoding::from_u8(b'B').unwrap(), FrameEncoding::Hex);
        assert_eq!(FrameEncoding::from_u8(b'A').unwrap(), FrameEncoding::Bin16);
        assert_eq!(FrameEncoding::from_u8(b'C').unwrap(), FrameEncoding::Bin32);

        assert!(FrameEncoding::from_u8(b'X').is_err());
    }

    #[test]
    fn test_frame_creation() {
        let frame = ZmodemFrame::new(FrameType::ZRINIT, FrameEncoding::Hex, [1, 2, 3, 4], None);
        assert_eq!(frame.frame_type, FrameType::ZRINIT);
        assert_eq!(frame.encoding, FrameEncoding::Hex);
        assert_eq!(frame.flags, [1, 2, 3, 4]);
        assert!(frame.data.is_none());
    }

    #[test]
    fn test_frame_with_defaults() {
        let frame = ZmodemFrame::with_defaults(FrameType::ZACK, FrameEncoding::Bin16);
        assert_eq!(frame.frame_type, FrameType::ZACK);
        assert_eq!(frame.encoding, FrameEncoding::Bin16);
        assert_eq!(frame.flags, [0, 0, 0, 0]);
        assert!(frame.data.is_none());
    }

    #[test]
    fn test_flags_as_u32() {
        let frame = ZmodemFrame::new(
            FrameType::ZRINIT,
            FrameEncoding::Hex,
            [0x12, 0x34, 0x56, 0x78],
            None,
        );
        assert_eq!(frame.flags_as_u32(), 0x78563412); // Little-endian
    }

    #[test]
    fn test_set_flags_from_u32() {
        let mut frame = ZmodemFrame::with_defaults(FrameType::ZRINIT, FrameEncoding::Hex);
        frame.set_flags_from_u32(0x12345678);
        assert_eq!(frame.flags, [0x78, 0x56, 0x34, 0x12]); // Little-endian
    }

    #[test]
    fn test_hex_digit() {
        assert_eq!(hex_digit(0), b'0');
        assert_eq!(hex_digit(9), b'9');
        assert_eq!(hex_digit(10), b'a');
        assert_eq!(hex_digit(15), b'f');
    }

    #[test]
    fn test_serialize_hex() {
        let frame = ZmodemFrame::new(FrameType::ZRINIT, FrameEncoding::Hex, [0, 0, 0, 0], None);
        let serialized = frame.serialize();

        // Check header: ZPAD ZPAD ZDLE 'B'
        assert_eq!(serialized[0], escape::ZPAD);
        assert_eq!(serialized[1], escape::ZPAD);
        assert_eq!(serialized[2], ZDLE);
        assert_eq!(serialized[3], b'B');

        // Check frame type (ZRINIT = 1) in hex
        assert_eq!(serialized[4], b'0');
        assert_eq!(serialized[5], b'1');

        // Should end with CR LF|0x80
        assert_eq!(serialized[serialized.len() - 2], 0x0D);
        assert_eq!(serialized[serialized.len() - 1], 0x8A);
    }

    #[test]
    fn test_serialize_bin16() {
        let frame = ZmodemFrame::new(FrameType::ZACK, FrameEncoding::Bin16, [0, 0, 0, 0], None);
        let serialized = frame.serialize();

        // Check header: ZPAD ZDLE 'A'
        assert_eq!(serialized[0], escape::ZPAD);
        assert_eq!(serialized[1], ZDLE);
        assert_eq!(serialized[2], b'A');

        // Should contain encoded frame type and CRC
        assert!(serialized.len() > 3);
    }

    #[test]
    fn test_serialize_bin32() {
        let frame = ZmodemFrame::new(FrameType::ZFILE, FrameEncoding::Bin32, [0, 0, 0, 0], None);
        let serialized = frame.serialize();

        // Check header: ZPAD ZDLE 'C'
        assert_eq!(serialized[0], escape::ZPAD);
        assert_eq!(serialized[1], ZDLE);
        assert_eq!(serialized[2], b'C');

        // Should contain encoded frame type and CRC
        assert!(serialized.len() > 3);
    }

    #[test]
    fn test_serialize_different_encodings() {
        let frame_hex = ZmodemFrame::new(FrameType::ZRINIT, FrameEncoding::Hex, [0; 4], None);
        let frame_bin16 = ZmodemFrame::new(FrameType::ZRINIT, FrameEncoding::Bin16, [0; 4], None);
        let frame_bin32 = ZmodemFrame::new(FrameType::ZRINIT, FrameEncoding::Bin32, [0; 4], None);

        let hex_data = frame_hex.serialize();
        let bin16_data = frame_bin16.serialize();
        let bin32_data = frame_bin32.serialize();

        // Different encodings should produce different output
        assert_ne!(hex_data, bin16_data);
        assert_ne!(hex_data, bin32_data);
        assert_ne!(bin16_data, bin32_data);
    }

    #[test]
    fn test_all_frame_types() {
        let frame_types = [
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

        // Verify all frame types can be serialized
        for frame_type in frame_types {
            let frame = ZmodemFrame::with_defaults(frame_type, FrameEncoding::Hex);
            let serialized = frame.serialize();
            assert!(!serialized.is_empty());
        }
    }
}
