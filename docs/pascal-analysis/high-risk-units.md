# High-Risk Units Detailed Analysis

Units requiring special attention during conversion.

## EXEC.PAS
**Risk Score:** 31 (CRITICAL)

**Risk Factors:**
- Low-level DOS interrupts
- Inline assembly
- Process execution (DOS Exec)
- Heavy pointer usage (52 occurrences)
- Possible variant records
- Medium-sized module (974 lines)

**Mitigation Strategies:**
- Evaluate if assembly is still needed
- Replace with Rust intrinsics where possible
- Use inline assembly syntax: asm!() macro
- Consider pure Rust alternatives
- Replace DOS Exec with std::process::Command
- Use spawn() for async execution
- Handle process I/O with pipes

**Dependencies:**
- 2 dependencies: Dos, checkpat

---

## EXECOLD.PAS
**Risk Score:** 31 (CRITICAL)

**Risk Factors:**
- Low-level DOS interrupts
- Inline assembly
- Process execution (DOS Exec)
- Heavy pointer usage (52 occurrences)
- Possible variant records
- Medium-sized module (974 lines)

**Mitigation Strategies:**
- Evaluate if assembly is still needed
- Replace with Rust intrinsics where possible
- Use inline assembly syntax: asm!() macro
- Consider pure Rust alternatives
- Replace DOS Exec with std::process::Command
- Use spawn() for async execution
- Handle process I/O with pipes

**Dependencies:**
- 2 dependencies: Dos, checkpat

---

## RECORDS.PAS
**Risk Score:** 28 (CRITICAL)

**Risk Factors:**
- Low-level DOS interrupts
- Inline assembly
- Process execution (DOS Exec)
- Possible variant records
- Medium-sized module (830 lines)

**Mitigation Strategies:**
- Evaluate if assembly is still needed
- Replace with Rust intrinsics where possible
- Use inline assembly syntax: asm!() macro
- Consider pure Rust alternatives
- Replace DOS Exec with std::process::Command
- Use spawn() for async execution
- Handle process I/O with pipes

**Dependencies:**
- No dependencies

---

## COMMON.PAS
**Risk Score:** 27 (CRITICAL)

**Risk Factors:**
- Inline assembly
- Process execution (DOS Exec)
- Binary file I/O
- Heavy pointer usage (107 occurrences)
- Possible variant records
- Large module (1960 lines)

**Mitigation Strategies:**
- Evaluate if assembly is still needed
- Replace with Rust intrinsics where possible
- Use inline assembly syntax: asm!() macro
- Consider pure Rust alternatives
- Replace DOS Exec with std::process::Command
- Use spawn() for async execution
- Handle process I/O with pipes
- Use bincode for binary serialization
- Maintain compatibility with original file formats
- Add versioning to file formats
- Provide migration tools for existing data

**Dependencies:**
- 5 dependencies: Crt, Dos, timejunk, records, output

---

## COMMON5.PAS
**Risk Score:** 27 (CRITICAL)

**Risk Factors:**
- Low-level DOS interrupts
- Inline assembly
- Process execution (DOS Exec)
- Possible variant records

**Mitigation Strategies:**
- Evaluate if assembly is still needed
- Replace with Rust intrinsics where possible
- Use inline assembly syntax: asm!() macro
- Consider pure Rust alternatives
- Replace DOS Exec with std::process::Command
- Use spawn() for async execution
- Handle process I/O with pipes

**Dependencies:**
- 1 dependencies: records

---

## INSTALL.PAS
**Risk Score:** 25 (CRITICAL)

**Risk Factors:**
- Low-level DOS interrupts
- Process execution (DOS Exec)
- Binary file I/O
- DOS file system calls

**Mitigation Strategies:**
- Replace DOS Exec with std::process::Command
- Use spawn() for async execution
- Handle process I/O with pipes
- Use bincode for binary serialization
- Maintain compatibility with original file formats
- Add versioning to file formats
- Provide migration tools for existing data

**Dependencies:**
- 3 dependencies: records, crt, dos

---

## SCRIPT.PAS
**Risk Score:** 24 (CRITICAL)

**Risk Factors:**
- Low-level DOS interrupts
- Process execution (DOS Exec)
- DOS file system calls
- Possible variant records
- Medium-sized module (951 lines)

