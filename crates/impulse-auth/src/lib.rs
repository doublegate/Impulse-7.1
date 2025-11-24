//! Authentication and session management for the BBS
//!
//! This module provides password hashing, session management, and authentication
//! services for user login/logout operations.

use argon2::{
    Argon2,
    password_hash::{
        PasswordHash, PasswordHasher as Argon2PasswordHasher, PasswordVerifier, SaltString,
        rand_core::OsRng,
    },
};
use impulse_types::{
    Error,
    user::{User, UserId},
};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use thiserror::Error;
use tokio::sync::RwLock;

/// Authentication-specific errors
#[derive(Debug, Error)]
pub enum AuthError {
    /// Invalid credentials provided
    #[error("Invalid credentials")]
    InvalidCredentials,

    /// Session not found
    #[error("Session not found: {0}")]
    SessionNotFound(String),

    /// Session expired
    #[error("Session expired: {0}")]
    SessionExpired(String),

    /// Password hashing error
    #[error("Password hashing error: {0}")]
    HashingError(String),

    /// Generic authentication error
    #[error("Authentication error: {0}")]
    Generic(String),
}

impl From<AuthError> for Error {
    fn from(err: AuthError) -> Self {
        Error::Authentication(err.to_string())
    }
}

/// Session token - unique identifier for a user session
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SessionToken(String);

impl SessionToken {
    /// Create a new random session token
    pub fn new() -> Self {
        use sha2::{Digest, Sha256};
        let random_bytes: [u8; 32] = rand::random();
        let hash = Sha256::digest(random_bytes);
        SessionToken(format!("{:x}", hash))
    }

    /// Get the token string
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for SessionToken {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for SessionToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Active user session
#[derive(Debug, Clone)]
pub struct Session {
    /// Session token
    token: SessionToken,

    /// User ID associated with this session
    user_id: UserId,

    /// Session creation time
    created_at: SystemTime,

    /// Last activity time
    last_activity: SystemTime,

    /// Session timeout duration
    timeout: Duration,
}

impl Session {
    /// Create a new session
    pub fn new(user_id: UserId, timeout: Duration) -> Self {
        let now = SystemTime::now();
        Self {
            token: SessionToken::new(),
            user_id,
            created_at: now,
            last_activity: now,
            timeout,
        }
    }

    /// Get the session token
    pub fn token(&self) -> &SessionToken {
        &self.token
    }

    /// Get the user ID
    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    /// Get creation time
    pub fn created_at(&self) -> SystemTime {
        self.created_at
    }

    /// Get last activity time
    pub fn last_activity(&self) -> SystemTime {
        self.last_activity
    }

    /// Update last activity time
    pub fn touch(&mut self) {
        self.last_activity = SystemTime::now();
    }

    /// Check if session is expired
    pub fn is_expired(&self) -> bool {
        if let Ok(elapsed) = self.last_activity.elapsed() {
            elapsed > self.timeout
        } else {
            true // If we can't determine elapsed time, consider expired
        }
    }

    /// Get remaining time until expiration
    pub fn remaining_time(&self) -> Option<Duration> {
        if let Ok(elapsed) = self.last_activity.elapsed() {
            self.timeout.checked_sub(elapsed)
        } else {
            None
        }
    }
}

/// Password hasher using Argon2id
pub struct PasswordHasher {
    argon2: Argon2<'static>,
}

impl PasswordHasher {
    /// Create a new password hasher with default parameters
    pub fn new() -> Self {
        Self {
            argon2: Argon2::default(),
        }
    }

    /// Hash a password
    ///
    /// # Errors
    ///
    /// Returns `AuthError::HashingError` if hashing fails
    pub fn hash_password(&self, password: &str) -> Result<String, AuthError> {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash =
            Argon2PasswordHasher::hash_password(&self.argon2, password.as_bytes(), &salt)
                .map_err(|e| AuthError::HashingError(e.to_string()))?;
        Ok(password_hash.to_string())
    }

    /// Verify a password against a hash
    ///
    /// # Errors
    ///
    /// Returns `AuthError::InvalidCredentials` if verification fails
    pub fn verify_password(&self, password: &str, hash: &str) -> Result<(), AuthError> {
        let parsed_hash =
            PasswordHash::new(hash).map_err(|e| AuthError::HashingError(e.to_string()))?;

        self.argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| AuthError::InvalidCredentials)
    }
}

impl Default for PasswordHasher {
    fn default() -> Self {
        Self::new()
    }
}

/// Session manager for tracking active sessions
pub struct SessionManager {
    /// Active sessions by token
    sessions: Arc<RwLock<HashMap<SessionToken, Session>>>,

