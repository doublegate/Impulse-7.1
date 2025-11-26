//! ZDLE (Zmodem Data Link Escape) encoding and decoding.
//!
//! This module handles the escaping of special characters in Zmodem protocol
//! to prevent interference with control characters and ensure data integrity.

use super::error::{Result, ZmodemError};

/// ZDLE: Zmodem Data Link Escape character
pub const ZDLE: u8 = 0x18;

/// Special control characters
pub const XON: u8 = 0x11;
pub const XOFF: u8 = 0x13;
pub const ZPAD: u8 = 0x2A; // '*' padding character
pub const ZCRCE: u8 = 0x68; // CRC next, frame ends, header packet follows
pub const ZCRCG: u8 = 0x69; // CRC next, frame continues nonstop
pub const ZCRCQ: u8 = 0x6A; // CRC next, frame continues, ZACK expected
pub const ZCRCW: u8 = 0x6B; // CRC next, ZACK expected, end of frame

/// Characters that must be escaped in ZDLE encoding.
const ESCAPE_CHARS: &[u8] = &[
    0x00, // NUL
    0x0D, // CR
    0x10, // DLE (Ctrl-P)
    0x11, // XON (Ctrl-Q)
    0x13, // XOFF (Ctrl-S)
    0x18, // ZDLE/CAN (Ctrl-X)
    0x7F, // DEL
    0x80, // High bit set variants
    0x8D, 0x90, 0x91, 0x93, 0xFF,
];

/// Encode data using ZDLE escaping.
///
/// Characters in the escape set are preceded by ZDLE and XORed with 0x40.
///
/// # Arguments
///
/// * `data` - The raw data to encode
///
/// # Returns
///
/// Vector containing the ZDLE-encoded data
///
/// # Examples
///
/// ```
/// use impulse_protocol::zmodem::escape::{encode, ZDLE};
///
/// let data = &[0x11, 0x42]; // XON + 'B'
/// let encoded = encode(data);
/// assert_eq!(encoded, vec![ZDLE, 0x11 ^ 0x40, 0x42]);
/// ```
pub fn encode(data: &[u8]) -> Vec<u8> {
    let mut encoded = Vec::with_capacity(data.len() * 2); // Worst case: all chars escaped

    for &byte in data {
        if should_escape(byte) {
            encoded.push(ZDLE);
            encoded.push(byte ^ 0x40);
        } else {
            encoded.push(byte);
        }
    }

    encoded
}

/// Decode ZDLE-encoded data.
///
/// Processes escape sequences and reconstructs the original data.
///
/// # Arguments
///
/// * `data` - The ZDLE-encoded data
///
/// # Returns
///
/// Result containing the decoded data or an error if invalid escape sequences found
///
/// # Errors
///
/// Returns `ZmodemError::InvalidEscape` if:
/// - ZDLE appears at the end without a following byte
/// - Invalid escape sequence is encountered
///
/// # Examples
///
/// ```
/// use impulse_protocol::zmodem::escape::{encode, decode, ZDLE};
///
/// let original = b"Hello\x11World";
/// let encoded = encode(original);
/// let decoded = decode(&encoded).unwrap();
/// assert_eq!(decoded, original);
/// ```
pub fn decode(data: &[u8]) -> Result<Vec<u8>> {
    let mut decoded = Vec::with_capacity(data.len());
    let mut i = 0;

    while i < data.len() {
        if data[i] == ZDLE {
            // ZDLE must be followed by another byte
            if i + 1 >= data.len() {
                return Err(ZmodemError::InvalidEscape);
            }

            let next = data[i + 1];

            // Handle special ZDLE sequences
            match next {
                // ZCRCE, ZCRCG, ZCRCQ, ZCRCW are frame endings - pass through
                ZCRCE | ZCRCG | ZCRCQ | ZCRCW => {
                    decoded.push(ZDLE);
                    decoded.push(next);
                }
                // ZDLE ZDLE -> single ZDLE
                ZDLE => {
                    decoded.push(ZDLE);
                }
                // Normal escape: XOR with 0x40
                _ => {
                    decoded.push(next ^ 0x40);
                }
            }

            i += 2;
        } else {
            decoded.push(data[i]);
            i += 1;
        }
    }

    Ok(decoded)
}

/// Check if a byte should be escaped.
///
/// # Arguments
///
/// * `byte` - The byte to check
///
/// # Returns
///
/// `true` if the byte should be escaped, `false` otherwise
fn should_escape(byte: u8) -> bool {
    ESCAPE_CHARS.contains(&byte)
}

/// Encode a 32-bit value in little-endian byte order with ZDLE escaping.
///
/// # Arguments
///
/// * `value` - The 32-bit value to encode
///
/// # Returns
///
/// Vector containing the ZDLE-encoded bytes
pub fn encode_u32(value: u32) -> Vec<u8> {
    let bytes = value.to_le_bytes();
    encode(&bytes)
}

