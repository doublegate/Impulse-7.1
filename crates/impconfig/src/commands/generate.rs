//! Generate command implementation

use anyhow::{Context, Result, bail};
use colored::Colorize;
use impulse_config::Config;
use std::path::PathBuf;

/// Execute the generate command
///
/// Creates a new configuration file with default values at the specified path.
///
/// # Arguments
/// * `output` - Path where the configuration file should be created
/// * `format` - Output format ("toml" or "json")
/// * `force` - Overwrite existing file without prompting
pub fn execute(output: PathBuf, format: String, force: bool) -> Result<()> {
    // Check if file already exists
    if output.exists() && !force {
        // Ask for confirmation
        println!(
            "{}",
            format!("File {} already exists.", output.display()).yellow()
        );
        println!("Use --force to overwrite, or specify a different output path.");
        bail!("File already exists");
    }

    // Create default configuration
    let config = Config::with_defaults();

    // Write based on format
    match format.to_lowercase().as_str() {
        "toml" => {
            let toml_string = toml::to_string_pretty(config.inner())
                .context("Failed to serialize configuration to TOML")?;

            std::fs::write(&output, toml_string)
                .with_context(|| format!("Failed to write to {}", output.display()))?;

            println!(
                "{} {}",
                "Generated default configuration:".green().bold(),
                output.display()
            );
        }
        "json" => {
            let json_string = serde_json::to_string_pretty(config.inner())
                .context("Failed to serialize configuration to JSON")?;

            std::fs::write(&output, json_string)
                .with_context(|| format!("Failed to write to {}", output.display()))?;

            println!(
                "{} {}",
                "Generated default configuration:".green().bold(),
                output.display()
            );
        }
        _ => {
            bail!("Unsupported format: {}. Use 'toml' or 'json'.", format);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_generate_toml() {
        let temp_dir = TempDir::new().unwrap();
        let output = temp_dir.path().join("config.toml");

        let result = execute(output.clone(), "toml".to_string(), false);
        assert!(result.is_ok());
        assert!(output.exists());

        // Verify it's valid TOML
        let content = std::fs::read_to_string(&output).unwrap();
        let parsed: impulse_types::config::BbsConfig = toml::from_str(&content).unwrap();
        assert_eq!(parsed.name, "Impulse BBS");
    }

    #[test]
    fn test_generate_json() {
        let temp_dir = TempDir::new().unwrap();
        let output = temp_dir.path().join("config.json");

        let result = execute(output.clone(), "json".to_string(), false);
        assert!(result.is_ok());
        assert!(output.exists());

        // Verify it's valid JSON
        let content = std::fs::read_to_string(&output).unwrap();
        let parsed: impulse_types::config::BbsConfig = serde_json::from_str(&content).unwrap();
        assert_eq!(parsed.name, "Impulse BBS");
    }

    #[test]
    fn test_generate_no_overwrite() {
        let temp_dir = TempDir::new().unwrap();
        let output = temp_dir.path().join("config.toml");

        // First generation should succeed
        execute(output.clone(), "toml".to_string(), false).unwrap();

        // Second generation without force should fail
        let result = execute(output.clone(), "toml".to_string(), false);
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_with_force() {
        let temp_dir = TempDir::new().unwrap();
        let output = temp_dir.path().join("config.toml");

        // First generation
        execute(output.clone(), "toml".to_string(), false).unwrap();

        // Second generation with force should succeed
        let result = execute(output.clone(), "toml".to_string(), true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_invalid_format() {
        let temp_dir = TempDir::new().unwrap();
        let output = temp_dir.path().join("config.yaml");

        let result = execute(output, "yaml".to_string(), false);
        assert!(result.is_err());
    }
}
