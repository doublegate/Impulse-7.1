# Pascal to Rust Conversion Priority Order

**Total Units:** 114
**Analysis Date:** 2025-11-23

## Priority Methodology

Conversion order determined by:

1. **Dependency Impact** (most important)
   - Units with many dependents converted first
   - Establishes foundation for other conversions

2. **Risk Level**
   - Low-risk units preferred for early conversion
   - High-risk units deferred until foundation solid

3. **Dependency Count**
   - Units with fewer dependencies easier to convert early
   - Complex dependency chains handled later

**Priority Formula:**
```
priority_score = (risk_score * 10) + (dependency_count * 2) - (dependency_impact * 5)
Lower score = Higher priority (convert earlier)
```

## Phase 1 - Foundation (28 units)

| Priority | Unit | Risk Level | Dep Impact | Dependencies | Rationale |
|----------|------|------------|------------|--------------|-----------|
| 1 | STRPROC | LOW | 88 | 0 | High dependency impact, Low risk |
| 2 | TIMEJUNK | LOW | 87 | 1 | High dependency impact, Low risk |
| 3 | SCRLBK | LOW | 86 | 3 | High dependency impact, Low risk |
| 4 | COMMON3 | LOW | 87 | 7 | High dependency impact, Low risk, Common utilities |
| 5 | COMMON4 | LOW | 86 | 5 | High dependency impact, Low risk, Common utilities |
| 6 | OUTPUT | LOW | 86 | 8 | High dependency impact, Low risk |
| 7 | MYIO | MEDIUM | 89 | 2 | High dependency impact, Medium risk |
| 8 | ASMSAUCE | MEDIUM | 88 | 0 | High dependency impact, Medium risk |
| 9 | CMD | HIGH | 86 | 6 | High dependency impact, HIGH risk |
| 10 | COMMON1 | HIGH | 87 | 5 | High dependency impact, HIGH risk, Common utilities |
| 11 | TMPCOM | HIGH | 89 | 1 | High dependency impact, HIGH risk |
| 12 | COMMON2 | HIGH | 86 | 11 | High dependency impact, HIGH risk, Common utilities |
| 13 | FILE3 | LOW | 40 | 2 | High dependency impact, Low risk |
| 14 | RECORDS | CRITICAL | 93 | 0 | High dependency impact, CRITICAL risk, Core types |
| 15 | COMMON5 | CRITICAL | 88 | 1 | High dependency impact, CRITICAL risk, Common utilities |
| 16 | ANSIDRV | MEDIUM | 48 | 2 | High dependency impact, Medium risk |
| 17 | FILE14 | LOW | 36 | 8 | High dependency impact, Low risk |
| 18 | COMMON | CRITICAL | 86 | 5 | High dependency impact, CRITICAL risk, Common utilities |
| 19 | SYS | HIGH | 47 | 2 | High dependency impact, HIGH risk |
| 20 | ULCHECK | LOW | 39 | 12 | High dependency impact, Low risk |
| 21 | EXECBAT | HIGH | 47 | 9 | High dependency impact, HIGH risk |
| 22 | FILE0 | MEDIUM | 39 | 10 | High dependency impact, Medium risk |
| 23 | FILE11 | MEDIUM | 37 | 13 | High dependency impact, Medium risk |
| 24 | MULTINOD | MEDIUM | 41 | 10 | High dependency impact, Medium risk |
| 25 | ISLU | LOW | 20 | 3 | Medium dependency impact, Low risk |
| 26 | FILE4 | MEDIUM | 35 | 7 | High dependency impact, Medium risk |
| 27 | TIMEBANK | LOW | 19 | 4 | Medium dependency impact, Low risk |
| 28 | SYSOP2Z | LOW | 19 | 7 | Medium dependency impact, Low risk |

## Phase 2 - Core Services (28 units)

