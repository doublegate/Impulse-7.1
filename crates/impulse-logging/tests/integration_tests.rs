//! Integration tests for impulse-logging crate
//!
//! Tests the complete logging system with real file I/O and rotation.

use impulse_logging::{
    ArchivalConfig, ArchiveManager, AuditEvent, AuditEventType, AuditLogger, ErrorContext,
    ErrorReporter, ErrorSeverity, LogFormat, LogLevel, LogOutput, LoggerBuilder, RotationManager,
    RotationPolicy, init_console_logging, init_file_logging,
};
use serial_test::serial;
use tempfile::TempDir;
use tokio::time::{Duration, sleep};

#[test]
#[serial]
fn test_console_logging_initialization() {
    // Should succeed or already be initialized
    let result = init_console_logging();
    // Accept success or "already initialized" error
    if let Err(e) = &result {
        let error_msg = format!("{:?}", e);
        assert!(
            error_msg.contains("global default trace dispatcher has already been set"),
            "Unexpected error: {:?}",
            e
        );
    }
}

#[tokio::test]
#[serial]
async fn test_file_logging_initialization() {
    let temp_dir = TempDir::new().unwrap();
    let log_path = temp_dir.path().join("test.log");

    let result = init_file_logging(log_path.to_str().unwrap());
    // Accept success or "already initialized" error
    if let Err(e) = &result {
        let error_msg = format!("{:?}", e);
        assert!(
            error_msg.contains("global default trace dispatcher has already been set"),
            "Unexpected error: {:?}",
            e
        );
    } else {
        // Only check file existence if initialization succeeded
        // Log something to ensure file is created
        tracing::info!("Test log message");
        // Allow time for the log to be flushed to disk
        sleep(Duration::from_millis(100)).await;

        // With daily rotation, the file might have a date suffix
        // Check if any log files were created in the directory
        let log_files: Vec<_> = std::fs::read_dir(temp_dir.path())
            .unwrap()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_name().to_str().unwrap_or("").starts_with("test"))
            .collect();

        assert!(
            !log_files.is_empty(),
            "No log files created in {:?}. Expected file at {:?}",
            temp_dir.path(),
            log_path
        );
    }
}

#[test]
#[serial]
fn test_logger_builder_stdout() {
    let result = LoggerBuilder::new()
        .with_level(LogLevel::Debug)
        .with_format(LogFormat::Json)
        .with_output(LogOutput::Stdout)
        .build();

    // Accept success or "already initialized" error
    if let Err(e) = &result {
        let error_msg = format!("{:?}", e);
        assert!(
            error_msg.contains("global default trace dispatcher has already been set"),
            "Unexpected error: {:?}",
            e
        );
    }
}

#[test]
#[serial]
fn test_logger_builder_file_with_rotation() {
    let temp_dir = TempDir::new().unwrap();
    let log_path = temp_dir.path().join("rotating.log");

    let result = LoggerBuilder::new()
        .with_level(LogLevel::Info)
        .with_output(LogOutput::File(log_path.clone()))
        .with_rotation(RotationPolicy::Daily)
        .with_max_files(10)
        .build();

    // Accept success or "already initialized" error
    if let Err(e) = &result {
        let error_msg = format!("{:?}", e);
        assert!(
            error_msg.contains("global default trace dispatcher has already been set"),
            "Unexpected error: {:?}",
            e
        );
    }
}

#[tokio::test]
async fn test_rotation_manager_cleanup() {
    let temp_dir = TempDir::new().unwrap();
    let log_dir = temp_dir.path();

    // Create 15 log files
    for i in 0..15 {
        let filename = format!("app.log.2025-01-{:02}", i + 1);
        tokio::fs::write(log_dir.join(filename), format!("log{}", i))
            .await
            .unwrap();
        sleep(Duration::from_millis(10)).await;
    }

    let manager = RotationManager::new(log_dir, 10);
    assert_eq!(manager.count_log_files().unwrap(), 15);

    manager.cleanup_old_files().unwrap();
    assert_eq!(manager.count_log_files().unwrap(), 10);
}

#[tokio::test]
async fn test_archive_manager_compression() {
    let temp_dir = TempDir::new().unwrap();
    let source = temp_dir.path().join("test.log");
    let content = b"This is a test log file with repeated content. ".repeat(100);

    tokio::fs::write(&source, &content).await.unwrap();

    let config = ArchivalConfig::default();
    let manager = ArchiveManager::new(config);

    let compressed = manager.compress_file(&source, None).await.unwrap();

    assert!(compressed.exists());
    assert!(compressed.to_str().unwrap().ends_with(".gz"));

    // Compressed file should exist and have content
    let compressed_size = tokio::fs::metadata(&compressed).await.unwrap().len();
    assert!(compressed_size > 0);
    assert!(compressed_size < content.len() as u64); // Should be smaller
}

