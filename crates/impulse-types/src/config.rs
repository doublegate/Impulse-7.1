//! Configuration types
//!
//! This module defines the BBS system configuration structures,
//! including server settings, paths, limits, and security policies.

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Network protocol type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Protocol {
    /// Telnet protocol
    Telnet,
    /// SSH protocol
    Ssh,
    /// Raw TCP
    Raw,
}

/// BBS system limits
///
/// Defines operational limits for the BBS system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemLimits {
    /// Maximum number of simultaneous connections
    pub max_connections: u32,
    /// Maximum login time per session (minutes)
    pub max_time_per_session: u32,
    /// Maximum daily downloads per user
    pub max_daily_downloads: u32,
    /// Maximum upload file size (bytes)
    pub max_upload_size: u64,
    /// Maximum message length (bytes)
    pub max_message_length: u32,
    /// Minimum password length
    pub min_password_length: u8,
    /// Maximum password attempts before lockout
    pub max_password_attempts: u8,
}

impl Default for SystemLimits {
    fn default() -> Self {
        Self {
            max_connections: 10,
            max_time_per_session: 60,
            max_daily_downloads: 100,
            max_upload_size: 10 * 1024 * 1024, // 10 MB
            max_message_length: 65536,         // 64 KB
            min_password_length: 6,
            max_password_attempts: 3,
        }
    }
}

/// BBS security settings
///
/// Defines security policies for the BBS system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySettings {
    /// Require secure password hashing (Argon2id)
    pub require_strong_passwords: bool,
    /// Enable account lockout after failed attempts
    pub enable_account_lockout: bool,
    /// Lockout duration in minutes
    pub lockout_duration_minutes: u32,
    /// Enable IP-based rate limiting
    pub enable_rate_limiting: bool,
    /// Maximum requests per minute per IP
    pub rate_limit_per_minute: u32,
    /// Enable audit logging
    pub enable_audit_logging: bool,
    /// Require email verification for new accounts
    pub require_email_verification: bool,
}

impl Default for SecuritySettings {
    fn default() -> Self {
        Self {
            require_strong_passwords: true,
            enable_account_lockout: true,
            lockout_duration_minutes: 30,
            enable_rate_limiting: true,
            rate_limit_per_minute: 60,
            enable_audit_logging: true,
            require_email_verification: false,
        }
    }
}

/// BBS paths configuration
///
/// Defines filesystem paths for BBS data storage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BbsPaths {
    /// Base data directory
    pub data_dir: PathBuf,
    /// User files directory
    pub users_dir: PathBuf,
    /// Message base directory
    pub messages_dir: PathBuf,
    /// File areas directory
    pub files_dir: PathBuf,
    /// Logs directory
    pub logs_dir: PathBuf,
    /// Temporary files directory
    pub temp_dir: PathBuf,
    /// Door games directory
    pub doors_dir: PathBuf,
}

impl Default for BbsPaths {
    fn default() -> Self {
        let base = PathBuf::from("./data");
        Self {
            users_dir: base.join("users"),
            messages_dir: base.join("messages"),
            files_dir: base.join("files"),
            logs_dir: base.join("logs"),
            temp_dir: base.join("temp"),
            doors_dir: base.join("doors"),
            data_dir: base,
        }
    }
}

/// Network server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Server bind address
    pub bind_address: String,
    /// Server port
    pub port: u16,
    /// Protocol type
    pub protocol: Protocol,
    /// Enable TLS/SSL
    pub enable_tls: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_address: "0.0.0.0".to_string(),
            port: 2323,
            protocol: Protocol::Telnet,
            enable_tls: false,
        }
    }
}

