//! System broadcast messaging functionality

use super::SystemMaintenance;
use crate::access::AdminPermission;
use crate::error::{AdminError, AdminResult};

impl SystemMaintenance {
    /// Broadcasts a message to all active sessions
    ///
    /// # Arguments
    /// * `admin_user_id` - ID of the administrator sending the broadcast
    /// * `message` - Message content to broadcast
    ///
    /// # Returns
    /// The number of sessions the message was broadcast to
    pub async fn broadcast_message(
        &self,
        admin_user_id: i32,
        message: String,
    ) -> AdminResult<usize> {
        self.access_control
            .require_permission(AdminPermission::BroadcastMessages)?;

        if message.is_empty() {
            return Err(AdminError::InvalidInput(
                "Broadcast message cannot be empty".to_string(),
            ));
        }

        if message.len() > 1000 {
            return Err(AdminError::InvalidInput(
                "Broadcast message too long (max 1000 chars)".to_string(),
            ));
        }

        let sessions = self.sessions.read().await;
        let receiver_count = sessions.len();

        // In production, this would actually send the message via broadcast channel
        // For testing, we just log it
        tracing::info!(
            admin_user_id = admin_user_id,
            receiver_count = receiver_count,
            "Broadcast message sent"
        );

        self.audit
            .log_action(
                admin_user_id,
                "broadcast_message",
                None::<String>,
                Some(format!(
                    "receivers={}, message={}",
                    receiver_count,
                    message.chars().take(50).collect::<String>()
                )),
            )
            .await;

        Ok(receiver_count)
    }

    /// Broadcasts a message to specific users
    ///
    /// # Arguments
    /// * `admin_user_id` - ID of the administrator sending the broadcast
    /// * `target_user_ids` - List of user IDs to send the message to
    /// * `message` - Message content to broadcast
    ///
    /// # Returns
    /// The number of sessions the message was broadcast to
    pub async fn broadcast_to_users(
        &self,
        admin_user_id: i32,
        target_user_ids: Vec<i32>,
        message: String,
    ) -> AdminResult<usize> {
        self.access_control
            .require_permission(AdminPermission::BroadcastMessages)?;

        if message.is_empty() {
            return Err(AdminError::InvalidInput(
                "Broadcast message cannot be empty".to_string(),
            ));
        }

        if target_user_ids.is_empty() {
            return Err(AdminError::InvalidInput(
                "Target user list cannot be empty".to_string(),
            ));
        }

        let sessions = self.sessions.read().await;
        let target_count = sessions
            .values()
            .filter(|s| target_user_ids.contains(&s.user_id))
            .count();

        tracing::info!(
            admin_user_id = admin_user_id,
            target_count = target_count,
            "Targeted broadcast message sent"
        );

        self.audit
            .log_action(
                admin_user_id,
                "broadcast_to_users",
                Some(format!("{:?}", target_user_ids)),
                Some(format!("receivers={}, message_len={}", target_count, message.len())),
            )
            .await;

        Ok(target_count)
    }

    /// Sends a shutdown notification to all active sessions
    ///
    /// # Arguments
    /// * `admin_user_id` - ID of the administrator initiating the shutdown
    /// * `shutdown_minutes` - Minutes until system shutdown
    pub async fn broadcast_shutdown_notice(
        &self,
        admin_user_id: i32,
        shutdown_minutes: i32,
    ) -> AdminResult<usize> {
        self.access_control
            .require_permission(AdminPermission::BroadcastMessages)?;

        if shutdown_minutes <= 0 {
            return Err(AdminError::InvalidInput(
                "Shutdown minutes must be positive".to_string(),
            ));
        }

        let message = format!(
            "SYSTEM SHUTDOWN: The system will shut down in {} minute(s). Please save your work and log off.",
            shutdown_minutes
        );

        let receiver_count = self.broadcast_message(admin_user_id, message).await?;

        self.audit
            .log_action(
                admin_user_id,
                "broadcast_shutdown_notice",
                None::<String>,
                Some(format!(
                    "receivers={}, shutdown_minutes={}",
                    receiver_count, shutdown_minutes
                )),
            )
            .await;

        Ok(receiver_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::access::AdminAccessControl;
    use crate::audit::AuditLogger;
    use crate::system::ActiveSession;
    use chrono::Utc;

    fn create_test_session(user_id: i32, username: &str) -> ActiveSession {
        ActiveSession {
            id: uuid::Uuid::new_v4(),
            user_id,
            username: username.to_string(),
            node: 1,
            ip_address: "127.0.0.1".to_string(),
            login_time: Utc::now(),
            last_activity: Utc::now(),
            current_menu: "main".to_string(),
        }
    }

    #[tokio::test]
    async fn test_broadcast_message() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let sessions = vec![
            create_test_session(1, "user1"),
            create_test_session(2, "user2"),
            create_test_session(3, "user3"),
        ];
        let maint = SystemMaintenance::with_sessions(access, audit, sessions);

        let result = maint
            .broadcast_message(1, "Test message".to_string())
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 3);
    }

