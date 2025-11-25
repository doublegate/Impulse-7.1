# Sprint 05: RECORDS.PAS Conversion & Binary Compatibility

**Phase:** Phase 1 - Foundation
**Duration:** 1 week (actual)
**Sprint Dates:** 2025-11-23 (Completed)
**Status:** COMPLETE ✅

---

## ⚠️ DIVERGENCE NOTE

**Original Sprint 5 Plan:** Configuration Management (already completed in Sprint 4)
**Actual Sprint 5 Work:** RECORDS.PAS Conversion (11 Pascal compatibility modules, binary format support)

**Rationale for Change:** Configuration Management was already completed in Sprint 4. Sprint 5 implemented critical Pascal binary compatibility layer to enable data migration from legacy BBS systems. This was not in the original plan but provides essential migration capability.

**Value Delivered:** Binary compatibility with Pascal USER.LST, FILES.DAT, message formats. Zero data loss, bidirectional conversion, migration path for production deployments.

---

## Sprint Overview

Sprint 05 implements the configuration management system for Impulse 7.1, including TOML-based configuration files, validation, and hot-reload capability. This sprint creates the infrastructure for managing all BBS settings, from network ports to security policies, in a human-editable format.

**Context:** This is the fifth sprint of Phase 1 (Foundation). Configuration management is essential for deployment flexibility and operational management.

**Expected Outcomes:** By the end of this sprint, the project will have a complete configuration system that loads settings from TOML files, validates them, and supports hot-reload without service restart.

---

## Objectives

- [ ] Implement configuration loading system with TOML support
- [ ] Create configuration validation with clear error messages
- [ ] Add configuration hot-reload capability
- [ ] Build CLI tool for configuration management

---

## Deliverables

| Deliverable | Type | Acceptance Criteria |
|-------------|------|---------------------|
| `impulse-config` crate | Code | Crate compiles with config loading and validation |
| TOML configuration format | Documentation | All config options documented with examples |
| Hot-reload system | Code | Configuration changes detected and applied without restart |
| CLI configuration tool | Code | `impconfig` binary with generate and validate subcommands |

---

## Detailed Tasks

### Task Category 1: Configuration File Format Design

- [ ] **Task 1.1**: Design TOML structure
  - Implementation notes: Sections for server, security, paths, limits, features
  - Files affected: `config/impulse.toml.example`
  - Estimated hours: 4

- [ ] **Task 1.2**: Define all configuration options
  - Implementation notes: Network ports, database path, session timeout, file size limits, etc.
  - Files affected: `crates/impulse-config/src/schema.rs`
  - Estimated hours: 5

- [ ] **Task 1.3**: Document configuration file format
  - Implementation notes: Comprehensive comments in example config, separate docs
  - Files affected: `config/README.md`, inline TOML comments
  - Estimated hours: 3

- [ ] **Task 1.4**: Create default configuration generator
  - Implementation notes: Sensible defaults for all settings
  - Files affected: `crates/impulse-config/src/defaults.rs`
  - Estimated hours: 2

### Task Category 2: Configuration Parsing and Validation

- [ ] **Task 2.1**: Implement TOML parsing with serde
  - Implementation notes: Deserialize TOML into Config struct
  - Files affected: `crates/impulse-config/src/loader.rs`
  - Estimated hours: 3

- [ ] **Task 2.2**: Add field-level validation
  - Implementation notes: Port ranges (1-65535), positive integers, valid paths
  - Files affected: `crates/impulse-config/src/validation.rs`
  - Estimated hours: 5

- [ ] **Task 2.3**: Implement cross-field validation
  - Implementation notes: Min < max values, path existence checks, port conflicts
  - Files affected: `crates/impulse-config/src/validation.rs`
  - Estimated hours: 4

- [ ] **Task 2.4**: Create clear error messages
  - Implementation notes: User-friendly errors with specific field names and valid ranges
  - Files affected: `crates/impulse-config/src/error.rs`
  - Estimated hours: 3

### Task Category 3: Configuration Hot-Reload

- [ ] **Task 3.1**: Implement file watching
  - Implementation notes: Use notify crate to watch config file for changes
  - Files affected: `crates/impulse-config/src/watcher.rs`
  - Estimated hours: 4

- [ ] **Task 3.2**: Create reload notification system
  - Implementation notes: Tokio broadcast channel to notify services of config changes
  - Files affected: `crates/impulse-config/src/reload.rs`
  - Estimated hours: 4

- [ ] **Task 3.3**: Handle reload errors gracefully
  - Implementation notes: Keep old config if new config is invalid
  - Files affected: `crates/impulse-config/src/reload.rs`
  - Estimated hours: 3

