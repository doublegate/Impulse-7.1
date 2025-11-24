//! Show command implementation

use anyhow::{Context, Result, bail};
use colored::Colorize;
use impulse_config::Config;
use std::path::PathBuf;

/// Execute the show command
///
/// Loads and displays the effective configuration, including environment overrides.
///
/// # Arguments
/// * `config` - Path to the configuration file to display
/// * `format` - Output format ("toml", "json", or "table")
pub fn execute(config: PathBuf, format: String) -> Result<()> {
    // Load configuration
    let loaded_config = Config::load(&config)
        .with_context(|| format!("Failed to load configuration from {}", config.display()))?;

    match format.to_lowercase().as_str() {
        "toml" => {
            println!(
                "{}\n",
                format!("Configuration from: {}", config.display())
                    .cyan()
                    .bold()
            );

            let toml_string = toml::to_string_pretty(loaded_config.inner())
                .context("Failed to serialize configuration to TOML")?;

            println!("{}", toml_string);
        }
        "json" => {
            println!(
                "{}\n",
                format!("Configuration from: {}", config.display())
                    .cyan()
                    .bold()
            );

            let json_string = serde_json::to_string_pretty(loaded_config.inner())
                .context("Failed to serialize configuration to JSON")?;

            println!("{}", json_string);
        }
        "table" => {
            println!(
                "{}\n",
                format!("Configuration from: {}", config.display())
                    .cyan()
                    .bold()
            );

            // Display in a table-like format
            let cfg = loaded_config.inner();

            println!("{}", "Basic Settings:".bold());
            println!("  {:20} {}", "BBS Name:", cfg.name);
            println!("  {:20} {}", "Sysop:", cfg.sysop);
            println!(
                "  {:20} {}",
                "Location:",
                cfg.location.as_deref().unwrap_or("(not set)")
            );

            println!("\n{}", "Network Servers:".bold());
            if cfg.servers.is_empty() {
                println!("  (no servers configured)");
            } else {
                for (i, server) in cfg.servers.iter().enumerate() {
                    println!("  Server {}:", i + 1);
                    println!("    {:18} {}", "Bind Address:", server.bind_address);
                    println!("    {:18} {}", "Port:", server.port);
                    println!("    {:18} {:?}", "Protocol:", server.protocol);
                    println!("    {:18} {}", "TLS Enabled:", server.enable_tls);
                }
            }

            println!("\n{}", "Paths:".bold());
            println!(
                "  {:20} {}",
                "Data Directory:",
                cfg.paths.data_dir.display()
            );
            println!(
                "  {:20} {}",
                "Users Directory:",
                cfg.paths.users_dir.display()
            );
            println!(
                "  {:20} {}",
                "Messages Directory:",
                cfg.paths.messages_dir.display()
            );
            println!(
                "  {:20} {}",
                "Files Directory:",
                cfg.paths.files_dir.display()
            );
            println!(
                "  {:20} {}",
                "Logs Directory:",
                cfg.paths.logs_dir.display()
            );
            println!(
                "  {:20} {}",
                "Temp Directory:",
                cfg.paths.temp_dir.display()
            );
            println!(
                "  {:20} {}",
                "Doors Directory:",
                cfg.paths.doors_dir.display()
            );

            println!("\n{}", "System Limits:".bold());
            println!("  {:20} {}", "Max Connections:", cfg.limits.max_connections);
            println!(
                "  {:20} {} min",
                "Max Session Time:", cfg.limits.max_time_per_session
            );
            println!(
                "  {:20} {}",
                "Max Daily Downloads:", cfg.limits.max_daily_downloads
            );
            println!(
                "  {:20} {} bytes",
                "Max Upload Size:", cfg.limits.max_upload_size
            );
            println!(
                "  {:20} {} bytes",
                "Max Message Length:", cfg.limits.max_message_length
            );
            println!(
                "  {:20} {}",
                "Min Password Length:", cfg.limits.min_password_length
            );
            println!(
                "  {:20} {}",
                "Max Password Attempts:", cfg.limits.max_password_attempts
            );

            println!("\n{}", "Security:".bold());
            println!(
                "  {:20} {}",
                "Strong Passwords:", cfg.security.require_strong_passwords
            );
            println!(
                "  {:20} {}",
                "Account Lockout:", cfg.security.enable_account_lockout
            );
            println!(
                "  {:20} {} minutes",
                "Lockout Duration:", cfg.security.lockout_duration_minutes
            );
            println!(
                "  {:20} {}",
                "Rate Limiting:", cfg.security.enable_rate_limiting
            );
            println!(
                "  {:20} {}/min",
                "Rate Limit:", cfg.security.rate_limit_per_minute
            );
            println!(
                "  {:20} {}",
                "Audit Logging:", cfg.security.enable_audit_logging
            );
            println!(
                "  {:20} {}",
                "Email Verification:", cfg.security.require_email_verification
            );

            println!("\n{}", "Other Settings:".bold());
            println!("  {:20} {}", "Web Admin Enabled:", cfg.enable_web_admin);
        }
        _ => {
            bail!(
                "Unsupported format: {}. Use 'toml', 'json', or 'table'.",
                format
            );
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_show_toml() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        // Generate a valid config
        let config = Config::with_defaults();
        config.save(path).unwrap();

        // Show should succeed
        let result = execute(path.to_path_buf(), "toml".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_show_json() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        let config = Config::with_defaults();
        config.save(path).unwrap();

        let result = execute(path.to_path_buf(), "json".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_show_table() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        let config = Config::with_defaults();
        config.save(path).unwrap();

        let result = execute(path.to_path_buf(), "table".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_show_nonexistent_file() {
        let result = execute(
            PathBuf::from("/nonexistent/config.toml"),
            "toml".to_string(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_show_invalid_format() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        let config = Config::with_defaults();
        config.save(path).unwrap();

        let result = execute(path.to_path_buf(), "yaml".to_string());
        assert!(result.is_err());
    }
}
