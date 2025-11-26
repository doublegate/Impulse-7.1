//! Frame parser for Zmodem protocol.
//!
//! This module provides stateful parsing of incoming Zmodem frames from
//! a byte stream, handling hex and binary frame formats.

use super::crc16;
use super::crc32;
use super::error::{Result, ZmodemError};
use super::escape::{self, ZDLE, ZPAD};
use super::frame::{FrameEncoding, FrameType, ZmodemFrame};

/// Parser state for frame detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParserState {
    /// Waiting for first ZPAD
    WaitingForZpad,
    /// Waiting for ZDLE after ZPAD
    WaitingForZdle,
    /// Reading hex-encoded frame
    ReadingHexFrame,
    /// Reading binary CRC-16 frame
    ReadingBinaryFrame16,
    /// Reading binary CRC-32 frame
    ReadingBinaryFrame32,
}

/// CRC type for binary frames.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrcType {
    /// CRC-16/XMODEM
    Crc16,
    /// CRC-32/ZMODEM
    Crc32,
}

/// Stateful parser for Zmodem frames.
///
/// Accumulates incoming data and extracts complete frames when available.
/// Handles both hex and binary encodings.
///
/// # Examples
///
/// ```
/// use impulse_protocol::zmodem::parser::FrameParser;
/// use impulse_protocol::zmodem::{FrameType, FrameEncoding, ZmodemFrame};
///
/// let mut parser = FrameParser::new();
///
/// // Create a test frame
/// let frame = ZmodemFrame::with_defaults(FrameType::ZRINIT, FrameEncoding::Hex);
/// let serialized = frame.serialize();
///
/// // Feed data to parser
/// let frames = parser.feed(&serialized);
/// assert_eq!(frames.len(), 1);
/// assert!(frames[0].is_ok());
/// ```
pub struct FrameParser {
    buffer: Vec<u8>,
    state: ParserState,
}

impl FrameParser {
    /// Create a new frame parser.
    pub fn new() -> Self {
        Self {
            buffer: Vec::with_capacity(1024),
            state: ParserState::WaitingForZpad,
        }
    }

    /// Feed incoming data to the parser.
    ///
    /// Returns a vector of parsed frames (successful or error).
    /// Multiple frames may be returned if the data contains several complete frames.
    ///
    /// # Arguments
    ///
    /// * `data` - Incoming byte data
    ///
    /// # Returns
    ///
    /// Vector of parsing results (one per complete frame found)
    pub fn feed(&mut self, data: &[u8]) -> Vec<Result<ZmodemFrame>> {
        let mut frames = Vec::new();

        for &byte in data {
            self.buffer.push(byte);
            let len = self.buffer.len();

            match self.state {
                ParserState::WaitingForZpad => {
                    // Look for ZPAD ZPAD ZDLE or ZPAD ZDLE pattern
                    if len >= 3 {
                        let last_three = &self.buffer[len - 3..];
                        if last_three[0] == ZPAD && last_three[1] == ZPAD && last_three[2] == ZDLE {
                            // Hex frame start: ZPAD ZPAD ZDLE
                            self.state = ParserState::WaitingForZdle;
                        }
                    }
                    // Check for binary frame (ZPAD ZDLE) - separate check, not else-if
                    if len >= 2 && self.state == ParserState::WaitingForZpad {
                        let last_two = &self.buffer[len - 2..];
                        if last_two[0] == ZPAD && last_two[1] == ZDLE {
                            // Binary frame start: ZPAD ZDLE
                            self.state = ParserState::WaitingForZdle;
                        }
                    }
                }

                ParserState::WaitingForZdle => {
                    // Next byte after ZDLE is encoding type
                    match byte {
                        b'B' => self.state = ParserState::ReadingHexFrame,
                        b'A' => self.state = ParserState::ReadingBinaryFrame16,
                        b'C' => self.state = ParserState::ReadingBinaryFrame32,
                        _ => {
                            frames.push(Err(ZmodemError::InvalidFrameEncoding(byte)));
                            self.reset();
                        }
                    }
                }

                ParserState::ReadingHexFrame => {
                    // Check for end of hex frame: CR LF|0x80
                    if len >= 2 {
                        let last_two = &self.buffer[len - 2..];
                        if last_two[0] == 0x0D && last_two[1] == 0x8A {
                            match self.parse_hex_frame(&self.buffer) {
                                Ok(frame) => frames.push(Ok(frame)),
                                Err(e) => frames.push(Err(e)),
                            }
                            self.reset();
                        }
                    }
                }

                ParserState::ReadingBinaryFrame16 => {
                    // Try to parse when we likely have enough data
                    // With worst-case ZDLE encoding: 3 header + 5*2 (header) + 2*2 (CRC) = 17 bytes
                    // Use 20 to be safe, or try parsing with error recovery
                    match self.parse_binary_frame(&self.buffer, CrcType::Crc16) {
                        Ok(frame) => {
                            frames.push(Ok(frame));
                            self.reset();
                        }
                        Err(ZmodemError::InvalidEscape) | Err(ZmodemError::UnexpectedEof) => {
                            // Need more data, keep accumulating
                        }
                        Err(ZmodemError::InvalidFrame(msg))
                            if msg.contains("Insufficient") || msg.contains("too short") =>
                        {
                            // Need more data, keep accumulating
                        }
                        Err(e) => {
                            frames.push(Err(e));
                            self.reset();
                        }
                    }
                }

                ParserState::ReadingBinaryFrame32 => {
                    // Try to parse when we likely have enough data
                    // With worst-case ZDLE encoding: 3 header + 5*2 (header) + 4*2 (CRC) = 21 bytes
                    match self.parse_binary_frame(&self.buffer, CrcType::Crc32) {
                        Ok(frame) => {
                            frames.push(Ok(frame));
                            self.reset();
                        }
                        Err(ZmodemError::InvalidEscape) | Err(ZmodemError::UnexpectedEof) => {
                            // Need more data, keep accumulating
                        }
                        Err(ZmodemError::InvalidFrame(msg))
                            if msg.contains("Insufficient") || msg.contains("too short") =>
                        {
                            // Need more data, keep accumulating
                        }
                        Err(e) => {
                            frames.push(Err(e));
                            self.reset();
                        }
                    }
                }
            }
        }

        frames
    }

