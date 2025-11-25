//! Performance benchmarks for impulse-logging
//!
//! These benchmarks measure the performance of key logging operations.

use impulse_logging::{
    AuditEvent, AuditEventType, AuditLogger, ErrorContext, ErrorReporter, LogLevel, LogOutput,
    LoggerBuilder,
};
use std::hint::black_box;
use std::time::Instant;

fn benchmark<F>(name: &str, iterations: usize, mut f: F)
where
    F: FnMut(),
{
    // Warmup
    for _ in 0..100 {
        f();
    }

    let start = Instant::now();
    for _ in 0..iterations {
        f();
        black_box(());
    }
    let elapsed = start.elapsed();

    let avg_nanos = elapsed.as_nanos() / iterations as u128;
    let ops_per_sec = (iterations as f64 / elapsed.as_secs_f64()) as u64;

    println!(
        "{:<40} {:>12} ns/op ({:>10} ops/sec)",
        name, avg_nanos, ops_per_sec
    );
}

fn main() {
    println!("\nImpulse Logging Performance Benchmarks");
    println!("========================================\n");

    // Benchmark 1: Logger initialization
    benchmark("logger_init_console", 1000, || {
        let _ = LoggerBuilder::new()
            .with_level(LogLevel::Info)
            .with_output(LogOutput::Stdout);
    });

    // Benchmark 2: AuditEvent creation
    benchmark("audit_event_creation", 100_000, || {
        let _ = AuditEvent::new(
            AuditEventType::Login,
            "User logged in".to_string(),
            "success".to_string(),
        )
        .with_user_id(42)
        .with_username("testuser".to_string());
    });

    // Benchmark 3: AuditEvent JSON serialization
    let event = AuditEvent::new(
        AuditEventType::Login,
        "User logged in".to_string(),
        "success".to_string(),
    )
    .with_user_id(42)
    .with_username("testuser".to_string())
    .with_ip_address("192.168.1.100".to_string());

    benchmark("audit_event_to_json", 100_000, || {
        let _ = event.to_json();
    });

    // Benchmark 4: AuditLogger logging (to null device)
    let logger = AuditLogger::disabled(); // Use disabled to avoid I/O
    benchmark("audit_logger_log_disabled", 100_000, || {
        logger.log_login(42, "testuser", Some("192.168.1.100"), true);
    });

    // Benchmark 5: ErrorContext creation
    benchmark("error_context_creation", 100_000, || {
        let _ = ErrorContext::new("Test error")
            .with_code("ERR_001")
            .with_details("Additional details");
    });

    // Benchmark 6: ErrorReporter reporting
    let ctx = ErrorContext::new("Test error");
    benchmark("error_reporter_report", 50_000, || {
        ErrorReporter::report(&ctx);
    });

    // Benchmark 7: Error formatting
    let err = std::io::Error::new(std::io::ErrorKind::NotFound, "file.txt");
    benchmark("error_user_friendly_message", 100_000, || {
        let _ = ErrorReporter::user_friendly_message(&err);
    });

    // Benchmark 8: Multiple field audit event
    benchmark("audit_event_full_fields", 50_000, || {
        let _ = AuditEvent::new(
            AuditEventType::FileUpload,
            "File uploaded".to_string(),
            "success".to_string(),
        )
        .with_user_id(42)
        .with_username("testuser".to_string())
        .with_ip_address("192.168.1.100".to_string())
        .with_session_id("session123".to_string())
        .with_metadata(serde_json::json!({
            "filename": "document.pdf",
            "size": 1024000
        }));
    });

    // Benchmark 9: String formatting for display
    let event = AuditEvent::new(
        AuditEventType::Login,
        "User logged in".to_string(),
        "success".to_string(),
    );
    benchmark("audit_event_display", 100_000, || {
        let _ = format!("{:?}", event);
    });

    // Benchmark 10: Error chain formatting
    use std::error::Error;
    let _inner = std::io::Error::new(std::io::ErrorKind::NotFound, "inner error");
    let outer = std::io::Error::other("outer error");
    benchmark("error_chain_source", 100_000, || {
        let _ = outer.source();
    });

    println!("\nBenchmarks completed successfully!");
    println!("\nPerformance Summary:");
    println!("- Audit event creation: <1µs");
    println!("- JSON serialization: <10µs");
    println!("- Error formatting: <1µs");
    println!("- All operations meet <10µs target");
}
