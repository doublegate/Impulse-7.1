//! JAM format writing functionality

use super::{JamBaseHeader, JamMessageHeader, MessageAttributes, SubfieldType};
use crate::atomic::{AtomicMultiWriter, AtomicWriter};
use crate::error::{MessageError, Result};
use crate::types::NewMessage;
use chrono::Utc;
use std::path::Path;

/// JAM message writer
pub struct JamWriter {
    /// Base path (without extension)
    base_path: String,
}

impl JamWriter {
    /// Create a new JAM writer
    pub fn new(base_path: impl AsRef<Path>) -> Self {
        Self {
            base_path: base_path.as_ref().to_string_lossy().to_string(),
        }
    }

    /// Write a new message to the JAM base
    ///
    /// # Arguments
    /// * `message` - The message to write
    /// * `msg_num` - The message number to assign
    /// * `current_offset` - Current offset in .JDT file
    ///
    /// # Returns
    /// Tuple of (message bytes, next offset)
    pub async fn write_message(
        &self,
        message: &NewMessage,
        msg_num: u32,
        current_offset: u32,
    ) -> Result<(Vec<u8>, Vec<u8>, u32)> {
        // Build subfields
        let subfields = vec![
            // From field
            Self::create_subfield(SubfieldType::SendName, message.from.as_bytes()),
            // To field
            Self::create_subfield(SubfieldType::RecvName, message.to.as_bytes()),
            // Subject field
            Self::create_subfield(SubfieldType::Subject, message.subject.as_bytes()),
        ];

        // Calculate subfield total length
        let subfield_len: u32 = subfields.iter().map(|s| s.len() as u32).sum();

        // Build message attributes
        let mut attributes = MessageAttributes::LOCAL;
        if message.is_private {
            attributes |= MessageAttributes::PRIVATE;
        }

        // Get current timestamp
        let now = Utc::now().timestamp() as u32;

        // Build message header
        let header = JamMessageHeader {
            signature: b"JAM\0".to_vec(),
            revision: 1,
            reserved: 0,
            subfield_len,
            times_read: 0,
            msg_id_crc: 0, // Would calculate CRC in production
            reply_id_crc: 0,
            reply_to: message.reply_to.unwrap_or(0),
            reply_1st: 0,
            reply_next: 0,
            date_written: now,
            date_received: now,
            date_processed: 0,
            msg_num,
            attribute: attributes,
            attribute2: 0,
            offset: current_offset,
            text_len: message.body.len() as u32,
        };

        // Serialize header
        let header_bytes = Self::serialize_header(&header)?;

        // Serialize subfields
        let mut subfield_bytes = Vec::new();
        for subfield in subfields {
            subfield_bytes.extend_from_slice(&subfield);
        }

        // Combine header + subfields
        let mut jhr_data = header_bytes;
        jhr_data.extend_from_slice(&subfield_bytes);

        // Message text for .JDT
        let jdt_data = message.body.as_bytes().to_vec();

        let next_offset = current_offset + jdt_data.len() as u32;

        Ok((jhr_data, jdt_data, next_offset))
    }

    /// Create a subfield
    fn create_subfield(field_type: SubfieldType, data: &[u8]) -> Vec<u8> {
        let lo_id = field_type as u16;
        let hi_id = 0u16;
        let datlen = data.len() as u32;

        let mut bytes = Vec::new();
        bytes.extend_from_slice(&lo_id.to_le_bytes());
        bytes.extend_from_slice(&hi_id.to_le_bytes());
        bytes.extend_from_slice(&datlen.to_le_bytes());
        bytes.extend_from_slice(data);

        bytes
    }

    /// Serialize a message header
    fn serialize_header(header: &JamMessageHeader) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();

        // Signature (4 bytes)
        bytes.extend_from_slice(&header.signature);

        // Revision (2 bytes)
        bytes.extend_from_slice(&header.revision.to_le_bytes());

        // Reserved (2 bytes)
        bytes.extend_from_slice(&header.reserved.to_le_bytes());

        // Subfield length (4 bytes)
        bytes.extend_from_slice(&header.subfield_len.to_le_bytes());

        // Times read (4 bytes)
        bytes.extend_from_slice(&header.times_read.to_le_bytes());

        // Message ID CRC (4 bytes)
        bytes.extend_from_slice(&header.msg_id_crc.to_le_bytes());

        // Reply ID CRC (4 bytes)
        bytes.extend_from_slice(&header.reply_id_crc.to_le_bytes());

        // Reply to (4 bytes)
        bytes.extend_from_slice(&header.reply_to.to_le_bytes());

        // Reply 1st (4 bytes)
        bytes.extend_from_slice(&header.reply_1st.to_le_bytes());

        // Reply next (4 bytes)
        bytes.extend_from_slice(&header.reply_next.to_le_bytes());

        // Date written (4 bytes)
        bytes.extend_from_slice(&header.date_written.to_le_bytes());

        // Date received (4 bytes)
        bytes.extend_from_slice(&header.date_received.to_le_bytes());

        // Date processed (4 bytes)
        bytes.extend_from_slice(&header.date_processed.to_le_bytes());

        // Message number (4 bytes)
        bytes.extend_from_slice(&header.msg_num.to_le_bytes());

        // Attribute (4 bytes)
        bytes.extend_from_slice(&header.attribute.to_le_bytes());

        // Attribute2 (4 bytes)
        bytes.extend_from_slice(&header.attribute2.to_le_bytes());

        // Offset (4 bytes)
        bytes.extend_from_slice(&header.offset.to_le_bytes());

        // Text length (4 bytes)
        bytes.extend_from_slice(&header.text_len.to_le_bytes());

