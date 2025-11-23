# Pascal Unit-Level Analysis

Detailed analysis of each Pascal unit in the Impulse BBS codebase.

**Total Units:** 114

## Main Program (1 units)

### IMP.PAS

**Unit Name:** Impulse (Program)
**Risk Level:** CRITICAL
**Risk Score:** 22

**Risk Factors:**
- Process execution (DOS Exec)
- Overlay system usage
- Binary file I/O
- DOS file system calls
- High dependency count (69 dependencies)

**Dependencies (69):**
- AnsiDrv
- BpTrap
- CUser
- Checkpat
- Common
- Common1
- Common2
- Common3
- Common4
- Common5
- Crt
- Doors
- Dos
- File0
- File1
- File10
- File11
- File12
- File13
- File14
- File2
- File4
- File5
- File6
- File8
- File9
- InitP
- Logon1
- Logon2
- MModem
- Mail0
- Mail1
- Mail2
- Mail3
- Mail4
- Mail5
- Mail6
- Mail9
- Menus
- Menus2
- Menus3
- Misc1
- Misc2
- Misc3
- Misc4
- Misc5
- MsgPack
- MyIO
- NewUsers
- Overlay
- OvrLay
- Records
- Rumors
- ScrlBk
- Sysop11
- Sysop3
- Sysop7
- Sysop8
- Sysop9
- TimeJunk
- Tmpcom
- Ulcheck
- WfcMenu
- cmd
- multinod
- outPut
- strProc
- strProc2
- sys

**Purpose:** Main program entry point, initializes BBS and manages user sessions

---

## Core Types (1 units)

### RECORDS.PAS

**Unit Name:** Records
**Risk Level:** CRITICAL
**Risk Score:** 28

**Risk Factors:**
- Low-level DOS interrupts
- Inline assembly
- Process execution (DOS Exec)
- Possible variant records
- Medium-sized module (830 lines)

**No dependencies**

**Purpose:** Core type definitions and data structures used throughout BBS

---

## Common Utilities (6 units)

### COMMON.PAS

**Unit Name:** common
**Risk Level:** CRITICAL
**Risk Score:** 27

**Risk Factors:**
- Inline assembly
- Process execution (DOS Exec)
- Binary file I/O
- Heavy pointer usage (107 occurrences)
- Possible variant records
- Large module (1960 lines)

**Dependencies (5):**
- Crt
- Dos
- output
- records
- timejunk

**Purpose:** Common utility functions shared across modules

---

### COMMON1.PAS

**Unit Name:** common1
**Risk Level:** HIGH
**Risk Score:** 15

**Risk Factors:**
- Binary file I/O
- DOS file system calls
- Heavy pointer usage (85 occurrences)
- Possible variant records
- Medium-sized module (882 lines)

**Dependencies (5):**
- Crt
- Dos
- myio
- records
- tmpcom

**Purpose:** Common utility functions shared across modules

---

### COMMON2.PAS

**Unit Name:** common2
**Risk Level:** HIGH
**Risk Score:** 16

**Risk Factors:**
- Binary file I/O
- DOS file system calls
- Heavy pointer usage (62 occurrences)
- Possible variant records
- Medium-sized module (956 lines)
- Medium dependency count (11 dependencies)

**Dependencies (11):**
- Crt
- Dos
- cmd
- common
- common4
- common5
- myio
- records
- scrlbk
- strProc
- tmpcom

**Purpose:** Common utility functions shared across modules

---

### COMMON3.PAS

**Unit Name:** common3
**Risk Level:** LOW
**Risk Score:** 3

**Risk Factors:**
- Possible variant records

**Dependencies (7):**
- Crt
- Dos
- common5
- myio
- records
- strProc
- tmpcom

**Purpose:** Common utility functions shared across modules

---

### COMMON4.PAS

**Unit Name:** Common4
**Risk Level:** LOW
**Risk Score:** 3

**Risk Factors:**
- Possible variant records

**Dependencies (5):**
- Common
- Common5
- Crt
- Dos
- Records

**Purpose:** Common utility functions shared across modules

---

### COMMON5.PAS

**Unit Name:** Common5
**Risk Level:** CRITICAL
**Risk Score:** 27

**Risk Factors:**
- Low-level DOS interrupts
- Inline assembly
- Process execution (DOS Exec)
- Possible variant records

**Dependencies (1):**
- records

**Purpose:** Common utility functions shared across modules

---

## File Management (14 units)

### FILE0.PAS

**Unit Name:** file0
**Risk Level:** MEDIUM
**Risk Score:** 6

**Risk Factors:**
- DOS file system calls
- Possible variant records

**Dependencies (10):**
- Crt
- Dos
- common
- common1
- common2
- common3
- common5
- myio
- records
- strproc

**Purpose:** File area management (upload, download, listing)

---

### FILE1.PAS

**Unit Name:** file1
**Risk Level:** HIGH
**Risk Score:** 14

**Risk Factors:**
- Process execution (DOS Exec)
- Binary file I/O
- Medium-sized module (530 lines)
- Medium dependency count (17 dependencies)

**Dependencies (17):**
- Crt
- Dos
- cmd
- common
- common1
- common2
- common3
- common4
- common5
- execbat
- file3
- multinod
- myio
- records
- strproc
- sys
- ulcheck

