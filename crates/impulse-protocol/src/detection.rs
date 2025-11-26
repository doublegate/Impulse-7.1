//! Protocol auto-detection.
//!
//! This module provides functionality to detect which file transfer protocol
//! a client is trying to use based on the initial bytes received.
//!
//! # Supported Protocols
//!
//! - **Zmodem**: Detected by ZRQINIT frame pattern
//! - **Ymodem/Ymodem-G**: Detected by 'G' or NAK/'C' receiver handshake
//! - **Xmodem**: Detected by SOH/STX sender blocks or NAK/'C' receiver handshake
//!
//! # Detection Strategy
//!
//! The detector examines the first few bytes of the stream to identify patterns:
//!
//! 1. **Zmodem**: Looks for `**\x18B` (ZRQINIT header in hex encoding)
//! 2. **Ymodem-G**: Looks for 'G' (0x47) byte
//! 3. **Ymodem**: Looks for 'C' (CRC mode request) or NAK (0x15)
//! 4. **Xmodem**: Looks for SOH (0x01) or STX (0x02) header bytes
//!
//! # Examples
//!
//! ```no_run
//! use impulse_protocol::detection::{ProtocolDetector, DetectedProtocol};
//! use tokio::net::TcpStream;
//!
//! # async fn example() -> std::io::Result<()> {
//! let mut stream = TcpStream::connect("localhost:2323").await?;
//! let detector = ProtocolDetector::new(5000); // 5 second timeout
//!
//! match detector.detect(&mut stream).await {
//!     DetectedProtocol::Zmodem => println!("Client wants Zmodem"),
//!     DetectedProtocol::Ymodem => println!("Client wants Ymodem"),
//!     DetectedProtocol::YmodemG => println!("Client wants Ymodem-G"),
//!     DetectedProtocol::Xmodem => println!("Client wants Xmodem"),
//!     DetectedProtocol::Unknown => println!("Unknown protocol"),
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Detecting from bytes without consuming
//!
//! ```
//! use impulse_protocol::detection::{ProtocolDetector, DetectedProtocol};
//!
//! let bytes = b"**\x18B0100000023be50\r\x8a\x11";
//! let protocol = ProtocolDetector::detect_from_bytes(bytes);
//! assert_eq!(protocol, DetectedProtocol::Zmodem);
//! ```

use crate::xmodem::{NAK, SOH, STX};
use crate::ymodem::streaming::YMODEM_G;
use std::time::Duration;
use tokio::io::{AsyncRead, AsyncReadExt};
use tokio::time::timeout;

/// Detected file transfer protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DetectedProtocol {
    /// Zmodem protocol detected.
    Zmodem,
    /// Ymodem protocol detected (standard, with per-block ACK).
    Ymodem,
    /// Ymodem-G protocol detected (streaming, no per-block ACK).
    YmodemG,
    /// Xmodem protocol detected (any variant).
    Xmodem,
    /// Protocol could not be determined.
    Unknown,
}

impl DetectedProtocol {
    /// Get the name of the detected protocol.
    pub const fn name(self) -> &'static str {
        match self {
            DetectedProtocol::Zmodem => "Zmodem",
            DetectedProtocol::Ymodem => "Ymodem",
            DetectedProtocol::YmodemG => "Ymodem-G",
            DetectedProtocol::Xmodem => "Xmodem",
            DetectedProtocol::Unknown => "Unknown",
        }
    }

    /// Check if a valid protocol was detected.
    pub const fn is_known(self) -> bool {
        !matches!(self, DetectedProtocol::Unknown)
    }
}

impl std::fmt::Display for DetectedProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Protocol detector.
///
/// Examines incoming bytes to determine which file transfer protocol
/// a client is attempting to use.
pub struct ProtocolDetector {
    /// Timeout for receiving initial bytes (in milliseconds).
    timeout_ms: u64,
}

impl ProtocolDetector {
    /// Create a new protocol detector with the specified timeout.
    ///
    /// # Arguments
    ///
    /// * `timeout_ms` - Timeout in milliseconds for receiving initial bytes
    pub fn new(timeout_ms: u64) -> Self {
        Self { timeout_ms }
    }

    /// Detect the protocol by peeking at incoming bytes from a stream.
    ///
    /// This method reads bytes from the stream to detect the protocol.
    /// The bytes are consumed and cannot be read again.
    ///
    /// # Arguments
    ///
    /// * `stream` - The stream to read from
    ///
    /// # Returns
    ///
    /// The detected protocol, or `Unknown` if no protocol could be identified.
    pub async fn detect<S: AsyncRead + Unpin>(&self, stream: &mut S) -> DetectedProtocol {
        // Try to read first bytes with timeout
        let mut buffer = vec![0u8; 32]; // Read up to 32 bytes for detection
        let timeout_duration = Duration::from_millis(self.timeout_ms);

        match timeout(timeout_duration, stream.read(&mut buffer)).await {
            Ok(Ok(n)) if n > 0 => {
                let bytes = &buffer[..n];
                Self::detect_from_bytes(bytes)
            }
            _ => DetectedProtocol::Unknown,
        }
    }

