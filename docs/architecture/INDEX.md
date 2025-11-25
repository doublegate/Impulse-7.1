# Architecture Documentation

System design, technical architecture, and security documentation.

**Last Updated:** 2025-11-24

---

## Overview

This directory contains architectural documentation for Impulse-Next BBS, including system design, component architecture, technical implementation details, and security architecture.

---

## Files

### [system-architecture.md](system-architecture.md)

**Overall system architecture and component design**

Documents the high-level architecture, component organization, and architectural decisions.

**Topics:**
- Component architecture (16-crate workspace)
- Data flow and communication patterns
- Protocol handling (Telnet, SSH)
- Session management
- Database architecture
- Service integration

### [technical-details.md](technical-details.md)

**Technical implementation details and data structures**

Deep dive into technical implementation, algorithms, and data structures.

**Topics:**
- Data structures and types
- Algorithms and implementations
- Performance considerations
- Memory management patterns
- Async/await architecture
- Error handling strategies

### [security-architecture.md](security-architecture.md)

**Security design, authentication, and threat model**

Comprehensive security architecture documentation.

**Topics:**
- Authentication system (Argon2id, TOTP, session tokens)
- Authorization and access control
- Security levels and permissions
- Threat model and attack vectors
- Security best practices
- Cryptographic implementations
- Audit logging

---

## Architecture Principles

**Key Design Principles:**
1. **Safety First:** Leverage Rust's memory safety guarantees
2. **Modularity:** Clear separation of concerns (16 crates)
3. **Async-First:** Tokio-based async runtime
4. **Protocol Agnostic:** Abstract protocol handling
5. **Security by Default:** Secure defaults, defense in depth
6. **Cross-Platform:** Linux, Windows, macOS, BSD support

**Technology Stack:**
- **Language:** Rust 1.85+ (Edition 2024)
- **Runtime:** Tokio async runtime
- **Database:** SQLite/PostgreSQL via SQLx
- **Web:** Axum framework
- **Authentication:** Argon2id, SHA-256
- **Protocols:** Telnet (RFC 854, 857, 858, 1073), SSH (RFC 4253, 4254)

---

## Related Documentation

- **[Getting Started](../getting-started/)** - Project overview and vision
- **[Implementation Guides](../implementation/)** - Developer guides
- **[Testing Strategy](../testing/)** - Testing approach
- **[Planning Documents](../planning/)** - Phase and sprint plans

---

[← Back to Documentation Index](../INDEX.md)
