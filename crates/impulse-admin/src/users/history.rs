//! User login history viewing functionality

use super::UserManager;
use crate::access::AdminPermission;
use crate::error::AdminResult;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// A login history entry for a user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginHistoryEntry {
    pub user_id: i32,
    pub login_time: DateTime<Utc>,
    pub logout_time: Option<DateTime<Utc>>,
    pub ip_address: String,
    pub session_duration_minutes: Option<i32>,
}

/// In-memory login history storage (in production, this would be a database)
type HistoryStore = Arc<RwLock<Vec<LoginHistoryEntry>>>;

/// Login history manager (normally part of UserManager, but separated for testing)
#[derive(Debug, Clone)]
pub struct LoginHistoryManager {
    history: HistoryStore,
}

impl LoginHistoryManager {
    /// Creates a new login history manager
    pub fn new() -> Self {
        Self {
            history: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Adds a login history entry (for testing)
    #[cfg(test)]
    pub async fn add_entry(&self, entry: LoginHistoryEntry) {
        let mut history = self.history.write().await;
        history.push(entry);
    }

    /// Gets login history for a specific user
    pub async fn get_history(&self, user_id: i32, limit: usize) -> Vec<LoginHistoryEntry> {
        let history = self.history.read().await;
        history
            .iter()
            .filter(|e| e.user_id == user_id)
            .rev()
            .take(limit)
            .cloned()
            .collect()
    }
}

impl Default for LoginHistoryManager {
    fn default() -> Self {
        Self::new()
    }
}

impl UserManager {
    /// Views login history for a specific user
    ///
    /// # Arguments
    /// * `admin_user_id` - ID of the administrator requesting the history
    /// * `user_id` - ID of the user whose history to view
    /// * `limit` - Maximum number of history entries to return
    pub async fn view_login_history(
        &self,
        admin_user_id: i32,
        user_id: i32,
        limit: usize,
    ) -> AdminResult<Vec<LoginHistoryEntry>> {
        self.access_control
            .require_permission(AdminPermission::ViewUsers)?;

        // In production, this would query a database
        // For now, return an empty vector
        let history = Vec::new();

        self.audit
            .log_action(
                admin_user_id,
                "view_login_history",
                Some(user_id.to_string()),
                Some(format!("limit={}", limit)),
            )
            .await;

        Ok(history)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::access::AdminAccessControl;
    use crate::audit::AuditLogger;
    use crate::users::UserRecord;

    fn create_test_user(id: i32, username: &str, security_level: u8) -> UserRecord {
        UserRecord {
            id,
            username: username.to_string(),
            email: Some(format!("{}@example.com", username)),
            security_level,
            last_login: Some(Utc::now()),
            login_count: 10,
            is_banned: false,
            ban_reason: None,
            is_deleted: false,
            time_limit_minutes: 60,
            upload_kb_total: 1024,
            download_kb_total: 2048,
            created_at: Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_login_history_manager_creation() {
        let manager = LoginHistoryManager::new();
        let history = manager.get_history(1, 10).await;
        assert_eq!(history.len(), 0);
    }

    #[tokio::test]
    async fn test_add_and_get_history() {
        let manager = LoginHistoryManager::new();

        let entry = LoginHistoryEntry {
            user_id: 1,
            login_time: Utc::now(),
            logout_time: None,
            ip_address: "192.168.1.1".to_string(),
            session_duration_minutes: None,
        };

        manager.add_entry(entry).await;

        let history = manager.get_history(1, 10).await;
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].user_id, 1);
        assert_eq!(history[0].ip_address, "192.168.1.1");
    }

    #[tokio::test]
    async fn test_get_history_for_user() {
        let manager = LoginHistoryManager::new();

        let entry1 = LoginHistoryEntry {
            user_id: 1,
            login_time: Utc::now(),
            logout_time: None,
            ip_address: "192.168.1.1".to_string(),
            session_duration_minutes: None,
        };

        let entry2 = LoginHistoryEntry {
            user_id: 2,
            login_time: Utc::now(),
            logout_time: None,
            ip_address: "192.168.1.2".to_string(),
            session_duration_minutes: None,
        };

        manager.add_entry(entry1).await;
        manager.add_entry(entry2).await;

        let history_user1 = manager.get_history(1, 10).await;
        assert_eq!(history_user1.len(), 1);
        assert_eq!(history_user1[0].user_id, 1);

        let history_user2 = manager.get_history(2, 10).await;
        assert_eq!(history_user2.len(), 1);
        assert_eq!(history_user2[0].user_id, 2);
    }

    #[tokio::test]
    async fn test_get_history_limit() {
        let manager = LoginHistoryManager::new();

        for i in 1..=5 {
            let entry = LoginHistoryEntry {
                user_id: 1,
                login_time: Utc::now(),
                logout_time: None,
                ip_address: format!("192.168.1.{}", i),
                session_duration_minutes: None,
            };
            manager.add_entry(entry).await;
        }

        let history = manager.get_history(1, 3).await;
        assert_eq!(history.len(), 3);
    }

    #[tokio::test]
    async fn test_get_history_reverse_order() {
        let manager = LoginHistoryManager::new();

        let entry1 = LoginHistoryEntry {
            user_id: 1,
            login_time: Utc::now(),
            logout_time: None,
            ip_address: "first".to_string(),
            session_duration_minutes: None,
        };

        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        let entry2 = LoginHistoryEntry {
            user_id: 1,
            login_time: Utc::now(),
            logout_time: None,
            ip_address: "second".to_string(),
            session_duration_minutes: None,
        };

        manager.add_entry(entry1).await;
        manager.add_entry(entry2).await;

        let history = manager.get_history(1, 10).await;
        assert_eq!(history.len(), 2);
        // Most recent should be first
        assert_eq!(history[0].ip_address, "second");
        assert_eq!(history[1].ip_address, "first");
    }

    #[tokio::test]
    async fn test_view_login_history() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let user = create_test_user(1, "testuser", 100);
        let manager = UserManager::with_users(access, audit, vec![user]);

        let result = manager.view_login_history(1, 1, 10).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_view_login_history_permission_denied() {
        let access = AdminAccessControl::new(100, 200);
        let audit = AuditLogger::new();
        let manager = UserManager::new(access, audit);

        let result = manager.view_login_history(1, 1, 10).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_audit_log_view_history() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let user = create_test_user(1, "testuser", 100);
        let manager = UserManager::with_users(access, audit.clone(), vec![user]);

        manager.view_login_history(42, 1, 10).await.unwrap();

        let entries = audit.get_entries_by_action("view_login_history").await;
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].admin_user_id, 42);
        assert_eq!(entries[0].target, Some("1".to_string()));
    }

    #[test]
    fn test_login_history_entry_serialization() {
        let entry = LoginHistoryEntry {
            user_id: 1,
            login_time: Utc::now(),
            logout_time: None,
            ip_address: "192.168.1.1".to_string(),
            session_duration_minutes: Some(30),
        };

        let json = serde_json::to_string(&entry).unwrap();
        let deserialized: LoginHistoryEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(entry.user_id, deserialized.user_id);
        assert_eq!(entry.ip_address, deserialized.ip_address);
    }
}
