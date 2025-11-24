//! Protocol transfer flags for file transfer protocols
//!
//! This module defines flag types for external protocol configurations
//! from the original Pascal RECORDS.PAS file.

use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    /// External protocol flags (Pascal: `xbflags` set type)
    ///
    /// Original Pascal definition (RECORDS.PAS lines 754-758):
    /// ```pascal
    /// xbflags=
    ///  (xbactive,      { protocol is active/enabled }
    ///   xbisbatch,     { protocol supports batch transfers }
    ///   xbisresume,    { protocol supports resume capability }
    ///   xbxferokcode); { transfer OK code }
    /// ```
    ///
    /// Controls external file transfer protocol behavior and capabilities.
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::protocol_flags::ProtocolFlags;
    ///
    /// let mut flags = ProtocolFlags::ACTIVE | ProtocolFlags::BATCH;
    ///
    /// assert!(flags.is_active());
    /// assert!(flags.supports_batch());
    /// assert!(!flags.supports_resume());
    /// ```
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct ProtocolFlags: u8 {
        /// Protocol is active/enabled
        const ACTIVE            = 0b0000_0001;

        /// Protocol supports batch transfers
        const BATCH             = 0b0000_0010;

        /// Protocol supports resume capability
        const RESUME            = 0b0000_0100;

        /// Transfer OK code
        const XFER_OK_CODE      = 0b0000_1000;
    }
}

impl Default for ProtocolFlags {
    fn default() -> Self {
        ProtocolFlags::empty()
    }
}

impl ProtocolFlags {
    /// Create from Pascal byte
    pub fn from_pascal_byte(byte: u8) -> Self {
        ProtocolFlags::from_bits_truncate(byte)
    }

    /// Convert to Pascal byte
    pub fn to_pascal_byte(self) -> u8 {
        self.bits()
    }

    /// Check if protocol is active/enabled
    pub fn is_active(self) -> bool {
        self.contains(ProtocolFlags::ACTIVE)
    }

    /// Check if protocol supports batch transfers
    pub fn supports_batch(self) -> bool {
        self.contains(ProtocolFlags::BATCH)
    }

    /// Check if protocol supports resume capability
    pub fn supports_resume(self) -> bool {
        self.contains(ProtocolFlags::RESUME)
    }

    /// Check if transfer OK code is set
    pub fn has_xfer_ok_code(self) -> bool {
        self.contains(ProtocolFlags::XFER_OK_CODE)
    }

    /// Check if protocol can be used (is active)
    pub fn is_usable(self) -> bool {
        self.is_active()
    }

    /// Check if protocol supports advanced features (batch or resume)
    pub fn supports_advanced_features(self) -> bool {
        self.intersects(ProtocolFlags::BATCH | ProtocolFlags::RESUME)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_flags_default() {
        let flags = ProtocolFlags::default();
        assert!(flags.is_empty());
        assert!(!flags.is_active());
    }

    #[test]
    fn test_protocol_flags_pascal_conversion() {
        let flags = ProtocolFlags::ACTIVE | ProtocolFlags::BATCH | ProtocolFlags::RESUME;
        let byte = flags.to_pascal_byte();
        let restored = ProtocolFlags::from_pascal_byte(byte);
        assert_eq!(flags, restored);
    }

    #[test]
    fn test_protocol_flags_active() {
        let mut flags = ProtocolFlags::empty();
        assert!(!flags.is_active());
        assert!(!flags.is_usable());

        flags.insert(ProtocolFlags::ACTIVE);
        assert!(flags.is_active());
        assert!(flags.is_usable());
    }

    #[test]
    fn test_protocol_flags_batch() {
        let flags = ProtocolFlags::ACTIVE | ProtocolFlags::BATCH;
        assert!(flags.supports_batch());
        assert!(flags.supports_advanced_features());
    }

    #[test]
    fn test_protocol_flags_resume() {
        let flags = ProtocolFlags::ACTIVE | ProtocolFlags::RESUME;
        assert!(flags.supports_resume());
        assert!(flags.supports_advanced_features());
    }

    #[test]
    fn test_protocol_flags_xfer_ok_code() {
        let flags = ProtocolFlags::XFER_OK_CODE;
        assert!(flags.has_xfer_ok_code());
    }

    #[test]
    fn test_protocol_flags_full_featured() {
        let flags = ProtocolFlags::ACTIVE
            | ProtocolFlags::BATCH
            | ProtocolFlags::RESUME
            | ProtocolFlags::XFER_OK_CODE;

        assert!(flags.is_active());
        assert!(flags.supports_batch());
        assert!(flags.supports_resume());
        assert!(flags.has_xfer_ok_code());
        assert!(flags.supports_advanced_features());
    }

    #[test]
    fn test_protocol_flags_advanced_features() {
        let basic = ProtocolFlags::ACTIVE;
        assert!(!basic.supports_advanced_features());

        let batch_only = ProtocolFlags::ACTIVE | ProtocolFlags::BATCH;
        assert!(batch_only.supports_advanced_features());

        let resume_only = ProtocolFlags::ACTIVE | ProtocolFlags::RESUME;
        assert!(resume_only.supports_advanced_features());

        let both = ProtocolFlags::ACTIVE | ProtocolFlags::BATCH | ProtocolFlags::RESUME;
        assert!(both.supports_advanced_features());
    }

    #[test]
    fn test_protocol_flags_serialization() {
        let flags = ProtocolFlags::ACTIVE | ProtocolFlags::BATCH;
        let json = serde_json::to_string(&flags).unwrap();
        let restored: ProtocolFlags = serde_json::from_str(&json).unwrap();
        assert_eq!(flags, restored);
    }

    #[test]
    fn test_protocol_flags_binary_compatibility() {
        // Test that bit positions match Pascal expectations
        assert_eq!(ProtocolFlags::ACTIVE.bits(), 0b0001);
        assert_eq!(ProtocolFlags::BATCH.bits(), 0b0010);
        assert_eq!(ProtocolFlags::RESUME.bits(), 0b0100);
        assert_eq!(ProtocolFlags::XFER_OK_CODE.bits(), 0b1000);
    }

    #[test]
    fn test_protocol_flags_roundtrip() {
        for byte in 0..=15u8 {
            // Only test valid combinations (4 bits)
            let flags = ProtocolFlags::from_pascal_byte(byte);
            let restored = flags.to_pascal_byte();
            assert_eq!(byte & 0b1111, restored);
        }
    }
}
