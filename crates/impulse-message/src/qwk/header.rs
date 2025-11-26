//! QWK message header structures
//!
//! QWK headers are 128-byte blocks containing message metadata.

use binrw::binrw;
use chrono::{NaiveDate, NaiveDateTime};

/// QWK message status flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageStatus {
    /// Public message
    Public = b' ' as isize,
    /// Private message
    Private = b'*' as isize,
    /// Unread private message
    UnreadPrivate = b'+' as isize,
    /// Comment to sysop
    CommentToSysop = b'~' as isize,
    /// Password protected
    PasswordProtected = b'%' as isize,
    /// Group message
    Group = b'!' as isize,
}

impl MessageStatus {
    /// Convert from byte to MessageStatus
    pub fn from_byte(b: u8) -> Option<Self> {
        match b {
            b' ' => Some(Self::Public),
            b'*' => Some(Self::Private),
            b'+' => Some(Self::UnreadPrivate),
            b'~' => Some(Self::CommentToSysop),
            b'%' => Some(Self::PasswordProtected),
            b'!' => Some(Self::Group),
            _ => None,
        }
    }

    /// Convert to byte
    pub fn to_byte(self) -> u8 {
        self as u8
    }
}

/// QWK message header (128 bytes)
///
/// The QWK message header format:
/// - Bytes 0: Message status (space = public, * = private, etc.)
/// - Bytes 1-7: Message number (ASCII)
/// - Bytes 8-20: Date (MM-DD-YY)
/// - Bytes 21-33: Time (HH:MM)
/// - Bytes 34-58: To (25 bytes)
/// - Bytes 59-83: From (25 bytes)
/// - Bytes 84-108: Subject (25 bytes)
/// - Bytes 109-116: Password (8 bytes, usually spaces)
/// - Bytes 117-120: Reply to message number (ASCII)
/// - Bytes 121-126: Number of 128-byte blocks (ASCII, 6 bytes)
/// - Byte 127: Active flag (usually 0xE1 or 0x20)
#[binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct QwkMessageHeader {
    /// Message status flag
    pub status: u8,

    /// Message number (7 ASCII digits)
    #[br(count = 7)]
    pub message_number: Vec<u8>,

    /// Date (MM-DD-YY, 13 bytes)
    #[br(count = 13)]
    pub date: Vec<u8>,

    /// Time (HH:MM, 13 bytes)
    #[br(count = 13)]
    pub time: Vec<u8>,

    /// To field (25 bytes, space-padded)
    #[br(count = 25)]
    pub to: Vec<u8>,

    /// From field (25 bytes, space-padded)
    #[br(count = 25)]
    pub from: Vec<u8>,

    /// Subject (25 bytes, space-padded)
    #[br(count = 25)]
    pub subject: Vec<u8>,

    /// Password (8 bytes, usually spaces)
    #[br(count = 8)]
    pub password: Vec<u8>,

    /// Reply to message number (4 ASCII digits)
    #[br(count = 4)]
    pub reply_to: Vec<u8>,

    /// Number of 128-byte blocks (6 ASCII digits)
    #[br(count = 6)]
    pub num_blocks: Vec<u8>,

    /// Active flag (0xE1 or 0x20)
    pub active: u8,
}

impl QwkMessageHeader {
    /// Create a new QWK message header with default values
    pub fn new() -> Self {
        Self {
            status: b' ',
            message_number: vec![b'0'; 7],
            date: vec![b' '; 13],
            time: vec![b' '; 13],
            to: vec![b' '; 25],
            from: vec![b' '; 25],
            subject: vec![b' '; 25],
            password: vec![b' '; 8],
            reply_to: vec![b' '; 4],
            num_blocks: vec![b'0'; 6],
            active: 0xE1,
        }
    }

    /// Set message status
    pub fn with_status(mut self, status: MessageStatus) -> Self {
        self.status = status.to_byte();
        self
    }

