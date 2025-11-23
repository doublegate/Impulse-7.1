//! Serialization round-trip tests
//!
//! Tests to ensure all types can be serialized and deserialized correctly
//! in both JSON and binary (bincode) formats.

use impulse_types::{
    config::{BbsConfig, BbsPaths, Protocol, SecuritySettings, ServerConfig, SystemLimits},
    file::FileEntry,
    message::Message,
    user::{SecurityLevel, User, UserStats},
};

#[test]
fn test_user_json_roundtrip() {
    let user = User {
        id: 42,
        name: "TestUser".to_string(),
        password_hash: "hashed_password_here".to_string(),
        security_level: SecurityLevel::Validated,
        real_name: Some("Test User".to_string()),
        location: Some("Test City".to_string()),
        stats: UserStats {
            calls: 100,
            uploads: 50,
            downloads: 75,
            upload_kb: 10240,
            download_kb: 20480,
            messages_posted: 200,
            time_online: 500,
        },
        registration_date: chrono::Utc::now(),
        last_login: Some(chrono::Utc::now()),
        email: Some("test@example.com".to_string()),
        phone: Some("555-1234".to_string()),
        birthday: Some(chrono::NaiveDate::from_ymd_opt(1990, 1, 1).unwrap()),
        notes: Some("Test notes".to_string()),
    };

    // Serialize to JSON
    let json = serde_json::to_string(&user).expect("Failed to serialize to JSON");

    // Deserialize back
    let deserialized: User = serde_json::from_str(&json).expect("Failed to deserialize from JSON");

    // Verify fields match
    assert_eq!(user.id, deserialized.id);
    assert_eq!(user.name, deserialized.name);
    assert_eq!(user.password_hash, deserialized.password_hash);
    assert_eq!(user.security_level, deserialized.security_level);
    assert_eq!(user.email, deserialized.email);
}

#[test]
fn test_user_bincode_roundtrip() {
    let user = User {
        id: 42,
        name: "TestUser".to_string(),
        password_hash: "hashed".to_string(),
        security_level: SecurityLevel::SysOp,
        real_name: None,
        location: None,
        stats: UserStats::default(),
        registration_date: chrono::Utc::now(),
        last_login: None,
        email: None,
        phone: None,
        birthday: None,
        notes: None,
    };

    // Serialize to bincode
    let encoded = bincode::serialize(&user).expect("Failed to serialize to bincode");

    // Deserialize back
    let decoded: User = bincode::deserialize(&encoded).expect("Failed to deserialize from bincode");

    // Verify fields match
    assert_eq!(user.id, decoded.id);
    assert_eq!(user.name, decoded.name);
    assert_eq!(user.security_level, decoded.security_level);
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
        area_id: 5,
        parent_id: Some(10),
        is_read: false,
        is_private: true,
        is_deleted: false,
    };

    // Serialize to JSON
    let json = serde_json::to_string(&message).expect("Failed to serialize to JSON");

    // Deserialize back
    let deserialized: Message =
        serde_json::from_str(&json).expect("Failed to deserialize from JSON");

    // Verify fields match
    assert_eq!(message.id, deserialized.id);
    assert_eq!(message.from, deserialized.from);
    assert_eq!(message.to, deserialized.to);
    assert_eq!(message.subject, deserialized.subject);
    assert_eq!(message.body, deserialized.body);
    assert_eq!(message.area_id, deserialized.area_id);
    assert_eq!(message.parent_id, deserialized.parent_id);
    assert_eq!(message.is_private, deserialized.is_private);
}

