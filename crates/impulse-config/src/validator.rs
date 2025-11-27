//! Enhanced configuration validation with filesystem checks

use crate::error::{ConfigError, Result};
use impulse_types::config::BbsConfig;
use std::path::Path;

/// Validation options for configuration
#[derive(Debug, Clone, Default)]
pub struct ValidationOptions {
    /// Check if paths exist on the filesystem
    pub check_paths: bool,
    /// Check if ports are actually available
    pub check_ports: bool,
    /// Allow empty directories (will be created)
    pub allow_empty_dirs: bool,
}

impl ValidationOptions {
    /// Create validation options that only check configuration values
    ///
    /// Does not perform filesystem or network checks.
    pub fn config_only() -> Self {
        Self {
            check_paths: false,
            check_ports: false,
            allow_empty_dirs: true,
        }
    }

    /// Create validation options that perform all checks
    ///
    /// Includes filesystem path checks and port availability checks.
    pub fn strict() -> Self {
        Self {
            check_paths: true,
            check_ports: true,
            allow_empty_dirs: false,
        }
    }

    /// Create validation options for deployment
    ///
    /// Checks filesystem paths but allows empty directories that will be created.
    /// Does not check port availability as the BBS may not be running yet.
    pub fn deployment() -> Self {
        Self {
            check_paths: true,
            check_ports: false,
            allow_empty_dirs: true,
        }
    }
}

/// Validate configuration with enhanced checks
///
/// This function wraps the standard `BbsConfig::validate()` and adds
/// optional filesystem and network checks based on the provided options.
///
/// # Example
/// ```
/// use impulse_config::{Config, ValidationOptions, validate_config};
///
/// let config = Config::with_defaults();
/// let options = ValidationOptions::config_only();
///
/// // Validate configuration values (no filesystem/port checks)
/// validate_config(config.inner(), &options)?;
/// # Ok::<(), impulse_config::ConfigError>(())
/// ```
pub fn validate_config(config: &BbsConfig, options: &ValidationOptions) -> Result<()> {
    // First run the standard validation
    config.validate().map_err(|e| {
        ConfigError::ValidationError(format!("Configuration validation failed: {}", e))
    })?;

    // Perform filesystem checks if enabled
    if options.check_paths {
        validate_paths(config, options.allow_empty_dirs)?;
    }

    // Perform port availability checks if enabled
    if options.check_ports {
        validate_ports(config)?;
    }

    Ok(())
}

/// Validate that configured paths exist or can be created
fn validate_paths(config: &BbsConfig, allow_empty: bool) -> Result<()> {
    // Check data directory
    check_path(&config.paths.data_dir, "data directory", allow_empty)?;

    // Check users directory
    check_path(&config.paths.users_dir, "users directory", allow_empty)?;

    // Check messages directory
    check_path(
        &config.paths.messages_dir,
        "messages directory",
        allow_empty,
    )?;

    // Check files directory
    check_path(&config.paths.files_dir, "files directory", allow_empty)?;

    // Check logs directory
    check_path(&config.paths.logs_dir, "logs directory", allow_empty)?;

    // Check temp directory
    check_path(&config.paths.temp_dir, "temp directory", allow_empty)?;

    // Check doors directory
    check_path(&config.paths.doors_dir, "doors directory", allow_empty)?;

    Ok(())
}

/// Check if a path exists or if its parent directory exists (for creation)
fn check_path(path: &Path, name: &str, allow_empty: bool) -> Result<()> {
    let path_obj = path;

    if path_obj.exists() {
        if !path_obj.is_dir() {
            return Err(ConfigError::InvalidValue(format!(
                "{} exists but is not a directory: {}",
                name,
                path_obj.display()
            )));
        }
        Ok(())
    } else if allow_empty {
        // Check if parent directory exists so we can create this directory
        if let Some(parent) = path_obj.parent()
            && !parent.exists()
        {
            return Err(ConfigError::PathNotFound(parent.to_path_buf()));
        }
        Ok(())
    } else {
        Err(ConfigError::PathNotFound(path_obj.to_path_buf()))
    }
}

/// Validate that configured ports are available
fn validate_ports(config: &BbsConfig) -> Result<()> {
    // Check each server port
    for server in &config.servers {
        if !is_port_available(server.port) {
            return Err(ConfigError::PortInUse(server.port));
        }
    }

    // Check web admin port
    if !is_port_available(config.web_admin_port) {
        return Err(ConfigError::PortInUse(config.web_admin_port));
    }

    Ok(())
}

