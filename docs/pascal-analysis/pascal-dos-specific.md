# Pascal DOS-Specific Code Analysis

**Files with DOS calls:** 23

## DOS Function Usage

| File | DOS Functions Used |
|------|-------------------|
| SCRIPT.PAS | EXEC, GETDIR, MKDIR, CHDIR, RMDIR, SWAPVECTORS |
| IMPDOS.PAS | GETDIR, MKDIR, CHDIR, RMDIR |
| INSTALL.PAS | EXEC, MKDIR, CHDIR, SWAPVECTORS |
| COMMON1.PAS | GETDIR, CHDIR |
| COMMON2.PAS | GETDIR, CHDIR |
| COMMON5.PAS | EXEC, SWAPVECTORS |
| EXEC.PAS | EXEC, SWAPVECTORS |
| EXECBAT.PAS | GETDIR, CHDIR |
| EXECOLD.PAS | EXEC, SWAPVECTORS |
| FILE0.PAS | GETDIR, CHDIR |
| FILE2.PAS | GETDIR, CHDIR |
| SYSOP9.PAS | GETDIR, MKDIR |
| WFCMENU.PAS | EXEC, CHDIR |
| BPTRAP.PAS | KEEP |
| DOORS.PAS | CHDIR |
| FILE4.PAS | EXEC |
| IMP.PAS | GETDIR |
| INITP.PAS | MKDIR |
| RECORDS.PAS | KEEP |
| SYS.PAS | GETDIR |
| SYSOP2D.PAS | KEEP |
| SYSOP6.PAS | EXEC |
| SYSOP8.PAS | MKDIR |

## Rust Migration Strategy

| DOS Function | Rust Equivalent |
|--------------|----------------|
| EXEC | std::process::Command |
| GETDIR | std::env::current_dir() |
| MKDIR | std::fs::create_dir() |
| CHDIR | std::env::set_current_dir() |
| RMDIR | std::fs::remove_dir() |
| SWAPVECTORS | N/A (DOS-specific) |
| KEEP | N/A (TSR functionality) |
| GETINTERVEC/SETINTERVEC | Signal handlers (OS-specific) |