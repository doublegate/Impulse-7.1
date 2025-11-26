# CLAUDE.md - Impulse-Next_BBS Project Memory

Project-specific guidance for Impulse-Next_BBS modernization (classic Impulse 7.1 BBS: Borland Pascal 7.0 → Rust 2024).

**Version:** 0.9.0 | **Updated:** 2025-11-26

---

## Project Overview

**Repository:** https://github.com/doublegate/Impulse-Next_BBS
**Type:** Rust 2024 edition workspace (21 crates: 18 libraries + 3 binaries)
**Goal:** Next-generation BBS software - modernizing the classic Impulse 7.1 BBS from Borland Pascal to Rust for cross-platform operation
**License:** MIT OR Apache-2.0

---

## Current Status

**Phase:** 4 - Polish & Launch (Sprint 25 Starting - Performance Optimization)
**Sprints Complete:** 24 of 32 (75.0%)
**Version:** 0.9.0 (Phases 1, 2, 3: 100% COMPLETE - All Advanced Features Implemented)
**Last Commit:** 87ecdc3 (2025-11-26 - Phase 3 Server Integration Complete)

### Sprint Progress
- ✅ **Phase 1:** Foundation (Sprints 1-8, 100%)
- ✅ **Phase 2:** Core Features (Sprints 9-16, 100%)
- ✅ **Sprint 9:** User Authentication (100%)
- ✅ **Sprint 10:** Menu System (100%)
- ✅ **Sprint 11:** Message Read (100%)
- ✅ **Sprint 12:** Message Write (100%)
- ✅ **Sprint 13:** File Browsing (100%)
- ✅ **Sprint 14:** File Upload (100%)
- ✅ **Sprint 15:** User Profiles & Statistics (100%)
- ✅ **Sprint 16:** Integration & Testing (100%)
- ✅ **Server Infrastructure:** Telnet, Session, Terminal, Server (Post Phase 2)
- ✅ **Sprint 16 (Session Management):** Concurrent sessions, conflict resolution, timeouts, WebSocket (100%)
- ✅ **CI/CD Fixes (2025-11-26):** MSRV 1.88, bincode 2.0, Rust 2024 let-chains (19 files), cargo-audit 0.22
- ✅ **Sprint 17 (Zmodem Protocol):** Frame structure, CRC-16/32, handshake, file transfer, crash recovery, batch mode (236 tests)
- ✅ **Sprint 18 (Xmodem/Ymodem):** Xmodem (checksum/CRC/1K), Ymodem batch mode, error recovery (112 tests)
- ✅ **Sprint 19 (Protocol Completion):** Ymodem-G streaming, auto-detection, user preferences (108 tests)
- ✅ **Sprint 20 (Theme System):** Theme architecture, 3 default themes (Classic, Matrix, Cyberpunk), color schemes (62 tests)
- ✅ **Sprint 21 (Door Game Interface):** DOOR.SYS/DORINFO1.DEF dropfiles, door manager, executor with DOSBox support, async I/O (126 tests)
- ✅ **Sprint 22 (Advanced Messaging):** QWK offline mail, message import/export, FidoNet addressing, routing infrastructure (79 tests)
- ✅ **Sprint 23 (Administration):** SysOp admin interface, user/file/system management, access control, audit logging (149 tests)
- ✅ **Sprint 24 (Integration Testing):** End-to-end journeys, security audits, load testing, cross-crate tests (83 tests)
- ✅ **Phase 3 COMPLETE:** All 8 sprints (17-24) finished, 100%
- 📋 **Sprints 25-32:** Phase 4 - Polish & Deployment (Continuing)

### Quality Metrics
- **Tests:** 2,165 passing (100% pass rate)
- **Coverage:** 75.43% achieved (target: 75%+ - GOAL MET!)
- **Clippy:** 0 warnings
- **CI/CD:** 12 jobs, 100% passing on main branch
- **Crates:** 21 (18 libraries + 3 binaries)
- **Code:** ~71,987 lines total (46,750 code + 12,035 comments + 12,814 blank)
- **Commits:** 138 total
- **Build Time:** <2s dev, <10s release
- **Test Execution:** <12s all tests

---

## Project Structure

