# Pascal Source Dependencies

**Total Units:** 114
**Total Dependencies:** 1070
**Analysis Date:** 2025-11-23

## Units with Most Dependencies (Top 10)

| Unit | Dependency Count | Dependencies |
|------|------------------|--------------|
| Impulse (Program) | 69 | Checkpat, Crt, Overlay, OvrLay, BpTrap, ... (+64 more) |
| Menus | 62 | Crt, Dos, Sysop3, Script, Rumors, ... (+57 more) |
| wfcmenu | 33 | Crt, Dos, sysop3, sysop7, sysop8, ... (+28 more) |
| logon1 | 26 | Crt, Dos, common5, logon2, newusers, ... (+21 more) |
| logon2 | 23 | Crt, Dos, tmpcom, common5, records, ... (+18 more) |
| impDos | 21 | Crt, Dos, common5, records, common, ... (+16 more) |
| sysop2 | 21 | Crt, Dos, common1, common2, common3, ... (+16 more) |
| newusers | 19 | Crt, Dos, uconfig, common5, mail0, ... (+14 more) |
| file12 | 18 | Crt, Dos, records, file0, file8, ... (+13 more) |
| file1 | 17 | Crt, Dos, ulcheck, file3, execbat, ... (+12 more) |

## All Dependencies

### ANSIDRV.PAS
**Unit Name:** AnsiDrv
**Dependencies (2):**

- Crt
- common

### ANSIEDIT.PAS
**Unit Name:** ansiedit
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

### ASMSAUCE.PAS
**Unit Name:** ASMSAUCE
**No dependencies**

### BPTRAP.PAS
**Unit Name:** BPTrap
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

### CMD.PAS
**Unit Name:** cmd
**Dependencies (6):**

- asmSauce
- common
- common1
- common2
- common3
- records

### COMMON.PAS
**Unit Name:** common
**Dependencies (5):**

- Crt
- Dos
- output
- records
- timejunk

### COMMON1.PAS
**Unit Name:** common1
**Dependencies (5):**

- Crt
- Dos
- myio
- records
- tmpcom

### COMMON2.PAS
**Unit Name:** common2
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

### COMMON3.PAS
**Unit Name:** common3
**Dependencies (7):**

- Crt
- Dos
- common5
- myio
- records
- strProc
- tmpcom

### COMMON4.PAS
**Unit Name:** Common4
**Dependencies (5):**

- Common
- Common5
- Crt
- Dos
- Records

### COMMON5.PAS
**Unit Name:** Common5
**Dependencies (1):**

- records

### CONF.PAS
**Unit Name:** CONF
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

### CUSER.PAS
**Unit Name:** cuser
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

### DOORS.PAS
**Unit Name:** doors
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

### EXAMPLE.PAS
**Unit Name:** PtrProc (Program)
**Dependencies (1):**

- Crt

### EXEC.PAS
**Unit Name:** exec
**Dependencies (2):**

- Dos
- checkpat

### EXECBAT.PAS
**Unit Name:** execbat
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

### EXECOLD.PAS
**Unit Name:** exec
**Dependencies (2):**

- Dos
- checkpat

### FILE0.PAS
**Unit Name:** file0
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

### FILE1.PAS
**Unit Name:** file1
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

### FILE10.PAS
**Unit Name:** file10
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

### FILE11.PAS
**Unit Name:** file11
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

### FILE12.PAS
**Unit Name:** file12
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

### FILE13.PAS
**Unit Name:** file13
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

### FILE14.PAS
**Unit Name:** file14
**Dependencies (8):**

- Crt
- Dos
- common
- common5
- file0
- file11
- myio
- records

### FILE2.PAS
**Unit Name:** file2
**Dependencies (8):**

- Crt
- Dos
- common
- common5
- execbat
- file0
- file4
- records

### FILE3.PAS
**Unit Name:** File3
**Dependencies (2):**

- Common
- records

### FILE4.PAS
**Unit Name:** file4
**Dependencies (7):**

- Crt
- Dos
- common
- common5
- file0
- file14
- records

