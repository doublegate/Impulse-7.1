//! User removal and banning functionality

use super::UserManager;
use crate::access::AdminPermission;
use crate::error::{AdminError, AdminResult};

impl UserManager {
    /// Bans a user from the system
    ///
    /// # Arguments
    /// * `admin_user_id` - ID of the administrator performing the ban
    /// * `user_id` - ID of the user to ban
    /// * `reason` - Reason for the ban
    pub async fn ban_user(
        &self,
        admin_user_id: i32,
        user_id: i32,
        reason: String,
    ) -> AdminResult<()> {
        self.access_control
            .require_permission(AdminPermission::BanUsers)?;

        if reason.is_empty() {
            return Err(AdminError::InvalidInput(
                "Ban reason cannot be empty".to_string(),
            ));
        }

        let mut users = self.users.write().await;
        let user = users
            .iter_mut()
            .find(|u| u.id == user_id)
            .ok_or(AdminError::UserNotFound(user_id))?;

        user.is_banned = true;
        user.ban_reason = Some(reason.clone());

        self.audit
            .log_action(
                admin_user_id,
                "ban_user",
                Some(user_id.to_string()),
                Some(reason),
            )
            .await;

        Ok(())
    }

    /// Unbans a previously banned user
    ///
    /// # Arguments
    /// * `admin_user_id` - ID of the administrator performing the unban
    /// * `user_id` - ID of the user to unban
    pub async fn unban_user(&self, admin_user_id: i32, user_id: i32) -> AdminResult<()> {
        self.access_control
            .require_permission(AdminPermission::BanUsers)?;

        let mut users = self.users.write().await;
        let user = users
            .iter_mut()
            .find(|u| u.id == user_id)
            .ok_or(AdminError::UserNotFound(user_id))?;

        user.is_banned = false;
        user.ban_reason = None;

        self.audit
            .log_action(
                admin_user_id,
                "unban_user",
                Some(user_id.to_string()),
                None::<String>,
            )
            .await;

        Ok(())
    }

    /// Deletes a user (soft delete - marks as deleted but preserves data)
    ///
    /// # Arguments
    /// * `admin_user_id` - ID of the administrator performing the deletion
    /// * `user_id` - ID of the user to delete
    pub async fn delete_user(&self, admin_user_id: i32, user_id: i32) -> AdminResult<()> {
        self.access_control
            .require_permission(AdminPermission::DeleteUsers)?;

        let mut users = self.users.write().await;
        let user = users
            .iter_mut()
            .find(|u| u.id == user_id)
            .ok_or(AdminError::UserNotFound(user_id))?;

        if user.is_deleted {
            return Err(AdminError::InvalidInput(
                "User is already deleted".to_string(),
            ));
        }

        user.is_deleted = true;

        self.audit
            .log_action(
                admin_user_id,
                "delete_user",
                Some(user_id.to_string()),
                None::<String>,
            )
            .await;

        Ok(())
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
    async fn test_ban_user() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let user = create_test_user(1, "testuser", 100);
        let manager = UserManager::with_users(access, audit, vec![user]);

        let result = manager
            .ban_user(1, 1, "Violation of terms".to_string())
            .await;
        assert!(result.is_ok());

        let banned = manager.get_user(1).await.unwrap();
        assert!(banned.is_banned);
        assert_eq!(banned.ban_reason, Some("Violation of terms".to_string()));
    }

    #[tokio::test]
    async fn test_ban_user_empty_reason() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let user = create_test_user(1, "testuser", 100);
        let manager = UserManager::with_users(access, audit, vec![user]);

        let result = manager.ban_user(1, 1, "".to_string()).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AdminError::InvalidInput(_)));
    }

    #[tokio::test]
    async fn test_ban_user_not_found() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let manager = UserManager::new(access, audit);

        let result = manager.ban_user(1, 999, "reason".to_string()).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AdminError::UserNotFound(999)));
    }

    #[tokio::test]
    async fn test_ban_user_permission_denied() {
        let access = AdminAccessControl::new(100, 200);
        let audit = AuditLogger::new();
        let manager = UserManager::new(access, audit);

        let result = manager.ban_user(1, 1, "reason".to_string()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_unban_user() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let mut user = create_test_user(1, "testuser", 100);
        user.is_banned = true;
        user.ban_reason = Some("Previously banned".to_string());
        let manager = UserManager::with_users(access, audit, vec![user]);

        let result = manager.unban_user(1, 1).await;
        assert!(result.is_ok());

        let unbanned = manager.get_user(1).await.unwrap();
        assert!(!unbanned.is_banned);
        assert_eq!(unbanned.ban_reason, None);
    }

    #[tokio::test]
    async fn test_unban_user_not_found() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let manager = UserManager::new(access, audit);

        let result = manager.unban_user(1, 999).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_user() {
        let access = AdminAccessControl::new(250, 200);
        let audit = AuditLogger::new();
        let user = create_test_user(1, "testuser", 100);
        let manager = UserManager::with_users(access, audit, vec![user]);

        let result = manager.delete_user(1, 1).await;
        assert!(result.is_ok());

        let deleted = manager.get_user(1).await.unwrap();
        assert!(deleted.is_deleted);
    }

    #[tokio::test]
    async fn test_delete_user_already_deleted() {
        let access = AdminAccessControl::new(250, 200);
        let audit = AuditLogger::new();
        let mut user = create_test_user(1, "testuser", 100);
        user.is_deleted = true;
        let manager = UserManager::with_users(access, audit, vec![user]);

        let result = manager.delete_user(1, 1).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AdminError::InvalidInput(_)));
    }

    #[tokio::test]
    async fn test_delete_user_not_found() {
        let access = AdminAccessControl::new(250, 200);
        let audit = AuditLogger::new();
        let manager = UserManager::new(access, audit);

        let result = manager.delete_user(1, 999).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AdminError::UserNotFound(999)));
    }

    #[tokio::test]
    async fn test_delete_user_permission_denied() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let manager = UserManager::new(access, audit);

        let result = manager.delete_user(1, 1).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_audit_log_ban_user() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let user = create_test_user(1, "testuser", 100);
        let manager = UserManager::with_users(access, audit.clone(), vec![user]);

        manager
            .ban_user(42, 1, "Test ban".to_string())
            .await
            .unwrap();

        let entries = audit.get_entries_by_action("ban_user").await;
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].admin_user_id, 42);
        assert_eq!(entries[0].target, Some("1".to_string()));
    }

    #[tokio::test]
    async fn test_audit_log_delete_user() {
        let access = AdminAccessControl::new(250, 200);
        let audit = AuditLogger::new();
        let user = create_test_user(1, "testuser", 100);
        let manager = UserManager::with_users(access, audit.clone(), vec![user]);

        manager.delete_user(42, 1).await.unwrap();

        let entries = audit.get_entries_by_action("delete_user").await;
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].admin_user_id, 42);
    }
}