### Workspace Layout
```
Impulse-Next_BBS/
├── crates/              # 21 crates (18 libraries + 3 binaries)
│   ├── impulse-core/    # Core BBS functionality
│   ├── impulse-types/   # Type definitions (User, FileEntry, Message, BbsConfig)
│   ├── impulse-config/  # Configuration management (TOML + ENV, hot-reload)
│   ├── impulse-protocol/# Protocol implementations (Zmodem, Xmodem, Ymodem)
│   ├── impulse-telnet/  # Telnet server (RFC 854)
│   ├── impulse-ssh/     # SSH server (RFC 4253, 4254)
│   ├── impulse-session/ # Session management (concurrent, timeouts, WebSocket)
│   ├── impulse-terminal/# Terminal handling (ANSI/Avatar/RIP, themes)
│   ├── impulse-auth/    # Authentication (Argon2id, rate limiting, lockout)
│   ├── impulse-message/ # Message system (JAM/Hudson, QWK, FidoNet)
│   ├── impulse-file/    # File management (upload/download, ClamAV, FILE_ID.DIZ)
│   ├── impulse-user/    # User management (profiles, stats, achievements)
│   ├── impulse-door/    # Door game interface (DOOR.SYS, DORINFO1.DEF, DOSBox)
│   ├── impulse-menu/    # Menu system (TOML parser, navigation)
│   ├── impulse-admin/   # Administration (access control, audit logging)
│   ├── impulse-web/     # Web admin panel (planned Sprint 27)
│   ├── impulse-logging/ # Logging (rotation, archival, audit trails)
│   ├── impulse-integration-tests/ # Integration test suite
│   ├── impulse-server/  # Main BBS server (binary)
│   ├── impulse-cli/     # CLI management tool (binary)
│   └── impconfig/       # Configuration CLI (binary)
├── docs/                # 67+ documentation files
│   ├── architecture/    # System design docs
│   ├── implementation/  # Implementation guides
│   ├── planning/        # Phase/sprint planning (5 files)
│   ├── reports/         # Analysis reports (CI/CD, sprints, docs)
│   ├── pascal-reference/# Pascal analysis (21 files)
│   ├── retrospectives/  # Phase retrospectives
│   └── reference/       # Historical context
├── to-dos/              # Sprint TODO files (32 sprints)
│   ├── phase-1-foundation/        # Sprints 1-8 (COMPLETE)
│   ├── phase-2-core-features/     # Sprints 9-16 (COMPLETE)
│   ├── phase-3-feature-completion/# Sprints 17-24 (COMPLETE)
│   └── phase-4-polish-launch/     # Sprints 25-32 (PLANNED)
├── .github/workflows/   # CI/CD configuration (12-job pipeline)
├── Cargo.toml           # Workspace manifest (21 crates)
├── README.md            # Project documentation
├── CHANGELOG.md         # Version history (Keep a Changelog format)
├── CONTRIBUTING.md      # Contribution guidelines (336 lines)
├── CLAUDE.md            # This file - Project memory bank
└── CLAUDE.local.md      # Current session state
```

### Key Crates

**Core Libraries:**
- `impulse-types` - Fundamental types (User, FileEntry, Message, BbsConfig, error handling)
- `impulse-core` - Core BBS functionality and utilities
- `impulse-config` - Configuration loading, validation, defaults

**Protocol Handlers:**
- `impulse-telnet` - Telnet server (RFC 854, 857, 858, 1073)
- `impulse-ssh` - SSH server (RFC 4253, 4254)
- `impulse-protocol` - Protocol abstraction layer

**Services:**
- `impulse-session` - Session management, timeouts, state
- `impulse-auth` - Authentication (Argon2, TOTP, session tokens)
- `impulse-message` - Message base, email, networking
- `impulse-file` - File areas, uploads, downloads
- `impulse-user` - User management, security levels
- `impulse-door` - Door game interface (dropfiles, FOSSIL)

**UI/Admin:**
- `impulse-terminal` - Terminal emulation (ANSI, Avatar, RIP)
- `impulse-web` - Web admin panel (Axum framework)

**Testing:**
- `impulse-integration-tests` - Phase 3 integration testing suite

**Executables:**
- `impulse-server` - Main BBS server
- `impulse-cli` - Command-line management tool

---

## Technology Stack

