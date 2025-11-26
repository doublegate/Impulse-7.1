//! Hudson message base format implementation
//!
//! Hudson (aka QuickBBS/RA/SuperBBS) is an older message base format
//! that stores messages in a single file with a separate index.

use crate::error::{MessageError, Result};
use crate::traits::MessageBase;
use crate::types::{
    FullMessage, KludgeLine, MessageBaseStats, MessageHeader, MessageThread, NewMessage,
    SearchCriteria,
};
use async_trait::async_trait;
use binrw::{BinRead, binread};
use chrono::{DateTime, NaiveDate, Utc};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use tokio::sync::RwLock;

/// Hudson message header (stored in message file)
#[binread]
#[derive(Debug, Clone)]
#[br(little)]
pub struct HudsonMessageHeader {
    /// Message number
    pub msg_num: u16,

    /// Previous message in reply chain
    pub prev_reply: u16,

    /// Next message in reply chain
    pub next_reply: u16,

    /// Times read (low word)
    pub times_read_lo: u16,

    /// Start block of message text
    pub start_block: u16,

    /// Number of blocks used
    pub num_blocks: u16,

    /// Destination net
    pub dest_net: u16,

    /// Destination node
    pub dest_node: u16,

    /// Origin net
    pub orig_net: u16,

    /// Origin node
    pub orig_node: u16,

    /// Destination zone (ushort in some implementations)
    pub dest_zone: u16,

    /// Origin zone
    pub orig_zone: u16,

    /// Written date (packed DOS date: year-1980, month, day)
    pub written_date: u16,

    /// Written time (packed DOS time: hour, minute, second/2)
    pub written_time: u16,

    /// Received date (packed DOS date)
    pub recd_date: u16,

    /// Received time (packed DOS time)
    pub recd_time: u16,

    /// Message cost
    pub cost: u16,

    /// Message attributes
    pub attr: u16,

    /// Next reply linking
    pub net_attr: u16,
}

/// Message attributes bitflags
#[derive(Debug, Clone, Copy)]
pub struct HudsonAttributes(pub u16);

impl HudsonAttributes {
    /// Message is deleted
    pub const DELETED: u16 = 0x0001;
    /// Message is unmoved
    pub const UNMOVED: u16 = 0x0002;
    /// Message is netmail
    pub const NETMAIL: u16 = 0x0004;
    /// Message is private
    pub const PRIVATE: u16 = 0x0008;
    /// Message has been received
    pub const RECEIVED: u16 = 0x0010;
    /// Message is unread
    pub const UNREAD: u16 = 0x0020;
    /// Message is locked
    pub const LOCKED: u16 = 0x0040;
    /// Message is local
    pub const LOCAL: u16 = 0x0080;

    /// Create new attributes
    pub fn new(value: u16) -> Self {
        Self(value)
    }

    /// Check if attribute is set
    pub fn has(&self, flag: u16) -> bool {
        (self.0 & flag) != 0
    }

    /// Is message private
    pub fn is_private(&self) -> bool {
        self.has(Self::PRIVATE)
    }

    /// Is message deleted
    pub fn is_deleted(&self) -> bool {
        self.has(Self::DELETED)
    }

    /// Is message read (NOT unread)
    pub fn is_read(&self) -> bool {
        !self.has(Self::UNREAD)
    }

    /// Is message local
    pub fn is_local(&self) -> bool {
        self.has(Self::LOCAL)
    }
}

impl HudsonMessageHeader {
    /// Convert DOS date to DateTime
    fn dos_date_to_datetime(date: u16, time: u16) -> Option<DateTime<Utc>> {
        // DOS date: bits 15-9 = year (relative to 1980), 8-5 = month, 4-0 = day
        let year = ((date >> 9) & 0x7F) as i32 + 1980;
        let month = ((date >> 5) & 0x0F) as u32;
        let day = (date & 0x1F) as u32;

        // DOS time: bits 15-11 = hour, 10-5 = minute, 4-0 = second/2
        let hour = ((time >> 11) & 0x1F) as u32;
        let minute = ((time >> 5) & 0x3F) as u32;
        let second = ((time & 0x1F) * 2) as u32;

        NaiveDate::from_ymd_opt(year, month, day)
            .and_then(|d| d.and_hms_opt(hour, minute, second))
            .map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc))
    }

    /// Get written date/time
    pub fn written_datetime(&self) -> Option<DateTime<Utc>> {
        Self::dos_date_to_datetime(self.written_date, self.written_time)
    }

    /// Get received date/time
    pub fn received_datetime(&self) -> Option<DateTime<Utc>> {
        Self::dos_date_to_datetime(self.recd_date, self.recd_time)
    }

    /// Get message attributes
    pub fn attributes(&self) -> HudsonAttributes {
        HudsonAttributes::new(self.attr)
    }

    /// Check if this is a reply
    pub fn is_reply(&self) -> bool {
        self.prev_reply != 0
    }

    /// Check if this has replies
    pub fn has_replies(&self) -> bool {
        self.next_reply != 0
    }
}