**Purpose:** File area management (upload, download, listing)

---

### FILE10.PAS

**Unit Name:** file10
**Risk Level:** MEDIUM
**Risk Score:** 5

**Risk Factors:**
- Possible variant records
- Medium-sized module (560 lines)
- Medium dependency count (15 dependencies)

**Dependencies (15):**
- Crt
- Dos
- common
- common1
- common2
- common3
- common5
- file0
- file1
- file2
- file4
- file9
- myio
- records
- strproc

**Purpose:** File area management (upload, download, listing)

---

### FILE11.PAS

**Unit Name:** file11
**Risk Level:** MEDIUM
**Risk Score:** 5

**Risk Factors:**
- Possible variant records
- Medium-sized module (700 lines)
- Medium dependency count (13 dependencies)

**Dependencies (13):**
- Crt
- Dos
- common
- common1
- common2
- common3
- common4
- common5
- file0
- file1
- myio
- records
- strproc

**Purpose:** File area management (upload, download, listing)

---

### FILE12.PAS

**Unit Name:** file12
**Risk Level:** HIGH
**Risk Score:** 13

**Risk Factors:**
- Process execution (DOS Exec)
- Binary file I/O
- Medium dependency count (18 dependencies)

**Dependencies (18):**
- Crt
- Dos
- common
- common1
- common2
- common3
- common5
- execbat
- file0
- file1
- file2
- file3
- file4
- file6
- file8
- file9
- mmodem
- records

**Purpose:** File area management (upload, download, listing)

---

### FILE13.PAS

**Unit Name:** file13
**Risk Level:** LOW
**Risk Score:** 3

**Risk Factors:**
- Possible variant records

**Dependencies (9):**
- Crt
- Dos
- common
- common5
- file0
- file1
- file2
- myio
- records

**Purpose:** File area management (upload, download, listing)

---

### FILE14.PAS

**Unit Name:** file14
**Risk Level:** LOW
**Risk Score:** 0

**Dependencies (8):**
- Crt
- Dos
- common
- common5
- file0
- file11
- myio
- records

**Purpose:** File area management (upload, download, listing)

---

### FILE2.PAS

**Unit Name:** file2
**Risk Level:** HIGH
**Risk Score:** 10

**Risk Factors:**
- Process execution (DOS Exec)
- DOS file system calls

**Dependencies (8):**
- Crt
- Dos
- common
- common5
- execbat
- file0
- file4
- records

**Purpose:** File area management (upload, download, listing)

---

### FILE3.PAS

**Unit Name:** File3
**Risk Level:** LOW
**Risk Score:** 0

**Dependencies (2):**
- Common
- records

**Purpose:** File area management (upload, download, listing)

---

### FILE4.PAS

**Unit Name:** file4
**Risk Level:** MEDIUM
**Risk Score:** 7

**Risk Factors:**
- Process execution (DOS Exec)

**Dependencies (7):**
- Crt
- Dos
- common
- common5
- file0
- file14
- records

**Purpose:** File area management (upload, download, listing)

---

### FILE5.PAS

**Unit Name:** file5
**Risk Level:** HIGH
**Risk Score:** 16

**Risk Factors:**
- Process execution (DOS Exec)
- Binary file I/O
- Possible variant records
- Medium dependency count (17 dependencies)

**Dependencies (17):**
- Crt
- Dos
- common
- common1
- common2
- common3
- common5
- execbat
- file0
- file1
- file11
- file2
- file3
- file4
- file8
- file9
- records

**Purpose:** File area management (upload, download, listing)

---

### FILE6.PAS

**Unit Name:** file6
**Risk Level:** MEDIUM
**Risk Score:** 9

**Risk Factors:**
- Process execution (DOS Exec)
- Medium-sized module (531 lines)
- Medium dependency count (15 dependencies)

**Dependencies (15):**
- Crt
- Dos
- common
- common1
- common2
- common3
- common5
- execbat
- file0
- file1
- file2
- file4
- file9
- records
- strProc

**Purpose:** File area management (upload, download, listing)

---

### FILE8.PAS

**Unit Name:** file8
**Risk Level:** HIGH
**Risk Score:** 15

**Risk Factors:**
- Process execution (DOS Exec)
- Binary file I/O
- Possible variant records

**Dependencies (10):**
- Crt
- Dos
- common
- common4
- common5
- execbat
- file0
- file6
- myio
- records

**Purpose:** File area management (upload, download, listing)

---

### FILE9.PAS

**Unit Name:** file9
**Risk Level:** LOW
**Risk Score:** 4

**Risk Factors:**
- Possible variant records
- Medium dependency count (13 dependencies)

**Dependencies (13):**
- Crt
- Dos
- common
- common1
- common2
- common3
- common5
- file0
- file1
- file2
- myio
- records
- strproc

**Purpose:** File area management (upload, download, listing)

---

## Mail/Message System (9 units)

### MAIL0.PAS

**Unit Name:** mail0
**Risk Level:** LOW
**Risk Score:** 3

**Risk Factors:**
- Possible variant records

**Dependencies (7):**
- Crt
- Dos
- common
- common5
- records
- strproc
- timejunk

**Purpose:** Message base and email system functionality

---

### MAIL1.PAS

