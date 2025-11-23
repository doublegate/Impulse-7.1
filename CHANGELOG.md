# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- **Project renamed from "Impulse-7.1" to "Impulse-Next_BBS"**
  - Repository URL: https://github.com/doublegate/Impulse-Next_BBS
  - Updated repository name, description, and topics on GitHub
  - Updated all documentation to reflect new project name
  - Historical references to "Impulse 7.1" preserved where referring to original Pascal source
- Migrated to Rust edition 2024 (MSRV 1.85+)
- Updated minimum supported Rust version from 1.80 to 1.85
- All crates now use edition 2024 features and improvements
- Verified: All 82 tests passing, 0 clippy warnings, all platforms compatible

### Planned - Sprint 3 (Pascal Analysis)
- Deep analysis of original Pascal source code
- Module dependency graph generation
- Data structure mapping documentation (Pascal types → Rust types)
- Risk assessment for each Pascal unit

### Planned - Sprints 4-8 (Phase 1 Foundation)
- File parsing capabilities for legacy .DAT formats
- ANSI rendering engine
- Basic Telnet server
- User management system
- Authentication with Argon2id
- Security infrastructure

### Planned - Phase 2 (Sprints 9-16)
- Message base implementation (JAM/Hudson)
- File areas and browsing
- Menu system and navigation
- User profiles and statistics

### Planned - Phase 3 (Sprints 17-24)
- File transfer protocols (Zmodem, Xmodem, Ymodem)
- Theme system
- Door game interface
- Advanced message features

### Planned - Phase 4 (Sprints 25-32)
- Performance optimization
- Web-based administration
- Legacy migration tools
- Public 1.0 release

## [0.1.0] - 2025-11-23

### Added - Sprint 1 (Project Setup)

#### Infrastructure
- 16-crate Rust workspace with workspace dependency inheritance
- GitHub Actions CI/CD pipeline with 4 jobs:
  - Lint job: `cargo clippy` with zero warnings enforcement
  - Test job: `cargo test --workspace` across all crates
  - Build job: `cargo build --workspace --release`
  - Coverage job: Tarpaulin + Codecov integration
- Multi-OS support (Linux, Windows, macOS) in CI pipeline
- Comprehensive `.gitignore` for Rust projects

#### Documentation
- `CONTRIBUTING.md` (336 lines) with:
  - Development workflow and branch naming conventions
  - Coding standards and quality requirements
  - Testing requirements and procedures
  - Pull request process and guidelines
  - Commit message guidelines (Conventional Commits)
- Dual licensing: MIT OR Apache-2.0
  - `LICENSE-MIT` - MIT License
  - `LICENSE-APACHE` - Apache License 2.0

#### Workspace Structure
- Created 16 crate directories with initial scaffolding:
  - **Core**: `impulse-core`, `impulse-types`, `impulse-config`
  - **Protocol**: `impulse-protocol`, `impulse-telnet`, `impulse-ssh`
  - **Features**: `impulse-session`, `impulse-terminal`, `impulse-auth`, `impulse-message`, `impulse-file`, `impulse-user`, `impulse-door`
  - **Application**: `impulse-web`, `impulse-cli`, `impulse-server`

### Added - Sprint 2 (Core Type System)

#### Core Data Types (`impulse-types` crate)
- **User type** (265 lines):
  - 13 fields including id, name, password_hash, security_level, stats, registration_date
  - `SecurityLevel` enum with 6 levels (Locked, NewUser, Validated, Privileged, AssistantSysOp, SysOp)
  - `UserStats` struct with 7 metrics (calls, uploads, downloads, KB transferred, posts, time online)
  - Methods: `validate()`, `has_security_level()`, `is_sysop()`, `record_login()`
  - Comprehensive rustdoc with examples
  - 10 unit tests covering all validation logic

