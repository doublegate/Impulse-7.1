//! # Impulse Integration Tests
//!
//! Comprehensive integration testing suite for Phase 3 of Impulse-Next_BBS.
//! Tests all advanced features working together including file transfer protocols,
//! door games, themes, administration, and advanced messaging.
//!
//! ## Test Modules
//!
//! - `fixtures`: Test fixtures and factories for creating test environments
//! - `journeys`: End-to-end user journey tests
//! - `security`: Security audit and penetration testing
//! - `stress`: Load testing and performance benchmarks
//! - `cross_crate`: Cross-crate integration testing
//!
//! ## Usage
//!
//! Run all integration tests:
//! ```bash
//! cargo test --package impulse-integration-tests
//! ```
//!
//! Run specific test category:
//! ```bash
//! cargo test --package impulse-integration-tests journeys::
//! ```
//!
//! Run load tests:
//! ```bash
//! cargo test --package impulse-integration-tests stress:: --release -- --ignored
//! ```

// Public modules
pub mod cross_crate;
pub mod fixtures;
pub mod journeys;
pub mod security;
pub mod stress;

// Re-export commonly used types
pub use fixtures::{BbsTestFixture, UserFactory};
pub use stress::{LoadGenerator, LoadMetrics};

/// Test configuration constants
pub mod test_config {
    /// Default BBS port for testing
    pub const TEST_PORT: u16 = 12323;

    /// Maximum concurrent users for load testing
    pub const MAX_CONCURRENT_USERS: usize = 50;

    /// Load test duration in seconds
    pub const LOAD_TEST_DURATION: u64 = 60;

    /// Timeout for individual operations
    pub const OPERATION_TIMEOUT_SECS: u64 = 30;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_constants() {
        assert_eq!(test_config::TEST_PORT, 12323);
        assert_eq!(test_config::MAX_CONCURRENT_USERS, 50);
        assert_eq!(test_config::LOAD_TEST_DURATION, 60);
    }
}