    /// Parse a hex-encoded frame.
    ///
    /// # Arguments
    ///
    /// * `data` - Complete hex frame data (including ZPAD header and terminator)
    ///
    /// # Returns
    ///
    /// Parsed frame or error
    ///
    /// # Errors
    ///
    /// Returns error if frame format is invalid or CRC doesn't match
    pub fn parse_hex_frame(&self, data: &[u8]) -> Result<ZmodemFrame> {
        // Hex frame format: ZPAD ZPAD ZDLE 'B' <hex-header> <hex-crc16> CR LF|0x80
        // Header: frame_type(2) + flags(8) = 10 hex digits
        // CRC: 4 hex digits

        if data.len() < 18 {
            // Minimum: 4 (header) + 10 (header hex) + 4 (crc hex) + 2 (terminator) = 20
            // But we need at least ZPAD ZPAD ZDLE 'B' (4) + 10 + 4 = 18
            return Err(ZmodemError::InvalidFrame("Hex frame too short".to_string()));
        }

        // Skip ZPAD ZPAD ZDLE 'B' header (4 bytes)
        let hex_start = 4;
        let hex_end = data.len() - 2; // Skip CR LF|0x80

        let hex_data = &data[hex_start..hex_end];

        // Parse hex digits
        if hex_data.len() < 14 {
            // 10 header + 4 crc
            return Err(ZmodemError::InvalidFrame(
                "Insufficient hex data".to_string(),
            ));
        }

        // Parse header (5 bytes = 10 hex digits)
        let mut header = [0u8; 5];
        for i in 0..5 {
            let high = parse_hex_digit(hex_data[i * 2])?;
            let low = parse_hex_digit(hex_data[i * 2 + 1])?;
            header[i] = (high << 4) | low;
        }

        // Parse CRC (2 bytes = 4 hex digits)
        let high_byte_high = parse_hex_digit(hex_data[10])?;
        let high_byte_low = parse_hex_digit(hex_data[11])?;
        let low_byte_high = parse_hex_digit(hex_data[12])?;
        let low_byte_low = parse_hex_digit(hex_data[13])?;

        let expected_crc = u16::from_be_bytes([
            (high_byte_high << 4) | high_byte_low,
            (low_byte_high << 4) | low_byte_low,
        ]);

        // Verify CRC
        let actual_crc = crc16::calculate(&header);
        if actual_crc != expected_crc {
            return Err(ZmodemError::CrcMismatch {
                expected: u32::from(expected_crc),
                actual: u32::from(actual_crc),
            });
        }

        // Extract frame type and flags
        let frame_type = FrameType::from_u8(header[0])?;
        let flags = [header[1], header[2], header[3], header[4]];

        Ok(ZmodemFrame::new(
            frame_type,
            FrameEncoding::Hex,
            flags,
            None,
        ))
    }