#[test]
fn test_message_bincode_roundtrip() {
    let message = Message {
        id: 1,
        from: "Alice".to_string(),
        to: "All".to_string(),
        subject: "Public Announcement".to_string(),
        body: "Hello everyone!".to_string(),
        date: chrono::Utc::now(),
        area_id: 1,
        parent_id: None,
        is_read: true,
        is_private: false,
        is_deleted: false,
    };

    // Serialize to bincode
    let encoded = bincode::serialize(&message).expect("Failed to serialize to bincode");

    // Deserialize back
    let decoded: Message =
        bincode::deserialize(&encoded).expect("Failed to deserialize from bincode");

    // Verify fields match
    assert_eq!(message.id, decoded.id);
    assert_eq!(message.from, decoded.from);
    assert_eq!(message.subject, decoded.subject);
}

#[test]
fn test_file_entry_json_roundtrip() {
    let file = FileEntry {
        id: 1,
        filename: "testfile.zip".to_string(),
        description: "Test file description".to_string(),
        uploader: "Alice".to_string(),
        uploader_id: 42,
        size_bytes: 1024000,
        upload_date: chrono::Utc::now(),
        area_id: 3,
        download_count: 10,
        is_offline: false,
        is_missing: false,
        password: Some("secret".to_string()),
        cost_credits: Some(100),
    };

    // Serialize to JSON
    let json = serde_json::to_string(&file).expect("Failed to serialize to JSON");

    // Deserialize back
    let deserialized: FileEntry =
        serde_json::from_str(&json).expect("Failed to deserialize from JSON");

    // Verify fields match
    assert_eq!(file.id, deserialized.id);
    assert_eq!(file.filename, deserialized.filename);
    assert_eq!(file.description, deserialized.description);
    assert_eq!(file.uploader, deserialized.uploader);
    assert_eq!(file.size_bytes, deserialized.size_bytes);
    assert_eq!(file.download_count, deserialized.download_count);
    assert_eq!(file.password, deserialized.password);
    assert_eq!(file.cost_credits, deserialized.cost_credits);
}

#[test]
fn test_file_entry_bincode_roundtrip() {
    let file = FileEntry {
        id: 2,
        filename: "document.pdf".to_string(),
        description: "Important document".to_string(),
        uploader: "Bob".to_string(),
        uploader_id: 99,
        size_bytes: 2048000,
        upload_date: chrono::Utc::now(),
        area_id: 1,
        download_count: 0,
        is_offline: false,
        is_missing: false,
        password: None,
        cost_credits: None,
    };

    // Serialize to bincode
    let encoded = bincode::serialize(&file).expect("Failed to serialize to bincode");

    // Deserialize back
    let decoded: FileEntry =
        bincode::deserialize(&encoded).expect("Failed to deserialize from bincode");

    // Verify fields match
    assert_eq!(file.id, decoded.id);
    assert_eq!(file.filename, decoded.filename);
    assert_eq!(file.size_bytes, decoded.size_bytes);
}

#[test]
fn test_bbs_config_json_roundtrip() {
    let config = BbsConfig::builder()
        .name("Test BBS".to_string())
        .sysop("Admin".to_string())
        .sysop_email("admin@test.com".to_string())
        .location("Test City".to_string())
        .build();

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&config).expect("Failed to serialize to JSON");

    // Deserialize back
    let deserialized: BbsConfig =
        serde_json::from_str(&json).expect("Failed to deserialize from JSON");

    // Verify fields match
    assert_eq!(config.name, deserialized.name);
    assert_eq!(config.sysop, deserialized.sysop);
    assert_eq!(config.sysop_email, deserialized.sysop_email);
    assert_eq!(config.location, deserialized.location);
    assert_eq!(
        config.limits.max_connections,
        deserialized.limits.max_connections
    );
    assert_eq!(
        config.security.require_strong_passwords,
        deserialized.security.require_strong_passwords
    );
}

#[test]
fn test_bbs_config_bincode_roundtrip() {
    let config = BbsConfig::default();

    // Serialize to bincode
    let encoded = bincode::serialize(&config).expect("Failed to serialize to bincode");

    // Deserialize back
    let decoded: BbsConfig =
        bincode::deserialize(&encoded).expect("Failed to deserialize from bincode");

    // Verify fields match
    assert_eq!(config.name, decoded.name);
    assert_eq!(config.sysop, decoded.sysop);
    assert_eq!(config.enable_ansi, decoded.enable_ansi);
}