    /// Default session timeout
    default_timeout: Duration,
}

impl SessionManager {
    /// Create a new session manager
    pub fn new(default_timeout: Duration) -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            default_timeout,
        }
    }

    /// Create a new session for a user
    pub async fn create_session(&self, user_id: UserId) -> SessionToken {
        let session = Session::new(user_id, self.default_timeout);
        let token = session.token().clone();

        let mut sessions = self.sessions.write().await;
        sessions.insert(token.clone(), session);

        token
    }

    /// Get a session by token
    ///
    /// # Errors
    ///
    /// Returns `AuthError::SessionNotFound` if session doesn't exist
    /// Returns `AuthError::SessionExpired` if session is expired
    pub async fn get_session(&self, token: &SessionToken) -> Result<Session, AuthError> {
        let sessions = self.sessions.read().await;
        let session = sessions
            .get(token)
            .ok_or_else(|| AuthError::SessionNotFound(token.to_string()))?;

        if session.is_expired() {
            return Err(AuthError::SessionExpired(token.to_string()));
        }

        Ok(session.clone())
    }

    /// Update session activity time
    ///
    /// # Errors
    ///
    /// Returns `AuthError::SessionNotFound` if session doesn't exist
    /// Returns `AuthError::SessionExpired` if session is expired
    pub async fn touch_session(&self, token: &SessionToken) -> Result<(), AuthError> {
        let mut sessions = self.sessions.write().await;
        let session = sessions
            .get_mut(token)
            .ok_or_else(|| AuthError::SessionNotFound(token.to_string()))?;

        if session.is_expired() {
            return Err(AuthError::SessionExpired(token.to_string()));
        }

        session.touch();
        Ok(())
    }

    /// Remove a session
    pub async fn remove_session(&self, token: &SessionToken) -> bool {
        let mut sessions = self.sessions.write().await;
        sessions.remove(token).is_some()
    }

    /// Get all active sessions for a user
    pub async fn get_user_sessions(&self, user_id: UserId) -> Vec<Session> {
        let sessions = self.sessions.read().await;
        sessions
            .values()
            .filter(|s| s.user_id() == user_id && !s.is_expired())
            .cloned()
            .collect()
    }

    /// Remove all sessions for a user
    pub async fn remove_user_sessions(&self, user_id: UserId) -> usize {
        let mut sessions = self.sessions.write().await;
        let tokens: Vec<_> = sessions
            .iter()
            .filter(|(_, s)| s.user_id() == user_id)
            .map(|(t, _)| t.clone())
            .collect();

        let count = tokens.len();
        for token in tokens {
            sessions.remove(&token);
        }
        count
    }

    /// Clean up expired sessions
    pub async fn cleanup_expired(&self) -> usize {
        let mut sessions = self.sessions.write().await;
        let expired_tokens: Vec<_> = sessions
            .iter()
            .filter(|(_, s)| s.is_expired())
            .map(|(t, _)| t.clone())
            .collect();

        let count = expired_tokens.len();
        for token in expired_tokens {
            sessions.remove(&token);
        }
        count
    }

    /// Get count of active sessions
    pub async fn session_count(&self) -> usize {
        let sessions = self.sessions.read().await;
        sessions.values().filter(|s| !s.is_expired()).count()
    }
}

impl Clone for SessionManager {
    fn clone(&self) -> Self {
        Self {
            sessions: Arc::clone(&self.sessions),
            default_timeout: self.default_timeout,
        }
    }
}

/// Authentication service combining password verification and session management
pub struct AuthService {
    /// Password hasher
    hasher: PasswordHasher,

    /// Session manager
    sessions: SessionManager,
}

impl AuthService {
    /// Create a new authentication service
    pub fn new(session_timeout: Duration) -> Self {
        Self {
            hasher: PasswordHasher::new(),
            sessions: SessionManager::new(session_timeout),
        }
    }

    /// Hash a password for storage
    ///
    /// # Errors
    ///
    /// Returns `AuthError::HashingError` if hashing fails
    pub fn hash_password(&self, password: &str) -> Result<String, AuthError> {
        self.hasher.hash_password(password)
    }

    /// Authenticate a user and create a session
    ///
    /// # Arguments
    ///
    /// * `user` - The user to authenticate
    /// * `password` - The password to verify
    /// * `stored_hash` - The stored password hash for this user
    ///
    /// # Errors
    ///
    /// Returns `AuthError::InvalidCredentials` if password doesn't match
    pub async fn login(
        &self,
        user: &User,
        password: &str,
        stored_hash: &str,
    ) -> Result<SessionToken, AuthError> {
        // Verify password
        self.hasher.verify_password(password, stored_hash)?;

        // Create session
        let token = self.sessions.create_session(user.id()).await;

        Ok(token)
    }

