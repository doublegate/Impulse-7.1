//! Serialization round-trip tests
//!
//! Tests to ensure all types can be serialized and deserialized correctly
//! in both JSON and binary (bincode) formats.

use impulse_types::{
    config::{BbsConfig, BbsPaths, Protocol, SecuritySettings, ServerConfig, SystemLimits},
    file::FileEntry,
    message::Message,
    security::SecurityLevel,
    user::User,
    user_stats::UserStats,
};

#[test]
fn test_user_json_roundtrip() {
    let user = User::new("TestUser").expect("Valid username");

    // Serialize to JSON
    let json = serde_json::to_string(&user).expect("Failed to serialize to JSON");

    // Deserialize back
    let deserialized: User = serde_json::from_str(&json).expect("Failed to deserialize from JSON");

    // Verify key fields match
    assert_eq!(user.id(), deserialized.id());
    assert_eq!(user.username(), deserialized.username());
    assert_eq!(user.security_level(), deserialized.security_level());
}

#[test]
fn test_user_bincode_roundtrip() {
    let user = User::new("TestUser").expect("Valid username");

    // Serialize to bincode (bincode 2.0 API)
    let encoded = bincode::serde::encode_to_vec(&user, bincode::config::standard())
        .expect("Failed to serialize to bincode");

    // Deserialize back (bincode 2.0 API returns (T, usize) tuple)
    let decoded: User = bincode::serde::decode_from_slice(&encoded, bincode::config::standard())
        .map(|(v, _)| v)
        .expect("Failed to deserialize from bincode");

    // Verify key fields match
    assert_eq!(user.id(), decoded.id());
    assert_eq!(user.username(), decoded.username());
    assert_eq!(user.security_level(), decoded.security_level());
}

#[test]
fn test_message_json_roundtrip() {
    let message = Message {
        id: 1,
        from: "Alice".to_string(),
        to: "Bob".to_string(),
        subject: "Test Subject".to_string(),
        body: "This is a test message body.".to_string(),
        date: chrono::Utc::now(),
        area_id: 1,
        parent_id: None,
        is_read: false,
        is_private: false,
        is_deleted: false,
    };

    // Serialize to JSON
    let json = serde_json::to_string(&message).expect("Failed to serialize to JSON");

    // Deserialize back
    let deserialized: Message =
        serde_json::from_str(&json).expect("Failed to deserialize from JSON");

    // Verify key fields match
    assert_eq!(message.from, deserialized.from);
    assert_eq!(message.to, deserialized.to);
    assert_eq!(message.subject, deserialized.subject);
    assert_eq!(message.body, deserialized.body);
}

#[test]
fn test_message_bincode_roundtrip() {
    let message = Message {
        id: 1,
        from: "Alice".to_string(),
        to: "Bob".to_string(),
        subject: "Test Subject".to_string(),
        body: "Message body".to_string(),
        date: chrono::Utc::now(),
        area_id: 1,
        parent_id: None,
        is_read: false,
        is_private: false,
        is_deleted: false,
    };

    // Serialize to bincode (bincode 2.0 API)
    let encoded = bincode::serde::encode_to_vec(&message, bincode::config::standard())
        .expect("Failed to serialize to bincode");

    // Deserialize back (bincode 2.0 API returns (T, usize) tuple)
    let decoded: Message = bincode::serde::decode_from_slice(&encoded, bincode::config::standard())
        .map(|(v, _)| v)
        .expect("Failed to deserialize from bincode");

    // Verify key fields match
    assert_eq!(message.from, decoded.from);
    assert_eq!(message.to, decoded.to);
    assert_eq!(message.subject, decoded.subject);
}

#[test]
fn test_file_entry_json_roundtrip() {
    let file_entry = FileEntry {
        id: 1,
        filename: "testfile.zip".to_string(),
        description: "A test file".to_string(),
        uploader: "uploader".to_string(),
        uploader_id: 42,
        size_bytes: 1024,
        upload_date: chrono::Utc::now(),
        area_id: 1,
        download_count: 0,
        is_offline: false,
        is_missing: false,
        password: None,
        cost_credits: None,
    };

    // Serialize to JSON
    let json = serde_json::to_string(&file_entry).expect("Failed to serialize to JSON");

    // Deserialize back
    let deserialized: FileEntry =
        serde_json::from_str(&json).expect("Failed to deserialize from JSON");

    // Verify key fields match
    assert_eq!(file_entry.filename, deserialized.filename);
    assert_eq!(file_entry.size_bytes, deserialized.size_bytes);
    assert_eq!(file_entry.description, deserialized.description);
    assert_eq!(file_entry.uploader, deserialized.uploader);
}

#[test]
fn test_file_entry_bincode_roundtrip() {
    let file_entry = FileEntry {
        id: 1,
        filename: "testfile.zip".to_string(),
        description: "Another test file".to_string(),
        uploader: "admin".to_string(),
        uploader_id: 1,
        size_bytes: 2048,
        upload_date: chrono::Utc::now(),
        area_id: 1,
        download_count: 0,
        is_offline: false,
        is_missing: false,
        password: None,
        cost_credits: None,
    };

    // Serialize to bincode (bincode 2.0 API)
    let encoded = bincode::serde::encode_to_vec(&file_entry, bincode::config::standard())
        .expect("Failed to serialize to bincode");

    // Deserialize back (bincode 2.0 API returns (T, usize) tuple)
    let decoded: FileEntry =
        bincode::serde::decode_from_slice(&encoded, bincode::config::standard())
            .map(|(v, _)| v)
            .expect("Failed to deserialize from bincode");

    // Verify key fields match
    assert_eq!(file_entry.filename, decoded.filename);
    assert_eq!(file_entry.size_bytes, decoded.size_bytes);
    assert_eq!(file_entry.description, decoded.description);
}

