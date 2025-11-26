//! Protocol selection and preferences.
//!
//! This module provides types for selecting between different file transfer protocols.

use crate::xmodem::XmodemVariant;
use serde::{Deserialize, Serialize};

/// File transfer protocol selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FileProtocol {
    /// Zmodem protocol (robust, crash recovery, streaming).
    Zmodem,
    /// Ymodem protocol (batch transfers, file metadata).
    Ymodem,
    /// Ymodem-G protocol (streaming variant without per-block ACK).
    YmodemG,
    /// Xmodem-1K protocol (1024-byte blocks, CRC-16).
    Xmodem1K,
    /// Xmodem-CRC protocol (128-byte blocks, CRC-16).
    XmodemCrc,
    /// Xmodem protocol (128-byte blocks, checksum).
    Xmodem,
}

impl FileProtocol {
    /// Get the name of the protocol.
    pub const fn name(self) -> &'static str {
        match self {
            FileProtocol::Zmodem => "Zmodem",
            FileProtocol::Ymodem => "Ymodem",
            FileProtocol::YmodemG => "Ymodem-G",
            FileProtocol::Xmodem1K => "Xmodem-1K",
            FileProtocol::XmodemCrc => "Xmodem-CRC",
            FileProtocol::Xmodem => "Xmodem",
        }
    }

    /// Get a description of the protocol.
    pub const fn description(self) -> &'static str {
        match self {
            FileProtocol::Zmodem => "Robust protocol with crash recovery and streaming",
            FileProtocol::Ymodem => "Batch transfers with file metadata",
            FileProtocol::YmodemG => "Fast streaming variant of Ymodem (no per-block ACK)",
            FileProtocol::Xmodem1K => "1024-byte blocks with CRC-16 error detection",
            FileProtocol::XmodemCrc => "128-byte blocks with CRC-16 error detection",
            FileProtocol::Xmodem => "Original protocol with 128-byte blocks and checksum",
        }
    }

    /// Check if this protocol supports batch transfers.
    pub const fn supports_batch(self) -> bool {
        matches!(self, FileProtocol::Ymodem | FileProtocol::YmodemG)
    }

    /// Check if this protocol supports crash recovery.
    pub const fn supports_crash_recovery(self) -> bool {
        matches!(self, FileProtocol::Zmodem)
    }

    /// Check if this protocol uses streaming (minimal ACKs).
    pub const fn is_streaming(self) -> bool {
        matches!(self, FileProtocol::Zmodem | FileProtocol::YmodemG)
    }

    /// Get the typical block size for this protocol.
    pub const fn block_size(self) -> usize {
        match self {
            FileProtocol::Zmodem => 1024, // Typical, can vary
            FileProtocol::Ymodem | FileProtocol::YmodemG | FileProtocol::Xmodem1K => 1024,
            FileProtocol::XmodemCrc | FileProtocol::Xmodem => 128,
        }
    }

    /// Get the Xmodem variant for this protocol, if applicable.
    pub const fn xmodem_variant(self) -> Option<XmodemVariant> {
        match self {
            FileProtocol::Xmodem1K => Some(XmodemVariant::OneK),
            FileProtocol::XmodemCrc => Some(XmodemVariant::Crc),
            FileProtocol::Xmodem => Some(XmodemVariant::Checksum),
            _ => None,
        }
    }

    /// List all available protocols in order of capability.
    pub const fn all() -> &'static [FileProtocol] {
        &[
            FileProtocol::Zmodem,
            FileProtocol::Ymodem,
            FileProtocol::YmodemG,
            FileProtocol::Xmodem1K,
            FileProtocol::XmodemCrc,
            FileProtocol::Xmodem,
        ]
    }
}

impl Default for FileProtocol {
    /// Default protocol is Zmodem (most capable).
    fn default() -> Self {
        FileProtocol::Zmodem
    }
}

impl std::fmt::Display for FileProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Protocol preferences for a user or system.
#[derive(Debug, Clone)]
pub struct ProtocolPreferences {
    /// Preferred protocol for downloads.
    pub download: FileProtocol,
    /// Preferred protocol for uploads.
    pub upload: FileProtocol,
    /// Fallback protocols in order of preference.
    pub fallbacks: Vec<FileProtocol>,
}

impl Default for ProtocolPreferences {
    fn default() -> Self {
        Self {
            download: FileProtocol::Zmodem,
            upload: FileProtocol::Zmodem,
            fallbacks: vec![
                FileProtocol::Ymodem,
                FileProtocol::Xmodem1K,
                FileProtocol::XmodemCrc,
            ],
        }
    }
}

impl ProtocolPreferences {
    /// Create new preferences with a preferred protocol.
    pub fn new(preferred: FileProtocol) -> Self {
        Self {
            download: preferred,
            upload: preferred,
            fallbacks: FileProtocol::all()
                .iter()
                .filter(|&&p| p != preferred)
                .copied()
                .collect(),
        }
    }

    /// Set the download protocol.
    pub fn download(mut self, protocol: FileProtocol) -> Self {
        self.download = protocol;
        self
    }

