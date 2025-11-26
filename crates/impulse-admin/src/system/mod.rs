//! System maintenance functionality for administrators

pub mod broadcast;
pub mod kick;
pub mod sessions;

use crate::access::AdminAccessControl;
use crate::audit::AuditLogger;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Active session information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveSession {
    pub id: uuid::Uuid,
    pub user_id: i32,
    pub username: String,
    pub node: u16,
    pub ip_address: String,
    pub login_time: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub current_menu: String,
}

/// System message that can be broadcast to users
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemMessage {
    /// Broadcast message from administrator
    Broadcast {
        from_admin: i32,
        message: String,
        timestamp: DateTime<Utc>,
    },
    /// Disconnect notification
    Disconnect { reason: String },
}

/// In-memory session storage (in production, this would integrate with the session manager)
type SessionStore = Arc<RwLock<HashMap<uuid::Uuid, ActiveSession>>>;

/// System maintenance manager for administrative operations
#[derive(Debug, Clone)]
pub struct SystemMaintenance {
    sessions: SessionStore,
    audit: AuditLogger,
    access_control: AdminAccessControl,
}

impl SystemMaintenance {
    /// Creates a new system maintenance manager
    pub fn new(access_control: AdminAccessControl, audit: AuditLogger) -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            audit,
            access_control,
        }
    }

    /// Creates a system maintenance manager with pre-populated sessions (for testing)
    pub fn with_sessions(
        access_control: AdminAccessControl,
        audit: AuditLogger,
        sessions: Vec<ActiveSession>,
    ) -> Self {
        let session_map: HashMap<uuid::Uuid, ActiveSession> =
            sessions.into_iter().map(|s| (s.id, s)).collect();

        Self {
            sessions: Arc::new(RwLock::new(session_map)),
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

    /// Adds a session (for testing)
    #[cfg(test)]
    pub async fn add_session(&self, session: ActiveSession) {
        let mut sessions = self.sessions.write().await;
        sessions.insert(session.id, session);
    }

    /// Gets a session by ID
    pub async fn get_session(&self, session_id: uuid::Uuid) -> Option<ActiveSession> {
        let sessions = self.sessions.read().await;
        sessions.get(&session_id).cloned()
    }

    /// Removes a session
    async fn remove_session(&self, session_id: uuid::Uuid) {
        let mut sessions = self.sessions.write().await;
        sessions.remove(&session_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::access::AdminPermission;

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
    async fn test_system_maintenance_creation() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let maint = SystemMaintenance::new(access, audit);

        assert!(
            maint
                .access_control()
                .has_permission(AdminPermission::ViewSessions)
        );
    }

    #[tokio::test]
    async fn test_add_and_get_session() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let maint = SystemMaintenance::new(access, audit);

        let session = create_test_session(1, "testuser");
        let session_id = session.id;

        maint.add_session(session).await;

        let retrieved = maint.get_session(session_id).await;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().username, "testuser");
    }

    #[tokio::test]
    async fn test_get_nonexistent_session() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let maint = SystemMaintenance::new(access, audit);

        let result = maint.get_session(uuid::Uuid::new_v4()).await;
        assert!(result.is_none());
    }

    #[test]
    fn test_system_message_serialization() {
        let msg = SystemMessage::Broadcast {
            from_admin: 1,
            message: "Test message".to_string(),
            timestamp: Utc::now(),
        };

        let json = serde_json::to_string(&msg).unwrap();
        let deserialized: SystemMessage = serde_json::from_str(&json).unwrap();

        if let SystemMessage::Broadcast { from_admin, .. } = deserialized {
            assert_eq!(from_admin, 1);
        } else {
            panic!("Wrong message type");
        }
    }
}