    /// Parse a binary-encoded frame with specified CRC type.
    ///
    /// # Arguments
    ///
    /// * `data` - Complete binary frame data
    /// * `crc_type` - Type of CRC to use for validation
    ///
    /// # Returns
    ///
    /// Parsed frame or error
    ///
    /// # Errors
    ///
    /// Returns error if frame format is invalid or CRC doesn't match
    pub fn parse_binary_frame(&self, data: &[u8], crc_type: CrcType) -> Result<ZmodemFrame> {
        // Binary frame format: ZPAD ZDLE encoding <ZDLE-encoded-header> <ZDLE-encoded-crc>
        // Header: 5 bytes (frame_type + flags)
        // CRC: 2 bytes (CRC-16) or 4 bytes (CRC-32)

        if data.len() < 8 {
            return Err(ZmodemError::InvalidFrame(
                "Binary frame too short".to_string(),
            ));
        }

        // Skip ZPAD ZDLE encoding (3 bytes)
        let encoded_data = &data[3..];

        // Decode the ZDLE-encoded data
        let decoded = escape::decode(encoded_data)?;

        let (header_len, crc_len) = match crc_type {
            CrcType::Crc16 => (5, 2),
            CrcType::Crc32 => (5, 4),
        };

        if decoded.len() < header_len + crc_len {
            return Err(ZmodemError::InvalidFrame(
                "Insufficient decoded data".to_string(),
            ));
        }

        // Split header and CRC
        let header = &decoded[0..header_len];
        let crc_bytes = &decoded[header_len..header_len + crc_len];

        // Verify CRC
        match crc_type {
            CrcType::Crc16 => {
                let expected_crc = u16::from_be_bytes([crc_bytes[0], crc_bytes[1]]);
                let actual_crc = crc16::calculate(header);
                if actual_crc != expected_crc {
                    return Err(ZmodemError::CrcMismatch {
                        expected: u32::from(expected_crc),
                        actual: u32::from(actual_crc),
                    });
                }
            }
            CrcType::Crc32 => {
                let expected_crc =
                    u32::from_le_bytes([crc_bytes[0], crc_bytes[1], crc_bytes[2], crc_bytes[3]]);
                let actual_crc = crc32::calculate(header);
                if actual_crc != expected_crc {
                    return Err(ZmodemError::CrcMismatch {
                        expected: expected_crc,
                        actual: actual_crc,
                    });
                }
            }
        }

        // Extract frame type and flags
        let frame_type = FrameType::from_u8(header[0])?;
        let flags = [header[1], header[2], header[3], header[4]];

        let encoding = match crc_type {
            CrcType::Crc16 => FrameEncoding::Bin16,
            CrcType::Crc32 => FrameEncoding::Bin32,
        };

        Ok(ZmodemFrame::new(frame_type, encoding, flags, None))
    }

    /// Reset parser state and buffer.
    fn reset(&mut self) {
        self.buffer.clear();
        self.state = ParserState::WaitingForZpad;
    }
}

