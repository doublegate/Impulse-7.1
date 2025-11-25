//! JAM format header parsing
//!
//! JAM (Joaquim-Andrew-Mats) is a message base format widely used in FidoNet and BBS systems.
//! It consists of three files:
//! - .JHR - Header file containing message headers
//! - .JDT - Data file containing message text
//! - .JDX - Index file for quick access

use binrw::{BinRead, binread};
use chrono::{DateTime, TimeZone, Utc};
use std::io::{Read, Seek};

/// JAM base header (stored in .JHR file)
#[binread]
#[derive(Debug, Clone)]
#[br(little)]
pub struct JamBaseHeader {
    /// Signature ("JAM\0")
    #[br(count = 4)]
    pub signature: Vec<u8>,

    /// Date created (Unix timestamp)
    pub created: u32,

    /// Date modified (Unix timestamp)
    pub modified: u32,

    /// Number of active messages
    pub active: u32,

    /// Password CRC (0 if no password)
    pub password_crc: u32,

    /// Base message number (lowest message number)
    pub base_msg_num: u32,
}

impl JamBaseHeader {
    /// Validate the JAM signature
    pub fn is_valid(&self) -> bool {
        self.signature == b"JAM\0"
    }

    /// Get created date as DateTime
    pub fn created_date(&self) -> Option<DateTime<Utc>> {
        Utc.timestamp_opt(self.created as i64, 0).single()
    }

    /// Get modified date as DateTime
    pub fn modified_date(&self) -> Option<DateTime<Utc>> {
        Utc.timestamp_opt(self.modified as i64, 0).single()
    }
}

/// JAM message header (stored in .JHR file, one per message)
#[binread]
#[derive(Debug, Clone)]
#[br(little)]
pub struct JamMessageHeader {
    /// Signature ("JAM\0")
    #[br(count = 4)]
    pub signature: Vec<u8>,

    /// Revision (1)
    pub revision: u16,

    /// Reserved (0)
    pub reserved: u16,

    /// Subfield length (total length of subfields)
    pub subfield_len: u32,

    /// Times read
    pub times_read: u32,

    /// Message ID CRC
    pub msg_id_crc: u32,

    /// Reply ID CRC
    pub reply_id_crc: u32,

    /// Reply to message number
    pub reply_to: u32,

    /// First reply message number
    pub reply_1st: u32,

    /// Next reply message number
    pub reply_next: u32,

    /// Date written (Unix timestamp)
    pub date_written: u32,

    /// Date received (Unix timestamp)
    pub date_received: u32,

    /// Date processed (Unix timestamp)
    pub date_processed: u32,

    /// Message number
    pub msg_num: u32,

    /// Message attributes
    pub attribute: u32,

    /// Message attributes 2
    pub attribute2: u32,

    /// Offset in .JDT file
    pub offset: u32,

    /// Text length in .JDT file
    pub text_len: u32,
}

/// JAM message subfield types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum SubfieldType {
    /// Sender name
    SendName = 0,
    /// Recipient name
    RecvName = 1,
    /// Sender address
    SendAddr = 2,
    /// Recipient address
    RecvAddr = 3,
    /// Message ID
    MsgId = 4,
    /// Reply ID
    ReplyId = 5,
    /// Subject
    Subject = 6,
    /// Path
    Path = 7,
    /// Seen-by
    SeenBy = 8,
    /// Unknown/other
    Unknown = 0xFFFF,
}

impl From<u16> for SubfieldType {
    fn from(value: u16) -> Self {
        match value {
            0 => SubfieldType::SendName,
            1 => SubfieldType::RecvName,
            2 => SubfieldType::SendAddr,
            3 => SubfieldType::RecvAddr,
            4 => SubfieldType::MsgId,
            5 => SubfieldType::ReplyId,
            6 => SubfieldType::Subject,
            7 => SubfieldType::Path,
            8 => SubfieldType::SeenBy,
            _ => SubfieldType::Unknown,
        }
    }
}

/// JAM message subfield
#[binread]
#[derive(Debug, Clone)]
#[br(little)]
pub struct JamSubfield {
    /// Low ID
    pub lo_id: u16,

    /// High ID (always 0)
    pub hi_id: u16,

    /// Data length
    pub datlen: u32,

    /// Data (read separately based on datlen)
    #[br(count = datlen)]
    pub data: Vec<u8>,
}

