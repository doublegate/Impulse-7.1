//! Xmodem protocol variants.
//!
//! This module defines the different variants of the Xmodem protocol.

use super::error::{Result, XmodemError};

/// Xmodem protocol variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum XmodemVariant {
    /// Original Xmodem with 128-byte blocks and simple checksum.
    Checksum,
    /// Xmodem-CRC with 128-byte blocks and CRC-16 error detection.
    Crc,
    /// Xmodem-1K with 1024-byte blocks and CRC-16 error detection.
    OneK,
}

impl XmodemVariant {
    /// Get the block size for this variant.
    ///
    /// # Returns
    ///
    /// * `128` for Checksum and Crc variants
    /// * `1024` for OneK variant
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_protocol::xmodem::XmodemVariant;
    ///
    /// assert_eq!(XmodemVariant::Checksum.block_size(), 128);
    /// assert_eq!(XmodemVariant::Crc.block_size(), 128);
    /// assert_eq!(XmodemVariant::OneK.block_size(), 1024);
    /// ```
    pub const fn block_size(self) -> usize {
        match self {
            XmodemVariant::Checksum | XmodemVariant::Crc => 128,
            XmodemVariant::OneK => 1024,
        }
    }

    /// Check if this variant uses CRC-16 for error detection.
    ///
    /// # Returns
    ///
    /// * `true` for Crc and OneK variants
    /// * `false` for Checksum variant
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_protocol::xmodem::XmodemVariant;
    ///
    /// assert!(!XmodemVariant::Checksum.uses_crc());
    /// assert!(XmodemVariant::Crc.uses_crc());
    /// assert!(XmodemVariant::OneK.uses_crc());
    /// ```
    pub const fn uses_crc(self) -> bool {
        matches!(self, XmodemVariant::Crc | XmodemVariant::OneK)
    }

    /// Get the start-of-header byte for this variant.
    ///
    /// # Returns
    ///
    /// * `SOH (0x01)` for 128-byte blocks (Checksum, Crc)
    /// * `STX (0x02)` for 1024-byte blocks (OneK)
    pub const fn header_byte(self) -> u8 {
        match self {
            XmodemVariant::Checksum | XmodemVariant::Crc => super::SOH,
            XmodemVariant::OneK => super::STX,
        }
    }

    /// Parse variant from start-of-header byte.
    ///
    /// # Arguments
    ///
    /// * `byte` - The header byte to parse
    /// * `use_crc` - Whether to prefer CRC variant for SOH
    ///
    /// # Returns
    ///
    /// * `Ok(variant)` if byte is valid (SOH or STX)
    /// * `Err(XmodemError)` if byte is invalid
    pub fn from_header_byte(byte: u8, use_crc: bool) -> Result<Self> {
        match byte {
            super::SOH => {
                if use_crc {
                    Ok(XmodemVariant::Crc)
                } else {
                    Ok(XmodemVariant::Checksum)
                }
            }
            super::STX => Ok(XmodemVariant::OneK),
            _ => Err(XmodemError::InvalidBlockHeader(byte)),
        }
    }

    /// Get the error detection size in bytes.
    ///
    /// # Returns
    ///
    /// * `1` for checksum (Checksum variant)
    /// * `2` for CRC-16 (Crc and OneK variants)
    pub const fn error_detection_size(self) -> usize {
        match self {
            XmodemVariant::Checksum => 1,
            XmodemVariant::Crc | XmodemVariant::OneK => 2,
        }
    }

    /// Get the total block packet size (header + block# + ~block# + data + error detection).
    ///
    /// # Returns
    ///
    /// * `132` for Checksum (1 + 1 + 1 + 128 + 1)
    /// * `133` for Crc (1 + 1 + 1 + 128 + 2)
    /// * `1029` for OneK (1 + 1 + 1 + 1024 + 2)
    pub const fn packet_size(self) -> usize {
        1 + // header byte (SOH or STX)
        1 + // block number
        1 + // block number complement
        self.block_size() +
        self.error_detection_size()
    }

    /// Get a human-readable name for this variant.
    pub const fn name(self) -> &'static str {
        match self {
            XmodemVariant::Checksum => "Xmodem (Checksum)",
            XmodemVariant::Crc => "Xmodem-CRC",
            XmodemVariant::OneK => "Xmodem-1K",
        }
    }
}