### Core Dependencies
| Dependency | Version | Purpose |
|------------|---------|---------|
| Rust | 1.88+ (2024 edition) | Primary language (MSRV) |
| Tokio | 1.47+ | Async runtime |
| crossterm | 0.29 | Terminal I/O |
| bincode | 2.0 | Binary serialization |
| rand | 0.9 | Secure randomness |
| colored | 3.0 | Terminal colors |
| notify | 8.2 | File system watching |
| SQLx | 0.8 | Database (SQLite/PostgreSQL) |
| Axum | 0.7 | Web framework |
| serde | 1.0 | Serialization (JSON, bincode) |
| thiserror | 2.0 | Error handling |
| anyhow | 1.0 | Error context |
| argon2 | 0.5 | Password hashing |
| tracing | 0.1 | Logging |

### Development Tools
| Tool | Purpose |
|------|---------|
| cargo-tarpaulin | Code coverage |
| cargo-audit | Security audits |
| clippy | Linting |
| rustfmt | Code formatting |

---

## Development Practices

### Git Workflow
1. **Branch Strategy:** Feature branches from main
2. **Commit Convention:** Conventional commits (feat:, fix:, docs:, test:, refactor:, chore:)
3. **Co-Authorship:** All commits include Claude Code co-authorship
4. **Never Commit Unless Asked:** Explicit user request required

### Testing Strategy
1. **Unit Tests:** Test individual components in isolation
2. **Integration Tests:** Test component interactions
3. **Serialization Tests:** Verify JSON/bincode round-trips
4. **Current Coverage:** 82 tests across impulse-types crate

### Quality Checks (Pre-Commit)
```bash
# Format check
cargo fmt --all -- --check

# Lint check
cargo clippy --all-targets --all-features -- -D warnings

# Test suite
cargo test --workspace --all-features --verbose

# Build check
cargo build --workspace --all-features
```

### CI/CD Pipeline
**Location:** `.github/workflows/ci.yml`

**Jobs:**
1. **Lint** - rustfmt + clippy
2. **Test** - 3 platforms (Linux, Windows, macOS)
3. **Build** - 3 platforms with release profile
4. **Coverage** - tarpaulin coverage report → Codecov

**CI Status:**
- All 12 jobs passing (lint, test×3, build×3, coverage, benchmark, audit, MSRV, gate)
- cargo-audit 0.22.0 (Cargo.lock v4 support)
- MSRV testing (Rust 1.88)
- Rust 2024 let-chains syntax across 19 files

### Dependabot
**Configuration:** `.github/dependabot.yml`
- **Schedule:** Weekly (Mondays)
- **Grouping:** Related packages (tokio*, serde*)
- **Limits:** 10 Cargo PRs, 5 GitHub Actions PRs

---

## Sprint Documentation

### Completed Sprints

#### Sprint 1: Project Setup
**TODO:** `to-dos/phase-1-foundation/sprint-01-project-setup.md`

**Deliverables:**
- ✅ 16-crate Rust workspace
- ✅ CI/CD pipeline (GitHub Actions)
- ✅ CONTRIBUTING.md (336 lines)
- ✅ Dual MIT/Apache-2.0 licensing
- ✅ Cross-platform testing
- ✅ 48+ documentation files

#### Sprint 2: Core Types
**TODO:** `to-dos/phase-1-foundation/sprint-02-core-types.md`

**Deliverables:**
- ✅ User type (265 lines, 13 fields, 10 tests)
- ✅ FileEntry type (293 lines, 13 fields, 10 tests)
- ✅ Message type (214 lines, 11 fields, 11 tests)
- ✅ BbsConfig type (502 lines, nested structure, 13 tests)
- ✅ Error handling (117 lines, 15 error variants)
- ✅ Serialization tests (372 lines, 11 round-trip tests)

### Recent Sprints (Phase 2)

#### Sprint 11: Message Read
**TODO:** `to-dos/phase-2-core-features/sprint-11-message-read.md`

**Deliverables:**
- ✅ MessageBase trait (9 async methods)
- ✅ JAM format support (.JHR, .JDT, .JDX)
- ✅ Hudson format support
- ✅ Message list screen
- ✅ Message read screen with threading
- ✅ 72+ tests (42 JAM, 18 Hudson, 12 screens)

#### Sprint 12: Message Write
**TODO:** `to-dos/phase-2-core-features/sprint-12-message-write.md`