**Unit Name:** mail1
**Risk Level:** MEDIUM
**Risk Score:** 8

**Risk Factors:**
- Heavy pointer usage (139 occurrences)
- Possible variant records
- Large module (1275 lines)

**Dependencies (10):**
- Crt
- Dos
- common
- common1
- common2
- common3
- common5
- records
- strproc
- timejunk

**Purpose:** Message base and email system functionality

---

### MAIL2.PAS

**Unit Name:** mail2
**Risk Level:** LOW
**Risk Score:** 4

**Risk Factors:**
- Possible variant records
- Medium dependency count (12 dependencies)

**Dependencies (12):**
- Crt
- Dos
- common
- common1
- common2
- common3
- common5
- mail0
- mail1
- records
- strproc
- timejunk

**Purpose:** Message base and email system functionality

---

### MAIL3.PAS

**Unit Name:** mail3
**Risk Level:** LOW
**Risk Score:** 4

**Risk Factors:**
- Possible variant records
- Medium dependency count (13 dependencies)

**Dependencies (13):**
- Crt
- Dos
- common
- common1
- common2
- common3
- common5
- mail0
- records
- scrlbk
- strproc
- sys
- timejunk

**Purpose:** Message base and email system functionality

---

### MAIL4.PAS

**Unit Name:** mail4
**Risk Level:** LOW
**Risk Score:** 4

**Risk Factors:**
- Possible variant records
- Medium dependency count (16 dependencies)

**Dependencies (16):**
- Crt
- Dos
- common
- common1
- common2
- common3
- common5
- mail0
- mail1
- mail2
- mail3
- misc3
- misc5
- records
- sysop3
- timejunk

**Purpose:** Message base and email system functionality

---

### MAIL5.PAS

**Unit Name:** mail5
**Risk Level:** MEDIUM
**Risk Score:** 5

**Risk Factors:**
- Possible variant records
- Medium-sized module (632 lines)
- Medium dependency count (15 dependencies)

**Dependencies (15):**
- Crt
- Dos
- common
- common1
- common2
- common3
- common5
- mail0
- mail1
- mail2
- mail3
- mail4
- mail6
- records
- timejunk

**Purpose:** Message base and email system functionality

---

### MAIL6.PAS

**Unit Name:** mail6
**Risk Level:** LOW
**Risk Score:** 4

**Risk Factors:**
- Possible variant records
- Medium dependency count (12 dependencies)

**Dependencies (12):**
- Crt
- Dos
- common
- common1
- common2
- common3
- common5
- mail0
- mail3
- mail9
- msgpack
- records

**Purpose:** Message base and email system functionality

---

### MAIL7.PAS

**Unit Name:** mail7
**Risk Level:** CRITICAL
**Risk Score:** 22

**Risk Factors:**
- ABSOLUTE variables (memory-mapped)
- Process execution (DOS Exec)
- Possible variant records
- Medium-sized module (517 lines)
- Medium dependency count (15 dependencies)

**Dependencies (15):**
- Crt
- Dos
- common
- common1
- common2
- common3
- common5
- execbat
- file2
- file8
- mail0
- mail5
- records
- strproc
- timejunk

**Purpose:** Message base and email system functionality

---

### MAIL9.PAS

**Unit Name:** mail9
**Risk Level:** LOW
**Risk Score:** 4

**Risk Factors:**
- Possible variant records
- Medium dependency count (13 dependencies)

**Dependencies (13):**
- Crt
- Dos
- common
- common1
- common2
- common3
- common4
- common5
- mail0
- mail4
- records
- sys
- timejunk

**Purpose:** Message base and email system functionality

---

## System Operator (22 units)

### SYSOP1.PAS

**Unit Name:** sysop1
**Risk Level:** LOW
**Risk Score:** 4

**Risk Factors:**
- Possible variant records
- Medium dependency count (11 dependencies)

**Dependencies (11):**
- Crt
- Dos
- common
- common1
- common2
- common3
- common4
- common5
- file1
- menus2
- records

**Purpose:** System operator functions and administration

---

### SYSOP11.PAS

**Unit Name:** sysop11
**Risk Level:** MEDIUM
**Risk Score:** 9

**Risk Factors:**
- Binary file I/O
- Possible variant records
- Medium dependency count (11 dependencies)

**Dependencies (11):**
- Crt
- Dos
- common
- common1
- common2
- common3
- common5
- menus2
- misc1
- misc5
- records

**Purpose:** System operator functions and administration

---

### SYSOP2.PAS

**Unit Name:** sysop2
**Risk Level:** MEDIUM
**Risk Score:** 5

**Risk Factors:**
- Possible variant records
- High dependency count (21 dependencies)

**Dependencies (21):**
- Crt
- Dos
- common
- common1
- common2
- common3
- common5
- records
- sysop1
- sysop21
- sysop2a
- sysop2b
- sysop2c
- sysop2d
- sysop2e
- sysop2f
- sysop2g
- sysop2h
- sysop2i
- sysop2s
- sysop2z

**Purpose:** System operator functions and administration

---

### SYSOP21.PAS

**Unit Name:** sysop21
**Risk Level:** LOW
**Risk Score:** 3

**Risk Factors:**
- Possible variant records

**Dependencies (8):**
- Crt
- Dos
- common
- common1
- common2
- common3
- common5
- records

