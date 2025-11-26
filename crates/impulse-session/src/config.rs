//! Session configuration

use std::time::Duration;

/// Session manager configuration
#[derive(Debug, Clone)]
pub struct SessionConfig {
    /// Maximum idle time before session is terminated (default: 15 minutes)
    pub idle_timeout: Duration,
    /// Maximum concurrent sessions per user (default: 3)
    pub max_sessions_per_user: usize,
    /// Maximum total concurrent sessions (default: 100)
    pub max_total_sessions: usize,
    /// Session cleanup interval (default: 1 minute)
    pub cleanup_interval: Duration,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            idle_timeout: Duration::from_secs(900), // 15 minutes
            max_sessions_per_user: 3,
            max_total_sessions: 100,
            cleanup_interval: Duration::from_secs(60), // 1 minute
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = SessionConfig::default();
        assert_eq!(config.idle_timeout, Duration::from_secs(900));
        assert_eq!(config.max_sessions_per_user, 3);
        assert_eq!(config.max_total_sessions, 100);
    }

    #[test]
    fn test_config_builder() {
        let config = SessionConfig::new()
            .with_idle_timeout(Duration::from_secs(1800))
            .with_max_sessions_per_user(5);

        assert_eq!(config.idle_timeout, Duration::from_secs(1800));
        assert_eq!(config.max_sessions_per_user, 5);
    }
}
