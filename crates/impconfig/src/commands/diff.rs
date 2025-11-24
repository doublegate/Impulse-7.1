//! Diff command implementation

use anyhow::{Context, Result};
use colored::Colorize;
use impulse_config::Config;
use std::path::PathBuf;

/// Execute the diff command
///
/// Compares two configuration files and displays the differences.
///
/// # Arguments
/// * `config1` - Path to the first configuration file
/// * `config2` - Path to the second configuration file
pub fn execute(config1: PathBuf, config2: PathBuf) -> Result<()> {
    // Load both configurations
    let cfg1 = Config::load(&config1)
        .with_context(|| format!("Failed to load configuration from {}", config1.display()))?;

    let cfg2 = Config::load(&config2)
        .with_context(|| format!("Failed to load configuration from {}", config2.display()))?;

    println!(
        "{}\n",
        format!(
            "Comparing configurations:\n  {} (A)\n  {} (B)",
            config1.display(),
            config2.display()
        )
        .cyan()
        .bold()
    );

    let c1 = cfg1.inner();
    let c2 = cfg2.inner();

    let mut differences_found = false;

    // Compare basic settings
    println!("{}", "Basic Settings:".bold());
    differences_found |= compare_field("BBS Name", &c1.name, &c2.name);
    differences_found |= compare_field("Sysop", &c1.sysop, &c2.sysop);
    differences_found |= compare_field(
        "Location",
        c1.location.as_deref().unwrap_or("(none)"),
        c2.location.as_deref().unwrap_or("(none)"),
    );

    // Compare network servers
    println!("\n{}", "Network Servers:".bold());
    differences_found |= compare_field(
        "Server Count",
        &c1.servers.len().to_string(),
        &c2.servers.len().to_string(),
    );

    // Compare first server if both have at least one
    if let (Some(s1), Some(s2)) = (c1.servers.first(), c2.servers.first()) {
        differences_found |=
            compare_field("Server 1 Port", &s1.port.to_string(), &s2.port.to_string());
        differences_found |= compare_field(
            "Server 1 Protocol",
            &format!("{:?}", s1.protocol),
            &format!("{:?}", s2.protocol),
        );
        differences_found |= compare_field(
            "Server 1 TLS",
            &s1.enable_tls.to_string(),
            &s2.enable_tls.to_string(),
        );
    }

    // Compare paths
    println!("\n{}", "Paths:".bold());
    differences_found |= compare_field(
        "Data Directory",
        &c1.paths.data_dir.display().to_string(),
        &c2.paths.data_dir.display().to_string(),
    );
    differences_found |= compare_field(
        "Users Directory",
        &c1.paths.users_dir.display().to_string(),
        &c2.paths.users_dir.display().to_string(),
    );
    differences_found |= compare_field(
        "Messages Directory",
        &c1.paths.messages_dir.display().to_string(),
        &c2.paths.messages_dir.display().to_string(),
    );
    differences_found |= compare_field(
        "Files Directory",
        &c1.paths.files_dir.display().to_string(),
        &c2.paths.files_dir.display().to_string(),
    );
    differences_found |= compare_field(
        "Logs Directory",
        &c1.paths.logs_dir.display().to_string(),
        &c2.paths.logs_dir.display().to_string(),
    );
    differences_found |= compare_field(
        "Temp Directory",
        &c1.paths.temp_dir.display().to_string(),
        &c2.paths.temp_dir.display().to_string(),
    );
    differences_found |= compare_field(
        "Doors Directory",
        &c1.paths.doors_dir.display().to_string(),
        &c2.paths.doors_dir.display().to_string(),
    );

    // Compare system limits
    println!("\n{}", "System Limits:".bold());
    differences_found |= compare_field(
        "Max Connections",
        &c1.limits.max_connections.to_string(),
        &c2.limits.max_connections.to_string(),
    );
    differences_found |= compare_field(
        "Max Session Time (min)",
        &c1.limits.max_time_per_session.to_string(),
        &c2.limits.max_time_per_session.to_string(),
    );
    differences_found |= compare_field(
        "Max Daily Downloads",
        &c1.limits.max_daily_downloads.to_string(),
        &c2.limits.max_daily_downloads.to_string(),
    );
    differences_found |= compare_field(
        "Max Upload Size (bytes)",
        &c1.limits.max_upload_size.to_string(),
        &c2.limits.max_upload_size.to_string(),
    );
    differences_found |= compare_field(
        "Max Message Length",
        &c1.limits.max_message_length.to_string(),
        &c2.limits.max_message_length.to_string(),
    );
    differences_found |= compare_field(
        "Min Password Length",
        &c1.limits.min_password_length.to_string(),
        &c2.limits.min_password_length.to_string(),
    );
    differences_found |= compare_field(
        "Max Password Attempts",
        &c1.limits.max_password_attempts.to_string(),
        &c2.limits.max_password_attempts.to_string(),
    );

    // Compare security settings
    println!("\n{}", "Security Settings:".bold());
    differences_found |= compare_field(
        "Strong Passwords",
        &c1.security.require_strong_passwords.to_string(),
        &c2.security.require_strong_passwords.to_string(),
    );
    differences_found |= compare_field(
        "Account Lockout",
        &c1.security.enable_account_lockout.to_string(),
        &c2.security.enable_account_lockout.to_string(),
    );
    differences_found |= compare_field(
        "Lockout Duration (min)",
        &c1.security.lockout_duration_minutes.to_string(),
        &c2.security.lockout_duration_minutes.to_string(),
    );
    differences_found |= compare_field(
        "Rate Limiting",
        &c1.security.enable_rate_limiting.to_string(),
        &c2.security.enable_rate_limiting.to_string(),
    );
    differences_found |= compare_field(
        "Rate Limit (per min)",
        &c1.security.rate_limit_per_minute.to_string(),
        &c2.security.rate_limit_per_minute.to_string(),
    );
    differences_found |= compare_field(
        "Audit Logging",
        &c1.security.enable_audit_logging.to_string(),
        &c2.security.enable_audit_logging.to_string(),
    );
    differences_found |= compare_field(
        "Email Verification",
        &c1.security.require_email_verification.to_string(),
        &c2.security.require_email_verification.to_string(),
    );

    // Compare other settings
    println!("\n{}", "Other Settings:".bold());
    differences_found |= compare_field(
        "Web Admin Enabled",
        &c1.enable_web_admin.to_string(),
        &c2.enable_web_admin.to_string(),
    );

    if !differences_found {
        println!("\n{}", "✓ Configurations are identical".green().bold());
    } else {
        let legend = format!(
            "Legend: {} A only | {} B only | {} Different",
            "[-]".red(),
            "[+]".green(),
            "[≠]".yellow()
        );
        println!("\n{}", legend);
    }

    Ok(())
}