**Mitigation Strategies:**
- Replace DOS Exec with std::process::Command
- Use spawn() for async execution
- Handle process I/O with pipes

**Dependencies:**
- 10 dependencies: Common, common5, Crt, Common1, Common2
  (and 5 more)

---

## IMP.PAS
**Risk Score:** 22 (CRITICAL)

**Risk Factors:**
- Process execution (DOS Exec)
- Overlay system usage
- Binary file I/O
- DOS file system calls
- High dependency count (69 dependencies)

**Mitigation Strategies:**
- Replace DOS Exec with std::process::Command
- Use spawn() for async execution
- Handle process I/O with pipes
- Remove overlay directives (not needed in modern OS)
- Use standard module system
- OS handles memory paging automatically
- Use bincode for binary serialization
- Maintain compatibility with original file formats
- Add versioning to file formats
- Provide migration tools for existing data

**Dependencies:**
- 69 dependencies: Checkpat, Crt, Overlay, OvrLay, BpTrap
  (and 64 more)

---

## MAIL7.PAS
**Risk Score:** 22 (CRITICAL)

**Risk Factors:**
- ABSOLUTE variables (memory-mapped)
- Process execution (DOS Exec)
- Possible variant records
- Medium-sized module (517 lines)
- Medium dependency count (15 dependencies)

**Mitigation Strategies:**
- Replace memory-mapped I/O with OS APIs
- Use memory mapping: mmap on Unix, MapViewOfFile on Windows
- Abstract hardware access behind traits
- Replace DOS Exec with std::process::Command
- Use spawn() for async execution
- Handle process I/O with pipes

**Dependencies:**
- 15 dependencies: Crt, Dos, common1, common3, common5
  (and 10 more)

---

## BPTRAP.PAS
**Risk Score:** 21 (CRITICAL)

**Risk Factors:**
- Low-level DOS interrupts
- Inline assembly
- Possible variant records
- Medium dependency count (11 dependencies)

**Mitigation Strategies:**
- Evaluate if assembly is still needed
- Replace with Rust intrinsics where possible
- Use inline assembly syntax: asm!() macro
- Consider pure Rust alternatives

**Dependencies:**
- 11 dependencies: myio, crt, dos, common5, common
  (and 6 more)

---

## IMPDOS.PAS
**Risk Score:** 20 (CRITICAL)

**Risk Factors:**
- Process execution (DOS Exec)
- Binary file I/O
- DOS file system calls
- Possible variant records
- High dependency count (21 dependencies)

**Mitigation Strategies:**
- Replace DOS Exec with std::process::Command
- Use spawn() for async execution
- Handle process I/O with pipes
- Use bincode for binary serialization
- Maintain compatibility with original file formats
- Add versioning to file formats
- Provide migration tools for existing data

**Dependencies:**
- 21 dependencies: Crt, Dos, common5, records, common
  (and 16 more)

---

## WFCMENU.PAS
**Risk Score:** 19 (HIGH)

**Risk Factors:**
- Process execution (DOS Exec)
- DOS file system calls
- Heavy pointer usage (55 occurrences)
- Possible variant records
- Medium-sized module (885 lines)
- High dependency count (33 dependencies)

**Mitigation Strategies:**
- Replace DOS Exec with std::process::Command
- Use spawn() for async execution
- Handle process I/O with pipes

**Dependencies:**
- 33 dependencies: Crt, Dos, sysop3, sysop7, sysop8
  (and 28 more)

---

## INITP.PAS
**Risk Score:** 18 (HIGH)

**Risk Factors:**
- Process execution (DOS Exec)
- DOS file system calls
- Heavy pointer usage (57 occurrences)
- Possible variant records
- Medium-sized module (583 lines)
- Medium dependency count (16 dependencies)

**Mitigation Strategies:**
- Replace DOS Exec with std::process::Command
- Use spawn() for async execution
- Handle process I/O with pipes

**Dependencies:**
- 16 dependencies: Overlay, Crt, Dos, common4, sysop2
  (and 11 more)

---

## TMPCOM.PAS
**Risk Score:** 18 (HIGH)

**Risk Factors:**
- Interrupt handlers (platform-specific)
- Inline assembly
- Medium-sized module (754 lines)

