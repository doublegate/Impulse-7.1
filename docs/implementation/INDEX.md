# Implementation Guides

Step-by-step implementation guides for specific features and systems.

**Last Updated:** 2025-11-24

---

## Overview

This directory contains practical implementation guides for developers working on Impulse-Next BBS, including development workflows, coding standards, and integration guides.

---

## Files

### [development-guide.md](development-guide.md)

**Developer onboarding and contribution workflow**

Comprehensive guide for developers joining the project.

**Topics:**
- **Development Environment:** Setup instructions for Linux, Windows, macOS
- **Workspace Structure:** Understanding the 16-crate architecture
- **Coding Standards:** Rust style guide, naming conventions, documentation
- **Git Workflow:** Branch strategy, commit conventions, pull requests
- **Testing Practices:** Unit tests, integration tests, coverage requirements
- **Code Review:** Review process, quality gates, approval requirements
- **CI/CD Integration:** GitHub Actions workflow, automated checks
- **Debugging Techniques:** Tools, strategies, common issues

**Getting Started:**
1. Clone repository
2. Install Rust 1.85+
3. Run `cargo build --workspace`
4. Run `cargo test --workspace`
5. Read CONTRIBUTING.md

### [logging-integration.md](logging-integration.md)

**Logging framework integration and best practices**

Guide to implementing structured logging throughout the codebase.

**Topics:**
- **Logging Framework:** tracing crate usage and configuration
- **Log Levels:** TRACE, DEBUG, INFO, WARN, ERROR usage guidelines
- **Structured Logging:** Key-value pairs, spans, events
- **Log Format:** JSON, text, custom formatters
- **Performance:** Async logging, buffering, sampling
- **Integration Points:** User actions, authentication, errors, performance metrics
- **Audit Logging:** Security events, access logs, compliance
- **Log Management:** Rotation, archival, retention policies

**Sprint 9 Implementation:**
- Centralized logging configuration
- Log rotation and management
- Audit logging for security events
- Integration with user authentication system
- Performance monitoring and metrics

---

## Development Best Practices

**Code Quality:**
- ✅ Run `cargo fmt` before committing
- ✅ Run `cargo clippy` with zero warnings
- ✅ Maintain 80%+ test coverage
- ✅ Document all public APIs with rustdoc
- ✅ Use `cargo audit` for security checks

**Testing:**
- Unit tests for individual components
- Integration tests for component interactions
- Property-based tests for invariants
- Serialization round-trip tests
- Performance benchmarks

**Git Workflow:**
- Feature branches from main
- Conventional commits (feat:, fix:, docs:, test:, refactor:, chore:)
- Pull request reviews (minimum 2 reviewers for core modules)
- CI must pass before merge
- Update CHANGELOG.md

**Documentation:**
- Inline comments for complex logic
- Rustdoc for public APIs
- Update markdown docs when adding features
- Keep examples up-to-date
- Update INDEX.md files

---

## Common Development Tasks

### Running Tests
```bash
# All tests
cargo test --workspace --all-features --verbose

# Specific crate
cargo test -p impulse-types --verbose

# Specific test
cargo test test_user_serialization --verbose
```

### Building
```bash
# Development build
cargo build --workspace

# Release build
cargo build --workspace --release

# Specific binary
cargo build --bin impulse-server --release
```

### Linting and Formatting
```bash
# Format code
cargo fmt --all

# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings
```

### Coverage
```bash
# Generate coverage report
cargo tarpaulin --workspace --out Html --output-dir coverage

# Open report
xdg-open coverage/index.html
```

---

## Related Documentation

- **[Architecture](../architecture/)** - System design and component structure
- **[Testing Strategy](../testing/)** - Comprehensive testing approach
- **[Planning](../planning/)** - Sprint plans and roadmaps
- **[Getting Started](../getting-started/)** - Project overview

---

[← Back to Documentation Index](../INDEX.md)
