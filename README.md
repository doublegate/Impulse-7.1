# Impulse-Next_BBS - Modern BBS Software

[![CI](https://github.com/doublegate/Impulse-Next_BBS/workflows/CI/badge.svg)](https://github.com/doublegate/Impulse-Next_BBS/actions)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/doublegate/Impulse-Next_BBS#license)
[![Rust Version](https://img.shields.io/badge/rust-1.85%2B%20(edition%202024)-orange.svg)](https://www.rust-lang.org)

A complete modernization of the classic Impulse 7.1 Bulletin Board System from Borland Pascal 7.0 to modern Rust, preserving BBS history while leveraging contemporary software engineering practices.

## Table of Contents

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

**Current Version**: 0.1.0 (Sprint 1-4 Complete)
**Development Phase**: Phase 1 - Foundation
**Completion**: Sprint 1-4 (50%) - Ready for Sprint 5

### Recent Milestones

- **Sprint 1 Complete** (Project Setup): Full workspace infrastructure with 16 crates, CI/CD pipeline
- **Sprint 2 Complete** (Core Types): User, FileEntry, Message, BbsConfig types with validation and serialization
- **Sprint 3 Complete** (Pascal Analysis): 114 Pascal files analyzed (39,079 LOC), dependency graph (1,070 edges), risk assessment, 4-phase conversion roadmap
- **Sprint 4 Complete** (Configuration System): impulse-config crate with figment integration, hierarchical config loading (TOML + env vars), 3 validation modes, 37 tests, comprehensive README
- **Quality Metrics**: 105 tests passing (27 unit + 11 integration + 67 doc tests), 0 clippy warnings, comprehensive documentation
- **Latest Commit**: TBD - Sprint 4 Configuration System

### Next Steps

- **Sprint 5**: Error Handling & Logging Infrastructure
- **Phase 1 Goal**: Complete foundation (8 sprints, months 1-6)
- **Timeline**: 24 months total, 32 sprints across 4 phases

## Features

### Current Implementation (v0.1.0)

- Core data types (User, FileEntry, Message, BbsConfig)
- Unified error handling (15 error variants)
- JSON and binary serialization support
- Comprehensive validation framework
- Configuration system with hierarchical loading (TOML + environment variables)
- CI/CD pipeline (test, lint, build, coverage)
- Cross-platform workspace structure

### Planned Features

**Phase 1 (Months 1-6) - Foundation**
- ANSI terminal rendering
- Telnet server with IAC negotiation
- User authentication (Argon2id)
- Basic session management

**Phase 2 (Months 7-12) - Core Features**
- Message base (JAM/Hudson formats)
- File areas and browsing
- File upload handling
- Menu system and navigation

**Phase 3 (Months 13-18) - Feature Completion**
- File transfer protocols (Zmodem, Xmodem, Ymodem)
- Theme system
- Door game interface (DOSBox integration)
- QWK offline reader support

**Phase 4 (Months 19-24) - Polish & Launch**
- Performance optimization
- Web-based administration
- Legacy data migration tools
- Production deployment support

### Modern Enhancements

- **Multi-Protocol Support**: Telnet, SSH, WebSocket, REST API
- **Async Architecture**: Tokio-based concurrent session handling
- **Modern Storage**: SQLite/PostgreSQL with legacy format support
- **Security**: Argon2id password hashing, rate limiting, input validation
- **Cloud-Ready**: Docker, Kubernetes, containerized deployment
- **Monitoring**: Prometheus metrics, structured logging

## Architecture

### High-Level Design

```
┌─────────────────────────────────────────────────────────────────┐
│                     Impulse BBS System                          │
│                                                                 │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐        │
│  │   Telnet/    │  │     SSH      │  │  HTTP/REST   │        │
│  │  Serial Port │  │   Server     │  │     API      │        │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘        │
│         │                  │                  │                 │
│         └──────────────────┴──────────────────┘                │
│                            │                                    │
│                 ┌──────────▼──────────┐                        │
│                 │  Session Manager    │                        │
│                 │   (Async/Tokio)     │                        │
│                 └──────────┬──────────┘                        │
│                            │                                    │
│         ┌──────────────────┼──────────────────┐               │
│         │                  │                  │                │
│  ┌──────▼───────┐  ┌──────▼──────┐  ┌───────▼────────┐       │
│  │ Terminal I/O │  │   Message    │  │  File Transfer │       │
│  │  Subsystem   │  │   Subsystem  │  │   Subsystem    │       │
│  └──────┬───────┘  └──────┬───────┘  └───────┬────────┘       │
│         │                  │                  │                 │
│         └──────────────────┴──────────────────┘                │
│                            │                                    │
│                 ┌──────────▼──────────┐                        │
│                 │   Storage Layer     │                        │
│                 │  (SQLite/Postgres)  │                        │
│                 └─────────────────────┘                        │
└─────────────────────────────────────────────────────────────────┘
```

### 16-Crate Workspace Structure

**Core Crates:**
- `impulse-core` - Core BBS logic and state management
- `impulse-types` - Shared data types and error handling
- `impulse-config` - Configuration management

**Protocol Crates:**
- `impulse-protocol` - Protocol trait definitions
- `impulse-telnet` - Telnet protocol implementation
- `impulse-ssh` - SSH protocol implementation

**Feature Crates:**
- `impulse-session` - Session management and event loops
- `impulse-terminal` - Terminal I/O and ANSI rendering
- `impulse-auth` - Authentication (Argon2id)
- `impulse-message` - Message bases (JAM/Hudson)
- `impulse-file` - File areas and transfers
- `impulse-user` - User management
- `impulse-door` - Door game support

**Application Crates:**
- `impulse-web` - Web admin panel (Axum)
- `impulse-cli` - CLI tools
- `impulse-server` - Main server binary

See [/home/parobek/Code/Impulse-Next_BBS/docs/02-architecture.md](/home/parobek/Code/Impulse-Next_BBS/docs/02-architecture.md) for complete architecture documentation.

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

The project uses GitHub Actions with 4 jobs:

- **Lint**: `cargo clippy --all-targets --all-features -- -D warnings`
- **Test**: `cargo test --workspace --all-features --verbose`
- **Build**: `cargo build --workspace --release`
- **Coverage**: Tarpaulin + Codecov integration

Runs on: Linux, Windows, macOS

### Contributing

We welcome contributions! Please see [/home/parobek/Code/Impulse-Next_BBS/CONTRIBUTING.md](/home/parobek/Code/Impulse-Next_BBS/CONTRIBUTING.md) for:

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
│   ├── impulse-web/        # Web admin panel
│   ├── impulse-cli/        # CLI tools
│   └── impulse-server/     # Main server binary
├── docs/                   # Comprehensive documentation
│   ├── 00-project-overview.md
│   ├── 01-phase-sprint-plan.md
│   ├── 02-architecture.md
│   ├── 03-technical-details.md
│   ├── 04-development-guide.md
│   ├── 05-testing-strategy.md
│   ├── 06-deployment-guide.md
│   ├── 07-migration-guide.md
│   └── 08-security-architecture.md
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

- **Language**: Rust 2021 edition
- **Minimum Version**: Rust 1.80+
- **Async Runtime**: Tokio 1.47

### Key Dependencies

**Production:**
- `tokio` 1.47 - Async runtime (full features)
- `tokio-util` 0.7 - Async utilities
- `crossterm` 0.28 - Terminal I/O
- `serde` 1.0 - Serialization framework
- `serde_json` 1.0 - JSON support
- `toml` 0.8 - Configuration files
- `binrw` 0.14 - Binary parsing
- `thiserror` 2.0 - Error handling
- `anyhow` 1.0 - Error context
- `tracing` 0.1 - Structured logging
- `argon2` 0.5 - Password hashing
- `sqlx` 0.8 - Database access
- `axum` 0.7 - Web framework

**Development:**
- `proptest` 1.5 - Property-based testing

**Build Optimization:**
- LTO enabled in release builds
- Single codegen unit for maximum optimization
- Debug symbols stripped in release

## Documentation

### Core Documentation (docs/)

Comprehensive documentation covering all aspects:

1. [00-project-overview.md](/home/parobek/Code/Impulse-Next_BBS/docs/00-project-overview.md) - Vision, objectives, stakeholders (272 lines)
2. [01-phase-sprint-plan.md](/home/parobek/Code/Impulse-Next_BBS/docs/01-phase-sprint-plan.md) - 32-sprint roadmap (1,270 lines)
3. [02-architecture.md](/home/parobek/Code/Impulse-Next_BBS/docs/02-architecture.md) - System design (1,219 lines)
4. [03-technical-details.md](/home/parobek/Code/Impulse-Next_BBS/docs/03-technical-details.md) - Pascal→Rust conversion (1,768 lines)
5. [04-development-guide.md](/home/parobek/Code/Impulse-Next_BBS/docs/04-development-guide.md) - Developer onboarding (965 lines)
6. [05-testing-strategy.md](/home/parobek/Code/Impulse-Next_BBS/docs/05-testing-strategy.md) - Testing methodology (948 lines)
7. [06-deployment-guide.md](/home/parobek/Code/Impulse-Next_BBS/docs/06-deployment-guide.md) - Docker, K8s (1,084 lines)
8. [07-migration-guide.md](/home/parobek/Code/Impulse-Next_BBS/docs/07-migration-guide.md) - Legacy data migration (956 lines)
9. [08-security-architecture.md](/home/parobek/Code/Impulse-Next_BBS/docs/08-security-architecture.md) - Security design (1,150 lines)

**Total**: 9,632 lines of comprehensive documentation

### Sprint TODO Files (to-dos/)

Detailed sprint plans for all 32 sprints:

- **Phase 1** (Sprints 1-8): Foundation
- **Phase 2** (Sprints 9-16): Core Features
- **Phase 3** (Sprints 17-24): Feature Completion
- **Phase 4** (Sprints 25-32): Polish & Launch

**Total**: 19,214 lines across 30 files with 93 Rust code examples

### Reference Documentation (ref-docs/)

- [impulse-history.md](/home/parobek/Code/Impulse-Next_BBS/ref-docs/impulse-history.md) - BBS history and cultural context
- [rust-conversion-technical.md](/home/parobek/Code/Impulse-Next_BBS/ref-docs/rust-conversion-technical.md) - Conversion strategies

### API Documentation

Generate API documentation:

```bash
cargo doc --workspace --no-deps --open
```

## Roadmap

### 4 Phases, 24 Months, 32 Sprints

**Phase 1: Foundation (Months 1-6, Sprints 1-8)**
- ✅ Sprint 1: Project setup (COMPLETE)
- ✅ Sprint 2: Core type system (COMPLETE)
- ✅ Sprint 3: Pascal analysis (COMPLETE)
- Sprint 4-8: Configuration, error handling, logging, database, testing

**Phase 2: Core Features (Months 7-12, Sprints 9-16)**
- User authentication and sessions
- Menu system and navigation
- Message base (read/write)
- File areas (browse/upload)
- User profiles and statistics

**Phase 3: Feature Completion (Months 13-18, Sprints 17-24)**
- Zmodem and file transfer protocols
- Theme system
- Door game interface
- Advanced message features
- Administration interface

**Phase 4: Polish & Launch (Months 19-24, Sprints 25-32)**
- Performance optimization
- Comprehensive documentation
- Legacy migration tools
- Web-based administration
- Beta testing and bug fixes
- Public 1.0 release

### Key Milestones

| Milestone | Target | Status |
|-----------|--------|--------|
| Phase 1 Complete | Month 6 | In Progress |
| Phase 2 Complete | Month 12 | Pending |
| Phase 3 Complete | Month 18 | Pending |
| Phase 4 Complete | Month 24 | Pending |
| Production Launch | Month 24 | Pending |

## Testing

### Current Test Suite

**Total Tests**: 82 (100% passing)

- **Unit Tests**: 56 tests (validation logic)
- **Integration Tests**: 11 tests (serialization)
- **Doc Tests**: 15 tests (documentation examples)

### Test Coverage by Component

- User type: 10 tests
- FileEntry type: 10 tests
- Message type: 11 tests
- BbsConfig type: 13 tests
- Error handling: 3 tests
- Serialization: 11 tests (JSON + bincode)

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

See [/home/parobek/Code/Impulse-Next_BBS/docs/05-testing-strategy.md](/home/parobek/Code/Impulse-Next_BBS/docs/05-testing-strategy.md) for:

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

Please read our [CONTRIBUTING.md](/home/parobek/Code/Impulse-Next_BBS/CONTRIBUTING.md) for:

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

- **MIT License** ([LICENSE-MIT](/home/parobek/Code/Impulse-Next_BBS/LICENSE-MIT))
- **Apache License 2.0** ([LICENSE-APACHE](/home/parobek/Code/Impulse-Next_BBS/LICENSE-APACHE))

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
- **Documentation**: [/home/parobek/Code/Impulse-Next_BBS/docs](/home/parobek/Code/Impulse-Next_BBS/docs)

---

**"We're figuring it out!"** - Preserving BBS history, one commit at a time.

*For detailed sprint plans, architecture decisions, and technical specifications, see the comprehensive documentation in the [/home/parobek/Code/Impulse-Next_BBS/docs](/home/parobek/Code/Impulse-Next_BBS/docs) directory.*
