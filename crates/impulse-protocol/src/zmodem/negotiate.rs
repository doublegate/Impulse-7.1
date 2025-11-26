//! Protocol parameter negotiation for Zmodem.
//!
//! This module handles negotiation of session parameters between sender
//! and receiver, including CRC type, escape mode, and buffer size.

use super::init::ZmodemInit;

/// CRC type to use for data transfer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrcType {
    /// CRC-16/XMODEM (16-bit checksum)
    Crc16,
    /// CRC-32/ZMODEM (32-bit checksum)
    Crc32,
}

impl CrcType {
    /// Check if this is CRC-32.
    pub fn is_crc32(self) -> bool {
        matches!(self, CrcType::Crc32)
    }

    /// Check if this is CRC-16.
    pub fn is_crc16(self) -> bool {
        matches!(self, CrcType::Crc16)
    }
}

/// Escape mode for data transmission.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EscapeMode {
    /// Only escape required characters (ZDLE, XON, XOFF)
    Minimal,
    /// Escape all control characters (0x00-0x1F)
    ControlChars,
    /// Escape all high-bit characters (0x80-0xFF) in addition to control chars
    All8Bit,
}

impl EscapeMode {
    /// Check if control characters should be escaped.
    pub fn escape_control(self) -> bool {
        matches!(self, EscapeMode::ControlChars | EscapeMode::All8Bit)
    }

    /// Check if 8-bit characters should be escaped.
    pub fn escape_8bit(self) -> bool {
        matches!(self, EscapeMode::All8Bit)
    }

    /// Get the most conservative of two escape modes.
    pub fn most_conservative(self, other: Self) -> Self {
        match (self, other) {
            (EscapeMode::All8Bit, _) | (_, EscapeMode::All8Bit) => EscapeMode::All8Bit,
            (EscapeMode::ControlChars, _) | (_, EscapeMode::ControlChars) => {
                EscapeMode::ControlChars
            }
            _ => EscapeMode::Minimal,
        }
    }
}

/// Negotiated parameters for a Zmodem session.
///
/// Represents the agreed-upon parameters after sender and receiver
/// exchange their capabilities.
///
/// # Examples
///
/// ```
/// use impulse_protocol::zmodem::negotiate::{negotiate, EscapeMode, CrcType};
/// use impulse_protocol::zmodem::init::ZmodemInit;
///
/// let sender = ZmodemInit::new();
/// let receiver = ZmodemInit::conservative();
///
/// let params = negotiate(&sender, &receiver);
///
/// // Conservative receiver forces more escaping
/// assert_eq!(params.escape_mode, EscapeMode::All8Bit);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NegotiatedParams {
    /// CRC type to use for data frames
    pub crc_type: CrcType,

    /// Escape mode for data transmission
    pub escape_mode: EscapeMode,

    /// Buffer size for data transfer (minimum of sender/receiver)
    pub buffer_size: u16,

    /// Whether file resume is supported
    pub can_resume: bool,
}

impl NegotiatedParams {
    /// Create conservative negotiated parameters.
    ///
    /// Conservative:
    /// - crc_type: CRC-32 (more reliable)
    /// - escape_mode: All8Bit (maximum escaping)
    /// - buffer_size: 1024 bytes
    /// - can_resume: false (safer)
    pub fn conservative() -> Self {
        Self {
            crc_type: CrcType::Crc32,
            escape_mode: EscapeMode::All8Bit,
            buffer_size: 1024,
            can_resume: false,
        }
    }
}

impl Default for NegotiatedParams {
    fn default() -> Self {
        Self {
            crc_type: CrcType::Crc32,
            escape_mode: EscapeMode::Minimal,
            buffer_size: 8192,
            can_resume: true,
        }
    }
}

