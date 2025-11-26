//! Message import functionality
//!
//! Import messages from various formats including text and JSON.

use crate::MessageError;
use crate::types::{FullMessage, MessageHeader};
use chrono::NaiveDateTime;
use std::path::Path;
use tokio::fs;

/// Import format options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportFormat {
    /// Plain text format
    Text,
    /// JSON format
    Json,
}

/// Message importer
pub struct MessageImporter {
    format: ImportFormat,
}

impl MessageImporter {
    /// Create a new message importer
    pub fn new(format: ImportFormat) -> Self {
        Self { format }
    }

    /// Import messages from a file
    pub async fn import_from_file<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> crate::Result<Vec<FullMessage>> {
        let content = fs::read_to_string(path).await?;

        match self.format {
            ImportFormat::Text => self.import_from_text(&content),
            ImportFormat::Json => self.import_from_json(&content),
        }
    }

    /// Import messages from plain text
    fn import_from_text(&self, content: &str) -> crate::Result<Vec<FullMessage>> {
        let mut messages = Vec::new();
        let mut current_message: Option<ImportedMessage> = None;

        for line in content.lines() {
            if line.starts_with("=== Message") {
                // Save previous message if any
                if let Some(msg) = current_message.take() {
                    messages.push(msg.into_full_message()?);
                }
                // Start new message
                current_message = Some(ImportedMessage::default());
            } else if let Some(msg) = &mut current_message {
                if let Some(from) = line.strip_prefix("From: ") {
                    msg.from = from.to_string();
                } else if let Some(to) = line.strip_prefix("To: ") {
                    msg.to = to.to_string();
                } else if let Some(subject) = line.strip_prefix("Subject: ") {
                    msg.subject = subject.to_string();
                } else if let Some(date) = line.strip_prefix("Date: ") {
                    msg.date = date.to_string();
                } else if let Some(private) = line.strip_prefix("Private: ") {
                    msg.is_private = private.parse().unwrap_or(false);
                } else if let Some(reply_to) = line.strip_prefix("Reply To: ") {
                    msg.reply_to = reply_to.parse().ok();
                } else if !line.is_empty() && msg.has_required_fields() {
                    // Body content
                    if !msg.body.is_empty() {
                        msg.body.push('\n');
                    }
                    msg.body.push_str(line);
                }
            }
        }

        // Save last message
        if let Some(msg) = current_message {
            messages.push(msg.into_full_message()?);
        }

        Ok(messages)
    }

    /// Import messages from JSON
    fn import_from_json(&self, content: &str) -> crate::Result<Vec<FullMessage>> {
        serde_json::from_str(content).map_err(|e| MessageError::Deserialization(e.to_string()))
    }
}

/// Intermediate structure for importing text messages
#[derive(Default)]
struct ImportedMessage {
    from: String,
    to: String,
    subject: String,
    date: String,
    is_private: bool,
    reply_to: Option<u32>,
    body: String,
}

impl ImportedMessage {
    fn has_required_fields(&self) -> bool {
        !self.from.is_empty() && !self.to.is_empty()
    }

