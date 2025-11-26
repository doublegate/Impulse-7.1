//! Session manager implementation

use crate::config::SessionConfig;
use crate::error::{Result, SessionError};
use crate::session::{Session, SessionId, SessionState};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Thread-safe session manager
#[derive(Clone)]
pub struct SessionManager {
    /// Configuration
    config: Arc<SessionConfig>,
    /// Active sessions (by ID)
    sessions: Arc<RwLock<HashMap<SessionId, Session>>>,
    /// User session mapping (username -> Vec<SessionId>)
    user_sessions: Arc<RwLock<HashMap<String, Vec<SessionId>>>>,
}

impl SessionManager {
    /// Create a new session manager
    pub fn new(config: SessionConfig) -> Self {
        Self {
            config: Arc::new(config),
            sessions: Arc::new(RwLock::new(HashMap::new())),
            user_sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create a new session
    ///
    /// # Example
    ///
    /// ```
    /// use impulse_session::{SessionManager, SessionConfig, Result};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let manager = SessionManager::new(SessionConfig::default());
    ///     let session_id = manager.create_session("192.168.1.1:1234").await?;
    ///     println!("Created session: {}", session_id);
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_session(&self, remote_addr: impl Into<String>) -> Result<SessionId> {
        let session = Session::new(remote_addr.into());
        let session_id = session.id();

        // Check total session limit
        let sessions = self.sessions.read().await;
        if sessions.len() >= self.config.max_total_sessions {
            return Err(SessionError::TooManySessions {
                limit: self.config.max_total_sessions,
            });
        }
        drop(sessions);

        // Insert session
        let mut sessions = self.sessions.write().await;
        sessions.insert(session_id, session.clone());

        info!(
            session_id = %session_id,
            remote_addr = %session.remote_addr(),
            "Created new session"
        );

        Ok(session_id)
    }

    /// Get a session by ID
    pub async fn get_session(&self, session_id: SessionId) -> Result<Session> {
        let sessions = self.sessions.read().await;
        sessions
            .get(&session_id)
            .cloned()
            .ok_or(SessionError::NotFound(session_id.to_string()))
    }

    /// Update a session
    pub async fn update_session(&self, session: Session) -> Result<()> {
        let session_id = session.id();
        let mut sessions = self.sessions.write().await;

        if !sessions.contains_key(&session_id) {
            return Err(SessionError::NotFound(session_id.to_string()));
        }

        sessions.insert(session_id, session);
        Ok(())
    }

    /// Authenticate a session
    pub async fn authenticate_session(
        &self,
        session_id: SessionId,
        username: String,
        user_id: u32,
    ) -> Result<()> {
        use crate::config::ConflictPolicy;

        // Check sessions per user limit and handle conflicts
        {
            let user_sessions = self.user_sessions.read().await;
            if let Some(sessions) = user_sessions.get(&username) {
                if sessions.len() >= self.config.max_sessions_per_user {
                    match self.config.conflict_policy {
                        ConflictPolicy::Allow => {
                            // Should not reach here, but handle gracefully
                            return Err(SessionError::TooManySessions {
                                limit: self.config.max_sessions_per_user,
                            });
                        }
                        ConflictPolicy::KickOldest => {
                            // Find oldest session
                            drop(user_sessions);
                            let oldest_id = self.find_oldest_user_session(&username).await?;
                            warn!(
                                username = %username,
                                old_session = %oldest_id,
                                new_session = %session_id,
                                "Kicking oldest session due to limit"
                            );
                            self.terminate_session(oldest_id).await?;
                        }
                        ConflictPolicy::DenyNew => {
                            return Err(SessionError::TooManySessions {
                                limit: self.config.max_sessions_per_user,
                            });
                        }
                    }
                }
            }
        }

        // Update session
        let mut sessions = self.sessions.write().await;
        let session = sessions
            .get_mut(&session_id)
            .ok_or(SessionError::NotFound(session_id.to_string()))?;

        session.authenticate(username.clone(), user_id);

        // Add to user session mapping
        drop(sessions);
        let mut user_sessions = self.user_sessions.write().await;
        user_sessions
            .entry(username.clone())
            .or_default()
            .push(session_id);

        info!(
            session_id = %session_id,
            username = %username,
            user_id = %user_id,
            "Authenticated session"
        );

        Ok(())
    }

    /// Update session activity
    pub async fn update_activity(&self, session_id: SessionId) -> Result<()> {
        let mut sessions = self.sessions.write().await;
        let session = sessions
            .get_mut(&session_id)
            .ok_or(SessionError::NotFound(session_id.to_string()))?;

        session.update_activity();
        Ok(())
    }

    /// Set session state
    pub async fn set_session_state(
        &self,
        session_id: SessionId,
        state: SessionState,
    ) -> Result<()> {
        let mut sessions = self.sessions.write().await;
        let session = sessions
            .get_mut(&session_id)
            .ok_or(SessionError::NotFound(session_id.to_string()))?;

        session.set_state(state);
        Ok(())
    }

    /// Terminate a session
    pub async fn terminate_session(&self, session_id: SessionId) -> Result<()> {
        let mut sessions = self.sessions.write().await;
        let session = sessions
            .remove(&session_id)
            .ok_or(SessionError::NotFound(session_id.to_string()))?;

        // Remove from user session mapping
        if let Some(username) = session.username() {
            let mut user_sessions = self.user_sessions.write().await;
            if let Some(sessions) = user_sessions.get_mut(username) {
                sessions.retain(|id| *id != session_id);
                if sessions.is_empty() {
                    user_sessions.remove(username);
                }
            }
        }

        info!(
            session_id = %session_id,
            username = ?session.username(),
            "Terminated session"
        );

        Ok(())
    }

    /// Get all sessions for a user
    pub async fn get_user_sessions(&self, username: &str) -> Vec<Session> {
        let user_sessions = self.user_sessions.read().await;
        let session_ids = match user_sessions.get(username) {
            Some(ids) => ids.clone(),
            None => return Vec::new(),
        };
        drop(user_sessions);

        let sessions = self.sessions.read().await;
        session_ids
            .iter()
            .filter_map(|id| sessions.get(id).cloned())
            .collect()
    }

    /// Get total active session count
    pub async fn active_session_count(&self) -> usize {
        let sessions = self.sessions.read().await;
        sessions.len()
    }

    /// Find the oldest session for a given user
    async fn find_oldest_user_session(&self, username: &str) -> Result<SessionId> {
        let user_sessions = self.user_sessions.read().await;
        let session_ids = user_sessions
            .get(username)
            .ok_or_else(|| SessionError::NotFound(format!("No sessions for user {}", username)))?;

        let sessions = self.sessions.read().await;
        let oldest = session_ids
            .iter()
            .filter_map(|id| sessions.get(id).map(|s| (id, s)))
            .max_by_key(|(_, s)| s.age())
            .map(|(id, _)| *id)
            .ok_or_else(|| SessionError::NotFound(format!("No sessions for user {}", username)))?;

        Ok(oldest)
    }

    /// List all active sessions
    ///
    /// Returns a vector of all active sessions, useful for "who's online" functionality
    pub async fn list_all_sessions(&self) -> Vec<Session> {
        let sessions = self.sessions.read().await;
        sessions.values().cloned().collect()
    }

    /// List sessions with filters
    ///
    /// Returns sessions matching the given criteria
    pub async fn list_sessions_filtered<F>(&self, predicate: F) -> Vec<Session>
    where
        F: Fn(&Session) -> bool,
    {
        let sessions = self.sessions.read().await;
        sessions
            .values()
            .filter(|s| predicate(s))
            .cloned()
            .collect()
    }

    /// Check for sessions needing timeout warnings and return their IDs
    ///
    /// Returns a tuple of (sessions_needing_idle_warning, sessions_needing_absolute_warning)
    pub async fn check_timeout_warnings(&self) -> (Vec<SessionId>, Vec<SessionId>) {
        let sessions = self.sessions.read().await;
        let mut idle_warnings = Vec::new();
        let mut absolute_warnings = Vec::new();

        for (id, session) in sessions.iter() {
            // Skip unlimited users
            if let Some(username) = session.username() {
                if self.config.is_unlimited_user(username) {
                    continue;
                }
            }

            // Check idle warning
            if session.should_send_idle_warning(
                self.config.idle_timeout,
                self.config.warning_before_timeout,
            ) {
                idle_warnings.push(*id);
            }

            // Check absolute warning (if configured)
            if let Some(absolute_timeout) = self.config.absolute_timeout {
                if session.should_send_absolute_warning(
                    absolute_timeout,
                    self.config.warning_before_timeout,
                ) {
                    absolute_warnings.push(*id);
                }
            }
        }

        (idle_warnings, absolute_warnings)
    }

    /// Mark idle warning as sent for a session
    pub async fn mark_idle_warning_sent(&self, session_id: SessionId) -> Result<()> {
        let mut sessions = self.sessions.write().await;
        let session = sessions
            .get_mut(&session_id)
            .ok_or(SessionError::NotFound(session_id.to_string()))?;

        session.mark_idle_warning_sent();
        Ok(())
    }

    /// Mark absolute warning as sent for a session
    pub async fn mark_absolute_warning_sent(&self, session_id: SessionId) -> Result<()> {
        let mut sessions = self.sessions.write().await;
        let session = sessions
            .get_mut(&session_id)
            .ok_or(SessionError::NotFound(session_id.to_string()))?;

        session.mark_absolute_warning_sent();
        Ok(())
    }

    /// Clean up expired sessions
    ///
    /// Checks for idle timeouts and absolute timeouts (respecting unlimited users)
    pub async fn cleanup_expired_sessions(&self) -> usize {
        let mut expired = Vec::new();

        {
            let sessions = self.sessions.read().await;
            for (id, session) in sessions.iter() {
                // Skip unlimited users for absolute timeout
                let is_unlimited = session
                    .username()
                    .map(|u| self.config.is_unlimited_user(u))
                    .unwrap_or(false);

                // Check termination state
                if session.state() == SessionState::Terminated {
                    expired.push(*id);
                    continue;
                }

                // Check idle timeout
                if session.is_idle(self.config.idle_timeout) {
                    expired.push(*id);
                    continue;
                }

                // Check absolute timeout (only for non-unlimited users)
                if !is_unlimited {
                    if let Some(absolute_timeout) = self.config.absolute_timeout {
                        if session.is_absolute_timeout(absolute_timeout) {
                            expired.push(*id);
                            continue;
                        }
                    }
                }
            }
        }

        let count = expired.len();
        for id in expired {
            if let Err(e) = self.terminate_session(id).await {
                warn!(session_id = %id, error = %e, "Failed to terminate expired session");
            } else {
                debug!(session_id = %id, "Cleaned up expired session");
            }
        }

        count
    }

    /// Start automatic session cleanup task
    ///
    /// Returns a join handle for the spawned task
    pub fn spawn_cleanup_task(&self) -> tokio::task::JoinHandle<()> {
        let manager = self.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(manager.config.cleanup_interval);

            loop {
                interval.tick().await;
                let cleaned = manager.cleanup_expired_sessions().await;
                if cleaned > 0 {
                    info!(count = cleaned, "Cleaned up expired sessions");
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_create_session() {
        let manager = SessionManager::new(SessionConfig::default());
        let session_id = manager.create_session("192.168.1.1:1234").await.unwrap();

        let session = manager.get_session(session_id).await.unwrap();
        assert_eq!(session.remote_addr(), "192.168.1.1:1234");
    }

    #[tokio::test]
    async fn test_authenticate_session() {
        let manager = SessionManager::new(SessionConfig::default());
        let session_id = manager.create_session("192.168.1.1:1234").await.unwrap();

        manager
            .authenticate_session(session_id, "testuser".to_string(), 123)
            .await
            .unwrap();

        let session = manager.get_session(session_id).await.unwrap();
        assert!(session.is_authenticated());
        assert_eq!(session.username(), Some("testuser"));
    }

    #[tokio::test]
    async fn test_terminate_session() {
        let manager = SessionManager::new(SessionConfig::default());
        let session_id = manager.create_session("192.168.1.1:1234").await.unwrap();

        manager.terminate_session(session_id).await.unwrap();

        let result = manager.get_session(session_id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_max_sessions_per_user() {
        let config = SessionConfig::default().with_max_sessions_per_user(2);
        let manager = SessionManager::new(config);

        let id1 = manager.create_session("192.168.1.1:1234").await.unwrap();
        manager
            .authenticate_session(id1, "testuser".to_string(), 123)
            .await
            .unwrap();

        let id2 = manager.create_session("192.168.1.1:1235").await.unwrap();
        manager
            .authenticate_session(id2, "testuser".to_string(), 123)
            .await
            .unwrap();

        let id3 = manager.create_session("192.168.1.1:1236").await.unwrap();
        let result = manager
            .authenticate_session(id3, "testuser".to_string(), 123)
            .await;

        assert!(matches!(
            result,
            Err(SessionError::TooManySessions { limit: 2 })
        ));
    }

    #[tokio::test]
    async fn test_conflict_policy_kick_oldest() {
        use crate::config::ConflictPolicy;

        let config = SessionConfig::default()
            .with_max_sessions_per_user(2)
            .with_conflict_policy(ConflictPolicy::KickOldest);
        let manager = SessionManager::new(config);

        let id1 = manager.create_session("192.168.1.1:1234").await.unwrap();
        manager
            .authenticate_session(id1, "testuser".to_string(), 123)
            .await
            .unwrap();

        // Small delay to ensure distinguishable ages
        tokio::time::sleep(Duration::from_millis(10)).await;

        let id2 = manager.create_session("192.168.1.1:1235").await.unwrap();
        manager
            .authenticate_session(id2, "testuser".to_string(), 123)
            .await
            .unwrap();

        // Small delay to ensure distinguishable ages
        tokio::time::sleep(Duration::from_millis(10)).await;

        // Third session should kick the oldest (id1)
        let id3 = manager.create_session("192.168.1.1:1236").await.unwrap();
        manager
            .authenticate_session(id3, "testuser".to_string(), 123)
            .await
            .unwrap();

        // id1 should be terminated
        let result = manager.get_session(id1).await;
        assert!(result.is_err());

        // id2 and id3 should still exist
        assert!(manager.get_session(id2).await.is_ok());
        assert!(manager.get_session(id3).await.is_ok());

        let user_sessions = manager.get_user_sessions("testuser").await;
        assert_eq!(user_sessions.len(), 2);
    }

    #[tokio::test]
    async fn test_conflict_policy_deny_new() {
        use crate::config::ConflictPolicy;

        let config = SessionConfig::default()
            .with_max_sessions_per_user(2)
            .with_conflict_policy(ConflictPolicy::DenyNew);
        let manager = SessionManager::new(config);

        let id1 = manager.create_session("192.168.1.1:1234").await.unwrap();
        manager
            .authenticate_session(id1, "testuser".to_string(), 123)
            .await
            .unwrap();

        let id2 = manager.create_session("192.168.1.1:1235").await.unwrap();
        manager
            .authenticate_session(id2, "testuser".to_string(), 123)
            .await
            .unwrap();

        // Third session should be denied
        let id3 = manager.create_session("192.168.1.1:1236").await.unwrap();
        let result = manager
            .authenticate_session(id3, "testuser".to_string(), 123)
            .await;

        assert!(matches!(
            result,
            Err(SessionError::TooManySessions { limit: 2 })
        ));
    }

    #[tokio::test]
    async fn test_list_all_sessions() {
        let manager = SessionManager::new(SessionConfig::default());

        let id1 = manager.create_session("192.168.1.1:1234").await.unwrap();
        let id2 = manager.create_session("192.168.1.1:1235").await.unwrap();

        let sessions = manager.list_all_sessions().await;
        assert_eq!(sessions.len(), 2);
        assert!(sessions.iter().any(|s| s.id() == id1));
        assert!(sessions.iter().any(|s| s.id() == id2));
    }

    #[tokio::test]
    async fn test_list_sessions_filtered() {
        let manager = SessionManager::new(SessionConfig::default());

        let id1 = manager.create_session("192.168.1.1:1234").await.unwrap();
        manager
            .authenticate_session(id1, "user1".to_string(), 1)
            .await
            .unwrap();

        let id2 = manager.create_session("192.168.1.1:1235").await.unwrap();
        manager
            .authenticate_session(id2, "user2".to_string(), 2)
            .await
            .unwrap();

        // Filter for authenticated sessions
        let authenticated = manager
            .list_sessions_filtered(|s| s.is_authenticated())
            .await;
        assert_eq!(authenticated.len(), 2);

        // Filter for specific user
        let user1_sessions = manager
            .list_sessions_filtered(|s| s.username() == Some("user1"))
            .await;
        assert_eq!(user1_sessions.len(), 1);
        assert_eq!(user1_sessions[0].username(), Some("user1"));
    }

    #[tokio::test]
    async fn test_unlimited_user_no_timeout() {
        let config = SessionConfig::default()
            .with_absolute_timeout(Some(Duration::from_millis(100)))
            .with_unlimited_user("sysop".to_string());
        let manager = SessionManager::new(config);

        let id1 = manager.create_session("192.168.1.1:1234").await.unwrap();
        manager
            .authenticate_session(id1, "sysop".to_string(), 999)
            .await
            .unwrap();

        let id2 = manager.create_session("192.168.1.1:1235").await.unwrap();
        manager
            .authenticate_session(id2, "user".to_string(), 1)
            .await
            .unwrap();

        // Wait for absolute timeout
        tokio::time::sleep(Duration::from_millis(150)).await;

        let cleaned = manager.cleanup_expired_sessions().await;

        // Only regular user should be cleaned up, not sysop
        assert_eq!(cleaned, 1);
        assert!(manager.get_session(id1).await.is_ok()); // sysop still exists
        assert!(manager.get_session(id2).await.is_err()); // user removed
    }

    #[tokio::test]
    async fn test_warning_tracking() {
        let manager = SessionManager::new(SessionConfig::default());
        let id = manager.create_session("192.168.1.1:1234").await.unwrap();

        manager.mark_idle_warning_sent(id).await.unwrap();
        let session = manager.get_session(id).await.unwrap();
        assert!(session.is_idle_warning_sent());

        manager.mark_absolute_warning_sent(id).await.unwrap();
        let session = manager.get_session(id).await.unwrap();
        assert!(session.is_absolute_warning_sent());
        assert!(session.has_warning_sent());
    }
}