impl Default for FrameParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse a hex digit character to its numeric value.
fn parse_hex_digit(ch: u8) -> Result<u8> {
    match ch {
        b'0'..=b'9' => Ok(ch - b'0'),
        b'a'..=b'f' => Ok(ch - b'a' + 10),
        b'A'..=b'F' => Ok(ch - b'A' + 10),
        _ => Err(ZmodemError::InvalidFrame(format!(
            "Invalid hex digit: {}",
            ch
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_creation() {
        let parser = FrameParser::new();
        assert_eq!(parser.state, ParserState::WaitingForZpad);
        assert!(parser.buffer.is_empty());
    }

    #[test]
    fn test_parse_hex_digit() {
        assert_eq!(parse_hex_digit(b'0').unwrap(), 0);
        assert_eq!(parse_hex_digit(b'9').unwrap(), 9);
        assert_eq!(parse_hex_digit(b'a').unwrap(), 10);
        assert_eq!(parse_hex_digit(b'f').unwrap(), 15);
        assert_eq!(parse_hex_digit(b'A').unwrap(), 10);
        assert_eq!(parse_hex_digit(b'F').unwrap(), 15);
        assert!(parse_hex_digit(b'g').is_err());
        assert!(parse_hex_digit(b'G').is_err());
    }

    #[test]
    fn test_parse_hex_frame_roundtrip() {
        let original = ZmodemFrame::new(FrameType::ZRINIT, FrameEncoding::Hex, [1, 2, 3, 4], None);
        let serialized = original.serialize();

        let parser = FrameParser::new();
        let parsed = parser.parse_hex_frame(&serialized).unwrap();

        assert_eq!(parsed.frame_type, original.frame_type);
        assert_eq!(parsed.flags, original.flags);
    }

    #[test]
    fn test_parse_binary16_frame_roundtrip() {
        let original = ZmodemFrame::new(FrameType::ZACK, FrameEncoding::Bin16, [5, 6, 7, 8], None);
        let serialized = original.serialize();

        let parser = FrameParser::new();
        let parsed = parser
            .parse_binary_frame(&serialized, CrcType::Crc16)
            .unwrap();

        assert_eq!(parsed.frame_type, original.frame_type);
        assert_eq!(parsed.flags, original.flags);
    }

    #[test]
    fn test_parse_binary32_frame_roundtrip() {
        let original = ZmodemFrame::new(
            FrameType::ZFILE,
            FrameEncoding::Bin32,
            [9, 10, 11, 12],
            None,
        );
        let serialized = original.serialize();

        let parser = FrameParser::new();
        let parsed = parser
            .parse_binary_frame(&serialized, CrcType::Crc32)
            .unwrap();

        assert_eq!(parsed.frame_type, original.frame_type);
        assert_eq!(parsed.flags, original.flags);
    }

    #[test]
    fn test_feed_hex_frame() {
        let mut parser = FrameParser::new();

        let frame = ZmodemFrame::with_defaults(FrameType::ZRINIT, FrameEncoding::Hex);
        let serialized = frame.serialize();

        let frames = parser.feed(&serialized);

        assert_eq!(frames.len(), 1);
        assert!(frames[0].is_ok());
        let parsed = frames[0].as_ref().unwrap();
        assert_eq!(parsed.frame_type, FrameType::ZRINIT);
    }

    #[test]
    fn test_feed_binary16_frame() {
        let mut parser = FrameParser::new();

        let frame = ZmodemFrame::with_defaults(FrameType::ZACK, FrameEncoding::Bin16);
        let serialized = frame.serialize();

        let frames = parser.feed(&serialized);

        assert_eq!(frames.len(), 1);
        if let Err(ref e) = frames[0] {
            panic!("Parser error: {}", e);
        }
        assert!(frames[0].is_ok());
        let parsed = frames[0].as_ref().unwrap();
        assert_eq!(parsed.frame_type, FrameType::ZACK);
    }

    #[test]
    fn test_feed_binary32_frame() {
        let mut parser = FrameParser::new();

        let frame = ZmodemFrame::with_defaults(FrameType::ZFILE, FrameEncoding::Bin32);
        let serialized = frame.serialize();

        let frames = parser.feed(&serialized);

        assert_eq!(frames.len(), 1);
        assert!(frames[0].is_ok());
        let parsed = frames[0].as_ref().unwrap();
        assert_eq!(parsed.frame_type, FrameType::ZFILE);
    }

    #[test]
    fn test_feed_multiple_frames() {
        let mut parser = FrameParser::new();

        let frame1 = ZmodemFrame::with_defaults(FrameType::ZRINIT, FrameEncoding::Hex);
        let frame2 = ZmodemFrame::with_defaults(FrameType::ZACK, FrameEncoding::Hex);

        let mut data = frame1.serialize();
        data.extend_from_slice(&frame2.serialize());

        let frames = parser.feed(&data);

        assert_eq!(frames.len(), 2);
        assert!(frames[0].is_ok());
        assert!(frames[1].is_ok());
    }

    #[test]
    fn test_feed_incomplete_frame() {
        let mut parser = FrameParser::new();

        let frame = ZmodemFrame::with_defaults(FrameType::ZRINIT, FrameEncoding::Hex);
        let serialized = frame.serialize();

        // Feed only half the data
        let half = serialized.len() / 2;
        let frames = parser.feed(&serialized[..half]);

        // No complete frames yet
        assert_eq!(frames.len(), 0);
    }

    #[test]
    fn test_parse_hex_frame_invalid_crc() {
        let mut data = vec![ZPAD, ZPAD, ZDLE, b'B'];

        // Add valid header
        data.extend_from_slice(b"0100000000");

        // Add invalid CRC
        data.extend_from_slice(b"0000");

        // Add terminator
        data.push(0x0D);
        data.push(0x8A);

        let parser = FrameParser::new();
        let result = parser.parse_hex_frame(&data);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ZmodemError::CrcMismatch { .. }
        ));
    }

    #[test]
    fn test_parse_hex_frame_too_short() {
        let data = vec![ZPAD, ZPAD, ZDLE, b'B'];

        let parser = FrameParser::new();
        let result = parser.parse_hex_frame(&data);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_binary_frame_too_short() {
        let data = vec![ZPAD, ZDLE, b'A', 0x01];

        let parser = FrameParser::new();
        let result = parser.parse_binary_frame(&data, CrcType::Crc16);

        assert!(result.is_err());
    }

    #[test]
    fn test_crc_type() {
        assert_eq!(CrcType::Crc16, CrcType::Crc16);
        assert_eq!(CrcType::Crc32, CrcType::Crc32);
        assert_ne!(CrcType::Crc16, CrcType::Crc32);
    }
}