| Priority | Unit | Risk Level | Dep Impact | Dependencies | Rationale |
|----------|------|------------|------------|--------------|-----------|
| 29 | SYSOP2E | LOW | 19 | 8 | Medium dependency impact, Low risk |
| 30 | SYSOP2F | LOW | 19 | 8 | Medium dependency impact, Low risk |
| 31 | MISC1 | MEDIUM | 28 | 9 | High dependency impact, Medium risk |
| 32 | MAIL0 | LOW | 23 | 7 | High dependency impact, Low risk |
| 33 | FILE9 | LOW | 27 | 13 | High dependency impact, Low risk |
| 34 | MISC4 | LOW | 20 | 11 | Medium dependency impact, Low risk |
| 35 | MISC3 | LOW | 21 | 8 | High dependency impact, Low risk |
| 36 | MENUS3 | LOW | 20 | 7 | Medium dependency impact, Low risk |
| 37 | MMODEM | LOW | 20 | 7 | Medium dependency impact, Low risk |
| 38 | CUSER | LOW | 23 | 10 | High dependency impact, Low risk |
| 39 | SYSOP7M | LOW | 20 | 8 | Medium dependency impact, Low risk |
| 40 | UCONFIG | LOW | 20 | 10 | Medium dependency impact, Low risk |
| 41 | SYSOP21 | LOW | 19 | 8 | Medium dependency impact, Low risk |
| 42 | FILE13 | LOW | 19 | 9 | Medium dependency impact, Low risk |
| 43 | SYSOP2C | LOW | 19 | 9 | Medium dependency impact, Low risk |
| 44 | SYSOP2H | LOW | 19 | 10 | Medium dependency impact, Low risk |
| 45 | MISC5 | MEDIUM | 25 | 11 | High dependency impact, Medium risk |
| 46 | SYSOP2G | LOW | 18 | 10 | Medium dependency impact, Low risk |
| 47 | FILE2 | HIGH | 31 | 8 | High dependency impact, HIGH risk |
| 48 | MAIL2 | LOW | 20 | 12 | Medium dependency impact, Low risk |
| 49 | MAIL3 | LOW | 20 | 13 | Medium dependency impact, Low risk |
| 50 | MSGPACK | MEDIUM | 19 | 6 | Medium dependency impact, Medium risk |
| 51 | SYSOP1 | LOW | 19 | 11 | Medium dependency impact, Low risk |
| 52 | SYSOP7 | LOW | 19 | 13 | Medium dependency impact, Low risk |
| 53 | STRPROC2 | MEDIUM | 20 | 1 | Medium dependency impact, Medium risk |
| 54 | SYSOP2B | LOW | 18 | 11 | Medium dependency impact, Low risk |
| 55 | RUMORS | MEDIUM | 19 | 9 | Medium dependency impact, Medium risk |
| 56 | MAIL6 | LOW | 18 | 12 | Medium dependency impact, Low risk |

## Phase 3 - Advanced Features (28 units)

