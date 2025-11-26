//! CRC-32/ZMODEM implementation for Zmodem protocol.
//!
//! This module provides CRC-32 calculation using the Zmodem polynomial (0xEDB88320).
//! Used for ZBIN32 frame type checksums.

/// CRC-32/ZMODEM polynomial (reversed)
const CRC32_POLY: u32 = 0xEDB88320;

/// Precomputed CRC-32 lookup table for fast calculation.
const CRC32_TABLE: [u32; 256] = generate_crc32_table();

/// Generate CRC-32 lookup table at compile time.
const fn generate_crc32_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    let mut i = 0;
    while i < 256 {
        let mut crc = i as u32;
        let mut j = 0;
        while j < 8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ CRC32_POLY;
            } else {
                crc >>= 1;
            }
            j += 1;
        }
        table[i] = crc;
        i += 1;
    }
    table
}

/// Calculate CRC-32/ZMODEM checksum for the given data.
///
/// # Arguments
///
/// * `data` - The data to calculate CRC for
///
/// # Returns
///
/// The 32-bit CRC checksum
///
/// # Examples
///
/// ```
/// use impulse_protocol::zmodem::crc32::calculate;
///
/// let data = b"123456789";
/// let crc = calculate(data);
/// assert_eq!(crc, 0xCBF43926); // Known test vector
/// ```
pub fn calculate(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFFFFFF; // Initial value

    for &byte in data {
        let table_index = ((crc ^ u32::from(byte)) & 0xFF) as usize;
        crc = (crc >> 8) ^ CRC32_TABLE[table_index];
    }

    crc ^ 0xFFFFFFFF // XOR out
}

/// Update an existing CRC-32 with new data.
///
/// This allows incremental CRC calculation for streaming data.
///
/// # Arguments
///
/// * `crc` - The current CRC value (without final XOR)
/// * `data` - The new data to add to CRC
///
/// # Returns
///
/// The updated CRC value (without final XOR)
///
/// # Note
///
/// When starting a new calculation, pass `0xFFFFFFFF` as the initial CRC.
/// When finalizing, XOR the result with `0xFFFFFFFF`.
pub fn update(mut crc: u32, data: &[u8]) -> u32 {
    for &byte in data {
        let table_index = ((crc ^ u32::from(byte)) & 0xFF) as usize;
        crc = (crc >> 8) ^ CRC32_TABLE[table_index];
    }
    crc
}

/// Finalize a CRC-32 calculation.
///
/// # Arguments
///
/// * `crc` - The CRC value to finalize
///
/// # Returns
///
/// The finalized CRC value
pub fn finalize(crc: u32) -> u32 {
    crc ^ 0xFFFFFFFF
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc32_empty() {
        let crc = calculate(&[]);
        assert_eq!(crc, 0x00000000);
    }

    #[test]
    fn test_crc32_single_byte() {
        let crc = calculate(&[0x00]);
        assert_eq!(crc, 0xD202EF8D);

        let crc = calculate(&[0xFF]);
        assert_eq!(crc, 0xFF000000);
    }

    #[test]
    fn test_crc32_known_vectors() {
        // Standard test vector: "123456789"
        // CRC-32 with polynomial 0xEDB88320 (reversed)
        let data = b"123456789";
        let crc = calculate(data);
        assert_eq!(crc, 0xCBF43926, "CRC-32/ZMODEM test vector failed");

        // Test various data produces non-zero CRCs
        assert_ne!(calculate(b"The quick brown fox jumps over the lazy dog"), 0);
        assert_ne!(calculate(b"ZMODEM"), 0);
        assert_ne!(calculate(b"Hello World"), 0);
    }

    #[test]
    fn test_crc32_incremental() {
        let data = b"123456789";

        // Calculate in one go
        let crc_full = calculate(data);

        // Calculate incrementally
        let mut crc_incremental = 0xFFFFFFFF;
        crc_incremental = update(crc_incremental, &data[0..3]); // "123"
        crc_incremental = update(crc_incremental, &data[3..6]); // "456"
        crc_incremental = update(crc_incremental, &data[6..9]); // "789"
        crc_incremental = finalize(crc_incremental);

        assert_eq!(crc_full, crc_incremental, "Incremental CRC mismatch");
    }

    #[test]
    fn test_crc32_table_generation() {
        // Verify the lookup table is generated correctly
        // Test a few known values
        assert_eq!(CRC32_TABLE[0], 0x00000000);
        assert_eq!(CRC32_TABLE[1], 0x77073096);
        assert_eq!(CRC32_TABLE[255], 0x2D02EF8D);
    }

    #[test]
    fn test_crc32_all_bytes() {
        // Test that all byte values can be processed
        let data: Vec<u8> = (0..=255).collect();
        let crc = calculate(&data);
        // Verify against known value for this sequence
        assert_eq!(crc, 0x29058C73);
    }

    #[test]
    fn test_crc32_consistency() {
        // Verify that calculating the same data twice gives the same result
        let data = b"consistency test data";
        let crc1 = calculate(data);
        let crc2 = calculate(data);
        assert_eq!(crc1, crc2);
    }

    #[test]
    fn test_crc32_finalize() {
        // Test the finalize function
        let crc = 0x12345678;
        let finalized = finalize(crc);
        assert_eq!(finalized, crc ^ 0xFFFFFFFF);
    }

    #[test]
    fn test_crc32_large_data() {
        // Test with larger data set
        let data = vec![0x55; 1024];
        let crc = calculate(&data);
        assert!(crc != 0, "CRC should not be zero for non-empty data");
    }
}
