//! Integration tests for impulse-config
//!
//! These tests verify the complete configuration loading pipeline including:
//! - Loading from TOML files
//! - Environment variable overrides
//! - Validation with different options
//! - Save and reload functionality

use impulse_config::{Config, ConfigError, ValidationOptions, validate_config};
use serial_test::serial;
use std::fs;
use tempfile::{NamedTempFile, TempDir};

#[test]
#[serial]
fn test_load_from_toml_file() {
    // Clean up any environment variables that might interfere
    unsafe {
        std::env::remove_var("IMPULSE_NAME");
    }

    let temp_file = NamedTempFile::new().unwrap();
    let path = temp_file.path();

    // Create a simple config file
    let toml_content = r#"
        name = "Test BBS"
        sysop = "Test SysOp"

        [[servers]]
        bind_address = "0.0.0.0"
        protocol = "Telnet"
        port = 2323
        enable_tls = false
    "#;
    fs::write(path, toml_content).unwrap();

    // Load the config
    let config = Config::load(path).unwrap();

    // Verify values
    assert_eq!(config.inner().name, "Test BBS");
    assert_eq!(config.inner().sysop, "Test SysOp");
    assert_eq!(config.inner().servers[0].port, 2323);
}

#[test]
#[serial]
fn test_environment_variable_override() {
    // Clean up any pre-existing environment variable
    unsafe {
        std::env::remove_var("IMPULSE_NAME");
    }

    let temp_file = NamedTempFile::new().unwrap();
    let path = temp_file.path();

    // Create a config file with a name
    let toml_content = r#"
        name = "Original Name"
        sysop = "Test SysOp"

        [[servers]]
        bind_address = "0.0.0.0"
        protocol = "Telnet"
        port = 2323
        enable_tls = false
    "#;
    fs::write(path, toml_content).unwrap();

    // Set environment variable to override the name
    unsafe {
        std::env::set_var("IMPULSE_NAME", "Overridden Name");
    }

    // Load the config (environment should override TOML)
    let config = Config::load(path).unwrap();

    // Verify environment override worked
    assert_eq!(config.inner().name, "Overridden Name");
    assert_eq!(config.inner().sysop, "Test SysOp");

    // Clean up environment immediately
    unsafe {
        std::env::remove_var("IMPULSE_NAME");
    }
}

#[test]
#[serial]
fn test_save_and_reload_config() {
    // Clean up any environment variables that might interfere
    unsafe {
        std::env::remove_var("IMPULSE_NAME");
    }

    let temp_file = NamedTempFile::new().unwrap();
    let path = temp_file.path();

    // Create and save a default config
    let config1 = Config::with_defaults();
    config1.save(path).unwrap();

    // Load it back
    let config2 = Config::load(path).unwrap();

    // Verify all fields match
    assert_eq!(config1.inner().name, config2.inner().name);
    assert_eq!(config1.inner().sysop, config2.inner().sysop);
    assert_eq!(config1.inner().servers.len(), config2.inner().servers.len());
    assert_eq!(
        config1.inner().limits.max_connections,
        config2.inner().limits.max_connections
    );
}

#[test]
#[serial]
fn test_validation_with_invalid_config() {
    // Clean up any environment variables that might interfere
    unsafe {
        std::env::remove_var("IMPULSE_NAME");
    }

    let temp_file = NamedTempFile::new().unwrap();
    let path = temp_file.path();

    // Create a config file with invalid data (empty name)
    let toml_content = r#"
        name = ""
        sysop = "Test SysOp"

        [[servers]]
        bind_address = "0.0.0.0"
        protocol = "Telnet"
        port = 2323
        enable_tls = false
    "#;
    fs::write(path, toml_content).unwrap();

    // Attempt to load should fail validation
    let result = Config::load(path);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        ConfigError::ValidationError(_)
    ));
}

#[test]
fn test_validation_with_filesystem_checks() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();
    let config_path = temp_path.join("config.toml");

    // Create all required directories
    let data_dir = temp_path.join("data");
    let users_dir = temp_path.join("users");
    let messages_dir = temp_path.join("messages");
    let files_dir = temp_path.join("files");
    let logs_dir = temp_path.join("logs");
    let temp_subdir = temp_path.join("temp");
    let doors_dir = temp_path.join("doors");

    fs::create_dir_all(&data_dir).unwrap();
    fs::create_dir_all(&users_dir).unwrap();
    fs::create_dir_all(&messages_dir).unwrap();
    fs::create_dir_all(&files_dir).unwrap();
    fs::create_dir_all(&logs_dir).unwrap();
    fs::create_dir_all(&temp_subdir).unwrap();
    fs::create_dir_all(&doors_dir).unwrap();

    // Create a config file with the created paths
    let toml_content = format!(
        r#"
        name = "Test BBS"
        sysop = "Test SysOp"

        [[servers]]
        bind_address = "0.0.0.0"
        protocol = "Telnet"
        port = 2323
        enable_tls = false

        [paths]
        data_dir = "{}"
        users_dir = "{}"
        messages_dir = "{}"
        files_dir = "{}"
        logs_dir = "{}"
        temp_dir = "{}"
        doors_dir = "{}"
        "#,
        data_dir.display(),
        users_dir.display(),
        messages_dir.display(),
        files_dir.display(),
        logs_dir.display(),
        temp_subdir.display(),
        doors_dir.display()
    );
    fs::write(&config_path, toml_content).unwrap();

    // Load the config
    let config = Config::load(&config_path).unwrap();

    // Validate with strict options (requires all paths to exist)
    let strict = ValidationOptions::strict();
    assert!(validate_config(config.inner(), &strict).is_ok());
}

