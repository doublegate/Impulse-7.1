//! QWK reply packet parsing
//!
//! Parses QWK reply packets (.REP files) uploaded by users.

use super::compress::QwkDecompressor;
use super::error::{QwkError, Result};
use super::header::QwkMessageHeader;
use crate::types::NewMessage;
use binrw::BinRead;
use std::io::Cursor;
use std::path::Path;

/// Parsed message from a QWK reply packet
#[derive(Debug, Clone)]
pub struct ParsedReply {
    /// Message recipient
    pub to: String,
    /// Message subject
    pub subject: String,
    /// Message body
    pub body: String,
    /// Conference/area number
    pub conference: u16,
    /// Reply to message number (0 if not a reply)
    pub reply_to: u32,
    /// Whether this is a private message
    pub is_private: bool,
}

impl ParsedReply {
    /// Convert to NewMessage for posting
    pub fn to_new_message(self, from: &str) -> NewMessage {
        let mut msg = NewMessage::new(from, &self.to, &self.subject).with_body(&self.body);

        if self.is_private {
            msg = msg.private();
        }

        if self.reply_to > 0 {
            msg = msg.reply_to(self.reply_to);
        }

        msg
    }
}

/// QWK reply packet parser
pub struct QwkReplyParser {
    decompressor: QwkDecompressor,
}

