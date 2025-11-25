//! Configuration loading and management

use crate::error::{ConfigError, Result};
use figment::{
    Figment,
    providers::{Env, Format, Serialized, Toml},
};
use impulse_types::config::BbsConfig;
use std::path::Path;

/// Configuration manager wrapping BbsConfig with loading/saving capabilities
#[derive(Debug, Clone)]
pub struct Config {
    inner: BbsConfig,
}

impl Config {
    /// Load configuration from a TOML file with environment variable overrides
    ///
    /// # Configuration Precedence (lowest to highest)
    /// 1. Hardcoded defaults (`BbsConfig::default()`)
    /// 2. TOML configuration file (typically `config.toml`)
    /// 3. Environment variables (prefixed with `IMPULSE_`, e.g., `IMPULSE_NAME`)
    ///
    /// # Example
    /// ```no_run
    /// use impulse_config::Config;
    ///
    /// let config = Config::load("config.toml")?;
    /// # Ok::<(), impulse_config::ConfigError>(())
    /// ```
    ///
    /// # Errors
    /// Returns `ConfigError` if:
    /// - File cannot be read
    /// - TOML parsing fails
    /// - Configuration validation fails
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_ref = path.as_ref();

        tracing::debug!(
            file_path = ?path_ref,
            "Loading configuration from file"
        );

        // Check if file exists first
        if !path_ref.exists() {
            tracing::error!(
                file_path = ?path_ref,
                "Configuration file not found"
            );
            return Err(ConfigError::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Configuration file not found: {}", path_ref.display()),
            )));
        }

        let config: BbsConfig = Figment::new()
            .merge(Serialized::defaults(BbsConfig::default()))
            .merge(Toml::file(path_ref))
            .merge(Env::prefixed("IMPULSE_").split("_"))
            .extract()
            .map_err(|e| {
                tracing::error!(
                    file_path = ?path_ref,
                    error = %e,
                    "Failed to parse configuration"
                );
                ConfigError::from(e)
            })?;

        // Validate the configuration
        config.validate().map_err(|e| {
            tracing::warn!(
                file_path = ?path_ref,
                error = %e,
                "Configuration validation failed"
            );
            ConfigError::ValidationError(format!("Configuration validation failed: {}", e))
        })?;

        tracing::info!(
            file_path = ?path_ref,
            "Successfully loaded configuration"
        );

        Ok(Self { inner: config })
    }

    /// Load configuration with defaults only (no file or environment variables)
    ///
    /// Useful for testing or generating a default configuration file.
    ///
    /// # Example
    /// ```
    /// use impulse_config::Config;
    ///
    /// let config = Config::with_defaults();
    /// ```
    pub fn with_defaults() -> Self {
        Self {
            inner: BbsConfig::default(),
        }
    }

    /// Save configuration to a TOML file
    ///
    /// # Example
    /// ```no_run
    /// use impulse_config::Config;
    ///
    /// let config = Config::with_defaults();
    /// config.save("config.toml")?;
    /// # Ok::<(), impulse_config::ConfigError>(())
    /// ```
    ///
    /// # Errors
    /// Returns `ConfigError` if:
    /// - TOML serialization fails
    /// - File cannot be written
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path_ref = path.as_ref();

        tracing::debug!(
            file_path = ?path_ref,
            "Saving configuration to file"
        );

        let toml_string = toml::to_string_pretty(&self.inner).map_err(|e| {
            tracing::error!(
                file_path = ?path_ref,
                error = %e,
                "Failed to serialize configuration to TOML"
            );
            ConfigError::from(e)
        })?;

        std::fs::write(path_ref, toml_string).map_err(|e| {
            tracing::error!(
                file_path = ?path_ref,
                error = %e,
                "Failed to write configuration file"
            );
            ConfigError::SaveError(format!(
                "Failed to write config to {}: {}",
                path_ref.display(),
                e
            ))
        })?;

        tracing::info!(
            file_path = ?path_ref,
            "Successfully saved configuration"
        );

        Ok(())
    }

    /// Generate a default configuration file
    ///
    /// Creates a new configuration file with default values at the specified path.
    /// If the file already exists, it will be overwritten.
    ///
    /// # Example
    /// ```no_run
    /// use impulse_config::Config;
    ///
    /// Config::generate_default("config.toml")?;
    /// # Ok::<(), impulse_config::ConfigError>(())
    /// ```
    ///
    /// # Errors
    /// Returns `ConfigError` if file cannot be written
    pub fn generate_default<P: AsRef<Path>>(path: P) -> Result<()> {
        tracing::info!(
            file_path = ?path.as_ref(),
            "Generating default configuration file"
        );
        let config = Self::with_defaults();
        config.save(path)
    }

    /// Get a reference to the inner BbsConfig
    ///
    /// # Example
    /// ```
    /// use impulse_config::Config;
    ///
    /// let config = Config::with_defaults();
    /// let bbs_config = config.inner();
    /// println!("BBS Name: {}", bbs_config.name);
    /// ```
    pub fn inner(&self) -> &BbsConfig {
        &self.inner
    }

    /// Get a mutable reference to the inner BbsConfig
    ///
    /// Note: After modifying the config, you should call `validate()` to ensure
    /// the configuration is still valid.
    ///
    /// # Example
    /// ```
    /// use impulse_config::Config;
    ///
    /// let mut config = Config::with_defaults();
    /// config.inner_mut().name = "My BBS".to_string();
    /// ```
    pub fn inner_mut(&mut self) -> &mut BbsConfig {
        &mut self.inner
    }

    /// Consume the Config and return the inner BbsConfig
    ///
    /// # Example
    /// ```
    /// use impulse_config::Config;
    ///
    /// let config = Config::with_defaults();
    /// let bbs_config = config.into_inner();
    /// ```
    pub fn into_inner(self) -> BbsConfig {
        self.inner
    }

    /// Validate the current configuration
    ///
    /// This calls the validation logic on the inner BbsConfig.
    ///
    /// # Example
    /// ```
    /// use impulse_config::Config;
    ///
    /// let mut config = Config::with_defaults();
    /// config.inner_mut().name = "".to_string(); // Invalid: empty name
    /// assert!(config.validate().is_err());
    /// ```
    ///
    /// # Errors
    /// Returns `ConfigError` if validation fails
    pub fn validate(&self) -> Result<()> {
        tracing::debug!("Validating configuration");

        self.inner.validate().map_err(|e| {
            tracing::warn!(
                error = %e,
                "Configuration validation failed"
            );
            ConfigError::ValidationError(format!("Configuration validation failed: {}", e))
        })?;

        tracing::debug!("Configuration validation successful");
        Ok(())
    }
}

