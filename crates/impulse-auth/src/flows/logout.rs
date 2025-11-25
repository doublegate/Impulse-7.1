//! Logout flow implementation
//!
//! Handles session termination for individual sessions or all sessions
//! associated with a user account.

use crate::{AuthService, SessionToken};
use impulse_types::user::UserId;
use std::sync::Arc;

/// Logout result
///
/// Represents the outcome of a logout attempt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogoutResult {
    /// Logout successful - session invalidated
    Success,

    /// Session token not found or already invalidated
    InvalidSession,

    /// Session was already logged out
    AlreadyLoggedOut,
}

/// Logout flow handler
///
/// Coordinates session termination by invalidating session tokens
/// and cleaning up session state.
///
/// # Examples
///
/// ```no_run
/// use impulse_auth::{AuthService, SessionToken, flows::logout::LogoutFlow};
/// use std::sync::Arc;
/// use std::time::Duration;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let auth = Arc::new(AuthService::new(Duration::from_secs(1800)));
/// let logout_flow = LogoutFlow::new(auth);
///
/// let token = SessionToken::new();
/// let result = logout_flow.execute(&token.to_string()).await;
/// match result {
///     impulse_auth::flows::logout::LogoutResult::Success => {
///         println!("Logged out successfully");
///     }
///     _ => println!("Logout failed"),
/// }
/// # Ok(())
/// # }
/// ```
pub struct LogoutFlow {
    auth_service: Arc<AuthService>,
}

impl LogoutFlow {
    /// Create a new logout flow handler
    ///
    /// # Arguments
    ///
    /// * `auth_service` - Shared authentication service
    pub fn new(auth_service: Arc<AuthService>) -> Self {
        Self { auth_service }
    }

    /// Execute logout - invalidate session
    ///
    /// Removes the session from the active session store, effectively
    /// logging out the user. The session token cannot be used for
    /// authentication after this call.
    ///
    /// # Arguments
    ///
    /// * `session_token` - Session token string to invalidate
    ///
    /// # Returns
    ///
    /// Returns a `LogoutResult` indicating success or failure reason
    pub async fn execute(&self, session_token: &str) -> LogoutResult {
        // Parse session token
        let token = SessionToken::from_string(session_token);

        // Attempt to remove session
        match self.auth_service.logout(&token).await {
            true => {
                tracing::info!(
                    token = %session_token,
                    "Logout successful"
                );
                LogoutResult::Success
            }
            false => {
                tracing::warn!(
                    token = %session_token,
                    "Logout failed: session not found"
                );
                LogoutResult::InvalidSession
            }
        }
    }

    /// Force logout all sessions for a user (admin function)
    ///
    /// Invalidates all active sessions associated with a user account.
    /// Useful for:
    /// - Security incidents (compromised account)
    /// - Password changes
    /// - Account suspension
    /// - Administrative actions
    ///
    /// # Arguments
    ///
    /// * `user_id` - User ID whose sessions should be terminated
    ///
    /// # Returns
    ///
    /// Returns the number of sessions that were logged out
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use impulse_auth::{AuthService, flows::logout::LogoutFlow};
    /// use impulse_types::user::UserId;
    /// use std::sync::Arc;
    /// use std::time::Duration;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let auth = Arc::new(AuthService::new(Duration::from_secs(1800)));
    /// let logout_flow = LogoutFlow::new(auth);
    ///
    /// let user_id = UserId::new();
    /// let count = logout_flow.logout_all_sessions(&user_id).await;
    /// println!("Logged out {} sessions", count);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn logout_all_sessions(&self, user_id: &UserId) -> u32 {
        let count = self.auth_service.logout_all(*user_id).await as u32;
        tracing::info!(
            user_id = ?user_id,
            session_count = count,
            "Logged out all user sessions"
        );
        count
    }

    /// Check if a session is valid before attempting logout
    ///
    /// Can be used to distinguish between invalid sessions and
    /// already-logged-out sessions in the UI.
    ///
    /// # Arguments
    ///
    /// * `session_token` - Session token string to check
    ///
    /// # Returns
    ///
    /// Returns `true` if session is valid, `false` otherwise
    pub async fn is_session_valid(&self, session_token: &str) -> bool {
        let token = SessionToken::from_string(session_token);
        self.auth_service.validate_session(&token).await.is_ok()
    }
}