#[test]
fn test_deployment_validation_allows_missing_dirs() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();
    let config_path = temp_path.join("config.toml");

    // Create a config file with non-existent subdirectories
    let toml_content = format!(
        r#"
        name = "Test BBS"
        sysop = "Test SysOp"

        [[servers]]
        bind_address = "0.0.0.0"
        protocol = "Telnet"
        port = 2323
        enable_tls = false

        [paths]
        data_dir = "{}/data"
        users_dir = "{}/users"
        messages_dir = "{}/messages"
        files_dir = "{}/files"
        logs_dir = "{}/logs"
        temp_dir = "{}/temp"
        doors_dir = "{}/doors"
        "#,
        temp_path.display(),
        temp_path.display(),
        temp_path.display(),
        temp_path.display(),
        temp_path.display(),
        temp_path.display(),
        temp_path.display()
    );
    fs::write(&config_path, toml_content).unwrap();

    // Load the config
    let config = Config::load(&config_path).unwrap();

    // Deployment validation should succeed (allows empty dirs to be created)
    let deployment = ValidationOptions::deployment();
    assert!(validate_config(config.inner(), &deployment).is_ok());

    // Strict validation should fail (dirs don't exist)
    let strict = ValidationOptions::strict();
    assert!(validate_config(config.inner(), &strict).is_err());
}

#[test]
fn test_config_modifications() {
    let mut config = Config::with_defaults();

    // Modify via inner_mut()
    config.inner_mut().name = "Modified BBS".to_string();
    assert_eq!(config.inner().name, "Modified BBS");

    // Validate after modification
    assert!(config.validate().is_ok());

    // Make invalid modification
    config.inner_mut().name = "".to_string();
    assert!(config.validate().is_err());
}

#[test]
#[serial]
fn test_config_round_trip_with_all_fields() {
    // Clean up any environment variables that might interfere
    unsafe {
        std::env::remove_var("IMPULSE_NAME");
    }

    let temp_file = NamedTempFile::new().unwrap();
    let path = temp_file.path();

    // Create a config with non-default values
    let mut config = Config::with_defaults();
    config.inner_mut().name = "Round Trip Test BBS".to_string();
    config.inner_mut().sysop = "Round Trip SysOp".to_string();
    config.inner_mut().limits.max_connections = 42;
    config.inner_mut().limits.max_time_per_session = 1800;

    // Save it
    config.save(path).unwrap();

    // Load it back
    let loaded = Config::load(path).unwrap();

    // Verify all modified fields
    assert_eq!(loaded.inner().name, "Round Trip Test BBS");
    assert_eq!(loaded.inner().sysop, "Round Trip SysOp");
    assert_eq!(loaded.inner().limits.max_connections, 42);
    assert_eq!(loaded.inner().limits.max_time_per_session, 1800);
}

#[test]
#[serial]
fn test_generate_default_creates_valid_file() {
    // Clean up any environment variables that might interfere
    unsafe {
        std::env::remove_var("IMPULSE_NAME");
    }

    let temp_file = NamedTempFile::new().unwrap();
    let path = temp_file.path();

    // Generate a default config file
    Config::generate_default(path).unwrap();

    // Verify the file exists and can be read
    assert!(path.exists());
    let content = fs::read_to_string(path).unwrap();
    assert!(!content.is_empty());

    // Verify it can be loaded
    let config = Config::load(path).unwrap();
    assert_eq!(config.inner().name, "Impulse BBS");
}

#[test]
fn test_missing_file_error() {
    let result = Config::load("/nonexistent/path/config.toml");
    assert!(result.is_err());
    // Should be an IoError (file not found)
    assert!(matches!(result.unwrap_err(), ConfigError::IoError(_)));
}

#[test]
fn test_malformed_toml_error() {
    let temp_file = NamedTempFile::new().unwrap();
    let path = temp_file.path();

    // Write malformed TOML
    fs::write(path, "this is not valid TOML [[[").unwrap();

    let result = Config::load(path);
    assert!(result.is_err());
    // Should be a TOML or Figment parsing error
    assert!(matches!(result.unwrap_err(), ConfigError::FigmentError(_)));
}
