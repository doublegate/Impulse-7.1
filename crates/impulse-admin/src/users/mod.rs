//! User management functionality for administrators

pub mod edit;
pub mod history;
pub mod list;
pub mod remove;

// Re-export commonly used types
pub use edit::UserEditRequest;
pub use history::LoginHistoryEntry;
pub use list::UserSummary;

use crate::access::AdminAccessControl;
use crate::audit::AuditLogger;
use crate::error::{AdminError, AdminResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// In-memory user storage (in production, this would be a database)
type UserStore = Arc<RwLock<Vec<UserRecord>>>;

/// Complete user record stored in the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRecord {
    pub id: i32,
    pub username: String,
    pub email: Option<String>,
    pub security_level: u8,
    pub last_login: Option<DateTime<Utc>>,
    pub login_count: i32,
    pub is_banned: bool,
    pub ban_reason: Option<String>,
    pub is_deleted: bool,
    pub time_limit_minutes: i32,
    pub upload_kb_total: i64,
    pub download_kb_total: i64,
    pub created_at: DateTime<Utc>,
}

/// User manager for administrative operations
#[derive(Debug, Clone)]
pub struct UserManager {
    users: UserStore,
    audit: AuditLogger,
    access_control: AdminAccessControl,
}

impl UserManager {
    /// Creates a new user manager
    pub fn new(access_control: AdminAccessControl, audit: AuditLogger) -> Self {
        Self {
            users: Arc::new(RwLock::new(Vec::new())),
            audit,
            access_control,
        }
    }

    /// Creates a user manager with pre-populated users (for testing)
    pub fn with_users(
        access_control: AdminAccessControl,
        audit: AuditLogger,
        users: Vec<UserRecord>,
    ) -> Self {
        Self {
            users: Arc::new(RwLock::new(users)),
            audit,
            access_control,
        }
    }

    /// Returns a reference to the access control
    pub fn access_control(&self) -> &AdminAccessControl {
        &self.access_control
    }

    /// Returns a reference to the audit logger
    pub fn audit(&self) -> &AuditLogger {
        &self.audit
    }

    /// Adds a user to the manager (for testing)
    #[cfg(test)]
    pub async fn add_user(&self, user: UserRecord) {
        let mut users = self.users.write().await;
        users.push(user);
    }

    /// Gets a user by ID
    pub async fn get_user(&self, user_id: i32) -> AdminResult<UserRecord> {
        let users = self.users.read().await;
        users
            .iter()
            .find(|u| u.id == user_id)
            .cloned()
            .ok_or(AdminError::UserNotFound(user_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::access::AdminPermission;

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
    async fn test_user_manager_creation() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let manager = UserManager::new(access, audit);

        assert!(manager.access_control().has_permission(AdminPermission::ViewUsers));
    }

    #[tokio::test]
    async fn test_get_user() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let user = create_test_user(1, "testuser", 100);
        let manager = UserManager::with_users(access, audit, vec![user.clone()]);

        let result = manager.get_user(1).await;
        assert!(result.is_ok());
        let retrieved = result.unwrap();
        assert_eq!(retrieved.id, 1);
        assert_eq!(retrieved.username, "testuser");
    }

    #[tokio::test]
    async fn test_get_user_not_found() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let manager = UserManager::new(access, audit);

        let result = manager.get_user(999).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AdminError::UserNotFound(999)));
    }

    #[test]
    fn test_user_record_serialization() {
        let user = create_test_user(1, "testuser", 100);
        let json = serde_json::to_string(&user).unwrap();
        let deserialized: UserRecord = serde_json::from_str(&json).unwrap();
        assert_eq!(user.id, deserialized.id);
        assert_eq!(user.username, deserialized.username);
    }
}