| Priority | Unit | Risk Level | Dep Impact | Dependencies | Rationale |
|----------|------|------------|------------|--------------|-----------|
| 57 | MAIL9 | LOW | 18 | 13 | Medium dependency impact, Low risk |
| 58 | FILE1 | HIGH | 39 | 17 | High dependency impact, HIGH risk |
| 59 | MAIL4 | LOW | 18 | 16 | Medium dependency impact, Low risk |
| 60 | FILE10 | MEDIUM | 19 | 15 | Medium dependency impact, Medium risk |
| 61 | MAIL5 | MEDIUM | 18 | 15 | Medium dependency impact, Medium risk |
| 62 | MAIL1 | MEDIUM | 21 | 10 | High dependency impact, Medium risk |
| 63 | OVRLAY | LOW | 1 | 2 | Low risk |
| 64 | INPUT | LOW | 0 | 0 | Low risk |
| 65 | ISLC | LOW | 0 | 0 | Low risk |
| 66 | ISLD | LOW | 0 | 0 | Low risk |
| 67 | MAKENUO | LOW | 0 | 0 | Low risk |
| 68 | MAKEWFC | LOW | 0 | 0 | Low risk |
| 69 | TEMP | LOW | 0 | 0 | Low risk |
| 70 | EXAMPLE | LOW | 0 | 1 | Low risk |
| 71 | SYSOP2 | MEDIUM | 18 | 21 | Medium dependency impact, Medium risk |
| 72 | SYSOP2I | MEDIUM | 19 | 9 | Medium dependency impact, Medium risk |
| 73 | FILE6 | MEDIUM | 23 | 15 | High dependency impact, Medium risk |
| 74 | SYSOP2A | MEDIUM | 19 | 10 | Medium dependency impact, Medium risk |
| 75 | SYSOP2J | MEDIUM | 19 | 10 | Medium dependency impact, Medium risk |
| 76 | SYSOP8 | MEDIUM | 18 | 12 | Medium dependency impact, Medium risk |
| 77 | MENUS2 | HIGH | 23 | 10 | High dependency impact, HIGH risk |
| 78 | SYSOP9 | MEDIUM | 18 | 13 | Medium dependency impact, Medium risk |
| 79 | SYSOP11 | MEDIUM | 19 | 11 | Medium dependency impact, Medium risk |
| 80 | SYSOP2D | HIGH | 19 | 8 | Medium dependency impact, HIGH risk |
| 81 | CONF | MEDIUM | 18 | 11 | Medium dependency impact, Medium risk |
| 82 | NUV | MEDIUM | 18 | 11 | Medium dependency impact, Medium risk |
| 83 | DOORS | HIGH | 27 | 11 | High dependency impact, HIGH risk |
| 84 | MNUCONV | LOW | 0 | 1 | Low risk |

## Phase 4 - Integration (30 units)

| Priority | Unit | Risk Level | Dep Impact | Dependencies | Rationale |
|----------|------|------------|------------|--------------|-----------|
| 85 | LZHVIEW | LOW | 0 | 7 | Low risk |
| 86 | MAKECLEA | MEDIUM | 0 | 1 | Medium risk |
| 87 | MISC2 | HIGH | 20 | 12 | Medium dependency impact, HIGH risk |
| 88 | FILE8 | HIGH | 22 | 10 | High dependency impact, HIGH risk |
| 89 | FILE12 | HIGH | 19 | 18 | Medium dependency impact, HIGH risk |
| 90 | LADLE | MEDIUM | 0 | 1 | Medium risk |
| 91 | SYSOP3 | HIGH | 18 | 11 | Medium dependency impact, HIGH risk |
| 92 | TIMESLIC | MEDIUM | 0 | 2 | Medium risk |
| 93 | NEWUSERS | MEDIUM | 4 | 19 | Medium risk |
| 94 | SYSOP2S | HIGH | 19 | 11 | Medium dependency impact, HIGH risk |
| 95 | FILE5 | HIGH | 19 | 17 | Medium dependency impact, HIGH risk |
| 96 | NETFOSSL | HIGH | 0 | 0 | HIGH risk |
| 97 | LOGON2 | MEDIUM | 2 | 23 | Medium risk |
| 98 | ZIPVIEWU | HIGH | 0 | 6 | HIGH risk |
| 99 | TIMETASK | HIGH | 0 | 1 | HIGH risk |
| 100 | SYSOP6 | HIGH | 0 | 4 | HIGH risk |
| 101 | IMPDOS | CRITICAL | 19 | 21 | Medium dependency impact, CRITICAL risk |
| 102 | MAIL7 | CRITICAL | 18 | 15 | Medium dependency impact, CRITICAL risk |
| 103 | MENUS | HIGH | 18 | 62 | Medium dependency impact, HIGH risk |
| 104 | SCRIPT | CRITICAL | 19 | 10 | Medium dependency impact, CRITICAL risk |
| 105 | ANSIEDIT | HIGH | 0 | 11 | HIGH risk |
| 106 | TMPOLD | HIGH | 0 | 1 | HIGH risk |
| 107 | INITP | HIGH | 1 | 16 | HIGH risk |
| 108 | LOGON1 | HIGH | 1 | 26 | HIGH risk |
| 109 | BPTRAP | CRITICAL | 1 | 11 | CRITICAL risk |
| 110 | WFCMENU | HIGH | 2 | 33 | HIGH risk |
| 111 | INSTALL | CRITICAL | 0 | 3 | CRITICAL risk |
| 112 | EXEC | CRITICAL | 0 | 2 | CRITICAL risk |
| 113 | EXECOLD | CRITICAL | 0 | 2 | CRITICAL risk |
| 114 | IMP | CRITICAL | 0 | 69 | CRITICAL risk |

