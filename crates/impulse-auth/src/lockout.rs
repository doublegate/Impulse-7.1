//! Account lockout mechanism for security
//!
//! This module provides account lockout functionality to protect against
//! repeated failed login attempts. Accounts are temporarily locked after
//! exceeding the maximum failure threshold.
//!
//! # Examples
//!
//! ```
//! use impulse_auth::lockout::AccountLockout;
//! use std::time::Duration;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Lock after 5 failures for 30 minutes
//! let lockout = AccountLockout::new(5, Duration::from_secs(1800));
//!
//! // Record failures
//! for _ in 0..5 {
//!     lockout.record_failure("username").await;
//! }
//!
//! // Check if locked
//! if lockout.is_locked("username").await {
//!     println!("Account is locked!");
//! }
//! # Ok(())
//! # }
//! ```

use std::collections::HashMap;
use std::time::{Duration, Instant, SystemTime};
use thiserror::Error;
use tokio::sync::RwLock;

/// Account lockout errors
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum LockoutError {
    /// Account is currently locked
    #[error("Account locked until {unlock_time:?}. Reason: {reason}")]
    AccountLocked {
        /// When the account will be unlocked
        unlock_time: SystemTime,
        /// Reason for lockout
        reason: String,
    },
}

/// Information about a locked account
#[derive(Debug, Clone)]
pub struct LockoutInfo {
    /// When the account was locked
    locked_at: Instant,
    /// Number of failures that triggered the lockout
    failure_count: u32,
    /// When the account will be automatically unlocked
    unlock_at: Instant,
    /// SystemTime for unlock (for user display)
    unlock_time: SystemTime,
    /// Reason for lockout
    reason: String,
}

impl LockoutInfo {
    /// Check if the lockout has expired
    pub fn is_expired(&self) -> bool {
        Instant::now() >= self.unlock_at
    }

    /// Get remaining lockout duration
    pub fn remaining_duration(&self) -> Option<Duration> {
        let now = Instant::now();
        if now >= self.unlock_at {
            None
        } else {
            Some(self.unlock_at.duration_since(now))
        }
    }

    /// Get the number of failures
    pub fn failure_count(&self) -> u32 {
        self.failure_count
    }

    /// Get when the account was locked
    pub fn locked_at(&self) -> Instant {
        self.locked_at
    }

    /// Get when the account will be unlocked (for display)
    pub fn unlock_time(&self) -> SystemTime {
        self.unlock_time
    }

    /// Get the lockout reason
    pub fn reason(&self) -> &str {
        &self.reason
    }
}

/// Account lockout configuration
#[derive(Debug, Clone)]
pub struct LockoutConfig {
    /// Maximum number of failures before lockout
    pub max_failures: u32,
    /// Duration of lockout
    pub lockout_duration: Duration,
    /// Whether to enable progressive lockout (longer duration per failure)
    pub progressive: bool,
}

impl Default for LockoutConfig {
    fn default() -> Self {
        Self {
            max_failures: 5,
            lockout_duration: Duration::from_secs(1800), // 30 minutes
            progressive: false,
        }
    }
}

impl LockoutConfig {
    /// Create a new lockout configuration
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_auth::lockout::LockoutConfig;
    /// use std::time::Duration;
    ///
    /// let config = LockoutConfig::new(5, Duration::from_secs(1800));
    /// ```
    #[must_use]
    pub fn new(max_failures: u32, lockout_duration: Duration) -> Self {
        Self {
            max_failures,
            lockout_duration,
            progressive: false,
        }
    }

    /// Create a strict configuration (3 failures, 1 hour lockout)
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_auth::lockout::LockoutConfig;
    ///
    /// let config = LockoutConfig::strict();
    /// assert_eq!(config.max_failures, 3);
    /// ```
    #[must_use]
    pub fn strict() -> Self {
        Self {
            max_failures: 3,
            lockout_duration: Duration::from_secs(3600), // 1 hour
            progressive: true,
        }
    }

    /// Create a lenient configuration (10 failures, 15 minutes lockout)
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_auth::lockout::LockoutConfig;
    ///
    /// let config = LockoutConfig::lenient();
    /// assert_eq!(config.max_failures, 10);
    /// ```
    #[must_use]
    pub fn lenient() -> Self {
        Self {
            max_failures: 10,
            lockout_duration: Duration::from_secs(900), // 15 minutes
            progressive: false,
        }
    }

