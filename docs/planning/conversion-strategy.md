# Pascal to Rust Conversion Strategy & Prioritization Plan

**Project:** Impulse-Next_BBS (Impulse 7.1 BBS Modernization)
**Date:** 2025-11-23
**Version:** 1.0
**Status:** Sprint 3 Complete - Ready for Implementation

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Strategic Principles](#2-strategic-principles)
3. [Risk-Based Prioritization Framework](#3-risk-based-prioritization-framework)
4. [Platform-Specific Migration Strategy](#4-platform-specific-migration-strategy)
5. [4-Phase Conversion Roadmap](#5-4-phase-conversion-roadmap)
6. [Type System Migration Strategy](#6-type-system-migration-strategy)
7. [Global State Refactoring Strategy](#7-global-state-refactoring-strategy)
8. [Binary File Format Strategy](#8-binary-file-format-strategy)
9. [Dependency Management Strategy](#9-dependency-management-strategy)
10. [Testing Strategy](#10-testing-strategy)
11. [High-Risk Module Strategy](#11-high-risk-module-strategy)
12. [Sprint Execution Guidelines](#12-sprint-execution-guidelines)
13. [Success Metrics & KPIs](#13-success-metrics--kpis)
14. [Risk Mitigation Timeline](#14-risk-mitigation-timeline)
15. [Parallel Conversion Opportunities](#15-parallel-conversion-opportunities)
16. [Cross-References](#16-cross-references)

---

## 1. Executive Summary

### Overview

This document provides a comprehensive strategy for converting the Impulse 7.1 BBS system from Borland Pascal 7.0 to Rust 2024 edition. The conversion encompasses **114 Pascal units** (39,079 lines of code) with **1,070 dependencies**, transforming a DOS-era bulletin board system into a modern, cross-platform application.

**Source Files:** Based on comprehensive analysis documented in [../pascal-reference/](../pascal-reference/):
- [pascal-inventory.md](../pascal-reference/analysis/pascal-inventory.md) - Complete unit inventory
- [pascal-unit-analysis.md](../pascal-reference/analysis/pascal-unit-analysis.md) - Detailed unit analysis
- [pascal-dependencies.md](../pascal-reference/analysis/pascal-dependencies.md) - Dependency documentation
- [conversion-risk-assessment.md](../pascal-reference/risk-assessment/conversion-risk-assessment.md) - Risk ratings
- [conversion-order.md](../pascal-reference/conversion/conversion-order.md) - Priority order
- 13 additional analysis documents

### Key Statistics

| Metric | Value | Source |
|--------|-------|--------|
| **Total Pascal Units** | 114 | [pascal-inventory.md](../pascal-reference/analysis/pascal-inventory.md) |
| **Total Lines of Code** | 39,079 | [pascal-inventory.md](../pascal-reference/analysis/pascal-inventory.md) |
| **Total Dependencies** | 1,070 | [pascal-dependencies.md](../pascal-reference/analysis/pascal-dependencies.md) |
| **Average Dependencies/Unit** | 9.4 | [pascal-dependencies.md](../pascal-reference/analysis/pascal-dependencies.md) |
| **CRITICAL Risk Units** | 11 (9.6%) | [conversion-risk-assessment.md](../pascal-reference/risk-assessment/conversion-risk-assessment.md) |
| **HIGH Risk Units** | 27 (23.7%) | [conversion-risk-assessment.md](../pascal-reference/risk-assessment/conversion-risk-assessment.md) |
| **MEDIUM Risk Units** | 30 (26.3%) | [conversion-risk-assessment.md](../pascal-reference/risk-assessment/conversion-risk-assessment.md) |
| **LOW Risk Units** | 46 (40.4%) | [conversion-risk-assessment.md](../pascal-reference/risk-assessment/conversion-risk-assessment.md) |

### Platform Dependencies

| Category | Count | Files | Source |
|----------|-------|-------|--------|
| **DOS Function Calls** | 23 files | EXEC, GETDIR, MKDIR, CHDIR, RMDIR, SWAPVECTORS, KEEP | [pascal-dos-specific.md](../pascal-reference/analysis/pascal-dos-specific.md) |
| **Overlay Directives** | 75 directives | IMP.PAS (74), TIMETASK.PAS (1) | [pascal-overlays.md](../pascal-reference/analysis/pascal-overlays.md) |
| **Interrupt Handlers** | 2 files | TMPCOM.PAS (48), TMPOLD.PAS (48) | [pascal-interrupts.md](../pascal-reference/analysis/pascal-interrupts.md) |
| **Inline Assembly** | 14 files | NETFOSSL.PAS (26), MYIO.PAS (8), others | [pascal-interrupts.md](../pascal-reference/analysis/pascal-interrupts.md) |
| **ABSOLUTE Variables** | 2 files | MAIL7.PAS (2), ZIPVIEWU.PAS (1) | [pascal-interrupts.md](../pascal-reference/analysis/pascal-interrupts.md) |
| **Binary File I/O** | 29 files | FILE OF RecordType pattern | [pascal-binary-formats.md](../pascal-reference/conversion/pascal-binary-formats.md) |
| **Global Variables** | 90 files | Mutable state in INTERFACE section | [pascal-globals.md](../pascal-reference/pascal-globals.md) |

### Conversion Timeline

**Total Duration:** 24 months (Sprints 3-32)

| Phase | Sprints | Duration | Units | Focus |
|-------|---------|----------|-------|-------|
| **Phase 1** | 3-10 | Months 1-4 | 28 | Foundation (core types, utilities) |
| **Phase 2** | 11-18 | Months 5-10 | 28 | Core Services (files, mail, users) |
| **Phase 3** | 19-26 | Months 11-18 | 28 | Advanced Features (sysop, protocols) |
| **Phase 4** | 27-32 | Months 19-24 | 30 | Integration (main program, high-risk) |

**Source:** [conversion-order.md](../pascal-reference/conversion/conversion-order.md)

### Critical Success Factors

1. **Dependency-Aware Conversion** - Convert foundation modules (RECORDS.PAS, COMMON*.PAS) first
2. **Binary Compatibility** - Maintain compatibility with existing BBS data files or provide migration tools
3. **Risk Mitigation** - Address high-risk modules (33.3% of codebase) with expert review and extensive testing
4. **Parallel Conversion** - Exploit module independence to parallelize work (estimated 30-40% time reduction)
5. **Continuous Testing** - Test each converted module against original Pascal behavior

---

## 2. Strategic Principles

### Guiding Philosophy

The conversion strategy is built on six core principles that guide all technical decisions:

#### 2.1 Preserve Functionality, Modernize Architecture

**Principle:** Maintain 100% functional compatibility with Impulse 7.1 while modernizing the underlying architecture.

**Application:**
- Convert business logic faithfully (same algorithms, same behavior)
- Modernize platform dependencies (DOS→cross-platform)
- Improve error handling (Pascal→Result<T, E>)
- Add safety (global state→owned/borrowed types)

**Example:**
```rust
// Pascal: procedure ProcessUpload(var f: ulfrec);
// Rust: fn process_upload(file_entry: &mut FileEntry) -> Result<(), BbsError>
```

**Source:** [type-mapping.md](../pascal-reference/type-mapping.md)

#### 2.2 Safety Without Sacrifice

**Principle:** Leverage Rust's safety guarantees without compromising performance or functionality.

**Application:**
- Replace unsafe pointer manipulation with safe references
- Use `Option<T>` instead of nullable pointers
- Eliminate global mutable state where possible
- Maintain binary compatibility where necessary (bincode serialization)

**Trade-offs:**
- Some performance overhead for safety (acceptable for BBS workloads)
- Increased code verbosity (Result types, error handling)
- Stricter type requirements (worth the compile-time guarantees)

**Source:** [pascal-globals.md](../pascal-reference/pascal-globals.md)

#### 2.3 Dependency-Driven Conversion Order

**Principle:** Convert modules in dependency order to minimize rework and integration issues.

**Application:**
- **Foundation First:** RECORDS.PAS (core types) must be converted before dependent modules
- **Leaf Nodes Last:** Modules with many dependencies (IMP.PAS with 69 deps) converted late
- **Common Utilities Early:** COMMON*.PAS modules converted in Phase 1 (high dependency impact)

**Priority Formula:**
```
priority_score = (risk_score * 10) + (dependency_count * 2) - (dependency_impact * 5)
Lower score = Higher priority (convert earlier)
```

**Source:** [conversion-order.md](../pascal-reference/conversion/conversion-order.md)

#### 2.4 Risk-Based Resource Allocation

**Principle:** Allocate senior developers and extra time to high-risk modules (33.3% of codebase).

**Risk Categories:**

| Risk Level | Count | Percentage | Strategy |
|------------|-------|------------|----------|
| **CRITICAL** | 11 | 9.6% | Senior dev + pair programming + extensive testing |
| **HIGH** | 27 | 23.7% | Senior dev + peer review + integration tests |
| **MEDIUM** | 30 | 26.3% | Standard process + unit tests |
| **LOW** | 46 | 40.4% | Junior/mid-level dev + basic tests |

**Source:** [conversion-risk-assessment.md](../pascal-reference/risk-assessment/conversion-risk-assessment.md)

#### 2.5 Test-Driven Migration

**Principle:** Write tests before converting modules to ensure behavioral equivalence.

**Application:**
- **Pre-Conversion:** Characterization tests using Pascal original as oracle
- **During Conversion:** Unit tests for each converted function
- **Post-Conversion:** Integration tests for module interactions
- **Binary Compatibility:** Round-trip tests (write with Rust, read with Pascal, and vice versa)

**Test Coverage Targets:**
- Critical risk modules: 95%+ coverage
- High risk modules: 85%+ coverage
- Medium risk modules: 75%+ coverage
- Low risk modules: 60%+ coverage

**Source:** [SPRINT-03-COMPLETION-REPORT.md](../reports/sprints/SPRINT-03-COMPLETION-REPORT.md)

#### 2.6 Cross-Platform from Day One

**Principle:** Design for Linux, Windows, and macOS from the first line of code.

**Application:**
- Abstract platform-specific calls behind traits
- Use `std::process::Command` instead of DOS Exec
- Replace interrupts with OS-specific signal handlers
- Test on all three platforms in CI pipeline

**Platform Abstraction Example:**
```rust
trait FileSystemOps {
    fn current_dir() -> Result<PathBuf, std::io::Error>;
    fn create_dir(&self, path: &Path) -> Result<(), std::io::Error>;
    fn change_dir(&self, path: &Path) -> Result<(), std::io::Error>;
}

// DOS: GETDIR, MKDIR, CHDIR → Rust: std::env, std::fs
```

**Source:** [pascal-dos-specific.md](../pascal-reference/analysis/pascal-dos-specific.md)

---

## 3. Risk-Based Prioritization Framework

### Risk Scoring Methodology

The conversion risk assessment uses a **quantitative scoring system** to identify modules requiring special attention.

**Source:** [conversion-risk-assessment.md](../pascal-reference/risk-assessment/conversion-risk-assessment.md)

#### High Risk Factors (10 points each)

1. **Interrupt Handlers** - Platform-specific, no modern equivalent
2. **ABSOLUTE Variables** - Direct memory access, hardware I/O
3. **Low-level DOS Interrupts** - SwapVectors, GetInterVec, SetInterVec

**Affected Modules:**
- TMPCOM.PAS, TMPOLD.PAS (interrupt handlers)
- MAIL7.PAS, ZIPVIEWU.PAS (ABSOLUTE variables)
- EXEC.PAS, EXECOLD.PAS, RECORDS.PAS, COMMON5.PAS, SCRIPT.PAS, INSTALL.PAS, BPTRAP.PAS, SYSOP2D.PAS (DOS interrupts)

#### Medium-High Risk (7 points)

1. **Inline Assembly** - CPU-specific, may need intrinsics or pure Rust
2. **Process Execution** - DOS Exec requires std::process::Command

**Affected Modules:** 14 files with inline assembly (NETFOSSL.PAS, MYIO.PAS, ANSIDRV.PAS, etc.)

#### Medium Risk (5 points)

1. **Overlay System Usage** - DOS memory management, remove directives
2. **Binary File I/O** - Maintain compatibility with existing data

**Affected Modules:** 29 files with binary I/O, 2 files with overlays

#### Low-Medium Risk (3 points)

1. **DOS File System Calls** - GETDIR, MKDIR, CHDIR, RMDIR
2. **Heavy Pointer Usage** - Manual memory management (>50 occurrences)
3. **Variant Records** - Pascal unions → Rust enums

#### Complexity Factors (1-2 points)

1. **Module Size** - >500 lines
2. **Dependency Count** - >10 dependencies

### Risk Level Thresholds

| Level | Score Range | Count | Strategy |
|-------|-------------|-------|----------|
| **CRITICAL** | ≥ 20 | 11 | Expert team, extensive testing, phased conversion |
| **HIGH** | 10-19 | 27 | Senior developer, peer review, integration tests |
| **MEDIUM** | 5-9 | 30 | Standard process, unit tests |
| **LOW** | < 5 | 46 | Straightforward conversion, basic tests |

### Top 10 Highest-Risk Modules

**Source:** [high-risk-units.md](../pascal-reference/high-risk-units.md)

| Rank | Module | Risk Score | Risk Level | Key Challenges | Dependencies |
|------|--------|------------|------------|----------------|--------------|
| 1 | EXEC.PAS | 31 | CRITICAL | DOS interrupts, assembly, Exec, pointers, 974 lines | 2 |
| 2 | EXECOLD.PAS | 31 | CRITICAL | DOS interrupts, assembly, Exec, pointers, 974 lines | 2 |
| 3 | RECORDS.PAS | 28 | CRITICAL | DOS interrupts, assembly, Exec, 830 lines, **CORE TYPES** | 0 |
| 4 | COMMON.PAS | 27 | CRITICAL | Assembly, Exec, binary I/O, 107 pointers, 1960 lines | 5 |
| 5 | COMMON5.PAS | 27 | CRITICAL | DOS interrupts, assembly, Exec | 1 |
| 6 | INSTALL.PAS | 25 | CRITICAL | DOS interrupts, Exec, binary I/O, file system | 3 |
| 7 | SCRIPT.PAS | 24 | CRITICAL | DOS interrupts, Exec, file system, 951 lines | 10 |
| 8 | IMP.PAS | 22 | CRITICAL | Exec, overlays, binary I/O, file system, **69 deps** | 69 |
| 9 | MAIL7.PAS | 22 | CRITICAL | **ABSOLUTE vars**, Exec, 517 lines, 15 deps | 15 |
| 10 | BPTRAP.PAS | 21 | CRITICAL | DOS interrupts, assembly, 11 deps | 11 |

**Critical Insight:** RECORDS.PAS (rank 3) has **zero dependencies** and defines all core types - it **must be converted first** despite CRITICAL risk level.

---

## 4. Platform-Specific Migration Strategy

### DOS-to-Modern OS Migration

The original Impulse 7.1 BBS was built for DOS, requiring extensive platform-specific code modernization.

**Source:** [pascal-dos-specific.md](../pascal-reference/analysis/pascal-dos-specific.md), [pascal-interrupts.md](../pascal-reference/analysis/pascal-interrupts.md)

#### 4.1 DOS Function Replacement

| DOS Function | Count | Rust Replacement | Strategy |
|--------------|-------|------------------|----------|
| **EXEC** | 23 files | `std::process::Command` | Spawn processes, handle I/O with pipes |
| **GETDIR** | 16 files | `std::env::current_dir()` | Direct replacement |
| **MKDIR** | 7 files | `std::fs::create_dir()` | Direct replacement |
| **CHDIR** | 12 files | `std::env::set_current_dir()` | Direct replacement |
| **RMDIR** | 4 files | `std::fs::remove_dir()` | Direct replacement |
| **SWAPVECTORS** | 6 files | N/A (DOS-specific) | Remove, not needed in modern OS |
| **KEEP** | 3 files | N/A (TSR functionality) | Remove, not applicable |

**Implementation Example:**

```rust
// Pascal: Exec(GetEnv('COMSPEC'), '/C ' + command);
// Rust:
use std::process::Command;

fn execute_shell_command(command: &str) -> Result<(), BbsError> {
    let shell = std::env::var("COMSPEC")
        .or_else(|_| std::env::var("SHELL"))
        .unwrap_or_else(|_| "/bin/sh".to_string());

    let output = Command::new(shell)
        .arg("-c")
        .arg(command)
        .output()?;

    if !output.status.success() {
        return Err(BbsError::CommandFailed(output.stderr));
    }

    Ok(())
}
```

#### 4.2 Interrupt Handler Migration

**Challenge:** 2 files (TMPCOM.PAS, TMPOLD.PAS) use DOS interrupt handlers (48 each) for serial port communication.

**Modern Solution:** Replace with OS-specific signal handlers or serial port libraries.

```rust
// Unix/Linux
use signal_hook::{consts::SIGINT, iterator::Signals};

fn setup_signal_handlers() -> Result<(), std::io::Error> {
    let mut signals = Signals::new(&[SIGINT])?;
    std::thread::spawn(move || {
        for signal in signals.forever() {
            match signal {
                SIGINT => handle_interrupt(),
                _ => {},
            }
        }
    });
    Ok(())
}

// Windows
#[cfg(target_os = "windows")]
use winapi::um::consoleapi::SetConsoleCtrlHandler;
```

**Source:** [risk-mitigations.md](../pascal-reference/risk-assessment/risk-mitigations.md)

#### 4.3 Inline Assembly Replacement

**Challenge:** 14 files use inline assembly for:
- CPU feature detection
- Direct hardware I/O
- Performance-critical operations

**Strategy:**

1. **Evaluate Necessity** - Often Rust has better alternatives
2. **Use std::arch** - For CPU intrinsics (SIMD, etc.)
3. **Use asm! macro** - For unavoidable assembly
4. **Abstract Behind Safe Interface** - Isolate unsafe blocks

```rust
use std::arch::x86_64::*;

// Safe wrapper around assembly
fn cpu_feature_check() -> bool {
    unsafe { is_x86_feature_detected!("sse2") }
}

// If asm! is absolutely necessary
#[cfg(target_arch = "x86_64")]
unsafe fn low_level_operation() {
    asm!(
        "mov {0}, rax",
        out(reg) _,
        options(nostack, preserves_flags)
    );
}
```

**Source:** [risk-mitigations.md](../pascal-reference/risk-assessment/risk-mitigations.md)

#### 4.4 Overlay System Removal

**Challenge:** 75 overlay directives in 2 files (IMP.PAS: 74, TIMETASK.PAS: 1)

**Solution:** Simply remove overlay directives - modern OS handles memory paging automatically.

**Pascal:**
```pascal
{$O MAIL0}
{$O MAIL1}
{$O MAIL2}
```

**Rust:**
```rust
// No equivalent needed - standard module system
mod mail0;
mod mail1;
mod mail2;
```

**Source:** [pascal-overlays.md](../pascal-reference/analysis/pascal-overlays.md)

#### 4.5 ABSOLUTE Variables

**Challenge:** 2 files (MAIL7.PAS, ZIPVIEWU.PAS) use ABSOLUTE variables for memory-mapped I/O.

**Solution:** Replace with OS-specific memory mapping APIs.

```rust
// Unix: mmap, Windows: MapViewOfFile
use memmap2::MmapMut;

fn map_hardware_memory(address: usize, size: usize) -> Result<MmapMut, std::io::Error> {
    // Platform-specific implementation
    #[cfg(unix)]
    {
        use std::os::unix::io::AsRawFd;
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/mem")?;
        unsafe { MmapMut::map_mut(file.as_raw_fd()) }
    }

    #[cfg(windows)]
    {
        // Windows-specific memory mapping
        unimplemented!("Windows memory mapping not yet implemented")
    }
}
```

**Source:** [high-risk-units.md](../pascal-reference/high-risk-units.md)

---

## 5. 4-Phase Conversion Roadmap

The conversion is organized into **4 phases** across **30 sprints** (Sprints 3-32), with each phase building upon the previous.

**Source:** [conversion-order.md](../pascal-reference/conversion/conversion-order.md)

### Phase 1: Foundation (Sprints 3-10, Months 1-4)

**Goal:** Establish core types, utilities, and infrastructure for subsequent phases.

**Duration:** 8 sprints (~3 weeks each)

**Units to Convert:** 28 units (24.6% of codebase)

#### Key Modules

| Priority | Module | Risk Level | Dependencies | Rationale |
|----------|--------|------------|--------------|-----------|
| 1 | STRPROC | LOW | 0 | High dependency impact (88), zero deps |
| 2 | TIMEJUNK | LOW | 1 | High dependency impact (87) |
| 3 | SCRLBK | LOW | 3 | High dependency impact (86) |
| 14 | **RECORDS** | **CRITICAL** | 0 | **MUST BE FIRST - Core types, 93 deps on it** |
| 15 | COMMON5 | CRITICAL | 1 | High dependency impact (88) |
| 18 | COMMON | CRITICAL | 5 | High dependency impact (86), 1960 lines |
| 10 | COMMON1 | HIGH | 5 | Common utilities |
| 12 | COMMON2 | HIGH | 11 | Common utilities |

**Sprint Breakdown:**

- **Sprint 3:** Pascal Analysis (COMPLETE)
- **Sprint 4:** Configuration System (`impulse-config` crate)
- **Sprint 5:** Core Types Implementation (RECORDS.PAS → Rust)
  - Convert all record types
  - Implement bincode serialization
  - Binary compatibility validation
- **Sprint 6-7:** Common Utilities (COMMON*.PAS modules)
  - STRPROC, TIMEJUNK, SCRLBK (low risk)
  - COMMON5, COMMON, COMMON1, COMMON2 (critical/high risk)
- **Sprint 8-9:** Binary File I/O and Serialization
  - Implement binary file reading/writing
  - Test with existing BBS data files
  - Create migration tools if needed
- **Sprint 10:** Phase 1 Integration and Testing
  - Integration tests for all Phase 1 modules
  - Performance benchmarking
  - Documentation updates

**Success Criteria:**

- ✅ All core types implemented and tested
- ✅ Common utilities passing tests (100% for CRITICAL modules)
- ✅ Binary file I/O working for at least one record type
- ✅ Serialization round-trip tests passing
- ✅ Foundation ready for Phase 2 (file/mail modules)

### Phase 2: Core Services (Sprints 11-18, Months 5-10)

**Goal:** Build functional BBS core - file areas, message system, user management.

**Duration:** 8 sprints

**Units to Convert:** 28 units (24.6% of codebase)

#### Key Modules

| Module Group | Count | Risk Distribution | Focus |
|--------------|-------|-------------------|-------|
| **FILE*** | 14 | 5 HIGH, 4 MEDIUM, 5 LOW | File area management, uploads/downloads |
| **MAIL*** | 9 | 1 CRITICAL, 1 HIGH, 3 MEDIUM, 4 LOW | Message base, email, networking |
| **MISC*** | 5 | 1 HIGH, 3 MEDIUM, 1 LOW | Miscellaneous utilities |
| **Other** | - | - | User auth, session management |

**Sprint Breakdown:**

- **Sprints 11-13:** File Management Modules
  - FILE0-FILE14 (file areas, upload/download)
  - File listing, searching, batch operations
- **Sprints 14-16:** User and Message System
  - MAIL0-MAIL9 (message base, email)
  - User authentication (CUSER, NEWUSERS)
  - Session management
- **Sprint 17:** Authentication and Sessions
  - Argon2 password hashing
  - Session tokens and timeouts
  - Multi-user support
- **Sprint 18:** Phase 2 Integration and Testing
  - Integration tests for file/mail/user modules
  - End-to-end testing (user login → file upload → message post)
  - Performance optimization

**Success Criteria:**

- ✅ File upload/download operational
- ✅ Message base working (create, read, reply)
- ✅ User authentication functional
- ✅ Session state management tested
- ✅ Binary compatibility with Impulse 7.1 data files verified

### Phase 3: Advanced Features (Sprints 19-26, Months 11-18)

**Goal:** System operator functions, networking, protocol handlers, terminal emulation.

**Duration:** 8 sprints

**Units to Convert:** 28 units (24.6% of codebase)

#### Key Modules

| Module Group | Count | Risk Distribution | Focus |
|--------------|-------|-------------------|-------|
| **SYSOP*** | 22 | 4 HIGH, 6 MEDIUM, 12 LOW | System operator menus and functions |
| **Protocols** | - | - | Telnet (RFC 854, 857, 858, 1073), SSH (RFC 4253, 4254) |
| **Terminal** | - | - | ANSI, Avatar, RIP emulation |
| **Doors** | - | - | Door game interface, dropfiles, FOSSIL |

**Sprint Breakdown:**

- **Sprints 19-21:** System Operator Functions
  - SYSOP1-SYSOP11, SYSOP2* modules
  - User management, file management, system config
- **Sprints 22-23:** Protocol Handlers
  - Telnet server (`impulse-telnet` crate)
  - SSH server (`impulse-ssh` crate)
  - Protocol abstraction layer
- **Sprint 24-25:** Terminal Emulation and Door Games
  - ANSI/Avatar/RIP emulation (`impulse-terminal` crate)
  - Door game interface (`impulse-door` crate)
  - Dropfile generation (DOOR.SYS, DORINFO*.DEF)
- **Sprint 26:** Phase 3 Integration and Testing
  - Integration tests for all Phase 3 modules
  - Multi-protocol testing (Telnet + SSH simultaneously)
  - Terminal emulation compatibility testing

**Success Criteria:**

- ✅ SysOp menu functional
- ✅ Message networking operational
- ✅ Telnet connections working
- ✅ SSH connections working
- ✅ ANSI terminal emulation tested
- ✅ Door game interface tested (at least one door game working)

### Phase 4: Integration (Sprints 27-32, Months 19-24)

**Goal:** Complete system integration, high-risk modules, final testing, deployment.

**Duration:** 6 sprints

**Units to Convert:** 30 units (26.3% of codebase, includes highest-risk modules)

#### Key Modules

| Module | Risk Score | Risk Level | Rationale |
|--------|------------|------------|-----------|
| **IMP.PAS** | 22 | CRITICAL | Main program, 69 dependencies, overlay system |
| EXEC.PAS | 31 | CRITICAL | Process execution, DOS interrupts, assembly |
| EXECOLD.PAS | 31 | CRITICAL | Legacy process execution |
| INSTALL.PAS | 25 | CRITICAL | Installation program |
| IMPDOS.PAS | 20 | CRITICAL | DOS-specific operations |
| MAIL7.PAS | 22 | CRITICAL | ABSOLUTE variables |
| SCRIPT.PAS | 24 | CRITICAL | Script execution |
| MENUS.PAS | 13 | HIGH | Menu system, 62 dependencies |
| WFCMENU.PAS | 19 | HIGH | Wait for caller menu, 33 dependencies |

**Sprint Breakdown:**

- **Sprint 27-28:** Main Program and High-Risk Modules
  - IMP.PAS → impulse-server binary
  - EXEC/EXECOLD → process execution abstraction
  - SCRIPT → script interpreter
- **Sprint 29:** Full System Integration
  - All crates integrated into workspace
  - End-to-end testing (complete BBS operation)
  - Migration tools for existing BBS data
- **Sprint 30:** Cross-Platform Testing
  - Linux (primary target)
  - Windows 11 (secondary target)
  - macOS (tertiary target)
  - CI/CD pipeline verification
- **Sprint 31:** Performance Optimization
  - Profiling (flamegraphs, perf)
  - Hotspot optimization
  - Memory usage reduction
  - Concurrency improvements (Tokio)
- **Sprint 32:** Documentation and Release Preparation
  - User documentation (installation, operation, migration)
  - Developer documentation (architecture, contributing)
  - Release notes and changelog
  - v1.0 release candidate

**Success Criteria:**

- ✅ Complete BBS operational (all features working)
- ✅ All 114 modules converted and tested
- ✅ Verified on Linux, Windows 11, and macOS
- ✅ Performance benchmarks met (latency < 100ms for typical operations)
- ✅ Migration tools tested with real Impulse 7.1 BBS data
- ✅ v1.0 release candidate ready for deployment

---

## 6. Type System Migration Strategy

The Pascal type system differs significantly from Rust, requiring careful mapping to preserve functionality while gaining safety.

**Source:** [type-mapping.md](../pascal-reference/type-mapping.md)

### Integer Types

| Pascal Type | Size | Range | Rust Type | Migration Notes |
|-------------|------|-------|-----------|-----------------|
| `Byte` | 1 byte | 0..255 | `u8` | Direct mapping |
| `ShortInt` | 1 byte | -128..127 | `i8` | Direct mapping |
| `Word` | 2 bytes | 0..65535 | `u16` | Direct mapping |
| `Integer` | 2 bytes | -32768..32767 | `i16` | **Most common integer type** |
| `LongInt` | 4 bytes | -2³¹..2³¹-1 | `i32` | Direct mapping |
| `Cardinal` | 4 bytes | 0..2³²-1 | `u32` | Direct mapping |
| `Comp` | 8 bytes | Large integer | `i64` | Rare in codebase |

**Critical Insight:** Pascal `Integer` is 16-bit, while Rust `i32` is the default. Use `i16` for Pascal `Integer` to avoid overflow issues.

### String Types

| Pascal Type | Rust Type | Migration Strategy |
|-------------|-----------|--------------------|
| `String[N]` (fixed-length) | `String` or `[u8; N]` | Dynamic String for flexibility, fixed array for binary compatibility |
| `String` (ShortString, max 255) | `String` | Direct mapping to dynamic string |
| `PChar` (null-terminated) | `&str` or `CString` | Use `&str` for text, `CString` for C interop |
| `AnsiString` (dynamic) | `String` | Direct mapping |

**Binary Compatibility Concern:** For binary file I/O, use `[u8; N]` to match Pascal's fixed-length string layout:

```rust
#[derive(Serialize, Deserialize)]
struct UserRecord {
    // Pascal: name: String[80];
    name: [u8; 80],  // Fixed-length for binary compatibility
    name_len: u8,    // Length byte (Pascal string format)
}

impl UserRecord {
    fn name_as_str(&self) -> &str {
        let len = self.name_len as usize;
        std::str::from_utf8(&self.name[..len]).unwrap_or("")
    }
}
```

### Pointer Types

| Pascal Type | Rust Type | Use Case |
|-------------|-----------|----------|
| `^Type` (pointer) | `Box<Type>` | Heap-allocated, owned |
| `^Type` | `&Type` or `&mut Type` | Borrowed reference (prefer this) |
| `^Type` | `Arc<Type>` | Shared ownership, thread-safe |
| `^Type` | `Rc<Type>` | Shared ownership, single-threaded |
| `nil` | `None` | Use `Option<T>` for nullable pointers |

**Migration Pattern:**

```rust
// Pascal: if ptr <> nil then ptr^.field := value;
// Rust:
if let Some(ref mut data) = ptr_opt {
    data.field = value;
}
```

**Heavy Pointer Usage:** 8 modules have >50 pointer occurrences (COMMON: 107, MAIL1: 139, COMMON1: 85, ANSIEDIT: 66, COMMON2: 62, SYSOP3: 59, INITP: 57, WFCMENU: 55). These require careful refactoring to safe Rust patterns.

### Record Types

Pascal `Record` maps directly to Rust `struct`:

```rust
// Pascal:
// type
//   UserRecord = record
//     name: String[80];
//     age: Integer;
//     active: Boolean;
//   end;

// Rust:
#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserRecord {
    name: String,  // or [u8; 80] for binary compatibility
    age: i16,      // Pascal Integer = i16
    active: bool,
}
```

**Variant Records** (Pascal case...of) → Rust enums with data:

```rust
// Pascal:
// type
//   ShapeType = (Circle, Rectangle);
//   Shape = record
//     case kind: ShapeType of
//       Circle: (radius: Real);
//       Rectangle: (width, height: Real);
//   end;

// Rust:
#[derive(Debug, Clone)]
enum Shape {
    Circle { radius: f32 },
    Rectangle { width: f32, height: f32 },
}
```

### Set Types

Pascal `Set of` types require Rust collections or bitflags:

| Pascal Set | Rust Type | Use Case |
|------------|-----------|----------|
| `Set of Enum` | `HashSet<Enum>` | General purpose, dynamic membership |
| `Set of Byte` | `BitVec` or `[bool; 256]` | Small sets, efficient storage |
| `Set of Char` | `HashSet<char>` | Character sets |
| Flag sets | `bitflags!` macro | Efficient flag storage |

**Example:**

```rust
// Pascal:
// type
//   Flags = (fRead, fWrite, fExec);
//   FlagSet = Set of Flags;

// Rust (bitflags):
use bitflags::bitflags;
bitflags! {
    struct FlagSet: u8 {
        const READ  = 0b001;
        const WRITE = 0b010;
        const EXEC  = 0b100;
    }
}
```

### File Types

| Pascal Type | Rust Type | Strategy |
|-------------|-----------|----------|
| `Text` | `std::fs::File` + `BufReader/Writer` | Text file I/O |
| `File of Type` | `std::fs::File` + `bincode` | Binary file I/O with serialization |
| `File` (untyped) | `std::fs::File` | Raw file access |

**Critical:** 29 files use `File of RecordType` for binary I/O. See [Section 8: Binary File Format Strategy](#8-binary-file-format-strategy) for detailed migration.

---

## 7. Global State Refactoring Strategy

The original Pascal codebase uses extensive global mutable state, which conflicts with Rust's ownership model and thread-safety guarantees.

**Source:** [pascal-globals.md](../pascal-reference/pascal-globals.md)

### Global State Analysis

**Scope:**
- **33 files** export global constants (safe, map to `const` or `static`)
- **90 files** export global mutable variables (problematic, need refactoring)

**Categories of Global State:**

1. **Session State** - Current user, connection info, session timeout
2. **Configuration** - BBS configuration, system paths
3. **File Handles** - Open files (USER.LST, BOARDS.DAT, etc.)
4. **Caches** - User cache, board cache, file area cache
5. **Flags** - System flags, feature toggles

**Example Global Variables (COMMON.PAS):**

```pascal
VAR
  uf: file of userrec;           { USER.LST }
  bf: file of boardrec;          { BOARDS.DAT }
  sf: file of smalrec;           { NAMES.LST }
  thisuser: userrec;             { current user }
  systat: configrec;             { system configuration }
```

### Rust Refactoring Strategy

#### Option 1: BbsState Struct (Recommended)

Encapsulate all global state in a single struct passed as parameter or stored in async context.

```rust
struct BbsState {
    // Configuration
    config: BbsConfig,

    // Session state
    current_user: Option<User>,
    session_start: std::time::Instant,

    // File handles (use Arc<Mutex<T>> for shared mutable access)
    user_db: Arc<Mutex<UserDatabase>>,
    board_db: Arc<Mutex<BoardDatabase>>,

    // Caches
    user_cache: Arc<RwLock<HashMap<UserId, User>>>,

    // Flags
    system_flags: Arc<RwLock<SystemFlags>>,
}

impl BbsState {
    fn new(config: BbsConfig) -> Self {
        Self {
            config,
            current_user: None,
            session_start: std::time::Instant::now(),
            user_db: Arc::new(Mutex::new(UserDatabase::open("data/users.dat")?)),
            board_db: Arc::new(Mutex::new(BoardDatabase::open("data/boards.dat")?)),
            user_cache: Arc::new(RwLock::new(HashMap::new())),
            system_flags: Arc::new(RwLock::new(SystemFlags::default())),
        }
    }
}

// Usage in async context (Tokio)
tokio::task_local! {
    static BBS_STATE: Arc<BbsState>;
}
```

#### Option 2: Thread-Local Storage

For single-threaded or thread-per-connection models:

```rust
use std::cell::RefCell;

thread_local! {
    static BBS_STATE: RefCell<BbsState> = RefCell::new(BbsState::new());
}

fn get_current_user() -> Option<User> {
    BBS_STATE.with(|state| state.borrow().current_user.clone())
}
```

#### Option 3: Static with Mutex (Thread-Safe)

For truly global state accessed from multiple threads:

```rust
use std::sync::Mutex;
use once_cell::sync::Lazy;

static BBS_STATE: Lazy<Mutex<BbsState>> = Lazy::new(|| {
    Mutex::new(BbsState::new())
});

fn get_current_user() -> Option<User> {
    BBS_STATE.lock().unwrap().current_user.clone()
}
```

### Recommended Approach

**Primary:** BbsState struct with Tokio task-local storage (Option 1)

**Rationale:**
- Async-friendly (Tokio-based architecture)
- Per-connection state isolation
- Thread-safe with Arc/Mutex/RwLock
- Explicit state passing (better for testing)

**Migration Steps:**

1. **Sprint 5:** Define BbsState struct with core fields
2. **Sprint 6-7:** Refactor COMMON*.PAS to use BbsState
3. **Sprint 11-18:** Extend BbsState for file/mail modules
4. **Sprint 19-26:** Add protocol-specific state (Telnet, SSH)
5. **Sprint 27-32:** Final integration and optimization

---

## 8. Binary File Format Strategy

The original Impulse 7.1 BBS stores data in binary files using Pascal's `File of RecordType` pattern. Maintaining compatibility is **critical** for migration of existing BBS systems.

**Source:** [pascal-binary-formats.md](../pascal-reference/conversion/pascal-binary-formats.md)

### Binary File Inventory

**Total:** 29 files use binary file I/O

**Critical Data Files:**

| File | Record Type | Purpose | Estimated Size |
|------|-------------|---------|----------------|
| `USER.LST` | `userrec` | User accounts and statistics | ~1KB per user |
| `BOARDS.DAT` | `boardrec` | Message board configuration | ~512B per board |
| `*.DIR` | `ulfrec` | File area listings | ~512B per file |
| `MESSAGES.DAT` | `messagerec` | Message headers | ~256B per message |
| `CONFIG.DAT` | `configrec` | System configuration | ~10KB |
| `PROTOCOL.DAT` | `protrec` | Protocol definitions | ~256B per protocol |
| `UPLOADS.DAT` | `ulrec` | Upload tracking | ~256B per upload |

### Migration Strategy

#### Approach: bincode + serde

Use the `bincode` crate for binary serialization with `serde` derive macros.

**Advantages:**
- Binary format (compact, fast)
- Versioning support (can add version field)
- Compatible with Pascal record layout (with proper configuration)
- Easy migration path (can read old format, write new format)

**Implementation:**

```rust
use serde::{Serialize, Deserialize};
use bincode;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct UserRecord {
    // Match Pascal record layout exactly
    name: [u8; 80],      // String[80] in Pascal
    password: [u8; 20],  // String[20] in Pascal (NEVER store plaintext!)
    age: i16,            // Integer in Pascal
    // ... other fields matching Pascal layout
}

// Reading legacy Pascal format
fn read_legacy_user(file: &mut File) -> Result<UserRecord, BbsError> {
    let mut buffer = vec![0u8; std::mem::size_of::<UserRecord>()];
    file.read_exact(&mut buffer)?;
    Ok(bincode::deserialize(&buffer)?)
}

// Writing new format (with versioning)
#[derive(Serialize, Deserialize)]
struct UserRecordV2 {
    version: u32,  // Always 2
    name: String,  // Dynamic string
    password_hash: String,  // Argon2 hash
    age: i16,
    // ... other fields
}
```

#### Migration Tools

**Required:** Migration tool to convert existing BBS data to new format (or maintain backward compatibility).

**Tool Requirements:**

1. **Read Old Format** - Parse Pascal binary files
2. **Convert Data** - Transform to new Rust types
3. **Validate** - Ensure data integrity
4. **Write New Format** - Save in Rust binary format or structured format (JSON, TOML, SQLite)

**Example Migration Tool:**

```rust
// impulse-cli tool: migrate command
fn migrate_user_database(
    old_path: &Path,
    new_path: &Path,
) -> Result<(), BbsError> {
    let mut old_file = File::open(old_path)?;
    let mut new_db = UserDatabase::create(new_path)?;

    // Read all users from old format
    while let Ok(old_user) = read_legacy_user(&mut old_file) {
        // Convert to new format
        let new_user = User {
            name: String::from_utf8_lossy(&old_user.name).into_owned(),
            // Hash old password (or require password reset)
            password_hash: hash_password(&old_user.password)?,
            age: old_user.age,
            // ... other fields
        };

        // Write to new database
        new_db.insert(new_user)?;
    }

    Ok(())
}
```

#### Backward Compatibility Option

**Alternative:** Maintain backward compatibility by supporting both formats.

```rust
enum UserRecordFormat {
    Legacy(LegacyUserRecord),
    Modern(UserRecord),
}

impl UserDatabase {
    fn read_user(&mut self, id: UserId) -> Result<User, BbsError> {
        match self.detect_format()? {
            UserRecordFormat::Legacy(_) => self.read_legacy_user(id),
            UserRecordFormat::Modern(_) => self.read_modern_user(id),
        }
    }
}
```

#### Testing Strategy

1. **Round-Trip Tests** - Write with Rust, read with Pascal (and vice versa)
2. **Data Validation** - Checksum verification, integrity checks
3. **Migration Tests** - Convert real BBS data, verify all fields
4. **Performance Tests** - Benchmark read/write speed vs. Pascal original

**Sprint Allocation:**

- **Sprint 8-9:** Implement binary file I/O with bincode
- **Sprint 29:** Create migration tools and test with real BBS data

---

## 9. Dependency Management Strategy

The conversion order must respect module dependencies to avoid circular dependencies and minimize rework.

**Source:** [pascal-dependencies.md](../pascal-reference/analysis/pascal-dependencies.md), [pascal-dependency-matrix.csv](../pascal-reference/pascal-dependency-matrix.csv)

### Dependency Statistics

**Total Dependencies:** 1,070 USES relationships
**Average Dependencies per Unit:** 9.4

**Top Dependencies (used by most modules):**

| Module | Used By | Category | Priority |
|--------|---------|----------|----------|
| **RECORDS** | 93 units | Core Types | **CONVERT FIRST** |
| **COMMON** | 86 units | Common Utilities | Phase 1 |
| **COMMON5** | 88 units | Common Utilities | Phase 1 |
| **COMMON1** | 87 units | Common Utilities | Phase 1 |
| **COMMON2** | 86 units | Common Utilities | Phase 1 |
| **COMMON3** | 87 units | Common Utilities | Phase 1 |
| **STRPROC** | 88 units | String Utilities | Phase 1 |
| **MYIO** | 89 units | I/O Utilities | Phase 1 |

**Top Consumers (use most modules):**

| Module | Uses | Category | Priority |
|--------|------|----------|----------|
| **IMP** | 69 units | Main Program | **Phase 4 (last)** |
| **MENUS** | 62 units | Menu System | Phase 3 |
| **WFCMENU** | 33 units | WFC Menu | Phase 4 |
| **LOGON1** | 26 units | Logon | Phase 2 |
| **LOGON2** | 23 units | Logon | Phase 2 |
| **SYSOP2** | 21 units | SysOp | Phase 3 |

### Dependency Resolution Rules

1. **Foundation First** - Convert modules with zero dependencies and high impact (RECORDS, STRPROC, TIMEJUNK)
2. **Layer by Layer** - Convert modules whose dependencies are already converted
3. **Defer Main Program** - IMP.PAS (69 deps) must be converted last
4. **Circular Dependencies** - Break circular deps by refactoring shared code into new module

**Circular Dependency Detection:**

Found via [pascal-dependencies.dot](../pascal-reference/pascal-dependencies.dot) graph analysis:

- COMMON ↔ OUTPUT
- MENUS ↔ NUV
- FILE* modules (multiple circular chains)

**Resolution Strategy:** Extract shared interfaces into new modules or use dependency injection.

### Dependency Visualization

**Tool:** Graphviz dependency graph available at [pascal-dependencies.svg](../pascal-reference/pascal-dependencies.svg) (556KB, 114 nodes, 1,070 edges)

**Color Legend:**
- **Red:** Main Program (IMP.PAS)
- **Cyan:** Core Types (RECORDS.PAS)
- **Blue:** Common Utilities (COMMON*.PAS)
- **Orange:** File Management (FILE*.PAS)
- **Green:** Mail/Message (MAIL*.PAS)
- **Yellow:** SysOp (SYSOP*.PAS)
- **Purple:** Miscellaneous (MISC*.PAS)
- **Gray:** Other modules

---

## 10. Testing Strategy

Comprehensive testing is essential to ensure behavioral equivalence between Pascal original and Rust conversion.

**Source:** [SPRINT-03-COMPLETION-REPORT.md](../reports/sprints/SPRINT-03-COMPLETION-REPORT.md), [risk-mitigations.md](../pascal-reference/risk-assessment/risk-mitigations.md)

### Test Infrastructure (Sprint 8)

**Required Infrastructure:**

1. **Unit Test Framework** - Standard Rust testing (`#[test]`, `#[cfg(test)]`)
2. **Integration Test Framework** - `tests/` directory with cross-crate tests
3. **Property-Based Testing** - `proptest` crate for complex logic
4. **Binary Compatibility Tests** - Read/write Pascal data files
5. **Characterization Tests** - Test Pascal original as oracle for Rust conversion

**Implementation:**

```rust
// Unit test example
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_serialization() {
        let user = User {
            name: "testuser".to_string(),
            age: 25,
            // ...
        };

        // Serialize to binary
        let encoded = bincode::serialize(&user).unwrap();

        // Deserialize
        let decoded: User = bincode::deserialize(&encoded).unwrap();

        assert_eq!(user.name, decoded.name);
        assert_eq!(user.age, decoded.age);
    }

    #[test]
    fn test_binary_compatibility_with_pascal() {
        // Read Pascal-generated USER.LST
        let legacy_data = std::fs::read("tests/fixtures/USER.LST").unwrap();
        let user: User = bincode::deserialize(&legacy_data).unwrap();

        // Verify fields match expected values
        assert_eq!(user.name, "sysop");
    }
}
```

### Test Coverage Targets

| Risk Level | Coverage Target | Rationale |
|------------|----------------|-----------|
| **CRITICAL** | 95%+ | Expert review, extensive testing required |
| **HIGH** | 85%+ | Senior dev + peer review |
| **MEDIUM** | 75%+ | Standard process |
| **LOW** | 60%+ | Basic validation |

### Testing Phases

#### Phase 1: Pre-Conversion (Characterization Tests)

**Goal:** Document expected behavior of Pascal original.

**Method:**
1. Run Pascal original with test inputs
2. Capture outputs
3. Create characterization tests in Rust

```rust
#[test]
fn characterization_test_user_login() {
    // Run Pascal original: IMPULSE.EXE with scripted input
    // Capture output
    // Compare Rust implementation output to Pascal output
}
```

#### Phase 2: During Conversion (Unit Tests)

**Goal:** Test each converted function/module in isolation.

**Method:**
1. Write unit tests before converting module
2. Convert Pascal → Rust
3. Run tests, iterate until passing
4. Code review

#### Phase 3: Post-Conversion (Integration Tests)

**Goal:** Test interactions between converted modules.

**Method:**
1. End-to-end scenarios (user login → file upload → message post → logout)
2. Multi-module tests (file area + message base)
3. Protocol tests (Telnet/SSH connections)

#### Phase 4: Regression Tests (Continuous)

**Goal:** Ensure no breaking changes during development.

**Method:**
1. Run full test suite in CI on every commit
2. Performance regression tests (compare to baseline)
3. Memory leak detection (valgrind, ASAN)

### Cross-Platform Testing

**Platforms:**
- Linux (Ubuntu 24.04, primary target)
- Windows 11 (secondary target)
- macOS (tertiary target)

**CI Matrix:**

```yaml
# .github/workflows/test.yml
strategy:
  matrix:
    os: [ubuntu-latest, windows-latest, macos-latest]
    rust: [stable, beta]
```

### Test BBS Environment

**Setup:** Maintain test BBS with sample data for integration testing.

**Data:**
- 100 test users
- 50 test messages
- 20 test files
- 10 test boards
- Real Impulse 7.1 data files (for binary compatibility testing)

**Usage:**
- Integration tests
- Manual testing
- Demonstration environment

---

## 11. High-Risk Module Strategy

**38 modules** (33.3% of codebase) are classified as HIGH or CRITICAL risk, requiring special handling.

**Source:** [high-risk-units.md](../pascal-reference/high-risk-units.md), [risk-mitigations.md](../pascal-reference/risk-assessment/risk-mitigations.md)

### CRITICAL Risk Modules (11 units, 9.6%)

**Modules:** EXEC, EXECOLD, RECORDS, COMMON, COMMON5, INSTALL, SCRIPT, IMP, MAIL7, BPTRAP, IMPDOS

**Special Handling:**

1. **Senior Developer Assignment** - Only senior developers convert these modules
2. **Pair Programming** - Two developers work together (driver + navigator)
3. **Extended Timeline** - Allocate 2x normal time for CRITICAL modules
4. **Code Review** - Mandatory peer review by another senior developer
5. **Extensive Testing** - 95%+ coverage, property-based testing, manual testing
6. **Phased Conversion** - Convert in small increments, test frequently

**Example: RECORDS.PAS (Score 28, CRITICAL)**

**Challenges:**
- DOS interrupts, inline assembly, process execution
- 830 lines, core type definitions
- **93 modules depend on it** (highest dependency impact)

**Strategy:**
1. **Sprint 5:** Full sprint dedicated to RECORDS.PAS
2. **Team:** 2 senior developers (pair programming)
3. **Approach:**
   - Convert record types one at a time
   - Write unit tests for each type
   - Implement bincode serialization
   - Verify binary compatibility with Pascal original
4. **Testing:** 95%+ coverage, binary round-trip tests
5. **Review:** Mandatory peer review + architecture review

### HIGH Risk Modules (27 units, 23.7%)

**Modules:** WFCMENU, INITP, TMPCOM, TMPOLD, SYSOP2S, COMMON2, FILE5, LOGON1, ANSIEDIT, COMMON1, FILE8, DOORS, FILE1, SYSOP3, FILE12, MENUS, MISC2, CMD, SYSOP6, TIMETASK, MENUS2, EXECBAT, FILE2, NETFOSSL, SYS, SYSOP2D, ZIPVIEWU

**Special Handling:**

1. **Senior Developer Assignment** - Senior or strong mid-level developer
2. **Code Review** - Mandatory peer review
3. **Integration Tests** - Test interactions with other modules
4. **Extended Testing** - 85%+ coverage

**Example: TMPCOM.PAS (Score 18, HIGH)**

**Challenges:**
- Interrupt handlers (platform-specific)
- Inline assembly
- 754 lines, serial port communication

**Strategy:**
1. **Sprint:** Allocated to Phase 4 (later conversion)
2. **Team:** Senior developer
3. **Approach:**
   - Replace interrupt handlers with signal-hook (Unix) or SetConsoleCtrlHandler (Windows)
   - Replace inline assembly with safe Rust or std::arch intrinsics
   - Abstract serial port I/O behind trait
4. **Testing:** 85%+ coverage, cross-platform tests

---

## 12. Sprint Execution Guidelines

### Sprint Structure (3-week sprints)

**Week 1: Planning & Setup**
- Sprint planning meeting (2 hours)
- Review analysis documentation
- Assign modules to developers
- Set up feature branches

**Week 2: Implementation**
- Convert Pascal → Rust
- Write unit tests
- Daily stand-ups (15 minutes)
- Mid-sprint check-in (1 hour)

**Week 3: Testing & Review**
- Integration tests
- Code review
- Sprint review (1 hour)
- Sprint retrospective (1 hour)

### Definition of Done

A module is considered "done" when:

1. ✅ **Converted** - All Pascal code converted to Rust
2. ✅ **Tests Passing** - Unit tests + integration tests passing
3. ✅ **Coverage Met** - Test coverage meets target for risk level
4. ✅ **Reviewed** - Code review completed and approved
5. ✅ **Documented** - Migration notes documented
6. ✅ **Integrated** - Merged to main branch
7. ✅ **CI Passing** - All CI checks passing (format, lint, test, build)

### Development Workflow

1. **Create Feature Branch** - `git checkout -b feature/convert-records-pas`
2. **Convert Module** - Translate Pascal → Rust
3. **Write Tests** - Unit tests + integration tests
4. **Run Quality Checks** - `cargo fmt`, `cargo clippy`, `cargo test`
5. **Commit Changes** - Conventional commits (`feat:`, `fix:`, etc.)
6. **Create Pull Request** - Include migration notes in description
7. **Code Review** - Address reviewer feedback
8. **Merge** - Squash merge to main
9. **Update Documentation** - Mark module as complete in conversion-order.md

### Team Structure

**Recommended Team Size:** 3-5 developers

**Roles:**
- **Tech Lead** - Architecture, code review, CRITICAL modules
- **Senior Developer** - CRITICAL/HIGH modules, code review
- **Mid-Level Developer** - HIGH/MEDIUM modules
- **Junior Developer** - LOW modules, testing

### Communication

- **Daily Stand-ups** - 15 minutes, async (Slack/Discord) or sync (video)
- **Weekly Syncs** - 1 hour, review progress, blockers
- **Sprint Planning** - 2 hours, every 3 weeks
- **Sprint Review** - 1 hour, demonstrate completed work
- **Sprint Retrospective** - 1 hour, continuous improvement

---

## 13. Success Metrics & KPIs

### Progress Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| **Modules Converted** | 114/114 (100%) | Count of completed modules |
| **Lines of Code Converted** | 39,079/39,079 (100%) | Total Rust LOC vs. Pascal LOC |
| **Test Coverage** | 80%+ overall | `cargo tarpaulin` |
| **CRITICAL Modules Coverage** | 95%+ | Per-module coverage |
| **HIGH Modules Coverage** | 85%+ | Per-module coverage |
| **CI Success Rate** | 100% | CI passing on main branch |

### Quality Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| **Clippy Warnings** | 0 | `cargo clippy -- -D warnings` |
| **rustfmt Compliance** | 100% | `cargo fmt -- --check` |
| **Binary Compatibility** | 100% | Round-trip tests passing |
| **Cross-Platform Tests** | 100% passing | Linux + Windows + macOS |

### Performance Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| **User Login Latency** | < 100ms | Benchmark vs. Pascal original |
| **File Upload Throughput** | ≥ Pascal original | MB/s comparison |
| **Message Post Latency** | < 50ms | Benchmark vs. Pascal original |
| **Memory Usage** | ≤ 2x Pascal original | RSS measurement |
| **Concurrent Users** | 100+ | Load testing with vegeta |

### Sprint Velocity

**Target:** 3-4 modules per sprint (average)

| Phase | Sprints | Modules | Avg/Sprint |
|-------|---------|---------|------------|
| **Phase 1** | 8 | 28 | 3.5 |
| **Phase 2** | 8 | 28 | 3.5 |
| **Phase 3** | 8 | 28 | 3.5 |
| **Phase 4** | 6 | 30 | 5.0 |

**Note:** Phase 4 has higher velocity due to parallelization and infrastructure reuse.

---

## 14. Risk Mitigation Timeline

### Critical Risks & Mitigation Schedule

| Risk | Impact | Probability | Mitigation | Sprint |
|------|--------|-------------|------------|--------|
| **Binary Compatibility Failure** | HIGH | MEDIUM | Design serialization strategy early, test with real data | 5, 8-9 |
| **Platform-Specific Code Blocking** | HIGH | MEDIUM | Create abstraction layer early, cross-platform testing | 6-7, 30 |
| **Global State Refactoring Complexity** | MEDIUM | HIGH | Design BbsState architecture early, incremental migration | 5, 6-7 |
| **High-Risk Module Conversion Delays** | HIGH | MEDIUM | Allocate senior devs, 2x time, pair programming | Throughout |
| **Dependency Circular Dependencies** | MEDIUM | LOW | Refactor shared code, dependency injection | As discovered |
| **Performance Regression** | MEDIUM | MEDIUM | Continuous benchmarking, optimization sprint | 31 |

### Mitigation Actions by Phase

**Phase 1 (Sprints 3-10):**
- ✅ Design BbsState architecture (Sprint 5)
- ✅ Implement binary file I/O with bincode (Sprint 8-9)
- ✅ Create platform abstraction layer (Sprint 6-7)
- ✅ Convert RECORDS.PAS with 95%+ coverage (Sprint 5)

**Phase 2 (Sprints 11-18):**
- ✅ Validate binary compatibility with real BBS data (Sprint 18)
- ✅ Extend BbsState for file/mail modules (Sprints 11-16)
- ✅ Performance baseline measurement (Sprint 18)

**Phase 3 (Sprints 19-26):**
- ✅ Protocol abstraction layer (Sprints 22-23)
- ✅ Cross-platform testing (all sprints)
- ✅ High-risk module conversion (SysOp, protocols)

**Phase 4 (Sprints 27-32):**
- ✅ IMP.PAS conversion (Sprint 27-28)
- ✅ Full system integration (Sprint 29)
- ✅ Cross-platform testing (Sprint 30)
- ✅ Performance optimization (Sprint 31)

---

## 15. Parallel Conversion Opportunities

### Parallelization Strategy

**Goal:** Reduce sequential conversion time by 30-40% through parallel work.

**Prerequisite:** RECORDS.PAS (core types) must be complete before parallelization.

### Independent Module Groups

Once RECORDS.PAS is complete (Sprint 5), the following groups can be converted in parallel:

#### Group 1: FILE* Modules (14 units)

**Modules:** FILE0-FILE14

**Rationale:**
- Each file area module is relatively independent
- All depend on RECORDS.PAS (already converted)
- Minimal cross-dependencies within group

**Sprint Allocation:** Sprints 11-13 (3 sprints)

**Team Assignment:**
- Developer A: FILE0, FILE1, FILE2, FILE3, FILE4
- Developer B: FILE5, FILE6, FILE8, FILE9
- Developer C: FILE10, FILE11, FILE12, FILE13, FILE14

#### Group 2: MAIL* Modules (9 units)

**Modules:** MAIL0-MAIL9

**Rationale:**
- Message system modules share similar structure
- All depend on RECORDS.PAS and COMMON*.PAS
- Can be split into message base (MAIL0-4) and networking (MAIL5-9)

**Sprint Allocation:** Sprints 14-16 (3 sprints)

**Team Assignment:**
- Developer A: MAIL0, MAIL1, MAIL2
- Developer B: MAIL3, MAIL4, MAIL5
- Developer C: MAIL6, MAIL7, MAIL9

#### Group 3: SYSOP* Modules (22 units)

**Modules:** SYSOP1-SYSOP11, SYSOP2*, SYSOP21, SYSOP3, SYSOP6-9, SYSOP7M

**Rationale:**
- System operator functions are largely independent
- Can be split into user management, file management, system config

**Sprint Allocation:** Sprints 19-21 (3 sprints)

**Team Assignment:**
- Developer A: SYSOP1, SYSOP2, SYSOP3, SYSOP6, SYSOP7
- Developer B: SYSOP8, SYSOP9, SYSOP11, SYSOP21
- Developer C: SYSOP2A-SYSOP2J (sub-modules of SYSOP2)

#### Group 4: MISC* Modules (5 units)

**Modules:** MISC1-MISC5

**Rationale:**
- Miscellaneous utilities, independent

**Sprint Allocation:** Sprints 14-16 (concurrent with MAIL*)

**Team Assignment:**
- Developer D: MISC1, MISC2, MISC3, MISC4, MISC5

### Estimated Time Savings

**Sequential Conversion:**
- 114 modules × 3.5 modules/sprint = 33 sprints
- 33 sprints × 3 weeks = 99 weeks (23 months)

**Parallel Conversion (Actual Plan):**
- 30 sprints × 3 weeks = 90 weeks (21 months)
- **Time Savings:** 9 weeks (2 months, 9%)

**Additional Parallelization (If Team Size Allows):**
- With 5 developers, estimated time reduction: 30-40%
- Potential timeline: 18-21 months

### Coordination Overhead

**Challenges:**
- Merge conflicts (resolved with good branch hygiene)
- Integration issues (mitigated with frequent integration tests)
- Communication overhead (mitigated with daily stand-ups)

**Mitigation:**
- Daily stand-ups (15 minutes)
- Shared documentation (conversion notes, architecture decisions)
- Continuous integration (test on every commit)

---

## 16. Cross-References

### Internal Documentation

**Pascal Analysis Files:**
1. [pascal-inventory.md](../pascal-reference/analysis/pascal-inventory.md) - Complete inventory of 114 units
2. [pascal-unit-analysis.md](../pascal-reference/analysis/pascal-unit-analysis.md) - Detailed unit analysis
3. [pascal-dependencies.md](../pascal-reference/analysis/pascal-dependencies.md) - 1,070 dependencies documented
4. [pascal-dependency-matrix.csv](../pascal-reference/pascal-dependency-matrix.csv) - Structured dependency data
5. [pascal-globals.md](../pascal-reference/pascal-globals.md) - Global constants and variables
6. [pascal-overlays.md](../pascal-reference/analysis/pascal-overlays.md) - Overlay system analysis
7. [pascal-interrupts.md](../pascal-reference/analysis/pascal-interrupts.md) - Interrupt handlers and assembly
8. [pascal-dos-specific.md](../pascal-reference/analysis/pascal-dos-specific.md) - DOS function dependencies
9. [pascal-binary-formats.md](../pascal-reference/conversion/pascal-binary-formats.md) - Binary file formats
10. [type-mapping.md](../pascal-reference/type-mapping.md) - Pascal→Rust type mappings
11. [conversion-risk-assessment.md](../pascal-reference/risk-assessment/conversion-risk-assessment.md) - Risk ratings
12. [high-risk-units.md](../pascal-reference/high-risk-units.md) - 38 high-risk units detailed
13. [risk-mitigations.md](../pascal-reference/risk-assessment/risk-mitigations.md) - Mitigation strategies
14. [conversion-order.md](../pascal-reference/conversion/conversion-order.md) - Priority-ordered conversion plan
15. [dependencies.json](../pascal-reference/dependencies.json) - Machine-readable dependencies
16. [risk-data.json](../pascal-reference/risk-data.json) - Machine-readable risk scores
17. [SPRINT-03-COMPLETION-REPORT.md](../reports/sprints/SPRINT-03-COMPLETION-REPORT.md) - Sprint 3 summary
18. [pascal-dependencies.dot](../pascal-reference/pascal-dependencies.dot) - Graphviz dependency graph
19. [pascal-dependencies.svg](../pascal-reference/pascal-dependencies.svg) - Visual dependency graph

**Project Documentation:**
- [README.md](../README.md) - Project overview
- [CHANGELOG.md](../CHANGELOG.md) - Version history
- [CONTRIBUTING.md](../CONTRIBUTING.md) - Contribution guidelines
- [CLAUDE.md](../CLAUDE.md) - Project memory

**Sprint TODO Files:**
- [to-dos/phase-1-foundation/](../to-dos/phase-1-foundation/) - Sprints 1-8
- [to-dos/phase-2-core-services/](../to-dos/phase-2-core-services/) - Sprints 9-16
- [to-dos/phase-3-advanced-features/](../to-dos/phase-3-advanced-features/) - Sprints 17-24
- [to-dos/phase-4-polish-deployment/](../to-dos/phase-4-polish-deployment/) - Sprints 25-32

### External References

**Rust Resources:**
- [Rust Book](https://doc.rust-lang.org/book/) - Official Rust documentation
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn Rust with examples
- [Rust std library](https://doc.rust-lang.org/std/) - Standard library documentation
- [bincode crate](https://docs.rs/bincode/) - Binary serialization
- [serde crate](https://docs.rs/serde/) - Serialization framework
- [Tokio](https://tokio.rs/) - Async runtime

**Pascal Resources:**
- [Borland Pascal 7.0 Reference](https://www.freepascal.org/docs-html/ref/ref.html) - Language reference
- [Turbo Pascal Documentation](http://www.bitsavers.org/pdf/borland/turbo_pascal/) - Historical documentation

**BBS Resources:**
- [BBS History](https://www.bbsdocumentary.com/) - BBS documentary
- [ANSI Art](http://www.roysac.com/learn-ansi.html) - ANSI terminal codes
- [Telnet Protocol](https://www.rfc-editor.org/rfc/rfc854) - RFC 854

### Cross-Reference Matrix

**By Topic:**

| Topic | Primary Document | Supporting Documents |
|-------|------------------|---------------------|
| **Risk Assessment** | [conversion-risk-assessment.md](../pascal-reference/risk-assessment/conversion-risk-assessment.md) | [high-risk-units.md](../pascal-reference/high-risk-units.md), [risk-mitigations.md](../pascal-reference/risk-assessment/risk-mitigations.md) |
| **Dependencies** | [pascal-dependencies.md](../pascal-reference/analysis/pascal-dependencies.md) | [pascal-dependency-matrix.csv](../pascal-reference/pascal-dependency-matrix.csv), [dependencies.json](../pascal-reference/dependencies.json), [pascal-dependencies.svg](../pascal-reference/pascal-dependencies.svg) |
| **Type Mapping** | [type-mapping.md](../pascal-reference/type-mapping.md) | [pascal-globals.md](../pascal-reference/pascal-globals.md), [pascal-binary-formats.md](../pascal-reference/conversion/pascal-binary-formats.md) |
| **Platform Migration** | [pascal-dos-specific.md](../pascal-reference/analysis/pascal-dos-specific.md) | [pascal-interrupts.md](../pascal-reference/analysis/pascal-interrupts.md), [pascal-overlays.md](../pascal-reference/analysis/pascal-overlays.md) |
| **Conversion Order** | [conversion-order.md](../pascal-reference/conversion/conversion-order.md) | [pascal-dependencies.md](../pascal-reference/analysis/pascal-dependencies.md), [conversion-risk-assessment.md](../pascal-reference/risk-assessment/conversion-risk-assessment.md) |

---

**Document Status:** Final
**Next Review:** Sprint 4 (Configuration System implementation)
**Feedback:** Update this document as conversion progresses to reflect lessons learned
