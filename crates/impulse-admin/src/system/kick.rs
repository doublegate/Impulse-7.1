//! User kick and disconnect functionality

use super::SystemMaintenance;
use crate::access::AdminPermission;
use crate::error::{AdminError, AdminResult};
use chrono::{Duration, Utc};

impl SystemMaintenance {
    /// Kicks a specific user by session ID
    ///
    /// # Arguments
    /// * `admin_user_id` - ID of the administrator performing the kick
    /// * `session_id` - ID of the session to kick
    /// * `reason` - Reason for kicking the user
    pub async fn kick_user(
        &self,
        admin_user_id: i32,
        session_id: uuid::Uuid,
        reason: String,
    ) -> AdminResult<()> {
        self.access_control
            .require_permission(AdminPermission::KickUsers)?;

        if reason.is_empty() {
            return Err(AdminError::InvalidInput(
                "Kick reason cannot be empty".to_string(),
            ));
        }

        // Verify session exists
        let sessions = self.sessions.read().await;
        if !sessions.contains_key(&session_id) {
            return Err(AdminError::SessionNotFound(session_id));
        }
        drop(sessions);

        // Remove the session
        self.remove_session(session_id).await;

        self.audit
            .log_action(
                admin_user_id,
                "kick_user",
                Some(session_id.to_string()),
                Some(reason),
            )
            .await;

        tracing::info!(
            session_id = %session_id,
            admin_user_id = admin_user_id,
            "User kicked by administrator"
        );

        Ok(())
    }

    /// Kicks all idle users who haven't been active for the specified duration
    ///
    /// # Arguments
    /// * `admin_user_id` - ID of the administrator performing the operation
    /// * `idle_minutes` - Number of minutes of inactivity after which to kick users
    pub async fn kick_idle_users(
        &self,
        admin_user_id: i32,
        idle_minutes: i64,
    ) -> AdminResult<Vec<uuid::Uuid>> {
        self.access_control
            .require_permission(AdminPermission::KickUsers)?;

        if idle_minutes <= 0 {
            return Err(AdminError::InvalidInput(
                "Idle minutes must be positive".to_string(),
            ));
        }

        let idle_threshold = Utc::now() - Duration::minutes(idle_minutes);
        let mut kicked = Vec::new();

        let sessions = self.sessions.read().await;
        let idle_session_ids: Vec<uuid::Uuid> = sessions
            .values()
            .filter(|s| s.last_activity < idle_threshold)
            .map(|s| s.id)
            .collect();
        drop(sessions);

        // Kick each idle session
        for session_id in idle_session_ids {
            self.remove_session(session_id).await;
            kicked.push(session_id);
        }

        self.audit
            .log_action(
                admin_user_id,
                "kick_idle_users",
                None::<String>,
                Some(format!(
                    "kicked={}, idle_minutes={}",
                    kicked.len(),
                    idle_minutes
                )),
            )
            .await;

        Ok(kicked)
    }

    /// Kicks all sessions for a specific user
    ///
    /// # Arguments
    /// * `admin_user_id` - ID of the administrator performing the operation
    /// * `target_user_id` - ID of the user whose sessions should be kicked
    /// * `reason` - Reason for kicking the user
    pub async fn kick_all_user_sessions(
        &self,
        admin_user_id: i32,
        target_user_id: i32,
        reason: String,
    ) -> AdminResult<Vec<uuid::Uuid>> {
        self.access_control
            .require_permission(AdminPermission::KickUsers)?;

        if reason.is_empty() {
            return Err(AdminError::InvalidInput(
                "Kick reason cannot be empty".to_string(),
            ));
        }

        let sessions = self.sessions.read().await;
        let user_session_ids: Vec<uuid::Uuid> = sessions
            .values()
            .filter(|s| s.user_id == target_user_id)
            .map(|s| s.id)
            .collect();
        drop(sessions);

        let mut kicked = Vec::new();
        for session_id in user_session_ids {
            self.remove_session(session_id).await;
            kicked.push(session_id);
        }

        self.audit
            .log_action(
                admin_user_id,
                "kick_all_user_sessions",
                Some(target_user_id.to_string()),
                Some(format!("kicked={}, reason={}", kicked.len(), reason)),
            )
            .await;

        Ok(kicked)
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

    fn create_idle_session(user_id: i32, username: &str, idle_minutes: i64) -> ActiveSession {
        let mut session = create_test_session(user_id, username);
        session.last_activity = Utc::now() - Duration::minutes(idle_minutes);
        session
    }

    #[tokio::test]
    async fn test_kick_user() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let session = create_test_session(1, "testuser");
        let session_id = session.id;
        let maint = SystemMaintenance::with_sessions(access, audit, vec![session]);

        let result = maint
            .kick_user(1, session_id, "Test kick".to_string())
            .await;
        assert!(result.is_ok());

        // Verify session was removed
        let remaining = maint.get_session(session_id).await;
        assert!(remaining.is_none());
    }