- **FileEntry type** (293 lines):
  - 13 fields including id, filename, description, uploader, size, dates, download tracking
  - Methods: `validate()`, `human_readable_size()`, `is_available()`, `is_protected()`, `extension()`, `record_download()`
  - Full rustdoc documentation with usage examples
  - 10 unit tests for all helper methods

- **Message type** (214 lines):
  - 11 fields including threading support (parent_id), privacy flags, deletion tracking
  - Methods: `validate()`, `is_public()`, `is_reply()`, `mark_read()`, `mark_unread()`, `delete()`, `undelete()`
  - Complete documentation with examples
  - 11 unit tests including threading validation

- **BbsConfig type** (502 lines):
  - Complex nested configuration structure with 13 top-level fields
  - Supporting types:
    - `Protocol` enum (Telnet, SSH, WebSocket)
    - `SystemLimits` struct (7 fields for max users, sessions, sizes)
    - `SecuritySettings` struct (7 fields for timeouts, rate limits)
    - `BbsPaths` struct (6 fields for system directories)
    - `ServerConfig` struct (4 fields per protocol server)
  - Builder pattern with `BbsConfigBuilder`
  - Methods: `validate()`, `builder()`, `default()`, `primary_server()`
  - 13 unit tests covering all configuration aspects

#### Error Handling Framework
- Unified `Error` enum with 15 comprehensive variants:
  1. `Validation(String)` - Input validation failures
  2. `Io(#[from] std::io::Error)` - I/O errors with automatic conversion
  3. `Database(String)` - Database operation failures
  4. `Authentication(String)` - Authentication failures
  5. `Authorization(String)` - Permission denied errors
  6. `NotFound(String)` - Resource not found
  7. `AlreadyExists(String)` - Duplicate resource errors
  8. `Configuration(String)` - Configuration errors
  9. `Network(String)` - Network-related errors
  10. `Parse(String)` - Parsing errors
  11. `Serialization(String)` - Serialization failures
  12. `Deserialization(String)` - Deserialization failures
  13. `Timeout` - Operation timeout
  14. `Internal(String)` - Internal server errors
  15. `External(Box<dyn std::error::Error + Send + Sync>)` - External errors
- `thiserror` derives with proper error messages
- Workspace-wide `Result<T>` type alias
- 3 unit tests for error handling

#### Serialization Infrastructure
- Serde derives (`Serialize`, `Deserialize`) on all core types
- JSON serialization support via `serde_json`
- Binary serialization support via `bincode`
- Comprehensive serialization test suite (11 tests):
  - JSON round-trip tests for all 4 core types
  - Binary round-trip tests for all 4 core types
  - Complex nested structure tests
  - Enum serialization tests
  - Data integrity verification through serialize → deserialize cycles

### Changed

#### Code Quality Improvements
- Replaced manual `impl Default` with `#[derive(Default)]` for `SecurityLevel`
- Replaced manual `impl Default` with `#[derive(Default)]` for `UserStats`
- Fixed field reassignment patterns in configuration tests
- Removed placeholder `assert!(true)` tests

#### Dependencies
- Added `chrono` 0.4.42 for date/time handling
- Added `bincode` 1.3 for binary serialization testing
- Added `serde_json` 1.0 for JSON serialization testing

### Infrastructure

#### Build Configuration
- Cargo workspace with resolver = "2"
- Workspace-level dependency inheritance for consistency
- Profile optimizations:
  - **Release**: LTO enabled, single codegen unit, stripped symbols
  - **Dev**: Debug symbols enabled, minimal optimization
  - **Test**: Level 1 optimization for faster test execution

#### Quality Metrics (Sprint 1-2 Complete)
- **Tests**: 82 total (100% passing)
  - 56 unit tests (validation logic)
  - 11 integration tests (serialization)
  - 15 doc tests (documentation examples)
- **Code Coverage**: Comprehensive coverage of all core types
- **Clippy Warnings**: 0 warnings
- **Documentation**: 100% rustdoc coverage on public APIs
- **Code Size**: ~2,473 lines of production code, tests, and documentation