**Purpose:** System operator functions and administration

---

### SYSOP2A.PAS

**Unit Name:** sysop2a
**Risk Level:** MEDIUM
**Risk Score:** 8

**Risk Factors:**
- Binary file I/O
- Possible variant records

**Dependencies (10):**
- Crt
- Dos
- common
- common1
- common2
- common3
- common4
- common5
- cuser
- records

**Purpose:** System operator functions and administration

---

### SYSOP2B.PAS

**Unit Name:** sysop2b
**Risk Level:** LOW
**Risk Score:** 4

**Risk Factors:**
- Possible variant records
- Medium dependency count (11 dependencies)

**Dependencies (11):**
- Crt
- Dos
- common
- common1
- common2
- common3
- common4
- common5
- records
- sysop2j
- sysop3

**Purpose:** System operator functions and administration

---

### SYSOP2C.PAS

**Unit Name:** sysop2c
**Risk Level:** LOW
**Risk Score:** 3

**Risk Factors:**
- Possible variant records

**Dependencies (9):**
- Crt
- Dos
- common
- common1
- common2
- common3
- common4
- common5
- records

**Purpose:** System operator functions and administration

---

### SYSOP2D.PAS

**Unit Name:** sysop2d
**Risk Level:** HIGH
**Risk Score:** 10

**Risk Factors:**
- Low-level DOS interrupts

**Dependencies (8):**
- Crt
- Dos
- common
- common1
- common2
- common3
- common4
- common5

**Purpose:** System operator functions and administration

---

### SYSOP2E.PAS

**Unit Name:** sysop2e
**Risk Level:** LOW
**Risk Score:** 0

**Dependencies (8):**
- Crt
- Dos
- common
- common1
- common2
- common3
- common4
- common5

**Purpose:** System operator functions and administration

---

### SYSOP2F.PAS

**Unit Name:** sysop2f
**Risk Level:** LOW
**Risk Score:** 0

**Dependencies (8):**
- Crt
- Dos
- common
- common1
- common2
- common3
- common4
- common5

**Purpose:** System operator functions and administration

---

### SYSOP2G.PAS

**Unit Name:** sysop2g
**Risk Level:** LOW
**Risk Score:** 3

**Risk Factors:**
- Possible variant records

**Dependencies (10):**
- Crt
- Dos
- common
- common1
- common2
- common3
- common4
- common5
- records
- sysop3

**Purpose:** System operator functions and administration

---

### SYSOP2H.PAS

**Unit Name:** sysop2h
**Risk Level:** LOW
**Risk Score:** 3

**Risk Factors:**
- Possible variant records

**Dependencies (10):**
- Crt
- Dos
- cmd
- common
- common1
- common2
- common3
- common4
- common5
- records

**Purpose:** System operator functions and administration

---

### SYSOP2I.PAS

**Unit Name:** sysop2i
**Risk Level:** MEDIUM
**Risk Score:** 8

**Risk Factors:**
- Binary file I/O
- Possible variant records

**Dependencies (9):**
- Crt
- Dos
- common
- common1
- common2
- common3
- common4
- common5
- records

**Purpose:** System operator functions and administration

---

### SYSOP2J.PAS

**Unit Name:** sysop2j
**Risk Level:** MEDIUM
**Risk Score:** 8

**Risk Factors:**
- Binary file I/O
- Possible variant records

**Dependencies (10):**
- Crt
- Dos
- common
- common1
- common2
- common3
- common4
- common5
- multinod
- records

**Purpose:** System operator functions and administration

---

### SYSOP2S.PAS

**Unit Name:** sysop2s
**Risk Level:** HIGH
**Risk Score:** 17

**Risk Factors:**
- Inline assembly
- Binary file I/O
- Possible variant records
- Medium-sized module (528 lines)
- Medium dependency count (11 dependencies)

**Dependencies (11):**
- cmd
- common
- common2
- common3
- common5
- crt
- dos
- output
- records
- strProc
- strProc2

**Purpose:** System operator functions and administration

---

### SYSOP2Z.PAS

**Unit Name:** sysop2z
**Risk Level:** LOW
**Risk Score:** 0

**Dependencies (7):**
- Crt
- Dos
- common
- common4
- common5
- cuser
- records

**Purpose:** System operator functions and administration

---

### SYSOP3.PAS

**Unit Name:** sysop3
**Risk Level:** HIGH
**Risk Score:** 14

**Risk Factors:**
- Binary file I/O
- Heavy pointer usage (59 occurrences)
- Possible variant records
- Large module (1282 lines)
- Medium dependency count (11 dependencies)

**Dependencies (11):**
- Crt
- Dos
- common
- common1
- common2
- common3
- common5
- myio
- nuv
- records
- strproc

**Purpose:** System operator functions and administration

---

### SYSOP6.PAS

**Unit Name:** sysop6
**Risk Level:** HIGH
**Risk Score:** 12

**Risk Factors:**
- Process execution (DOS Exec)
- Binary file I/O

**Dependencies (4):**
- Crt
- Dos
- common
- overlay

**Purpose:** System operator functions and administration

---

### SYSOP7.PAS

**Unit Name:** sysop7
**Risk Level:** LOW
**Risk Score:** 4

**Risk Factors:**
- Possible variant records
- Medium dependency count (13 dependencies)

