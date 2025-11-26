//! Message export functionality
//!
//! Export messages to various formats including text, JSON, and CSV.

use crate::MessageError;
use crate::types::FullMessage;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

/// Export format options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    /// Plain text format
    Text,
    /// JSON format
    Json,
    /// CSV format
    Csv,
}

/// Message exporter
pub struct MessageExporter {
    format: ExportFormat,
}

impl MessageExporter {
    /// Create a new message exporter
    pub fn new(format: ExportFormat) -> Self {
        Self { format }
    }

    /// Export messages to a file
    pub async fn export_to_file<P: AsRef<Path>>(
        &self,
        messages: &[FullMessage],
        path: P,
    ) -> crate::Result<()> {
        let content = match self.format {
            ExportFormat::Text => self.export_as_text(messages),
            ExportFormat::Json => self.export_as_json(messages)?,
            ExportFormat::Csv => self.export_as_csv(messages)?,
        };

        let mut file = File::create(path).await?;
        file.write_all(content.as_bytes()).await?;
        file.flush().await?;

        Ok(())
    }

    /// Export messages as plain text
    fn export_as_text(&self, messages: &[FullMessage]) -> String {
        let mut output = String::new();

        for (idx, msg) in messages.iter().enumerate() {
            output.push_str(&format!("=== Message {} ===\n", idx + 1));
            output.push_str(&format!("From: {}\n", msg.header.from));
            output.push_str(&format!("To: {}\n", msg.header.to));
            output.push_str(&format!("Subject: {}\n", msg.header.subject));
            output.push_str(&format!("Date: {}\n", msg.header.date));
            output.push_str(&format!("Private: {}\n", msg.header.is_private));

            if let Some(reply_to) = msg.header.reply_to {
                output.push_str(&format!("Reply To: {}\n", reply_to));
            }

            output.push('\n');
            output.push_str(&msg.body);
            output.push_str("\n\n");
        }

        output
    }

    /// Export messages as JSON
    fn export_as_json(&self, messages: &[FullMessage]) -> crate::Result<String> {
        serde_json::to_string_pretty(messages)
            .map_err(|e| MessageError::Serialization(e.to_string()))
    }

    /// Export messages as CSV
    fn export_as_csv(&self, messages: &[FullMessage]) -> crate::Result<String> {
        let mut wtr = csv::Writer::from_writer(Vec::new());

        // Write header
        wtr.write_record([
            "Number", "From", "To", "Subject", "Date", "Private", "Reply To", "Body",
        ])
        .map_err(|e| MessageError::Serialization(e.to_string()))?;

        // Write records
        for msg in messages {
            wtr.write_record([
                msg.header.msg_num.to_string(),
                msg.header.from.clone(),
                msg.header.to.clone(),
                msg.header.subject.clone(),
                msg.header.date.to_string(),
                msg.header.is_private.to_string(),
                msg.header
                    .reply_to
                    .map(|n| n.to_string())
                    .unwrap_or_default(),
                msg.body.clone(),
            ])
            .map_err(|e| MessageError::Serialization(e.to_string()))?;
        }

        let data = wtr
            .into_inner()
            .map_err(|e| MessageError::Serialization(e.to_string()))?;
        String::from_utf8(data).map_err(|e| MessageError::Serialization(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

    #[test]
    fn test_export_as_text() {
        let messages = vec![create_test_message(1)];
        let exporter = MessageExporter::new(ExportFormat::Text);
        let text = exporter.export_as_text(&messages);

        assert!(text.contains("From: User1"));
        assert!(text.contains("To: All"));
        assert!(text.contains("Subject: Subject 1"));
        assert!(text.contains("This is message body 1."));
    }

    #[test]
    fn test_export_as_json() {
        let messages = vec![create_test_message(1), create_test_message(2)];
        let exporter = MessageExporter::new(ExportFormat::Json);
        let json = exporter.export_as_json(&messages).unwrap();

        assert!(json.contains("User1"));
        assert!(json.contains("User2"));
        assert!(json.contains("Subject 1"));
        assert!(json.contains("Subject 2"));
    }

    #[test]
    fn test_export_as_csv() {
        let messages = vec![create_test_message(1), create_test_message(2)];
        let exporter = MessageExporter::new(ExportFormat::Csv);
        let csv = exporter.export_as_csv(&messages).unwrap();

        assert!(csv.contains("Number,From,To,Subject"));
        assert!(csv.contains("User1"));
        assert!(csv.contains("User2"));
    }

    #[tokio::test]
    async fn test_export_to_file_text() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("export.txt");

        let messages = vec![create_test_message(1)];
        let exporter = MessageExporter::new(ExportFormat::Text);
        exporter
            .export_to_file(&messages, &file_path)
            .await
            .unwrap();

        let content = tokio::fs::read_to_string(&file_path).await.unwrap();
        assert!(content.contains("From: User1"));
    }

    #[tokio::test]
    async fn test_export_to_file_json() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("export.json");

        let messages = vec![create_test_message(1), create_test_message(2)];
        let exporter = MessageExporter::new(ExportFormat::Json);
        exporter
            .export_to_file(&messages, &file_path)
            .await
            .unwrap();

        let content = tokio::fs::read_to_string(&file_path).await.unwrap();
        let parsed: Vec<FullMessage> = serde_json::from_str(&content).unwrap();
        assert_eq!(parsed.len(), 2);
    }

    #[tokio::test]
    async fn test_export_to_file_csv() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("export.csv");

        let messages = vec![create_test_message(1), create_test_message(2)];
        let exporter = MessageExporter::new(ExportFormat::Csv);
        exporter
            .export_to_file(&messages, &file_path)
            .await
            .unwrap();

        let content = tokio::fs::read_to_string(&file_path).await.unwrap();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines.len(), 3); // Header + 2 messages
    }

    #[test]
    fn test_export_empty_messages() {
        let messages: Vec<FullMessage> = Vec::new();
        let exporter = MessageExporter::new(ExportFormat::Text);
        let text = exporter.export_as_text(&messages);
        assert_eq!(text, "");
    }

    #[test]
    fn test_export_message_with_reply() {
        let mut msg = create_test_message(1);
        msg.header.reply_to = Some(42);

        let messages = vec![msg];
        let exporter = MessageExporter::new(ExportFormat::Text);
        let text = exporter.export_as_text(&messages);

        assert!(text.contains("Reply To: 42"));
    }

    #[test]
    fn test_export_private_message() {
        let mut msg = create_test_message(1);
        msg.header.is_private = true;

        let messages = vec![msg];
        let exporter = MessageExporter::new(ExportFormat::Csv);
        let csv = exporter.export_as_csv(&messages).unwrap();

        assert!(csv.contains("true"));
    }
}
