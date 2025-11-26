//! Active session viewing functionality

use super::{ActiveSession, SystemMaintenance};
use crate::access::AdminPermission;
use crate::error::AdminResult;

impl SystemMaintenance {
    /// Views all active sessions
    ///
    /// # Arguments
    /// * `admin_user_id` - ID of the administrator requesting the session list
    pub async fn view_active_sessions(
        &self,
        admin_user_id: i32,
    ) -> AdminResult<Vec<ActiveSession>> {
        self.access_control
            .require_permission(AdminPermission::ViewSessions)?;

        let sessions = self.sessions.read().await;
        let result: Vec<ActiveSession> = sessions.values().cloned().collect();

        self.audit
            .log_action(
                admin_user_id,
                "view_active_sessions",
                None::<String>,
                Some(format!("count={}", result.len())),
            )
            .await;

        Ok(result)
    }

    /// Gets session count
    pub async fn session_count(&self) -> AdminResult<usize> {
        self.access_control
            .require_permission(AdminPermission::ViewSessions)?;

        let sessions = self.sessions.read().await;
        Ok(sessions.len())
    }

    /// Gets sessions for a specific user
    pub async fn get_user_sessions(
        &self,
        admin_user_id: i32,
        target_user_id: i32,
    ) -> AdminResult<Vec<ActiveSession>> {
        self.access_control
            .require_permission(AdminPermission::ViewSessions)?;

        let sessions = self.sessions.read().await;
        let result: Vec<ActiveSession> = sessions
            .values()
            .filter(|s| s.user_id == target_user_id)
            .cloned()
            .collect();

        self.audit
            .log_action(
                admin_user_id,
                "get_user_sessions",
                Some(target_user_id.to_string()),
                Some(format!("count={}", result.len())),
            )
            .await;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::access::AdminAccessControl;
    use crate::audit::AuditLogger;
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
    async fn test_view_active_sessions() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let sessions = vec![
            create_test_session(1, "user1"),
            create_test_session(2, "user2"),
        ];
        let maint = SystemMaintenance::with_sessions(access, audit, sessions);

        let result = maint.view_active_sessions(1).await;
        assert!(result.is_ok());
        let list = result.unwrap();
        assert_eq!(list.len(), 2);
    }

    #[tokio::test]
    async fn test_view_active_sessions_empty() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let maint = SystemMaintenance::new(access, audit);

        let result = maint.view_active_sessions(1).await;
        assert!(result.is_ok());
        let list = result.unwrap();
        assert_eq!(list.len(), 0);
    }

    #[tokio::test]
    async fn test_view_active_sessions_permission_denied() {
        let access = AdminAccessControl::new(100, 200);
        let audit = AuditLogger::new();
        let maint = SystemMaintenance::new(access, audit);

        let result = maint.view_active_sessions(1).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_session_count() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let sessions = vec![
            create_test_session(1, "user1"),
            create_test_session(2, "user2"),
            create_test_session(3, "user3"),
        ];
        let maint = SystemMaintenance::with_sessions(access, audit, sessions);

        let count = maint.session_count().await.unwrap();
        assert_eq!(count, 3);
    }

    #[tokio::test]
    async fn test_get_user_sessions() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let sessions = vec![
            create_test_session(1, "user1"),
            create_test_session(1, "user1"), // Same user, different session
            create_test_session(2, "user2"),
        ];
        let maint = SystemMaintenance::with_sessions(access, audit, sessions);

        let result = maint.get_user_sessions(1, 1).await;
        assert!(result.is_ok());
        let user_sessions = result.unwrap();
        assert_eq!(user_sessions.len(), 2);
        assert!(user_sessions.iter().all(|s| s.user_id == 1));
    }

    #[tokio::test]
    async fn test_get_user_sessions_none() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let sessions = vec![create_test_session(1, "user1")];
        let maint = SystemMaintenance::with_sessions(access, audit, sessions);

        let result = maint.get_user_sessions(1, 999).await;
        assert!(result.is_ok());
        let user_sessions = result.unwrap();
        assert_eq!(user_sessions.len(), 0);
    }

    #[tokio::test]
    async fn test_audit_log_view_sessions() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let maint = SystemMaintenance::new(access, audit.clone());

        maint.view_active_sessions(42).await.unwrap();

        let entries = audit.get_entries_by_action("view_active_sessions").await;
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].admin_user_id, 42);
    }

    #[tokio::test]
    async fn test_audit_log_get_user_sessions() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let maint = SystemMaintenance::new(access, audit.clone());

        maint.get_user_sessions(42, 1).await.unwrap();

        let entries = audit.get_entries_by_action("get_user_sessions").await;
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].admin_user_id, 42);
        assert_eq!(entries[0].target, Some("1".to_string()));
    }
}