**Dependencies (13):**
- Crt
- Dos
- common
- common1
- common2
- common3
- common5
- file2
- file9
- menus2
- records
- strproc
- sysop7m

**Purpose:** System operator functions and administration

---

### SYSOP7M.PAS

**Unit Name:** sysop7m
**Risk Level:** LOW
**Risk Score:** 3

**Risk Factors:**
- Possible variant records

**Dependencies (8):**
- Crt
- Dos
- common
- common3
- common5
- file9
- menus2
- records

**Purpose:** System operator functions and administration

---

### SYSOP8.PAS

**Unit Name:** sysop8
**Risk Level:** MEDIUM
**Risk Score:** 8

**Risk Factors:**
- DOS file system calls
- Possible variant records
- Medium-sized module (752 lines)
- Medium dependency count (12 dependencies)

**Dependencies (12):**
- Conf
- Crt
- Dos
- common
- common1
- common2
- common3
- common4
- common5
- file0
- mail0
- records

**Purpose:** System operator functions and administration

---

### SYSOP9.PAS

**Unit Name:** sysop9
**Risk Level:** MEDIUM
**Risk Score:** 8

**Risk Factors:**
- DOS file system calls
- Possible variant records
- Medium-sized module (546 lines)
- Medium dependency count (13 dependencies)

**Dependencies (13):**
- Conf
- Crt
- Dos
- common
- common1
- common2
- common3
- common4
- common5
- file0
- file2
- records
- sysop8

**Purpose:** System operator functions and administration

---

## Miscellaneous (5 units)

### MISC1.PAS

**Unit Name:** misc1
**Risk Level:** MEDIUM
**Risk Score:** 5

**Risk Factors:**
- Binary file I/O

**Dependencies (9):**
- Crt
- Dos
- ansidrv
- common
- common1
- common2
- common3
- common5
- records


---

### MISC2.PAS

**Unit Name:** misc2
**Risk Level:** HIGH
**Risk Score:** 13

**Risk Factors:**
- Inline assembly
- Binary file I/O
- Medium dependency count (12 dependencies)

**Dependencies (12):**
- Crt
- Dos
- ansidrv
- common
- common1
- common2
- common3
- common5
- file2
- misc1
- records
- sys


---

### MISC3.PAS

**Unit Name:** misc3
**Risk Level:** LOW
**Risk Score:** 3

**Risk Factors:**
- Possible variant records

**Dependencies (8):**
- Crt
- Dos
- common
- common1
- common2
- common3
- common5
- records


---

### MISC4.PAS

**Unit Name:** misc4
**Risk Level:** LOW
**Risk Score:** 1

**Risk Factors:**
- Medium dependency count (11 dependencies)

**Dependencies (11):**
- Crt
- Dos
- common
- common1
- common2
- common3
- common5
- doors
- misc3
- records
- strproc


---

### MISC5.PAS

**Unit Name:** misc5
**Risk Level:** MEDIUM
**Risk Score:** 6

**Risk Factors:**
- Binary file I/O
- Medium dependency count (11 dependencies)

**Dependencies (11):**
- Crt
- Dos
- common
- common1
- common2
- common3
- common5
- doors
- misc1
- records
- strProc


---

## Other (56 units)

### ANSIDRV.PAS

**Unit Name:** AnsiDrv
**Risk Level:** MEDIUM
**Risk Score:** 7

**Risk Factors:**
- Inline assembly

**Dependencies (2):**
- Crt
- common


---

### ANSIEDIT.PAS

**Unit Name:** ansiedit
**Risk Level:** HIGH
**Risk Score:** 15

**Risk Factors:**
- Inline assembly
- Heavy pointer usage (66 occurrences)
- Possible variant records
- Medium-sized module (869 lines)
- Medium dependency count (11 dependencies)

**Dependencies (11):**
- Crt
- Dos
- ansidrv
- common
- common1
- common2
- common3
- common4
- common5
- records
- sys


---

### ASMSAUCE.PAS

**Unit Name:** ASMSAUCE
**Risk Level:** MEDIUM
**Risk Score:** 7

**Risk Factors:**
- Inline assembly

**No dependencies**


---

### BPTRAP.PAS

**Unit Name:** BPTrap
**Risk Level:** CRITICAL
**Risk Score:** 21

**Risk Factors:**
- Low-level DOS interrupts
- Inline assembly
- Possible variant records
- Medium dependency count (11 dependencies)

**Dependencies (11):**
- ansidrv
- common
- common2
- common4
- common5
- crt
- dos
- misc1
- myio
- records
- sys


---

### CMD.PAS

**Unit Name:** cmd
**Risk Level:** HIGH
**Risk Score:** 12

**Risk Factors:**
- Inline assembly
- Binary file I/O

**Dependencies (6):**
- asmSauce
- common
- common1
- common2
- common3
- records


---

### CONF.PAS

**Unit Name:** CONF
**Risk Level:** MEDIUM
**Risk Score:** 9

**Risk Factors:**
- Binary file I/O
- Possible variant records
- Medium dependency count (11 dependencies)

**Dependencies (11):**
- Common
- File11
- Mail0
- Mail5
- common1
- common2
- common3
- common4
- common5
- records
- strproc


---

### CUSER.PAS

