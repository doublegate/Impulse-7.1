# Logging Integration Guide

**Version:** 1.0.0
**Date:** 2025-11-24
**Sprint:** 7 - Logging Infrastructure

---

## Overview

The `impulse-logging` crate provides a comprehensive structured logging system built on the `tracing` ecosystem. It features file rotation, log archival, security audit logging, and error reporting capabilities.

This guide demonstrates how to integrate structured logging throughout the Impulse BBS codebase, with real-world examples from the `impulse-auth`, `impulse-user`, and `impulse-config` crates.

---

## Table of Contents

1. [Quick Start](#quick-start)
2. [Core Components](#core-components)
3. [Integration Patterns](#integration-patterns)
4. [Logging Levels](#logging-levels)
5. [Structured Fields](#structured-fields)
6. [Real-World Examples](#real-world-examples)
7. [Best Practices](#best-practices)
8. [Configuration](#configuration)
9. [Testing](#testing)
10. [Performance Considerations](#performance-considerations)

---

## Quick Start

### Adding the Dependency

Add `tracing` to your crate's `Cargo.toml`:

```toml
[dependencies]
tracing = { workspace = true }
```

### Initializing Logging

In your application's main entry point:

```rust
use impulse_logging::{LoggerBuilder, LogLevel, LogOutput, RotationPolicy};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize with file rotation
    LoggerBuilder::new()
        .with_level(LogLevel::Info)
        .with_output(LogOutput::File("logs/bbs.log".into()))
        .with_rotation(RotationPolicy::Daily)
        .with_max_files(30)
        .build()?;

    // Your application code
    tracing::info!("Application started");

    Ok(())
}
```

### Simple Logging

```rust
use tracing::{debug, info, warn, error};

// Simple messages
info!("User session started");
warn!("Configuration file not found, using defaults");
error!("Database connection failed");

// Structured fields
info!(
    user_id = ?user.id(),
    username = %user.username(),
    "User logged in successfully"
);
```

---

## Core Components

### LoggerBuilder

Central configuration for the logging system:

```rust
use impulse_logging::{LoggerBuilder, LogLevel, LogFormat, LogOutput};

LoggerBuilder::new()
    .with_level(LogLevel::Debug)      // Minimum log level
    .with_format(LogFormat::Json)     // Output format
    .with_output(LogOutput::Stdout)   // Output destination
    .build()?;
```

### Log Levels

- **TRACE**: Very fine-grained debug information
- **DEBUG**: Debugging information for routine operations
- **INFO**: General informational messages about normal operations
- **WARN**: Warning messages for expected failures or unusual conditions
- **ERROR**: Error messages for unexpected failures

### Log Outputs

- **Stdout**: Console output (development)
- **Stderr**: Error stream (development)
- **File**: File with rotation support (production)
- **Syslog**: System logging (production)

### Rotation Policies

- **Hourly**: Rotate every hour
- **Daily**: Rotate every day at midnight
- **Weekly**: Rotate every week
- **Size(bytes)**: Rotate when file exceeds size

### Archive Management

Automatic log compression and retention:

```rust
use impulse_logging::{ArchiveManager, ArchivalConfig};

let config = ArchivalConfig {
    max_age_days: 90,           // Keep logs for 90 days
    compression_enabled: true,   // Compress old logs
    archive_dir: "logs/archive".into(),
};

let manager = ArchiveManager::new(config);
manager.archive_old_logs("logs")?;
```

---

## Integration Patterns

### Pattern 1: Operation Success/Failure

Log successful operations at **INFO** level, failures at **WARN** or **ERROR**:

```rust
// Success
tracing::info!(
    user_id = ?user.id(),
    username = %user.username(),
    "User created successfully"
);

// Expected failure (user not found, duplicate entry)
tracing::warn!(
    username = %username,
    "Failed to create user: username already exists"
);

// Unexpected failure (I/O error, database error)
tracing::error!(
    error = %e,
    "Failed to write user file"
);
```

### Pattern 2: Start/End of Operations

Use **DEBUG** for operation start, **INFO** for completion:

```rust
// Start of operation
tracing::debug!(
    file_path = ?path,
    "Loading configuration from file"
);

// ... perform operation ...

// Successful completion
tracing::info!(
    file_path = ?path,
    "Successfully loaded configuration"
);
```

### Pattern 3: Contextual Error Logging

Add context before returning errors:

```rust
match some_operation() {
    Ok(result) => {
        tracing::info!("Operation succeeded");
        Ok(result)
    }
    Err(e) => {
        tracing::error!(
            error = %e,
            context = "database operation",
            "Operation failed"
        );
        Err(e)
    }
}
```

---

## Logging Levels

### When to Use Each Level

#### TRACE
- Very detailed execution flow
- Function entry/exit points
- Loop iterations
- Generally disabled in production

```rust
tracing::trace!(iteration = i, "Processing item");
```

#### DEBUG
- Routine operations that aid debugging
- State transitions
- Configuration loading
- Resource allocation
- Session validation

```rust
tracing::debug!(
    file_path = ?path,
    "Loading users from file"
);
```

#### INFO
- Significant state changes
- User actions (login, logout, create, update, delete)
- Successful operations
- System startup/shutdown
- Configuration changes

```rust
tracing::info!(
    user_id = ?user_id,
    username = %username,
    "User created successfully"
);
```

#### WARN
- Expected failures (user not found, invalid credentials)
- Recoverable errors
- Deprecated feature usage
- Configuration issues
- Rate limit exceeded

```rust
tracing::warn!(
    username = %username,
    "Login failed: invalid credentials"
);
```

#### ERROR
- Unexpected failures
- Unrecoverable errors
- System errors (I/O, database, network)
- Data corruption
- Security violations

```rust
tracing::error!(
    file_path = ?path,
    error = %e,
    "Failed to write configuration file"
);
```

---

## Structured Fields

### Field Format Specifiers

- `%` - Display formatting (implements `Display` trait)
- `?` - Debug formatting (implements `Debug` trait)

### Common Field Types

```rust
// Strings and &str - use %
username = %"alice"

// Numeric types - use %
user_id = %42
port = %8080

// Custom types with Display - use %
token = %session_token
error = %error_msg

// Types with only Debug - use ?
user_id = ?UserId(42)
file_path = ?PathBuf::from("/var/log")
config = ?bbs_config

// Error types - use %
error = %e
```

### Standard Fields

Use consistent field names across the codebase:

| Field Name | Type | Example | Description |
|------------|------|---------|-------------|
| `user_id` | Debug | `user_id = ?UserId(42)` | User identifier |
| `username` | Display | `username = %"alice"` | Username string |
| `token` | Display | `token = %session_token` | Session token |
| `file_path` | Debug | `file_path = ?path` | File path |
| `error` | Display | `error = %e` | Error message |
| `duration_ms` | Display | `duration_ms = %duration.as_millis()` | Operation duration |
| `user_count` | Display | `user_count = %count` | Number of users |
| `port` | Display | `port = %8080` | Network port |

---

## Real-World Examples

### impulse-auth: Authentication Service

#### Login with Structured Logging

```rust
use tracing::{info, warn};
use impulse_types::user::{User, UserId};
use impulse_auth::{AuthError, SessionToken};

pub async fn login(
    &self,
    user: &User,
    password: &str,
    stored_hash: &str,
) -> Result<SessionToken, AuthError> {
    // Verify password
    match self.hasher.verify_password(password, stored_hash) {
        Ok(()) => {
            // Create session
            let token = self.sessions.create_session(user.id()).await;

            tracing::info!(
                user_id = ?user.id(),
                username = %user.username(),
                token = %token,
                "User logged in successfully"
            );

            Ok(token)
        }
        Err(e) => {
            tracing::warn!(
                user_id = ?user.id(),
                username = %user.username(),
                "Login failed: invalid credentials"
            );
            Err(e)
        }
    }
}
```

#### Session Validation

```rust
pub async fn validate_session(&self, token: &SessionToken) -> Result<UserId, AuthError> {
    match self.sessions.get_session(token).await {
        Ok(session) => {
            let user_id = session.user_id();
            match self.sessions.touch_session(token).await {
                Ok(()) => {
                    tracing::debug!(
                        user_id = ?user_id,
                        token = %token,
                        "Session validated successfully"
                    );
                    Ok(user_id)
                }
                Err(e) => {
                    tracing::warn!(
                        user_id = ?user_id,
                        token = %token,
                        error = %e,
                        "Session validation failed: could not update activity"
                    );
                    Err(e)
                }
            }
        }
        Err(e) => {
            tracing::warn!(
                token = %token,
                error = %e,
                "Session validation failed"
            );
            Err(e)
        }
    }
}
```

#### Logout Operation

```rust
pub async fn logout(&self, token: &SessionToken) -> bool {
    let result = self.sessions.remove_session(token).await;

    if result {
        tracing::info!(
            token = %token,
            "User logged out successfully"
        );
    } else {
        tracing::warn!(
            token = %token,
            "Logout failed: session not found"
        );
    }

    result
}
```

### impulse-user: User Management

#### Creating Users

```rust
use tracing::{info, warn};
use impulse_types::user::User;

async fn create_user(&mut self, user: User) -> Result<()> {
    let mut users = self.users.write().unwrap();

    // Check if username already exists
    if users.values().any(|u| u.username() == user.username()) {
        tracing::warn!(
            username = %user.username(),
            "Failed to create user: username already exists"
        );
        return Err(Error::AlreadyExists(format!(
            "User '{}' already exists",
            user.username()
        )));
    }

    let user_id = user.id();
    let username = user.username().to_string();
    users.insert(user.id(), user);

    tracing::info!(
        user_id = ?user_id,
        username = %username,
        "User created successfully"
    );

    Ok(())
}
```

#### Loading Users from File

```rust
use tracing::{debug, info, warn, error};
use std::path::Path;

pub async fn load(&mut self) -> Result<()> {
    use binrw::BinRead;
    use std::fs::File;
    use std::io::BufReader;

    tracing::debug!(
        file_path = ?self.path,
        "Loading users from file"
    );

    let file = File::open(&self.path).map_err(|e| {
        tracing::error!(
            file_path = ?self.path,
            error = %e,
            "Failed to open USER.LST file"
        );
        Error::UserManagement(format!(
            "Failed to open USER.LST at {:?}: {}",
            self.path,
            e
        ))
    })?;

    let mut reader = BufReader::new(file);
    let mut users_map = HashMap::new();

    // Read records until EOF
    loop {
        let pos = reader.stream_position().map_err(|e| {
            Error::UserManagement(format!("Failed to get stream position: {}", e))
        })?;

        match PascalUserRec::read_le(&mut reader) {
            Ok(rec) => {
                match User::from_pascal(&rec) {
                    Ok(user) => {
                        users_map.insert(user.id(), user);
                    }
                    Err(e) => {
                        tracing::warn!(
                            file_path = ?self.path,
                            position = pos,
                            error = %e,
                            "Failed to convert user record, skipping"
                        );
                    }
                }
            }
            Err(e) => {
                // Check if we reached EOF
                if reader.stream_position().map(|p| p == pos).unwrap_or(true) {
                    break; // Normal EOF
                } else {
                    tracing::error!(
                        file_path = ?self.path,
                        position = pos,
                        error = %e,
                        "Failed to read user record"
                    );
                    return Err(Error::UserManagement(format!(
                        "Failed to read user record at position {}: {}",
                        pos, e
                    )));
                }
            }
        }
    }

    let user_count = users_map.len();
    *self.users.write().unwrap() = users_map;

    tracing::info!(
        file_path = ?self.path,
        user_count = user_count,
        "Successfully loaded users from file"
    );

    Ok(())
}
```

### impulse-config: Configuration Management

#### Loading Configuration

```rust
use tracing::{debug, info, warn, error};
use std::path::Path;

pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
    let path_ref = path.as_ref();

    tracing::debug!(
        file_path = ?path_ref,
        "Loading configuration from file"
    );

    // Check if file exists
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

    // Load and parse configuration
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

    // Validate
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
```

#### Saving Configuration

```rust
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
```

---

## Best Practices

### 1. Choose Appropriate Log Levels

```rust
// ❌ DON'T: Log routine operations at INFO
tracing::info!("Checking if user exists");

// ✅ DO: Use DEBUG for routine checks
tracing::debug!(username = %username, "Checking if user exists");

// ❌ DON'T: Log expected failures at ERROR
tracing::error!("User not found");

// ✅ DO: Use WARN for expected failures
tracing::warn!(user_id = ?user_id, "User not found");
```

### 2. Add Contextual Information

```rust
// ❌ DON'T: Generic error messages
tracing::error!("Failed to save");

// ✅ DO: Include context and structured fields
tracing::error!(
    file_path = ?path,
    user_count = users.len(),
    error = %e,
    "Failed to save users to file"
);
```

### 3. Use Consistent Field Names

```rust
// ✅ DO: Use standard field names across codebase
tracing::info!(
    user_id = ?user.id(),      // Always "user_id" for user IDs
    username = %user.username(), // Always "username" for usernames
    "User logged in"
);
```

### 4. Log Before Returning Errors

```rust
// ✅ DO: Log error context before returning
match file.read_to_string(&mut contents) {
    Ok(_) => Ok(contents),
    Err(e) => {
        tracing::error!(
            file_path = ?path,
            error = %e,
            "Failed to read file"
        );
        Err(Error::IoError(e))
    }
}
```

### 5. Avoid Logging in Hot Paths

```rust
// ❌ DON'T: Log in tight loops
for item in items {
    tracing::debug!("Processing item");  // Expensive!
    process(item);
}

// ✅ DO: Log summary after loop
tracing::debug!(
    item_count = items.len(),
    "Processing items"
);
for item in items {
    process(item);
}
tracing::info!(
    item_count = items.len(),
    "Successfully processed all items"
);
```

### 6. Use Spans for Operation Context

```rust
use tracing::{info_span, instrument};

#[instrument(skip(self), fields(user_id = ?user.id()))]
async fn process_user(&self, user: &User) -> Result<()> {
    // All logs within this function automatically include user_id
    tracing::info!("Starting user processing");

    // ... processing ...

    tracing::info!("User processing complete");
    Ok(())
}
```

### 7. Don't Log Sensitive Data

```rust
// ❌ DON'T: Log passwords, tokens, or sensitive data
tracing::info!(password = %password, "User logged in");

// ✅ DO: Log only non-sensitive identifiers
tracing::info!(
    user_id = ?user.id(),
    username = %user.username(),
    "User logged in successfully"
);
```

### 8. Use Tracing Macros, Not println!

```rust
// ❌ DON'T: Use println! for logging
println!("User logged in: {}", username);

// ✅ DO: Use tracing macros
tracing::info!(username = %username, "User logged in");
```

---

## Configuration

### Development Configuration

```rust
use impulse_logging::{LoggerBuilder, LogLevel, LogFormat, LogOutput};

// Console logging with human-readable format
LoggerBuilder::new()
    .with_level(LogLevel::Debug)
    .with_format(LogFormat::Human)
    .with_output(LogOutput::Stdout)
    .build()?;
```

### Production Configuration

```rust
use impulse_logging::{LoggerBuilder, LogLevel, LogFormat, LogOutput, RotationPolicy};

// File logging with JSON format and rotation
LoggerBuilder::new()
    .with_level(LogLevel::Info)
    .with_format(LogFormat::Json)
    .with_output(LogOutput::File("logs/bbs.log".into()))
    .with_rotation(RotationPolicy::Daily)
    .with_max_files(90)
    .build()?;
```

### Environment-Based Configuration

```rust
use std::env;

let log_level = env::var("LOG_LEVEL")
    .unwrap_or_else(|_| "info".to_string());

let log_format = if env::var("PRODUCTION").is_ok() {
    LogFormat::Json
} else {
    LogFormat::Human
};

LoggerBuilder::new()
    .with_level(log_level.parse()?)
    .with_format(log_format)
    .build()?;
```

### Multiple Outputs

```rust
// Combine file and console logging
LoggerBuilder::new()
    .with_level(LogLevel::Info)
    .with_output(LogOutput::File("logs/bbs.log".into()))
    .with_rotation(RotationPolicy::Daily)
    .build()?;

// Errors also go to stderr (configured in subscriber)
```

---

## Testing

### Testing with Logging

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tracing_subscriber;

    #[test]
    fn test_with_logging() {
        // Initialize test logging
        let _ = tracing_subscriber::fmt()
            .with_test_writer()
            .try_init();

        // Your test code
        tracing::info!("Test starting");

        let result = some_function();

        tracing::info!("Test complete");
        assert!(result.is_ok());
    }
}
```

### Capturing Logs in Tests

```rust
use tracing_subscriber::{layer::SubscriberExt, Layer};
use std::sync::{Arc, Mutex};

#[test]
fn test_log_capture() {
    let logs = Arc::new(Mutex::new(Vec::new()));
    let logs_clone = Arc::clone(&logs);

    let layer = tracing_subscriber::fmt::layer()
        .with_writer(move || {
            // Capture logs
            logs_clone.lock().unwrap()
        });

    let subscriber = tracing_subscriber::registry().with(layer);

    tracing::subscriber::with_default(subscriber, || {
        tracing::info!("Test log message");
    });

    let captured = logs.lock().unwrap();
    assert!(captured.len() > 0);
}
```

### Integration Testing

```rust
#[tokio::test]
async fn test_user_creation_with_logging() {
    // Initialize test logging
    impulse_logging::init_console_logging()?;

    let mut manager = InMemoryUserManager::new();
    let user = User::new("testuser");

    // This will generate logs
    let result = manager.create_user(user).await;

    assert!(result.is_ok());
}
```

---

## Performance Considerations

### Log Level Filtering

The tracing system uses compile-time and runtime filtering to minimize overhead:

```rust
// Only evaluated if DEBUG level is enabled
tracing::debug!(
    expensive_calculation = expensive_function(),
    "Debug information"
);
```

### Async Logging

For high-throughput applications, use async logging:

```rust
use tracing_subscriber::fmt::writer::MakeWriterExt;

let (non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());

tracing_subscriber::fmt()
    .with_writer(non_blocking)
    .init();
```

### Sampling

For very high-volume logs, consider sampling:

```rust
use rand::Rng;

// Log 1% of requests
if rand::thread_rng().gen_ratio(1, 100) {
    tracing::debug!(request_id = %id, "Request details");
}
```

### Benchmarking

The impulse-logging crate includes benchmarks:

```bash
cargo bench --package impulse-logging
```

Results show:
- Structured logging: ~500ns per log call
- JSON formatting: ~1-2µs per log
- File rotation: ~10-50ms per rotation

---

## Migration Checklist

When integrating logging into an existing crate:

- [ ] Add `tracing = { workspace = true }` to Cargo.toml
- [ ] Identify critical operations (create, update, delete, load, save)
- [ ] Add INFO logs for successful state changes
- [ ] Add WARN logs for expected failures
- [ ] Add ERROR logs for unexpected failures
- [ ] Add DEBUG logs for routine operations
- [ ] Use consistent structured field names
- [ ] Use `?` format for Debug types, `%` for Display types
- [ ] Test that all logs appear correctly
- [ ] Verify no performance regressions
- [ ] Update documentation with logging examples

---

## Troubleshooting

### Logs Not Appearing

1. Check log level configuration:
```rust
// Ensure level is set appropriately
LoggerBuilder::new()
    .with_level(LogLevel::Debug)  // Not LogLevel::Error
    .build()?;
```

2. Verify logger initialization:
```rust
// Ensure logger is initialized before any logging
impulse_logging::init_console_logging()?;
tracing::info!("This will appear");
```

### Format Specifier Errors

```rust
// ❌ ERROR: UserId doesn't implement Display
tracing::info!(user_id = %user.id(), "User logged in");

// ✅ CORRECT: Use Debug formatting
tracing::info!(user_id = ?user.id(), "User logged in");
```

### Missing Structured Fields

```rust
// ❌ DON'T: Interpolate into message string
tracing::info!("User {} logged in", username);

// ✅ DO: Use structured fields
tracing::info!(username = %username, "User logged in");
```

---

## Additional Resources

- [Tracing Documentation](https://docs.rs/tracing)
- [Tracing Subscriber](https://docs.rs/tracing-subscriber)
- [impulse-logging API Docs](../crates/impulse-logging/README.md)
- [Sprint 7 Completion Report](../to-dos/phase-1-foundation/sprint-07-logging-infrastructure.md)

---

## Summary

This guide has demonstrated:

1. **Quick Start**: How to initialize and use the logging system
2. **Components**: Core logging components and their configuration
3. **Patterns**: Common integration patterns for different scenarios
4. **Levels**: When to use each log level
5. **Fields**: How to structure log fields for consistency
6. **Examples**: Real-world examples from production code
7. **Practices**: Best practices for effective logging
8. **Configuration**: Development and production configurations
9. **Testing**: How to test code with logging
10. **Performance**: Performance considerations and optimizations

By following these patterns and examples, you can integrate structured logging throughout the Impulse BBS codebase, creating a comprehensive audit trail and debugging capability.

---

**Document Status:** Complete
**Review Date:** 2025-11-24
**Next Review:** Sprint 8 (Testing Framework)