impl JamSubfield {
    /// Get subfield type
    pub fn subfield_type(&self) -> SubfieldType {
        SubfieldType::from(self.lo_id)
    }

    /// Get subfield data as string (lossy UTF-8 conversion)
    pub fn as_string(&self) -> String {
        String::from_utf8_lossy(&self.data)
            .trim_end_matches('\0')
            .to_string()
    }
}

/// Message attributes bitflags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MessageAttributes(pub u32);

impl MessageAttributes {
    /// Message is local (not from network)
    pub const LOCAL: u32 = 0x0001;
    /// Message is in transit
    pub const IN_TRANSIT: u32 = 0x0002;
    /// Message is private
    pub const PRIVATE: u32 = 0x0004;
    /// Message has been read
    pub const READ: u32 = 0x0008;
    /// Message has been sent
    pub const SENT: u32 = 0x0010;
    /// Kill message after sending
    pub const KILL_SENT: u32 = 0x0020;
    /// Archive message after sending
    pub const ARCHIVE_SENT: u32 = 0x0040;
    /// Hold message for later sending
    pub const HOLD: u32 = 0x0080;
    /// Send message immediately (crash mail)
    pub const CRASH: u32 = 0x0100;
    /// Send immediately with highest priority
    pub const IMMEDIATE: u32 = 0x0200;
    /// Send directly without routing
    pub const DIRECT: u32 = 0x0400;
    /// Route through gateway
    pub const GATE: u32 = 0x0800;
    /// Message is a file request
    pub const FILE_REQUEST: u32 = 0x1000;
    /// Message has file attached
    pub const FILE_ATTACH: u32 = 0x2000;
    /// Truncate file after sending
    pub const TRUNCATE_FILE: u32 = 0x4000;
    /// Delete file after sending
    pub const KILL_FILE: u32 = 0x8000;

    /// Create new attributes
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    /// Check if attribute is set
    pub fn has(&self, flag: u32) -> bool {
        (self.0 & flag) != 0
    }

    /// Is message private
    pub fn is_private(&self) -> bool {
        self.has(Self::PRIVATE)
    }

    /// Is message read
    pub fn is_read(&self) -> bool {
        self.has(Self::READ)
    }

    /// Is message local
    pub fn is_local(&self) -> bool {
        self.has(Self::LOCAL)
    }
}

impl JamMessageHeader {
    /// Validate the JAM message signature
    pub fn is_valid(&self) -> bool {
        self.signature == b"JAM\0"
    }

    /// Get written date as DateTime
    pub fn written_date(&self) -> Option<DateTime<Utc>> {
        Utc.timestamp_opt(self.date_written as i64, 0).single()
    }

    /// Get received date as DateTime
    pub fn received_date(&self) -> Option<DateTime<Utc>> {
        Utc.timestamp_opt(self.date_received as i64, 0).single()
    }

    /// Get message attributes
    pub fn attributes(&self) -> MessageAttributes {
        MessageAttributes::new(self.attribute)
    }

    /// Check if this is a reply
    pub fn is_reply(&self) -> bool {
        self.reply_to != 0
    }

    /// Check if this has replies
    pub fn has_replies(&self) -> bool {
        self.reply_1st != 0
    }

    /// Read subfields from reader
    pub fn read_subfields<R: Read + Seek>(
        reader: &mut R,
    ) -> Result<Vec<JamSubfield>, binrw::Error> {
        let mut subfields = Vec::new();

        // Read subfields until we hit the text or run out
        while let Ok(subfield) = JamSubfield::read(reader) {
            subfields.push(subfield);
        }

        Ok(subfields)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_attributes() {
        let attrs = MessageAttributes::new(
            MessageAttributes::PRIVATE | MessageAttributes::READ | MessageAttributes::LOCAL,
        );

        assert!(attrs.is_private());
        assert!(attrs.is_read());
        assert!(attrs.is_local());
        assert!(!attrs.has(MessageAttributes::SENT));
    }

    #[test]
    fn test_subfield_type_conversion() {
        assert_eq!(SubfieldType::from(0), SubfieldType::SendName);
        assert_eq!(SubfieldType::from(1), SubfieldType::RecvName);
        assert_eq!(SubfieldType::from(6), SubfieldType::Subject);
        assert_eq!(SubfieldType::from(99), SubfieldType::Unknown);
    }
}