**Unit Name:** cuser
**Risk Level:** LOW
**Risk Score:** 4

**Risk Factors:**
- Possible variant records
- Medium-sized module (793 lines)

**Dependencies (10):**
- Crt
- Dos
- common
- common1
- common2
- common3
- common5
- misc5
- records
- strproc


---

### DOORS.PAS

**Unit Name:** doors
**Risk Level:** HIGH
**Risk Score:** 14

**Risk Factors:**
- Process execution (DOS Exec)
- DOS file system calls
- Possible variant records
- Medium dependency count (11 dependencies)

**Dependencies (11):**
- Crt
- Dos
- common
- common1
- common2
- common3
- common5
- execbat
- records
- strProc
- tmpcom


---

### EXAMPLE.PAS

**Unit Name:** PtrProc (Program)
**Risk Level:** LOW
**Risk Score:** 0

**Dependencies (1):**
- Crt


---

### EXEC.PAS

**Unit Name:** exec
**Risk Level:** CRITICAL
**Risk Score:** 31

**Risk Factors:**
- Low-level DOS interrupts
- Inline assembly
- Process execution (DOS Exec)
- Heavy pointer usage (52 occurrences)
- Possible variant records
- Medium-sized module (974 lines)

**Dependencies (2):**
- Dos
- checkpat


---

### EXECBAT.PAS

**Unit Name:** execbat
**Risk Level:** HIGH
**Risk Score:** 10

**Risk Factors:**
- Process execution (DOS Exec)
- DOS file system calls

**Dependencies (9):**
- Crt
- Dos
- common
- common1
- common2
- common3
- common5
- myio
- records


---

### EXECOLD.PAS

**Unit Name:** exec
**Risk Level:** CRITICAL
**Risk Score:** 31

**Risk Factors:**
- Low-level DOS interrupts
- Inline assembly
- Process execution (DOS Exec)
- Heavy pointer usage (52 occurrences)
- Possible variant records
- Medium-sized module (974 lines)

**Dependencies (2):**
- Dos
- checkpat


---

### IMPDOS.PAS

**Unit Name:** impDos
**Risk Level:** CRITICAL
**Risk Score:** 20

**Risk Factors:**
- Process execution (DOS Exec)
- Binary file I/O
- DOS file system calls
- Possible variant records
- High dependency count (21 dependencies)

**Dependencies (21):**
- Crt
- Dos
- cmd
- common
- common1
- common2
- common3
- common5
- execbat
- file0
- file1
- file11
- file2
- file3
- file4
- file8
- file9
- islu
- menus3
- records
- strproc


---

### INITP.PAS

**Unit Name:** initp
**Risk Level:** HIGH
**Risk Score:** 18

**Risk Factors:**
- Process execution (DOS Exec)
- DOS file system calls
- Heavy pointer usage (57 occurrences)
- Possible variant records
- Medium-sized module (583 lines)
- Medium dependency count (16 dependencies)

**Dependencies (16):**
- Crt
- Dos
- Overlay
- common
- common1
- common2
- common3
- common4
- common5
- myio
- records
- strProc
- sys
- sysop2
- sysop8
- wfcmenu


---

### INPUT.PAS

**Unit Name:** INPUT
**Risk Level:** LOW
**Risk Score:** 0

**No dependencies**


---

### INSTALL.PAS

**Unit Name:** impulse_install (Program)
**Risk Level:** CRITICAL
**Risk Score:** 25

**Risk Factors:**
- Low-level DOS interrupts
- Process execution (DOS Exec)
- Binary file I/O
- DOS file system calls

**Dependencies (3):**
- crt
- dos
- records


---

### ISLC.PAS

**Unit Name:** isl_Compiler (Program)
**Risk Level:** LOW
**Risk Score:** 0

**No dependencies**


---

### ISLD.PAS

**Unit Name:** ISL_DeCompiler (Program)
**Risk Level:** LOW
**Risk Score:** 0

**No dependencies**


---

### ISLU.PAS

**Unit Name:** ISLU
**Risk Level:** LOW
**Risk Score:** 0

**Dependencies (3):**
- common
- common5
- strproc


---

### LADLE.PAS

**Unit Name:** Ladle (Program)
**Risk Level:** MEDIUM
**Risk Score:** 7

**Risk Factors:**
- Inline assembly

**Dependencies (1):**
- AsmSauce


---

### LOGON1.PAS

**Unit Name:** logon1
**Risk Level:** HIGH
**Risk Score:** 16

**Risk Factors:**
- Process execution (DOS Exec)
- Heavy pointer usage (91 occurrences)
- Possible variant records
- Medium-sized module (606 lines)
- High dependency count (26 dependencies)

**Dependencies (26):**
- Crt
- Dos
- common
- common1
- common2
- common3
- common4
- common5
- cuser
- doors
- logon2
- mail0
- mail1
- mail2
- mail3
- mail4
- menus
- menus2
- misc1
- misc2
- misc5
- multinod
- newusers
- records
- scrlbk
- strProc


---

### LOGON2.PAS

**Unit Name:** logon2
**Risk Level:** MEDIUM
**Risk Score:** 7

**Risk Factors:**
- Binary file I/O
- High dependency count (23 dependencies)

