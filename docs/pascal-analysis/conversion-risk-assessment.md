# Pascal to Rust Conversion Risk Assessment

**Total Units:** 114
**Analysis Date:** 2025-11-23

## Risk Distribution

| Risk Level | Count | Percentage |
|------------|-------|------------|
| CRITICAL | 11 | 9.6% |
| HIGH | 27 | 23.7% |
| MEDIUM | 30 | 26.3% |
| LOW | 46 | 40.4% |

## Risk Scoring Methodology

**High Risk Factors (10 points each):**
- Interrupt handlers (platform-specific)
- ABSOLUTE variables (memory-mapped I/O)
- Low-level DOS interrupts (SwapVectors, GetInterVec, etc.)

**Medium-High Risk (7 points):**
- Inline assembly
- Process execution (DOS Exec)

**Medium Risk (5 points):**
- Overlay system usage
- Binary file I/O

**Low-Medium Risk (3 points):**
- DOS file system calls
- Heavy pointer usage (>50 occurrences)
- Variant records

**Complexity Factors (1-2 points):**
- Module size (>500 lines)
- Dependency count (>10 dependencies)

**Risk Level Thresholds:**
- CRITICAL: Score >= 20
- HIGH: Score >= 10
- MEDIUM: Score >= 5
- LOW: Score < 5

## CRITICAL Risk Units

| Unit | Risk Score | Risk Factors |
|------|------------|--------------|
| EXEC | 31 | Low-level DOS interrupts; Inline assembly; Process execution (DOS Exec); Heavy pointer usage (52 occurrences); Possible variant records; Medium-sized module (974 lines) |
| EXECOLD | 31 | Low-level DOS interrupts; Inline assembly; Process execution (DOS Exec); Heavy pointer usage (52 occurrences); Possible variant records; Medium-sized module (974 lines) |
| RECORDS | 28 | Low-level DOS interrupts; Inline assembly; Process execution (DOS Exec); Possible variant records; Medium-sized module (830 lines) |
| COMMON | 27 | Inline assembly; Process execution (DOS Exec); Binary file I/O; Heavy pointer usage (107 occurrences); Possible variant records; Large module (1960 lines) |
| COMMON5 | 27 | Low-level DOS interrupts; Inline assembly; Process execution (DOS Exec); Possible variant records |
| INSTALL | 25 | Low-level DOS interrupts; Process execution (DOS Exec); Binary file I/O; DOS file system calls |
| SCRIPT | 24 | Low-level DOS interrupts; Process execution (DOS Exec); DOS file system calls; Possible variant records; Medium-sized module (951 lines) |
| IMP | 22 | Process execution (DOS Exec); Overlay system usage; Binary file I/O; DOS file system calls; High dependency count (69 dependencies) |
| MAIL7 | 22 | ABSOLUTE variables (memory-mapped); Process execution (DOS Exec); Possible variant records; Medium-sized module (517 lines); Medium dependency count (15 dependencies) |
| BPTRAP | 21 | Low-level DOS interrupts; Inline assembly; Possible variant records; Medium dependency count (11 dependencies) |
| IMPDOS | 20 | Process execution (DOS Exec); Binary file I/O; DOS file system calls; Possible variant records; High dependency count (21 dependencies) |

## HIGH Risk Units

