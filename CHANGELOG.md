# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added - Sprint 5 (Core Types Implementation)

#### RECORDS.PAS Conversion Complete
- **Converted Pascal RECORDS.PAS** (829 lines, 40+ types) to Rust with binary compatibility
- **11 new source modules** created in impulse-types crate
- **195 tests** in impulse-types (up from 82) - All passing
- **9,331 lines of code** added across 18 files

#### Pascal Compatibility Layer
- **pascal_types.rs** - Core Pascal types (AR flags, colors, enums) (428 lines)
- **pascal_config.rs** - System configuration (SYSTAT.DAT) (710 lines, 22 tests)
- **pascal_user.rs** - User records (USER.LST) with PascalString<N> type (443 lines, 15 tests)
- **pascal_message.rs** - Message system (*.BRD, BOARDS.DAT) (782 lines, 28 tests)
- **pascal_file.rs** - File areas (UPLOADS.DAT, *.DIR) (565 lines, 18 tests)
- **pascal_aux.rs** - Auxiliary records (NAMES.LST, ZSCAN.DAT, ZLOG.DAT) (477 lines, 16 tests)

#### Supporting Flag Modules
- **user_flags.rs** - User permissions/preferences (24 flags, 340 lines, 6 tests)
- **message_enums.rs** - Message board enumerations (147 lines, 4 tests)
- **board_flags.rs** - Board/conference flags (182 lines, 5 tests)
- **menu_flags.rs** - Menu/command flags (243 lines, 8 tests)
- **protocol_flags.rs** - File transfer protocols (141 lines, 4 tests)

#### Key Type Conversions
- **PascalString<N>**: Generic fixed-length string type matching Pascal String[N] format
  - Zero-padded byte arrays for exact binary layout compatibility
  - Conversion methods: from_string(), to_string(), as_bytes()
  - Verified through round-trip serialization tests
- **Bitflags Integration**: Pascal set types mapped to Rust bitflags with byte array conversion
- **Binary Serialization**: binrw integration for all record types maintaining byte-level Pascal compatibility

#### Pascal Type Coverage
- **System Configuration**: systatrec (60 fields), bbsrec (30 fields), eventsrec (10 events)
- **User Management**: PascalString<N>, user flags (24 flags), validation methods
- **Message System**: mheaderrec, boardsrec, msgscanrec, messageidx
- **File Areas**: ulrec (file area config), ulfrec (upload records), verbose descriptions
- **Auxiliary Types**: PackedDateTime (6-byte format), ZScanRec (1588 bytes), ZLogRec (system usage)

#### Documentation Created (3 files)
- `docs/pascal-analysis/records-pas-conversion-plan.md` (1,124 lines) - Complete conversion strategy
- `docs/pascal-analysis/type-reconciliation.md` (486 lines) - Type conflict analysis and resolution
- `docs/pascal-analysis/quick-reference-pascal-to-rust.md` (312 lines) - Quick reference guide

#### Dependencies Added
- **binrw 0.15** - Binary read/write for Pascal record compatibility
- **bitflags 2.6** - Pascal set type support (already in workspace)

#### Quality Metrics
- **224 tests total** (195 in impulse-types, 29 in impulse-config)
- **Build succeeds** with all features enabled
- **Binary compatibility verified** through round-trip serialization tests
- **Comprehensive validation** methods for all record types

### Added - Sprint 4 (Configuration System)

#### impulse-config Crate Implementation
- **New crate: impulse-config** - Complete configuration management system
  - Hierarchical configuration loading with figment integration
  - TOML file support for human-readable configuration
  - Environment variable overrides (IMPULSE_* prefix)
  - Three validation modes: config_only(), strict(), deployment()
  - Comprehensive error handling with 15 error variants
  - Save/load functionality with round-trip support
  - Type-safe configuration through Rust's type system

#### Configuration Features
- **Config Precedence**: Hardcoded defaults < TOML file < Environment variables
- **Validation Options**:
  - Config-only: Value validation without filesystem/network checks
  - Strict: Full validation including path existence and port availability
  - Deployment: Path validation allowing directory creation, skipping port checks
- **Path Validation**: Checks all 7 BBS directory paths (data, users, messages, files, logs, temp, doors)
- **Port Validation**: TCP listener checks for Telnet (2323), SSH (2222), Web Admin (8080)
- **Environment Variables**: Override any config value via IMPULSE_* prefix (e.g., IMPULSE_NAME, IMPULSE_SERVERS_0_PORT)

#### Testing & Quality
- **37 tests total** (27 unit + 11 integration + 10 doc tests) - All passing
- **Integration tests** with serial execution for environment variable isolation
- **Rust 2024 edition safety**: Proper unsafe blocks for environment manipulation
- **Test fixtures**: TOML round-trip tests, environment override tests, validation mode tests
- **0 clippy warnings**: Boxed large error variants, proper struct initialization patterns