impl SessionToken {
    /// Create a SessionToken from a string
    ///
    /// This is a convenience method for parsing session tokens
    /// from string representations.
    fn from_string(s: &str) -> Self {
        SessionToken(s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use impulse_types::user::User;
    use std::time::Duration;

    #[tokio::test]
    async fn test_logout_success() {
        let auth = Arc::new(AuthService::new(Duration::from_secs(1800)));
        let flow = LogoutFlow::new(Arc::clone(&auth));

        // Create a session first
        let user = User::new("testuser").unwrap();
        let password = "TestPassword123!";
        let hash = auth.hash_password(password).unwrap();
        let token = auth.login(&user, password, &hash).await.unwrap();

        // Verify session is valid
        assert!(auth.validate_session(&token).await.is_ok());

        // Logout
        let result = flow.execute(token.as_str()).await;
        assert_eq!(result, LogoutResult::Success);

        // Verify session is now invalid
        assert!(auth.validate_session(&token).await.is_err());
    }

    #[tokio::test]
    async fn test_logout_invalid_session() {
        let auth = Arc::new(AuthService::new(Duration::from_secs(1800)));
        let flow = LogoutFlow::new(auth);

        // Try to logout non-existent session
        let result = flow.execute("invalid_token_12345").await;
        assert_eq!(result, LogoutResult::InvalidSession);
    }

    #[tokio::test]
    async fn test_logout_all_sessions() {
        let auth = Arc::new(AuthService::new(Duration::from_secs(1800)));
        let flow = LogoutFlow::new(Arc::clone(&auth));

        // Create multiple sessions for same user
        let user = User::new("testuser").unwrap();
        let password = "TestPassword123!";
        let hash = auth.hash_password(password).unwrap();

        let token1 = auth.login(&user, password, &hash).await.unwrap();
        let token2 = auth.login(&user, password, &hash).await.unwrap();
        let token3 = auth.login(&user, password, &hash).await.unwrap();

        // Verify all sessions are valid
        assert!(auth.validate_session(&token1).await.is_ok());
        assert!(auth.validate_session(&token2).await.is_ok());
        assert!(auth.validate_session(&token3).await.is_ok());

        // Logout all sessions
        let count = flow.logout_all_sessions(&user.id()).await;
        assert_eq!(count, 3);

        // Verify all sessions are now invalid
        assert!(auth.validate_session(&token1).await.is_err());
        assert!(auth.validate_session(&token2).await.is_err());
        assert!(auth.validate_session(&token3).await.is_err());
    }

    #[tokio::test]
    async fn test_is_session_valid() {
        let auth = Arc::new(AuthService::new(Duration::from_secs(1800)));
        let flow = LogoutFlow::new(Arc::clone(&auth));

        // Create a session
        let user = User::new("testuser").unwrap();
        let password = "TestPassword123!";
        let hash = auth.hash_password(password).unwrap();
        let token = auth.login(&user, password, &hash).await.unwrap();

        // Check validity
        assert!(flow.is_session_valid(token.as_str()).await);

        // Logout
        flow.execute(token.as_str()).await;

        // Check validity after logout
        assert!(!flow.is_session_valid(token.as_str()).await);
    }

    #[tokio::test]
    async fn test_logout_all_sessions_no_sessions() {
        let auth = Arc::new(AuthService::new(Duration::from_secs(1800)));
        let flow = LogoutFlow::new(auth);

        // Try to logout all sessions for user with no sessions
        let user = User::new("testuser").unwrap();
        let count = flow.logout_all_sessions(&user.id()).await;
        assert_eq!(count, 0);
    }

    #[tokio::test]
    async fn test_logout_does_not_affect_other_users() {
        let auth = Arc::new(AuthService::new(Duration::from_secs(1800)));
        let flow = LogoutFlow::new(Arc::clone(&auth));

        // Create sessions for two different users
        let user1 = User::new("user1").unwrap();
        let user2 = User::new("user2").unwrap();
        let password = "TestPassword123!";
        let hash = auth.hash_password(password).unwrap();

        let token1 = auth.login(&user1, password, &hash).await.unwrap();
        let token2 = auth.login(&user2, password, &hash).await.unwrap();

        // Logout user1
        flow.execute(token1.as_str()).await;

        // Verify user1 session is invalid
        assert!(auth.validate_session(&token1).await.is_err());

        // Verify user2 session is still valid
        assert!(auth.validate_session(&token2).await.is_ok());
    }
}