#[tokio::test]
async fn test_archive_manager_disabled() {
    let temp_dir = TempDir::new().unwrap();
    let config = ArchivalConfig::disabled();
    let manager = ArchiveManager::new(config);

    let count = manager.archive_old_logs(temp_dir.path()).await.unwrap();
    assert_eq!(count, 0);
}

#[tokio::test]
async fn test_archive_manager_maintenance() {
    let temp_dir = TempDir::new().unwrap();
    let log_dir = temp_dir.path().join("logs");
    let archive_dir = temp_dir.path().join("archive");

    tokio::fs::create_dir_all(&log_dir).await.unwrap();
    tokio::fs::create_dir_all(&archive_dir).await.unwrap();

    // Create an old log file (simulate by setting modification time)
    let old_log = log_dir.join("old.log");
    tokio::fs::write(&old_log, b"old log content")
        .await
        .unwrap();

    let config = ArchivalConfig::new(archive_dir, 90, 6);
    let manager = ArchiveManager::new(config);

    // Should run without error
    let result = manager.run_maintenance(&log_dir).await;
    assert!(result.is_ok());
}

#[test]
fn test_audit_logger_login() {
    let logger = AuditLogger::new();
    logger.log_login(42, "testuser", Some("192.168.1.100"), true);
    // Should not panic
}

#[test]
fn test_audit_logger_logout() {
    let logger = AuditLogger::new();
    logger.log_logout(42, "testuser", Some("session123"));
    // Should not panic
}

#[test]
fn test_audit_logger_file_upload() {
    let logger = AuditLogger::new();
    logger.log_file_upload(42, "testuser", "document.zip", 1024 * 1024);
    // Should not panic
}

#[test]
fn test_audit_logger_config_change() {
    let logger = AuditLogger::new();
    logger.log_config_change(1, "admin", "max_users", "100", "200");
    // Should not panic
}

#[test]
fn test_audit_logger_security_violation() {
    let logger = AuditLogger::new();
    logger.log_security_violation(
        Some(42),
        Some("baduser"),
        Some("1.2.3.4"),
        "Too many failed login attempts",
    );
    // Should not panic
}

#[test]
fn test_audit_event_serialization() {
    let event = AuditEvent::new(
        AuditEventType::Login,
        "User logged in".to_string(),
        "success".to_string(),
    )
    .with_user_id(42)
    .with_username("testuser".to_string())
    .with_ip_address("192.168.1.100".to_string());

    let json = event.to_json().unwrap();
    assert!(json.contains("login"));
    assert!(json.contains("42"));
    assert!(json.contains("testuser"));

    // Should be able to deserialize back
    let deserialized: AuditEvent = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.event_type, AuditEventType::Login);
    assert_eq!(deserialized.user_id, Some(42));
}

#[test]
fn test_error_reporter_context() {
    let ctx = ErrorContext::new("Test error")
        .with_code("ERR_001")
        .with_details("Additional details")
        .with_severity(ErrorSeverity::Warning);

    ErrorReporter::report(&ctx);
    // Should not panic
}

#[test]
fn test_error_reporter_io_error() {
    let err = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
    ErrorReporter::report_error(&err);
    // Should not panic
}

#[test]
fn test_error_reporter_user_friendly() {
    let err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let message = ErrorReporter::user_friendly_message(&err);
    assert!(message.contains("not found"));

    let err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "permission denied");
    let message = ErrorReporter::user_friendly_message(&err);
    assert!(message.contains("Permission"));

    let err = std::io::Error::new(std::io::ErrorKind::TimedOut, "connection timeout");
    let message = ErrorReporter::user_friendly_message(&err);
    assert!(message.contains("timed out"));
}

#[tokio::test]
async fn test_full_logging_workflow() {
    let temp_dir = TempDir::new().unwrap();
    let log_file = temp_dir.path().join("workflow.log");

    // Initialize logging
    let result = LoggerBuilder::new()
        .with_level(LogLevel::Debug)
        .with_output(LogOutput::File(log_file.clone()))
        .with_format(LogFormat::Json)
        .build();

    // Accept success or "already initialized" error
    let can_test_logging = result.is_ok();
    if let Err(e) = &result {
        let error_msg = format!("{:?}", e);
        assert!(
            error_msg.contains("global default trace dispatcher has already been set"),
            "Unexpected error: {:?}",
            e
        );
    }

    // Only test logging if initialization succeeded
    if can_test_logging {
        // Log various events
        tracing::info!(user_id = 42, "User logged in");
        tracing::warn!(warning = "test", "Warning message");
        tracing::error!(error = "test", "Error message");

        // Create audit logger
        let audit = AuditLogger::new();
        audit.log_login(42, "testuser", Some("192.168.1.100"), true);

        // Report errors
        let ctx = ErrorContext::new("Test error").with_code("TEST_001");
        ErrorReporter::report(&ctx);

        // Log file should exist and have content
        tokio::time::sleep(Duration::from_millis(100)).await; // Allow logs to flush
        assert!(log_file.exists());

        let content = tokio::fs::read_to_string(&log_file).await.unwrap();
        assert!(!content.is_empty());
    }
}