/// Check if a port is available by attempting to bind to it
fn is_port_available(port: u16) -> bool {
    use std::net::{SocketAddr, TcpListener};

    // Port 0 is always available (kernel assigns a free port)
    if port == 0 {
        return true;
    }

    // Try IPv4 first (0.0.0.0 - all interfaces)
    let ipv4 = SocketAddr::from(([0, 0, 0, 0], port));
    if TcpListener::bind(ipv4).is_ok() {
        // IPv4 binding succeeded, now check if we can also bind IPv6
        // A port is truly available if we can bind to it on the primary protocol
        // IPv6 is optional, so we don't fail if it's not available
        return true;
    }

    // IPv4 failed, try IPv6 (:: - all interfaces)
    let ipv6 = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], port));
    TcpListener::bind(ipv6).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use impulse_types::config::BbsConfig;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_validation_options() {
        let config_only = ValidationOptions::config_only();
        assert!(!config_only.check_paths);
        assert!(!config_only.check_ports);
        assert!(config_only.allow_empty_dirs);

        let strict = ValidationOptions::strict();
        assert!(strict.check_paths);
        assert!(strict.check_ports);
        assert!(!strict.allow_empty_dirs);

        let deployment = ValidationOptions::deployment();
        assert!(deployment.check_paths);
        assert!(!deployment.check_ports);
        assert!(deployment.allow_empty_dirs);
    }

    #[test]
    fn test_validate_config_only() {
        let config = BbsConfig::default();
        let options = ValidationOptions::config_only();
        assert!(validate_config(&config, &options).is_ok());
    }

    #[test]
    fn test_validate_invalid_config() {
        let config = BbsConfig {
            name: String::new(), // Invalid: empty name
            ..Default::default()
        };
        let options = ValidationOptions::config_only();
        assert!(validate_config(&config, &options).is_err());
    }

    #[test]
    fn test_validate_paths_with_temp_dir() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        let mut config = BbsConfig::default();
        config.paths.data_dir = temp_path.join("data");
        config.paths.users_dir = temp_path.join("users");
        config.paths.messages_dir = temp_path.join("messages");
        config.paths.files_dir = temp_path.join("files");
        config.paths.logs_dir = temp_path.join("logs");
        config.paths.temp_dir = temp_path.join("temp");
        config.paths.doors_dir = temp_path.join("doors");

        // Set ports to high values unlikely to be in use so port validation
        // doesn't interfere with path validation testing
        // Note: Port 0 is rejected by BbsConfig::validate()
        for (i, server) in config.servers.iter_mut().enumerate() {
            server.port = 60000 + i as u16;
        }
        config.web_admin_port = 60100;

        // Should fail with strict (paths don't exist)
        let strict = ValidationOptions::strict();
        assert!(validate_config(&config, &strict).is_err());

        // Should succeed with deployment (allows empty dirs)
        let deployment = ValidationOptions::deployment();
        assert!(validate_config(&config, &deployment).is_ok());

        // Create the directories
        fs::create_dir_all(&config.paths.data_dir).unwrap();
        fs::create_dir_all(&config.paths.users_dir).unwrap();
        fs::create_dir_all(&config.paths.messages_dir).unwrap();
        fs::create_dir_all(&config.paths.files_dir).unwrap();
        fs::create_dir_all(&config.paths.logs_dir).unwrap();
        fs::create_dir_all(&config.paths.temp_dir).unwrap();
        fs::create_dir_all(&config.paths.doors_dir).unwrap();

        // Now strict should work too
        assert!(validate_config(&config, &strict).is_ok());
    }

    #[test]
    fn test_check_path_not_directory() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("not_a_dir");
        fs::write(&file_path, "test").unwrap();

        let result = check_path(&file_path, "test path", false);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ConfigError::InvalidValue(_)));
    }

    #[test]
    fn test_check_path_parent_missing() {
        let result = check_path(Path::new("/nonexistent/parent/child"), "test path", true);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ConfigError::PathNotFound(_)));
    }

    #[test]
    #[cfg_attr(
        not(target_os = "linux"),
        ignore = "Port binding behavior is platform-specific - SO_REUSEADDR differs on Windows/macOS"
    )]
    fn test_is_port_available() {
        // Port 0 should be available (kernel will assign a free port)
        assert!(is_port_available(0));

        // Bind to all IPv4 interfaces (0.0.0.0) to properly block the port
        let listener = std::net::TcpListener::bind(("0.0.0.0", 0)).unwrap();
        let bound_port = listener.local_addr().unwrap().port();

        // That port should not be available
        assert!(!is_port_available(bound_port));

        // After dropping the listener, port should be available again
        drop(listener);
        // Small delay to allow OS to fully release the port
        std::thread::sleep(std::time::Duration::from_millis(50));
        assert!(is_port_available(bound_port));
    }

    #[test]
    #[cfg_attr(
        not(target_os = "linux"),
        ignore = "Port binding behavior is platform-specific - SO_REUSEADDR differs on Windows/macOS"
    )]
    fn test_validate_ports_in_use() {
        // Bind to all interfaces (0.0.0.0) to properly block the port
        let listener = std::net::TcpListener::bind(("0.0.0.0", 0)).unwrap();
        let bound_port = listener.local_addr().unwrap().port();

        let mut config = BbsConfig::default();
        config.servers[0].port = bound_port;

        // Use custom options that only check ports (not paths)
        let options = ValidationOptions {
            check_paths: false,
            check_ports: true,
            allow_empty_dirs: true,
        };
        let result = validate_config(&config, &options);
        assert!(result.is_err());

        // Check the error type
        match result.unwrap_err() {
            ConfigError::PortInUse(_) => {} // Expected error
            other => panic!("Expected PortInUse error, got: {:?}", other),
        }
    }
}
