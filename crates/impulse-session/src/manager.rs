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
        // Check sessions per user limit
        {
            let user_sessions = self.user_sessions.read().await;
            if let Some(sessions) = user_sessions.get(&username) {
                if sessions.len() >= self.config.max_sessions_per_user {
                    return Err(SessionError::TooManySessions {
                        limit: self.config.max_sessions_per_user,
                    });
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

    /// Clean up expired sessions
    pub async fn cleanup_expired_sessions(&self) -> usize {
        let mut expired = Vec::new();

        {
            let sessions = self.sessions.read().await;
            for (id, session) in sessions.iter() {
                if session.state() == SessionState::Terminated
                    || session.is_idle(self.config.idle_timeout)
                {
                    expired.push(*id);
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
}