/// Hudson message base
pub struct HudsonMessageBase {
    /// Path to message file
    msg_file: PathBuf,
    /// Path to index file - reserved for future use
    #[allow(dead_code)]
    idx_file: PathBuf,
    /// Header cache
    header_cache: Arc<RwLock<HashMap<u32, HudsonMessageHeader>>>,
}

impl HudsonMessageBase {
    /// Block size in Hudson format
    const BLOCK_SIZE: usize = 256;

    /// Create a new Hudson message base
    pub fn new(base_path: impl AsRef<Path>) -> Self {
        let base = base_path.as_ref();
        Self {
            msg_file: base.with_extension("msg"),
            idx_file: base.with_extension("idx"),
            header_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Load a message header
    async fn load_header(&self, msg_num: u32) -> Result<HudsonMessageHeader> {
        // Check cache
        {
            let cache = self.header_cache.read().await;
            if let Some(header) = cache.get(&msg_num) {
                return Ok(header.clone());
            }
        }

        // Calculate file position
        // Hudson stores messages sequentially with 190-byte headers
        let header_size = 190; // Actual Hudson header size
        let position = (msg_num - 1) as u64 * header_size;

        let mut file = File::open(&self.msg_file).await?;
        file.seek(std::io::SeekFrom::Start(position)).await?;

        let mut buffer = vec![0u8; header_size as usize];
        file.read_exact(&mut buffer).await?;

        let mut cursor = std::io::Cursor::new(buffer);
        let header = HudsonMessageHeader::read(&mut cursor)
            .map_err(|e| MessageError::InvalidHeader(e.to_string()))?;

        // Cache it
        {
            let mut cache = self.header_cache.write().await;
            cache.insert(msg_num, header.clone());
        }

        Ok(header)
    }

    /// Load message text
    async fn load_text(&self, header: &HudsonMessageHeader) -> Result<String> {
        let start_pos = header.start_block as u64 * Self::BLOCK_SIZE as u64;
        let length = header.num_blocks as usize * Self::BLOCK_SIZE;

        let mut file = File::open(&self.msg_file).await?;
        file.seek(std::io::SeekFrom::Start(start_pos)).await?;

        let mut buffer = vec![0u8; length];
        file.read_exact(&mut buffer).await?;

        // Hudson text is null-terminated
        if let Some(pos) = buffer.iter().position(|&b| b == 0) {
            buffer.truncate(pos);
        }

        String::from_utf8(buffer).map_err(|e| e.into())
    }

    /// Parse Hudson message text format
    fn parse_text(text: &str) -> (String, String, String, String, Vec<KludgeLine>) {
        // Hudson format:
        // Line 1: From name (null-padded to 36 chars)
        // Line 2: To name (null-padded to 36 chars)
        // Line 3: Subject (null-padded to 72 chars)
        // Line 4+: Message body

        let lines: Vec<&str> = text.lines().collect();
        let from = lines
            .first()
            .unwrap_or(&"")
            .trim_end_matches('\0')
            .to_string();
        let to = lines
            .get(1)
            .unwrap_or(&"")
            .trim_end_matches('\0')
            .to_string();
        let subject = lines
            .get(2)
            .unwrap_or(&"")
            .trim_end_matches('\0')
            .to_string();

        let body = if lines.len() > 3 {
            lines[3..].join("\n")
        } else {
            String::new()
        };

        // Hudson doesn't have kludges in the same way, but we can parse them
        let kludges = Vec::new(); // Simplified for now

        (from, to, subject, body, kludges)
    }
}

#[async_trait]
impl MessageBase for HudsonMessageBase {
    async fn read_message(&self, msg_num: u32) -> Result<FullMessage> {
        let header = self.load_header(msg_num).await?;

        if header.attributes().is_deleted() {
            return Err(MessageError::MessageNotFound(msg_num));
        }

        let text = self.load_text(&header).await?;
        let (from, to, subject, body, kludges) = Self::parse_text(&text);

        let msg_header = MessageHeader {
            msg_num,
            from,
            to,
            subject,
            date: header.written_datetime().unwrap_or_else(Utc::now),
            is_read: header.attributes().is_read(),
            is_private: header.attributes().is_private(),
            reply_to: if header.prev_reply != 0 {
                Some(header.prev_reply as u32)
            } else {
                None
            },
            reply_count: if header.next_reply != 0 { 1 } else { 0 },
        };

        Ok(FullMessage {
            header: msg_header,
            body,
            kludges,
        })
    }

    async fn message_count(&self) -> Result<u32> {
        let metadata = tokio::fs::metadata(&self.msg_file).await?;
        let header_size = 190u64;
        Ok((metadata.len() / header_size) as u32)
    }

    async fn search(&self, criteria: &SearchCriteria) -> Result<Vec<u32>> {
        let count = self.message_count().await?;
        let mut results = Vec::new();

        for msg_num in 1..=count {
            match self.read_message(msg_num).await {
                Ok(msg) => {
                    let mut matches = true;

                    if let Some(ref subject) = criteria.subject
                        && !msg
                            .header
                            .subject
                            .to_lowercase()
                            .contains(&subject.to_lowercase())
                    {
                        matches = false;
                    }

                    if let Some(ref from) = criteria.from
                        && !msg
                            .header
                            .from
                            .to_lowercase()
                            .contains(&from.to_lowercase())
                    {
                        matches = false;
                    }

                    if let Some(ref to) = criteria.to
                        && !msg.header.to.to_lowercase().contains(&to.to_lowercase())
                    {
                        matches = false;
                    }

                    if let Some(ref body_search) = criteria.body
                        && !msg
                            .body
                            .to_lowercase()
                            .contains(&body_search.to_lowercase())
                    {
                        matches = false;
                    }

                    if criteria.unread_only && msg.header.is_read {
                        matches = false;
                    }

                    if criteria.private_only && !msg.header.is_private {
                        matches = false;
                    }

                    if matches {
                        results.push(msg_num);
                    }
                }
                Err(_) => continue,
            }
        }

        Ok(results)
    }

    async fn get_thread(&self, msg_num: u32) -> Result<MessageThread> {
        let header = self.load_header(msg_num).await?;
        let mut thread = MessageThread::new(msg_num);

        thread.parent_id = if header.prev_reply != 0 {
            Some(header.prev_reply as u32)
        } else {
            None
        };

        if header.next_reply != 0 {
            thread.add_child(header.next_reply as u32);
        }

        Ok(thread)
    }

    async fn list_messages(&self, start: u32, count: u32) -> Result<Vec<MessageHeader>> {
        let total = self.message_count().await?;
        let end = std::cmp::min(start + count - 1, total);
        let mut headers = Vec::new();

        for msg_num in start..=end {
            match self.read_message(msg_num).await {
                Ok(msg) => headers.push(msg.header),
                Err(_) => continue,
            }
        }

        Ok(headers)
    }

    async fn get_stats(&self) -> Result<MessageBaseStats> {
        let total = self.message_count().await?;
        Ok(MessageBaseStats {
            total_messages: total,
            unread_messages: 0,
            oldest_message: None,
            newest_message: None,
            total_size: tokio::fs::metadata(&self.msg_file).await?.len(),
        })
    }

    async fn mark_read(&mut self, _msg_num: u32) -> Result<()> {
        // Would update the header in the file
        Ok(())
    }

    async fn message_exists(&self, msg_num: u32) -> Result<bool> {
        let total = self.message_count().await?;
        Ok(msg_num >= 1 && msg_num <= total)
    }

    async fn get_message_range(&self) -> Result<(u32, u32)> {
        let total = self.message_count().await?;
        Ok((1, total))
    }

    async fn post_message(&mut self, _message: NewMessage) -> Result<u32> {
        // TODO: Implement Hudson message writing in future sprint
        Err(MessageError::WriteError(
            "Hudson write support not yet implemented".to_string(),
        ))
    }

    async fn reply_to_message(
        &mut self,
        _parent_msg_num: u32,
        _message: NewMessage,
    ) -> Result<u32> {
        // TODO: Implement Hudson reply writing in future sprint
        Err(MessageError::WriteError(
            "Hudson write support not yet implemented".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hudson_attributes() {
        let attrs = HudsonAttributes::new(HudsonAttributes::PRIVATE | HudsonAttributes::LOCAL);

        assert!(attrs.is_private());
        assert!(attrs.is_local());
        assert!(!attrs.is_deleted());
    }

    #[test]
    fn test_dos_date_conversion() {
        // Date: 1990-01-15 (10 years after 1980 = year bits)
        // year=10 (bits 15-9), month=1 (bits 8-5), day=15 (bits 4-0)
        let date = (10 << 9) | (1 << 5) | 15;
        // Time: 14:30:00
        // hour=14 (bits 15-11), minute=30 (bits 10-5), second/2=0 (bits 4-0)
        let time = (14 << 11) | (30 << 5);

        let datetime = HudsonMessageHeader::dos_date_to_datetime(date, time);
        assert!(datetime.is_some());
    }
}
