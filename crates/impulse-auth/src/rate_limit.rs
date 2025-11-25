//! Rate limiting for authentication attempts
//!
//! This module provides rate limiting functionality to prevent brute-force attacks
//! by tracking login attempts and enforcing configurable limits.
//!
//! # Examples
//!
//! ```
//! use impulse_auth::rate_limit::RateLimiter;
//! use std::time::Duration;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Allow 5 attempts per 15 minutes
//! let limiter = RateLimiter::new(5, Duration::from_secs(900));
//!
//! // Check if user is rate-limited
//! if limiter.check("username").await.is_ok() {
//!     // Attempt allowed, record it
//!     limiter.record_attempt("username").await;
//!
//!     // After successful login, clear attempts
//!     limiter.record_success("username").await;
//! }
//! # Ok(())
//! # }
//! ```

use std::collections::HashMap;
use std::time::{Duration, Instant};
use thiserror::Error;
use tokio::sync::RwLock;

/// Rate limiting errors
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum RateLimitError {
    /// Rate limit exceeded
    #[error("Rate limit exceeded. Try again in {retry_after_secs} seconds")]
    LimitExceeded {
        /// Seconds until retry is allowed
        retry_after_secs: u64,
    },
}

/// Rate limiter configuration
#[derive(Debug, Clone)]
pub struct RateLimiterConfig {
    /// Maximum number of attempts allowed within the window
    pub max_attempts: usize,
    /// Time window for rate limiting
    pub window: Duration,
}

impl Default for RateLimiterConfig {
    fn default() -> Self {
        Self {
            max_attempts: 5,
            window: Duration::from_secs(900), // 15 minutes
        }
    }
}

impl RateLimiterConfig {
    /// Create a new rate limiter configuration
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_auth::rate_limit::RateLimiterConfig;
    /// use std::time::Duration;
    ///
    /// let config = RateLimiterConfig::new(5, Duration::from_secs(900));
    /// ```
    #[must_use]
    pub fn new(max_attempts: usize, window: Duration) -> Self {
        Self {
            max_attempts,
            window,
        }
    }

    /// Create a strict configuration (3 attempts per 30 minutes)
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_auth::rate_limit::RateLimiterConfig;
    ///
    /// let config = RateLimiterConfig::strict();
    /// assert_eq!(config.max_attempts, 3);
    /// ```
    #[must_use]
    pub fn strict() -> Self {
        Self {
            max_attempts: 3,
            window: Duration::from_secs(1800), // 30 minutes
        }
    }

    /// Create a lenient configuration (10 attempts per 5 minutes)
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_auth::rate_limit::RateLimiterConfig;
    ///
    /// let config = RateLimiterConfig::lenient();
    /// assert_eq!(config.max_attempts, 10);
    /// ```
    #[must_use]
    pub fn lenient() -> Self {
        Self {
            max_attempts: 10,
            window: Duration::from_secs(300), // 5 minutes
        }
    }
}

/// Rate limiter for tracking and limiting authentication attempts
///
/// Uses a sliding window approach to track attempts over time.
/// Thread-safe using RwLock for concurrent access.
#[derive(Clone)]
pub struct RateLimiter {
    /// Attempts by key (username or IP)
    attempts: std::sync::Arc<RwLock<HashMap<String, Vec<Instant>>>>,
    /// Configuration
    config: RateLimiterConfig,
}

impl RateLimiter {
    /// Create a new rate limiter with default configuration
    ///
    /// Default: 5 attempts per 15 minutes
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_auth::rate_limit::RateLimiter;
    ///
    /// let limiter = RateLimiter::new_default();
    /// ```
    #[must_use]
    pub fn new_default() -> Self {
        Self::with_config(RateLimiterConfig::default())
    }