#[test]
fn test_bbs_config_json_roundtrip() {
    let config = BbsConfig {
        name: "Test BBS".to_string(),
        sysop: "TestSysOp".to_string(),
        sysop_email: Some("sysop@test.com".to_string()),
        location: Some("Test City".to_string()),
        servers: vec![ServerConfig {
            bind_address: "127.0.0.1".to_string(),
            port: 2323,
            protocol: Protocol::Telnet,
            enable_tls: false,
        }],
        paths: BbsPaths {
            data_dir: "/tmp/bbs/data".into(),
            users_dir: "/tmp/bbs/users".into(),
            messages_dir: "/tmp/bbs/messages".into(),
            files_dir: "/tmp/bbs/files".into(),
            logs_dir: "/tmp/bbs/logs".into(),
            temp_dir: "/tmp/bbs/temp".into(),
            doors_dir: "/tmp/bbs/doors".into(),
        },
        limits: SystemLimits {
            max_connections: 32,
            max_time_per_session: 120,
            max_daily_downloads: 10,
            max_upload_size: 10 * 1024 * 1024,
            max_message_length: 4096,
            min_password_length: 8,
            max_password_attempts: 3,
        },
        security: SecuritySettings {
            require_strong_passwords: true,
            enable_account_lockout: true,
            lockout_duration_minutes: 15,
            enable_rate_limiting: true,
            rate_limit_per_minute: 60,
            enable_audit_logging: true,
            require_email_verification: false,
        },
        enable_web_admin: true,
        web_admin_port: 8080,
        enable_ansi: true,
        enable_utf8: true,
        tagline: Some("Test BBS System".to_string()),
    };

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&config).expect("Failed to serialize to JSON");

    // Deserialize back
    let deserialized: BbsConfig =
        serde_json::from_str(&json).expect("Failed to deserialize from JSON");

    // Verify key fields match
    assert_eq!(config.name, deserialized.name);
    assert_eq!(config.sysop, deserialized.sysop);
    assert_eq!(
        config.servers[0].bind_address,
        deserialized.servers[0].bind_address
    );
    assert_eq!(config.servers[0].port, deserialized.servers[0].port);
    assert_eq!(
        config.security.lockout_duration_minutes,
        deserialized.security.lockout_duration_minutes
    );
}

#[test]
fn test_security_level_json_roundtrip() {
    let levels = vec![
        SecurityLevel::MIN,
        SecurityLevel::NEW_USER,
        SecurityLevel::VALIDATED,
        SecurityLevel::PRIVILEGED,
        SecurityLevel::COSYSOP,
        SecurityLevel::SYSOP,
    ];

    for level in levels {
        let json = serde_json::to_string(&level).expect("Failed to serialize to JSON");
        let deserialized: SecurityLevel =
            serde_json::from_str(&json).expect("Failed to deserialize from JSON");
        assert_eq!(level, deserialized);
    }
}

#[test]
fn test_user_stats_json_roundtrip() {
    let mut stats = UserStats::new();
    stats.record_upload(10, 1024);
    stats.record_download(20, 2048);
    stats.record_post();
    stats.record_time(120);

    // Serialize to JSON
    let json = serde_json::to_string(&stats).expect("Failed to serialize to JSON");

    // Deserialize back
    let deserialized: UserStats =
        serde_json::from_str(&json).expect("Failed to deserialize from JSON");

    // Verify fields match
    assert_eq!(stats.uploads, deserialized.uploads);
    assert_eq!(stats.downloads, deserialized.downloads);
    assert_eq!(stats.upload_kb, deserialized.upload_kb);
    assert_eq!(stats.download_kb, deserialized.download_kb);
    assert_eq!(stats.posts, deserialized.posts);
    assert_eq!(stats.total_time_minutes, deserialized.total_time_minutes);
}

#[test]
fn test_user_stats_bincode_roundtrip() {
    let mut stats = UserStats::new();
    stats.record_upload(5, 512);
    stats.record_download(15, 1024);

    // Serialize to bincode (bincode 2.0 API)
    let encoded = bincode::serde::encode_to_vec(&stats, bincode::config::standard())
        .expect("Failed to serialize to bincode");

    // Deserialize back (bincode 2.0 API returns (T, usize) tuple)
    let decoded: UserStats =
        bincode::serde::decode_from_slice(&encoded, bincode::config::standard())
            .map(|(v, _)| v)
            .expect("Failed to deserialize from bincode");

    // Verify fields match
    assert_eq!(stats.uploads, decoded.uploads);
    assert_eq!(stats.downloads, decoded.downloads);
    assert_eq!(stats.upload_kb, decoded.upload_kb);
    assert_eq!(stats.download_kb, decoded.download_kb);
}
