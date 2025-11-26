//! Xmodem block structure and encoding.
//!
//! This module defines the structure of Xmodem data blocks and provides
//! serialization/deserialization functionality.

use super::checksum;
use super::crc;
use super::error::{Result, XmodemError};
use super::variants::XmodemVariant;

/// Maximum block size (Xmodem-1K).
pub const MAX_BLOCK_SIZE: usize = 1024;

/// Xmodem data block.
///
/// Represents a single block of data in the Xmodem protocol.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmodemBlock {
    /// Block number (wraps at 256).
    pub block_num: u8,
    /// Block data (128 or 1024 bytes).
    pub data: Vec<u8>,
    /// Protocol variant.
    pub variant: XmodemVariant,
}

impl XmodemBlock {
    /// Create a new Xmodem block.
    ///
    /// # Arguments
    ///
    /// * `block_num` - Block number (0-255, wrapping)
    /// * `data` - Block data (must match variant block size)
    /// * `variant` - Protocol variant
    ///
    /// # Returns
    ///
    /// * `Ok(block)` if data size matches variant
    /// * `Err(XmodemError)` if data size is incorrect
    pub fn new(block_num: u8, data: Vec<u8>, variant: XmodemVariant) -> Result<Self> {
        if data.len() != variant.block_size() {
            return Err(XmodemError::InvalidVariant(format!(
                "Data size {} does not match variant block size {}",
                data.len(),
                variant.block_size()
            )));
        }

        Ok(Self {
            block_num,
            data,
            variant,
        })
    }

    /// Serialize this block into bytes for transmission.
    ///
    /// Format: [header][block#][~block#][data...][checksum/crc]
    ///
    /// # Returns
    ///
    /// A vector containing the complete serialized block
    pub fn serialize(&self) -> Vec<u8> {
        let mut packet = Vec::with_capacity(self.variant.packet_size());

        // Header byte (SOH or STX)
        packet.push(self.variant.header_byte());

        // Block number and complement
        packet.push(self.block_num);
        packet.push(!self.block_num);

        // Data
        packet.extend_from_slice(&self.data);

        // Error detection (checksum or CRC)
        if self.variant.uses_crc() {
            let crc_val = crc::calculate(&self.data);
            packet.push((crc_val >> 8) as u8); // High byte first
            packet.push((crc_val & 0xFF) as u8); // Low byte
        } else {
            let checksum_val = checksum::calculate(&self.data);
            packet.push(checksum_val);
        }

        packet
    }

    /// Deserialize a block from received bytes.
    ///
    /// # Arguments
    ///
    /// * `packet` - The received packet bytes
    /// * `use_crc` - Whether to expect CRC (vs checksum)
    ///
    /// # Returns
    ///
    /// * `Ok(block)` if packet is valid
    /// * `Err(XmodemError)` if packet is malformed or validation fails
    pub fn deserialize(packet: &[u8], use_crc: bool) -> Result<Self> {
        if packet.is_empty() {
            return Err(XmodemError::UnexpectedEof);
        }

        // Determine variant from header byte
        let header = packet[0];
        let variant = XmodemVariant::from_header_byte(header, use_crc)?;

        // Check packet size
        if packet.len() != variant.packet_size() {
            return Err(XmodemError::InvalidVariant(format!(
                "Packet size {} does not match expected {}",
                packet.len(),
                variant.packet_size()
            )));
        }

        // Extract block number and complement
        let block_num = packet[1];
        let block_complement = packet[2];

        // Verify complement
        if block_num != !block_complement {
            return Err(XmodemError::ComplementMismatch {
                block: block_num,
                complement: block_complement,
            });
        }

        // Extract data
        let data_start = 3;
        let data_end = data_start + variant.block_size();
        let data = packet[data_start..data_end].to_vec();

        // Verify error detection
        if variant.uses_crc() {
            let received_crc = u16::from_be_bytes([packet[data_end], packet[data_end + 1]]);
            let calculated_crc = crc::calculate(&data);

            if received_crc != calculated_crc {
                return Err(XmodemError::CrcMismatch {
                    expected: received_crc,
                    actual: calculated_crc,
                });
            }
        } else {
            let received_checksum = packet[data_end];
            let calculated_checksum = checksum::calculate(&data);

            if received_checksum != calculated_checksum {
                return Err(XmodemError::ChecksumMismatch {
                    expected: received_checksum,
                    actual: calculated_checksum,
                });
            }
        }

        Ok(Self {
            block_num,
            data,
            variant,
        })
    }

