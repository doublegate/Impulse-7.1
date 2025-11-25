# CLAUDE.md - Impulse-Next_BBS Project Memory

Project-specific guidance for Impulse-Next_BBS modernization (classic Impulse 7.1 BBS: Borland Pascal 7.0 → Rust 2024).

**Version:** 0.1.0 | **Updated:** 2025-11-23

---

## Project Overview

**Repository:** https://github.com/doublegate/Impulse-Next_BBS
**Type:** Rust 2024 edition workspace (16 crates: 13 libraries + 3 binaries)
**Goal:** Next-generation BBS software - modernizing the classic Impulse 7.1 BBS from Borland Pascal to Rust for cross-platform operation
**License:** MIT OR Apache-2.0

---

## Current Status

**Phase:** 2 - Core Features (Sprints 9-16)
**Sprints Complete:** 12 of 32 (37.5%)
**Version:** 0.1.0 (Phase 2: 50% Complete)
**Last Commit:** 777750f (2025-11-25)

### Sprint Progress
- ✅ **Phase 1:** Foundation (Sprints 1-8, 100%)
- ✅ **Sprint 9:** User Authentication (100%)
- ✅ **Sprint 10:** Menu System (100%)
- ✅ **Sprint 11:** Message Read (100%)
- ✅ **Sprint 12:** Message Write (100%)
- 📋 **Sprints 13-32:** Planned

### Quality Metrics
- **Tests:** 870+ passing (100% pass rate)
- **Coverage:** 64.51% baseline (target: 75%+ Phase 2)
- **Clippy:** 0 warnings
- **CI/CD:** 100% passing on main branch
- **Crates:** 19 (17 libraries + 2 binaries)
- **Code:** 28,000+ lines total

---

## Project Structure

### Workspace Layout
```
Impulse-Next_BBS/
├── crates/              # 16 crates
│   ├── impulse-core/    # Core BBS functionality
│   ├── impulse-types/   # Type definitions (User, FileEntry, Message, BbsConfig)
│   ├── impulse-config/  # Configuration management
│   ├── impulse-protocol/# Protocol implementations
│   ├── impulse-telnet/  # Telnet server
│   ├── impulse-ssh/     # SSH server
│   ├── impulse-session/ # Session management
│   ├── impulse-terminal/# Terminal handling (ANSI/Avatar/RIP)
│   ├── impulse-auth/    # Authentication
│   ├── impulse-message/ # Message system
│   ├── impulse-file/    # File management
│   ├── impulse-user/    # User management
│   ├── impulse-door/    # Door game interface
│   ├── impulse-web/     # Web admin panel
│   ├── impulse-cli/     # CLI tool (binary)
│   └── impulse-server/  # Main server (binary)
├── docs/                # 48+ documentation files
│   ├── architecture/    # System design docs
│   ├── implementation/  # Implementation guides
│   └── planning/        # Phase/sprint planning
├── ref-docs/            # Reference documentation
│   └── original-pascal/ # Pascal source analysis
├── to-dos/              # Sprint TODO files (32 sprints)
│   ├── phase-1-foundation/
│   ├── phase-2-core-services/
│   ├── phase-3-advanced-features/
│   └── phase-4-polish-deployment/
├── logs/                # Development logs
├── .github/workflows/   # CI/CD configuration
├── Cargo.toml           # Workspace manifest
├── README.md            # Project documentation
├── CHANGELOG.md         # Version history
└── CONTRIBUTING.md      # Contribution guidelines
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

**Executables:**
- `impulse-server` - Main BBS server
- `impulse-cli` - Command-line management tool

---

## Technology Stack

### Core Dependencies
| Dependency | Version | Purpose |
|------------|---------|---------|
| Rust | 1.80+ (2021 edition) | Primary language |
| Tokio | 1.47+ | Async runtime |
| crossterm | 0.28 | Terminal I/O |
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

**Optimizations (PR #3 pending):**
- Swatinem/rust-cache@v2 for intelligent caching
- Security audit job (cargo-audit)
- MSRV testing (Rust 1.80)
- CI success gate job
- Expected improvement: 5m 30s → 3m 30s (36% faster)

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

### Next Sprint

#### Sprint 13: Terminal I/O
**TODO:** `to-dos/phase-2-core-features/sprint-13-terminal-io.md` (estimated)

**Goals:**
- ANSI escape sequence rendering
- Input handling (keyboard, mouse)
- Avatar graphics support
- Terminal emulation layer

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

### Phase 2: Core Features (Sprints 9-16, November 2025 - January 2026) - 50% COMPLETE
- ✅ Sprint 9: User authentication (rate limiting, lockout, validation)
- ✅ Sprint 10: Menu system (TOML parser, navigation)
- ✅ Sprint 11: Message read (MessageBase trait, JAM/Hudson, screens)
- ✅ Sprint 12: Message write (posting, replies, quoting)
- 📋 Sprint 13: Terminal I/O (ANSI rendering, input handling)
- 📋 Sprint 14: Telnet protocol (RFC 854, IAC negotiation)
- 📋 Sprint 15: File areas (browsing, upload/download)
- 📋 Sprint 16: Session management (WebSocket, concurrent handling)

### Phase 3: Advanced Features (Sprints 17-24, Months 11-18)
- Terminal emulation, door games
- Networking, web admin panel

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

**Last Updated:** 2025-11-23
**Session:** CI/CD Optimization
**Next Milestone:** Sprint 3 - Pascal Analysis
