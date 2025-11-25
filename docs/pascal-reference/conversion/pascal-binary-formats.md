# Pascal Binary File Format Analysis

**Files with binary file I/O:** 29

| File | Binary File Declarations |
|------|-------------------------|
| COMMON.PAS | 10 |
| MULTINOD.PAS | 4 |
| NUV.PAS | 4 |
| COMMON1.PAS | 3 |
| MAKECLEA.PAS | 3 |
| COMMON2.PAS | 2 |
| LOGON2.PAS | 2 |
| MISC1.PAS | 2 |
| MISC2.PAS | 2 |
| MISC5.PAS | 2 |
| CMD.PAS | 1 |
| CONF.PAS | 1 |
| FILE1.PAS | 1 |
| FILE12.PAS | 1 |
| FILE5.PAS | 1 |
| FILE8.PAS | 1 |
| IMP.PAS | 1 |
| IMPDOS.PAS | 1 |
| INSTALL.PAS | 1 |
| MSGPACK.PAS | 1 |
| NEWUSERS.PAS | 1 |
| RUMORS.PAS | 1 |
| SYSOP11.PAS | 1 |
| SYSOP2A.PAS | 1 |
| SYSOP2I.PAS | 1 |
| SYSOP2J.PAS | 1 |
| SYSOP2S.PAS | 1 |
| SYSOP3.PAS | 1 |
| SYSOP6.PAS | 1 |

## Binary File Types

Pascal uses typed files (FILE OF RecordType) for binary I/O.
These files store records in binary format with no metadata.

**Common Binary Files in BBS Systems:**
- USERS.DAT - User account records
- FILES.DAT - File area records
- MESSAGES.DAT - Message base
- CONFIG.DAT - System configuration

## Rust Migration Strategy

**Options:**
1. **bincode crate** - Binary serialization (similar to Pascal typed files)
2. **serde with bincode** - Structured serialization with metadata
3. **Custom binary format** - Manual Read/Write implementation

**Recommended Approach:**
```rust
use bincode;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct UserRecord {
    // fields
}

// Write
let encoded = bincode::serialize(&user)?;
file.write_all(&encoded)?;

// Read
let user: UserRecord = bincode::deserialize(&bytes)?;
```