    /// Set message number
    pub fn with_message_number(mut self, num: u32) -> Self {
        let num_str = format!("{:07}", num);
        self.message_number = num_str.bytes().take(7).collect();
        self
    }

    /// Set date and time from NaiveDateTime
    pub fn with_datetime(mut self, dt: NaiveDateTime) -> Self {
        let date_str = format!("{}", dt.format("%m-%d-%y"));
        let time_str = format!("{}", dt.format("%H:%M"));

        self.date = Self::pad_field(date_str.as_bytes(), 13);
        self.time = Self::pad_field(time_str.as_bytes(), 13);
        self
    }

    /// Set the "to" field
    pub fn with_to(mut self, to: &str) -> Self {
        self.to = Self::pad_field(to.as_bytes(), 25);
        self
    }

    /// Set the "from" field
    pub fn with_from(mut self, from: &str) -> Self {
        self.from = Self::pad_field(from.as_bytes(), 25);
        self
    }

    /// Set the subject
    pub fn with_subject(mut self, subject: &str) -> Self {
        self.subject = Self::pad_field(subject.as_bytes(), 25);
        self
    }

    /// Set reply to message number
    pub fn with_reply_to(mut self, reply_to: u32) -> Self {
        if reply_to > 0 {
            let reply_str = format!("{:04}", reply_to);
            self.reply_to = reply_str.bytes().take(4).collect();
        }
        self
    }

    /// Set number of blocks
    pub fn with_num_blocks(mut self, blocks: u32) -> Self {
        let blocks_str = format!("{:06}", blocks);
        self.num_blocks = blocks_str.bytes().take(6).collect();
        self
    }

    /// Get message number as u32
    pub fn get_message_number(&self) -> Option<u32> {
        String::from_utf8_lossy(&self.message_number)
            .trim()
            .parse()
            .ok()
    }

    /// Get "to" field as string
    pub fn get_to(&self) -> String {
        String::from_utf8_lossy(&self.to).trim().to_string()
    }

    /// Get "from" field as string
    pub fn get_from(&self) -> String {
        String::from_utf8_lossy(&self.from).trim().to_string()
    }

    /// Get subject as string
    pub fn get_subject(&self) -> String {
        String::from_utf8_lossy(&self.subject).trim().to_string()
    }

    /// Get reply to message number as u32
    pub fn get_reply_to(&self) -> Option<u32> {
        String::from_utf8_lossy(&self.reply_to).trim().parse().ok()
    }

    /// Get number of blocks as u32
    pub fn get_num_blocks(&self) -> Option<u32> {
        String::from_utf8_lossy(&self.num_blocks)
            .trim()
            .parse()
            .ok()
    }

    /// Get date and time as NaiveDateTime
    pub fn get_datetime(&self) -> Option<NaiveDateTime> {
        let date_str = String::from_utf8_lossy(&self.date).trim().to_string();
        let time_str = String::from_utf8_lossy(&self.time).trim().to_string();

        // Parse MM-DD-YY format
        let date = NaiveDate::parse_from_str(&date_str, "%m-%d-%y").ok()?;
        // Parse HH:MM format - need to add seconds
        let time_with_secs = format!("{}:00", time_str);
        let time = chrono::NaiveTime::parse_from_str(&time_with_secs, "%H:%M:%S").ok()?;

        Some(NaiveDateTime::new(date, time))
    }

    /// Helper to pad a field to a specific length
    fn pad_field(data: &[u8], len: usize) -> Vec<u8> {
        let mut result = Vec::with_capacity(len);
        result.extend_from_slice(&data[..data.len().min(len)]);
        result.resize(len, b' ');
        result
    }
}

