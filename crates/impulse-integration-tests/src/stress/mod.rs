//! Load and stress testing suite
//!
//! Comprehensive performance testing including:
//! - Concurrent user simulation
//! - Load metrics collection
//! - Performance benchmarks
//! - Endurance testing

mod benchmarks;
mod load_generator;
mod metrics;

pub use benchmarks::*;
pub use load_generator::LoadGenerator;
pub use metrics::LoadMetrics;

use anyhow::Result;

/// Run a basic load test with specified parameters
///
/// # Arguments
///
/// * `num_users` - Number of concurrent users to simulate
/// * `duration_secs` - Test duration in seconds
///
/// # Returns
///
/// LoadMetrics containing test results
pub async fn run_load_test(num_users: usize, duration_secs: u64) -> Result<LoadMetrics> {
    let generator = LoadGenerator::new().await?;
    generator.run_load_test(num_users, duration_secs).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_load_test_creation() {
        let generator = LoadGenerator::new().await.unwrap();
        assert!(generator.fixture().temp_path().exists());
    }

    #[tokio::test]
    #[ignore] // Expensive test, run explicitly
    async fn test_basic_load() {
        let metrics = run_load_test(5, 5).await.unwrap();
        assert!(
            metrics
                .total_connections
                .load(std::sync::atomic::Ordering::Relaxed)
                > 0
        );
    }
}
