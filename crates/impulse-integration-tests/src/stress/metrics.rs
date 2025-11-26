//! Load test metrics collection

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

/// Metrics collected during load testing
///
/// Uses atomic counters for lock-free concurrent updates
/// from multiple simulated users.
#[derive(Default)]
pub struct LoadMetrics {
    /// Total connection attempts made
    pub total_connections: AtomicU64,
    /// Successful login count
    pub successful_logins: AtomicU64,
    /// Failed login count
    pub failed_logins: AtomicU64,
    /// Messages posted during test
    pub messages_posted: AtomicU64,
    /// Files uploaded during test
    pub files_uploaded: AtomicU64,
    /// Files downloaded during test
    pub files_downloaded: AtomicU64,
    /// Door games launched
    pub doors_launched: AtomicU64,
    /// Total latency in milliseconds (sum)
    pub total_latency_ms: AtomicU64,
    /// Errors encountered
    pub errors: AtomicU64,
    /// Operations completed successfully
    pub operations_completed: AtomicU64,
}

impl LoadMetrics {
    /// Create new load metrics instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a shared Arc-wrapped instance
    pub fn new_shared() -> Arc<Self> {
        Arc::new(Self::default())
    }

    /// Increment total connections counter
    pub fn increment_connections(&self) {
        self.total_connections.fetch_add(1, Ordering::Relaxed);
    }

    /// Record successful login
    pub fn record_successful_login(&self) {
        self.successful_logins.fetch_add(1, Ordering::Relaxed);
    }

    /// Record failed login
    pub fn record_failed_login(&self) {
        self.failed_logins.fetch_add(1, Ordering::Relaxed);
    }

    /// Record message posted
    pub fn record_message_posted(&self) {
        self.messages_posted.fetch_add(1, Ordering::Relaxed);
    }

    /// Record file upload
    pub fn record_file_uploaded(&self) {
        self.files_uploaded.fetch_add(1, Ordering::Relaxed);
    }

    /// Record file download
    pub fn record_file_downloaded(&self) {
        self.files_downloaded.fetch_add(1, Ordering::Relaxed);
    }

    /// Record door launch
    pub fn record_door_launched(&self) {
        self.doors_launched.fetch_add(1, Ordering::Relaxed);
    }

    /// Add latency measurement
    pub fn add_latency(&self, latency_ms: u64) {
        self.total_latency_ms
            .fetch_add(latency_ms, Ordering::Relaxed);
    }

    /// Record error
    pub fn record_error(&self) {
        self.errors.fetch_add(1, Ordering::Relaxed);
    }

    /// Record operation completion
    pub fn record_operation_completed(&self) {
        self.operations_completed.fetch_add(1, Ordering::Relaxed);
    }

    /// Get total connections
    pub fn get_total_connections(&self) -> u64 {
        self.total_connections.load(Ordering::Relaxed)
    }

    /// Get successful login count
    pub fn get_successful_logins(&self) -> u64 {
        self.successful_logins.load(Ordering::Relaxed)
    }

    /// Get failed login count
    pub fn get_failed_logins(&self) -> u64 {
        self.failed_logins.load(Ordering::Relaxed)
    }

    /// Calculate average latency in milliseconds
    pub fn average_latency_ms(&self) -> f64 {
        let total = self.total_latency_ms.load(Ordering::Relaxed);
        let count = self.operations_completed.load(Ordering::Relaxed).max(1);
        total as f64 / count as f64
    }

    /// Calculate login success rate (0.0 to 1.0)
    pub fn login_success_rate(&self) -> f64 {
        let success = self.successful_logins.load(Ordering::Relaxed);
        let total = success + self.failed_logins.load(Ordering::Relaxed);
        if total == 0 {
            0.0
        } else {
            success as f64 / total as f64
        }
    }

    /// Get error count
    pub fn get_errors(&self) -> u64 {
        self.errors.load(Ordering::Relaxed)
    }

