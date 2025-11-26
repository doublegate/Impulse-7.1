//! Audit logging for administrative actions

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// An entry in the administrative audit log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    /// ID of the administrator who performed the action
    pub admin_user_id: i32,
    /// Action performed (e.g., "edit_user", "delete_file_area")
    pub action: String,
    /// Target of the action (e.g., user ID, file area ID)
    pub target: Option<String>,
    /// Additional details about the action
    pub details: Option<String>,
    /// Timestamp when the action was performed
    pub timestamp: DateTime<Utc>,
}

/// Logger for administrative actions
///
/// In production, this would integrate with a database or persistent log file.
/// This implementation uses in-memory storage for testing purposes.
#[derive(Debug, Clone)]
pub struct AuditLogger {
    entries: Arc<RwLock<Vec<AuditEntry>>>,
}

impl AuditLogger {
    /// Creates a new audit logger
    pub fn new() -> Self {
        Self {
            entries: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Logs an administrative action
    ///
    /// # Arguments
    /// * `admin_user_id` - ID of the administrator performing the action
    /// * `action` - Name of the action (e.g., "edit_user")
    /// * `target` - Optional target of the action (e.g., user ID)
    /// * `details` - Optional additional details
    pub async fn log_action(
        &self,
        admin_user_id: i32,
        action: impl Into<String>,
        target: Option<impl Into<String>>,
        details: Option<impl Into<String>>,
    ) {
        let entry = AuditEntry {
            admin_user_id,
            action: action.into(),
            target: target.map(|t| t.into()),
            details: details.map(|d| d.into()),
            timestamp: Utc::now(),
        };

        tracing::info!(
            admin_user_id = admin_user_id,
            action = %entry.action,
            target = ?entry.target,
            "Admin action logged"
        );

        let mut entries = self.entries.write().await;
        entries.push(entry);
    }

    /// Retrieves all audit log entries
    pub async fn get_all_entries(&self) -> Vec<AuditEntry> {
        let entries = self.entries.read().await;
        entries.clone()
    }

    /// Retrieves audit log entries for a specific administrator
    pub async fn get_entries_by_admin(&self, admin_user_id: i32) -> Vec<AuditEntry> {
        let entries = self.entries.read().await;
        entries
            .iter()
            .filter(|e| e.admin_user_id == admin_user_id)
            .cloned()
            .collect()
    }

    /// Retrieves audit log entries for a specific action type
    pub async fn get_entries_by_action(&self, action: &str) -> Vec<AuditEntry> {
        let entries = self.entries.read().await;
        entries
            .iter()
            .filter(|e| e.action == action)
            .cloned()
            .collect()
    }

    /// Retrieves the most recent N audit log entries
    pub async fn get_recent_entries(&self, limit: usize) -> Vec<AuditEntry> {
        let entries = self.entries.read().await;
        entries.iter().rev().take(limit).cloned().collect()
    }

    /// Returns the total number of audit log entries
    pub async fn count(&self) -> usize {
        let entries = self.entries.read().await;
        entries.len()
    }

    /// Clears all audit log entries (for testing only)
    #[cfg(test)]
    pub async fn clear(&self) {
        let mut entries = self.entries.write().await;
        entries.clear();
    }
}

impl Default for AuditLogger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_log_action() {
        let logger = AuditLogger::new();
        logger
            .log_action(1, "test_action", Some("target1"), Some("details1"))
            .await;

        let entries = logger.get_all_entries().await;
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].admin_user_id, 1);
        assert_eq!(entries[0].action, "test_action");
        assert_eq!(entries[0].target, Some("target1".to_string()));
        assert_eq!(entries[0].details, Some("details1".to_string()));
    }

    #[tokio::test]
    async fn test_log_action_no_target() {
        let logger = AuditLogger::new();
        logger
            .log_action(2, "broadcast", None::<String>, Some("message sent"))
            .await;

        let entries = logger.get_all_entries().await;
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].target, None);
    }

    #[tokio::test]
    async fn test_get_entries_by_admin() {
        let logger = AuditLogger::new();
        logger
            .log_action(1, "action1", None::<String>, None::<String>)
            .await;
        logger
            .log_action(2, "action2", None::<String>, None::<String>)
            .await;
        logger
            .log_action(1, "action3", None::<String>, None::<String>)
            .await;

        let admin1_entries = logger.get_entries_by_admin(1).await;
        assert_eq!(admin1_entries.len(), 2);
        assert!(admin1_entries.iter().all(|e| e.admin_user_id == 1));

        let admin2_entries = logger.get_entries_by_admin(2).await;
        assert_eq!(admin2_entries.len(), 1);
        assert_eq!(admin2_entries[0].admin_user_id, 2);
    }

    #[tokio::test]
    async fn test_get_entries_by_action() {
        let logger = AuditLogger::new();
        logger
            .log_action(1, "edit_user", None::<String>, None::<String>)
            .await;
        logger
            .log_action(1, "delete_user", None::<String>, None::<String>)
            .await;
        logger
            .log_action(2, "edit_user", None::<String>, None::<String>)
            .await;

        let edit_entries = logger.get_entries_by_action("edit_user").await;
        assert_eq!(edit_entries.len(), 2);
        assert!(edit_entries.iter().all(|e| e.action == "edit_user"));
    }

    #[tokio::test]
    async fn test_get_recent_entries() {
        let logger = AuditLogger::new();
        for i in 1..=5 {
            logger
                .log_action(1, format!("action{}", i), None::<String>, None::<String>)
                .await;
        }

        let recent = logger.get_recent_entries(3).await;
        assert_eq!(recent.len(), 3);
        // Most recent should be first
        assert_eq!(recent[0].action, "action5");
        assert_eq!(recent[1].action, "action4");
        assert_eq!(recent[2].action, "action3");
    }

    #[tokio::test]
    async fn test_count() {
        let logger = AuditLogger::new();
        assert_eq!(logger.count().await, 0);

        logger
            .log_action(1, "action1", None::<String>, None::<String>)
            .await;
        assert_eq!(logger.count().await, 1);

        logger
            .log_action(1, "action2", None::<String>, None::<String>)
            .await;
        assert_eq!(logger.count().await, 2);
    }

    #[tokio::test]
    async fn test_clear() {
        let logger = AuditLogger::new();
        logger
            .log_action(1, "action1", None::<String>, None::<String>)
            .await;
        assert_eq!(logger.count().await, 1);

        logger.clear().await;
        assert_eq!(logger.count().await, 0);
    }

    #[tokio::test]
    async fn test_timestamp_ordering() {
        let logger = AuditLogger::new();
        logger
            .log_action(1, "first", None::<String>, None::<String>)
            .await;
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        logger
            .log_action(1, "second", None::<String>, None::<String>)
            .await;

        let entries = logger.get_all_entries().await;
        assert!(entries[0].timestamp < entries[1].timestamp);
    }

    #[test]
    fn test_audit_entry_serialization() {
        let entry = AuditEntry {
            admin_user_id: 1,
            action: "test".to_string(),
            target: Some("user:42".to_string()),
            details: Some("test details".to_string()),
            timestamp: Utc::now(),
        };

        let json = serde_json::to_string(&entry).unwrap();
        let deserialized: AuditEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(entry.admin_user_id, deserialized.admin_user_id);
        assert_eq!(entry.action, deserialized.action);
    }
}