**Dependencies (23):**
- Crt
- Dos
- common
- common1
- common2
- common3
- common4
- common5
- cuser
- doors
- mail0
- mail1
- mail2
- mail3
- mail4
- mail9
- menus
- misc1
- misc2
- misc5
- nuv
- records
- tmpcom


---

### LZHVIEW.PAS

**Unit Name:** LZHview
**Risk Level:** LOW
**Risk Score:** 3

**Risk Factors:**
- Possible variant records

**Dependencies (7):**
- Common
- Crt
- Dos
- common2
- records
- strProc
- sys


---

### MAKECLEA.PAS

**Unit Name:** MAKECLEA
**Risk Level:** MEDIUM
**Risk Score:** 5

**Risk Factors:**
- Binary file I/O

**Dependencies (1):**
- records


---

### MAKENUO.PAS

**Unit Name:** MAKENUO
**Risk Level:** LOW
**Risk Score:** 0

**No dependencies**


---

### MAKEWFC.PAS

**Unit Name:** MAKEWFC
**Risk Level:** LOW
**Risk Score:** 0

**No dependencies**


---

### MENUS.PAS

**Unit Name:** Menus
**Risk Level:** HIGH
**Risk Score:** 13

**Risk Factors:**
- Process execution (DOS Exec)
- Possible variant records
- Medium-sized module (678 lines)
- High dependency count (62 dependencies)

**Dependencies (62):**
- AnsiDrv
- CUser
- Common
- Common1
- Common2
- Common3
- Common5
- Conf
- Crt
- Doors
- Dos
- File0
- File1
- File10
- File11
- File12
- File13
- File14
- File2
- File4
- File5
- File6
- File8
- File9
- Mail0
- Mail1
- Mail2
- Mail3
- Mail4
- Mail5
- Mail6
- Mail9
- Menus2
- Menus3
- Misc1
- Misc2
- Misc3
- Misc4
- Misc5
- MyIO
- Records
- Rumors
- Script
- Sysop1
- Sysop11
- Sysop2
- Sysop2s
- Sysop3
- Sysop7
- Sysop8
- Sysop9
- Uconfig
- cmd
- common4
- impDos
- mail7
- multinod
- nuv
- output
- strProc
- strProc2
- timeBank


---

### MENUS2.PAS

**Unit Name:** menus2
**Risk Level:** HIGH
**Risk Score:** 11

**Risk Factors:**
- Process execution (DOS Exec)
- Possible variant records
- Medium-sized module (534 lines)

**Dependencies (10):**
- Common1
- Common2
- Common3
- Crt
- Dos
- common
- common5
- file4
- records
- strproc


---

### MENUS3.PAS

**Unit Name:** menus3
**Risk Level:** LOW
**Risk Score:** 3

**Risk Factors:**
- Possible variant records

**Dependencies (7):**
- Crt
- Dos
- common
- common1
- file0
- records
- strproc


---

### MMODEM.PAS

**Unit Name:** mmodem
**Risk Level:** LOW
**Risk Score:** 3

**Risk Factors:**
- Possible variant records

**Dependencies (7):**
- Crt
- Dos
- common
- common5
- myio
- records
- tmpcom


---

### MNUCONV.PAS

**Unit Name:** Menu_Converter (Program)
**Risk Level:** LOW
**Risk Score:** 3

**Risk Factors:**
- Possible variant records

**Dependencies (1):**
- records


---

### MSGPACK.PAS

**Unit Name:** msgpack
**Risk Level:** MEDIUM
**Risk Score:** 5

**Risk Factors:**
- Binary file I/O

**Dependencies (6):**
- Crt
- Dos
- common
- mail0
- records
- strProc


---

### MULTINOD.PAS

**Unit Name:** multinod
**Risk Level:** MEDIUM
**Risk Score:** 8

**Risk Factors:**
- Binary file I/O
- Possible variant records

**Dependencies (10):**
- Crt
- Dos
- common
- common1
- common2
- common3
- common5
- records
- strproc
- timejunk


---

### MYIO.PAS

**Unit Name:** myio
**Risk Level:** MEDIUM
**Risk Score:** 7

**Risk Factors:**
- Inline assembly

**Dependencies (2):**
- Crt
- dos


---

### NETFOSSL.PAS

**Unit Name:** netFossl
**Risk Level:** HIGH
**Risk Score:** 10

**Risk Factors:**
- Inline assembly
- Possible variant records

**No dependencies**


---

### NEWUSERS.PAS

**Unit Name:** newusers
**Risk Level:** MEDIUM
**Risk Score:** 6

**Risk Factors:**
- Binary file I/O
- Medium dependency count (19 dependencies)

**Dependencies (19):**
- Crt
- Dos
- common
- common1
- common2
- common3
- common5
- cuser
- mail0
- mail1
- mail2
- mail3
- misc2
- misc3
- misc4
- misc5
- records
- strProc
- uconfig


---

### NUV.PAS

**Unit Name:** Nuv
**Risk Level:** MEDIUM
**Risk Score:** 9

**Risk Factors:**
- Binary file I/O
- Possible variant records
- Medium dependency count (11 dependencies)

**Dependencies (11):**
- common
- common2
- common3
- common5
- crt
- menus
- menus2
- misc4
- records
- sys
- sysop3


---

### OUTPUT.PAS