**Deliverables:**
- ✅ MessageWriter trait
- ✅ Message posting (validation, sanitization)
- ✅ Reply functionality with threading
- ✅ Message quoting with attribution
- ✅ JAM format writing
- ✅ 27+ tests (15 posting, 8 reply, 4 quoting)

#### Sprint 13: File Browsing
**TODO:** `to-dos/phase-2-core-features/sprint-13-file-browsing.md`

**Deliverables:**
- ✅ FileArea and FileRecord structs
- ✅ FileAreaManager trait with InMemory implementation
- ✅ File list screen (paginated, sortable)
- ✅ File details screen with FILE_ID.DIZ extraction
- ✅ Search with wildcards, date/size filters
- ✅ 76+ tests (18 area, 22 list, 16 details, 20 search)

#### Sprint 14: File Upload
**TODO:** `to-dos/phase-2-core-features/sprint-14-file-upload.md`

**Deliverables:**
- ✅ UploadProcessor pipeline with rollback
- ✅ File validation (size, duplicates, quotas, extensions, permissions)
- ✅ ClamAV virus scanning with quarantine
- ✅ FILE_ID.DIZ extraction (ZIP/RAR/7Z)
- ✅ Upload UI screens (prompt, progress, scanning, confirmation)
- ✅ 180 tests (45 upload, 35 validation, 28 scanning, 32 DIZ, 20 UI)

#### Sprint 15: User Profiles & Statistics
**TODO:** `to-dos/phase-2-core-features/sprint-15-user-profiles.md`

**Deliverables:**
- ✅ User profile display screen
- ✅ Statistics tracking (calls, uploads, downloads, posts, time online)
- ✅ User settings editor (password, theme, terminal config)
- ✅ Achievement system with notifications
- ✅ Privacy controls (hide email, stats, online status)
- ✅ User directory with search and filtering
- ✅ 128 tests (82 unit, 46 doc)

### Server Infrastructure (Post Phase 2)

#### Server Infrastructure Implementation
**Commit:** ebd1305 (2025-11-26)

**Deliverables:**
- ✅ impulse-server - Main BBS server binary (285 lines)
  - Async Tokio runtime with telnet listener on port 2323
  - Connection acceptance and session spawning
  - Graceful shutdown handling
- ✅ impulse-telnet - RFC 854 Telnet Protocol (764 lines, 40 tests)
  - TelnetServer, TelnetConnection, IAC negotiation
  - Telnet options: ECHO, SUPPRESS_GO_AHEAD, TERMINAL_TYPE, NAWS
- ✅ impulse-session - Session Management Base (747 lines, 11 tests)
  - SessionId (UUID), SessionState, SessionManager
  - Concurrent tracking, automatic expiry, CRUD operations
- ✅ impulse-terminal - ANSI Terminal (725 lines, 16 tests)
  - Color enum (16/256/RGB), AnsiSequence, AnsiRenderer
  - Cursor/screen control, text styling

### Sprint 16: Session Management (2025-11-26)

#### Session Management Enhancement
**Commit:** 2bf5b8e (2025-11-26)
**Sprint:** Sprint 16 - Session Management

**Deliverables:**
- ✅ **Concurrent Session Management**
  - Per-user session limits (default: 3, configurable)
  - System-wide total session limit (default: 100)
  - Conflict resolution policies: Allow, KickOldest, DenyNew
  - Automatic conflict detection and resolution
  - Session history tracking

- ✅ **Timeout Management System**
  - Idle timeout (default: 15 minutes, configurable)
  - Absolute timeout (default: 4 hours, optional/unlimited)
  - Timeout warning system (default: 1 minute before timeout)
  - Unlimited session time for privileged users (sysop whitelist)
  - Warning state tracking to prevent duplicate notifications

- ✅ **Connection Abstraction**
  - Connection trait for protocol-agnostic operations
  - ConnectionType enum: Telnet, WebSocket, SSH
  - Unified send/receive interface for all transports
  - ConnectionError type for error handling

- ✅ **WebSocket Support** (feature-gated)
  - WebSocketConnection implementation with tokio-tungstenite
  - BbsMessage JSON protocol for structured communication
  - SessionEvent notifications (NewMail, ChatRequest, TimeoutWarning, Terminated)
  - Ping/pong keepalive handling
  - Async send/receive with futures

