# Pascal Hardware Access Analysis

## Interrupt Handlers

**Files with interrupt handlers:** 2

| File | Interrupt Declarations |
|------|----------------------|
| TMPCOM.PAS | 48 |
| TMPOLD.PAS | 48 |

## Inline Assembly / Hardware Access

**Files with inline assembly:** 14

| File | Inline ASM Usage |
|------|-----------------|
| NETFOSSL.PAS | 26 |
| MYIO.PAS | 8 |
| ANSIDRV.PAS | 2 |
| BPTRAP.PAS | 2 |
| SYS.PAS | 2 |
| TIMESLIC.PAS | 2 |
| TIMETASK.PAS | 2 |
| TMPCOM.PAS | 2 |
| TMPOLD.PAS | 2 |
| ASMSAUCE.PAS | 1 |
| COMMON.PAS | 1 |
| COMMON5.PAS | 1 |
| EXEC.PAS | 1 |
| EXECOLD.PAS | 1 |

## ABSOLUTE Variables

**Files with ABSOLUTE variables:** 2

| File | ABSOLUTE Usage |
|------|---------------|
| MAIL7.PAS | 2 |
| ZIPVIEWU.PAS | 1 |

## Rust Migration Notes

**Interrupt Handlers:**
- Modern OS: Use signal handlers or OS-specific APIs
- Linux: signal() or sigaction()
- Windows: SetConsoleCtrlHandler()

**Inline Assembly:**
- Rust supports inline assembly with asm! macro
- Consider replacing with safe Rust abstractions
- Use crates like raw-cpuid for CPU feature detection

**ABSOLUTE Variables:**
- Used for memory-mapped I/O in DOS
- Replace with safe memory access patterns
- Use std::ptr for low-level memory access if needed