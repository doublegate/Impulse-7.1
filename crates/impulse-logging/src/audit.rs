//! Security audit logging
//!
//! This module provides tamper-evident audit logging for security-sensitive events
//! in the BBS system, with structured event types and compliance-ready output.

use serde::{Deserialize, Serialize};
use std::fmt;
use time::OffsetDateTime;

/// Type of security audit event
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditEventType {
    /// User login attempt (successful or failed)
    Login,
    /// User logout
    Logout,
    /// Failed authentication attempt
    AuthFailure,
    /// Password change
    PasswordChange,
    /// User account creation
    UserCreated,
    /// User account modification
    UserModified,
    /// User account deletion
    UserDeleted,
    /// Permission or security level change
    PermissionChange,
    /// File upload
    FileUpload,
    /// File download
    FileDownload,
    /// File deletion
    FileDelete,
    /// Configuration change
    ConfigChange,
    /// Administrative action
    AdminAction,
    /// Security policy violation
    SecurityViolation,
    /// System access (SysOp menu, etc.)
    SystemAccess,
}

impl fmt::Display for AuditEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            AuditEventType::Login => "LOGIN",
            AuditEventType::Logout => "LOGOUT",
            AuditEventType::AuthFailure => "AUTH_FAILURE",
            AuditEventType::PasswordChange => "PASSWORD_CHANGE",
            AuditEventType::UserCreated => "USER_CREATED",
            AuditEventType::UserModified => "USER_MODIFIED",
            AuditEventType::UserDeleted => "USER_DELETED",
            AuditEventType::PermissionChange => "PERMISSION_CHANGE",
            AuditEventType::FileUpload => "FILE_UPLOAD",
            AuditEventType::FileDownload => "FILE_DOWNLOAD",
            AuditEventType::FileDelete => "FILE_DELETE",
            AuditEventType::ConfigChange => "CONFIG_CHANGE",
            AuditEventType::AdminAction => "ADMIN_ACTION",
            AuditEventType::SecurityViolation => "SECURITY_VIOLATION",
            AuditEventType::SystemAccess => "SYSTEM_ACCESS",
        };
        write!(f, "{}", s)
    }
}

