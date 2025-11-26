//! CRC-16/XMODEM implementation for Zmodem protocol.
//!
//! This module provides CRC-16 calculation using the XMODEM polynomial (0x1021).
//! Used for ZBIN frame type checksums.

/// CRC-16/XMODEM polynomial: x^16 + x^12 + x^5 + 1
const CRC16_POLY: u16 = 0x1021;

/// Precomputed CRC-16 lookup table for fast calculation.
const CRC16_TABLE: [u16; 256] = generate_crc16_table();

/// Generate CRC-16 lookup table at compile time.
const fn generate_crc16_table() -> [u16; 256] {
    let mut table = [0u16; 256];
    let mut i = 0;
    while i < 256 {
        let mut crc = (i as u16) << 8;
        let mut j = 0;
        while j < 8 {
            if crc & 0x8000 != 0 {
                crc = (crc << 1) ^ CRC16_POLY;
            } else {
                crc <<= 1;
            }
            j += 1;
        }
        table[i] = crc;
        i += 1;
    }
    table
}

/// Calculate CRC-16/XMODEM checksum for the given data.
///
/// # Arguments
///
/// * `data` - The data to calculate CRC for
///
/// # Returns
///
/// The 16-bit CRC checksum
///
/// # Examples
///
/// ```
/// use impulse_protocol::zmodem::crc16::calculate;
///
/// let data = b"123456789";
/// let crc = calculate(data);
/// assert_eq!(crc, 0x31C3); // Known test vector
/// ```
pub fn calculate(data: &[u8]) -> u16 {
    let mut crc: u16 = 0x0000; // Initial value

    for &byte in data {
        let table_index = ((crc >> 8) ^ u16::from(byte)) as u8;
        crc = (crc << 8) ^ CRC16_TABLE[table_index as usize];
    }

    crc
}

/// Update an existing CRC-16 with new data.
///
/// This allows incremental CRC calculation for streaming data.
///
/// # Arguments
///
/// * `crc` - The current CRC value
/// * `data` - The new data to add to CRC
///
/// # Returns
///
/// The updated CRC value
pub fn update(mut crc: u16, data: &[u8]) -> u16 {
    for &byte in data {
        let table_index = ((crc >> 8) ^ u16::from(byte)) as u8;
        crc = (crc << 8) ^ CRC16_TABLE[table_index as usize];
    }
    crc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc16_empty() {
        let crc = calculate(&[]);
        assert_eq!(crc, 0x0000);
    }

    #[test]
    fn test_crc16_single_byte() {
        let crc = calculate(&[0x00]);
        assert_eq!(crc, 0x0000);

        let crc = calculate(&[0xFF]);
        assert_eq!(crc, 0x1EF0);
    }

    #[test]
    fn test_crc16_known_vectors() {
        // Standard test vector: "123456789"
        // CRC-16/XMODEM with polynomial 0x1021
        let data = b"123456789";
        let crc = calculate(data);
        assert_eq!(crc, 0x31C3, "CRC-16/XMODEM test vector failed");

        // Test various data produces non-zero CRCs
        assert_ne!(calculate(b"The quick brown fox"), 0);
        assert_ne!(calculate(b"ZMODEM"), 0);
        assert_ne!(calculate(b"Hello World"), 0);
    }

    #[test]
    fn test_crc16_incremental() {
        let data = b"123456789";

        // Calculate in one go
        let crc_full = calculate(data);

        // Calculate incrementally
        let mut crc_incremental = 0x0000;
        crc_incremental = update(crc_incremental, &data[0..3]); // "123"
        crc_incremental = update(crc_incremental, &data[3..6]); // "456"
        crc_incremental = update(crc_incremental, &data[6..9]); // "789"

        assert_eq!(crc_full, crc_incremental, "Incremental CRC mismatch");
    }

    #[test]
    fn test_crc16_table_generation() {
        // Verify the lookup table is generated correctly
        // Test a few known values
        assert_eq!(CRC16_TABLE[0], 0x0000);
        assert_eq!(CRC16_TABLE[1], 0x1021);
        assert_eq!(CRC16_TABLE[255], 0x1EF0);
    }

    #[test]
    fn test_crc16_all_bytes() {
        // Test that all byte values can be processed
        let data: Vec<u8> = (0..=255).collect();
        let crc = calculate(&data);
        // Just verify it doesn't panic and produces a value
        assert!(crc != 0);
    }

    #[test]
    fn test_crc16_consistency() {
        // Verify that calculating the same data twice gives the same result
        let data = b"consistency test data";
        let crc1 = calculate(data);
        let crc2 = calculate(data);
        assert_eq!(crc1, crc2);
    }
}