    /// Check if this block number matches the expected number.
    ///
    /// Handles wrap-around at 256.
    pub fn is_expected_block(&self, expected: u8) -> bool {
        self.block_num == expected
    }

    /// Get the size of the data in this block.
    pub fn data_len(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xmodem::{SOH, STX};

    #[test]
    fn test_block_creation() {
        let data = vec![0x42; 128];
        let block = XmodemBlock::new(1, data.clone(), XmodemVariant::Checksum).unwrap();
        assert_eq!(block.block_num, 1);
        assert_eq!(block.data, data);
        assert_eq!(block.variant, XmodemVariant::Checksum);
    }

    #[test]
    fn test_block_creation_wrong_size() {
        let data = vec![0x42; 64]; // Wrong size for 128-byte variant
        let result = XmodemBlock::new(1, data, XmodemVariant::Checksum);
        assert!(result.is_err());
    }

    #[test]
    fn test_serialize_checksum() {
        let data = vec![0x41; 128]; // 'A' repeated
        let block = XmodemBlock::new(1, data.clone(), XmodemVariant::Checksum).unwrap();
        let packet = block.serialize();

        assert_eq!(packet.len(), 132); // 1 + 1 + 1 + 128 + 1
        assert_eq!(packet[0], SOH); // Header
        assert_eq!(packet[1], 1); // Block number
        assert_eq!(packet[2], 254); // Complement of 1
        assert_eq!(&packet[3..131], &data[..]); // Data
        assert_eq!(packet[131], checksum::calculate(&data)); // Checksum
    }

    #[test]
    fn test_serialize_crc() {
        let data = vec![0x42; 128];
        let block = XmodemBlock::new(5, data.clone(), XmodemVariant::Crc).unwrap();
        let packet = block.serialize();

        assert_eq!(packet.len(), 133); // 1 + 1 + 1 + 128 + 2
        assert_eq!(packet[0], SOH); // Header
        assert_eq!(packet[1], 5); // Block number
        assert_eq!(packet[2], 250); // Complement of 5

        let crc_val = crc::calculate(&data);
        assert_eq!(packet[131], (crc_val >> 8) as u8); // CRC high byte
        assert_eq!(packet[132], (crc_val & 0xFF) as u8); // CRC low byte
    }

    #[test]
    fn test_serialize_1k() {
        let data = vec![0x55; 1024];
        let block = XmodemBlock::new(10, data.clone(), XmodemVariant::OneK).unwrap();
        let packet = block.serialize();

        assert_eq!(packet.len(), 1029); // 1 + 1 + 1 + 1024 + 2
        assert_eq!(packet[0], STX); // Header for 1K
        assert_eq!(packet[1], 10); // Block number
        assert_eq!(packet[2], 245); // Complement of 10
    }

    #[test]
    fn test_deserialize_checksum() {
        let data = vec![0x41; 128];
        let block = XmodemBlock::new(1, data.clone(), XmodemVariant::Checksum).unwrap();
        let packet = block.serialize();

        let deserialized = XmodemBlock::deserialize(&packet, false).unwrap();
        assert_eq!(deserialized.block_num, 1);
        assert_eq!(deserialized.data, data);
        assert_eq!(deserialized.variant, XmodemVariant::Checksum);
    }

    #[test]
    fn test_deserialize_crc() {
        let data = vec![0x42; 128];
        let block = XmodemBlock::new(5, data.clone(), XmodemVariant::Crc).unwrap();
        let packet = block.serialize();

        let deserialized = XmodemBlock::deserialize(&packet, true).unwrap();
        assert_eq!(deserialized.block_num, 5);
        assert_eq!(deserialized.data, data);
        assert_eq!(deserialized.variant, XmodemVariant::Crc);
    }

    #[test]
    fn test_deserialize_1k() {
        let data = vec![0x55; 1024];
        let block = XmodemBlock::new(10, data.clone(), XmodemVariant::OneK).unwrap();
        let packet = block.serialize();

        let deserialized = XmodemBlock::deserialize(&packet, true).unwrap();
        assert_eq!(deserialized.block_num, 10);
        assert_eq!(deserialized.data, data);
        assert_eq!(deserialized.variant, XmodemVariant::OneK);
    }

    #[test]
    fn test_deserialize_bad_checksum() {
        let data = vec![0x41; 128];
        let block = XmodemBlock::new(1, data, XmodemVariant::Checksum).unwrap();
        let mut packet = block.serialize();

        // Corrupt checksum
        packet[131] ^= 0xFF;

        let result = XmodemBlock::deserialize(&packet, false);
        assert!(matches!(result, Err(XmodemError::ChecksumMismatch { .. })));
    }

    #[test]
    fn test_deserialize_bad_crc() {
        let data = vec![0x42; 128];
        let block = XmodemBlock::new(1, data, XmodemVariant::Crc).unwrap();
        let mut packet = block.serialize();

        // Corrupt CRC
        packet[131] ^= 0xFF;

        let result = XmodemBlock::deserialize(&packet, true);
        assert!(matches!(result, Err(XmodemError::CrcMismatch { .. })));
    }

    #[test]
    fn test_deserialize_bad_complement() {
        let mut packet = vec![SOH, 1, 253]; // Wrong complement (should be 254)
        packet.extend_from_slice(&[0x41; 128]);
        packet.push(checksum::calculate(&[0x41; 128]));

        let result = XmodemBlock::deserialize(&packet, false);
        assert!(matches!(
            result,
            Err(XmodemError::ComplementMismatch { .. })
        ));
    }

    #[test]
    fn test_is_expected_block() {
        let data = vec![0x42; 128];
        let block = XmodemBlock::new(5, data, XmodemVariant::Checksum).unwrap();

        assert!(block.is_expected_block(5));
        assert!(!block.is_expected_block(6));
    }

    #[test]
    fn test_block_wrap_around() {
        let data = vec![0x42; 128];
        let block = XmodemBlock::new(255, data, XmodemVariant::Checksum).unwrap();
        assert_eq!(block.block_num, 255);

        // Next block would be 0
        assert!(block.is_expected_block(255));
        assert!(!block.is_expected_block(0));
    }

    #[test]
    fn test_data_len() {
        let data_128 = vec![0x42; 128];
        let block_128 = XmodemBlock::new(1, data_128, XmodemVariant::Checksum).unwrap();
        assert_eq!(block_128.data_len(), 128);

        let data_1k = vec![0x42; 1024];
        let block_1k = XmodemBlock::new(1, data_1k, XmodemVariant::OneK).unwrap();
        assert_eq!(block_1k.data_len(), 1024);
    }

    #[test]
    fn test_round_trip_all_variants() {
        for variant in [
            XmodemVariant::Checksum,
            XmodemVariant::Crc,
            XmodemVariant::OneK,
        ] {
            let data = vec![0x42; variant.block_size()];
            let block = XmodemBlock::new(123, data.clone(), variant).unwrap();
            let packet = block.serialize();
            let deserialized = XmodemBlock::deserialize(&packet, variant.uses_crc()).unwrap();

            assert_eq!(deserialized.block_num, 123);
            assert_eq!(deserialized.data, data);
            assert_eq!(deserialized.variant, variant);
        }
    }
}
