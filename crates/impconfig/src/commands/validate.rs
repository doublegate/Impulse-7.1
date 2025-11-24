//! Validate command implementation

use anyhow::Result;
use colored::Colorize;
use impulse_config::{Config, ValidationOptions};
use std::path::PathBuf;

/// Execute the validate command
///
/// Loads and validates a configuration file, checking for errors and printing results.
///
/// # Arguments
/// * `config` - Path to the configuration file to validate
/// * `mode` - Validation mode ("config-only", "strict", or "deployment")
pub fn execute(config: PathBuf, mode: String) -> Result<()> {
    // Parse validation mode
    let validation_mode = match mode.to_lowercase().as_str() {
        "config-only" => ValidationOptions::config_only(),
        "strict" => ValidationOptions::strict(),
        "deployment" => ValidationOptions::deployment(),
        _ => {
            anyhow::bail!(
                "Unknown validation mode: {}. Valid modes: config-only, strict, deployment",
                mode
            );
        }
    };

    println!(
        "{} {} (mode: {})",
        "Validating configuration:".cyan().bold(),
        config.display(),
        mode
    );

    // Try to load the configuration
    match Config::load(&config) {
        Ok(loaded_config) => {
            // Configuration loaded successfully, now validate with specified mode
            match impulse_config::validate_config(loaded_config.inner(), &validation_mode) {
                Ok(()) => {
                    println!("{}", "✓ Configuration is valid".green().bold());
                    println!("\n{}", "Configuration summary:".bold());
                    println!("  BBS Name: {}", loaded_config.inner().name);
                    println!("  Sysop: {}", loaded_config.inner().sysop);
                    if let Some(first_server) = loaded_config.inner().servers.first() {
                        println!("  Server Port: {}", first_server.port);
                        println!("  Server Protocol: {:?}", first_server.protocol);
                    }
                    println!(
                        "  Data Directory: {}",
                        loaded_config.inner().paths.data_dir.display()
                    );
                    println!(
                        "  Strong Passwords: {}",
                        loaded_config.inner().security.require_strong_passwords
                    );

                    Ok(())
                }
                Err(e) => {
                    println!("{}", "✗ Validation failed".red().bold());
                    println!("\n{}", format!("Error: {}", e).red());
                    anyhow::bail!("Validation failed: {}", e);
                }
            }
        }
        Err(e) => {
            println!("{}", "✗ Failed to load configuration".red().bold());
            println!("\n{}", format!("Error: {}", e).red());

            // Try to provide helpful context
            if !config.exists() {
                println!("\n{}", "The configuration file does not exist.".yellow());
                println!(
                    "Try running: {} {}",
                    "impconfig generate --output".cyan(),
                    config.display()
                );
            }

            anyhow::bail!("Failed to load configuration: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_validate_valid_config() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        // Generate a valid config
        let config = Config::with_defaults();
        config.save(path).unwrap();

        // Validate should succeed
        let result = execute(path.to_path_buf(), "config-only".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_nonexistent_file() {
        let result = execute(
            PathBuf::from("/nonexistent/config.toml"),
            "config-only".to_string(),
        );

        // Should fail with an error
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_invalid_toml() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        // Write invalid TOML
        std::fs::write(path, "this is not valid toml {[}").unwrap();

        let result = execute(path.to_path_buf(), "config-only".to_string());
        // Should fail due to TOML parsing error
        assert!(result.is_err());
    }
}
