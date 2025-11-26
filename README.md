# Impulse-Next - Modern BBS Software

[![CI](https://github.com/doublegate/Impulse-Next_BBS/workflows/CI/badge.svg)](https://github.com/doublegate/Impulse-Next_BBS/actions)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/doublegate/Impulse-Next_BBS#license)
[![Rust Version](<https://img.shields.io/badge/rust-1.85%2B%20(edition%202024)-orange.svg>)](https://www.rust-lang.org)

A complete modernization of the classic Impulse 7.1 Bulletin Board System from Borland Pascal 7.0 to modern Rust, preserving BBS history while leveraging contemporary software engineering practices.

## Table of Contents

- [Documentation Structure](#documentation-structure)
- [Overview](#overview)
- [Project Status](#project-status)
- [Features](#features)
- [Architecture](#architecture)
- [Quick Start](#quick-start)
- [Development](#development)
- [Project Structure](#project-structure)
- [Technology Stack](#technology-stack)
- [Documentation](#documentation)
- [Roadmap](#roadmap)
- [Testing](#testing)
- [Contributing](#contributing)
- [License](#license)
- [Acknowledgments](#acknowledgments)

## Documentation Structure

**Recently Reorganized** (November 2024): Our documentation has been comprehensively reorganized from 44 scattered files into a structured hierarchy with 9 categories and 7 subcategories for improved navigation and discoverability.

### Quick Navigation

All documentation is centralized in the `docs/` directory. Start with **[docs/INDEX.md](docs/INDEX.md)** for the complete documentation hub.

| Category                                   | Description                              | Files | Key Documents                                    |
| ------------------------------------------ | ---------------------------------------- | ----- | ------------------------------------------------ |
| [Getting Started](docs/getting-started/)   | Project overview and introduction        | 1     | project-overview.md                              |
| [Architecture](docs/architecture/)         | System design and technical architecture | 3     | system-architecture.md, security-architecture.md |
| [Planning](docs/planning/)                 | Phase plans, sprint roadmaps             | 2     | phase-sprint-plan.md, conversion-strategy.md     |
| [Implementation](docs/implementation/)     | Development guides, integration docs     | 2     | development-guide.md, logging-integration.md     |
| [Testing](docs/testing/)                   | Testing strategies and requirements      | 1     | testing-strategy.md                              |
| [Deployment](docs/deployment/)             | Deployment and migration guides          | 2     | deployment-guide.md, migration-guide.md          |
| [Pascal Reference](docs/pascal-reference/) | Complete Pascal analysis and conversion  | 21    | Analysis, conversion guides, risk assessments    |
| [Reports](docs/reports/)                   | Analysis reports, sprint completions     | 9     | CI/CD analysis, sprint reports                   |
| [Reference](docs/reference/)               | Historical context, technical notes      | 2     | impulse-history.md                               |

**Total Documentation**: 43 files (38 markdown + 5 data files) covering all aspects of the project.

### Benefits of New Structure

- **Clear Navigation**: Logical categorization by purpose (getting started → architecture → implementation)
- **Easier Discovery**: Find what you need quickly with organized categories
- **Comprehensive Coverage**: All documentation in one structured location
- **Maintained History**: Complete Pascal analysis and conversion tracking
- **Progress Tracking**: Sprint reports and completion documentation

## Overview

### What is Impulse 7.1?

Impulse 7.1 is a classic BBS (Bulletin Board System) software that powered dial-up bulletin board systems during the height of the BBS era in the 1990s. It provided features like:

- Multi-node message boards with threading
- File areas with descriptions and download tracking
- Door game support (external programs)
- User management with security levels
- Multi-protocol file transfers (Zmodem, Xmodem, Ymodem)
- ANSI art and terminal emulation

More historical context: [Impulse BBS on Archive.org](https://web.archive.org/web/20011204010133/http://www.demonic.net/impulse/)

### Why This Modernization?

This project aims to:

1. **Preserve BBS History**: Keep classic BBS software accessible and functional on modern systems
2. **Memory Safety**: Eliminate undefined behavior through Rust's ownership system
3. **Cross-Platform**: Run on Linux, Windows 11, macOS, and BSD variants
4. **Modern Protocols**: Add SSH, WebSocket, and REST API alongside traditional telnet
5. **Performance**: Replace DOS overlays with efficient async I/O
6. **Maintainability**: Convert ~96 Pascal units to well-tested, documented Rust modules

## Project Status

**Current Version**: 0.1.0 (Phase 2 - Core Features: 87.5% Complete!)
**Development Phase**: Phase 2 - Core Features (In Progress)
**Completion**: Sprint 15/32 (46.88%) - Phase 1: 8/8 (100% ✅), Phase 2: 7/8 (87.5%)

### Recent Milestones

- ✅ **Phase 1 Foundation COMPLETE** (November 2025 - 8 sprints in ~6 weeks)
- ✅ **Sprint 1** (Project Setup): 18-crate workspace, CI/CD pipeline (5 jobs), cross-platform support
- ✅ **Sprint 2** (Core Types): User, FileEntry, Message, BbsConfig types with 82 tests
- ✅ **Sprint 3** (Pascal Analysis): 114 files analyzed (39,079 LOC), 1,070 dependencies, 16 analysis documents
- ✅ **Sprint 4** (Configuration): impulse-config crate, TOML + ENV loading, 3 validation modes, 37 tests
- ✅ **Sprint 5** (RECORDS.PAS): 11 Pascal modules, PascalString<N> type, 195 tests, binary compatibility
- ✅ **Sprint 6** (User System): impulse-user (26 tests), impulse-auth (16 tests), Argon2id, session management
- ✅ **Sprint 7** (Logging): impulse-logging crate, rotation/archival/audit, 80 tests, <2µs overhead
- ✅ **Sprint 8** (Testing Framework): 64.51% coverage baseline, integration tests, property tests, 7 benchmarks
- ✅ **Sprint 9** (User Authentication): Rate limiting, account lockout, input validation, registration/login/logout flows
- ✅ **Sprint 10** (Menu System): impulse-menu crate, TOML parser, hotkey/fullmenu modes, navigation state machine
- ✅ **Sprint 11** (Message Read): MessageBase trait, JAM/Hudson formats, message list/read screens, threading, 72 tests
- ✅ **Sprint 12** (Message Write): Message posting, replies, quoting, validation, sanitization, atomic writes, 27 tests
- ✅ **Sprint 13** (File Browsing): File area management, list/details screens, wildcard search, FILE_ID.DIZ extraction, 76 tests
- ✅ **Sprint 14** (File Upload): Upload processor, ClamAV scanning, duplicate detection, validation, quarantine, 180 tests
- ✅ **Sprint 15** (User Profiles & Statistics): Profile display, stats tracking, settings editor, achievements, privacy controls, user directory, 128 tests

### Phase 1 Achievements

**Infrastructure:**

- 19 crates (17 libraries + 2 binaries)
- 5-job CI/CD pipeline (lint, test×3, build×3, coverage, benchmarks)
- 94+ commits across 98 Rust source files
- 28,000+ lines of code (production + tests)

**Quality Metrics (Phase 1+2 Current):**

- **Tests**: 1,254+ (100% passing rate)
- **Coverage**: 64.51% baseline (target: 75% for Phase 2 completion)
- **Clippy**: 0 warnings
- **Documentation**: 43 files, 35,000+ lines
- **Build**: <10s full workspace
- **Test Execution**: <5s all tests

**Key Features:**

- Complete type system with Pascal binary compatibility
- User management with Argon2id authentication
- Configuration system with hot-reload
- Structured logging with rotation and audit trails
- Testing infrastructure with coverage tracking

### Next Steps

- **Phase 2**: Core Features (Sprints 9-16, 7/8 complete - 87.5%)
  - ✅ Sprint 9: User Authentication (rate limiting, lockout, validation)
  - ✅ Sprint 10: Menu System (TOML parser, navigation)
  - ✅ Sprint 11: Message Read (MessageBase trait, JAM/Hudson, screens)
  - ✅ Sprint 12: Message Write (posting, replies, quoting)
  - ✅ Sprint 13: File Browsing (areas, list, details, search, FILE_ID.DIZ)
  - ✅ Sprint 14: File Upload (processor, ClamAV scanning, validation, quarantine)
  - ✅ Sprint 15: User Profiles & Statistics (profile display, settings, achievements, privacy)
  - 🔄 Sprint 16: Session Management - NEXT
- **Goal**: Functional BBS with messaging, file areas, and user profiles by end of Phase 2
- **Timeline**: 24 months total, currently 46.88% complete (ahead of schedule)

## Features

### Current Implementation (v0.1.0 - Phase 1 Complete + Phase 2 Sprints 9-15)

**Phase 1 Foundation (Sprints 1-8, November 2025):**

**Core Infrastructure (Sprint 1):**

- ✅ 18-crate Rust workspace (16 libraries + 2 binaries)
- ✅ CI/CD pipeline with 5 jobs (lint, test×3 platforms, build×3, coverage, benchmarks)
- ✅ Cross-platform support (Linux, Windows, macOS)
- ✅ Comprehensive documentation (34 files, 31,000+ lines)

**Type System (Sprint 2):**

- ✅ Core types: User, FileEntry, Message, BbsConfig
- ✅ Unified error handling (15 error variants)
- ✅ JSON and binary serialization (serde framework)
- ✅ 82 initial tests for validation and serialization

**Pascal Analysis (Sprint 3):**

- ✅ 114 Pascal files analyzed (39,079 lines of code)
- ✅ Dependency graph (1,070 relationships mapped)
- ✅ Risk assessment (11 critical, 27 high, 30 medium, 46 low risk units)
- ✅ 4-phase conversion roadmap with mitigation strategies
- ✅ 16 analysis documents (796KB documentation)

**Configuration System (Sprint 4):**

- ✅ impulse-config crate with hierarchical loading
- ✅ TOML + environment variable support
- ✅ 3 validation modes (config-only, strict, deployment)
- ✅ Hot-reload capability with file watching
- ✅ 37 tests covering all configuration scenarios

**Pascal Compatibility (Sprint 5):**

- ✅ 11 Pascal record modules (RECORDS.PAS conversion)
- ✅ PascalString<N> generic type (fixed-length strings)
- ✅ Binary format support for SYSTAT.DAT, USER.LST, BOARDS.DAT, UPLOADS.DAT
- ✅ 5 bitflags modules (UserFlags, BoardFlags, MenuFlags, MessageFlags, ProtocolFlags)
- ✅ PackedDateTime (Pascal 6-byte date/time format)
- ✅ 195 tests with binary round-trip verification

**User Management (Sprint 6):**

- ✅ UserManager trait (async CRUD API)
- ✅ InMemoryUserManager (HashMap-based, testing)
- ✅ FileUserManager (Pascal USER.LST binary compatibility)
- ✅ PasswordHasher (Argon2id: 19 MiB memory, 2 iterations, ~200ms)
- ✅ SessionManager (SHA-256 tokens, TTL expiry, concurrent-safe)
- ✅ User::from_pascal() / to_pascal() conversion
- ✅ 42 tests (26 impulse-user, 16 impulse-auth)

**Logging Infrastructure (Sprint 7):**

- ✅ impulse-logging crate (structured logging with tracing)
- ✅ File rotation (hourly, daily, weekly, size-based policies)
- ✅ Log archival (compression, retention management)
- ✅ Security audit logging (tamper-evident event tracking)
- ✅ Error reporting (structured context, severity levels)
- ✅ Multiple formats (JSON, human-readable)
- ✅ Integration across impulse-auth, impulse-user, impulse-config
- ✅ 80 tests (52 unit, 18 integration, 10 benchmarks)
- ✅ <2µs overhead per log event

**Testing Framework (Sprint 8):**

- ✅ Code coverage baseline: 64.51% (1018/1578 lines)
- ✅ Integration test framework (tests/common helpers)
- ✅ Property-based testing infrastructure (proptest 1.5)
- ✅ Performance benchmarking suite (criterion 0.5)
- ✅ 7 authentication benchmarks tracking critical paths
- ✅ CI integration (Codecov, artifact storage)
- ✅ Test fixtures and shared utilities
- ✅ 557+ total tests (100% passing rate)

**Phase 2 - Core Services (Sprints 9-10, November 2025):**

**User Authentication System (Sprint 9):**

- ✅ Rate limiting (sliding window algorithm, configurable limits)
- ✅ Account lockout (progressive delays, configurable thresholds)
- ✅ Input validation (username, password, email validation)
- ✅ Password strength checking (configurable complexity requirements)
- ✅ Authentication flows (login, registration, logout)
- ✅ Email validation with DNS and disposable domain checking
- ✅ Comprehensive error handling with detailed security events
- ✅ 130+ new tests (rate limiting, lockout, validation, flows)

**Menu System & Navigation (Sprint 10):**

- ✅ impulse-menu crate (complete menu framework)
- ✅ TOML-based menu configuration (4 menu files: main, files, messages, settings)
- ✅ Menu parser (item, submenu, command, separator support)
- ✅ Menu renderer (hotkey mode with numbered selections, fullmenu mode with descriptions)
- ✅ Command router (built-in commands: Quit, GoTo, Back, Up)
- ✅ Navigation state machine (history tracking, breadcrumbs)
- ✅ 84+ new tests (parser, renderer, router, state management)

**Message Base Read System (Sprint 11):**

- ✅ MessageBase trait (9 async methods: read, list, search, thread, mark read/unread, delete/undelete)
- ✅ JAM format support (.JHR/.JDT/.JDX files with CRC32 validation)
- ✅ Hudson format support (legacy compatibility)
- ✅ Message list screen (paginated display, status indicators, keyboard navigation)
- ✅ Message read screen (threaded view, word wrapping, depth indicators)
- ✅ Threading system (parent-child relationships, reply counts, conversation trees)
- ✅ 72+ new tests (42 JAM, 18 Hudson, 8 list, 4 read screen)

**Message Write System (Sprint 12):**

- ✅ MessageWriter trait (unified interface for message creation)
- ✅ Message posting (validation, sanitization, atomic writes)
- ✅ Reply functionality (thread-aware, parent tracking, depth limits)
- ✅ Message quoting (attribution, configurable prefix, multi-level support)
- ✅ Input validation (subject 1-72 chars, body max 64KB)
- ✅ Sanitization (HTML escaping, line break normalization)
- ✅ JAM format writing (.JHR updates, .JDT appends, .JDX index)
- ✅ 27+ new tests (15 posting, 8 reply, 4 quoting)

**File Area Browsing (Sprint 13):**

- ✅ FileArea and FileRecord structs with metadata
- ✅ FileAreaManager trait (async CRUD for file operations)
- ✅ InMemoryFileAreaManager implementation for testing
- ✅ File list screen (paginated, 20 per page, sortable columns)
- ✅ File details screen (extended description, FILE_ID.DIZ)
- ✅ FILE_ID.DIZ extraction from ZIP/RAR/7Z archives
- ✅ Search with wildcards (*, ?), case-insensitive
- ✅ Filtering by uploader, date range, file size
- ✅ Status indicators (new, offline, missing, popular)
- ✅ 76+ new tests (18 area, 22 list, 16 details, 20 search)

**File Upload System (Sprint 14):**

- ✅ UploadProcessor pipeline (validate → scan → extract → store → confirm)
- ✅ Multi-stage async upload processing with rollback
- ✅ File validation (size limits, duplicates, quotas, extensions, permissions)
- ✅ SHA-256 duplicate detection and prevention
- ✅ User upload quotas (per day/month/unlimited)
- ✅ ClamAV virus scanning (TCP/Unix socket, INSTREAM protocol)
- ✅ Quarantine management for infected files
- ✅ FILE_ID.DIZ extraction (ZIP/RAR/7Z archives)
- ✅ Safe temporary extraction with automatic cleanup
- ✅ Upload UI screens (prompt, progress, scanning, confirmation)
- ✅ SysOp notifications for infections
- ✅ Atomic operations with failure rollback
- ✅ 180 new tests (176 unit, 4 doc)

**User Profiles & Statistics (Sprint 15):**

- ✅ User profile display screen with comprehensive information
- ✅ Statistics tracking system (calls, uploads, downloads, posts, time online)
- ✅ Real-time statistics updates with atomic operations
- ✅ User settings editor (password, theme, terminal configuration)
- ✅ Preference persistence across sessions
- ✅ Achievement system (predefined achievement types, condition-based awarding)
- ✅ Achievement notifications and display
- ✅ Privacy controls (hide email, hide stats, hide online status)
- ✅ User directory with search and filtering
- ✅ Upload/download ratio calculation
- ✅ User signature display (custom taglines, ANSI art support)
- ✅ Last login tracking and display
- ✅ 128 new tests (82 unit, 46 doc)

### Planned Features

**Phase 2 (Sprint 16, remaining ~1 week) - Core Services**

- Sprint 16: Session management (concurrent sessions, timeouts, WebSocket)

**Phase 3 (Sprints 17-24, ~6-8 weeks) - Feature Completion**

- File transfer protocols (Zmodem, Xmodem, Ymodem)
- Theme system
- Door game interface (DOSBox integration)
- QWK offline reader support

**Phase 4 (Sprints 25-32, ~6-8 weeks) - Polish & Launch**

- Performance optimization
- Web-based administration
- Legacy data migration tools
- Production deployment support

### Modern Enhancements

- **Multi-Protocol Support**: Telnet, SSH (planned), WebSocket (planned), REST API (planned)
- **Async Architecture**: Tokio 1.47 async runtime for concurrent session handling
- **Modern Storage**: SQLite/PostgreSQL (planned) with Pascal binary format compatibility
- **Security**: Argon2id password hashing (19 MiB, 2 iterations), SHA-256 session tokens, audit logging
- **Cloud-Ready**: Docker, Kubernetes, containerized deployment (planned)
- **Observability**: Structured logging (tracing), file rotation, log archival, benchmarking (Prometheus planned)
- **Testing**: 64.51% coverage baseline, integration tests, property-based testing, performance benchmarks

## Architecture

### High-Level Design

```
┌─────────────────────────────────────────────────────────┐
│                     Impulse BBS System                  │
│                                                         │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐   │
│  │   Telnet/    │  │     SSH      │  │  HTTP/REST   │   │
│  │  Serial Port │  │   Server     │  │     API      │   │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘   │
│         │                 │                 │           │
│         └─────────────────┴─────────────────┘           │
│                            │                            │
│                 ┌──────────▼──────────┐                 │
│                 │  Session Manager    │                 │
│                 │   (Async/Tokio)     │                 │
│                 └──────────┬──────────┘                 │
│                            │                            │
│         ┌──────────────────┼──────────────────┐         │
│         │                  │                  │         │
│  ┌──────▼───────┐  ┌──────▼───────┐  ┌────────▼───────┐ │
│  │ Terminal I/O │  │   Message    │  │  File Transfer │ │
│  │  Subsystem   │  │   Subsystem  │  │   Subsystem    │ │
│  └──────┬───────┘  └──────┬───────┘  └───────┬────────┘ │
│         │                  │                  │         │
│         └──────────────────┴──────────────────┘         │
│                            │                            │
│                 ┌──────────▼──────────┐                 │
│                 │   Storage Layer     │                 │
│                 │  (SQLite/Postgres)  │                 │
│                 └─────────────────────┘                 │
└─────────────────────────────────────────────────────────┘
```

### 19-Crate Workspace Structure

**Core Crates:**

- `impulse-core` - Core BBS logic and state management
- `impulse-types` - Shared data types and error handling
- `impulse-config` - Configuration management
- `impulse-logging` - Structured logging, file rotation, audit trails

**Protocol Crates:**

- `impulse-protocol` - Protocol trait definitions
- `impulse-telnet` - Telnet protocol implementation
- `impulse-ssh` - SSH protocol implementation

**Feature Crates:**

- `impulse-session` - Session management and event loops
- `impulse-terminal` - Terminal I/O and ANSI rendering
- `impulse-auth` - Authentication (Argon2id, rate limiting, lockout)
- `impulse-message` - Message bases (JAM/Hudson)
- `impulse-file` - File areas and transfers
- `impulse-user` - User management
- `impulse-door` - Door game support
- `impulse-menu` - Menu system (TOML parser, renderer, navigation)

**Application Crates:**

- `impulse-web` - Web admin panel (Axum)
- `impulse-server` - Main server binary
- `impconfig` - Configuration management CLI tool (binary)

See [docs/architecture/system-architecture.md](docs/architecture/system-architecture.md) for complete architecture documentation.

## Quick Start

### Prerequisites

- **Rust**: 1.80 or later ([Install Rust](https://www.rust-lang.org/tools/install))
- **Git**: 2.30 or later
- **Platform**: Linux, Windows 11, or macOS

### Build Instructions

```bash
# Clone the repository
git clone https://github.com/doublegate/Impulse-Next_BBS.git
cd Impulse-Next_BBS

# Build all crates
cargo build --workspace

# Run tests
cargo test --workspace

# Build optimized release
cargo build --workspace --release
```

### Running the BBS Server

```bash
# Run the main server (when implemented)
cargo run --bin impulse-server

# Or run the release build
./target/release/impulse-server
```

### Generate Documentation

```bash
# Generate and open API documentation
cargo doc --workspace --no-deps --open
```

## Development

### Development Workflow

```bash
# Format code
cargo fmt --all

# Run linter
cargo clippy --all-targets --all-features -- -D warnings

# Run tests with output
cargo test --workspace -- --nocapture

# Run tests for specific crate
cargo test -p impulse-types

# Check compilation without building
cargo check --workspace
```

### Code Quality Standards

All code must:

1. **Compile without warnings**: `cargo clippy` passes with 0 warnings
2. **Be properly formatted**: `cargo fmt --all` applied
3. **Include tests**: Unit tests for all public APIs
4. **Have documentation**: Rustdoc comments on all public items
5. **Pass CI**: GitHub Actions workflow completes successfully

### CI/CD Pipeline

The project uses GitHub Actions with 5 jobs:

1. **Lint**: `cargo clippy` (0 warnings enforced) + `cargo fmt` checks
2. **Test**: `cargo test --workspace` on 3 platforms (Linux, Windows, macOS)
3. **Build**: `cargo build --workspace --release` on 3 platforms
4. **Coverage**: cargo-tarpaulin 0.31 + Codecov integration (baseline: 64.51%)
5. **Benchmark**: criterion benchmarks with artifact storage (7 auth benchmarks)

**Platform Matrix:** ubuntu-latest, windows-latest, macos-latest
**Cache Strategy:** Swatinem/rust-cache@v2 for faster builds
**Artifacts:** Coverage reports (HTML), benchmark results (30-day retention)

### Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for:

- Code of conduct
- Development workflow
- Coding standards
- Testing requirements
- Pull request process
- Commit message guidelines

## Project Structure

```
Impulse-Next_BBS/
├── Cargo.toml              # Workspace root
├── crates/                 # All crates
│   ├── impulse-core/       # Core BBS logic
│   ├── impulse-types/      # Shared types and errors
│   ├── impulse-config/     # Configuration
│   ├── impulse-protocol/   # Protocol traits
│   ├── impulse-telnet/     # Telnet server
│   ├── impulse-ssh/        # SSH server
│   ├── impulse-session/    # Session management
│   ├── impulse-terminal/   # Terminal I/O
│   ├── impulse-auth/       # Authentication
│   ├── impulse-message/    # Message bases
│   ├── impulse-file/       # File areas
│   ├── impulse-user/       # User management
│   ├── impulse-door/       # Door games
│   ├── impulse-menu/       # Menu system (NEW)
│   ├── impulse-web/        # Web admin panel
│   ├── impulse-logging/    # Logging infrastructure
│   ├── impulse-server/     # Main server binary
│   └── impconfig/          # CLI configuration tool (binary)
├── config/                 # Configuration files
│   └── menus/              # Menu definitions (NEW)
│       ├── main.toml       # Main menu
│       ├── files.toml      # File areas menu
│       ├── messages.toml   # Message areas menu
│       └── settings.toml   # User settings menu
├── docs/                   # Comprehensive documentation (43 files)
│   ├── INDEX.md            # Documentation hub
│   ├── getting-started/    # Project overview (1 file)
│   ├── architecture/       # System design (3 files)
│   ├── planning/           # Phase and sprint plans (2 files)
│   ├── implementation/     # Development guides (2 files)
│   ├── testing/            # Testing strategy (1 file)
│   ├── deployment/         # Deployment guides (2 files)
│   ├── pascal-reference/   # Pascal analysis (21 files)
│   │   ├── analysis/       # Source analysis (7 docs + 5 data)
│   │   ├── conversion/     # Conversion guides (6 docs)
│   │   └── risk-assessment/ # Risk analysis (3 docs)
│   ├── reports/            # Analysis reports (9 files)
│   │   ├── ci-cd/          # CI/CD reports (2 docs)
│   │   ├── documentation/  # Doc verification (3 docs)
│   │   ├── edition2024/    # Rust 2024 migration (2 docs)
│   │   └── sprints/        # Sprint completions (3 docs)
│   └── reference/          # Historical context (2 files)
├── to-dos/                 # Sprint TODO files (32 sprints)
│   ├── phase-1-foundation/
│   ├── phase-2-core-features/
│   ├── phase-3-feature-completion/
│   └── phase-4-polish-launch/
├── ref-docs/               # Reference documentation
│   ├── impulse-history.md
│   └── rust-conversion-technical.md
├── .github/workflows/      # CI/CD configuration
│   └── ci.yml
├── CONTRIBUTING.md         # Contribution guidelines
├── CHANGELOG.md            # Version history
├── README.md               # This file
└── LICENSE-*               # Dual licensing
```

## Technology Stack

### Core Technologies

- **Language**: Rust 2024 edition
- **Minimum Version**: Rust 1.85+
- **Async Runtime**: Tokio 1.47

### Key Dependencies

**Production:**

- `tokio` 1.47 - Async runtime (full features)
- `tokio-util` 0.7 - Async utilities
- `crossterm` 0.29 - Terminal I/O
- `serde` 1.0 - Serialization framework
- `serde_json` 1.0 - JSON support
- `toml` 0.9 - Configuration files
- `binrw` 0.15 - Binary parsing
- `thiserror` 2.0 - Error handling
- `anyhow` 1.0 - Error context
- `tracing` 0.1 - Structured logging
- `argon2` 0.5 - Password hashing
- `sha2` 0.10 - Session token generation
- `rand` 0.8 - Secure randomness
- `sqlx` 0.8 - Database access
- `axum` 0.8 - Web framework
- `bitflags` 2.6 - Pascal set types

**Development:**

- `proptest` 1.5 - Property-based testing
- `serial_test` 3.0 - Test isolation
- `tempfile` 3.8 - Temporary file handling

**Build Optimization:**

- LTO enabled in release builds
- Single codegen unit for maximum optimization
- Debug symbols stripped in release

## Documentation

### Documentation Hub

**Start Here**: [docs/INDEX.md](docs/INDEX.md) - Your complete guide to all project documentation.

All documentation has been reorganized into 9 logical categories with 43 files (38 markdown + 5 data files) totaling over 31,000 lines of comprehensive coverage.

### Key Documentation by Category

**Getting Started:**

- [Project Overview](docs/getting-started/project-overview.md) - Vision, objectives, stakeholders

**Architecture:**

- [System Architecture](docs/architecture/system-architecture.md) - Overall system design
- [Security Architecture](docs/architecture/security-architecture.md) - Security design and threat model
- [Data Model](docs/architecture/data-model.md) - Database schema and relationships

**Planning:**

- [Phase & Sprint Plan](docs/planning/phase-sprint-plan.md) - Complete 32-sprint roadmap
- [Conversion Strategy](docs/planning/conversion-strategy.md) - Pascal→Rust migration approach

**Implementation:**

- [Development Guide](docs/implementation/development-guide.md) - Developer onboarding
- [Logging Integration](docs/implementation/logging-integration.md) - Structured logging guide

**Testing:**

- [Testing Strategy](docs/testing/testing-strategy.md) - Comprehensive testing methodology

**Deployment:**

- [Deployment Guide](docs/deployment/deployment-guide.md) - Docker, Kubernetes, production setup
- [Migration Guide](docs/deployment/migration-guide.md) - Legacy data migration

**Pascal Reference:**

- [Analysis Reports](docs/pascal-reference/analysis/) - 7 analysis documents + 5 data files
- [Conversion Guides](docs/pascal-reference/conversion/) - 6 module-specific guides
- [Risk Assessments](docs/pascal-reference/risk-assessment/) - 3 risk analysis documents

**Reports:**

- [Sprint Completions](docs/reports/sprints/) - Phase 1 sprint reports
- [CI/CD Analysis](docs/reports/ci-cd/) - Pipeline optimization reports
- [Documentation Verification](docs/reports/documentation/) - Link checking, verification

**Reference:**

- [Impulse History](docs/reference/impulse-history.md) - BBS history and cultural context

### Sprint TODO Files (to-dos/)

Detailed sprint plans for all 32 sprints:

- **Phase 1** (Sprints 1-8): Foundation
- **Phase 2** (Sprints 9-16): Core Features
- **Phase 3** (Sprints 17-24): Feature Completion
- **Phase 4** (Sprints 25-32): Polish & Launch

**Total**: 19,214 lines across 30 files with 93 Rust code examples

### Additional Resources

For historical context and technical details:

- See [docs/reference/](docs/reference/) for BBS history and cultural context
- See [docs/pascal-reference/](docs/pascal-reference/) for complete Pascal analysis and conversion guides

### API Documentation

Generate API documentation:

```bash
cargo doc --workspace --no-deps --open
```

## Roadmap

### 4 Phases, 24 Months, 32 Sprints

**Phase 1: Foundation (November 2025, Sprints 1-8) - ✅ COMPLETE**

- ✅ Sprint 1: Project setup (18-crate workspace, CI/CD)
- ✅ Sprint 2: Core type system (User, FileEntry, Message, BbsConfig)
- ✅ Sprint 3: Pascal analysis (114 files, 1,070 dependencies)
- ✅ Sprint 4: Configuration system (TOML + ENV, hot-reload)
- ✅ Sprint 5: RECORDS.PAS conversion (11 modules, binary compatibility)
- ✅ Sprint 6: User system (impulse-user, impulse-auth, Argon2id)
- ✅ Sprint 7: Logging infrastructure (rotation, archival, audit)
- ✅ Sprint 8: Testing framework (64.51% coverage, benchmarks)

**Phase 2: Core Features (November 2025 - December 2025, Sprints 9-16)**

- ✅ Sprint 9: User authentication (rate limiting, lockout, validation, flows)
- ✅ Sprint 10: Menu system (TOML parser, renderer, navigation state machine)
- ✅ Sprint 11: Message read (MessageBase trait, JAM/Hudson formats, screens, threading)
- ✅ Sprint 12: Message write (posting, replies, quoting, validation, atomic writes)
- ✅ Sprint 13: File browsing (file areas, list/details screens, search, FILE_ID.DIZ)
- ✅ Sprint 14: File upload (processor, ClamAV scanning, validation, quarantine, DIZ extraction)
- ✅ Sprint 15: User profiles (profile display, stats tracking, settings, achievements, privacy)
- Sprint 16: Session management (WebSocket, concurrent handling, timeouts)

**Phase 3: Feature Completion (February - March 2026, Sprints 17-24)**

- Zmodem and file transfer protocols
- Theme system
- Door game interface
- Advanced message features
- Administration interface

**Phase 4: Polish & Launch (April - May 2026, Sprints 25-32)**

- Performance optimization
- Comprehensive documentation
- Legacy migration tools
- Web-based administration
- Beta testing and bug fixes
- Public 1.0 release

### Key Milestones

| Milestone         | Target   | Status              | Completion              |
| ----------------- | -------- | ------------------- | ----------------------- |
| Phase 1 Complete  | Month 6  | ✅ **COMPLETE**     | November 2025 (6 weeks) |
| Phase 2 Complete  | Month 12 | 🔄 In Progress 87.5%| Target: December 2025   |
| Phase 3 Complete  | Month 18 | Pending             | Target: February 2026   |
| Phase 4 Complete  | Month 24 | Pending             | Target: April 2026      |
| Production Launch | Month 24 | Pending             | Target: April 2026      |

**Progress:** 46.88% complete (15/32 sprints), ~15 weeks ahead of schedule

## Testing

### Current Test Suite (Phase 1 + Sprints 9-15)

**Total Tests**: 1,254+ (100% passing rate)
**Code Coverage**: 64.51% baseline (target: 75%+ by Phase 2 completion)

**Test Types:**

- **Unit Tests**: 720+ tests (validation logic, CRUD, authentication, logging, message I/O, menu parsing, file management)
- **Integration Tests**: 190+ tests (serialization, file I/O, sessions, cross-crate workflows, message threading, file searching)
- **Doc Tests**: 50+ tests (documentation examples)
- **Benchmarks**: 7 performance benchmarks (authentication critical paths)

**Test Breakdown by Crate:**

- impulse-types: 241 tests (Pascal compatibility, core types, serialization)
- impulse-auth: 146+ tests (hashing, sessions, rate limiting, lockout, validation, flows)
- impulse-message: 99+ tests (42 JAM format, 18 Hudson, 8 list screen, 4 read screen, 15 posting, 8 reply, 4 quoting)
- impulse-file: 256+ tests (18 area, 22 list, 16 details, 20 search, 45 upload, 35 validation, 28 scanning, 32 DIZ, 20 UI)
- impulse-user: 161+ tests (33 CRUD/file I/O, 128 new profile/stats/settings/achievements tests)
- impulse-menu: 84+ tests (parser, renderer, router, navigation state machine)
- impulse-logging: 80 tests (52 unit, 18 integration, 10 benchmarks)
- impulse-config: 37 tests (configuration, validation, hot-reload)
- Other crates: 150+ tests (protocols, terminal, door, web)

**Coverage by Crate:**

- impulse-types: 81.23% (highest coverage)
- impulse-auth: 75.89%
- impulse-user: 72.45%
- impulse-config: 68.12%
- impulse-logging: 65.34%
- Overall workspace: 64.51%

**Target for Phase 2**: 75%+ coverage across all crates

### Running Tests

```bash
# Run all tests
cargo test --workspace

# Run with output
cargo test --workspace -- --nocapture

# Run specific crate tests
cargo test -p impulse-types

# Run doc tests only
cargo test --workspace --doc
```

### Testing Strategy

See [docs/testing/testing-strategy.md](docs/testing/testing-strategy.md) for:

- Unit testing approach
- Integration testing
- Property-based testing
- Performance benchmarking
- Coverage targets (80%+ goal)

## Contributing

We welcome contributions from the community! Whether you're interested in:

- Fixing bugs
- Adding features
- Improving documentation
- Writing tests
- Optimizing performance

Please read our [CONTRIBUTING.md](CONTRIBUTING.md) for:

- Development workflow
- Coding standards
- Pull request process
- Commit message guidelines

### Getting Started

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature-name`
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass: `cargo test --workspace`
6. Run linter: `cargo clippy --all-targets --all-features -- -D warnings`
7. Format code: `cargo fmt --all`
8. Submit a pull request

## License

This project is dual-licensed under:

- **MIT License** ([LICENSE-MIT](LICENSE-MIT))
- **Apache License 2.0** ([LICENSE-APACHE](LICENSE-APACHE))

You may choose either license for your use.

## Acknowledgments

### Original Impulse Developers

Credit to the original developers who created Impulse BBS:

- **Brandon Sneed (Nivenh)**: Original developer through Version 6
- **Phillip Foose (Horrid)**: Further bugfixes and Version 7

### Community

Thanks to:

- The retro-computing and BBS preservation communities
- The Rust programming language community
- Contributors to this modernization project
- Digital preservationists keeping BBS history alive

### Technologies

Built with excellent open-source technologies:

- [Rust Programming Language](https://www.rust-lang.org)
- [Tokio](https://tokio.rs) - Async runtime
- [Serde](https://serde.rs) - Serialization framework
- [crossterm](https://github.com/crossterm-rs/crossterm) - Terminal manipulation
- [SQLx](https://github.com/launchbadge/sqlx) - SQL toolkit
- And many other amazing crates

### Historical Resources

- [BBS Documentary](http://www.bbsdocumentary.com)
- [Impulse Archive](https://web.archive.org/web/20011204010133/http://www.demonic.net/impulse/)
- [textfiles.com](http://textfiles.com) - BBS history preservation

## Contact & Links

- **Repository**: [https://github.com/doublegate/Impulse-Next_BBS](https://github.com/doublegate/Impulse-Next_BBS)
- **Issues**: [GitHub Issues](https://github.com/doublegate/Impulse-Next_BBS/issues)
- **Discussions**: [GitHub Discussions](https://github.com/doublegate/Impulse-Next_BBS/discussions)
- **Documentation Hub**: [docs/INDEX.md](docs/INDEX.md)

---

**"We're figuring it out!"** - Preserving BBS history, one commit at a time.

_For detailed sprint plans, architecture decisions, and technical specifications, see the comprehensive documentation in the [docs/](docs/) directory. Start with [docs/INDEX.md](docs/INDEX.md) for guided navigation._