    /// Detect the protocol from a byte slice without consuming it.
    ///
    /// This method examines the bytes to identify protocol patterns.
    ///
    /// # Arguments
    ///
    /// * `bytes` - The bytes to examine
    ///
    /// # Returns
    ///
    /// The detected protocol, or `Unknown` if no protocol could be identified.
    pub fn detect_from_bytes(bytes: &[u8]) -> DetectedProtocol {
        if bytes.is_empty() {
            return DetectedProtocol::Unknown;
        }

        // Check for Zmodem ZRQINIT pattern: "**\x18B"
        // ZRQINIT in hex encoding starts with ** (0x2A 0x2A) followed by ZDLE (0x18) and 'B'
        if Self::is_zmodem_pattern(bytes) {
            return DetectedProtocol::Zmodem;
        }

        // Check for Ymodem-G: 'G' character
        if bytes[0] == YMODEM_G {
            return DetectedProtocol::YmodemG;
        }

        // Check for Ymodem: 'C' (CRC mode request) or NAK
        // Note: NAK can also be Xmodem, but 'C' is more specific to Ymodem/Xmodem-CRC
        if bytes[0] == b'C' {
            // 'C' indicates CRC mode, which is used by both Ymodem and Xmodem-CRC
            // We'll assume Ymodem since it's more capable
            return DetectedProtocol::Ymodem;
        }

        // Check for Xmodem sender: SOH (128-byte) or STX (1024-byte)
        if bytes[0] == SOH || bytes[0] == STX {
            return DetectedProtocol::Xmodem;
        }

        // Check for Xmodem receiver: NAK (indicates receiver ready for checksum mode)
        if bytes[0] == NAK {
            // NAK indicates receiver ready, likely Xmodem (original with checksum)
            // Could also be Ymodem falling back, but we'll assume Xmodem
            return DetectedProtocol::Xmodem;
        }

        DetectedProtocol::Unknown
    }

    /// Check if bytes match Zmodem ZRQINIT pattern.
    fn is_zmodem_pattern(bytes: &[u8]) -> bool {
        // Zmodem ZRQINIT in hex encoding: "**\x18B"
        // Full pattern: ** (0x2A 0x2A) + ZDLE (0x18) + 'B' (0x42)
        if bytes.len() >= 4
            && bytes[0] == 0x2A
            && bytes[1] == 0x2A
            && bytes[2] == 0x18
            && bytes[3] == 0x42
        {
            return true;
        }

        // Alternative: Check for ZDLE (0x18) followed by various Zmodem frame types
        // This catches cases where ** prefix might be missing
        if bytes.len() >= 2 && bytes[0] == 0x18 {
            // Common Zmodem frame type indicators after ZDLE
            // 'A' (ZRQINIT), 'B' (ZRINIT), 'C' (ZSINIT), etc.
            if bytes[1] >= 0x41 && bytes[1] <= 0x5A {
                return true;
            }
        }

        false
    }
}