## Conversion Guidelines by Phase

### Phase 1 - Foundation

**Focus:** Core types, common utilities, low-risk foundational modules

**Objectives:**
- Convert RECORDS.PAS (type definitions) first
- Establish core data structures
- Convert common utility modules
- Build test infrastructure
- Create serialization framework for binary files

**Success Criteria:**
- All core types implemented and tested
- Common utilities passing tests
- Binary file I/O working for at least one record type

### Phase 2 - Core Services

**Focus:** File management, user management, basic message system

**Objectives:**
- Implement FILE* modules (file area management)
- Convert basic MAIL* modules (message system)
- Build authentication system
- Establish session management

**Success Criteria:**
- File areas working (upload/download/list)
- Basic message system operational
- User authentication working
- Session state management tested

### Phase 3 - Advanced Features

**Focus:** System operator functions, advanced messaging, protocol handlers

**Objectives:**
- Convert SYSOP* modules (system operator functions)
- Complete message networking
- Implement protocol handlers (Telnet, SSH)
- Add terminal emulation support

**Success Criteria:**
- SysOp menu functional
- Message networking operational
- Telnet/SSH connections working
- ANSI terminal emulation tested

### Phase 4 - Integration

**Focus:** Main program, high-risk modules, final integration

**Objectives:**
- Convert IMP.PAS (main program)
- Handle remaining high-risk modules
- Complete door game interface
- Perform full system integration
- Cross-platform testing

**Success Criteria:**
- Complete BBS operational
- All tests passing
- Verified on Linux, Windows, macOS
- Performance benchmarks met
- Migration tools for existing BBS data

## Special Considerations

### High-Risk Modules Requiring Special Attention

**Count:** 38 units

These modules should be:
- Converted later in the process
- Thoroughly tested with original Pascal version
- Reviewed by multiple developers
- Documented with migration notes

### Parallel Conversion Opportunities

Some modules can be converted in parallel:

1. **FILE* modules** - Each file area module is relatively independent
2. **MAIL* modules** - Message system can be split into subcomponents
3. **SYSOP* modules** - System operator functions are largely independent
4. **MISC* modules** - Miscellaneous utilities are independent

**Parallelization Strategy:**
- Assign different module categories to different developers
- Ensure core types (RECORDS.PAS) completed first
- Regular integration testing to catch interface issues early

## Recommended Sprint Allocation

Based on 32 sprints across 4 phases:

- **Phase 1 - Foundation:** Sprints 3-10 (8 sprints)
  - Sprint 3: Pascal analysis (current)
  - Sprint 4-5: Core types and records
  - Sprint 6-7: Common utilities
  - Sprint 8-9: Binary file I/O and serialization
  - Sprint 10: Phase 1 integration and testing

- **Phase 2 - Core Services:** Sprints 11-18 (8 sprints)
  - Sprints 11-13: File management modules
  - Sprints 14-16: User and message system
  - Sprint 17: Authentication and sessions
  - Sprint 18: Phase 2 integration and testing

- **Phase 3 - Advanced Features:** Sprints 19-26 (8 sprints)
  - Sprints 19-21: System operator functions
  - Sprints 22-23: Protocol handlers
  - Sprint 24-25: Terminal emulation and door games
  - Sprint 26: Phase 3 integration and testing

- **Phase 4 - Integration:** Sprints 27-32 (6 sprints)
  - Sprint 27-28: Main program and high-risk modules
  - Sprint 29: Full system integration
  - Sprint 30: Cross-platform testing
  - Sprint 31: Performance optimization
  - Sprint 32: Documentation and release preparation