/// Negotiate session parameters between sender and receiver.
///
/// Takes the capabilities of both sender and receiver and determines
/// the optimal parameters that both sides can support.
///
/// Negotiation rules:
/// - CRC: Use CRC-32 if both support it, otherwise CRC-16
/// - Escape: Use the most conservative escape mode requested
/// - Buffer: Use the smaller of the two buffer sizes
/// - Resume: Supported only if both sides agree
///
/// # Arguments
///
/// * `sender` - Sender's initialization parameters
/// * `receiver` - Receiver's initialization parameters
///
/// # Returns
///
/// Negotiated parameters that both sides can use
///
/// # Examples
///
/// ```
/// use impulse_protocol::zmodem::negotiate::negotiate;
/// use impulse_protocol::zmodem::init::ZmodemInit;
///
/// let sender = ZmodemInit::aggressive();
/// let receiver = ZmodemInit::conservative();
///
/// let params = negotiate(&sender, &receiver);
///
/// // Conservative settings win
/// assert!(params.buffer_size <= 1024);
/// ```
pub fn negotiate(sender: &ZmodemInit, receiver: &ZmodemInit) -> NegotiatedParams {
    // CRC negotiation: Use CRC-32 if both support it
    let crc_type = if sender.use_crc32 && receiver.use_crc32 {
        CrcType::Crc32
    } else {
        CrcType::Crc16
    };

    // Escape mode negotiation: Use the most conservative setting
    let sender_escape = if sender.escape_8bit {
        EscapeMode::All8Bit
    } else if sender.escape_ctrl {
        EscapeMode::ControlChars
    } else {
        EscapeMode::Minimal
    };

    let receiver_escape = if receiver.escape_8bit {
        EscapeMode::All8Bit
    } else if receiver.escape_ctrl {
        EscapeMode::ControlChars
    } else {
        EscapeMode::Minimal
    };

    let escape_mode = sender_escape.most_conservative(receiver_escape);

    // Buffer size: Use the minimum to avoid overflowing receiver's buffer
    let buffer_size = sender.buffer_size.min(receiver.buffer_size);

    // Resume capability: Both must support overlapped I/O and full duplex
    let can_resume = sender.can_full_duplex()
        && receiver.can_full_duplex()
        && sender.can_overlap_io()
        && receiver.can_overlap_io();

    NegotiatedParams {
        crc_type,
        escape_mode,
        buffer_size,
        can_resume,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc_type() {
        assert!(CrcType::Crc32.is_crc32());
        assert!(!CrcType::Crc32.is_crc16());
        assert!(CrcType::Crc16.is_crc16());
        assert!(!CrcType::Crc16.is_crc32());
    }

    #[test]
    fn test_escape_mode_control() {
        assert!(!EscapeMode::Minimal.escape_control());
        assert!(EscapeMode::ControlChars.escape_control());
        assert!(EscapeMode::All8Bit.escape_control());
    }

    #[test]
    fn test_escape_mode_8bit() {
        assert!(!EscapeMode::Minimal.escape_8bit());
        assert!(!EscapeMode::ControlChars.escape_8bit());
        assert!(EscapeMode::All8Bit.escape_8bit());
    }

    #[test]
    fn test_escape_mode_most_conservative() {
        assert_eq!(
            EscapeMode::Minimal.most_conservative(EscapeMode::Minimal),
            EscapeMode::Minimal
        );

        assert_eq!(
            EscapeMode::Minimal.most_conservative(EscapeMode::ControlChars),
            EscapeMode::ControlChars
        );

        assert_eq!(
            EscapeMode::ControlChars.most_conservative(EscapeMode::All8Bit),
            EscapeMode::All8Bit
        );

        assert_eq!(
            EscapeMode::All8Bit.most_conservative(EscapeMode::Minimal),
            EscapeMode::All8Bit
        );
    }

    #[test]
    fn test_negotiated_params_default() {
        let params = NegotiatedParams::default();
        assert_eq!(params.crc_type, CrcType::Crc32);
        assert_eq!(params.escape_mode, EscapeMode::Minimal);
        assert_eq!(params.buffer_size, 8192);
        assert!(params.can_resume);
    }

    #[test]
    fn test_negotiated_params_conservative() {
        let params = NegotiatedParams::conservative();
        assert_eq!(params.crc_type, CrcType::Crc32);
        assert_eq!(params.escape_mode, EscapeMode::All8Bit);
        assert_eq!(params.buffer_size, 1024);
        assert!(!params.can_resume);
    }

    #[test]
    fn test_negotiate_both_crc32() {
        let sender = ZmodemInit {
            use_crc32: true,
            escape_ctrl: false,
            escape_8bit: false,
            buffer_size: 8192,
        };

        let receiver = ZmodemInit {
            use_crc32: true,
            escape_ctrl: false,
            escape_8bit: false,
            buffer_size: 8192,
        };

        let params = negotiate(&sender, &receiver);
        assert_eq!(params.crc_type, CrcType::Crc32);
    }

    #[test]
    fn test_negotiate_one_crc16() {
        let sender = ZmodemInit {
            use_crc32: true,
            escape_ctrl: false,
            escape_8bit: false,
            buffer_size: 8192,
        };

        let receiver = ZmodemInit {
            use_crc32: false,
            escape_ctrl: false,
            escape_8bit: false,
            buffer_size: 8192,
        };

        let params = negotiate(&sender, &receiver);
        assert_eq!(params.crc_type, CrcType::Crc16);
    }

    #[test]
    fn test_negotiate_escape_minimal() {
        let sender = ZmodemInit {
            use_crc32: true,
            escape_ctrl: false,
            escape_8bit: false,
            buffer_size: 8192,
        };

        let receiver = ZmodemInit {
            use_crc32: true,
            escape_ctrl: false,
            escape_8bit: false,
            buffer_size: 8192,
        };

        let params = negotiate(&sender, &receiver);
        assert_eq!(params.escape_mode, EscapeMode::Minimal);
    }

    #[test]
    fn test_negotiate_escape_control() {
        let sender = ZmodemInit {
            use_crc32: true,
            escape_ctrl: true,
            escape_8bit: false,
            buffer_size: 8192,
        };

        let receiver = ZmodemInit {
            use_crc32: true,
            escape_ctrl: false,
            escape_8bit: false,
            buffer_size: 8192,
        };

        let params = negotiate(&sender, &receiver);
        assert_eq!(params.escape_mode, EscapeMode::ControlChars);
    }

    #[test]
    fn test_negotiate_escape_8bit() {
        let sender = ZmodemInit {
            use_crc32: true,
            escape_ctrl: false,
            escape_8bit: true,
            buffer_size: 8192,
        };

        let receiver = ZmodemInit {
            use_crc32: true,
            escape_ctrl: false,
            escape_8bit: false,
            buffer_size: 8192,
        };

        let params = negotiate(&sender, &receiver);
        assert_eq!(params.escape_mode, EscapeMode::All8Bit);
    }

    #[test]
    fn test_negotiate_buffer_size_minimum() {
        let sender = ZmodemInit {
            use_crc32: true,
            escape_ctrl: false,
            escape_8bit: false,
            buffer_size: 16384,
        };

        let receiver = ZmodemInit {
            use_crc32: true,
            escape_ctrl: false,
            escape_8bit: false,
            buffer_size: 4096,
        };

        let params = negotiate(&sender, &receiver);
        assert_eq!(params.buffer_size, 4096);
    }

    #[test]
    fn test_negotiate_buffer_size_equal() {
        let sender = ZmodemInit {
            use_crc32: true,
            escape_ctrl: false,
            escape_8bit: false,
            buffer_size: 8192,
        };

        let receiver = ZmodemInit {
            use_crc32: true,
            escape_ctrl: false,
            escape_8bit: false,
            buffer_size: 8192,
        };

        let params = negotiate(&sender, &receiver);
        assert_eq!(params.buffer_size, 8192);
    }

    #[test]
    fn test_negotiate_can_resume() {
        let sender = ZmodemInit::new();
        let receiver = ZmodemInit::new();

        let params = negotiate(&sender, &receiver);
        assert!(params.can_resume);
    }

    #[test]
    fn test_negotiate_aggressive_vs_conservative() {
        let sender = ZmodemInit::aggressive();
        let receiver = ZmodemInit::conservative();

        let params = negotiate(&sender, &receiver);

        // Conservative wins for escape mode
        assert_eq!(params.escape_mode, EscapeMode::All8Bit);

        // Smaller buffer wins
        assert_eq!(params.buffer_size, 1024);

        // Both support CRC-32
        assert_eq!(params.crc_type, CrcType::Crc32);

        // Both support resume
        assert!(params.can_resume);
    }

    #[test]
    fn test_negotiate_all_combinations() {
        let configs = [
            ZmodemInit::new(),
            ZmodemInit::conservative(),
            ZmodemInit::aggressive(),
        ];

        for sender in &configs {
            for receiver in &configs {
                let params = negotiate(sender, receiver);

                // Buffer size should be minimum
                assert!(params.buffer_size <= sender.buffer_size);
                assert!(params.buffer_size <= receiver.buffer_size);

                // CRC-32 only if both support
                if params.crc_type == CrcType::Crc32 {
                    assert!(sender.use_crc32 && receiver.use_crc32);
                }
            }
        }
    }
}