#### Documentation
- **Comprehensive README.md** (321 lines) with:
  - Quick start guide and basic usage examples
  - Environment variable override examples
  - Validation mode comparison (config_only vs strict vs deployment)
  - Complete example config.toml
  - Error handling patterns
  - Testing guide

#### Dependencies
- `figment 0.10` - Hierarchical configuration framework
- `toml 0.8` - TOML serialization/deserialization
- `serial_test 3.0` - Test isolation for environment variables
- `tempfile 3.8` - Temporary file handling in tests

#### Performance Optimizations
- **Boxed large error variants**: Reduced ConfigError size by boxing figment::Error (208+ bytes)
- **Efficient config merging**: Single-pass configuration loading with figment
- **Lazy validation**: Optional filesystem/network checks only when needed

### Added - Sprint 3 (Pascal Analysis)

#### Comprehensive Pascal Codebase Analysis
- **114 Pascal files analyzed** (39,079 lines of code)
- **1,070 dependency relationships** mapped and documented
- **16 documentation files created** (796KB total):
  - `pascal-inventory.md` - Complete inventory by functional category
  - `pascal-unit-analysis.md` - Detailed analysis of all 114 units
  - `pascal-dependencies.md` - Dependency documentation
  - `pascal-dependency-matrix.csv` - Structured dependency data
  - `pascal-dependencies.dot` - Graphviz dependency graph source
  - `pascal-dependencies.svg` - Visual dependency graph (556KB)
  - `pascal-globals.md` - Global state analysis (33 const files, 90 var files)
  - `pascal-overlays.md` - DOS overlay system documentation
  - `pascal-interrupts.md` - Hardware interrupt handlers
  - `pascal-dos-specific.md` - DOS-specific code patterns
  - `pascal-binary-formats.md` - Binary file format documentation
  - `type-mapping.md` - Comprehensive Pascal→Rust type mappings
  - `conversion-risk-assessment.md` - Risk ratings for all units
  - `high-risk-units.md` - Detailed analysis of 38 high/critical-risk units
  - `risk-mitigations.md` - Mitigation strategies for identified risks
  - `conversion-order.md` - 4-phase dependency-aware conversion plan

#### Risk Assessment Results
- **CRITICAL Risk Units:** 11 (9.6%) - Inline assembly, interrupt handlers, hardware access
- **HIGH Risk Units:** 27 (23.7%) - DOS-specific calls, binary I/O, pointer manipulation
- **MEDIUM Risk Units:** 30 (26.3%) - Complex logic, global state, overlay system
- **LOW Risk Units:** 46 (40.4%) - Straightforward conversion with standard patterns

#### Platform-Specific Patterns Identified
- **75 overlay directives** - DOS memory management (to be removed)
- **14 files with inline assembly** - Requires complete rewrite
- **2 interrupt handlers** - Replace with OS-agnostic signal handling
- **23 files with DOS-specific calls** - Abstract behind traits
- **29 files with binary file I/O** - bincode serialization strategy

#### 4-Phase Conversion Roadmap
- **Phase 1 (Sprints 4-10):** Foundation - RECORDS.PAS, COMMON*.PAS, utilities
- **Phase 2 (Sprints 11-18):** Core Services - FILE*.PAS, MAIL*.PAS, authentication
- **Phase 3 (Sprints 19-26):** Advanced Features - SYSOP*.PAS, protocols, terminal emulation
- **Phase 4 (Sprints 27-32):** Integration - IMP.PAS, high-risk modules, testing

#### Conversion Strategy & Prioritization Plan
- **Created comprehensive roadmap document** `docs/09-conversion-strategy-plan.md` (1,679 lines, ~11,750 words)
  - 16-part strategic document synthesizing all 19 Pascal analysis files
  - Executive summary with key statistics and timeline
  - 6 strategic principles guiding all conversion decisions
  - Risk-based prioritization framework with detailed scoring methodology
  - Platform-specific migration strategy (DOS → modern OS)
  - Complete 4-phase conversion roadmap (Sprints 3-32, 24 months)
  - Type system migration strategy (Pascal → Rust mappings)
  - Global state refactoring strategy (BbsState design)
  - Binary file format strategy (bincode + serde)
  - Dependency management strategy (1,070 dependencies)
  - Testing strategy with coverage targets by risk level
  - High-risk module strategy (38 units requiring special attention)
  - Sprint execution guidelines (3-week sprint structure)
  - Success metrics & KPIs (progress, quality, performance)
  - Risk mitigation timeline (critical risks by phase)
  - Parallel conversion opportunities (30-40% time reduction potential)
  - Cross-reference matrix linking all source documents

#### Sprint Efficiency
- **Estimated Duration:** 93 hours (3 weeks)
- **Actual Duration:** ~2 hours
- **Efficiency Gain:** 97.8% time reduction through automated analysis

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
- **Dependency updates merged** (PRs #4, #5, #7, #8):
  - toml: 0.8 → 0.9
  - crossterm: 0.28 → 0.29
  - binrw: 0.14 → 0.15
  - axum: 0.7 → 0.8

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
