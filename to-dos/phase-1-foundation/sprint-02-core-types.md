# Sprint 02: Core Type System

**Phase:** Phase 1 - Foundation
**Duration:** 3 weeks
**Sprint Dates:** TBD
**Status:** Not Started

---

## Sprint Overview

Sprint 02 establishes the fundamental data structures and error handling framework for the entire Impulse 7.1 modernization project. This sprint focuses on defining core domain types, implementing a unified error handling strategy, and creating serialization infrastructure. These types will be used throughout all subsequent sprints and form the foundation of the entire codebase.

**Context:** This is the second sprint of Phase 1 (Foundation). The core types defined here will be referenced by all feature implementations in future sprints.

**Expected Outcomes:** By the end of this sprint, the project will have a complete set of well-documented, thoroughly tested core types with consistent error handling and serialization support.

---

## Objectives

- [ ] Define fundamental data structures in `impulse-core` crate
- [ ] Implement unified error handling framework using `thiserror`
- [ ] Create serialization infrastructure with Serde
- [ ] Achieve 100% test coverage on validation functions

---

## Deliverables

| Deliverable | Type | Acceptance Criteria |
|-------------|------|---------------------|
| `impulse-core` crate with core types | Code | User, FileRecord, Message, SystemConfig structs defined and documented |
| Unified `Error` type | Code | Single Error enum using `thiserror` with all expected variants |
| `Result<T>` type alias | Code | Workspace-wide Result type standardized |
| Serde serialization traits | Code | All core types support JSON and binary serialization |
| Unit tests | Tests | 100% coverage on validation logic, serialization round-trips verified |

---

## Detailed Tasks

### Task Category 1: Core Domain Type Definitions

- [ ] **Task 1.1**: Define User struct
  - Implementation notes: Name, password hash, security level, stats (calls, uploads, downloads), registration date, last login
  - Files affected: `crates/impulse-core/src/user.rs`
  - Estimated hours: 4

- [ ] **Task 1.2**: Define FileRecord struct
  - Implementation notes: Filename, description, uploader, size, date, download count, area ID
  - Files affected: `crates/impulse-core/src/file.rs`
  - Estimated hours: 3

- [ ] **Task 1.3**: Define Message struct
  - Implementation notes: From, to, subject, body, date, area, parent_id (for threading), read status
  - Files affected: `crates/impulse-core/src/message.rs`
  - Estimated hours: 4

- [ ] **Task 1.4**: Define SystemConfig struct
  - Implementation notes: BBS name, sysop, ports, paths, security settings, limits
  - Files affected: `crates/impulse-core/src/config.rs`
  - Estimated hours: 3

- [ ] **Task 1.5**: Add validation methods to all types
  - Implementation notes: Username length/characters, password strength, file size limits, message body length
  - Files affected: All type definition files
  - Estimated hours: 5

### Task Category 2: Error Handling Framework

- [ ] **Task 2.1**: Create Error enum with comprehensive variants
  - Implementation notes: IoError, DatabaseError, ValidationError, AuthenticationError, ConfigError, NetworkError
  - Files affected: `crates/impulse-core/src/error.rs`
  - Estimated hours: 3

- [ ] **Task 2.2**: Implement `thiserror` derives
  - Implementation notes: Add Display and std::error::Error trait implementations via thiserror
  - Files affected: `crates/impulse-core/src/error.rs`
  - Estimated hours: 2

- [ ] **Task 2.3**: Add error context using `anyhow` where appropriate
  - Implementation notes: Use anyhow::Context for application-level error handling
  - Files affected: Various implementation files
  - Estimated hours: 2

- [ ] **Task 2.4**: Define workspace Result type alias
  - Implementation notes: `pub type Result<T> = std::result::Result<T, Error>;`
  - Files affected: `crates/impulse-core/src/lib.rs`
  - Estimated hours: 1

### Task Category 3: Serialization Infrastructure

- [ ] **Task 3.1**: Add Serialize/Deserialize derives to all types
  - Implementation notes: Use `#[derive(Serialize, Deserialize)]` from serde
  - Files affected: All type definition files
  - Estimated hours: 2

- [ ] **Task 3.2**: Test JSON serialization
  - Implementation notes: Ensure all types round-trip through serde_json correctly
  - Files affected: Unit test files
  - Estimated hours: 3

