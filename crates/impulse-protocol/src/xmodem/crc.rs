//! CRC-16/XMODEM implementation for Xmodem-CRC variant.
//!
//! This module re-exports the CRC-16 implementation from the Zmodem module,
//! as both protocols use the same CRC-16/XMODEM polynomial (0x1021).

// Re-export CRC-16 functions from zmodem module
pub use crate::zmodem::crc16::{calculate, update};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc_empty() {
        let crc = calculate(&[]);
        assert_eq!(crc, 0x0000);
    }

    #[test]
    fn test_crc_known_vector() {
        // Standard test vector: "123456789"
        // CRC-16/XMODEM with polynomial 0x1021
        let data = b"123456789";
        let crc = calculate(data);
        assert_eq!(crc, 0x31C3, "CRC-16/XMODEM test vector failed");
    }

    #[test]
    fn test_crc_128_block() {
        // Test typical Xmodem block size
        let data = [0x42; 128];
        let crc = calculate(&data);
        assert_ne!(crc, 0); // Should produce non-zero CRC
    }

    #[test]
    fn test_crc_1024_block() {
        // Test Xmodem-1K block size
        let data = [0x55; 1024];
        let crc = calculate(&data);
        assert_ne!(crc, 0); // Should produce non-zero CRC
    }

    #[test]
    fn test_crc_incremental() {
        let data = b"The quick brown fox";

        // Calculate in one go
        let crc_full = calculate(data);

        // Calculate incrementally
        let mut crc_incremental = 0x0000;
        crc_incremental = update(crc_incremental, &data[0..8]);
        crc_incremental = update(crc_incremental, &data[8..16]);
        crc_incremental = update(crc_incremental, &data[16..]);

        assert_eq!(crc_full, crc_incremental);
    }

    #[test]
    fn test_crc_consistency() {
        // Verify that calculating the same data twice gives the same result
        let data = b"consistency test";
        let crc1 = calculate(data);
        let crc2 = calculate(data);
        assert_eq!(crc1, crc2);
    }

    #[test]
    fn test_crc_different_data() {
        // Verify different data produces different CRCs
        let data1 = b"Hello";
        let data2 = b"World";
        let crc1 = calculate(data1);
        let crc2 = calculate(data2);
        assert_ne!(crc1, crc2);
    }
}