### FILE5.PAS
**Unit Name:** file5
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

### FILE6.PAS
**Unit Name:** file6
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

### FILE8.PAS
**Unit Name:** file8
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

### FILE9.PAS
**Unit Name:** file9
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

### IMP.PAS
**Unit Name:** Impulse (Program)
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

### IMPDOS.PAS
**Unit Name:** impDos
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

### INITP.PAS
**Unit Name:** initp
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

### INPUT.PAS
**Unit Name:** INPUT
**No dependencies**

### INSTALL.PAS
**Unit Name:** impulse_install (Program)
**Dependencies (3):**

- crt
- dos
- records

### ISLC.PAS
**Unit Name:** isl_Compiler (Program)
**No dependencies**

### ISLD.PAS
**Unit Name:** ISL_DeCompiler (Program)
**No dependencies**

### ISLU.PAS
**Unit Name:** ISLU
**Dependencies (3):**

- common
- common5
- strproc

### LADLE.PAS
**Unit Name:** Ladle (Program)
**Dependencies (1):**

- AsmSauce

### LOGON1.PAS
**Unit Name:** logon1
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

### LOGON2.PAS
**Unit Name:** logon2
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

### LZHVIEW.PAS
**Unit Name:** LZHview
**Dependencies (7):**

- Common
- Crt
- Dos
- common2
- records
- strProc
- sys

### MAIL0.PAS
**Unit Name:** mail0
**Dependencies (7):**

- Crt
- Dos
- common
- common5
- records
- strproc
- timejunk

### MAIL1.PAS
**Unit Name:** mail1
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

### MAIL2.PAS
**Unit Name:** mail2
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

### MAIL3.PAS
**Unit Name:** mail3
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

### MAIL4.PAS
**Unit Name:** mail4
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

### MAIL5.PAS
**Unit Name:** mail5
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

### MAIL6.PAS
**Unit Name:** mail6
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

### MAIL7.PAS
**Unit Name:** mail7
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

### MAIL9.PAS
**Unit Name:** mail9
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

### MAKECLEA.PAS
**Unit Name:** MAKECLEA
**Dependencies (1):**

- records

### MAKENUO.PAS
**Unit Name:** MAKENUO
**No dependencies**

### MAKEWFC.PAS
**Unit Name:** MAKEWFC
**No dependencies**

### MENUS.PAS
**Unit Name:** Menus
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

### MENUS2.PAS
**Unit Name:** menus2
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

### MENUS3.PAS
**Unit Name:** menus3
**Dependencies (7):**

- Crt
- Dos
- common
- common1
- file0
- records
- strproc

### MISC1.PAS
**Unit Name:** misc1
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

### MISC2.PAS
**Unit Name:** misc2
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

### MISC3.PAS
**Unit Name:** misc3
**Dependencies (8):**

- Crt
- Dos
- common
- common1
- common2
- common3
- common5
- records

### MISC4.PAS
**Unit Name:** misc4
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

### MISC5.PAS
**Unit Name:** misc5
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

### MMODEM.PAS
**Unit Name:** mmodem
**Dependencies (7):**

- Crt
- Dos
- common
- common5
- myio
- records
- tmpcom

### MNUCONV.PAS
**Unit Name:** Menu_Converter (Program)
**Dependencies (1):**

- records

### MSGPACK.PAS
**Unit Name:** msgpack
**Dependencies (6):**

- Crt
- Dos
- common
- mail0
- records
- strProc

### MULTINOD.PAS
**Unit Name:** multinod
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

### MYIO.PAS
**Unit Name:** myio
**Dependencies (2):**

- Crt
- dos

### NETFOSSL.PAS
**Unit Name:** netFossl
**No dependencies**

### NEWUSERS.PAS
**Unit Name:** newusers
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

### NUV.PAS
**Unit Name:** Nuv
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

### OUTPUT.PAS
**Unit Name:** output
**Dependencies (8):**

- common
- common1
- common2
- common5
- crt
- dos
- records
- strProc

### OVRLAY.PAS
**Unit Name:** OvrLay
**Dependencies (2):**