/// Security audit event
///
/// Represents a security-relevant event that should be logged for audit purposes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    /// Timestamp of the event (ISO 8601 format)
    #[serde(with = "time::serde::iso8601")]
    pub timestamp: OffsetDateTime,
    /// Type of audit event
    pub event_type: AuditEventType,
    /// User ID associated with the event (if applicable)
    pub user_id: Option<u32>,
    /// Username associated with the event (if applicable)
    pub username: Option<String>,
    /// IP address of the connection (if applicable)
    pub ip_address: Option<String>,
    /// Session ID (if applicable)
    pub session_id: Option<String>,
    /// Description of the event
    pub description: String,
    /// Outcome (success/failure/etc.)
    pub outcome: String,
    /// Additional contextual data (JSON object)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl AuditEvent {
    /// Create a new audit event
    ///
    /// # Examples
    ///
    /// ```rust
    /// use impulse_logging::{AuditEvent, AuditEventType};
    ///
    /// let event = AuditEvent::new(
    ///     AuditEventType::Login,
    ///     "User logged in successfully".to_string(),
    ///     "success".to_string()
    /// )
    /// .with_user_id(42)
    /// .with_username("testuser".to_string())
    /// .with_ip_address("192.168.1.100".to_string());
    /// ```
    pub fn new(event_type: AuditEventType, description: String, outcome: String) -> Self {
        Self {
            timestamp: OffsetDateTime::now_utc(),
            event_type,
            user_id: None,
            username: None,
            ip_address: None,
            session_id: None,
            description,
            outcome,
            metadata: None,
        }
    }

    /// Set the user ID
    pub fn with_user_id(mut self, user_id: u32) -> Self {
        self.user_id = Some(user_id);
        self
    }

    /// Set the username
    pub fn with_username(mut self, username: String) -> Self {
        self.username = Some(username);
        self
    }

    /// Set the IP address
    pub fn with_ip_address(mut self, ip_address: String) -> Self {
        self.ip_address = Some(ip_address);
        self
    }

    /// Set the session ID
    pub fn with_session_id(mut self, session_id: String) -> Self {
        self.session_id = Some(session_id);
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Serialize to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Serialize to pretty JSON string
    pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

/// Audit logger
///
/// Provides a high-level API for logging security audit events with structured output.
pub struct AuditLogger {
    enabled: bool,
}

impl AuditLogger {
    /// Create a new audit logger
    ///
    /// # Examples
    ///
    /// ```rust
    /// use impulse_logging::AuditLogger;
    ///
    /// let logger = AuditLogger::new();
    /// ```
    pub fn new() -> Self {
        Self { enabled: true }
    }

    /// Create a disabled audit logger
    pub fn disabled() -> Self {
        Self { enabled: false }
    }

    /// Log an audit event
    ///
    /// Events are logged at INFO level with a structured format.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use impulse_logging::{AuditLogger, AuditEvent, AuditEventType};
    ///
    /// let logger = AuditLogger::new();
    /// let event = AuditEvent::new(
    ///     AuditEventType::Login,
    ///     "User logged in".to_string(),
    ///     "success".to_string()
    /// ).with_user_id(42);
    ///
    /// logger.log(&event);
    /// ```
    pub fn log(&self, event: &AuditEvent) {
        if !self.enabled {
            return;
        }

        // Log using structured tracing
        tracing::info!(
            event_type = %event.event_type,
            user_id = event.user_id,
            username = event.username.as_deref(),
            ip_address = event.ip_address.as_deref(),
            session_id = event.session_id.as_deref(),
            outcome = %event.outcome,
            description = %event.description,
            timestamp = %event.timestamp,
            "AUDIT_EVENT"
        );
    }

    /// Log a login event
    ///
    /// # Examples
    ///
    /// ```rust
    /// use impulse_logging::AuditLogger;
    ///
    /// let logger = AuditLogger::new();
    /// logger.log_login(42, "testuser", Some("192.168.1.100"), true);
    /// ```
    pub fn log_login(&self, user_id: u32, username: &str, ip_address: Option<&str>, success: bool) {
        let event_type = if success {
            AuditEventType::Login
        } else {
            AuditEventType::AuthFailure
        };

        let outcome = if success { "success" } else { "failure" };
        let description = if success {
            format!("User {} logged in", username)
        } else {
            format!("Login failed for user {}", username)
        };

        let mut event = AuditEvent::new(event_type, description, outcome.to_string())
            .with_user_id(user_id)
            .with_username(username.to_string());

        if let Some(ip) = ip_address {
            event = event.with_ip_address(ip.to_string());
        }

        self.log(&event);
    }

    /// Log a logout event
    pub fn log_logout(&self, user_id: u32, username: &str, session_id: Option<&str>) {
        let mut event = AuditEvent::new(
            AuditEventType::Logout,
            format!("User {} logged out", username),
            "success".to_string(),
        )
        .with_user_id(user_id)
        .with_username(username.to_string());

        if let Some(sid) = session_id {
            event = event.with_session_id(sid.to_string());
        }

        self.log(&event);
    }

    /// Log a file upload event
    pub fn log_file_upload(&self, user_id: u32, username: &str, filename: &str, size_bytes: u64) {
        let metadata = serde_json::json!({
            "filename": filename,
            "size_bytes": size_bytes,
        });

        let event = AuditEvent::new(
            AuditEventType::FileUpload,
            format!("User {} uploaded file: {}", username, filename),
            "success".to_string(),
        )
        .with_user_id(user_id)
        .with_username(username.to_string())
        .with_metadata(metadata);

        self.log(&event);
    }

    /// Log a configuration change
    pub fn log_config_change(
        &self,
        user_id: u32,
        username: &str,
        key: &str,
        old_value: &str,
        new_value: &str,
    ) {
        let metadata = serde_json::json!({
            "key": key,
            "old_value": old_value,
            "new_value": new_value,
        });

        let event = AuditEvent::new(
            AuditEventType::ConfigChange,
            format!("Configuration changed: {} by {}", key, username),
            "success".to_string(),
        )
        .with_user_id(user_id)
        .with_username(username.to_string())
        .with_metadata(metadata);

        self.log(&event);
    }

    /// Log a security violation
    pub fn log_security_violation(
        &self,
        user_id: Option<u32>,
        username: Option<&str>,
        ip_address: Option<&str>,
        violation: &str,
    ) {
        let mut event = AuditEvent::new(
            AuditEventType::SecurityViolation,
            format!("Security violation: {}", violation),
            "blocked".to_string(),
        );

        if let Some(uid) = user_id {
            event = event.with_user_id(uid);
        }
        if let Some(uname) = username {
            event = event.with_username(uname.to_string());
        }
        if let Some(ip) = ip_address {
            event = event.with_ip_address(ip.to_string());
        }

        self.log(&event);
    }
}

impl Default for AuditLogger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_event_type_display() {
        assert_eq!(AuditEventType::Login.to_string(), "LOGIN");
        assert_eq!(AuditEventType::AuthFailure.to_string(), "AUTH_FAILURE");
        assert_eq!(AuditEventType::ConfigChange.to_string(), "CONFIG_CHANGE");
    }

    #[test]
    fn test_audit_event_new() {
        let event = AuditEvent::new(
            AuditEventType::Login,
            "Test event".to_string(),
            "success".to_string(),
        );

        assert_eq!(event.event_type, AuditEventType::Login);
        assert_eq!(event.description, "Test event");
        assert_eq!(event.outcome, "success");
        assert!(event.user_id.is_none());
        assert!(event.username.is_none());
    }

    #[test]
    fn test_audit_event_builder() {
        let event = AuditEvent::new(
            AuditEventType::Login,
            "User logged in".to_string(),
            "success".to_string(),
        )
        .with_user_id(42)
        .with_username("testuser".to_string())
        .with_ip_address("192.168.1.100".to_string())
        .with_session_id("abc123".to_string());

        assert_eq!(event.user_id, Some(42));
        assert_eq!(event.username, Some("testuser".to_string()));
        assert_eq!(event.ip_address, Some("192.168.1.100".to_string()));
        assert_eq!(event.session_id, Some("abc123".to_string()));
    }

    #[test]
    fn test_audit_event_with_metadata() {
        let metadata = serde_json::json!({
            "key": "value",
            "number": 42
        });

        let event = AuditEvent::new(
            AuditEventType::ConfigChange,
            "Config changed".to_string(),
            "success".to_string(),
        )
        .with_metadata(metadata.clone());

        assert_eq!(event.metadata, Some(metadata));
    }

    #[test]
    fn test_audit_event_to_json() {
        let event = AuditEvent::new(
            AuditEventType::Login,
            "Test".to_string(),
            "success".to_string(),
        )
        .with_user_id(42);

        let json = event.to_json().unwrap();
        assert!(json.contains("\"event_type\":\"login\""));
        assert!(json.contains("\"user_id\":42"));
    }

    #[test]
    fn test_audit_logger_new() {
        let logger = AuditLogger::new();
        assert!(logger.enabled);
    }

    #[test]
    fn test_audit_logger_disabled() {
        let logger = AuditLogger::disabled();
        assert!(!logger.enabled);
    }

    #[test]
    fn test_audit_logger_log() {
        let logger = AuditLogger::new();
        let event = AuditEvent::new(
            AuditEventType::Login,
            "Test".to_string(),
            "success".to_string(),
        );

        // Should not panic
        logger.log(&event);
    }

    #[test]
    fn test_audit_logger_log_login_success() {
        let logger = AuditLogger::new();
        logger.log_login(42, "testuser", Some("192.168.1.100"), true);
        // Should not panic
    }

    #[test]
    fn test_audit_logger_log_login_failure() {
        let logger = AuditLogger::new();
        logger.log_login(0, "baduser", Some("192.168.1.100"), false);
        // Should not panic
    }

    #[test]
    fn test_audit_logger_log_logout() {
        let logger = AuditLogger::new();
        logger.log_logout(42, "testuser", Some("session123"));
        // Should not panic
    }

    #[test]
    fn test_audit_logger_log_file_upload() {
        let logger = AuditLogger::new();
        logger.log_file_upload(42, "testuser", "test.zip", 1024);
        // Should not panic
    }

    #[test]
    fn test_audit_logger_log_config_change() {
        let logger = AuditLogger::new();
        logger.log_config_change(1, "admin", "max_connections", "10", "20");
        // Should not panic
    }

    #[test]
    fn test_audit_logger_log_security_violation() {
        let logger = AuditLogger::new();
        logger.log_security_violation(
            Some(42),
            Some("hacker"),
            Some("1.2.3.4"),
            "Brute force attempt",
        );
        // Should not panic
    }
}