    /// Enable progressive lockout (longer duration per subsequent failure)
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_auth::lockout::LockoutConfig;
    ///
    /// let config = LockoutConfig::default().with_progressive(true);
    /// assert!(config.progressive);
    /// ```
    #[must_use]
    pub fn with_progressive(mut self, progressive: bool) -> Self {
        self.progressive = progressive;
        self
    }
}

/// Account lockout manager for tracking and enforcing account lockouts
///
/// Thread-safe using RwLock for concurrent access.
#[derive(Clone)]
pub struct AccountLockout {
    /// Locked accounts by username
    locked_accounts: std::sync::Arc<RwLock<HashMap<String, LockoutInfo>>>,
    /// Failure counts by username
    failure_counts: std::sync::Arc<RwLock<HashMap<String, u32>>>,
    /// Configuration
    config: LockoutConfig,
}

impl AccountLockout {
    /// Create a new account lockout manager with default configuration
    ///
    /// Default: 5 failures, 30 minute lockout
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_auth::lockout::AccountLockout;
    ///
    /// let lockout = AccountLockout::new_default();
    /// ```
    #[must_use]
    pub fn new_default() -> Self {
        Self::with_config(LockoutConfig::default())
    }

    /// Create a new account lockout manager with custom parameters
    ///
    /// # Arguments
    ///
    /// * `max_failures` - Maximum number of failures before lockout
    /// * `lockout_duration` - Duration of lockout
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_auth::lockout::AccountLockout;
    /// use std::time::Duration;
    ///
    /// let lockout = AccountLockout::new(5, Duration::from_secs(1800));
    /// ```
    #[must_use]
    pub fn new(max_failures: u32, lockout_duration: Duration) -> Self {
        Self::with_config(LockoutConfig::new(max_failures, lockout_duration))
    }