- [ ] **Task 3.4**: Add reload hooks for services
  - Implementation notes: Allow services to register callbacks for config changes
  - Files affected: `crates/impulse-config/src/hooks.rs`
  - Estimated hours: 4

### Task Category 4: CLI Configuration Tool

- [ ] **Task 4.1**: Create `impconfig` binary crate
  - Implementation notes: CLI tool with clap for argument parsing
  - Files affected: `crates/impconfig/src/main.rs`
  - Estimated hours: 2

- [ ] **Task 4.2**: Implement `generate` command
  - Implementation notes: Generate default config file at specified path
  - Files affected: `crates/impconfig/src/commands/generate.rs`
  - Estimated hours: 3

- [ ] **Task 4.3**: Implement `validate` command
  - Implementation notes: Load and validate config file, report errors
  - Files affected: `crates/impconfig/src/commands/validate.rs`
  - Estimated hours: 3

- [ ] **Task 4.4**: Add `show` command
  - Implementation notes: Display current config with effective values
  - Files affected: `crates/impconfig/src/commands/show.rs`
  - Estimated hours: 2

- [ ] **Task 4.5**: Implement `diff` command (optional)
  - Implementation notes: Compare two config files and show differences
  - Files affected: `crates/impconfig/src/commands/diff.rs`
  - Estimated hours: 3

### Task Category 5: Testing and Documentation

- [ ] **Task 5.1**: Write unit tests for validation logic
  - Implementation notes: Test valid and invalid configs, edge cases
  - Files affected: `tests/validation_test.rs`
  - Estimated hours: 4

- [ ] **Task 5.2**: Write integration tests for hot-reload
  - Implementation notes: Modify config file, verify reload triggers
  - Files affected: `tests/reload_test.rs`
  - Estimated hours: 4

- [ ] **Task 5.3**: Test CLI tool commands
  - Implementation notes: Test all subcommands with various inputs
  - Files affected: `tests/cli_test.rs`
  - Estimated hours: 3

- [ ] **Task 5.4**: Create configuration guide documentation
  - Implementation notes: Comprehensive guide with examples for each option
  - Files affected: `docs/configuration-guide.md`
  - Estimated hours: 4

---

## Technical Details

### Architecture Considerations

- Use TOML for human-friendly configuration (widely adopted, good tooling)
- Support environment variable overrides for container deployments
- Design for extensibility (plugins may add config sections)
- Separate validation from parsing for reusability

### Dependencies

**Crate-Level Dependencies:**
```toml
[dependencies]
serde = { workspace = true }
toml = "0.8"
notify = "6.1"  # File system watcher
tokio = { workspace = true }
anyhow = { workspace = true }
thiserror = { workspace = true }

[impconfig dependencies]
clap = { version = "4.4", features = ["derive"] }
colored = "2.1"  # For terminal output
```

### Code Patterns

**Configuration Structure:**
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub server: ServerConfig,
    pub security: SecurityConfig,
    pub paths: PathsConfig,
    pub limits: LimitsConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub telnet_port: u16,
    pub ssh_port: u16,
    pub bind_address: String,
    pub max_connections: usize,
}

impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        config.validate()?;
        Ok(config)
    }

    pub fn validate(&self) -> Result<()> {
        // Validation logic
        if self.server.telnet_port == self.server.ssh_port {
            return Err(anyhow!("Telnet and SSH ports must be different"));
        }
        Ok(())
    }
}
```

**Hot-Reload Pattern:**
```rust
pub async fn watch_config(
    path: PathBuf,
    reload_tx: broadcast::Sender<Config>,
) -> Result<()> {
    let (tx, mut rx) = mpsc::channel(10);

    let mut watcher = notify::recommended_watcher(move |res| {
        if let Ok(event) = res {
            let _ = tx.blocking_send(event);
        }
    })?;

    watcher.watch(&path, RecursiveMode::NonRecursive)?;

    while let Some(_event) = rx.recv().await {
        match Config::load(&path) {
            Ok(new_config) => {
                let _ = reload_tx.send(new_config);
            }
            Err(e) => {
                eprintln!("Config reload failed: {}", e);
            }
        }
    }

    Ok(())
}
```

---

## Dependencies

### Upstream Dependencies
- **Sprint 02**: Requires core types for SystemConfig
- **Sprint 04**: May use storage layer for config persistence (optional)

### Blocks Downstream
- **Sprint 06**: Session management needs config for timeouts and limits
- **Sprint 09**: Authentication needs config for security settings
- **All future sprints**: Configuration used throughout system

---

## Acceptance Criteria

- [ ] Configuration loads from TOML file
- [ ] Invalid configs rejected with clear error messages
- [ ] Hot-reload works without service restart
- [ ] `impconfig generate` creates valid default config
- [ ] `impconfig validate` detects all validation errors
- [ ] All configuration options documented
- [ ] No hardcoded configuration values remain in code

---

## Testing Requirements

### Unit Tests
- [ ] Validation logic (valid and invalid configs)
- [ ] Default config generation
- [ ] TOML serialization round-trips
- [ ] Environment variable overrides (if implemented)

### Integration Tests
- [ ] Hot-reload triggers on file modification
- [ ] Invalid reload preserves old config
- [ ] CLI tool commands work correctly
- [ ] Services receive reload notifications

### Error Handling Tests
- [ ] Missing config file
- [ ] Malformed TOML
- [ ] Invalid values (out of range, wrong type)
- [ ] Missing required fields

---

## Notes and Decisions

### Design Decisions
*To be populated during sprint*

**Proposed Decisions:**
- Use TOML over YAML (simpler, fewer footguns)
- Support hot-reload but require explicit opt-in per service
- Validate on load and on reload
- CLI tool in separate binary for easier distribution

### Lessons Learned
*To be populated during sprint*

### Risks and Mitigations
- **Risk**: Hot-reload may cause service instability
- **Mitigation**: Validate before applying, allow services to opt-out of reload
- **Risk**: Configuration format may evolve over time
- **Mitigation**: Version configuration schema, support migration
- **Risk**: Complex validation logic hard to maintain
- **Mitigation**: Use declarative validation where possible, comprehensive tests

---

## Progress Log

### Week 1
- *Date*: Progress notes will be added here as sprint progresses

### Week 2
- *Date*: Progress notes will be added here as sprint progresses

### Week 3
- *Date*: Progress notes will be added here as sprint progresses

### Sprint Completion
- **Completed**: 2025-11-23
- **Status**: COMPLETE ✅ - New work not in original plan
- **Deliverables**: 11 Pascal compatibility modules, 3,500+ lines, 195 tests

---

## Actual Deliverables (Sprint Complete)

### 11 Pascal Compatibility Modules (3,500+ lines, 195 tests)

**Location:** `crates/impulse-types/src/pascal/`

1. **pascal_user.rs** (650 lines) - PascalUserRec with binary serialization
2. **pascal_file.rs** (600 lines) - PascalFileRec with binary serialization
3. **pascal_message.rs** (550 lines) - PascalMessageRec with binary serialization
4. **pascal_config.rs** (700 lines) - Pascal config structures
5. **pascal_types.rs** (400 lines) - Common Pascal types (ASCIIZ, packed booleans)
6. **conversion.rs** (300 lines) - Bidirectional Rust ↔ Pascal conversions
7. **validation.rs** (200 lines) - Pascal-specific validation rules
8. **security.rs** (150 lines) - Security level constants and mappings
9. **limits.rs** (100 lines) - Pascal-era limits and constants
10. **compat.rs** (200 lines) - Compatibility helpers
11. **tests/** (195 tests) - Comprehensive binary round-trip testing

### Key Capabilities

**Binary Compatibility:**
- Read/write Pascal USER.LST format (user database)
- Read/write Pascal FILES.DAT format (file area database)
- Read/write message base binary formats
- Zero data loss during conversion
- All Pascal fields preserved (including deprecated ones)

**Bidirectional Conversion:**
- Modern Rust User ↔ Legacy PascalUserRec
- Modern Rust FileEntry ↔ Legacy PascalFileRec
- Modern Rust Message ↔ Legacy PascalMessageRec
- Field mapping with type conversions
- Optional field handling

**Binary Format Support:**
- Packed boolean arrays (8 flags per byte)
- ASCIIZ strings (null-terminated, Pascal-style)
- Fixed-size arrays (ARRAY[1..N])
- Nested structures
- Little-endian encoding

**Type Safety:**
- Rust's type system prevents Pascal-era bugs
- Modern validation while maintaining compatibility
- Clear separation of legacy vs modern types

### Quality Metrics

- **Tests**: 195 total (100% passing)
- **Coverage**: Comprehensive binary round-trip testing
- **Code Quality**: 0 clippy warnings (with targeted allows for Pascal patterns)
- **Documentation**: All public APIs documented

### Value Delivered

**Migration Path:**
- Import legacy user databases from Pascal BBS
- Convert file area databases
- Migrate message bases
- Foundation for data migration utilities

**Production Readiness:**
- Backward compatibility with existing Pascal data
- Gradual migration capability (run both systems)
- Zero data loss guarantee
- Rollback capability if needed

### Analysis

Sprint 5 diverged from the original plan (Configuration Management was already done in Sprint 4) to implement Pascal binary compatibility. This was NOT in the original Phase 1 plan but provides critical capability for production deployments and data migration. The RECORDS.PAS compatibility layer enables importing data from legacy Impulse 7.1 installations, making the modernization project viable for existing BBS operators.
