//! SysOp notification system for virus detections

use chrono::Utc;
use std::path::Path;

/// Virus detection notification
#[derive(Debug, Clone)]
pub struct VirusNotification {
    /// Timestamp of detection
    pub timestamp: chrono::DateTime<chrono::Utc>,

    /// Username of uploader
    pub uploader: String,

    /// User ID
    pub uploader_id: u32,

    /// Original filename
    pub filename: String,

    /// Detected threat name
    pub threat_name: String,

    /// File area ID
    pub area_id: u32,

    /// Quarantine path (if quarantined)
    pub quarantine_path: Option<String>,
}

impl VirusNotification {
    /// Create a new virus notification
    pub fn new(
        uploader: String,
        uploader_id: u32,
        filename: String,
        threat_name: String,
        area_id: u32,
    ) -> Self {
        Self {
            timestamp: Utc::now(),
            uploader,
            uploader_id,
            filename,
            threat_name,
            area_id,
            quarantine_path: None,
        }
    }

    /// Set quarantine path
    pub fn with_quarantine_path(mut self, path: impl AsRef<Path>) -> Self {
        self.quarantine_path = Some(path.as_ref().to_string_lossy().to_string());
        self
    }

    /// Format notification message
    pub fn format_message(&self) -> String {
        let mut msg = format!(
            "VIRUS DETECTED!\n\n\
             Timestamp: {}\n\
             Uploader: {} (ID: {})\n\
             Filename: {}\n\
             Threat: {}\n\
             Area ID: {}",
            self.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
            self.uploader,
            self.uploader_id,
            self.filename,
            self.threat_name,
            self.area_id
        );

        if let Some(ref quar_path) = self.quarantine_path {
            msg.push_str(&format!("\nQuarantined: {}", quar_path));
        }

        msg
    }

    /// Format short summary
    pub fn format_summary(&self) -> String {
        format!(
            "[{}] {} uploaded by {} - {}",
            self.timestamp.format("%Y-%m-%d %H:%M"),
            self.filename,
            self.uploader,
            self.threat_name
        )
    }
}

/// Notification sender trait
///
/// Allows different notification backends (email, system message, etc.)
#[async_trait::async_trait]
pub trait NotificationSender: Send + Sync {
    /// Send a virus detection notification
    async fn send_notification(&self, notification: &VirusNotification) -> Result<(), String>;
}

/// Simple logger-based notification sender
pub struct LogNotificationSender;

impl LogNotificationSender {
    /// Create a new log-based notification sender
    pub fn new() -> Self {
        Self
    }
}

impl Default for LogNotificationSender {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl NotificationSender for LogNotificationSender {
    async fn send_notification(&self, notification: &VirusNotification) -> Result<(), String> {
        // In a real implementation, this would use the logging system
        eprintln!("{}", notification.format_message());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_virus_notification_new() {
        let notification = VirusNotification::new(
            "testuser".to_string(),
            42,
            "malware.exe".to_string(),
            "Trojan.Generic".to_string(),
            1,
        );

        assert_eq!(notification.uploader, "testuser");
        assert_eq!(notification.uploader_id, 42);
        assert_eq!(notification.filename, "malware.exe");
        assert_eq!(notification.threat_name, "Trojan.Generic");
        assert_eq!(notification.area_id, 1);
        assert!(notification.quarantine_path.is_none());
    }

    #[test]
    fn test_virus_notification_with_quarantine() {
        let notification = VirusNotification::new(
            "testuser".to_string(),
            42,
            "malware.exe".to_string(),
            "Trojan.Generic".to_string(),
            1,
        )
        .with_quarantine_path("/quarantine/malware.exe.20250125_120000");

        assert!(notification.quarantine_path.is_some());
        assert!(notification.quarantine_path.unwrap().contains("quarantine"));
    }

    #[test]
    fn test_format_message() {
        let notification = VirusNotification::new(
            "testuser".to_string(),
            42,
            "malware.exe".to_string(),
            "Trojan.Generic".to_string(),
            1,
        );

        let message = notification.format_message();

        assert!(message.contains("VIRUS DETECTED"));
        assert!(message.contains("testuser"));
        assert!(message.contains("malware.exe"));
        assert!(message.contains("Trojan.Generic"));
    }

    #[test]
    fn test_format_summary() {
        let notification = VirusNotification::new(
            "testuser".to_string(),
            42,
            "malware.exe".to_string(),
            "Trojan.Generic".to_string(),
            1,
        );

        let summary = notification.format_summary();

        assert!(summary.contains("malware.exe"));
        assert!(summary.contains("testuser"));
        assert!(summary.contains("Trojan.Generic"));
    }

    #[tokio::test]
    async fn test_log_notification_sender() {
        let sender = LogNotificationSender::new();
        let notification = VirusNotification::new(
            "testuser".to_string(),
            42,
            "test.exe".to_string(),
            "Test.Virus".to_string(),
            1,
        );

        let result = sender.send_notification(&notification).await;
        assert!(result.is_ok());
    }
}
