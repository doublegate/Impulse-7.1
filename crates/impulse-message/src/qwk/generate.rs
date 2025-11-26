//! QWK packet generation
//!
//! Generates QWK offline mail packets containing messages for download.

use super::error::{QwkError, Result};
use super::header::{MessageStatus, QwkMessageHeader};
use crate::qwk::compress::QwkCompressor;
use crate::types::FullMessage;
use binrw::BinWrite;
use chrono::Local;
use std::io::Cursor;
use std::path::Path;

/// Configuration for QWK packet generation
#[derive(Debug, Clone)]
pub struct QwkConfig {
    /// BBS name (for DOOR.ID)
    pub bbs_name: String,
    /// BBS location (for DOOR.ID)
    pub bbs_location: String,
    /// BBS sysop name (for DOOR.ID)
    pub sysop_name: String,
    /// BBS phone number (for DOOR.ID)
    pub phone: String,
    /// Default conference number
    pub default_conference: u16,
}

impl Default for QwkConfig {
    fn default() -> Self {
        Self {
            bbs_name: "Impulse BBS".to_string(),
            bbs_location: "Cyberspace".to_string(),
            sysop_name: "Sysop".to_string(),
            phone: "000-000-0000".to_string(),
            default_conference: 1,
        }
    }
}

/// QWK packet generator
pub struct QwkPacketGenerator {
    config: QwkConfig,
    messages: Vec<FullMessage>,
}

impl QwkPacketGenerator {
    /// Create a new QWK packet generator
    pub fn new(config: QwkConfig) -> Self {
        Self {
            config,
            messages: Vec::new(),
        }
    }

    /// Add a message to the packet
    pub fn add_message(&mut self, message: FullMessage) {
        self.messages.push(message);
    }

    /// Add multiple messages to the packet
    pub fn add_messages(&mut self, messages: Vec<FullMessage>) {
        self.messages.extend(messages);
    }

    /// Generate the QWK packet and save to file
    pub async fn generate<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let mut compressor = QwkCompressor::new(path)?;

        // Generate and add CONTROL.DAT
        let control_dat = self.generate_control_dat();
        compressor.add_file("CONTROL.DAT", control_dat.as_bytes())?;

        // Generate and add DOOR.ID
        let door_id = self.generate_door_id();
        compressor.add_file("DOOR.ID", door_id.as_bytes())?;

        // Generate and add MESSAGES.DAT
        let messages_dat = self.generate_messages_dat()?;
        compressor.add_file("MESSAGES.DAT", &messages_dat)?;