impl QwkReplyParser {
    /// Open a QWK reply packet for parsing
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let decompressor = QwkDecompressor::open(path)?;
        Ok(Self { decompressor })
    }

    /// Parse all messages from the reply packet
    pub fn parse_messages(&mut self) -> Result<Vec<ParsedReply>> {
        // Look for .MSG file or MESSAGES.DAT
        let msg_data = if self.decompressor.has_file("MESSAGES.DAT") {
            self.decompressor.extract_file("MESSAGES.DAT")?
        } else {
            // Try to find any .MSG file
            let names = self.decompressor.file_names();
            let msg_file = names
                .iter()
                .find(|n| n.to_uppercase().ends_with(".MSG"))
                .ok_or_else(|| QwkError::MissingFile("*.MSG or MESSAGES.DAT".to_string()))?;
            self.decompressor.extract_file(msg_file)?
        };

        Self::parse_message_data(&msg_data)
    }

    /// Parse message data from bytes
    fn parse_message_data(data: &[u8]) -> Result<Vec<ParsedReply>> {
        let mut messages = Vec::new();
        let mut pos = 0;

        // Skip first block (placeholder)
        if data.len() < 128 {
            return Ok(messages);
        }
        pos += 128;

        while pos + 128 <= data.len() {
            // Read message header
            let header_data = &data[pos..pos + 128];
            let mut cursor = Cursor::new(header_data);

            let header = QwkMessageHeader::read(&mut cursor)
                .map_err(|e| QwkError::HeaderParse(e.to_string()))?;

            pos += 128;

            // Get number of body blocks
            let num_blocks = header
                .get_num_blocks()
                .ok_or_else(|| QwkError::HeaderParse("invalid block count".to_string()))?;

            // Subtract 1 for header block
            let body_blocks = if num_blocks > 0 { num_blocks - 1 } else { 0 };

            // Read body blocks
            let mut body_data = Vec::new();
            for _ in 0..body_blocks {
                if pos + 128 > data.len() {
                    return Err(QwkError::InvalidFormat(
                        "unexpected end of message data".to_string(),
                    ));
                }
                body_data.extend_from_slice(&data[pos..pos + 128]);
                pos += 128;
            }

            // Parse body (trim trailing spaces and null bytes)
            let body = String::from_utf8_lossy(&body_data)
                .trim_end_matches([' ', '\0'])
                .to_string();

            // Create parsed reply
            let reply = ParsedReply {
                to: header.get_to(),
                subject: header.get_subject(),
                body,
                conference: 1, // Default conference
                reply_to: header.get_reply_to().unwrap_or(0),
                is_private: header.status == b'*' || header.status == b'+',
            };

            messages.push(reply);
        }

        Ok(messages)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::qwk::compress::QwkCompressor;
    use crate::qwk::header::{MessageStatus, QwkMessageHeader};
    use binrw::BinWrite;
    use chrono::NaiveDate;
    use std::io::Cursor;
    use tempfile::TempDir;

    fn create_test_reply_packet(temp_dir: &TempDir) -> std::path::PathBuf {
        let packet_path = temp_dir.path().join("test.rep");

        // Create MESSAGES.DAT content
        let mut messages_dat = Vec::new();

        // First block (placeholder)
        messages_dat.extend_from_slice(&[b' '; 128]);

        // Create a test message
        let dt = NaiveDate::from_ymd_opt(2025, 11, 26)
            .unwrap()
            .and_hms_opt(14, 30, 0)
            .unwrap();

        let body = "This is a test reply message.";
        let body_blocks = body.len().div_ceil(128) as u32;

        let header = QwkMessageHeader::new()
            .with_status(MessageStatus::Public)
            .with_message_number(1)
            .with_datetime(dt)
            .with_to("Sysop")
            .with_from("User")
            .with_subject("Test Reply")
            .with_num_blocks(body_blocks + 1);

        // Write header
        let mut cursor = Cursor::new(Vec::new());
        header.write(&mut cursor).unwrap();
        messages_dat.extend_from_slice(&cursor.into_inner());

        // Write body
        let mut body_block = vec![b' '; 128];
        body_block[..body.len()].copy_from_slice(body.as_bytes());
        messages_dat.extend_from_slice(&body_block);

        // Create ZIP packet
        let mut compressor = QwkCompressor::new(&packet_path).unwrap();
        compressor.add_file("MESSAGES.DAT", &messages_dat).unwrap();
        compressor.finish().unwrap();

        packet_path
    }

    #[test]
    fn test_parser_creation() {
        let temp_dir = TempDir::new().unwrap();
        let packet_path = create_test_reply_packet(&temp_dir);

        let parser = QwkReplyParser::open(&packet_path);
        assert!(parser.is_ok());
    }

    #[test]
    fn test_parse_messages() {
        let temp_dir = TempDir::new().unwrap();
        let packet_path = create_test_reply_packet(&temp_dir);

        let mut parser = QwkReplyParser::open(&packet_path).unwrap();
        let messages = parser.parse_messages().unwrap();

        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].to, "Sysop");
        assert_eq!(messages[0].subject, "Test Reply");
        assert_eq!(messages[0].body, "This is a test reply message.");
        assert!(!messages[0].is_private);
    }

    #[test]
    fn test_parse_private_message() {
        let temp_dir = TempDir::new().unwrap();
        let packet_path = temp_dir.path().join("private.rep");

        // Create packet with private message
        let mut messages_dat = Vec::new();
        messages_dat.extend_from_slice(&[b' '; 128]); // Placeholder

        let dt = NaiveDate::from_ymd_opt(2025, 11, 26)
            .unwrap()
            .and_hms_opt(14, 30, 0)
            .unwrap();

        let header = QwkMessageHeader::new()
            .with_status(MessageStatus::Private)
            .with_message_number(1)
            .with_datetime(dt)
            .with_to("Alice")
            .with_from("Bob")
            .with_subject("Private")
            .with_num_blocks(2);

        let mut cursor = Cursor::new(Vec::new());
        header.write(&mut cursor).unwrap();
        messages_dat.extend_from_slice(&cursor.into_inner());

        let mut body_block = vec![b' '; 128];
        body_block[..12].copy_from_slice(b"Private msg!");
        messages_dat.extend_from_slice(&body_block);

        let mut compressor = QwkCompressor::new(&packet_path).unwrap();
        compressor.add_file("MESSAGES.DAT", &messages_dat).unwrap();
        compressor.finish().unwrap();

        // Parse and verify
        let mut parser = QwkReplyParser::open(&packet_path).unwrap();
        let messages = parser.parse_messages().unwrap();

        assert_eq!(messages.len(), 1);
        assert!(messages[0].is_private);
    }

    #[test]
    fn test_parse_reply_message() {
        let temp_dir = TempDir::new().unwrap();
        let packet_path = temp_dir.path().join("reply.rep");

        // Create packet with reply
        let mut messages_dat = Vec::new();
        messages_dat.extend_from_slice(&[b' '; 128]);

        let dt = NaiveDate::from_ymd_opt(2025, 11, 26)
            .unwrap()
            .and_hms_opt(14, 30, 0)
            .unwrap();

        let header = QwkMessageHeader::new()
            .with_status(MessageStatus::Public)
            .with_message_number(2)
            .with_datetime(dt)
            .with_to("Alice")
            .with_from("Bob")
            .with_subject("Re: Original")
            .with_reply_to(42)
            .with_num_blocks(2);

        let mut cursor = Cursor::new(Vec::new());
        header.write(&mut cursor).unwrap();
        messages_dat.extend_from_slice(&cursor.into_inner());

        let mut body_block = vec![b' '; 128];
        body_block[..5].copy_from_slice(b"Reply");
        messages_dat.extend_from_slice(&body_block);

        let mut compressor = QwkCompressor::new(&packet_path).unwrap();
        compressor.add_file("MESSAGES.DAT", &messages_dat).unwrap();
        compressor.finish().unwrap();

        // Parse and verify
        let mut parser = QwkReplyParser::open(&packet_path).unwrap();
        let messages = parser.parse_messages().unwrap();

        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].reply_to, 42);
    }

    #[test]
    fn test_parse_multiple_messages() {
        let temp_dir = TempDir::new().unwrap();
        let packet_path = temp_dir.path().join("multi.rep");

        let mut messages_dat = Vec::new();
        messages_dat.extend_from_slice(&[b' '; 128]);

        let dt = NaiveDate::from_ymd_opt(2025, 11, 26)
            .unwrap()
            .and_hms_opt(14, 30, 0)
            .unwrap();

        // Add 3 messages
        for i in 1..=3 {
            let header = QwkMessageHeader::new()
                .with_status(MessageStatus::Public)
                .with_message_number(i)
                .with_datetime(dt)
                .with_to(&format!("User{}", i))
                .with_from("Sender")
                .with_subject(&format!("Message {}", i))
                .with_num_blocks(2);

            let mut cursor = Cursor::new(Vec::new());
            header.write(&mut cursor).unwrap();
            messages_dat.extend_from_slice(&cursor.into_inner());

            let mut body_block = vec![b' '; 128];
            let body = format!("Body {}", i);
            body_block[..body.len()].copy_from_slice(body.as_bytes());
            messages_dat.extend_from_slice(&body_block);
        }

        let mut compressor = QwkCompressor::new(&packet_path).unwrap();
        compressor.add_file("MESSAGES.DAT", &messages_dat).unwrap();
        compressor.finish().unwrap();

        // Parse and verify
        let mut parser = QwkReplyParser::open(&packet_path).unwrap();
        let messages = parser.parse_messages().unwrap();

        assert_eq!(messages.len(), 3);
        assert_eq!(messages[0].to, "User1");
        assert_eq!(messages[1].to, "User2");
        assert_eq!(messages[2].to, "User3");
    }

    #[test]
    fn test_to_new_message() {
        let reply = ParsedReply {
            to: "Bob".to_string(),
            subject: "Test".to_string(),
            body: "Hello".to_string(),
            conference: 1,
            reply_to: 0,
            is_private: false,
        };

        let new_msg = reply.to_new_message("Alice");
        assert_eq!(new_msg.from, "Alice");
        assert_eq!(new_msg.to, "Bob");
        assert_eq!(new_msg.subject, "Test");
        assert_eq!(new_msg.body, "Hello");
    }

    #[test]
    fn test_missing_messages_file() {
        let temp_dir = TempDir::new().unwrap();
        let packet_path = temp_dir.path().join("empty.rep");

        // Create packet without MESSAGES.DAT
        let mut compressor = QwkCompressor::new(&packet_path).unwrap();
        compressor.add_file("CONTROL.DAT", b"empty").unwrap();
        compressor.finish().unwrap();

        let mut parser = QwkReplyParser::open(&packet_path).unwrap();
        let result = parser.parse_messages();
        assert!(matches!(result, Err(QwkError::MissingFile(_))));
    }
}