/// Complete BBS system configuration
///
/// The main configuration structure for the Impulse 7.1 BBS system.
/// Use the builder pattern for complex initialization.
///
/// # Examples
///
/// ```
/// use impulse_types::config::BbsConfig;
///
/// let config = BbsConfig::builder()
///     .name("My BBS".to_string())
///     .sysop("SysOp Name".to_string())
///     .build();
///
/// assert!(config.validate().is_ok());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BbsConfig {
    /// BBS name/title
    pub name: String,

    /// System Operator name
    pub sysop: String,

    /// SysOp email address
    pub sysop_email: Option<String>,

    /// BBS location/city
    pub location: Option<String>,

    /// Network servers configuration
    pub servers: Vec<ServerConfig>,

    /// Filesystem paths
    pub paths: BbsPaths,

    /// System operational limits
    pub limits: SystemLimits,

    /// Security settings
    pub security: SecuritySettings,

    /// Enable web admin panel
    pub enable_web_admin: bool,

    /// Web admin port
    pub web_admin_port: u16,

    /// Enable ANSI/color support
    pub enable_ansi: bool,

    /// Enable UTF-8 support
    pub enable_utf8: bool,

    /// BBS tagline/slogan
    pub tagline: Option<String>,
}

impl Default for BbsConfig {
    fn default() -> Self {
        Self {
            name: "Impulse BBS".to_string(),
            sysop: "SysOp".to_string(),
            sysop_email: None,
            location: None,
            servers: vec![ServerConfig::default()],
            paths: BbsPaths::default(),
            limits: SystemLimits::default(),
            security: SecuritySettings::default(),
            enable_web_admin: true,
            web_admin_port: 8080,
            enable_ansi: true,
            enable_utf8: true,
            tagline: None,
        }
    }
}

impl BbsConfig {
    /// Create a new configuration builder
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::config::BbsConfig;
    ///
    /// let config = BbsConfig::builder()
    ///     .name("Test BBS".to_string())
    ///     .sysop("Admin".to_string())
    ///     .build();
    /// ```
    pub fn builder() -> BbsConfigBuilder {
        BbsConfigBuilder::default()
    }

    /// Validate the configuration
    ///
    /// Ensures all required fields are properly set and values are within acceptable ranges.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Config`] if:
    /// - BBS name is empty or too long
    /// - SysOp name is empty or too long
    /// - Port numbers are invalid
    /// - Paths are not set
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::config::BbsConfig;
    ///
    /// let config = BbsConfig::default();
    /// assert!(config.validate().is_ok());
    /// ```
    pub fn validate(&self) -> Result<()> {
        // Validate BBS name
        if self.name.is_empty() {
            return Err(Error::Config("BBS name cannot be empty".to_string()));
        }
        if self.name.len() > 80 {
            return Err(Error::Config(
                "BBS name must be 80 characters or less".to_string(),
            ));
        }

        // Validate SysOp name
        if self.sysop.is_empty() {
            return Err(Error::Config("SysOp name cannot be empty".to_string()));
        }
        if self.sysop.len() > 30 {
            return Err(Error::Config(
                "SysOp name must be 30 characters or less".to_string(),
            ));
        }

        // Validate email if provided
        if let Some(email) = &self.sysop_email
            && !email.is_empty()
            && !email.contains('@')
        {
            return Err(Error::Config("Invalid SysOp email format".to_string()));
        }

        // Validate servers
        if self.servers.is_empty() {
            return Err(Error::Config(
                "At least one server must be configured".to_string(),
            ));
        }

        // Validate ports
        for server in &self.servers {
            if server.port == 0 {
                return Err(Error::Config("Invalid server port (0)".to_string()));
            }
        }

        if self.web_admin_port == 0 {
            return Err(Error::Config("Invalid web admin port (0)".to_string()));
        }

        // Validate limits
        if self.limits.max_connections == 0 {
            return Err(Error::Config(
                "Maximum connections must be at least 1".to_string(),
            ));
        }

        if self.limits.min_password_length < 4 {
            return Err(Error::Config(
                "Minimum password length must be at least 4".to_string(),
            ));
        }

        Ok(())
    }