impl Default for XmodemVariant {
    /// Default variant is Xmodem-1K (most capable).
    fn default() -> Self {
        XmodemVariant::OneK
    }
}

impl std::fmt::Display for XmodemVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_sizes() {
        assert_eq!(XmodemVariant::Checksum.block_size(), 128);
        assert_eq!(XmodemVariant::Crc.block_size(), 128);
        assert_eq!(XmodemVariant::OneK.block_size(), 1024);
    }

    #[test]
    fn test_uses_crc() {
        assert!(!XmodemVariant::Checksum.uses_crc());
        assert!(XmodemVariant::Crc.uses_crc());
        assert!(XmodemVariant::OneK.uses_crc());
    }

    #[test]
    fn test_header_bytes() {
        assert_eq!(XmodemVariant::Checksum.header_byte(), 0x01); // SOH
        assert_eq!(XmodemVariant::Crc.header_byte(), 0x01); // SOH
        assert_eq!(XmodemVariant::OneK.header_byte(), 0x02); // STX
    }

    #[test]
    fn test_from_header_byte() {
        // SOH with checksum
        let variant = XmodemVariant::from_header_byte(0x01, false).unwrap();
        assert_eq!(variant, XmodemVariant::Checksum);

        // SOH with CRC
        let variant = XmodemVariant::from_header_byte(0x01, true).unwrap();
        assert_eq!(variant, XmodemVariant::Crc);

        // STX always means 1K
        let variant = XmodemVariant::from_header_byte(0x02, false).unwrap();
        assert_eq!(variant, XmodemVariant::OneK);

        let variant = XmodemVariant::from_header_byte(0x02, true).unwrap();
        assert_eq!(variant, XmodemVariant::OneK);

        // Invalid header
        assert!(XmodemVariant::from_header_byte(0xFF, false).is_err());
    }

    #[test]
    fn test_error_detection_size() {
        assert_eq!(XmodemVariant::Checksum.error_detection_size(), 1);
        assert_eq!(XmodemVariant::Crc.error_detection_size(), 2);
        assert_eq!(XmodemVariant::OneK.error_detection_size(), 2);
    }

    #[test]
    fn test_packet_size() {
        // Checksum: 1 (SOH) + 1 (block#) + 1 (~block#) + 128 (data) + 1 (checksum) = 132
        assert_eq!(XmodemVariant::Checksum.packet_size(), 132);

        // CRC: 1 (SOH) + 1 (block#) + 1 (~block#) + 128 (data) + 2 (CRC) = 133
        assert_eq!(XmodemVariant::Crc.packet_size(), 133);

        // 1K: 1 (STX) + 1 (block#) + 1 (~block#) + 1024 (data) + 2 (CRC) = 1029
        assert_eq!(XmodemVariant::OneK.packet_size(), 1029);
    }

    #[test]
    fn test_default() {
        assert_eq!(XmodemVariant::default(), XmodemVariant::OneK);
    }

    #[test]
    fn test_display() {
        assert_eq!(XmodemVariant::Checksum.to_string(), "Xmodem (Checksum)");
        assert_eq!(XmodemVariant::Crc.to_string(), "Xmodem-CRC");
        assert_eq!(XmodemVariant::OneK.to_string(), "Xmodem-1K");
    }

    #[test]
    fn test_clone_copy() {
        let variant = XmodemVariant::Crc;
        let cloned = variant;
        assert_eq!(variant, cloned);
    }

    #[test]
    fn test_eq_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(XmodemVariant::Checksum);
        set.insert(XmodemVariant::Crc);
        set.insert(XmodemVariant::OneK);

        assert_eq!(set.len(), 3);
        assert!(set.contains(&XmodemVariant::Checksum));
        assert!(set.contains(&XmodemVariant::Crc));
        assert!(set.contains(&XmodemVariant::OneK));
    }

    #[test]
    fn test_name() {
        assert_eq!(XmodemVariant::Checksum.name(), "Xmodem (Checksum)");
        assert_eq!(XmodemVariant::Crc.name(), "Xmodem-CRC");
        assert_eq!(XmodemVariant::OneK.name(), "Xmodem-1K");
    }
}