/// Decode a ZDLE-encoded 32-bit value.
///
/// # Arguments
///
/// * `data` - The encoded data (must contain at least 4 decoded bytes)
///
/// # Returns
///
/// Result containing the decoded 32-bit value
///
/// # Errors
///
/// Returns error if decoding fails or insufficient data
pub fn decode_u32(data: &[u8]) -> Result<u32> {
    let decoded = decode(data)?;
    if decoded.len() < 4 {
        return Err(ZmodemError::UnexpectedEof);
    }
    let bytes: [u8; 4] = decoded[0..4]
        .try_into()
        .map_err(|_| ZmodemError::UnexpectedEof)?;
    Ok(u32::from_le_bytes(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_no_escapes() {
        let data = b"HelloWorld";
        let encoded = encode(data);
        assert_eq!(encoded, data.to_vec());
    }

    #[test]
    fn test_encode_xon() {
        let data = &[XON];
        let encoded = encode(data);
        assert_eq!(encoded, vec![ZDLE, XON ^ 0x40]);
    }

    #[test]
    fn test_encode_xoff() {
        let data = &[XOFF];
        let encoded = encode(data);
        assert_eq!(encoded, vec![ZDLE, XOFF ^ 0x40]);
    }

    #[test]
    fn test_encode_zdle() {
        let data = &[ZDLE];
        let encoded = encode(data);
        assert_eq!(encoded, vec![ZDLE, ZDLE ^ 0x40]);
    }

    #[test]
    fn test_encode_multiple_escapes() {
        let data = &[0x11, 0x42, 0x13, 0x43];
        let encoded = encode(data);
        assert_eq!(
            encoded,
            vec![ZDLE, 0x11 ^ 0x40, 0x42, ZDLE, 0x13 ^ 0x40, 0x43]
        );
    }

    #[test]
    fn test_decode_no_escapes() {
        let data = b"HelloWorld";
        let decoded = decode(data).unwrap();
        assert_eq!(decoded, data.to_vec());
    }

    #[test]
    fn test_decode_xon() {
        let data = vec![ZDLE, XON ^ 0x40];
        let decoded = decode(&data).unwrap();
        assert_eq!(decoded, vec![XON]);
    }

    #[test]
    fn test_decode_xoff() {
        let data = vec![ZDLE, XOFF ^ 0x40];
        let decoded = decode(&data).unwrap();
        assert_eq!(decoded, vec![XOFF]);
    }

    #[test]
    fn test_decode_zdle() {
        let data = vec![ZDLE, ZDLE];
        let decoded = decode(&data).unwrap();
        assert_eq!(decoded, vec![ZDLE]);
    }

    #[test]
    fn test_encode_decode_roundtrip() {
        let original = b"Hello\x11\x13World\x18Test";
        let encoded = encode(original);
        let decoded = decode(&encoded).unwrap();
        assert_eq!(decoded, original.to_vec());
    }

    #[test]
    fn test_encode_decode_all_escape_chars() {
        let original = ESCAPE_CHARS.to_vec();
        let encoded = encode(&original);
        let decoded = decode(&encoded).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_decode_invalid_trailing_zdle() {
        let data = vec![0x42, ZDLE];
        let result = decode(&data);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ZmodemError::InvalidEscape));
    }

    #[test]
    fn test_decode_frame_endings() {
        // Frame ending sequences should pass through
        let data = vec![ZDLE, ZCRCE];
        let decoded = decode(&data).unwrap();
        assert_eq!(decoded, vec![ZDLE, ZCRCE]);

        let data = vec![ZDLE, ZCRCG];
        let decoded = decode(&data).unwrap();
        assert_eq!(decoded, vec![ZDLE, ZCRCG]);
    }

    #[test]
    fn test_encode_u32() {
        let value = 0x12345678u32;
        let encoded = encode_u32(value);

        // Decode manually to verify
        let decoded_bytes = decode(&encoded).unwrap();
        let decoded_value = u32::from_le_bytes([
            decoded_bytes[0],
            decoded_bytes[1],
            decoded_bytes[2],
            decoded_bytes[3],
        ]);
        assert_eq!(decoded_value, value);
    }

    #[test]
    fn test_decode_u32() {
        let value = 0x12345678u32;
        let bytes = value.to_le_bytes();
        let encoded = encode(&bytes);
        let decoded = decode_u32(&encoded).unwrap();
        assert_eq!(decoded, value);
    }

    #[test]
    fn test_decode_u32_insufficient_data() {
        let data = vec![ZDLE, 0x40, ZDLE, 0x41]; // Only 2 decoded bytes
        let result = decode_u32(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_should_escape() {
        assert!(should_escape(0x00));
        assert!(should_escape(0x11)); // XON
        assert!(should_escape(0x13)); // XOFF
        assert!(should_escape(0x18)); // ZDLE
        assert!(!should_escape(0x42)); // 'B'
        assert!(!should_escape(0x20)); // space
    }

    #[test]
    fn test_encode_empty() {
        let data = &[];
        let encoded = encode(data);
        assert_eq!(encoded, vec![]);
    }

    #[test]
    fn test_decode_empty() {
        let data = &[];
        let decoded = decode(data).unwrap();
        assert_eq!(decoded, vec![]);
    }
}