    /// Validate a session token and return the user ID
    ///
    /// # Errors
    ///
    /// Returns `AuthError::SessionNotFound` if session doesn't exist
    /// Returns `AuthError::SessionExpired` if session is expired
    pub async fn validate_session(&self, token: &SessionToken) -> Result<UserId, AuthError> {
        let session = self.sessions.get_session(token).await?;
        self.sessions.touch_session(token).await?;
        Ok(session.user_id())
    }

    /// Log out a user by removing their session
    pub async fn logout(&self, token: &SessionToken) -> bool {
        self.sessions.remove_session(token).await
    }

    /// Log out all sessions for a user
    pub async fn logout_all(&self, user_id: UserId) -> usize {
        self.sessions.remove_user_sessions(user_id).await
    }

    /// Get active sessions for a user
    pub async fn get_user_sessions(&self, user_id: UserId) -> Vec<Session> {
        self.sessions.get_user_sessions(user_id).await
    }

    /// Clean up expired sessions
    pub async fn cleanup_expired_sessions(&self) -> usize {
        self.sessions.cleanup_expired().await
    }

    /// Get count of active sessions
    pub async fn active_session_count(&self) -> usize {
        self.sessions.session_count().await
    }
}

impl Clone for AuthService {
    fn clone(&self) -> Self {
        Self {
            hasher: PasswordHasher::new(),
            sessions: self.sessions.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing() {
        let hasher = PasswordHasher::new();
        let password = "test_password_123";

        // Hash password
        let hash = hasher.hash_password(password).unwrap();
        assert!(!hash.is_empty());

        // Verify correct password
        assert!(hasher.verify_password(password, &hash).is_ok());

        // Verify incorrect password
        assert!(hasher.verify_password("wrong_password", &hash).is_err());
    }

    #[test]
    fn test_password_hash_uniqueness() {
        let hasher = PasswordHasher::new();
        let password = "test_password";

        let hash1 = hasher.hash_password(password).unwrap();
        let hash2 = hasher.hash_password(password).unwrap();

        // Hashes should be different due to random salt
        assert_ne!(hash1, hash2);

        // Both should verify correctly
        assert!(hasher.verify_password(password, &hash1).is_ok());
        assert!(hasher.verify_password(password, &hash2).is_ok());
    }

    #[test]
    fn test_session_token_creation() {
        let token1 = SessionToken::new();
        let token2 = SessionToken::new();

        // Tokens should be unique
        assert_ne!(token1, token2);

        // Tokens should be non-empty
        assert!(!token1.as_str().is_empty());
        assert!(!token2.as_str().is_empty());
    }

    #[test]
    fn test_session_creation() {
        let user_id = UserId::new();
        let timeout = Duration::from_secs(3600);
        let session = Session::new(user_id, timeout);

        assert_eq!(session.user_id(), user_id);
        assert!(!session.is_expired());
        assert!(session.remaining_time().is_some());
    }

    #[test]
    fn test_session_expiration() {
        let user_id = UserId::new();
        let timeout = Duration::from_millis(1);
        let session = Session::new(user_id, timeout);

        // Session should not be expired immediately
        assert!(!session.is_expired());

        // Wait for expiration
        std::thread::sleep(Duration::from_millis(10));

        // Session should now be expired
        assert!(session.is_expired());
        assert!(session.remaining_time().is_none());
    }

    #[tokio::test]
    async fn test_session_manager_create() {
        let manager = SessionManager::new(Duration::from_secs(3600));
        let user_id = UserId::new();

        let token = manager.create_session(user_id).await;
        let session = manager.get_session(&token).await.unwrap();

        assert_eq!(session.user_id(), user_id);
        assert!(!session.is_expired());
    }

    #[tokio::test]
    async fn test_session_manager_touch() {
        let manager = SessionManager::new(Duration::from_secs(3600));
        let user_id = UserId::new();
        let token = manager.create_session(user_id).await;

        let session_before = manager.get_session(&token).await.unwrap();

        // Wait a bit and touch session
        tokio::time::sleep(Duration::from_millis(100)).await;
        manager.touch_session(&token).await.unwrap();

        let session_after = manager.get_session(&token).await.unwrap();

        // Last activity should be updated
        assert!(session_after.last_activity() > session_before.last_activity());
    }

    #[tokio::test]
    async fn test_session_manager_remove() {
        let manager = SessionManager::new(Duration::from_secs(3600));
        let user_id = UserId::new();
        let token = manager.create_session(user_id).await;

        // Session should exist
        assert!(manager.get_session(&token).await.is_ok());

        // Remove session
        assert!(manager.remove_session(&token).await);

        // Session should not exist
        assert!(manager.get_session(&token).await.is_err());
    }

    #[tokio::test]
    async fn test_session_manager_user_sessions() {
        let manager = SessionManager::new(Duration::from_secs(3600));
        let user_id = UserId::new();

        // Create multiple sessions for same user
        manager.create_session(user_id).await;
        manager.create_session(user_id).await;
        manager.create_session(user_id).await;

        let sessions = manager.get_user_sessions(user_id).await;
        assert_eq!(sessions.len(), 3);
    }

    #[tokio::test]
    async fn test_session_manager_remove_user_sessions() {
        let manager = SessionManager::new(Duration::from_secs(3600));
        let user_id = UserId::new();

        // Create sessions
        manager.create_session(user_id).await;
        manager.create_session(user_id).await;

        // Remove all user sessions
        let count = manager.remove_user_sessions(user_id).await;
        assert_eq!(count, 2);

        // No sessions should remain
        let sessions = manager.get_user_sessions(user_id).await;
        assert_eq!(sessions.len(), 0);
    }

    #[tokio::test]
    async fn test_session_manager_cleanup() {
        let manager = SessionManager::new(Duration::from_millis(10));
        let user_id = UserId::new();

        // Create sessions
        manager.create_session(user_id).await;
        manager.create_session(user_id).await;

        // Wait for expiration
        tokio::time::sleep(Duration::from_millis(20)).await;

        // Cleanup expired sessions
        let count = manager.cleanup_expired().await;
        assert_eq!(count, 2);

        // Session count should be 0
        assert_eq!(manager.session_count().await, 0);
    }

    #[tokio::test]
    async fn test_auth_service_login() {
        let auth = AuthService::new(Duration::from_secs(3600));
        let password = "test_password";

        // Create a user and hash password separately
        let hash = auth.hash_password(password).unwrap();
        let user = User::new("testuser").unwrap();

        // Login should succeed
        let token = auth.login(&user, password, &hash).await.unwrap();
        assert!(!token.as_str().is_empty());

        // Session should be valid
        let user_id = auth.validate_session(&token).await.unwrap();
        assert_eq!(user_id, user.id());
    }

    #[tokio::test]
    async fn test_auth_service_invalid_password() {
        let auth = AuthService::new(Duration::from_secs(3600));

        let hash = auth.hash_password("correct_password").unwrap();
        let user = User::new("testuser").unwrap();

        // Login with wrong password should fail
        let result = auth.login(&user, "wrong_password", &hash).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_auth_service_logout() {
        let auth = AuthService::new(Duration::from_secs(3600));
        let password = "test_password";

        let hash = auth.hash_password(password).unwrap();
        let user = User::new("testuser").unwrap();

        // Login
        let token = auth.login(&user, password, &hash).await.unwrap();

        // Logout
        assert!(auth.logout(&token).await);

        // Session should be invalid
        assert!(auth.validate_session(&token).await.is_err());
    }

    #[tokio::test]
    async fn test_auth_service_logout_all() {
        let auth = AuthService::new(Duration::from_secs(3600));
        let password = "test_password";

        let hash = auth.hash_password(password).unwrap();
        let user = User::new("testuser").unwrap();

        // Create multiple sessions
        auth.login(&user, password, &hash).await.unwrap();
        auth.login(&user, password, &hash).await.unwrap();
        auth.login(&user, password, &hash).await.unwrap();

        // Verify sessions exist
        let sessions = auth.get_user_sessions(user.id()).await;
        assert_eq!(sessions.len(), 3);

        // Logout all
        let count = auth.logout_all(user.id()).await;
        assert_eq!(count, 3);

        // No sessions should remain
        let sessions = auth.get_user_sessions(user.id()).await;
        assert_eq!(sessions.len(), 0);
    }

    #[tokio::test]
    async fn test_auth_service_cleanup() {
        let auth = AuthService::new(Duration::from_millis(10));
        let password = "test_password";

        let hash = auth.hash_password(password).unwrap();
        let user = User::new("testuser").unwrap();

        // Create sessions
        auth.login(&user, password, &hash).await.unwrap();
        auth.login(&user, password, &hash).await.unwrap();

        // Wait for expiration
        tokio::time::sleep(Duration::from_millis(20)).await;

        // Cleanup
        let count = auth.cleanup_expired_sessions().await;
        assert_eq!(count, 2);
        assert_eq!(auth.active_session_count().await, 0);
    }
}