| Unit | Risk Score | Risk Factors |
|------|------------|--------------|
| WFCMENU | 19 | Process execution (DOS Exec); DOS file system calls; Heavy pointer usage (55 occurrences); Possible variant records; Medium-sized module (885 lines); High dependency count (33 dependencies) |
| INITP | 18 | Process execution (DOS Exec); DOS file system calls; Heavy pointer usage (57 occurrences); Possible variant records; Medium-sized module (583 lines); Medium dependency count (16 dependencies) |
| TMPCOM | 18 | Interrupt handlers (platform-specific); Inline assembly; Medium-sized module (754 lines) |
| TMPOLD | 18 | Interrupt handlers (platform-specific); Inline assembly; Medium-sized module (754 lines) |
| SYSOP2S | 17 | Inline assembly; Binary file I/O; Possible variant records; Medium-sized module (528 lines); Medium dependency count (11 dependencies) |
| COMMON2 | 16 | Binary file I/O; DOS file system calls; Heavy pointer usage (62 occurrences); Possible variant records; Medium-sized module (956 lines); Medium dependency count (11 dependencies) |
| FILE5 | 16 | Process execution (DOS Exec); Binary file I/O; Possible variant records; Medium dependency count (17 dependencies) |
| LOGON1 | 16 | Process execution (DOS Exec); Heavy pointer usage (91 occurrences); Possible variant records; Medium-sized module (606 lines); High dependency count (26 dependencies) |
| ANSIEDIT | 15 | Inline assembly; Heavy pointer usage (66 occurrences); Possible variant records; Medium-sized module (869 lines); Medium dependency count (11 dependencies) |
| COMMON1 | 15 | Binary file I/O; DOS file system calls; Heavy pointer usage (85 occurrences); Possible variant records; Medium-sized module (882 lines) |
| FILE8 | 15 | Process execution (DOS Exec); Binary file I/O; Possible variant records |
| DOORS | 14 | Process execution (DOS Exec); DOS file system calls; Possible variant records; Medium dependency count (11 dependencies) |
| FILE1 | 14 | Process execution (DOS Exec); Binary file I/O; Medium-sized module (530 lines); Medium dependency count (17 dependencies) |
| SYSOP3 | 14 | Binary file I/O; Heavy pointer usage (59 occurrences); Possible variant records; Large module (1282 lines); Medium dependency count (11 dependencies) |
| FILE12 | 13 | Process execution (DOS Exec); Binary file I/O; Medium dependency count (18 dependencies) |
| MENUS | 13 | Process execution (DOS Exec); Possible variant records; Medium-sized module (678 lines); High dependency count (62 dependencies) |
| MISC2 | 13 | Inline assembly; Binary file I/O; Medium dependency count (12 dependencies) |
| CMD | 12 | Inline assembly; Binary file I/O |
| SYSOP6 | 12 | Process execution (DOS Exec); Binary file I/O |
| TIMETASK | 12 | Inline assembly; Overlay system usage |
| MENUS2 | 11 | Process execution (DOS Exec); Possible variant records; Medium-sized module (534 lines) |
| EXECBAT | 10 | Process execution (DOS Exec); DOS file system calls |
| FILE2 | 10 | Process execution (DOS Exec); DOS file system calls |
| NETFOSSL | 10 | Inline assembly; Possible variant records |
| SYS | 10 | Inline assembly; DOS file system calls |
| SYSOP2D | 10 | Low-level DOS interrupts |
| ZIPVIEWU | 10 | ABSOLUTE variables (memory-mapped) |

## MEDIUM Risk Units

| Unit | Risk Score | Risk Factors |
|------|------------|--------------|
| CONF | 9 | Binary file I/O; Possible variant records; Medium dependency count (11 dependencies) |
| FILE6 | 9 | Process execution (DOS Exec); Medium-sized module (531 lines); Medium dependency count (15 dependencies) |
| NUV | 9 | Binary file I/O; Possible variant records; Medium dependency count (11 dependencies) |
| SYSOP11 | 9 | Binary file I/O; Possible variant records; Medium dependency count (11 dependencies) |
| MAIL1 | 8 | Heavy pointer usage (139 occurrences); Possible variant records; Large module (1275 lines) |
| MULTINOD | 8 | Binary file I/O; Possible variant records |
| SYSOP2A | 8 | Binary file I/O; Possible variant records |
| SYSOP2I | 8 | Binary file I/O; Possible variant records |
| SYSOP2J | 8 | Binary file I/O; Possible variant records |
| SYSOP8 | 8 | DOS file system calls; Possible variant records; Medium-sized module (752 lines); Medium dependency count (12 dependencies) |
| SYSOP9 | 8 | DOS file system calls; Possible variant records; Medium-sized module (546 lines); Medium dependency count (13 dependencies) |
| ANSIDRV | 7 | Inline assembly |
| ASMSAUCE | 7 | Inline assembly |
| FILE4 | 7 | Process execution (DOS Exec) |
| LADLE | 7 | Inline assembly |
| LOGON2 | 7 | Binary file I/O; High dependency count (23 dependencies) |
| MYIO | 7 | Inline assembly |
| STRPROC2 | 7 | Inline assembly |
| TIMESLIC | 7 | Inline assembly |
| FILE0 | 6 | DOS file system calls; Possible variant records |
| MISC5 | 6 | Binary file I/O; Medium dependency count (11 dependencies) |
| NEWUSERS | 6 | Binary file I/O; Medium dependency count (19 dependencies) |
| FILE10 | 5 | Possible variant records; Medium-sized module (560 lines); Medium dependency count (15 dependencies) |
| FILE11 | 5 | Possible variant records; Medium-sized module (700 lines); Medium dependency count (13 dependencies) |
| MAIL5 | 5 | Possible variant records; Medium-sized module (632 lines); Medium dependency count (15 dependencies) |
| MAKECLEA | 5 | Binary file I/O |
| MISC1 | 5 | Binary file I/O |
| MSGPACK | 5 | Binary file I/O |
| RUMORS | 5 | Binary file I/O |
| SYSOP2 | 5 | Possible variant records; High dependency count (21 dependencies) |

## LOW Risk Units (46 units)

Low risk units have straightforward conversion paths with minimal platform-specific code.
These are typically data structures, simple utilities, and standard I/O routines.
