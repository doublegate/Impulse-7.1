# impulse-config

Configuration management for the Impulse-Next_BBS system.

## Overview

`impulse-config` provides a robust configuration loading, validation, and management system for the BBS software. It wraps the `BbsConfig` type from `impulse-types` and adds hierarchical configuration loading with support for TOML files and environment variable overrides.

## Features

- **Hierarchical Configuration**: Merge values from multiple sources with clear precedence
- **TOML Support**: Human-readable configuration files
- **Environment Variables**: Override any config value via `IMPULSE_*` environment variables
- **Comprehensive Validation**: Multiple validation modes for different deployment scenarios
- **Filesystem Checks**: Optional validation of directory paths and port availability
- **Save/Load**: Round-trip configuration to and from TOML files
- **Type-Safe**: Full type safety with Rust's type system and `serde`

## Configuration Precedence

Configuration values are merged in the following order (lowest to highest priority):

1. **Hardcoded defaults** - `BbsConfig::default()`
2. **TOML configuration file** - Typically `config.toml`
3. **Environment variables** - Prefixed with `IMPULSE_` (e.g., `IMPULSE_NAME`)

Higher priority sources override lower priority sources.

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
impulse-config = { path = "../impulse-config" }
```

### Basic Usage

```rust
use impulse_config::Config;

// Load configuration from file
let config = Config::load("config.toml")?;
println!("BBS Name: {}", config.inner().name);

// Access configuration values
let name = &config.inner().name;
let sysop = &config.inner().sysop;
let port = config.inner().servers[0].port;
```

### Generate Default Configuration

```rust
use impulse_config::Config;

// Generate a default config.toml file
Config::generate_default("config.toml")?;
```

### Working with Configuration

```rust
use impulse_config::Config;

// Create with defaults
let mut config = Config::with_defaults();

// Modify values
config.inner_mut().name = "My BBS".to_string();
config.inner_mut().sysop = "SysOp Name".to_string();

// Validate changes
config.validate()?;

// Save to file
config.save("config.toml")?;
```

## Environment Variable Overrides

Any configuration value can be overridden using environment variables with the `IMPULSE_` prefix. Nested fields use underscores:

```bash
# Override BBS name
export IMPULSE_NAME="My BBS"

# Override SysOp name
export IMPULSE_SYSOP="John Doe"

# Override nested path values
export IMPULSE_PATHS_DATA_DIR="/var/lib/bbs/data"
export IMPULSE_PATHS_LOGS_DIR="/var/log/bbs"

# Override server configuration
export IMPULSE_SERVERS_0_PORT=2324
export IMPULSE_SERVERS_0_BIND_ADDRESS="127.0.0.1"
```

Environment variables take precedence over TOML file values but not over command-line arguments (when implemented).

## Validation

The crate provides three validation modes via `ValidationOptions`:

### Config-Only Validation

Validates configuration values only, without filesystem or network checks:

```rust
use impulse_config::{Config, ValidationOptions, validate_config};

let config = Config::load("config.toml")?;
let options = ValidationOptions::config_only();
validate_config(config.inner(), &options)?;
```

**Use case**: Quick validation during development or when directories don't exist yet.

### Strict Validation

Validates configuration values, requires all paths to exist, and checks port availability:

```rust
use impulse_config::{Config, ValidationOptions, validate_config};

let config = Config::load("config.toml")?;
let options = ValidationOptions::strict();
validate_config(config.inner(), &options)?;
```

**Use case**: Production deployment when all infrastructure is set up.

### Deployment Validation

Validates configuration values, checks filesystem paths exist or can be created, but skips port checks:

```rust
use impulse_config::{Config, ValidationOptions, validate_config};

let config = Config::load("config.toml")?;
let options = ValidationOptions::deployment();
validate_config(config.inner(), &options)?;
```

**Use case**: Initial deployment where directories will be created but services aren't running yet.

### Custom Validation

Create custom validation options:

```rust
use impulse_config::ValidationOptions;

let custom = ValidationOptions {
    check_paths: true,      // Validate filesystem paths
    check_ports: false,     // Skip port availability checks
    allow_empty_dirs: true, // Allow directories to be created
};
```

## Example Configuration File

Here's an example `config.toml`:

```toml
# Basic BBS Information
name = "Impulse BBS"
sysop = "SysOp Name"
sysop_email = "sysop@example.com"
location = "San Francisco, CA"
description = "A modern BBS system built with Rust"

# Server Configuration
[[servers]]
bind_address = "0.0.0.0"
port = 2323
protocol = "Telnet"
enable_tls = false

[[servers]]
bind_address = "0.0.0.0"
port = 2222
protocol = "Ssh"
enable_tls = false

# Web Admin Panel
web_admin_port = 8080
web_admin_enabled = true

# System Limits
[limits]
max_connections = 100
max_time_per_session = 7200
max_daily_downloads = 50
max_upload_size = 10485760
max_message_length = 4096

# Filesystem Paths
[paths]
data_dir = "./data"
users_dir = "./data/users"
messages_dir = "./data/messages"
files_dir = "./data/files"
logs_dir = "./logs"
temp_dir = "./temp"
doors_dir = "./doors"

# Security Settings
[security]
min_password_length = 8
max_login_attempts = 3
session_timeout_minutes = 30
require_email_verification = true
allow_new_users = true

# Display Settings
[display]
ansi_enabled = true
avatar_enabled = true
rip_enabled = false
```

## Error Handling

The crate provides detailed error types via `ConfigError`:

```rust
use impulse_config::{Config, ConfigError};

match Config::load("config.toml") {
    Ok(config) => {
        // Use configuration
    }
    Err(ConfigError::IoError(e)) => {
        eprintln!("File error: {}", e);
    }
    Err(ConfigError::FigmentError(e)) => {
        eprintln!("Configuration parsing error: {}", e);
    }
    Err(ConfigError::ValidationError(msg)) => {
        eprintln!("Validation failed: {}", msg);
    }
    Err(ConfigError::PathNotFound(path)) => {
        eprintln!("Required path not found: {}", path.display());
    }
    Err(ConfigError::PortInUse(port)) => {
        eprintln!("Port {} is already in use", port);
    }
    Err(e) => {
        eprintln!("Configuration error: {}", e);
    }
}
```

## Testing

The crate includes comprehensive tests:

- **Unit tests**: Test individual components (16 tests)
- **Integration tests**: Test complete loading pipeline (11 tests)
- **Doc tests**: Verify example code (10 tests)

Run all tests:

```bash
cargo test --package impulse-config --all-features
```

Run specific test suites:

```bash
# Unit tests only
cargo test --package impulse-config --lib

# Integration tests only
cargo test --package impulse-config --test integration_tests

# Doc tests only
cargo test --package impulse-config --doc
```

## Dependencies

- **impulse-types**: Core BBS type definitions
- **figment**: Hierarchical configuration loading
- **serde**: Serialization framework
- **toml**: TOML format support
- **thiserror**: Error type derivation

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](../../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](../../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

See [CONTRIBUTING.md](../../CONTRIBUTING.md) for guidelines.