**Unit Name:** output
**Risk Level:** LOW
**Risk Score:** 3

**Risk Factors:**
- Possible variant records

**Dependencies (8):**
- common
- common1
- common2
- common5
- crt
- dos
- records
- strProc


---

### OVRLAY.PAS

**Unit Name:** OvrLay
**Risk Level:** LOW
**Risk Score:** 0

**Dependencies (2):**
- Dos
- Overlay


---

### RUMORS.PAS

**Unit Name:** Rumors
**Risk Level:** MEDIUM
**Risk Score:** 5

**Risk Factors:**
- Binary file I/O

**Dependencies (9):**
- common
- common1
- common2
- common3
- common5
- output
- records
- strProc
- sys


---

### SCRIPT.PAS

**Unit Name:** Script
**Risk Level:** CRITICAL
**Risk Score:** 24

**Risk Factors:**
- Low-level DOS interrupts
- Process execution (DOS Exec)
- DOS file system calls
- Possible variant records
- Medium-sized module (951 lines)

**Dependencies (10):**
- Common
- Common1
- Common2
- Common3
- Crt
- Dos
- Records
- TmpCom
- common5
- strproc


---

### SCRLBK.PAS

**Unit Name:** SCRLBK
**Risk Level:** LOW
**Risk Score:** 0

**Dependencies (3):**
- Crt
- common
- myio


---

### STRPROC.PAS

**Unit Name:** strProc
**Risk Level:** LOW
**Risk Score:** 0

**No dependencies**


---

### STRPROC2.PAS

**Unit Name:** strProc2
**Risk Level:** MEDIUM
**Risk Score:** 7

**Risk Factors:**
- Inline assembly

**Dependencies (1):**
- common


---

### SYS.PAS

**Unit Name:** sys
**Risk Level:** HIGH
**Risk Score:** 10

**Risk Factors:**
- Inline assembly
- DOS file system calls

**Dependencies (2):**
- common
- records


---

### TEMP.PAS

**Unit Name:** TEMP
**Risk Level:** LOW
**Risk Score:** 0

**No dependencies**


---

### TIMEBANK.PAS

**Unit Name:** timeBank
**Risk Level:** LOW
**Risk Score:** 0

**Dependencies (4):**
- common
- common2
- common3
- sys


---

### TIMEJUNK.PAS

**Unit Name:** timejunk
**Risk Level:** LOW
**Risk Score:** 0

**Dependencies (1):**
- dos


---

### TIMESLIC.PAS

**Unit Name:** TIMESLIC
**Risk Level:** MEDIUM
**Risk Score:** 7

**Risk Factors:**
- Inline assembly

**Dependencies (2):**
- crt
- dos


---

### TIMETASK.PAS

**Unit Name:** TimeTask
**Risk Level:** HIGH
**Risk Score:** 12

**Risk Factors:**
- Inline assembly
- Overlay system usage

**Dependencies (1):**
- dos


---

### TMPCOM.PAS

**Unit Name:** tmpcom
**Risk Level:** HIGH
**Risk Score:** 18

**Risk Factors:**
- Interrupt handlers (platform-specific)
- Inline assembly
- Medium-sized module (754 lines)

**Dependencies (1):**
- Dos


---

### TMPOLD.PAS

**Unit Name:** tmpcom
**Risk Level:** HIGH
**Risk Score:** 18

**Risk Factors:**
- Interrupt handlers (platform-specific)
- Inline assembly
- Medium-sized module (754 lines)

**Dependencies (1):**
- Dos


---

### UCONFIG.PAS

**Unit Name:** UConfig
**Risk Level:** LOW
**Risk Score:** 3

**Risk Factors:**
- Possible variant records

**Dependencies (10):**
- AnsiDrv
- Common
- Crt
- Cuser
- Dos
- common1
- common2
- common3
- common5
- records


---

### ULCHECK.PAS

**Unit Name:** ulcheck
**Risk Level:** LOW
**Risk Score:** 4

**Risk Factors:**
- Possible variant records
- Medium dependency count (12 dependencies)

**Dependencies (12):**
- Common5
- Crt
- Dos
- ansidrv
- cmd
- common
- common1
- common2
- common3
- file1
- myio
- records


---

### WFCMENU.PAS

**Unit Name:** wfcmenu
**Risk Level:** HIGH
**Risk Score:** 19

**Risk Factors:**
- Process execution (DOS Exec)
- DOS file system calls
- Heavy pointer usage (55 occurrences)
- Possible variant records
- Medium-sized module (885 lines)
- High dependency count (33 dependencies)

**Dependencies (33):**
- Common1
- Common2
- Common3
- Crt
- Dos
- Sysop2
- common
- common4
- common5
- conf
- cuser
- mail2
- mail4
- mail5
- mail6
- misc1
- misc2
- misc3
- misc5
- mmodem
- msgpack
- multinod
- myio
- newusers
- records
- sys
- sysop1
- sysop11
- sysop3
- sysop7
- sysop8
- sysop9
- tmpcom


---

### ZIPVIEWU.PAS

**Unit Name:** zipviewu
**Risk Level:** HIGH
**Risk Score:** 10

**Risk Factors:**
- ABSOLUTE variables (memory-mapped)

**Dependencies (6):**
- Common
- Common5
- Crt
- Dos
- common2
- sys


---