impl AsRef<BbsConfig> for Config {
    fn as_ref(&self) -> &BbsConfig {
        &self.inner
    }
}

impl AsMut<BbsConfig> for Config {
    fn as_mut(&mut self) -> &mut BbsConfig {
        &mut self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_with_defaults() {
        let config = Config::with_defaults();
        assert_eq!(config.inner().name, "Impulse BBS");
    }

    #[test]
    fn test_save_and_load() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        // Create and save default config
        let config1 = Config::with_defaults();
        config1.save(path).unwrap();

        // Load it back
        let config2 = Config::load(path).unwrap();

        // Should match
        assert_eq!(config1.inner().name, config2.inner().name);
        assert_eq!(config1.inner().sysop, config2.inner().sysop);
    }

    #[test]
    fn test_generate_default() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        Config::generate_default(path).unwrap();

        // Should be able to load it
        let config = Config::load(path).unwrap();
        assert_eq!(config.inner().name, "Impulse BBS");
    }

    #[test]
    fn test_validate() {
        let config = Config::with_defaults();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_invalid() {
        let mut config = Config::with_defaults();
        config.inner_mut().name = "".to_string(); // Invalid: empty name
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_inner_accessors() {
        let mut config = Config::with_defaults();

        // Test inner()
        assert_eq!(config.inner().name, "Impulse BBS");

        // Test inner_mut()
        config.inner_mut().name = "Test BBS".to_string();
        assert_eq!(config.inner().name, "Test BBS");

        // Test into_inner()
        let bbs_config = config.into_inner();
        assert_eq!(bbs_config.name, "Test BBS");
    }

    #[test]
    fn test_as_ref() {
        let config = Config::with_defaults();
        let bbs_ref: &BbsConfig = config.as_ref();
        assert_eq!(bbs_ref.name, "Impulse BBS");
    }

    #[test]
    fn test_as_mut() {
        let mut config = Config::with_defaults();
        let bbs_mut: &mut BbsConfig = config.as_mut();
        bbs_mut.name = "Modified BBS".to_string();
        assert_eq!(config.inner().name, "Modified BBS");
    }
}