    /// Get the primary server configuration
    pub fn primary_server(&self) -> Option<&ServerConfig> {
        self.servers.first()
    }
}

/// Builder for BbsConfig
///
/// Provides a convenient way to construct complex BBS configurations.
#[derive(Default)]
pub struct BbsConfigBuilder {
    config: BbsConfig,
}

impl BbsConfigBuilder {
    /// Set the BBS name
    pub fn name(mut self, name: String) -> Self {
        self.config.name = name;
        self
    }

    /// Set the SysOp name
    pub fn sysop(mut self, sysop: String) -> Self {
        self.config.sysop = sysop;
        self
    }

    /// Set the SysOp email
    pub fn sysop_email(mut self, email: String) -> Self {
        self.config.sysop_email = Some(email);
        self
    }

    /// Set the BBS location
    pub fn location(mut self, location: String) -> Self {
        self.config.location = Some(location);
        self
    }

    /// Set the paths configuration
    pub fn paths(mut self, paths: BbsPaths) -> Self {
        self.config.paths = paths;
        self
    }

    /// Set the system limits
    pub fn limits(mut self, limits: SystemLimits) -> Self {
        self.config.limits = limits;
        self
    }

    /// Set the security settings
    pub fn security(mut self, security: SecuritySettings) -> Self {
        self.config.security = security;
        self
    }

    /// Add a server configuration
    pub fn add_server(mut self, server: ServerConfig) -> Self {
        if self.config.servers.is_empty() || self.config.servers[0].port == 2323 {
            self.config.servers = vec![server];
        } else {
            self.config.servers.push(server);
        }
        self
    }

    /// Build the configuration
    pub fn build(self) -> BbsConfig {
        self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = BbsConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_builder() {
        let config = BbsConfig::builder()
            .name("Test BBS".to_string())
            .sysop("Admin".to_string())
            .sysop_email("admin@test.com".to_string())
            .location("Test City".to_string())
            .build();

        assert_eq!(config.name, "Test BBS");
        assert_eq!(config.sysop, "Admin");
        assert_eq!(config.sysop_email, Some("admin@test.com".to_string()));
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_empty_name() {
        let config = BbsConfig {
            name: String::new(),
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_name_too_long() {
        let config = BbsConfig {
            name: "a".repeat(81),
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_empty_sysop() {
        let config = BbsConfig {
            sysop: String::new(),
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_email() {
        let config = BbsConfig {
            sysop_email: Some("not_an_email".to_string()),
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_no_servers() {
        let config = BbsConfig {
            servers: vec![],
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_port() {
        let mut config = BbsConfig::default();
        config.servers[0].port = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_max_connections_zero() {
        let mut config = BbsConfig::default();
        config.limits.max_connections = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_min_password_too_short() {
        let mut config = BbsConfig::default();
        config.limits.min_password_length = 3;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_primary_server() {
        let config = BbsConfig::default();
        assert!(config.primary_server().is_some());
        assert_eq!(config.primary_server().unwrap().port, 2323);
    }

    #[test]
    fn test_default_limits() {
        let limits = SystemLimits::default();
        assert_eq!(limits.max_connections, 10);
        assert_eq!(limits.min_password_length, 6);
    }

    #[test]
    fn test_default_security() {
        let security = SecuritySettings::default();
        assert!(security.require_strong_passwords);
        assert!(security.enable_account_lockout);
        assert!(security.enable_audit_logging);
    }

    #[test]
    fn test_default_paths() {
        let paths = BbsPaths::default();
        assert_eq!(paths.data_dir, PathBuf::from("./data"));
        assert!(paths.users_dir.ends_with("users"));
    }

    #[test]
    fn test_protocol_serialization() {
        let protocol = Protocol::Telnet;
        let json = serde_json::to_string(&protocol).unwrap();
        let deserialized: Protocol = serde_json::from_str(&json).unwrap();
        assert_eq!(protocol, deserialized);
    }
}