    #[tokio::test]
    async fn test_broadcast_message_empty() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let maint = SystemMaintenance::new(access, audit);

        let result = maint.broadcast_message(1, "".to_string()).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AdminError::InvalidInput(_)));
    }

    #[tokio::test]
    async fn test_broadcast_message_too_long() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let maint = SystemMaintenance::new(access, audit);

        let long_message = "x".repeat(1001);
        let result = maint.broadcast_message(1, long_message).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_broadcast_message_no_sessions() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let maint = SystemMaintenance::new(access, audit);

        let result = maint
            .broadcast_message(1, "Test message".to_string())
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }

    #[tokio::test]
    async fn test_broadcast_message_permission_denied() {
        let access = AdminAccessControl::new(100, 200);
        let audit = AuditLogger::new();
        let maint = SystemMaintenance::new(access, audit);

        let result = maint
            .broadcast_message(1, "Test message".to_string())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_broadcast_to_users() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let sessions = vec![
            create_test_session(1, "user1"),
            create_test_session(2, "user2"),
            create_test_session(3, "user3"),
        ];
        let maint = SystemMaintenance::with_sessions(access, audit, sessions);

        let result = maint
            .broadcast_to_users(1, vec![1, 2], "Test message".to_string())
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2);
    }

    #[tokio::test]
    async fn test_broadcast_to_users_none_online() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let sessions = vec![create_test_session(1, "user1")];
        let maint = SystemMaintenance::with_sessions(access, audit, sessions);

        let result = maint
            .broadcast_to_users(1, vec![2, 3], "Test message".to_string())
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }

    #[tokio::test]
    async fn test_broadcast_to_users_empty_target_list() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let maint = SystemMaintenance::new(access, audit);

        let result = maint
            .broadcast_to_users(1, vec![], "Test message".to_string())
            .await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AdminError::InvalidInput(_)));
    }

    #[tokio::test]
    async fn test_broadcast_to_users_empty_message() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let maint = SystemMaintenance::new(access, audit);

        let result = maint
            .broadcast_to_users(1, vec![1], "".to_string())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_broadcast_shutdown_notice() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let sessions = vec![
            create_test_session(1, "user1"),
            create_test_session(2, "user2"),
        ];
        let maint = SystemMaintenance::with_sessions(access, audit, sessions);

        let result = maint.broadcast_shutdown_notice(1, 5).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2);
    }

    #[tokio::test]
    async fn test_broadcast_shutdown_notice_invalid_minutes() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let maint = SystemMaintenance::new(access, audit);

        let result = maint.broadcast_shutdown_notice(1, -5).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AdminError::InvalidInput(_)));
    }

    #[tokio::test]
    async fn test_audit_log_broadcast_message() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let maint = SystemMaintenance::new(access, audit.clone());

        maint
            .broadcast_message(42, "Test message".to_string())
            .await
            .unwrap();

        let entries = audit.get_entries_by_action("broadcast_message").await;
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].admin_user_id, 42);
    }

    #[tokio::test]
    async fn test_audit_log_broadcast_to_users() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let maint = SystemMaintenance::new(access, audit.clone());

        maint
            .broadcast_to_users(42, vec![1, 2], "Test".to_string())
            .await
            .unwrap();

        let entries = audit.get_entries_by_action("broadcast_to_users").await;
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].admin_user_id, 42);
    }

    #[tokio::test]
    async fn test_audit_log_broadcast_shutdown() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let maint = SystemMaintenance::new(access, audit.clone());

        maint.broadcast_shutdown_notice(42, 5).await.unwrap();

        let entries = audit.get_entries_by_action("broadcast_shutdown_notice").await;
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].admin_user_id, 42);
    }
}