impl Default for QwkMessageHeader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use binrw::{BinRead, BinWrite};
    use chrono::{NaiveDate, Timelike};
    use std::io::Cursor;

    #[test]
    fn test_message_status_conversion() {
        assert_eq!(MessageStatus::from_byte(b' '), Some(MessageStatus::Public));
        assert_eq!(MessageStatus::from_byte(b'*'), Some(MessageStatus::Private));
        assert_eq!(MessageStatus::from_byte(b'X'), None);

        assert_eq!(MessageStatus::Public.to_byte(), b' ');
        assert_eq!(MessageStatus::Private.to_byte(), b'*');
    }

    #[test]
    fn test_header_size() {
        let header = QwkMessageHeader::new();
        let mut cursor = Cursor::new(Vec::new());
        header.write(&mut cursor).unwrap();

        let bytes = cursor.into_inner();
        assert_eq!(bytes.len(), 128, "QWK header must be exactly 128 bytes");
    }

    #[test]
    fn test_header_builder() {
        let dt = NaiveDate::from_ymd_opt(2025, 11, 26)
            .unwrap()
            .and_hms_opt(14, 30, 0)
            .unwrap();

        let header = QwkMessageHeader::new()
            .with_status(MessageStatus::Private)
            .with_message_number(42)
            .with_datetime(dt)
            .with_to("Bob")
            .with_from("Alice")
            .with_subject("Test Subject")
            .with_reply_to(10)
            .with_num_blocks(5);

        assert_eq!(header.status, b'*');
        assert_eq!(header.get_message_number(), Some(42));
        assert_eq!(header.get_to(), "Bob");
        assert_eq!(header.get_from(), "Alice");
        assert_eq!(header.get_subject(), "Test Subject");
        assert_eq!(header.get_reply_to(), Some(10));
        assert_eq!(header.get_num_blocks(), Some(5));
    }

    #[test]
    fn test_header_datetime() {
        let dt = NaiveDate::from_ymd_opt(2025, 11, 26)
            .unwrap()
            .and_hms_opt(14, 30, 0)
            .unwrap();

        let header = QwkMessageHeader::new().with_datetime(dt);
        let parsed_dt = header.get_datetime().unwrap();

        assert_eq!(parsed_dt.date(), dt.date());
        // Note: seconds are always 0 in QWK format
        assert_eq!(parsed_dt.time().hour(), dt.time().hour());
        assert_eq!(parsed_dt.time().minute(), dt.time().minute());
    }

    #[test]
    fn test_header_field_truncation() {
        let long_name = "ThisIsAReallyLongNameThatExceeds25Characters";
        let header = QwkMessageHeader::new().with_to(long_name);

        assert_eq!(header.to.len(), 25);
        assert!(header.get_to().len() <= 25);
    }

    #[test]
    fn test_header_roundtrip() {
        let dt = NaiveDate::from_ymd_opt(2025, 11, 26)
            .unwrap()
            .and_hms_opt(14, 30, 0)
            .unwrap();

        let original = QwkMessageHeader::new()
            .with_status(MessageStatus::Public)
            .with_message_number(123)
            .with_datetime(dt)
            .with_to("Bob")
            .with_from("Alice")
            .with_subject("Hello World")
            .with_num_blocks(3);

        // Write to bytes
        let mut cursor = Cursor::new(Vec::new());
        original.write(&mut cursor).unwrap();

        // Read back
        let bytes = cursor.into_inner();
        let mut read_cursor = Cursor::new(bytes);
        let read_back = QwkMessageHeader::read(&mut read_cursor).unwrap();

        assert_eq!(read_back.status, original.status);
        assert_eq!(
            read_back.get_message_number(),
            original.get_message_number()
        );
        assert_eq!(read_back.get_to(), original.get_to());
        assert_eq!(read_back.get_from(), original.get_from());
        assert_eq!(read_back.get_subject(), original.get_subject());
    }

    #[test]
    fn test_default_header() {
        let header = QwkMessageHeader::default();
        assert_eq!(header.status, b' ');
        assert_eq!(header.active, 0xE1);
        assert_eq!(header.get_message_number(), Some(0));
    }
}