- ✅ **Who's Online Functionality**
  - list_all_sessions() - Get all active sessions
  - list_sessions_filtered() - Filter by username, state, connection type
  - Session details: username, location, activity, duration
  - Real-time activity status display
  - Privacy controls for user visibility

**Tests Added:** 31 tests (29 unit + 2 doc tests)
**Code Added:** ~2,100 lines (production + tests)
**Modules:**
- config.rs: Enhanced with ConflictPolicy, timeouts, unlimited users
- session.rs: Enhanced with warning tracking and timeout detection
- manager.rs: Enhanced with conflict resolution and filtering
- connection.rs: NEW - Connection trait and types
- websocket.rs: NEW - WebSocket implementation

### Sprint 24: Phase 3 Integration Testing (2025-11-26)

#### Integration Testing Implementation
**Commit:** ca8c1a7 (2025-11-26)
**Sprint:** Sprint 24 - Phase 3 Integration Testing

**Deliverables:**
- ✅ **New Crate: impulse-integration-tests** (~3,148 lines, 83 tests)
  - Comprehensive Phase 3 integration testing suite
  - Test fixtures, user factories, mock BBS environments

- ✅ **Test Fixtures** (`fixtures/`)
  - BbsTestFixture: Complete BBS environment setup
  - UserFactory: Test user creation with various security levels
  - In-memory storage for test isolation

- ✅ **User Journey Tests** (`journeys/`)
  - Complete workflow scenarios (login → download → logout)
  - Multi-step interaction testing
  - State persistence verification

- ✅ **Security Audit Tests** (`security/`)
  - SQL injection prevention testing
  - Path traversal attack prevention
  - Authentication bypass attempts
  - File upload security validation
  - Input validation edge cases

- ✅ **Load Testing** (`stress/`)
  - LoadGenerator: Concurrent user simulation
  - LoadMetrics: Performance measurement
  - Configurable user counts and durations
  - Criterion benchmarks

- ✅ **Cross-Crate Tests** (`cross_crate/`)
  - Protocol integration (Zmodem, Xmodem, Ymodem)
  - Door game integration
  - Message system integration
  - Admin interface integration

**Tests Added:** 83 tests
**Code Added:** ~3,148 lines
**Files Created:** 22 files

---

## Common Commands

### Development
```bash
# Start development
cargo build --workspace

# Run tests
cargo test --workspace --verbose

# Watch mode (requires cargo-watch)
cargo watch -x test

# Run main server (when implemented)
cargo run --bin impulse-server

# Run CLI tool (when implemented)
cargo run --bin impulse-cli -- --help
```

### Quality Checks
```bash
# Full quality check (use /quality-check command)
cargo fmt --all -- --check && \
cargo clippy --all-targets --all-features -- -D warnings && \
cargo test --workspace --all-features

# Security audit (use /security-scan command)
cargo audit

# Coverage report (use /test-coverage command)
cargo tarpaulin --workspace --out Html --output-dir coverage
```

### Documentation
```bash
# Generate Rust docs
cargo doc --workspace --no-deps --open

# Check for broken links
cargo doc --workspace --no-deps 2>&1 | grep warning
```

---

## Important Files

### Core Documentation
- `README.md` - Project overview, status, architecture (570 lines)
- `CHANGELOG.md` - Version history following "Keep a Changelog" (249 lines)
- `CONTRIBUTING.md` - Contribution guidelines (336 lines)

### Analysis Reports
- `CI-CD-ANALYSIS-REPORT.md` - Comprehensive CI/CD analysis (16,000+ lines)
- `CI-CD-SUMMARY.md` - Executive summary (330 lines)
- `logs/2025-11-23-daily-log.md` - Development log (15,850 lines)

### Planning Documents
- `docs/planning/phase-1-overview.md` - Phase 1 roadmap
- `to-dos/sprint-XX-*.md` - Individual sprint TODO files (32 total)

---

## Active Pull Requests

### PR #3: Optimize CI/CD Pipeline (36% Faster)
**Branch:** ci/optimizations
**Status:** Open (CI running)
**Created:** 2025-11-23

**Changes:**
- Swatinem/rust-cache@v2 (intelligent caching)
- Security audit job (cargo-audit)
- MSRV testing (Rust 1.80)
- Network retry configuration
- CI success gate job