#[test]
fn test_security_level_serialization() {
    let levels = vec![
        SecurityLevel::Locked,
        SecurityLevel::NewUser,
        SecurityLevel::Validated,
        SecurityLevel::Privileged,
        SecurityLevel::AssistantSysOp,
        SecurityLevel::SysOp,
    ];

    for level in levels {
        // JSON round-trip
        let json = serde_json::to_string(&level).expect("Failed to serialize");
        let decoded: SecurityLevel = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(level, decoded);

        // Bincode round-trip
        let bin = bincode::serialize(&level).expect("Failed to serialize");
        let decoded: SecurityLevel = bincode::deserialize(&bin).expect("Failed to deserialize");
        assert_eq!(level, decoded);
    }
}

#[test]
fn test_protocol_serialization() {
    let protocols = vec![Protocol::Telnet, Protocol::Ssh, Protocol::Raw];

    for protocol in protocols {
        // JSON round-trip
        let json = serde_json::to_string(&protocol).expect("Failed to serialize");
        let decoded: Protocol = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(protocol, decoded);

        // Bincode round-trip
        let bin = bincode::serialize(&protocol).expect("Failed to serialize");
        let decoded: Protocol = bincode::deserialize(&bin).expect("Failed to deserialize");
        assert_eq!(protocol, decoded);
    }
}

#[test]
fn test_complex_nested_structure() {
    // Create a complex configuration with custom settings
    let paths = BbsPaths::default();
    let limits = SystemLimits {
        max_connections: 50,
        max_time_per_session: 120,
        max_daily_downloads: 200,
        max_upload_size: 50 * 1024 * 1024,
        max_message_length: 32768,
        min_password_length: 8,
        max_password_attempts: 5,
    };
    let security = SecuritySettings {
        require_strong_passwords: true,
        enable_account_lockout: true,
        lockout_duration_minutes: 60,
        enable_rate_limiting: true,
        rate_limit_per_minute: 100,
        enable_audit_logging: true,
        require_email_verification: true,
    };

    let config = BbsConfig {
        name: "Complex BBS".to_string(),
        sysop: "SuperAdmin".to_string(),
        sysop_email: Some("admin@complex.bbs".to_string()),
        location: Some("Cyberspace".to_string()),
        servers: vec![
            ServerConfig {
                bind_address: "0.0.0.0".to_string(),
                port: 2323,
                protocol: Protocol::Telnet,
                enable_tls: false,
            },
            ServerConfig {
                bind_address: "0.0.0.0".to_string(),
                port: 2222,
                protocol: Protocol::Ssh,
                enable_tls: true,
            },
        ],
        paths,
        limits,
        security,
        enable_web_admin: true,
        web_admin_port: 8443,
        enable_ansi: true,
        enable_utf8: true,
        tagline: Some("The most complex BBS in the world!".to_string()),
    };

    // JSON round-trip
    let json = serde_json::to_string_pretty(&config).expect("Failed to serialize");
    let decoded: BbsConfig = serde_json::from_str(&json).expect("Failed to deserialize");

    assert_eq!(config.name, decoded.name);
    assert_eq!(config.servers.len(), decoded.servers.len());
    assert_eq!(config.servers[0].port, decoded.servers[0].port);
    assert_eq!(config.servers[1].protocol, decoded.servers[1].protocol);
    assert_eq!(
        config.limits.max_connections,
        decoded.limits.max_connections
    );

    // Bincode round-trip
    let bin = bincode::serialize(&config).expect("Failed to serialize");
    let decoded: BbsConfig = bincode::deserialize(&bin).expect("Failed to deserialize");

    assert_eq!(config.name, decoded.name);
    assert_eq!(config.tagline, decoded.tagline);
}
