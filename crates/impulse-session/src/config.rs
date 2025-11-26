//! Session configuration

use std::time::Duration;

/// Session conflict resolution policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictPolicy {
    /// Allow multiple sessions (up to max_sessions_per_user)
    Allow,
    /// Kick the oldest session when limit is reached
    KickOldest,
    /// Deny the new login when limit is reached
    DenyNew,
}

/// Session manager configuration
#[derive(Debug, Clone)]
pub struct SessionConfig {
    /// Maximum idle time before session is terminated (default: 15 minutes)
    pub idle_timeout: Duration,
    /// Maximum session duration (default: 4 hours, None for unlimited)
    pub absolute_timeout: Option<Duration>,
    /// Time before timeout to send warning (default: 1 minute)
    pub warning_before_timeout: Duration,
    /// Maximum concurrent sessions per user (default: 3)
    pub max_sessions_per_user: usize,
    /// Maximum total concurrent sessions (default: 100)
    pub max_total_sessions: usize,
    /// Session cleanup interval (default: 1 minute)
    pub cleanup_interval: Duration,
    /// Session conflict resolution policy (default: Allow)
    pub conflict_policy: ConflictPolicy,
    /// Users with unlimited session time (e.g., sysop)
    pub unlimited_users: Vec<String>,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            idle_timeout: Duration::from_secs(900), // 15 minutes
            absolute_timeout: Some(Duration::from_secs(14400)), // 4 hours
            warning_before_timeout: Duration::from_secs(60), // 1 minute
            max_sessions_per_user: 3,
            max_total_sessions: 100,
            cleanup_interval: Duration::from_secs(60), // 1 minute
            conflict_policy: ConflictPolicy::Allow,
            unlimited_users: Vec::new(),
        }
    }
}

impl SessionConfig {
    /// Create a new configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Set idle timeout
    pub fn with_idle_timeout(mut self, timeout: Duration) -> Self {
        self.idle_timeout = timeout;
        self
    }

    /// Set absolute timeout (None for unlimited)
    pub fn with_absolute_timeout(mut self, timeout: Option<Duration>) -> Self {
        self.absolute_timeout = timeout;
        self
    }

    /// Set warning before timeout
    pub fn with_warning_before_timeout(mut self, duration: Duration) -> Self {
        self.warning_before_timeout = duration;
        self
    }

    /// Set maximum sessions per user
    pub fn with_max_sessions_per_user(mut self, max: usize) -> Self {
        self.max_sessions_per_user = max;
        self
    }

    /// Set maximum total sessions
    pub fn with_max_total_sessions(mut self, max: usize) -> Self {
        self.max_total_sessions = max;
        self
    }

    /// Set cleanup interval
    pub fn with_cleanup_interval(mut self, interval: Duration) -> Self {
        self.cleanup_interval = interval;
        self
    }

    /// Set conflict resolution policy
    pub fn with_conflict_policy(mut self, policy: ConflictPolicy) -> Self {
        self.conflict_policy = policy;
        self
    }

    /// Add a user with unlimited session time
    pub fn with_unlimited_user(mut self, username: String) -> Self {
        self.unlimited_users.push(username);
        self
    }

    /// Set users with unlimited session time
    pub fn with_unlimited_users(mut self, users: Vec<String>) -> Self {
        self.unlimited_users = users;
        self
    }

    /// Check if a user has unlimited session time
    pub fn is_unlimited_user(&self, username: &str) -> bool {
        self.unlimited_users.iter().any(|u| u == username)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = SessionConfig::default();
        assert_eq!(config.idle_timeout, Duration::from_secs(900));
        assert_eq!(config.absolute_timeout, Some(Duration::from_secs(14400)));
        assert_eq!(config.warning_before_timeout, Duration::from_secs(60));
        assert_eq!(config.max_sessions_per_user, 3);
        assert_eq!(config.max_total_sessions, 100);
        assert_eq!(config.conflict_policy, ConflictPolicy::Allow);
        assert!(config.unlimited_users.is_empty());
    }

    #[test]
    fn test_config_builder() {
        let config = SessionConfig::new()
            .with_idle_timeout(Duration::from_secs(1800))
            .with_max_sessions_per_user(5)
            .with_absolute_timeout(None)
            .with_conflict_policy(ConflictPolicy::KickOldest);

        assert_eq!(config.idle_timeout, Duration::from_secs(1800));
        assert_eq!(config.max_sessions_per_user, 5);
        assert_eq!(config.absolute_timeout, None);
        assert_eq!(config.conflict_policy, ConflictPolicy::KickOldest);
    }

    #[test]
    fn test_unlimited_users() {
        let config = SessionConfig::new()
            .with_unlimited_user("sysop".to_string())
            .with_unlimited_user("admin".to_string());

        assert!(config.is_unlimited_user("sysop"));
        assert!(config.is_unlimited_user("admin"));
        assert!(!config.is_unlimited_user("user"));
    }

    #[test]
    fn test_conflict_policies() {
        assert_eq!(ConflictPolicy::Allow, ConflictPolicy::Allow);
        assert_ne!(ConflictPolicy::Allow, ConflictPolicy::KickOldest);
        assert_ne!(ConflictPolicy::KickOldest, ConflictPolicy::DenyNew);
    }
}