**Mitigation Strategies:**
- Replace interrupt handlers with OS-specific signal handlers
- Use signal-hook crate for Unix signals
- Use SetConsoleCtrlHandler for Windows
- Evaluate if assembly is still needed
- Replace with Rust intrinsics where possible
- Use inline assembly syntax: asm!() macro
- Consider pure Rust alternatives

**Dependencies:**
- 1 dependencies: Dos

---

## TMPOLD.PAS
**Risk Score:** 18 (HIGH)

**Risk Factors:**
- Interrupt handlers (platform-specific)
- Inline assembly
- Medium-sized module (754 lines)

**Mitigation Strategies:**
- Replace interrupt handlers with OS-specific signal handlers
- Use signal-hook crate for Unix signals
- Use SetConsoleCtrlHandler for Windows
- Evaluate if assembly is still needed
- Replace with Rust intrinsics where possible
- Use inline assembly syntax: asm!() macro
- Consider pure Rust alternatives

**Dependencies:**
- 1 dependencies: Dos

---

## SYSOP2S.PAS
**Risk Score:** 17 (HIGH)

**Risk Factors:**
- Inline assembly
- Binary file I/O
- Possible variant records
- Medium-sized module (528 lines)
- Medium dependency count (11 dependencies)

**Mitigation Strategies:**
- Evaluate if assembly is still needed
- Replace with Rust intrinsics where possible
- Use inline assembly syntax: asm!() macro
- Consider pure Rust alternatives
- Use bincode for binary serialization
- Maintain compatibility with original file formats
- Add versioning to file formats
- Provide migration tools for existing data

**Dependencies:**
- 11 dependencies: cmd, common, common2, common3, common5
  (and 6 more)

---

## COMMON2.PAS
**Risk Score:** 16 (HIGH)

**Risk Factors:**
- Binary file I/O
- DOS file system calls
- Heavy pointer usage (62 occurrences)
- Possible variant records
- Medium-sized module (956 lines)
- Medium dependency count (11 dependencies)

**Mitigation Strategies:**
- Use bincode for binary serialization
- Maintain compatibility with original file formats
- Add versioning to file formats
- Provide migration tools for existing data

**Dependencies:**
- 11 dependencies: Crt, Dos, common5, records, myio
  (and 6 more)

---

## FILE5.PAS
**Risk Score:** 16 (HIGH)

**Risk Factors:**
- Process execution (DOS Exec)
- Binary file I/O
- Possible variant records
- Medium dependency count (17 dependencies)

**Mitigation Strategies:**
- Replace DOS Exec with std::process::Command
- Use spawn() for async execution
- Handle process I/O with pipes
- Use bincode for binary serialization
- Maintain compatibility with original file formats
- Add versioning to file formats
- Provide migration tools for existing data

**Dependencies:**
- 17 dependencies: Crt, Dos, common5, records, common
  (and 12 more)

---

## LOGON1.PAS
**Risk Score:** 16 (HIGH)

**Risk Factors:**
- Process execution (DOS Exec)
- Heavy pointer usage (91 occurrences)
- Possible variant records
- Medium-sized module (606 lines)
- High dependency count (26 dependencies)

**Mitigation Strategies:**
- Replace DOS Exec with std::process::Command
- Use spawn() for async execution
- Handle process I/O with pipes

**Dependencies:**
- 26 dependencies: Crt, Dos, common5, logon2, newusers
  (and 21 more)

---

## ANSIEDIT.PAS
**Risk Score:** 15 (HIGH)

**Risk Factors:**
- Inline assembly
- Heavy pointer usage (66 occurrences)
- Possible variant records
- Medium-sized module (869 lines)
- Medium dependency count (11 dependencies)

**Mitigation Strategies:**
- Evaluate if assembly is still needed
- Replace with Rust intrinsics where possible
- Use inline assembly syntax: asm!() macro
- Consider pure Rust alternatives

**Dependencies:**
- 11 dependencies: Crt, Dos, common, common1, common2
  (and 6 more)

---

## COMMON1.PAS
**Risk Score:** 15 (HIGH)

**Risk Factors:**
- Binary file I/O
- DOS file system calls
- Heavy pointer usage (85 occurrences)
- Possible variant records
- Medium-sized module (882 lines)