/// Compare two field values and print differences
///
/// Returns `true` if the fields differ, `false` if they are the same.
fn compare_field(name: &str, value1: &str, value2: &str) -> bool {
    if value1 != value2 {
        println!(
            "  {} {:20} {} → {}",
            "[≠]".yellow().bold(),
            name,
            value1.red(),
            value2.green()
        );
        true
    } else {
        println!("  {} {:20} {}", "[ ]".dimmed(), name, value1.dimmed());
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_diff_identical_configs() {
        let temp_file1 = NamedTempFile::new().unwrap();
        let temp_file2 = NamedTempFile::new().unwrap();

        // Generate identical configs
        let config = Config::with_defaults();
        config.save(temp_file1.path()).unwrap();
        config.save(temp_file2.path()).unwrap();

        let result = execute(
            temp_file1.path().to_path_buf(),
            temp_file2.path().to_path_buf(),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_diff_different_configs() {
        let temp_file1 = NamedTempFile::new().unwrap();
        let temp_file2 = NamedTempFile::new().unwrap();

        // Generate different configs
        let config1 = Config::with_defaults();
        config1.save(temp_file1.path()).unwrap();

        let mut config2 = Config::with_defaults();
        config2.inner_mut().name = "Different BBS".to_string();
        config2.save(temp_file2.path()).unwrap();

        let result = execute(
            temp_file1.path().to_path_buf(),
            temp_file2.path().to_path_buf(),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_diff_nonexistent_file1() {
        let temp_file2 = NamedTempFile::new().unwrap();
        let config = Config::with_defaults();
        config.save(temp_file2.path()).unwrap();

        let result = execute(
            PathBuf::from("/nonexistent/config.toml"),
            temp_file2.path().to_path_buf(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_diff_nonexistent_file2() {
        let temp_file1 = NamedTempFile::new().unwrap();
        let config = Config::with_defaults();
        config.save(temp_file1.path()).unwrap();

        let result = execute(
            temp_file1.path().to_path_buf(),
            PathBuf::from("/nonexistent/config.toml"),
        );
        assert!(result.is_err());
    }
}