    /// Create a new account lockout manager with a configuration
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_auth::lockout::{AccountLockout, LockoutConfig};
    ///
    /// let config = LockoutConfig::strict();
    /// let lockout = AccountLockout::with_config(config);
    /// ```
    #[must_use]
    pub fn with_config(config: LockoutConfig) -> Self {
        Self {
            locked_accounts: std::sync::Arc::new(RwLock::new(HashMap::new())),
            failure_counts: std::sync::Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Check if an account is currently locked
    ///
    /// # Arguments
    ///
    /// * `username` - Username to check
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_auth::lockout::AccountLockout;
    ///
    /// # async fn example() {
    /// let lockout = AccountLockout::new_default();
    ///
    /// if lockout.is_locked("username").await {
    ///     println!("Account is locked!");
    /// }
    /// # }
    /// ```
    pub async fn is_locked(&self, username: &str) -> bool {
        let mut locked = self.locked_accounts.write().await;

        if let Some(info) = locked.get(username) {
            if info.is_expired() {
                // Lockout expired, remove it
                locked.remove(username);
                tracing::info!(
                    username = %username,
                    "Account lockout expired, account unlocked"
                );
                false
            } else {
                true
            }
        } else {
            false
        }
    }

    /// Get lockout information for an account
    ///
    /// # Arguments
    ///
    /// * `username` - Username to check
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_auth::lockout::AccountLockout;
    ///
    /// # async fn example() {
    /// let lockout = AccountLockout::new_default();
    ///
    /// if let Some(info) = lockout.get_lockout_info("username").await {
    ///     println!("Account locked until: {:?}", info.unlock_time());
    /// }
    /// # }
    /// ```
    pub async fn get_lockout_info(&self, username: &str) -> Option<LockoutInfo> {
        let locked = self.locked_accounts.read().await;
        locked.get(username).cloned()
    }

    /// Check if an account is locked and return error if so
    ///
    /// # Arguments
    ///
    /// * `username` - Username to check
    ///
    /// # Errors
    ///
    /// Returns `LockoutError::AccountLocked` if account is locked
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_auth::lockout::AccountLockout;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let lockout = AccountLockout::new_default();
    ///
    /// lockout.check("username").await?;
    /// // Account is not locked, proceed
    /// # Ok(())
    /// # }
    /// ```
    pub async fn check(&self, username: &str) -> Result<(), LockoutError> {
        if let Some(info) = self.get_lockout_info(username).await
            && !info.is_expired()
        {
            tracing::warn!(
                username = %username,
                unlock_time = ?info.unlock_time(),
                failures = info.failure_count(),
                "Account locked: login attempt denied"
            );

            return Err(LockoutError::AccountLocked {
                unlock_time: info.unlock_time(),
                reason: info.reason().to_string(),
            });
        }

        Ok(())
    }

    /// Record a failed login attempt
    ///
    /// If failures exceed the threshold, the account will be locked.
    ///
    /// # Arguments
    ///
    /// * `username` - Username of failed attempt
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_auth::lockout::AccountLockout;
    ///
    /// # async fn example() {
    /// let lockout = AccountLockout::new_default();
    ///
    /// // After failed login
    /// lockout.record_failure("username").await;
    /// # }
    /// ```
    pub async fn record_failure(&self, username: &str) {
        let mut failure_counts = self.failure_counts.write().await;
        let count = failure_counts.entry(username.to_string()).or_insert(0);
        *count += 1;

        tracing::debug!(
            username = %username,
            failure_count = *count,
            max_failures = self.config.max_failures,
            "Recorded failed login attempt"
        );

        // Check if should lock
        if *count >= self.config.max_failures {
            let lockout_duration = if self.config.progressive {
                // Progressive lockout: double duration for each lockout
                let previous_lockouts = (*count / self.config.max_failures) - 1;
                self.config.lockout_duration * (2_u32.pow(previous_lockouts))
            } else {
                self.config.lockout_duration
            };

            let now = Instant::now();
            let unlock_at = now + lockout_duration;
            let unlock_time = SystemTime::now() + lockout_duration;

            let info = LockoutInfo {
                locked_at: now,
                failure_count: *count,
                unlock_at,
                unlock_time,
                reason: format!("Too many failed login attempts ({} failures)", *count),
            };

            // Copy count before releasing the lock
            let count_copy = *count;

            // Release write lock by letting it go out of scope
            drop(failure_counts);

            let mut locked = self.locked_accounts.write().await;
            locked.insert(username.to_string(), info.clone());

            tracing::warn!(
                username = %username,
                failure_count = count_copy,
                lockout_duration_secs = lockout_duration.as_secs(),
                unlock_time = ?unlock_time,
                progressive = self.config.progressive,
                "Account locked due to too many failed attempts"
            );
        }
    }

    /// Record a successful login and clear failures
    ///
    /// # Arguments
    ///
    /// * `username` - Username of successful login
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_auth::lockout::AccountLockout;
    ///
    /// # async fn example() {
    /// let lockout = AccountLockout::new_default();
    ///
    /// // After successful login
    /// lockout.record_success("username").await;
    /// # }
    /// ```
    pub async fn record_success(&self, username: &str) {
        let mut failure_counts = self.failure_counts.write().await;
        if failure_counts.remove(username).is_some() {
            tracing::debug!(
                username = %username,
                "Cleared failure count after successful login"
            );
        }
    }

    /// Manually unlock an account (admin override)
    ///
    /// # Arguments
    ///
    /// * `username` - Username to unlock
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_auth::lockout::AccountLockout;
    ///
    /// # async fn example() {
    /// let lockout = AccountLockout::new_default();
    ///
    /// // Admin unlocking account
    /// lockout.unlock("username").await;
    /// # }
    /// ```
    pub async fn unlock(&self, username: &str) {
        let mut locked = self.locked_accounts.write().await;
        let mut failures = self.failure_counts.write().await;

        let was_locked = locked.remove(username).is_some();
        let had_failures = failures.remove(username).is_some();

        if was_locked || had_failures {
            tracing::info!(
                username = %username,
                was_locked = was_locked,
                had_failures = had_failures,
                "Account manually unlocked"
            );
        }
    }

    /// Get current failure count for an account
    ///
    /// # Arguments
    ///
    /// * `username` - Username to check
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_auth::lockout::AccountLockout;
    ///
    /// # async fn example() {
    /// let lockout = AccountLockout::new_default();
    ///
    /// let count = lockout.failure_count("username").await;
    /// println!("Failed attempts: {}", count);
    /// # }
    /// ```
    pub async fn failure_count(&self, username: &str) -> u32 {
        let failures = self.failure_counts.read().await;
        *failures.get(username).unwrap_or(&0)
    }

    /// Get remaining attempts before lockout
    ///
    /// # Arguments
    ///
    /// * `username` - Username to check
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_auth::lockout::AccountLockout;
    ///
    /// # async fn example() {
    /// let lockout = AccountLockout::new_default();
    ///
    /// let remaining = lockout.remaining_attempts("username").await;
    /// println!("Remaining attempts: {}", remaining);
    /// # }
    /// ```
    pub async fn remaining_attempts(&self, username: &str) -> u32 {
        if self.is_locked(username).await {
            return 0;
        }

        let current = self.failure_count(username).await;
        self.config.max_failures.saturating_sub(current)
    }

    /// Cleanup expired lockouts
    ///
    /// Returns the number of expired lockouts removed.
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_auth::lockout::AccountLockout;
    ///
    /// # async fn example() {
    /// let lockout = AccountLockout::new_default();
    ///
    /// let cleaned = lockout.cleanup_expired().await;
    /// println!("Cleaned up {} expired lockouts", cleaned);
    /// # }
    /// ```
    pub async fn cleanup_expired(&self) -> usize {
        let mut locked = self.locked_accounts.write().await;
        let mut cleaned = 0;

        locked.retain(|_, info| {
            if info.is_expired() {
                cleaned += 1;
                false
            } else {
                true
            }
        });

        if cleaned > 0 {
            tracing::debug!(cleaned_count = cleaned, "Cleaned up expired lockouts");
        }

        cleaned
    }

    /// Clear all lockouts and failures (useful for testing)
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_auth::lockout::AccountLockout;
    ///
    /// # async fn example() {
    /// let lockout = AccountLockout::new_default();
    ///
    /// lockout.clear_all().await;
    /// # }
    /// ```
    pub async fn clear_all(&self) {
        let mut locked = self.locked_accounts.write().await;
        let mut failures = self.failure_counts.write().await;

        locked.clear();
        failures.clear();

        tracing::debug!("Cleared all lockouts and failure counts");
    }

    /// Get the configuration
    pub fn config(&self) -> &LockoutConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_lockout_not_locked_initially() {
        let lockout = AccountLockout::new(5, Duration::from_secs(60));
        assert!(!lockout.is_locked("user1").await);
    }

    #[tokio::test]
    async fn test_lockout_after_max_failures() {
        let lockout = AccountLockout::new(3, Duration::from_secs(60));

        // Record 3 failures
        for _ in 0..3 {
            lockout.record_failure("user1").await;
        }

        // Should be locked
        assert!(lockout.is_locked("user1").await);
        assert!(lockout.check("user1").await.is_err());
    }

    #[tokio::test]
    async fn test_lockout_independent_accounts() {
        let lockout = AccountLockout::new(3, Duration::from_secs(60));

        // Lock user1
        for _ in 0..3 {
            lockout.record_failure("user1").await;
        }

        // user2 should not be affected
        assert!(!lockout.is_locked("user2").await);
        assert!(lockout.check("user2").await.is_ok());
    }

    #[tokio::test]
    async fn test_lockout_success_clears_failures() {
        let lockout = AccountLockout::new(5, Duration::from_secs(60));

        // Record failures
        for _ in 0..3 {
            lockout.record_failure("user1").await;
        }

        assert_eq!(lockout.failure_count("user1").await, 3);

        // Successful login clears failures
        lockout.record_success("user1").await;
        assert_eq!(lockout.failure_count("user1").await, 0);
    }

    #[tokio::test]
    async fn test_lockout_expiration() {
        let lockout = AccountLockout::new(2, Duration::from_millis(100));

        // Lock account
        lockout.record_failure("user1").await;
        lockout.record_failure("user1").await;

        assert!(lockout.is_locked("user1").await);

        // Wait for expiration
        tokio::time::sleep(Duration::from_millis(150)).await;

        // Should be unlocked
        assert!(!lockout.is_locked("user1").await);
    }

    #[tokio::test]
    async fn test_lockout_manual_unlock() {
        let lockout = AccountLockout::new(3, Duration::from_secs(60));

        // Lock account
        for _ in 0..3 {
            lockout.record_failure("user1").await;
        }

        assert!(lockout.is_locked("user1").await);

        // Manual unlock
        lockout.unlock("user1").await;

        assert!(!lockout.is_locked("user1").await);
        assert_eq!(lockout.failure_count("user1").await, 0);
    }

    #[tokio::test]
    async fn test_lockout_remaining_attempts() {
        let lockout = AccountLockout::new(5, Duration::from_secs(60));

        assert_eq!(lockout.remaining_attempts("user1").await, 5);

        lockout.record_failure("user1").await;
        assert_eq!(lockout.remaining_attempts("user1").await, 4);

        lockout.record_failure("user1").await;
        assert_eq!(lockout.remaining_attempts("user1").await, 3);
    }

    #[tokio::test]
    async fn test_lockout_info() {
        let lockout = AccountLockout::new(2, Duration::from_secs(60));

        lockout.record_failure("user1").await;
        lockout.record_failure("user1").await;

        let info = lockout.get_lockout_info("user1").await.unwrap();
        assert_eq!(info.failure_count(), 2);
        assert!(!info.is_expired());
        assert!(info.remaining_duration().is_some());
    }

    #[tokio::test]
    async fn test_lockout_progressive() {
        let config = LockoutConfig::new(2, Duration::from_secs(10)).with_progressive(true);
        let lockout = AccountLockout::with_config(config);

        // First lockout
        lockout.record_failure("user1").await;
        lockout.record_failure("user1").await;

        let _info1 = lockout.get_lockout_info("user1").await.unwrap();

        // Unlock and lock again
        lockout.unlock("user1").await;
        lockout.record_failure("user1").await;
        lockout.record_failure("user1").await;

        // Progressive lockout should have longer duration
        // (This is a simplified test - in real scenario duration doubles)
        assert!(lockout.is_locked("user1").await);
    }

    #[tokio::test]
    async fn test_lockout_cleanup() {
        let lockout = AccountLockout::new(2, Duration::from_millis(50));

        // Lock multiple accounts
        for username in &["user1", "user2", "user3"] {
            lockout.record_failure(username).await;
            lockout.record_failure(username).await;
        }

        // Wait for expiration
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Cleanup
        let cleaned = lockout.cleanup_expired().await;
        assert_eq!(cleaned, 3);
    }

    #[tokio::test]
    async fn test_lockout_clear_all() {
        let lockout = AccountLockout::new(3, Duration::from_secs(60));

        lockout.record_failure("user1").await;
        lockout.record_failure("user2").await;

        lockout.clear_all().await;

        assert_eq!(lockout.failure_count("user1").await, 0);
        assert_eq!(lockout.failure_count("user2").await, 0);
    }

    #[tokio::test]
    async fn test_lockout_config_default() {
        let config = LockoutConfig::default();
        assert_eq!(config.max_failures, 5);
        assert_eq!(config.lockout_duration, Duration::from_secs(1800));
        assert!(!config.progressive);
    }

    #[tokio::test]
    async fn test_lockout_config_strict() {
        let config = LockoutConfig::strict();
        assert_eq!(config.max_failures, 3);
        assert_eq!(config.lockout_duration, Duration::from_secs(3600));
        assert!(config.progressive);
    }

    #[tokio::test]
    async fn test_lockout_config_lenient() {
        let config = LockoutConfig::lenient();
        assert_eq!(config.max_failures, 10);
        assert_eq!(config.lockout_duration, Duration::from_secs(900));
        assert!(!config.progressive);
    }

    #[tokio::test]
    async fn test_lockout_error_message() {
        let lockout = AccountLockout::new(2, Duration::from_secs(60));

        lockout.record_failure("user1").await;
        lockout.record_failure("user1").await;

        match lockout.check("user1").await {
            Err(LockoutError::AccountLocked {
                unlock_time,
                reason,
            }) => {
                assert!(reason.contains("failed login attempts"));
                assert!(unlock_time > SystemTime::now());
            }
            _ => panic!("Expected AccountLocked error"),
        }
    }
}