        compressor.finish()?;
        Ok(())
    }

    /// Generate CONTROL.DAT file content
    fn generate_control_dat(&self) -> String {
        let mut control = String::new();

        // Line 1: BBS name
        control.push_str(&self.config.bbs_name);
        control.push('\n');

        // Line 2: BBS location
        control.push_str(&self.config.bbs_location);
        control.push('\n');

        // Line 3: BBS phone
        control.push_str(&self.config.phone);
        control.push('\n');

        // Line 4: Sysop name
        control.push_str(&self.config.sysop_name);
        control.push('\n');

        // Line 5: Serial number (00000)
        control.push_str("00000,");
        control.push_str(&self.config.bbs_name);
        control.push('\n');

        // Line 6: Date and time
        control.push_str(&Local::now().format("%m-%d-%Y,%H:%M:%S").to_string());
        control.push('\n');

        // Line 7: User name (blank for now)
        control.push('\n');

        // Line 8: Menu name (blank)
        control.push('\n');

        // Line 9: Registration number (0)
        control.push_str("0\n");

        // Line 10: Number of messages
        control.push_str(&format!("{}\n", self.messages.len()));

        // Conference information
        control.push_str(&format!("{}\n", self.config.default_conference));
        control.push_str("Main\n");

        control
    }

    /// Generate DOOR.ID file content
    fn generate_door_id(&self) -> String {
        format!(
            "DOOR = Impulse BBS\nVERSION = 0.3.0\nSYSTEM = {}\nCONTROLNAME = IMPULSE\nCONTROLTYPE = ADD\n",
            self.config.bbs_name
        )
    }

    /// Generate MESSAGES.DAT file content
    fn generate_messages_dat(&self) -> Result<Vec<u8>> {
        let mut data = Vec::new();

        // First block is a placeholder (128 bytes of spaces)
        data.extend_from_slice(&[b' '; 128]);

        for (idx, message) in self.messages.iter().enumerate() {
            let msg_num = (idx + 1) as u32;

            // Calculate number of 128-byte blocks needed for message body
            let body_bytes = message.body.as_bytes();
            let num_blocks = body_bytes.len().div_ceil(128) as u32;

            // Create message header
            let header = QwkMessageHeader::new()
                .with_status(if message.header.is_private {
                    MessageStatus::Private
                } else {
                    MessageStatus::Public
                })
                .with_message_number(msg_num)
                .with_datetime(message.header.date.naive_utc())
                .with_to(&message.header.to)
                .with_from(&message.header.from)
                .with_subject(&message.header.subject)
                .with_reply_to(message.header.reply_to.unwrap_or(0))
                .with_num_blocks(num_blocks + 1); // +1 for header block

            // Write header
            let mut cursor = Cursor::new(Vec::new());
            header
                .write(&mut cursor)
                .map_err(|e| QwkError::Encoding(e.to_string()))?;
            data.extend_from_slice(&cursor.into_inner());

            // Write message body in 128-byte blocks
            let mut body_pos = 0;
            while body_pos < body_bytes.len() {
                let end_pos = (body_pos + 128).min(body_bytes.len());
                let mut block = vec![b' '; 128]; // Pad with spaces
                block[..end_pos - body_pos].copy_from_slice(&body_bytes[body_pos..end_pos]);
                data.extend_from_slice(&block);
                body_pos = end_pos;
            }

            // If body doesn't fill last block, it's already padded with spaces
        }

        Ok(data)
    }

    /// Get the number of messages in the packet
    pub fn message_count(&self) -> usize {
        self.messages.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::MessageHeader;
    use chrono::NaiveDate;
    use tempfile::TempDir;

    fn create_test_message(num: u32) -> FullMessage {
        use chrono::TimeZone;
        let dt = NaiveDate::from_ymd_opt(2025, 11, 26)
            .unwrap()
            .and_hms_opt(14, 30, 0)
            .unwrap();

        FullMessage {
            header: MessageHeader {
                msg_num: num,
                from: "Alice".to_string(),
                to: "Bob".to_string(),
                subject: "Test Subject".to_string(),
                date: chrono::Utc.from_utc_datetime(&dt),
                is_read: false,
                is_private: false,
                reply_to: None,
                reply_count: 0,
            },
            body: "This is a test message.".to_string(),
            kludges: Vec::new(),
        }
    }

    #[test]
    fn test_generator_creation() {
        let config = QwkConfig::default();
        let generator = QwkPacketGenerator::new(config);
        assert_eq!(generator.message_count(), 0);
    }

    #[test]
    fn test_add_message() {
        let config = QwkConfig::default();
        let mut generator = QwkPacketGenerator::new(config);

        generator.add_message(create_test_message(1));
        assert_eq!(generator.message_count(), 1);

        generator.add_message(create_test_message(2));
        assert_eq!(generator.message_count(), 2);
    }

    #[test]
    fn test_add_messages() {
        let config = QwkConfig::default();
        let mut generator = QwkPacketGenerator::new(config);

        let messages = vec![create_test_message(1), create_test_message(2)];
        generator.add_messages(messages);
        assert_eq!(generator.message_count(), 2);
    }

    #[test]
    fn test_control_dat_generation() {
        let config = QwkConfig {
            bbs_name: "Test BBS".to_string(),
            bbs_location: "Test City".to_string(),
            sysop_name: "Test Sysop".to_string(),
            phone: "555-1234".to_string(),
            default_conference: 1,
        };

        let generator = QwkPacketGenerator::new(config);
        let control_dat = generator.generate_control_dat();

        assert!(control_dat.contains("Test BBS"));
        assert!(control_dat.contains("Test City"));
        assert!(control_dat.contains("555-1234"));
        assert!(control_dat.contains("Test Sysop"));
    }

    #[test]
    fn test_door_id_generation() {
        let config = QwkConfig::default();
        let generator = QwkPacketGenerator::new(config);
        let door_id = generator.generate_door_id();

        assert!(door_id.contains("DOOR = Impulse BBS"));
        assert!(door_id.contains("VERSION = 0.3.0"));
        assert!(door_id.contains("CONTROLNAME = IMPULSE"));
    }

    #[test]
    fn test_messages_dat_structure() {
        let config = QwkConfig::default();
        let mut generator = QwkPacketGenerator::new(config);
        generator.add_message(create_test_message(1));

        let messages_dat = generator.generate_messages_dat().unwrap();

        // First block should be 128 bytes of spaces
        assert_eq!(&messages_dat[0..128], &[b' '; 128]);

        // Second block should be a message header
        assert_eq!(messages_dat.len() % 128, 0);
        assert!(messages_dat.len() >= 256); // At least header + one body block
    }

    #[tokio::test]
    async fn test_generate_packet() {
        let temp_dir = TempDir::new().unwrap();
        let packet_path = temp_dir.path().join("test.qwk");

        let config = QwkConfig::default();
        let mut generator = QwkPacketGenerator::new(config);
        generator.add_message(create_test_message(1));

        generator.generate(&packet_path).await.unwrap();

        // Verify packet was created
        assert!(packet_path.exists());

        // Verify packet contains required files
        use crate::qwk::compress::QwkDecompressor;
        let mut decompressor = QwkDecompressor::open(&packet_path).unwrap();
        assert!(decompressor.has_file("CONTROL.DAT"));
        assert!(decompressor.has_file("DOOR.ID"));
        assert!(decompressor.has_file("MESSAGES.DAT"));
    }

    #[tokio::test]
    async fn test_generate_empty_packet() {
        let temp_dir = TempDir::new().unwrap();
        let packet_path = temp_dir.path().join("empty.qwk");

        let config = QwkConfig::default();
        let generator = QwkPacketGenerator::new(config);

        generator.generate(&packet_path).await.unwrap();

        // Empty packet should still be valid
        assert!(packet_path.exists());
    }

    #[test]
    fn test_messages_dat_multiple_messages() {
        let config = QwkConfig::default();
        let mut generator = QwkPacketGenerator::new(config);
        generator.add_message(create_test_message(1));
        generator.add_message(create_test_message(2));
        generator.add_message(create_test_message(3));

        let messages_dat = generator.generate_messages_dat().unwrap();

        // Should have: 1 placeholder block + (header + body blocks) * 3 messages
        assert!(messages_dat.len() >= 128 * 4); // At least 4 blocks
        assert_eq!(messages_dat.len() % 128, 0); // Multiple of 128
    }
}
