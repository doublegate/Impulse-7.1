//! User listing functionality

use super::{UserManager, UserRecord};
use crate::access::AdminPermission;
use crate::error::AdminResult;
use serde::{Deserialize, Serialize};

/// Summary view of a user for list display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSummary {
    pub id: i32,
    pub username: String,
    pub email: Option<String>,
    pub security_level: u8,
    pub login_count: i32,
    pub is_banned: bool,
}

impl From<&UserRecord> for UserSummary {
    fn from(user: &UserRecord) -> Self {
        Self {
            id: user.id,
            username: user.username.clone(),
            email: user.email.clone(),
            security_level: user.security_level,
            login_count: user.login_count,
            is_banned: user.is_banned,
        }
    }
}

impl UserManager {
    /// Lists all users with pagination
    ///
    /// # Arguments
    /// * `admin_user_id` - ID of the administrator requesting the list
    /// * `page` - Page number (0-indexed)
    /// * `page_size` - Number of users per page
    pub async fn list_users(
        &self,
        admin_user_id: i32,
        page: usize,
        page_size: usize,
    ) -> AdminResult<Vec<UserSummary>> {
        self.access_control
            .require_permission(AdminPermission::ViewUsers)?;

        let users = self.users.read().await;
        let start = page * page_size;
        let end = (start + page_size).min(users.len());

        let summaries: Vec<UserSummary> = users[start..end]
            .iter()
            .filter(|u| !u.is_deleted)
            .map(UserSummary::from)
            .collect();

        self.audit
            .log_action(
                admin_user_id,
                "list_users",
                None::<String>,
                Some(format!("page={}, count={}", page, summaries.len())),
            )
            .await;

        Ok(summaries)
    }

    /// Returns the total count of non-deleted users
    pub async fn count_users(&self) -> AdminResult<usize> {
        self.access_control
            .require_permission(AdminPermission::ViewUsers)?;

        let users = self.users.read().await;
        Ok(users.iter().filter(|u| !u.is_deleted).count())
    }

    /// Searches for users by username (case-insensitive partial match)
    pub async fn search_users(
        &self,
        admin_user_id: i32,
        query: &str,
        limit: usize,
    ) -> AdminResult<Vec<UserSummary>> {
        self.access_control
            .require_permission(AdminPermission::ViewUsers)?;

        let users = self.users.read().await;
        let query_lower = query.to_lowercase();

        let summaries: Vec<UserSummary> = users
            .iter()
            .filter(|u| !u.is_deleted && u.username.to_lowercase().contains(&query_lower))
            .take(limit)
            .map(UserSummary::from)
            .collect();

        self.audit
            .log_action(
                admin_user_id,
                "search_users",
                None::<String>,
                Some(format!("query='{}', count={}", query, summaries.len())),
            )
            .await;

        Ok(summaries)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::access::AdminAccessControl;
    use crate::audit::AuditLogger;
    use crate::users::UserRecord;
    use chrono::Utc;

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
    async fn test_list_users() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let users = vec![
            create_test_user(1, "user1", 100),
            create_test_user(2, "user2", 100),
            create_test_user(3, "user3", 100),
        ];
        let manager = UserManager::with_users(access, audit, users);

        let result = manager.list_users(1, 0, 10).await;
        assert!(result.is_ok());
        let summaries = result.unwrap();
        assert_eq!(summaries.len(), 3);
    }

    #[tokio::test]
    async fn test_list_users_pagination() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let users = (1..=10)
            .map(|i| create_test_user(i, &format!("user{}", i), 100))
            .collect();
        let manager = UserManager::with_users(access, audit, users);

        // First page
        let page1 = manager.list_users(1, 0, 5).await.unwrap();
        assert_eq!(page1.len(), 5);
        assert_eq!(page1[0].id, 1);

        // Second page
        let page2 = manager.list_users(1, 1, 5).await.unwrap();
        assert_eq!(page2.len(), 5);
        assert_eq!(page2[0].id, 6);
    }

    #[tokio::test]
    async fn test_list_users_excludes_deleted() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let mut user1 = create_test_user(1, "user1", 100);
        let user2 = create_test_user(2, "user2", 100);
        user1.is_deleted = true;

        let manager = UserManager::with_users(access, audit, vec![user1, user2]);

        let summaries = manager.list_users(1, 0, 10).await.unwrap();
        assert_eq!(summaries.len(), 1);
        assert_eq!(summaries[0].id, 2);
    }

    #[tokio::test]
    async fn test_list_users_permission_denied() {
        let access = AdminAccessControl::new(100, 200);
        let audit = AuditLogger::new();
        let manager = UserManager::new(access, audit);

        let result = manager.list_users(1, 0, 10).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_count_users() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let users = vec![
            create_test_user(1, "user1", 100),
            create_test_user(2, "user2", 100),
            create_test_user(3, "user3", 100),
        ];
        let manager = UserManager::with_users(access, audit, users);

        let count = manager.count_users().await.unwrap();
        assert_eq!(count, 3);
    }

    #[tokio::test]
    async fn test_count_users_excludes_deleted() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let mut user1 = create_test_user(1, "user1", 100);
        user1.is_deleted = true;
        let user2 = create_test_user(2, "user2", 100);

        let manager = UserManager::with_users(access, audit, vec![user1, user2]);

        let count = manager.count_users().await.unwrap();
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn test_search_users() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let users = vec![
            create_test_user(1, "alice", 100),
            create_test_user(2, "bob", 100),
            create_test_user(3, "charlie", 100),
            create_test_user(4, "alice2", 100),
        ];
        let manager = UserManager::with_users(access, audit, users);

        let results = manager.search_users(1, "alice", 10).await.unwrap();
        assert_eq!(results.len(), 2);
        assert!(results.iter().any(|u| u.username == "alice"));
        assert!(results.iter().any(|u| u.username == "alice2"));
    }

    #[tokio::test]
    async fn test_search_users_case_insensitive() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let users = vec![create_test_user(1, "Alice", 100)];
        let manager = UserManager::with_users(access, audit, users);

        let results = manager.search_users(1, "alice", 10).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].username, "Alice");
    }

    #[tokio::test]
    async fn test_search_users_limit() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let users = (1..=10)
            .map(|i| create_test_user(i, &format!("user{}", i), 100))
            .collect();
        let manager = UserManager::with_users(access, audit, users);

        let results = manager.search_users(1, "user", 5).await.unwrap();
        assert_eq!(results.len(), 5);
    }

    #[tokio::test]
    async fn test_audit_log_list_users() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let manager = UserManager::new(access, audit.clone());

        manager.list_users(42, 0, 10).await.unwrap();

        let entries = audit.get_entries_by_action("list_users").await;
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].admin_user_id, 42);
    }
}
