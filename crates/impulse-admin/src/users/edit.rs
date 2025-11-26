//! User editing functionality

use super::UserManager;
use crate::access::AdminPermission;
use crate::error::{AdminError, AdminResult};
use serde::{Deserialize, Serialize};

/// Request to edit user properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEditRequest {
    pub email: Option<String>,
    pub security_level: Option<u8>,
    pub time_limit_minutes: Option<i32>,
}

impl UserManager {
    /// Edits user properties
    ///
    /// # Arguments
    /// * `admin_user_id` - ID of the administrator performing the edit
    /// * `user_id` - ID of the user to edit
    /// * `changes` - Requested changes to apply
    pub async fn edit_user(
        &self,
        admin_user_id: i32,
        user_id: i32,
        changes: UserEditRequest,
    ) -> AdminResult<()> {
        self.access_control
            .require_permission(AdminPermission::EditUsers)?;

        // Validate changes
        if let Some(ref email) = changes.email {
            if email.is_empty() {
                return Err(AdminError::InvalidInput(
                    "Email cannot be empty".to_string(),
                ));
            }
            if !email.contains('@') {
                return Err(AdminError::InvalidInput("Invalid email format".to_string()));
            }
        }

        // Security level validation (u8 is 0-255 by type)
        if let Some(_level) = changes.security_level {
            // All u8 values are valid, no need to check
        }

        if let Some(limit) = changes.time_limit_minutes
            && limit < 0
        {
            return Err(AdminError::InvalidInput(
                "Time limit cannot be negative".to_string(),
            ));
        }

        // Apply changes
        let mut users = self.users.write().await;
        let user = users
            .iter_mut()
            .find(|u| u.id == user_id)
            .ok_or(AdminError::UserNotFound(user_id))?;

        if let Some(ref email) = changes.email {
            user.email = Some(email.clone());
        }

        if let Some(security_level) = changes.security_level {
            user.security_level = security_level;
        }

        if let Some(time_limit) = changes.time_limit_minutes {
            user.time_limit_minutes = time_limit;
        }

        self.audit
            .log_action(
                admin_user_id,
                "edit_user",
                Some(user_id.to_string()),
                Some(format!("{:?}", changes)),
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
    async fn test_edit_user_email() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let user = create_test_user(1, "testuser", 100);
        let manager = UserManager::with_users(access, audit, vec![user]);

        let changes = UserEditRequest {
            email: Some("newemail@example.com".to_string()),
            security_level: None,
            time_limit_minutes: None,
        };

        let result = manager.edit_user(1, 1, changes).await;
        assert!(result.is_ok());

        let updated = manager.get_user(1).await.unwrap();
        assert_eq!(updated.email, Some("newemail@example.com".to_string()));
    }

    #[tokio::test]
    async fn test_edit_user_security_level() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let user = create_test_user(1, "testuser", 100);
        let manager = UserManager::with_users(access, audit, vec![user]);

        let changes = UserEditRequest {
            email: None,
            security_level: Some(150),
            time_limit_minutes: None,
        };

        let result = manager.edit_user(1, 1, changes).await;
        assert!(result.is_ok());

        let updated = manager.get_user(1).await.unwrap();
        assert_eq!(updated.security_level, 150);
    }

    #[tokio::test]
    async fn test_edit_user_time_limit() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let user = create_test_user(1, "testuser", 100);
        let manager = UserManager::with_users(access, audit, vec![user]);

        let changes = UserEditRequest {
            email: None,
            security_level: None,
            time_limit_minutes: Some(120),
        };

        let result = manager.edit_user(1, 1, changes).await;
        assert!(result.is_ok());

        let updated = manager.get_user(1).await.unwrap();
        assert_eq!(updated.time_limit_minutes, 120);
    }

    #[tokio::test]
    async fn test_edit_user_multiple_fields() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let user = create_test_user(1, "testuser", 100);
        let manager = UserManager::with_users(access, audit, vec![user]);

        let changes = UserEditRequest {
            email: Some("new@example.com".to_string()),
            security_level: Some(200),
            time_limit_minutes: Some(90),
        };

        let result = manager.edit_user(1, 1, changes).await;
        assert!(result.is_ok());

        let updated = manager.get_user(1).await.unwrap();
        assert_eq!(updated.email, Some("new@example.com".to_string()));
        assert_eq!(updated.security_level, 200);
        assert_eq!(updated.time_limit_minutes, 90);
    }

    #[tokio::test]
    async fn test_edit_user_not_found() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let manager = UserManager::new(access, audit);

        let changes = UserEditRequest {
            email: Some("new@example.com".to_string()),
            security_level: None,
            time_limit_minutes: None,
        };

        let result = manager.edit_user(1, 999, changes).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AdminError::UserNotFound(999)));
    }

    #[tokio::test]
    async fn test_edit_user_permission_denied() {
        let access = AdminAccessControl::new(100, 200);
        let audit = AuditLogger::new();
        let manager = UserManager::new(access, audit);

        let changes = UserEditRequest {
            email: Some("new@example.com".to_string()),
            security_level: None,
            time_limit_minutes: None,
        };

        let result = manager.edit_user(1, 1, changes).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_user_invalid_email_empty() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let user = create_test_user(1, "testuser", 100);
        let manager = UserManager::with_users(access, audit, vec![user]);

        let changes = UserEditRequest {
            email: Some("".to_string()),
            security_level: None,
            time_limit_minutes: None,
        };

        let result = manager.edit_user(1, 1, changes).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AdminError::InvalidInput(_)));
    }

    #[tokio::test]
    async fn test_edit_user_invalid_email_format() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let user = create_test_user(1, "testuser", 100);
        let manager = UserManager::with_users(access, audit, vec![user]);

        let changes = UserEditRequest {
            email: Some("notanemail".to_string()),
            security_level: None,
            time_limit_minutes: None,
        };

        let result = manager.edit_user(1, 1, changes).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_user_security_level_max() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let user = create_test_user(1, "testuser", 100);
        let manager = UserManager::with_users(access, audit, vec![user]);

        let changes = UserEditRequest {
            email: None,
            security_level: Some(255), // Max valid u8 value
            time_limit_minutes: None,
        };

        let result = manager.edit_user(1, 1, changes).await;
        assert!(result.is_ok());

        let updated = manager.get_user(1).await.unwrap();
        assert_eq!(updated.security_level, 255);
    }

    #[tokio::test]
    async fn test_edit_user_negative_time_limit() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let user = create_test_user(1, "testuser", 100);
        let manager = UserManager::with_users(access, audit, vec![user]);

        let changes = UserEditRequest {
            email: None,
            security_level: None,
            time_limit_minutes: Some(-10),
        };

        let result = manager.edit_user(1, 1, changes).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_audit_log_edit_user() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let user = create_test_user(1, "testuser", 100);
        let manager = UserManager::with_users(access, audit.clone(), vec![user]);

        let changes = UserEditRequest {
            email: Some("new@example.com".to_string()),
            security_level: None,
            time_limit_minutes: None,
        };

        manager.edit_user(42, 1, changes).await.unwrap();

        let entries = audit.get_entries_by_action("edit_user").await;
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].admin_user_id, 42);
        assert_eq!(entries[0].target, Some("1".to_string()));
    }
}