    /// Set the upload protocol.
    pub fn upload(mut self, protocol: FileProtocol) -> Self {
        self.upload = protocol;
        self
    }

    /// Set fallback protocols.
    pub fn fallbacks(mut self, protocols: Vec<FileProtocol>) -> Self {
        self.fallbacks = protocols;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_names() {
        assert_eq!(FileProtocol::Zmodem.name(), "Zmodem");
        assert_eq!(FileProtocol::Ymodem.name(), "Ymodem");
        assert_eq!(FileProtocol::YmodemG.name(), "Ymodem-G");
        assert_eq!(FileProtocol::Xmodem1K.name(), "Xmodem-1K");
        assert_eq!(FileProtocol::XmodemCrc.name(), "Xmodem-CRC");
        assert_eq!(FileProtocol::Xmodem.name(), "Xmodem");
    }

    #[test]
    fn test_protocol_descriptions() {
        for protocol in FileProtocol::all() {
            let desc = protocol.description();
            assert!(!desc.is_empty());
        }
    }

    #[test]
    fn test_supports_batch() {
        assert!(!FileProtocol::Zmodem.supports_batch());
        assert!(FileProtocol::Ymodem.supports_batch());
        assert!(FileProtocol::YmodemG.supports_batch());
        assert!(!FileProtocol::Xmodem1K.supports_batch());
    }

    #[test]
    fn test_supports_crash_recovery() {
        assert!(FileProtocol::Zmodem.supports_crash_recovery());
        assert!(!FileProtocol::Ymodem.supports_crash_recovery());
        assert!(!FileProtocol::Xmodem1K.supports_crash_recovery());
    }

    #[test]
    fn test_is_streaming() {
        assert!(FileProtocol::Zmodem.is_streaming());
        assert!(!FileProtocol::Ymodem.is_streaming());
        assert!(FileProtocol::YmodemG.is_streaming());
        assert!(!FileProtocol::Xmodem1K.is_streaming());
    }

    #[test]
    fn test_block_sizes() {
        assert_eq!(FileProtocol::Zmodem.block_size(), 1024);
        assert_eq!(FileProtocol::Ymodem.block_size(), 1024);
        assert_eq!(FileProtocol::Xmodem1K.block_size(), 1024);
        assert_eq!(FileProtocol::XmodemCrc.block_size(), 128);
        assert_eq!(FileProtocol::Xmodem.block_size(), 128);
    }

    #[test]
    fn test_xmodem_variant() {
        assert_eq!(
            FileProtocol::Xmodem1K.xmodem_variant(),
            Some(XmodemVariant::OneK)
        );
        assert_eq!(
            FileProtocol::XmodemCrc.xmodem_variant(),
            Some(XmodemVariant::Crc)
        );
        assert_eq!(
            FileProtocol::Xmodem.xmodem_variant(),
            Some(XmodemVariant::Checksum)
        );
        assert_eq!(FileProtocol::Zmodem.xmodem_variant(), None);
        assert_eq!(FileProtocol::Ymodem.xmodem_variant(), None);
    }

    #[test]
    fn test_all_protocols() {
        let all = FileProtocol::all();
        assert_eq!(all.len(), 6);
        assert!(all.contains(&FileProtocol::Zmodem));
        assert!(all.contains(&FileProtocol::Ymodem));
        assert!(all.contains(&FileProtocol::Xmodem));
    }

    #[test]
    fn test_default() {
        assert_eq!(FileProtocol::default(), FileProtocol::Zmodem);
    }

    #[test]
    fn test_display() {
        assert_eq!(FileProtocol::Zmodem.to_string(), "Zmodem");
        assert_eq!(FileProtocol::Ymodem.to_string(), "Ymodem");
    }

    #[test]
    fn test_preferences_default() {
        let prefs = ProtocolPreferences::default();
        assert_eq!(prefs.download, FileProtocol::Zmodem);
        assert_eq!(prefs.upload, FileProtocol::Zmodem);
        assert!(!prefs.fallbacks.is_empty());
    }

    #[test]
    fn test_preferences_new() {
        let prefs = ProtocolPreferences::new(FileProtocol::Ymodem);
        assert_eq!(prefs.download, FileProtocol::Ymodem);
        assert_eq!(prefs.upload, FileProtocol::Ymodem);
        assert!(!prefs.fallbacks.contains(&FileProtocol::Ymodem));
    }

    #[test]
    fn test_preferences_builder() {
        let prefs = ProtocolPreferences::new(FileProtocol::Zmodem)
            .download(FileProtocol::Ymodem)
            .upload(FileProtocol::Xmodem1K)
            .fallbacks(vec![FileProtocol::XmodemCrc]);

        assert_eq!(prefs.download, FileProtocol::Ymodem);
        assert_eq!(prefs.upload, FileProtocol::Xmodem1K);
        assert_eq!(prefs.fallbacks.len(), 1);
    }

    #[test]
    fn test_protocol_eq_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(FileProtocol::Zmodem);
        set.insert(FileProtocol::Ymodem);

        assert!(set.contains(&FileProtocol::Zmodem));
        assert!(set.contains(&FileProtocol::Ymodem));
        assert!(!set.contains(&FileProtocol::Xmodem));
    }
}
