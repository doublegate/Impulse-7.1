//! Simple checksum implementation for original Xmodem protocol.
//!
//! This module provides the basic checksum calculation used in the original
//! Xmodem protocol: sum of all bytes modulo 256.

/// Calculate simple 8-bit checksum for the given data.
///
/// This is the original Xmodem checksum: sum all bytes and take the low 8 bits.
///
/// # Arguments
///
/// * `data` - The data to calculate checksum for
///
/// # Returns
///
/// The 8-bit checksum value
///
/// # Examples
///
/// ```
/// use impulse_protocol::xmodem::checksum::calculate;
///
/// let data = b"Hello";
/// let checksum = calculate(data);
/// // Sum with wrapping: 'H'(72) + 'e'(101) + 'l'(108) + 'l'(108) + 'o'(111) = 500 % 256 = 244
/// assert_eq!(checksum, 244);
/// ```
pub fn calculate(data: &[u8]) -> u8 {
    data.iter().fold(0u8, |sum, &byte| sum.wrapping_add(byte))
}

/// Update an existing checksum with new data.
///
/// This allows incremental checksum calculation for streaming data.
///
/// # Arguments
///
/// * `checksum` - The current checksum value
/// * `data` - The new data to add to checksum
///
/// # Returns
///
/// The updated checksum value
///
/// # Examples
///
/// ```
/// use impulse_protocol::xmodem::checksum::{calculate, update};
///
/// let data = b"Hello, World!";
/// let checksum_full = calculate(data);
///
/// let mut checksum_incremental = 0;
/// checksum_incremental = update(checksum_incremental, b"Hello, ");
/// checksum_incremental = update(checksum_incremental, b"World!");
///
/// assert_eq!(checksum_full, checksum_incremental);
/// ```
pub fn update(checksum: u8, data: &[u8]) -> u8 {
    data.iter()
        .fold(checksum, |sum, &byte| sum.wrapping_add(byte))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum_empty() {
        let checksum = calculate(&[]);
        assert_eq!(checksum, 0x00);
    }

    #[test]
    fn test_checksum_single_byte() {
        let checksum = calculate(&[0x42]);
        assert_eq!(checksum, 0x42);

        let checksum = calculate(&[0xFF]);
        assert_eq!(checksum, 0xFF);

        let checksum = calculate(&[0x00]);
        assert_eq!(checksum, 0x00);
    }

    #[test]
    fn test_checksum_multiple_bytes() {
        let data = b"Hello";
        let expected = (b'H' as u16 + b'e' as u16 + b'l' as u16 + b'l' as u16 + b'o' as u16) as u8;
        assert_eq!(calculate(data), expected);
    }

    #[test]
    fn test_checksum_overflow() {
        // Test that overflow wraps correctly
        let data = [0xFF, 0xFF];
        let checksum = calculate(&data);
        assert_eq!(checksum, 0xFE); // 0xFF + 0xFF = 0x1FE, low byte = 0xFE
    }

    #[test]
    fn test_checksum_all_zeros() {
        let data = [0x00; 128];
        let checksum = calculate(&data);
        assert_eq!(checksum, 0x00);
    }

    #[test]
    fn test_checksum_all_ones() {
        let data = [0xFF; 128];
        let checksum = calculate(&data);
        // 128 * 0xFF = 0x7F80, low byte = 0x80
        assert_eq!(checksum, 0x80);
    }

    #[test]
    fn test_checksum_incremental() {
        let data = b"The quick brown fox jumps over the lazy dog";

        // Calculate in one go
        let checksum_full = calculate(data);

        // Calculate incrementally
        let mut checksum_incremental = 0;
        checksum_incremental = update(checksum_incremental, &data[0..10]);
        checksum_incremental = update(checksum_incremental, &data[10..20]);
        checksum_incremental = update(checksum_incremental, &data[20..30]);
        checksum_incremental = update(checksum_incremental, &data[30..]);

        assert_eq!(checksum_full, checksum_incremental);
    }

    #[test]
    fn test_checksum_consistency() {
        // Verify that calculating the same data twice gives the same result
        let data = b"consistency test data";
        let checksum1 = calculate(data);
        let checksum2 = calculate(data);
        assert_eq!(checksum1, checksum2);
    }

    #[test]
    fn test_checksum_different_data() {
        // Verify different data produces different checksums
        let data1 = b"Hello";
        let data2 = b"World";
        let checksum1 = calculate(data1);
        let checksum2 = calculate(data2);
        assert_ne!(checksum1, checksum2);
    }

    #[test]
    fn test_checksum_128_block() {
        // Test typical Xmodem block size
        let data = [0x42; 128];
        let checksum = calculate(&data);
        // 128 * 0x42 = 0x2100, low byte = 0x00
        assert_eq!(checksum, 0x00);
    }

    #[test]
    fn test_checksum_1024_block() {
        // Test Xmodem-1K block size
        let data = [0x55; 1024];
        let checksum = calculate(&data);
        // 1024 * 0x55 = 0x15400, low byte = 0x00
        assert_eq!(checksum, 0x00);
    }
}