    fn into_full_message(self) -> crate::Result<FullMessage> {
        use chrono::DateTime;

        // Clean up the date string (strip timezone suffix if present)
        let date_str = self.date.trim_end_matches(" UTC").trim();

        // Try to parse as DateTime<Utc> first (RFC3339 format from export)
        let date = if let Ok(dt) = DateTime::parse_from_rfc3339(date_str) {
            dt.with_timezone(&chrono::Utc)
        } else {
            // Fall back to parsing as NaiveDateTime and converting
            let date_time = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S")
                .or_else(|_| {
                    // Try alternative format with fractional seconds
                    NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S%.f")
                })
                .map_err(|e| {
                    MessageError::Deserialization(format!("Invalid date format: {}", e))
                })?;

            date_time.and_utc()
        };

        Ok(FullMessage {
            header: MessageHeader {
                msg_num: 0, // Will be assigned when added to base
                from: self.from,
                to: self.to,
                subject: self.subject,
                date,
                is_read: false,
                is_private: self.is_private,
                reply_to: self.reply_to,
                reply_count: 0,
            },
            body: self.body,
            kludges: Vec::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::export::{ExportFormat, MessageExporter};
    use crate::types::MessageHeader;
    use chrono::NaiveDate;
    use tempfile::TempDir;

    fn create_test_message(num: u32) -> FullMessage {
        let dt = NaiveDate::from_ymd_opt(2025, 11, 26)
            .unwrap()
            .and_hms_opt(14, 30, 0)
            .unwrap()
            .and_utc();

        FullMessage {
            header: MessageHeader {
                msg_num: num,
                from: format!("User{}", num),
                to: "All".to_string(),
                subject: format!("Subject {}", num),
                date: dt,
                is_read: false,
                is_private: false,
                reply_to: None,
                reply_count: 0,
            },
            body: format!("This is message body {}.", num),
            kludges: Vec::new(),
        }
    }

    #[tokio::test]
    async fn test_import_from_json() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("export.json");

        // Export messages
        let original_messages = vec![create_test_message(1), create_test_message(2)];
        let exporter = MessageExporter::new(ExportFormat::Json);
        exporter
            .export_to_file(&original_messages, &file_path)
            .await
            .unwrap();

        // Import messages
        let importer = MessageImporter::new(ImportFormat::Json);
        let imported_messages = importer.import_from_file(&file_path).await.unwrap();

        assert_eq!(imported_messages.len(), 2);
        assert_eq!(imported_messages[0].header.from, "User1");
        assert_eq!(imported_messages[1].header.from, "User2");
    }

    #[tokio::test]
    async fn test_import_from_text() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("export.txt");

        // Export messages
        let original_messages = vec![create_test_message(1), create_test_message(2)];
        let exporter = MessageExporter::new(ExportFormat::Text);
        exporter
            .export_to_file(&original_messages, &file_path)
            .await
            .unwrap();

        // Import messages
        let importer = MessageImporter::new(ImportFormat::Text);
        let imported_messages = importer.import_from_file(&file_path).await.unwrap();

        assert_eq!(imported_messages.len(), 2);
        assert_eq!(imported_messages[0].header.from, "User1");
        assert_eq!(imported_messages[1].header.from, "User2");
    }

    #[test]
    fn test_import_from_json_string() {
        let json = r#"[
            {
                "header": {
                    "msg_num": 1,
                    "from": "Alice",
                    "to": "Bob",
                    "subject": "Test",
                    "date": "2025-11-26T14:30:00Z",
                    "is_read": false,
                    "is_private": false,
                    "reply_to": null,
                    "reply_count": 0
                },
                "body": "Hello",
                "kludges": []
            }
        ]"#;

        let importer = MessageImporter::new(ImportFormat::Json);
        let messages = importer.import_from_json(json).unwrap();

        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].header.from, "Alice");
        assert_eq!(messages[0].header.to, "Bob");
        assert_eq!(messages[0].body, "Hello");
    }

    #[test]
    fn test_import_from_text_string() {
        let text = r#"=== Message 1 ===
From: Alice
To: Bob
Subject: Test
Date: 2025-11-26 14:30:00
Private: false

Hello World
This is a test message.
"#;

        let importer = MessageImporter::new(ImportFormat::Text);
        let messages = importer.import_from_text(text).unwrap();

        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].header.from, "Alice");
        assert_eq!(messages[0].header.to, "Bob");
        assert_eq!(messages[0].header.subject, "Test");
        assert!(messages[0].body.contains("Hello World"));
    }

    #[test]
    fn test_import_text_with_reply() {
        let text = r#"=== Message 1 ===
From: Bob
To: Alice
Subject: Re: Test
Date: 2025-11-26 14:30:00
Private: false
Reply To: 42

This is a reply.
"#;

        let importer = MessageImporter::new(ImportFormat::Text);
        let messages = importer.import_from_text(text).unwrap();

        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].header.reply_to, Some(42));
    }

    #[test]
    fn test_import_text_private_message() {
        let text = r#"=== Message 1 ===
From: Alice
To: Bob
Subject: Secret
Date: 2025-11-26 14:30:00
Private: true

Secret message.
"#;

        let importer = MessageImporter::new(ImportFormat::Text);
        let messages = importer.import_from_text(text).unwrap();

        assert_eq!(messages.len(), 1);
        assert!(messages[0].header.is_private);
    }

    #[test]
    fn test_import_multiple_messages() {
        let text = r#"=== Message 1 ===
From: Alice
To: Bob
Subject: First
Date: 2025-11-26 14:30:00
Private: false

First message.

=== Message 2 ===
From: Bob
To: Alice
Subject: Second
Date: 2025-11-26 15:30:00
Private: false

Second message.
"#;

        let importer = MessageImporter::new(ImportFormat::Text);
        let messages = importer.import_from_text(text).unwrap();

        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0].header.subject, "First");
        assert_eq!(messages[1].header.subject, "Second");
    }

    #[test]
    fn test_import_invalid_json() {
        let invalid_json = "{ invalid json }";
        let importer = MessageImporter::new(ImportFormat::Json);
        let result = importer.import_from_json(invalid_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_import_text_multiline_body() {
        let text = r#"=== Message 1 ===
From: Alice
To: Bob
Subject: Test
Date: 2025-11-26 14:30:00
Private: false

Line 1
Line 2
Line 3
"#;

        let importer = MessageImporter::new(ImportFormat::Text);
        let messages = importer.import_from_text(text).unwrap();

        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].body, "Line 1\nLine 2\nLine 3");
    }

    #[tokio::test]
    async fn test_roundtrip_json() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("roundtrip.json");

        let original = vec![create_test_message(1), create_test_message(2)];

        // Export
        let exporter = MessageExporter::new(ExportFormat::Json);
        exporter
            .export_to_file(&original, &file_path)
            .await
            .unwrap();

        // Import
        let importer = MessageImporter::new(ImportFormat::Json);
        let imported = importer.import_from_file(&file_path).await.unwrap();

        assert_eq!(original.len(), imported.len());
        for (orig, imp) in original.iter().zip(imported.iter()) {
            assert_eq!(orig.header.from, imp.header.from);
            assert_eq!(orig.header.to, imp.header.to);
            assert_eq!(orig.header.subject, imp.header.subject);
            assert_eq!(orig.body, imp.body);
        }
    }
}