**Expected Improvements:**
- Run time: 5m 30s → 3m 30s (36% reduction)
- Better caching strategy
- Security vulnerability detection
- MSRV compliance verification

### Dependabot PRs (8 open)
1. **#1:** actions/checkout 4 → 6
2. **#2:** codecov-action 4 → 5
3. **#4:** axum 0.7 → 0.8
4. **#5:** binrw 0.14 → 0.15
5. **#6:** bincode 1.3 → 2.0
6. **#7:** crossterm 0.28 → 0.29
7. **#8:** toml 0.8 → 0.9

---

## Known Issues

**None currently.** All CI checks passing on main branch.

---

## Quick Reference

### Claude Code Commands (Global)
```bash
# Quality checks
/quality-check           # 2-5 min comprehensive check
/code-review            # 15-20 min deep review
/security-scan          # 2-5 min security audit
/test-coverage          # Generate coverage report

# Git operations
/stage-commit           # Stage and commit with proper message
/pr-create             # Create pull request

# Documentation
/doc-update            # Update project documentation
/daily-log             # Generate daily development log

# Complex tasks
/sub-agent             # Launch specialized sub-agent
```

### Key Patterns
1. **ALWAYS read before editing files**
2. **Use TodoWrite for task tracking**
3. **Follow conventional commit format**
4. **Run quality checks before committing**
5. **Update CLAUDE.local.md for state changes**

---

## Phase Roadmap

### Phase 1: Foundation (Sprints 1-8, Months 1-4)
- ✅ Sprint 1: Project Setup
- ✅ Sprint 2: Core Types
- ⏳ Sprint 3: Pascal Analysis
- 📋 Sprint 4: Configuration System
- 📋 Sprint 5: Error Handling
- 📋 Sprint 6: Logging Infrastructure
- 📋 Sprint 7: Database Schema
- 📋 Sprint 8: Testing Framework

### Phase 2: Core Features (Sprints 9-16, November 2025 - December 2025) - 100% COMPLETE
- ✅ Sprint 9: User authentication (rate limiting, lockout, validation)
- ✅ Sprint 10: Menu system (TOML parser, navigation)
- ✅ Sprint 11: Message read (MessageBase trait, JAM/Hudson, screens)
- ✅ Sprint 12: Message write (posting, replies, quoting)
- ✅ Sprint 13: File browsing (areas, list, details, search, FILE_ID.DIZ)
- ✅ Sprint 14: File upload (processor, ClamAV scanning, validation, quarantine)
- ✅ Sprint 15: User profiles (profile display, stats, settings, achievements, privacy)
- ✅ Sprint 16: Integration & testing (cross-crate workflows, 68 tests, 32 benchmarks)

### Phase 3: Feature Completion (Sprints 17-24, November 2025) - 100% COMPLETE
- ✅ Sprint 17: Zmodem protocol (frame structure, CRC, transfer, recovery)
- ✅ Sprint 18: Xmodem/Ymodem protocols (checksum/CRC/1K, batch mode)
- ✅ Sprint 19: Protocol completion (Ymodem-G, auto-detection, preferences)
- ✅ Sprint 20: Theme system (architecture, 3 default themes, color schemes)
- ✅ Sprint 21: Door game interface (dropfiles, manager, executor, DOSBox)
- ✅ Sprint 22: Advanced messaging (QWK offline, import/export, FidoNet)
- ✅ Sprint 23: Administration interface (user/file/system management, audit)
- ✅ Sprint 24: Integration testing (journeys, security audits, load testing)

### Phase 4: Polish & Deployment (Sprints 25-32, Months 19-24)
- Performance optimization, security hardening
- Documentation, deployment, migration tools

**Total Timeline:** 24 months (32 sprints × ~3 weeks each)

---

## Notes

### Temp Files
Use `/tmp/impulse-next-bbs/` for temporary files (NOT `/tmp/` directly)

### Cross-Platform Testing
All code must work on Linux, Windows 11, and macOS

### Pascal Reference
Original Pascal source in `ref-docs/original-pascal/` for reference during conversion

---

**Last Updated:** 2025-11-26
**Session:** Sprint 24 - Phase 3 Integration Testing Complete
**Next Milestone:** Phase 4 - Sprint 25 (Polish & Deployment)