    /// Create a new rate limiter with custom parameters
    ///
    /// # Arguments
    ///
    /// * `max_attempts` - Maximum number of attempts within the window
    /// * `window` - Time window for rate limiting
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_auth::rate_limit::RateLimiter;
    /// use std::time::Duration;
    ///
    /// let limiter = RateLimiter::new(5, Duration::from_secs(900));
    /// ```
    #[must_use]
    pub fn new(max_attempts: usize, window: Duration) -> Self {
        Self::with_config(RateLimiterConfig::new(max_attempts, window))
    }

    /// Create a new rate limiter with a configuration
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_auth::rate_limit::{RateLimiter, RateLimiterConfig};
    ///
    /// let config = RateLimiterConfig::strict();
    /// let limiter = RateLimiter::with_config(config);
    /// ```
    #[must_use]
    pub fn with_config(config: RateLimiterConfig) -> Self {
        Self {
            attempts: std::sync::Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Check if an attempt is allowed for the given key
    ///
    /// # Arguments
    ///
    /// * `key` - Identifier for the rate limit (username, IP, etc.)
    ///
    /// # Errors
    ///
    /// Returns `RateLimitError::LimitExceeded` if the rate limit has been exceeded
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_auth::rate_limit::RateLimiter;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let limiter = RateLimiter::new_default();
    ///
    /// if limiter.check("username").await.is_ok() {
    ///     // Attempt allowed
    ///     limiter.record_attempt("username").await;
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn check(&self, key: &str) -> Result<(), RateLimitError> {
        let attempts = self.attempts.read().await;
        let now = Instant::now();

        if let Some(attempt_times) = attempts.get(key) {
            // Count attempts within the window
            let recent_attempts = attempt_times
                .iter()
                .filter(|t| now.duration_since(**t) < self.config.window)
                .count();

            if recent_attempts >= self.config.max_attempts {
                // Calculate retry time
                let oldest_in_window = attempt_times
                    .iter()
                    .filter(|t| now.duration_since(**t) < self.config.window)
                    .min()
                    .copied()
                    .unwrap_or(now);

                let retry_after = self
                    .config
                    .window
                    .saturating_sub(now.duration_since(oldest_in_window));

                tracing::warn!(
                    key = %key,
                    recent_attempts = recent_attempts,
                    max_attempts = self.config.max_attempts,
                    retry_after_secs = retry_after.as_secs(),
                    "Rate limit exceeded"
                );

                return Err(RateLimitError::LimitExceeded {
                    retry_after_secs: retry_after.as_secs(),
                });
            }
        }

        Ok(())
    }

    /// Record a failed attempt for the given key
    ///
    /// # Arguments
    ///
    /// * `key` - Identifier for the rate limit (username, IP, etc.)
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_auth::rate_limit::RateLimiter;
    ///
    /// # async fn example() {
    /// let limiter = RateLimiter::new_default();
    ///
    /// // After a failed login
    /// limiter.record_attempt("username").await;
    /// # }
    /// ```
    pub async fn record_attempt(&self, key: &str) {
        let mut attempts = self.attempts.write().await;
        let now = Instant::now();

        let entry = attempts.entry(key.to_string()).or_insert_with(Vec::new);

        // Clean up old attempts
        entry.retain(|t| now.duration_since(*t) < self.config.window);

        // Add new attempt
        entry.push(now);

        tracing::debug!(
            key = %key,
            attempt_count = entry.len(),
            "Recorded failed attempt"
        );
    }

    /// Clear attempts for the given key (called after successful login)
    ///
    /// # Arguments
    ///
    /// * `key` - Identifier for the rate limit (username, IP, etc.)
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_auth::rate_limit::RateLimiter;
    ///
    /// # async fn example() {
    /// let limiter = RateLimiter::new_default();
    ///
    /// // After successful login
    /// limiter.record_success("username").await;
    /// # }
    /// ```
    pub async fn record_success(&self, key: &str) {
        let mut attempts = self.attempts.write().await;
        if attempts.remove(key).is_some() {
            tracing::debug!(
                key = %key,
                "Cleared attempts after successful login"
            );
        }
    }

    /// Get the current attempt count for a key
    ///
    /// # Arguments
    ///
    /// * `key` - Identifier for the rate limit (username, IP, etc.)
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_auth::rate_limit::RateLimiter;
    ///
    /// # async fn example() {
    /// let limiter = RateLimiter::new_default();
    ///
    /// let count = limiter.attempt_count("username").await;
    /// println!("Failed attempts: {}", count);
    /// # }
    /// ```
    pub async fn attempt_count(&self, key: &str) -> usize {
        let attempts = self.attempts.read().await;
        let now = Instant::now();

        attempts
            .get(key)
            .map(|times| {
                times
                    .iter()
                    .filter(|t| now.duration_since(**t) < self.config.window)
                    .count()
            })
            .unwrap_or(0)
    }

    /// Get remaining attempts before rate limit
    ///
    /// # Arguments
    ///
    /// * `key` - Identifier for the rate limit (username, IP, etc.)
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_auth::rate_limit::RateLimiter;
    ///
    /// # async fn example() {
    /// let limiter = RateLimiter::new_default();
    ///
    /// let remaining = limiter.remaining_attempts("username").await;
    /// println!("Remaining attempts: {}", remaining);
    /// # }
    /// ```
    pub async fn remaining_attempts(&self, key: &str) -> usize {
        let current = self.attempt_count(key).await;
        self.config.max_attempts.saturating_sub(current)
    }

    /// Clear all attempts (useful for testing)
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_auth::rate_limit::RateLimiter;
    ///
    /// # async fn example() {
    /// let limiter = RateLimiter::new_default();
    ///
    /// limiter.clear_all().await;
    /// # }
    /// ```
    pub async fn clear_all(&self) {
        let mut attempts = self.attempts.write().await;
        attempts.clear();
        tracing::debug!("Cleared all rate limit attempts");
    }

    /// Cleanup expired attempts for all keys
    ///
    /// Removes attempts outside the rate limit window to free memory.
    /// Returns the number of keys that were cleaned up (had no remaining attempts).
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_auth::rate_limit::RateLimiter;
    ///
    /// # async fn example() {
    /// let limiter = RateLimiter::new_default();
    ///
    /// let cleaned = limiter.cleanup_expired().await;
    /// println!("Cleaned up {} keys", cleaned);
    /// # }
    /// ```
    pub async fn cleanup_expired(&self) -> usize {
        let mut attempts = self.attempts.write().await;
        let now = Instant::now();
        let mut cleaned = 0;

        attempts.retain(|_, times| {
            times.retain(|t| now.duration_since(*t) < self.config.window);
            if times.is_empty() {
                cleaned += 1;
                false
            } else {
                true
            }
        });

        if cleaned > 0 {
            tracing::debug!(
                cleaned_keys = cleaned,
                "Cleaned up expired rate limit entries"
            );
        }

        cleaned
    }

    /// Get the configuration
    pub fn config(&self) -> &RateLimiterConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiter_allows_initial_attempts() {
        let limiter = RateLimiter::new(5, Duration::from_secs(60));

        // First 5 attempts should be allowed
        for i in 1..=5 {
            assert!(limiter.check("user1").await.is_ok());
            limiter.record_attempt("user1").await;
            assert_eq!(limiter.attempt_count("user1").await, i);
        }
    }

    #[tokio::test]
    async fn test_rate_limiter_blocks_excess_attempts() {
        let limiter = RateLimiter::new(3, Duration::from_secs(60));

        // Make 3 attempts
        for _ in 0..3 {
            limiter.record_attempt("user1").await;
        }

        // 4th attempt should be blocked
        let result = limiter.check("user1").await;
        assert!(matches!(result, Err(RateLimitError::LimitExceeded { .. })));
    }

    #[tokio::test]
    async fn test_rate_limiter_independent_keys() {
        let limiter = RateLimiter::new(3, Duration::from_secs(60));

        // Make 3 attempts for user1
        for _ in 0..3 {
            limiter.record_attempt("user1").await;
        }

        // user2 should not be affected
        assert!(limiter.check("user2").await.is_ok());
        assert_eq!(limiter.attempt_count("user2").await, 0);
    }

    #[tokio::test]
    async fn test_rate_limiter_success_clears_attempts() {
        let limiter = RateLimiter::new(5, Duration::from_secs(60));

        // Make attempts
        for _ in 0..3 {
            limiter.record_attempt("user1").await;
        }

        assert_eq!(limiter.attempt_count("user1").await, 3);

        // Successful login clears attempts
        limiter.record_success("user1").await;
        assert_eq!(limiter.attempt_count("user1").await, 0);
    }

    #[tokio::test]
    async fn test_rate_limiter_window_expiration() {
        let limiter = RateLimiter::new(3, Duration::from_millis(100));

        // Make 3 attempts
        for _ in 0..3 {
            limiter.record_attempt("user1").await;
        }

        // Should be blocked
        assert!(limiter.check("user1").await.is_err());

        // Wait for window to expire
        tokio::time::sleep(Duration::from_millis(150)).await;

        // Should be allowed again
        assert!(limiter.check("user1").await.is_ok());
        assert_eq!(limiter.attempt_count("user1").await, 0);
    }

    #[tokio::test]
    async fn test_rate_limiter_remaining_attempts() {
        let limiter = RateLimiter::new(5, Duration::from_secs(60));

        assert_eq!(limiter.remaining_attempts("user1").await, 5);

        limiter.record_attempt("user1").await;
        assert_eq!(limiter.remaining_attempts("user1").await, 4);

        limiter.record_attempt("user1").await;
        assert_eq!(limiter.remaining_attempts("user1").await, 3);
    }

    #[tokio::test]
    async fn test_rate_limiter_cleanup() {
        let limiter = RateLimiter::new(5, Duration::from_millis(50));

        // Make attempts for multiple users
        limiter.record_attempt("user1").await;
        limiter.record_attempt("user2").await;
        limiter.record_attempt("user3").await;

        // Wait for expiration
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Cleanup should remove all entries
        let cleaned = limiter.cleanup_expired().await;
        assert_eq!(cleaned, 3);
    }

    #[tokio::test]
    async fn test_rate_limiter_clear_all() {
        let limiter = RateLimiter::new(5, Duration::from_secs(60));

        limiter.record_attempt("user1").await;
        limiter.record_attempt("user2").await;

        assert_eq!(limiter.attempt_count("user1").await, 1);
        assert_eq!(limiter.attempt_count("user2").await, 1);

        limiter.clear_all().await;

        assert_eq!(limiter.attempt_count("user1").await, 0);
        assert_eq!(limiter.attempt_count("user2").await, 0);
    }

    #[tokio::test]
    async fn test_rate_limiter_config_default() {
        let config = RateLimiterConfig::default();
        assert_eq!(config.max_attempts, 5);
        assert_eq!(config.window, Duration::from_secs(900));
    }

    #[tokio::test]
    async fn test_rate_limiter_config_strict() {
        let config = RateLimiterConfig::strict();
        assert_eq!(config.max_attempts, 3);
        assert_eq!(config.window, Duration::from_secs(1800));
    }

    #[tokio::test]
    async fn test_rate_limiter_config_lenient() {
        let config = RateLimiterConfig::lenient();
        assert_eq!(config.max_attempts, 10);
        assert_eq!(config.window, Duration::from_secs(300));
    }

    #[tokio::test]
    async fn test_rate_limiter_retry_after() {
        let limiter = RateLimiter::new(2, Duration::from_secs(10));

        // Make 2 attempts
        for _ in 0..2 {
            limiter.record_attempt("user1").await;
        }

        // Check rate limit error includes retry_after
        match limiter.check("user1").await {
            Err(RateLimitError::LimitExceeded { retry_after_secs }) => {
                assert!(retry_after_secs <= 10);
            }
            _ => panic!("Expected LimitExceeded error"),
        }
    }
}
