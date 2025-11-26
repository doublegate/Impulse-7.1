//! Performance benchmarks for critical paths

use std::time::Instant;

/// Benchmark result
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub name: String,
    pub iterations: u64,
    pub total_time_ms: u64,
    pub avg_time_ns: f64,
    pub ops_per_sec: f64,
}

impl BenchmarkResult {
    /// Print benchmark results
    pub fn print(&self) {
        println!("\nBenchmark: {}", self.name);
        println!("  Iterations:    {}", self.iterations);
        println!("  Total time:    {} ms", self.total_time_ms);
        println!("  Average time:  {:.2} ns", self.avg_time_ns);
        println!("  Throughput:    {:.2} ops/sec", self.ops_per_sec);
    }
}

/// Run a simple benchmark
///
/// # Arguments
///
/// * `name` - Benchmark name
/// * `iterations` - Number of iterations to run
/// * `f` - Function to benchmark
pub fn benchmark<F>(name: &str, iterations: u64, mut f: F) -> BenchmarkResult
where
    F: FnMut(),
{
    let start = Instant::now();

    for _ in 0..iterations {
        f();
    }

    let elapsed = start.elapsed();
    let total_time_ms = elapsed.as_millis() as u64;
    let total_time_ns = elapsed.as_nanos() as f64;
    let avg_time_ns = total_time_ns / iterations as f64;
    let ops_per_sec = (iterations as f64 / elapsed.as_secs_f64()).max(0.0);

    BenchmarkResult {
        name: name.to_string(),
        iterations,
        total_time_ms,
        avg_time_ns,
        ops_per_sec,
    }
}

/// Run an async benchmark
pub async fn benchmark_async<F, Fut>(name: &str, iterations: u64, mut f: F) -> BenchmarkResult
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = ()>,
{
    let start = Instant::now();

    for _ in 0..iterations {
        f().await;
    }

    let elapsed = start.elapsed();
    let total_time_ms = elapsed.as_millis() as u64;
    let total_time_ns = elapsed.as_nanos() as f64;
    let avg_time_ns = total_time_ns / iterations as f64;
    let ops_per_sec = (iterations as f64 / elapsed.as_secs_f64()).max(0.0);

    BenchmarkResult {
        name: name.to_string(),
        iterations,
        total_time_ms,
        avg_time_ns,
        ops_per_sec,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_benchmark() {
        let result = benchmark("test_operation", 1000, || {
            // Simulate some work
            let _ = (1..100).sum::<i32>();
        });

        assert_eq!(result.iterations, 1000);
        // Note: total_time_ms may be 0 for very fast operations
        // avg_time_ns will still be accurate
        assert!(result.avg_time_ns > 0.0);
        assert!(result.ops_per_sec > 0.0);

        result.print();
    }

    #[tokio::test]
    async fn test_async_benchmark() {
        let result = benchmark_async("async_operation", 100, || async {
            // Simulate async work
            tokio::time::sleep(tokio::time::Duration::from_micros(10)).await;
        })
        .await;

        assert_eq!(result.iterations, 100);
        assert!(result.total_time_ms > 0);

        result.print();
    }

    #[test]
    fn test_benchmark_result() {
        let result = BenchmarkResult {
            name: "test".to_string(),
            iterations: 1000,
            total_time_ms: 100,
            avg_time_ns: 100_000.0,
            ops_per_sec: 10_000.0,
        };

        // Should not panic
        result.print();
    }
}