impl Default for ProtocolDetector {
    /// Create a detector with default 10-second timeout.
    fn default() -> Self {
        Self::new(10000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detected_protocol_name() {
        assert_eq!(DetectedProtocol::Zmodem.name(), "Zmodem");
        assert_eq!(DetectedProtocol::Ymodem.name(), "Ymodem");
        assert_eq!(DetectedProtocol::YmodemG.name(), "Ymodem-G");
        assert_eq!(DetectedProtocol::Xmodem.name(), "Xmodem");
        assert_eq!(DetectedProtocol::Unknown.name(), "Unknown");
    }

    #[test]
    fn test_detected_protocol_is_known() {
        assert!(DetectedProtocol::Zmodem.is_known());
        assert!(DetectedProtocol::Ymodem.is_known());
        assert!(DetectedProtocol::YmodemG.is_known());
        assert!(DetectedProtocol::Xmodem.is_known());
        assert!(!DetectedProtocol::Unknown.is_known());
    }

    #[test]
    fn test_detected_protocol_display() {
        assert_eq!(DetectedProtocol::Zmodem.to_string(), "Zmodem");
        assert_eq!(DetectedProtocol::Unknown.to_string(), "Unknown");
    }

    #[test]
    fn test_protocol_detector_new() {
        let detector = ProtocolDetector::new(5000);
        assert_eq!(detector.timeout_ms, 5000);
    }

    #[test]
    fn test_protocol_detector_default() {
        let detector = ProtocolDetector::default();
        assert_eq!(detector.timeout_ms, 10000);
    }

    #[test]
    fn test_detect_zmodem_full_pattern() {
        let bytes = b"**\x18B0100000023be50\r\x8a\x11";
        let protocol = ProtocolDetector::detect_from_bytes(bytes);
        assert_eq!(protocol, DetectedProtocol::Zmodem);
    }

    #[test]
    fn test_detect_zmodem_zdle_pattern() {
        let bytes = b"\x18B0100000023be50";
        let protocol = ProtocolDetector::detect_from_bytes(bytes);
        assert_eq!(protocol, DetectedProtocol::Zmodem);
    }

    #[test]
    fn test_detect_zmodem_zdle_other_frame() {
        let bytes = b"\x18A0100000023be50";
        let protocol = ProtocolDetector::detect_from_bytes(bytes);
        assert_eq!(protocol, DetectedProtocol::Zmodem);
    }

    #[test]
    fn test_detect_ymodem_g() {
        let bytes = b"G";
        let protocol = ProtocolDetector::detect_from_bytes(bytes);
        assert_eq!(protocol, DetectedProtocol::YmodemG);
    }

    #[test]
    fn test_detect_ymodem_crc_mode() {
        let bytes = b"C";
        let protocol = ProtocolDetector::detect_from_bytes(bytes);
        assert_eq!(protocol, DetectedProtocol::Ymodem);
    }

    #[test]
    fn test_detect_xmodem_soh() {
        let bytes = b"\x01\x01\xFE...";
        let protocol = ProtocolDetector::detect_from_bytes(bytes);
        assert_eq!(protocol, DetectedProtocol::Xmodem);
    }

    #[test]
    fn test_detect_xmodem_stx() {
        let bytes = b"\x02\x01\xFE...";
        let protocol = ProtocolDetector::detect_from_bytes(bytes);
        assert_eq!(protocol, DetectedProtocol::Xmodem);
    }

    #[test]
    fn test_detect_xmodem_nak() {
        let bytes = b"\x15";
        let protocol = ProtocolDetector::detect_from_bytes(bytes);
        assert_eq!(protocol, DetectedProtocol::Xmodem);
    }

    #[test]
    fn test_detect_unknown_empty() {
        let bytes = b"";
        let protocol = ProtocolDetector::detect_from_bytes(bytes);
        assert_eq!(protocol, DetectedProtocol::Unknown);
    }

    #[test]
    fn test_detect_unknown_garbage() {
        let bytes = b"INVALID DATA";
        let protocol = ProtocolDetector::detect_from_bytes(bytes);
        assert_eq!(protocol, DetectedProtocol::Unknown);
    }

    #[test]
    fn test_is_zmodem_pattern_full() {
        assert!(ProtocolDetector::is_zmodem_pattern(b"**\x18B0100000023"));
    }

    #[test]
    fn test_is_zmodem_pattern_zdle_only() {
        assert!(ProtocolDetector::is_zmodem_pattern(b"\x18A"));
        assert!(ProtocolDetector::is_zmodem_pattern(b"\x18B"));
        assert!(ProtocolDetector::is_zmodem_pattern(b"\x18Z"));
    }

    #[test]
    fn test_is_zmodem_pattern_not() {
        assert!(!ProtocolDetector::is_zmodem_pattern(b"NOTZMDEM"));
        assert!(!ProtocolDetector::is_zmodem_pattern(b"\x18\x01"));
        assert!(!ProtocolDetector::is_zmodem_pattern(b"**AB"));
    }

    #[tokio::test]
    async fn test_detect_from_stream_zmodem() {
        let data = b"**\x18B0100000023be50";
        let mut stream = &data[..];
        let detector = ProtocolDetector::default();

        let protocol = detector.detect(&mut stream).await;
        assert_eq!(protocol, DetectedProtocol::Zmodem);
    }

    #[tokio::test]
    async fn test_detect_from_stream_ymodem_g() {
        let data = b"G";
        let mut stream = &data[..];
        let detector = ProtocolDetector::default();

        let protocol = detector.detect(&mut stream).await;
        assert_eq!(protocol, DetectedProtocol::YmodemG);
    }

    #[tokio::test]
    async fn test_detect_from_stream_timeout() {
        let data = b"";
        let mut stream = &data[..];
        let detector = ProtocolDetector::new(100); // 100ms timeout

        let protocol = detector.detect(&mut stream).await;
        assert_eq!(protocol, DetectedProtocol::Unknown);
    }

    #[test]
    fn test_protocol_equality() {
        assert_eq!(DetectedProtocol::Zmodem, DetectedProtocol::Zmodem);
        assert_ne!(DetectedProtocol::Zmodem, DetectedProtocol::Ymodem);
    }

    #[test]
    fn test_protocol_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(DetectedProtocol::Zmodem);
        set.insert(DetectedProtocol::Ymodem);

        assert!(set.contains(&DetectedProtocol::Zmodem));
        assert!(set.contains(&DetectedProtocol::Ymodem));
        assert!(!set.contains(&DetectedProtocol::Xmodem));
    }
}
