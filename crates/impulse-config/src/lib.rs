//! Configuration management for Impulse-Next_BBS
//!
//! This crate provides configuration loading, validation, and management for the BBS system.
//! It wraps the `BbsConfig` type from `impulse-types` and adds file-based and environment
//! variable loading capabilities using the `figment` library.
//!
//! # Features
//!
//! - Hierarchical configuration with sensible defaults
//! - TOML file-based configuration
//! - Environment variable overrides (prefixed with `IMPULSE_`)
//! - Configuration validation
//! - Save/load capabilities
//!
//! # Configuration Precedence
//!
//! Configuration values are merged in the following order (lowest to highest priority):
//!
//! 1. **Hardcoded defaults** - `BbsConfig::default()`
//! 2. **TOML file** - Typically `config.toml`
//! 3. **Environment variables** - Prefixed with `IMPULSE_` (e.g., `IMPULSE_NAME`)
//!
//! # Example
//!
//! ```no_run
//! use impulse_config::Config;
//!
//! // Load configuration from file with environment overrides
//! let config = Config::load("config.toml")?;
//! println!("BBS Name: {}", config.inner().name);
//!
//! // Generate a default configuration file
//! Config::generate_default("config.toml")?;
//!
//! // Create config with defaults only
//! let config = Config::with_defaults();
//! # Ok::<(), impulse_config::ConfigError>(())
//! ```
//!
//! # Environment Variables
//!
//! Environment variables use the `IMPULSE_` prefix and support nested fields using underscores:
//!
//! - `IMPULSE_NAME="My BBS"` - Sets BBS name
//! - `IMPULSE_SYSOP="Admin"` - Sets sysop name
//! - `IMPULSE_PATHS_DATA="/data"` - Sets data directory path
//!
//! # Validation
//!
//! All loaded configurations are automatically validated. Validation includes:
//!
//! - Required fields are not empty
//! - Port numbers are valid
//! - Paths exist (if specified)
//! - Security levels are within valid ranges

pub mod error;
pub mod loader;
pub mod validator;

// Re-export commonly used types
pub use error::{ConfigError, Result};
pub use loader::Config;
pub use validator::{ValidationOptions, validate_config};