    #[tokio::test]
    async fn test_kick_user_not_found() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let maint = SystemMaintenance::new(access, audit);

        let result = maint
            .kick_user(1, uuid::Uuid::new_v4(), "reason".to_string())
            .await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AdminError::SessionNotFound(_)));
    }

    #[tokio::test]
    async fn test_kick_user_empty_reason() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let session = create_test_session(1, "testuser");
        let session_id = session.id;
        let maint = SystemMaintenance::with_sessions(access, audit, vec![session]);

        let result = maint.kick_user(1, session_id, "".to_string()).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AdminError::InvalidInput(_)));
    }

    #[tokio::test]
    async fn test_kick_user_permission_denied() {
        let access = AdminAccessControl::new(100, 200);
        let audit = AuditLogger::new();
        let maint = SystemMaintenance::new(access, audit);

        let result = maint
            .kick_user(1, uuid::Uuid::new_v4(), "reason".to_string())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_kick_idle_users() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let sessions = vec![
            create_test_session(1, "active1"),
            create_idle_session(2, "idle1", 30), // Idle for 30 minutes
            create_idle_session(3, "idle2", 45), // Idle for 45 minutes
        ];
        let maint = SystemMaintenance::with_sessions(access, audit, sessions);

        let result = maint.kick_idle_users(1, 15).await;
        assert!(result.is_ok());
        let kicked = result.unwrap();
        assert_eq!(kicked.len(), 2);
    }

    #[tokio::test]
    async fn test_kick_idle_users_none_idle() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let sessions = vec![
            create_test_session(1, "user1"),
            create_test_session(2, "user2"),
        ];
        let maint = SystemMaintenance::with_sessions(access, audit, sessions);

        let result = maint.kick_idle_users(1, 15).await;
        assert!(result.is_ok());
        let kicked = result.unwrap();
        assert_eq!(kicked.len(), 0);
    }

    #[tokio::test]
    async fn test_kick_idle_users_invalid_minutes() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let maint = SystemMaintenance::new(access, audit);

        let result = maint.kick_idle_users(1, -5).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AdminError::InvalidInput(_)));
    }

    #[tokio::test]
    async fn test_kick_all_user_sessions() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let sessions = vec![
            create_test_session(1, "user1"),
            create_test_session(1, "user1"), // Same user, different session
            create_test_session(2, "user2"),
        ];
        let maint = SystemMaintenance::with_sessions(access, audit, sessions);

        let result = maint
            .kick_all_user_sessions(1, 1, "Test kick".to_string())
            .await;
        assert!(result.is_ok());
        let kicked = result.unwrap();
        assert_eq!(kicked.len(), 2);

        // Verify other user's session is still active
        let remaining_count = maint.session_count().await.unwrap();
        assert_eq!(remaining_count, 1);
    }

    #[tokio::test]
    async fn test_kick_all_user_sessions_none() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let sessions = vec![create_test_session(1, "user1")];
        let maint = SystemMaintenance::with_sessions(access, audit, sessions);

        let result = maint
            .kick_all_user_sessions(1, 999, "reason".to_string())
            .await;
        assert!(result.is_ok());
        let kicked = result.unwrap();
        assert_eq!(kicked.len(), 0);
    }

    #[tokio::test]
    async fn test_audit_log_kick_user() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let session = create_test_session(1, "testuser");
        let session_id = session.id;
        let maint = SystemMaintenance::with_sessions(access, audit.clone(), vec![session]);

        maint
            .kick_user(42, session_id, "Test".to_string())
            .await
            .unwrap();

        let entries = audit.get_entries_by_action("kick_user").await;
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].admin_user_id, 42);
    }

    #[tokio::test]
    async fn test_audit_log_kick_idle_users() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let maint = SystemMaintenance::new(access, audit.clone());

        maint.kick_idle_users(42, 15).await.unwrap();

        let entries = audit.get_entries_by_action("kick_idle_users").await;
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].admin_user_id, 42);
    }
}