**Mitigation Strategies:**
- Use bincode for binary serialization
- Maintain compatibility with original file formats
- Add versioning to file formats
- Provide migration tools for existing data

**Dependencies:**
- 5 dependencies: Crt, Dos, records, myio, tmpcom

---

## FILE8.PAS
**Risk Score:** 15 (HIGH)

**Risk Factors:**
- Process execution (DOS Exec)
- Binary file I/O
- Possible variant records

**Mitigation Strategies:**
- Replace DOS Exec with std::process::Command
- Use spawn() for async execution
- Handle process I/O with pipes
- Use bincode for binary serialization
- Maintain compatibility with original file formats
- Add versioning to file formats
- Provide migration tools for existing data

**Dependencies:**
- 10 dependencies: Crt, Dos, records, myio, common4
  (and 5 more)

---

## DOORS.PAS
**Risk Score:** 14 (HIGH)

**Risk Factors:**
- Process execution (DOS Exec)
- DOS file system calls
- Possible variant records
- Medium dependency count (11 dependencies)

**Mitigation Strategies:**
- Replace DOS Exec with std::process::Command
- Use spawn() for async execution
- Handle process I/O with pipes

**Dependencies:**
- 11 dependencies: Crt, Dos, common5, records, execbat
  (and 6 more)

---

## FILE1.PAS
**Risk Score:** 14 (HIGH)

**Risk Factors:**
- Process execution (DOS Exec)
- Binary file I/O
- Medium-sized module (530 lines)
- Medium dependency count (17 dependencies)

**Mitigation Strategies:**
- Replace DOS Exec with std::process::Command
- Use spawn() for async execution
- Handle process I/O with pipes
- Use bincode for binary serialization
- Maintain compatibility with original file formats
- Add versioning to file formats
- Provide migration tools for existing data

**Dependencies:**
- 17 dependencies: Crt, Dos, ulcheck, file3, execbat
  (and 12 more)

---

## SYSOP3.PAS
**Risk Score:** 14 (HIGH)

**Risk Factors:**
- Binary file I/O
- Heavy pointer usage (59 occurrences)
- Possible variant records
- Large module (1282 lines)
- Medium dependency count (11 dependencies)

**Mitigation Strategies:**
- Use bincode for binary serialization
- Maintain compatibility with original file formats
- Add versioning to file formats
- Provide migration tools for existing data

**Dependencies:**
- 11 dependencies: Crt, Dos, myio, common1, common2
  (and 6 more)

---

## FILE12.PAS
**Risk Score:** 13 (HIGH)

**Risk Factors:**
- Process execution (DOS Exec)
- Binary file I/O
- Medium dependency count (18 dependencies)

**Mitigation Strategies:**
- Replace DOS Exec with std::process::Command
- Use spawn() for async execution
- Handle process I/O with pipes
- Use bincode for binary serialization
- Maintain compatibility with original file formats
- Add versioning to file formats
- Provide migration tools for existing data

**Dependencies:**
- 18 dependencies: Crt, Dos, records, file0, file8
  (and 13 more)

---

## MENUS.PAS
**Risk Score:** 13 (HIGH)

**Risk Factors:**
- Process execution (DOS Exec)
- Possible variant records
- Medium-sized module (678 lines)
- High dependency count (62 dependencies)

**Mitigation Strategies:**
- Replace DOS Exec with std::process::Command
- Use spawn() for async execution
- Handle process I/O with pipes

**Dependencies:**
- 62 dependencies: Crt, Dos, Sysop3, Script, Rumors
  (and 57 more)

---

## MISC2.PAS
**Risk Score:** 13 (HIGH)

**Risk Factors:**
- Inline assembly
- Binary file I/O
- Medium dependency count (12 dependencies)

**Mitigation Strategies:**
- Evaluate if assembly is still needed
- Replace with Rust intrinsics where possible
- Use inline assembly syntax: asm!() macro
- Consider pure Rust alternatives
- Use bincode for binary serialization
- Maintain compatibility with original file formats
- Add versioning to file formats
- Provide migration tools for existing data

**Dependencies:**
- 12 dependencies: Crt, Dos, common1, common2, common3
  (and 7 more)

---

## CMD.PAS
**Risk Score:** 12 (HIGH)

**Risk Factors:**
- Inline assembly
- Binary file I/O

