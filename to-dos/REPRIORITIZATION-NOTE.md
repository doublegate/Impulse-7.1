# Sprint Reprioritization Note

**Date:** 2025-11-23
**Reason:** Configuration infrastructure identified as critical dependency

## Background

After completing Sprint 2 (Core Types), a gap analysis revealed that proceeding directly to Sprint 3 (File Parsing) would create technical debt. Configuration management infrastructure needed to be implemented before building service layers.

## Decision

**POSTPONED:**
- Sprint 3: File Parsing (Pascal .DAT file reading)
- Sprint 4: ANSI Engine  
- Sprint 5: Basic Telnet Protocol

**IMPLEMENTED (Gap Work):**
1. Configuration hot-reload system (impulse-config)
2. Configuration management CLI (impconfig binary)

## Rationale

### Why Configuration First?

1. **Service Dependency:** All future services (user system, message system, file management) require configuration
2. **Hot-Reload Requirement:** Production BBS systems need config updates without restart
3. **Sysop Tooling:** Admins need CLI tools to manage configurations before services exist
4. **Testing Foundation:** Config validation enables comprehensive testing of service configurations

### Why Not File Parsing Now?

1. **No Dependencies:** File management services don't exist yet
2. **Integration Complexity:** Pascal file format requires working message/file systems to test
3. **Lower Priority:** Config management provides more immediate value

## Impact on Sprint Numbers

**Original Plan (from docs/01-phase-sprint-plan.md):**
- Sprint 1: Project Setup [COMPLETED]
- Sprint 2: Core Types [COMPLETED]
- Sprint 3: File Parsing [DEFERRED]
- Sprint 4: ANSI Engine [PLANNED]
- Sprint 5: Basic Telnet [PLANNED]
- Sprint 6: User System [PLANNED]
- Sprint 7: Security & Auth [PLANNED]
- Sprint 8: Testing & CI [PLANNED]

**Actual Execution:**
- Sprint 1: Project Setup [COMPLETED]
- Sprint 2: Core Types [COMPLETED]
- **Gap Work:** Config Hot-Reload + CLI [COMPLETED] ← This work
- Sprint 3+: TBD based on revised priorities

## File Organization

### Sprint TODO Files

The sprint TODO files in `to-dos/phase-1-foundation/` reflect the ORIGINAL plan:
- `sprint-03-file-parsing.md` - NOT YET EXECUTED
- `sprint-04-ansi-engine.md` - NOT YET EXECUTED  
- `sprint-05-telnet-basic.md` - NOT YET EXECUTED

These files are preserved for historical reference. When these features are implemented, they may be:
1. Executed in different order (e.g., User System before File Parsing)
2. Split/combined based on actual work breakdown
3. Updated with lessons learned from earlier sprints

### Completion Documentation

See `docs/SPRINT-4-5-GAP-COMPLETION.md` for detailed notes on gap work.

## Future Sprint Planning

The next sprint (Sprint 3 in execution sequence) will be determined by:
1. Dependency analysis (what do we need next?)
2. User priorities (what features provide most value?)
3. Risk mitigation (what reduces technical debt?)

Likely candidates for next sprint:
- **User System (original Sprint 6):** Foundation for authentication and sessions
- **Security & Auth (original Sprint 7):** Required before network services
- **File Parsing (original Sprint 3):** When file management becomes priority

## Lessons Learned

1. **Flexible Sprint Planning:** Original sprint sequence was based on Pascal codebase structure, not optimal Rust architecture
2. **Infrastructure First:** Config management is more critical than file parsing
3. **Just-In-Time Planning:** Detailed sprint plans should be reviewed before execution
4. **Document Deviations:** This note ensures future developers understand sprint history

## References

- Original Plan: `docs/01-phase-sprint-plan.md`
- Gap Completion: `docs/SPRINT-4-5-GAP-COMPLETION.md`
- Sprint 1-2 Verification: `SPRINT-01-02-VERIFICATION-REPORT.md`

---

**Note:** This file explains why sprint numbers don't match execution order. Sprint TODO files are planning documents, not execution records.