    /// Print comprehensive metrics summary
    pub fn print_summary(&self) {
        let total = self.get_total_connections();
        let success = self.get_successful_logins();
        let failed = self.get_failed_logins();
        let messages = self.messages_posted.load(Ordering::Relaxed);
        let uploads = self.files_uploaded.load(Ordering::Relaxed);
        let downloads = self.files_downloaded.load(Ordering::Relaxed);
        let doors = self.doors_launched.load(Ordering::Relaxed);
        let errors = self.get_errors();

        println!("\n=== Load Test Results ===");
        println!("Total Connections:    {}", total);
        println!(
            "Successful Logins:    {} ({:.2}%)",
            success,
            self.login_success_rate() * 100.0
        );
        println!("Failed Logins:        {}", failed);
        println!("Messages Posted:      {}", messages);
        println!("Files Uploaded:       {}", uploads);
        println!("Files Downloaded:     {}", downloads);
        println!("Doors Launched:       {}", doors);
        println!("Average Latency:      {:.2} ms", self.average_latency_ms());
        println!("Errors:               {}", errors);
        println!("=========================\n");
    }

    /// Reset all metrics to zero
    pub fn reset(&self) {
        self.total_connections.store(0, Ordering::Relaxed);
        self.successful_logins.store(0, Ordering::Relaxed);
        self.failed_logins.store(0, Ordering::Relaxed);
        self.messages_posted.store(0, Ordering::Relaxed);
        self.files_uploaded.store(0, Ordering::Relaxed);
        self.files_downloaded.store(0, Ordering::Relaxed);
        self.doors_launched.store(0, Ordering::Relaxed);
        self.total_latency_ms.store(0, Ordering::Relaxed);
        self.errors.store(0, Ordering::Relaxed);
        self.operations_completed.store(0, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_creation() {
        let metrics = LoadMetrics::new();
        assert_eq!(metrics.get_total_connections(), 0);
        assert_eq!(metrics.get_errors(), 0);
    }

    #[test]
    fn test_increment_connections() {
        let metrics = LoadMetrics::new();
        metrics.increment_connections();
        metrics.increment_connections();
        assert_eq!(metrics.get_total_connections(), 2);
    }

    #[test]
    fn test_login_tracking() {
        let metrics = LoadMetrics::new();
        metrics.record_successful_login();
        metrics.record_successful_login();
        metrics.record_failed_login();

        assert_eq!(metrics.get_successful_logins(), 2);
        assert_eq!(metrics.get_failed_logins(), 1);
        assert_eq!(metrics.login_success_rate(), 2.0 / 3.0);
    }

    #[test]
    fn test_latency_calculation() {
        let metrics = LoadMetrics::new();
        metrics.add_latency(100);
        metrics.record_operation_completed();
        metrics.add_latency(200);
        metrics.record_operation_completed();
        metrics.add_latency(150);
        metrics.record_operation_completed();

        assert_eq!(metrics.average_latency_ms(), 150.0);
    }

    #[test]
    fn test_metrics_reset() {
        let metrics = LoadMetrics::new();
        metrics.increment_connections();
        metrics.record_successful_login();
        metrics.record_error();

        metrics.reset();

        assert_eq!(metrics.get_total_connections(), 0);
        assert_eq!(metrics.get_successful_logins(), 0);
        assert_eq!(metrics.get_errors(), 0);
    }

    #[test]
    fn test_print_summary() {
        let metrics = LoadMetrics::new();
        metrics.increment_connections();
        metrics.record_successful_login();
        metrics.record_message_posted();
        metrics.add_latency(50);
        metrics.record_operation_completed();

        // Should not panic
        metrics.print_summary();
    }

    #[test]
    fn test_concurrent_updates() {
        use std::thread;

        let metrics = Arc::new(LoadMetrics::new());
        let mut handles = vec![];

        // Spawn 10 threads, each incrementing 100 times
        for _ in 0..10 {
            let metrics = Arc::clone(&metrics);
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    metrics.increment_connections();
                    metrics.record_successful_login();
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        // Should have 1000 connections (10 threads × 100 increments)
        assert_eq!(metrics.get_total_connections(), 1000);
        assert_eq!(metrics.get_successful_logins(), 1000);
    }
}