**Mitigation Strategies:**
- Evaluate if assembly is still needed
- Replace with Rust intrinsics where possible
- Use inline assembly syntax: asm!() macro
- Consider pure Rust alternatives
- Use bincode for binary serialization
- Maintain compatibility with original file formats
- Add versioning to file formats
- Provide migration tools for existing data

**Dependencies:**
- 6 dependencies: common, common1, common2, common3, records
  (and 1 more)

---

## SYSOP6.PAS
**Risk Score:** 12 (HIGH)

**Risk Factors:**
- Process execution (DOS Exec)
- Binary file I/O

**Mitigation Strategies:**
- Replace DOS Exec with std::process::Command
- Use spawn() for async execution
- Handle process I/O with pipes
- Use bincode for binary serialization
- Maintain compatibility with original file formats
- Add versioning to file formats
- Provide migration tools for existing data

**Dependencies:**
- 4 dependencies: Crt, Dos, overlay, common

---

## TIMETASK.PAS
**Risk Score:** 12 (HIGH)

**Risk Factors:**
- Inline assembly
- Overlay system usage

**Mitigation Strategies:**
- Evaluate if assembly is still needed
- Replace with Rust intrinsics where possible
- Use inline assembly syntax: asm!() macro
- Consider pure Rust alternatives
- Remove overlay directives (not needed in modern OS)
- Use standard module system
- OS handles memory paging automatically

**Dependencies:**
- 1 dependencies: dos

---

## MENUS2.PAS
**Risk Score:** 11 (HIGH)

**Risk Factors:**
- Process execution (DOS Exec)
- Possible variant records
- Medium-sized module (534 lines)

**Mitigation Strategies:**
- Replace DOS Exec with std::process::Command
- Use spawn() for async execution
- Handle process I/O with pipes

**Dependencies:**
- 10 dependencies: Crt, Dos, common5, records, file4
  (and 5 more)

---

## EXECBAT.PAS
**Risk Score:** 10 (HIGH)

**Risk Factors:**
- Process execution (DOS Exec)
- DOS file system calls

**Mitigation Strategies:**
- Replace DOS Exec with std::process::Command
- Use spawn() for async execution
- Handle process I/O with pipes

**Dependencies:**
- 9 dependencies: Crt, Dos, common5, records, common
  (and 4 more)

---

## FILE2.PAS
**Risk Score:** 10 (HIGH)

**Risk Factors:**
- Process execution (DOS Exec)
- DOS file system calls

**Mitigation Strategies:**
- Replace DOS Exec with std::process::Command
- Use spawn() for async execution
- Handle process I/O with pipes

**Dependencies:**
- 8 dependencies: Crt, Dos, records, execbat, file0
  (and 3 more)

---

## NETFOSSL.PAS
**Risk Score:** 10 (HIGH)

**Risk Factors:**
- Inline assembly
- Possible variant records

**Mitigation Strategies:**
- Evaluate if assembly is still needed
- Replace with Rust intrinsics where possible
- Use inline assembly syntax: asm!() macro
- Consider pure Rust alternatives

**Dependencies:**
- No dependencies

---

## SYS.PAS
**Risk Score:** 10 (HIGH)

**Risk Factors:**
- Inline assembly
- DOS file system calls

**Mitigation Strategies:**
- Evaluate if assembly is still needed
- Replace with Rust intrinsics where possible
- Use inline assembly syntax: asm!() macro
- Consider pure Rust alternatives

**Dependencies:**
- 2 dependencies: common, records

---

## SYSOP2D.PAS
**Risk Score:** 10 (HIGH)

**Risk Factors:**
- Low-level DOS interrupts

**Mitigation Strategies:**

**Dependencies:**
- 8 dependencies: Crt, Dos, common1, common2, common3
  (and 3 more)

---

## ZIPVIEWU.PAS
**Risk Score:** 10 (HIGH)

**Risk Factors:**
- ABSOLUTE variables (memory-mapped)

**Mitigation Strategies:**
- Replace memory-mapped I/O with OS APIs
- Use memory mapping: mmap on Unix, MapViewOfFile on Windows
- Abstract hardware access behind traits

**Dependencies:**
- 6 dependencies: Dos, Crt, Common, Common5, common2
  (and 1 more)

---
