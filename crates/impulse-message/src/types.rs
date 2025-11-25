//! Message base types and structures

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Message header information (lightweight for list display)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageHeader {
    /// Message number
    pub msg_num: u32,
    /// Sender username
    pub from: String,
    /// Recipient username (or "All" for public)
    pub to: String,
    /// Message subject
    pub subject: String,
    /// Date/time posted
    pub date: DateTime<Utc>,
    /// Whether message has been read
    pub is_read: bool,
    /// Whether message is private
    pub is_private: bool,
    /// Parent message number (if reply)
    pub reply_to: Option<u32>,
    /// Number of replies to this message
    pub reply_count: u32,
}

/// Complete message with body
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullMessage {
    /// Message header
    pub header: MessageHeader,
    /// Message body text
    pub body: String,
    /// Kludge lines (control information)
    pub kludges: Vec<KludgeLine>,
}

/// Thread information for a message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageThread {
    /// Root message of thread
    pub root_msg: u32,
    /// Parent message number
    pub parent_id: Option<u32>,
    /// Direct replies to this message
    pub children: Vec<u32>,
    /// Total replies in thread
    pub reply_count: u32,
    /// Thread depth (0 = root)
    pub depth: u32,
    /// Thread path (list of message numbers from root to this message)
    pub path: Vec<u32>,
}

/// Kludge line (control information in message)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KludgeLine {
    /// Kludge type (e.g., "MSGID", "REPLY", "SEEN-BY")
    pub kludge_type: String,
    /// Kludge value
    pub value: String,
}

/// Search criteria for finding messages
#[derive(Debug, Clone, Default)]
pub struct SearchCriteria {
    /// Search in subject
    pub subject: Option<String>,
    /// Search in from field
    pub from: Option<String>,
    /// Search in to field
    pub to: Option<String>,
    /// Search in message body
    pub body: Option<String>,
    /// Search from date
    pub date_from: Option<DateTime<Utc>>,
    /// Search to date
    pub date_to: Option<DateTime<Utc>>,
    /// Only unread messages
    pub unread_only: bool,
    /// Only private messages
    pub private_only: bool,
}

/// Message base statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageBaseStats {
    /// Total number of messages
    pub total_messages: u32,
    /// Number of unread messages
    pub unread_messages: u32,
    /// Date of oldest message
    pub oldest_message: Option<DateTime<Utc>>,
    /// Date of newest message
    pub newest_message: Option<DateTime<Utc>>,
    /// Total size in bytes
    pub total_size: u64,
}

impl MessageHeader {
    /// Check if this is a public message
    pub fn is_public(&self) -> bool {
        self.to.eq_ignore_ascii_case("All")
    }

    /// Check if this is a reply
    pub fn is_reply(&self) -> bool {
        self.reply_to.is_some()
    }

    /// Check if this has replies
    pub fn has_replies(&self) -> bool {
        self.reply_count > 0
    }
}

impl FullMessage {
    /// Get message header
    pub fn header(&self) -> &MessageHeader {
        &self.header
    }

    /// Get message body
    pub fn body(&self) -> &str {
        &self.body
    }

    /// Get kludge line by type
    pub fn get_kludge(&self, kludge_type: &str) -> Option<&KludgeLine> {
        self.kludges
            .iter()
            .find(|k| k.kludge_type.eq_ignore_ascii_case(kludge_type))
    }

    /// Get all kludge lines of a type
    pub fn get_kludges(&self, kludge_type: &str) -> Vec<&KludgeLine> {
        self.kludges
            .iter()
            .filter(|k| k.kludge_type.eq_ignore_ascii_case(kludge_type))
            .collect()
    }
}

impl MessageThread {
    /// Create a new thread
    pub fn new(root_msg: u32) -> Self {
        Self {
            root_msg,
            parent_id: None,
            children: Vec::new(),
            reply_count: 0,
            depth: 0,
            path: vec![root_msg],
        }
    }

    /// Check if this is a root message
    pub fn is_root(&self) -> bool {
        self.parent_id.is_none()
    }

    /// Add a child message
    pub fn add_child(&mut self, child_msg: u32) {
        if !self.children.contains(&child_msg) {
            self.children.push(child_msg);
            self.reply_count += 1;
        }
    }
}

impl SearchCriteria {
    /// Create new empty search criteria
    pub fn new() -> Self {
        Self::default()
    }

    /// Set subject search
    pub fn with_subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into());
        self
    }

    /// Set from search
    pub fn with_from(mut self, from: impl Into<String>) -> Self {
        self.from = Some(from.into());
        self
    }

    /// Set to search
    pub fn with_to(mut self, to: impl Into<String>) -> Self {
        self.to = Some(to.into());
        self
    }

    /// Set body search
    pub fn with_body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// Set date range
    pub fn with_date_range(mut self, from: DateTime<Utc>, to: DateTime<Utc>) -> Self {
        self.date_from = Some(from);
        self.date_to = Some(to);
        self
    }

    /// Set unread only filter
    pub fn unread_only(mut self) -> Self {
        self.unread_only = true;
        self
    }

    /// Set private only filter
    pub fn private_only(mut self) -> Self {
        self.private_only = true;
        self
    }
}