### Documentation

#### Comprehensive Documentation Suite (30,363+ lines)
- **Core Documentation** (9 files, 9,632 lines):
  - `00-project-overview.md` - Vision, objectives, stakeholders
  - `01-phase-sprint-plan.md` - 32-sprint roadmap
  - `02-architecture.md` - System design, 16-crate structure
  - `03-technical-details.md` - Pascal→Rust conversion details
  - `04-development-guide.md` - Developer onboarding
  - `05-testing-strategy.md` - Testing methodology
  - `06-deployment-guide.md` - Docker, Kubernetes deployment
  - `07-migration-guide.md` - Legacy data migration
  - `08-security-architecture.md` - Security design

- **Sprint TODO Files** (30 files, 19,214 lines):
  - Phase 1 (Sprints 1-8): 2,802 lines, 17 Rust examples
  - Phase 2 (Sprints 9-16): 3,039 lines, 16 Rust examples
  - Phase 3 (Sprints 17-24): 4,660 lines, 23 Rust examples
  - Phase 4 (Sprints 25-32): 8,713 lines, 37 Rust examples

- **Reference Documentation** (2 files, 354 lines):
  - `impulse-history.md` - BBS history and cultural context
  - `rust-conversion-technical.md` - Conversion strategies

- **Verification Reports**:
  - `SPRINT-01-02-VERIFICATION-REPORT.md` - Complete implementation verification
  - `DOCUMENTATION-VERIFICATION-REPORT.md` - Documentation quality verification

## [0.0.0] - Initial Planning

### Initial - Project Conception
- Project vision: Modernize Impulse 7.1 BBS from Borland Pascal 7.0 to Rust
- Target: 24-month development cycle, 32 sprints, 4 phases
- Goal: ~96 Pascal units → 120-150 Rust modules
- Platforms: Linux, Windows 11, macOS, BSD variants

### Initial - Pascal Legacy Build System
- Automated build system for Borland Pascal 7.0 source
- DOSBox integration for DOS-based compiler
- GitLab CI/CD pipeline for Pascal builds
- Build scripts (`build.sh`, `clean.sh`)
- Docker containerization for consistent builds
- Complete Impulse 7.1 release files preserved in `imp71rel/`

### Initial - Documentation Planning
- Architecture decision: Hybrid rewrite (semantic conversion, not line-by-line)
- Technology stack selection:
  - Tokio 1.47+ for async runtime
  - crossterm 0.28 for terminal I/O
  - SQLx 0.8 for database access
  - Argon2id for password hashing
  - Axum 0.7 for web framework
- Planning complete for all 32 sprints

---

## Version History Summary

| Version | Date | Phase | Sprints | Key Achievements |
|---------|------|-------|---------|------------------|
| 0.1.0 | 2025-11-23 | Phase 1 Foundation | 1-2 | Infrastructure, core types, CI/CD, 82 tests |
| 0.0.0 | - | Planning | - | Project conception, documentation, Pascal legacy build |

---

## Links

- **Repository**: [https://github.com/doublegate/Impulse-Next_BBS](https://github.com/doublegate/Impulse-Next_BBS)
- **Issues**: [GitHub Issues](https://github.com/doublegate/Impulse-Next_BBS/issues)
- **Documentation**: [Project Documentation](https://github.com/doublegate/Impulse-Next_BBS/tree/main/docs)
- **Contributing**: [CONTRIBUTING.md](https://github.com/doublegate/Impulse-Next_BBS/blob/main/CONTRIBUTING.md)

---

**Note**: This project is converting a classic BBS system from 1990s Borland Pascal to modern Rust. The version numbers reflect the modernization project, not the original Impulse 7.1 software. The project aims to preserve BBS history while leveraging modern safety, performance, and maintainability benefits of Rust.