- [ ] **Task 3.3**: Test binary serialization with bincode
  - Implementation notes: Verify compact binary representation for wire protocols
  - Files affected: Unit test files
  - Estimated hours: 3

- [ ] **Task 3.4**: Add custom serialization for special fields
  - Implementation notes: Password hashes, dates, optional fields
  - Files affected: Type definition files with custom serde attributes
  - Estimated hours: 3

### Task Category 4: Testing and Documentation

- [ ] **Task 4.1**: Write unit tests for all validation logic
  - Implementation notes: Test edge cases, boundary conditions, invalid inputs
  - Files affected: `tests/` directories in impulse-core
  - Estimated hours: 6

- [ ] **Task 4.2**: Write serialization round-trip tests
  - Implementation notes: Ensure data integrity through serialize → deserialize cycles
  - Files affected: `tests/serialization.rs`
  - Estimated hours: 3

- [ ] **Task 4.3**: Add rustdoc documentation for all public types
  - Implementation notes: Include usage examples, field descriptions, invariants
  - Files affected: All public type files
  - Estimated hours: 4

- [ ] **Task 4.4**: Create documentation examples
  - Implementation notes: Show common usage patterns for each type
  - Files affected: Module-level documentation
  - Estimated hours: 2

---

## Technical Details

### Architecture Considerations

- Use newtype pattern for strongly-typed IDs (UserId, FileId, MessageId)
- Implement builder pattern for complex types (SystemConfig)
- Use `#[non_exhaustive]` for enum types that may grow
- Leverage Rust's type system for compile-time guarantees (username length via const generics)

### Dependencies

**Crate-Level Dependencies:**
```toml
[dependencies]
serde = { workspace = true }
thiserror = { workspace = true }
anyhow = { workspace = true }
chrono = "0.4"  # For date/time handling

[dev-dependencies]
serde_json = "1.0"
bincode = "1.3"
```

### Code Patterns

**Error Handling Pattern:**
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Validation failed: {0}")]
    Validation(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Database error: {0}")]
    Database(String),
}

pub type Result<T> = std::result::Result<T, Error>;
```

**Validation Pattern:**
```rust
impl User {
    pub fn validate(&self) -> Result<()> {
        if self.name.is_empty() || self.name.len() > 30 {
            return Err(Error::Validation(
                "Username must be 1-30 characters".to_string()
            ));
        }
        Ok(())
    }
}
```

---

## Dependencies

### Upstream Dependencies
- **Sprint 01**: Requires workspace structure and CI pipeline

### Blocks Downstream
- **Sprint 03**: Pascal analysis requires core types for mapping
- **Sprint 04**: Storage layer needs type definitions
- **All future sprints**: Core types used throughout codebase

---

## Acceptance Criteria

- [ ] All core types compile with no clippy warnings
- [ ] 100% test coverage on validation functions
- [ ] Documentation examples compile and run correctly
- [ ] JSON and binary serialization round-trips verified
- [ ] Error types cover all expected failure modes
- [ ] All public APIs have comprehensive rustdoc comments

---

## Testing Requirements

### Unit Tests
- [ ] Validation logic for all types (valid and invalid inputs)
- [ ] Serialization round-trips (JSON and bincode)
- [ ] Error conversion and context preservation
- [ ] Builder patterns (if implemented)

### Integration Tests
- [ ] Types work correctly across crate boundaries
- [ ] Error types integrate with anyhow context

### Property-Based Tests
- [ ] Use proptest for validation edge cases
- [ ] Serialization round-trip properties

---

## Notes and Decisions

### Design Decisions
*To be populated during sprint*

**Proposed Decisions:**
- Use chrono for date/time (widely adopted, comprehensive)
- Use thiserror for library errors, anyhow for application code
- Prefer newtype pattern over raw types for IDs

### Lessons Learned
*To be populated during sprint*

### Risks and Mitigations
- **Risk**: Type definitions may evolve during implementation
- **Mitigation**: Use `#[non_exhaustive]` on enums; plan for schema versioning
- **Risk**: Serialization format compatibility with legacy data
- **Mitigation**: Document serialization format; consider separate migration types

---

## Progress Log

### Week 1
- *Date*: Progress notes will be added here as sprint progresses

### Week 2
- *Date*: Progress notes will be added here as sprint progresses

### Week 3
- *Date*: Progress notes will be added here as sprint progresses

### Sprint Completion
- **Completed**: TBD
- **Velocity**: TBD
- **Burndown**: TBD