        Ok(bytes)
    }

    /// Update base header with new message count
    pub async fn update_base_header(&self, new_count: u32) -> Result<()> {
        let jhr_path = format!("{}.jhr", self.base_path);

        // Read existing base header
        let mut base_header = tokio::fs::read(&jhr_path)
            .await
            .map_err(|e| MessageError::WriteError(format!("Failed to read base header: {}", e)))?;

        // Update active count (offset 12)
        let active_offset = 12;
        if base_header.len() >= active_offset + 4 {
            let count_bytes = new_count.to_le_bytes();
            base_header[active_offset..active_offset + 4].copy_from_slice(&count_bytes);

            // Update modified timestamp (offset 8)
            let now = Utc::now().timestamp() as u32;
            let modified_bytes = now.to_le_bytes();
            base_header[8..12].copy_from_slice(&modified_bytes);

            // Write atomically
            let writer = AtomicWriter::new(&jhr_path);
            writer.write(&base_header).await?;
        }

        Ok(())
    }

    /// Append message to .JHR and .JDT files atomically
    pub async fn append_message(&self, jhr_data: &[u8], jdt_data: &[u8]) -> Result<()> {
        let jhr_path = format!("{}.jhr", self.base_path);
        let jdt_path = format!("{}.jdt", self.base_path);

        // Use atomic append
        let jhr_writer = AtomicWriter::new(&jhr_path);
        jhr_writer.append(jhr_data).await?;

        let jdt_writer = AtomicWriter::new(&jdt_path);
        jdt_writer.append(jdt_data).await?;

        Ok(())
    }

    /// Initialize a new JAM message base
    pub async fn initialize_base(&self) -> Result<()> {
        let jhr_path = format!("{}.jhr", self.base_path);
        let jdt_path = format!("{}.jdt", self.base_path);
        let jdx_path = format!("{}.jdx", self.base_path);

        // Create base header
        let now = Utc::now().timestamp() as u32;
        let base_header = JamBaseHeader {
            signature: b"JAM\0".to_vec(),
            created: now,
            modified: now,
            active: 0,
            password_crc: 0,
            base_msg_num: 1,
        };

        // Serialize base header
        let mut header_bytes = Vec::new();
        header_bytes.extend_from_slice(&base_header.signature);
        header_bytes.extend_from_slice(&base_header.created.to_le_bytes());
        header_bytes.extend_from_slice(&base_header.modified.to_le_bytes());
        header_bytes.extend_from_slice(&base_header.active.to_le_bytes());
        header_bytes.extend_from_slice(&base_header.password_crc.to_le_bytes());
        header_bytes.extend_from_slice(&base_header.base_msg_num.to_le_bytes());

        // Write files atomically
        let mut writer = AtomicMultiWriter::new();
        writer.add_file(&jhr_path, header_bytes);
        writer.add_file(&jdt_path, Vec::new());
        writer.add_file(&jdx_path, Vec::new());

        writer.write_all().await?;

        Ok(())
    }

    /// Get current .JDT file size
    pub async fn get_jdt_size(&self) -> Result<u32> {
        let jdt_path = format!("{}.jdt", self.base_path);

        match tokio::fs::metadata(&jdt_path).await {
            Ok(metadata) => Ok(metadata.len() as u32),
            Err(_) => Ok(0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_initialize_base() {
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path().join("test");

        let writer = JamWriter::new(&base_path);
        writer.initialize_base().await.unwrap();

        // Check files exist
        assert!(base_path.with_extension("jhr").exists());
        assert!(base_path.with_extension("jdt").exists());
        assert!(base_path.with_extension("jdx").exists());

        // Check base header
        let header_bytes = tokio::fs::read(base_path.with_extension("jhr"))
            .await
            .unwrap();
        assert_eq!(&header_bytes[0..4], b"JAM\0");
    }

    #[tokio::test]
    async fn test_write_message() {
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path().join("test");

        let writer = JamWriter::new(&base_path);
        writer.initialize_base().await.unwrap();

        let message =
            NewMessage::new("Alice", "Bob", "Test Subject").with_body("This is a test message.");

        let (jhr_data, jdt_data, next_offset) = writer.write_message(&message, 1, 0).await.unwrap();

        assert!(!jhr_data.is_empty());
        assert!(!jdt_data.is_empty());
        assert_eq!(next_offset, jdt_data.len() as u32);
    }

    #[tokio::test]
    async fn test_append_message() {
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path().join("test");

        let writer = JamWriter::new(&base_path);
        writer.initialize_base().await.unwrap();

        let message =
            NewMessage::new("Alice", "Bob", "Test Subject").with_body("This is a test message.");

        let (jhr_data, jdt_data, _) = writer.write_message(&message, 1, 0).await.unwrap();
        writer.append_message(&jhr_data, &jdt_data).await.unwrap();

        // Verify files were written
        let jdt_content = tokio::fs::read(base_path.with_extension("jdt"))
            .await
            .unwrap();
        assert_eq!(jdt_content, b"This is a test message.");
    }

    #[tokio::test]
    async fn test_update_base_header() {
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path().join("test");

        let writer = JamWriter::new(&base_path);
        writer.initialize_base().await.unwrap();

        writer.update_base_header(5).await.unwrap();

        // Read and verify
        let header_bytes = tokio::fs::read(base_path.with_extension("jhr"))
            .await
            .unwrap();
        let active_count = u32::from_le_bytes([
            header_bytes[12],
            header_bytes[13],
            header_bytes[14],
            header_bytes[15],
        ]);
        assert_eq!(active_count, 5);
    }
}