- Dos
- Overlay

### RECORDS.PAS
**Unit Name:** Records
**No dependencies**

### RUMORS.PAS
**Unit Name:** Rumors
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

### SCRIPT.PAS
**Unit Name:** Script
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

### SCRLBK.PAS
**Unit Name:** SCRLBK
**Dependencies (3):**

- Crt
- common
- myio

### STRPROC.PAS
**Unit Name:** strProc
**No dependencies**

### STRPROC2.PAS
**Unit Name:** strProc2
**Dependencies (1):**

- common

### SYS.PAS
**Unit Name:** sys
**Dependencies (2):**

- common
- records

### SYSOP1.PAS
**Unit Name:** sysop1
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

### SYSOP11.PAS
**Unit Name:** sysop11
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

### SYSOP2.PAS
**Unit Name:** sysop2
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

### SYSOP21.PAS
**Unit Name:** sysop21
**Dependencies (8):**

- Crt
- Dos
- common
- common1
- common2
- common3
- common5
- records

### SYSOP2A.PAS
**Unit Name:** sysop2a
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

### SYSOP2B.PAS
**Unit Name:** sysop2b
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

### SYSOP2C.PAS
**Unit Name:** sysop2c
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

### SYSOP2D.PAS
**Unit Name:** sysop2d
**Dependencies (8):**

- Crt
- Dos
- common
- common1
- common2
- common3
- common4
- common5

### SYSOP2E.PAS
**Unit Name:** sysop2e
**Dependencies (8):**

- Crt
- Dos
- common
- common1
- common2
- common3
- common4
- common5

### SYSOP2F.PAS
**Unit Name:** sysop2f
**Dependencies (8):**

- Crt
- Dos
- common
- common1
- common2
- common3
- common4
- common5

### SYSOP2G.PAS
**Unit Name:** sysop2g
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

### SYSOP2H.PAS
**Unit Name:** sysop2h
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

### SYSOP2I.PAS
**Unit Name:** sysop2i
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

### SYSOP2J.PAS
**Unit Name:** sysop2j
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

### SYSOP2S.PAS
**Unit Name:** sysop2s
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

### SYSOP2Z.PAS
**Unit Name:** sysop2z
**Dependencies (7):**

- Crt
- Dos
- common
- common4
- common5
- cuser
- records

### SYSOP3.PAS
**Unit Name:** sysop3
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

### SYSOP6.PAS
**Unit Name:** sysop6
**Dependencies (4):**

- Crt
- Dos
- common
- overlay

### SYSOP7.PAS
**Unit Name:** sysop7
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

### SYSOP7M.PAS
**Unit Name:** sysop7m
**Dependencies (8):**

- Crt
- Dos
- common
- common3
- common5
- file9
- menus2
- records

### SYSOP8.PAS
**Unit Name:** sysop8
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

### SYSOP9.PAS
**Unit Name:** sysop9
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

### TEMP.PAS
**Unit Name:** TEMP
**No dependencies**

### TIMEBANK.PAS
**Unit Name:** timeBank
**Dependencies (4):**

- common
- common2
- common3
- sys

### TIMEJUNK.PAS
**Unit Name:** timejunk
**Dependencies (1):**

- dos

### TIMESLIC.PAS
**Unit Name:** TIMESLIC
**Dependencies (2):**

- crt
- dos

### TIMETASK.PAS
**Unit Name:** TimeTask
**Dependencies (1):**

- dos

### TMPCOM.PAS
**Unit Name:** tmpcom
**Dependencies (1):**

- Dos

### TMPOLD.PAS
**Unit Name:** tmpcom
**Dependencies (1):**

- Dos

### UCONFIG.PAS
**Unit Name:** UConfig
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

### ULCHECK.PAS
**Unit Name:** ulcheck
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

### WFCMENU.PAS
**Unit Name:** wfcmenu
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

### ZIPVIEWU.PAS
**Unit Name:** zipviewu
**Dependencies (6):**

- Common
- Common5
- Crt
- Dos
- common2
- sys
