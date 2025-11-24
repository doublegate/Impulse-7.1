# Pascal to Rust Quick Reference Guide

**Source:** RECORDS.PAS type conversion analysis
**Sprint:** 5 - Core Types Implementation
**Date:** 2025-11-23

This is a quick reference guide for converting Pascal types from RECORDS.PAS to Rust. For comprehensive details, see [records-pas-conversion-plan.md](./records-pas-conversion-plan.md).

---

## Constants Quick Reference

| Pascal | Value | Rust | Type |
|--------|-------|------|------|
| `maxboards` | 254 | `MAX_BOARDS` | `usize` |
| `maxconfs` | 20 | `MAX_CONFERENCES` | `usize` |
| `maxuboards` | 254 | `MAX_FILE_AREAS` | `usize` |
| `maxprotocols` | 20 | `MAX_PROTOCOLS` | `usize` |
| `maxevents` | 10 | `MAX_EVENTS` | `usize` |
| `maxubatchfiles` | 20 | `MAX_BATCH_FILES` | `usize` |
| `maxmenucmds` | 50 | `MAX_MENU_COMMANDS` | `usize` |
| `maxBufSize` | 4096 | `MAX_BUFFER_SIZE` | `usize` |
| `maxStrings` | 255 | `MAX_STRINGS` | `usize` |

**Module:** `impulse-types/src/constants.rs`

---

## Type Aliases Quick Reference

| Pascal | Rust (Binary) | Rust (Modern) | Notes |
|--------|---------------|---------------|-------|
| `string[N]` | `[u8; N+1]` | `String` | Pascal: 1 len byte + N chars |
| `astr` (string[160]) | `[u8; 161]` | `String` | Large string |
| `str80` (string[80]) | `[u8; 81]` | `String` | Common string size |
| `acstring` (string[20]) | `[u8; 21]` | `String` | Access condition string |
| `acrq` ('@'..'Z') | `u8` | `ArFlag(u8)` | Character range 64-90 |
| `Byte` | `u8` | `u8` | Unsigned 8-bit |
| `Integer` | `i16` | `i16` | Signed 16-bit |
| `Word` | `u16` | `u16` | Unsigned 16-bit |
| `LongInt` | `i32` | `i32` | Signed 32-bit |
| `Boolean` | `u8` (0/1) | `bool` | 1 byte in Pascal |

**Module:** `impulse-types/src/pascal_compat.rs`

---

## Enumeration Types (10 enums, 88 variants)

### Summary Table

| Pascal Type | Variants | Rust Type | Storage | Module |
|------------|----------|-----------|---------|--------|
| `uflags` | 24 | `UserFlags` | `u32` (bitflags) | `user_flags.rs` |
| `flistflags` | 8 | `FileListFlags` | `u8` (bitflags) | `file_flags.rs` |
| `anontyp` | 5 | `AnonymousType` | `#[repr(u8)]` enum | `message_types.rs` |
| `msgindexstatr` | 8 | `MessageIndexStatus` | `u8` (bitflags) | `message_types.rs` |
| `mbflags` | 13 | `MessageBoardFlags` | `u16` (bitflags) | `message_types.rs` |
| `fbflags` | 6 | `FileBoardFlags` | `u8` (bitflags) | `file_types.rs` |
| `mnuflags` | 6 | `MenuFlags` | `u8` (bitflags) | `menu_types.rs` |
| `cmdflags` | 3 | `CommandFlags` | `u8` (bitflags) | `menu_types.rs` |
| `xbflags` | 4 | `ProtocolFlags` | `u8` (bitflags) | `protocol_types.rs` |
| `filstat` | 3 | `FileStatus` | `u8` (bitflags) | `file_types.rs` |

### When to Use Bitflags vs Enum

- **Bitflags:** Pascal `set of Enum` → Use `bitflags!` macro (allows multiple flags set)
- **Enum:** Pascal simple enum → Use `#[repr(u8)]` enum (single value)

**Example:**
```rust
// Pascal: set of uflags → Bitflags
bitflags! {
    pub struct UserFlags: u32 {
        const RESTRICTED_LOGON = 0b0001;
        const RESTRICTED_CHAT  = 0b0010;
        // ... can combine multiple flags
    }
}

// Pascal: anontyp enum → Simple enum
#[repr(u8)]
pub enum AnonymousType {
    NotAllowed = 0,
    Allowed = 1,
    Forced = 2,
    // ... only one value at a time
}
```

---

## Array and Specialized Types

| Pascal | Rust (Binary) | Rust (Modern) | Notes |
|--------|---------------|---------------|-------|
| `clrs` (array[FALSE..TRUE,0..9] of byte) | `[[u8; 10]; 2]` | `ColorArray` | 2D array |
| `secrange` (array[0..255] of integer) | `[i16; 256]` | `SecurityRange` | Security table |
| `cpackdatetime` (array[1..6] of byte) | `[u8; 6]` | `PackedDateTime` struct | Date/time |
| `mzscanr` (set of 1..maxboards) | `[u8; 32]` | `MessageScanSet` | Bitset, 256 bits |
| `fzscanr` (set of 0..maxuboards) | `[u8; 32]` | `FileScanSet` | Bitset, 256 bits |
| `mhireadr` (array[1..maxboards] of cpackdatetime) | `[PackedDateTime; 254]` | `MessageHighReadArray` | Array of dates |

**Module:** `datetime_types.rs`, `terminal_types.rs`, `scan_types.rs`

---

## Record Types - Priority Order

### Tier 1: CRITICAL (Implement First)

| Pascal Record | Lines | Rust Type | Binary Struct | Module |
|--------------|-------|-----------|---------------|--------|
| `userrec` | 83 | `User` | `PascalUserRecord` | `user.rs` |
| `systatrec` | 153 | `BbsConfig` | `PascalSystatRecord` | `system_config.rs` |
| `boardrec` | 27 | `MessageBoard` | `PascalBoardRecord` | `message_types.rs` |
| `ulrec` | 19 | `FileArea` | `PascalFileAreaRecord` | `file_types.rs` |

### Tier 2: HIGH

| Pascal Record | Rust Type | Module |
|--------------|-----------|--------|
| `msgindexrec` | `MessageIndex` | `message_types.rs` |
| `mheaderrec` | `MessageHeader` | `message_types.rs` |
| `fromtoinfo` | `FromToInfo` | `message_types.rs` |
| `ulfrec` | `FileRecord` | `file_types.rs` |
| `zscanrec` | `NewScanRecord` | `scan_types.rs` |
| `eventrec` | `Event` | `event_types.rs` |

### Tier 3: MEDIUM

| Pascal Record | Rust Type | Module |
|--------------|-----------|--------|
| `fstringrec` | `UiStrings` | `strings.rs` |
| `menurec` | `Menu` | `menu_types.rs` |
| `commandrec` | `Command` | `menu_types.rs` |
| `protrec` | `Protocol` | `protocol_types.rs` |
| `modemrec` | `Modem` | `network_types.rs` |

### Tier 4: LOW (Supporting)

| Pascal Record | Rust Type | Module |
|--------------|-----------|--------|
| `smalrec` | `SortedNameRecord` | `user.rs` |
| `zlogrec` | `SystemLog` | `system_config.rs` |
| `noderec` | `Node` | `network_types.rs` |
| `nodemessrec` | `NodeMessage` | `network_types.rs` |
| `forumrec` | `Forum` | `network_types.rs` |
| `ConfRec` | `Conference` | `system_config.rs` |
| `nuvrec` | `NewUserVote` | `user.rs` |
| `nuvcomrec` | `NewUserVoteComment` | `user.rs` |
| `lcallers` | `LastCaller` | `user.rs` |
| `verbrec` | `VerboseDescription` | `file_types.rs` |
| `datetimerec` | `DateTimeRecord` | `datetime_types.rs` |
| `fidorec` | `FidoNet` | `network_types.rs` |
| `promptrec` | `Prompt` | `strings.rs` |
| `BBSrec` | `BbsListEntry` | `network_types.rs` |

---

## Binary Compatibility Patterns

### Two-Layer Architecture

**Layer 1: Pascal-Compatible (Binary I/O)**
```rust
// Located in src/legacy/
#[repr(C, packed)]
pub struct PascalUserRecord {
    pub name: [u8; 37],  // String[36] = 1 len + 36 chars
    pub realname: [u8; 37],
    pub pw: [u8; 21],
    // ... exact Pascal layout
}
```

**Layer 2: Modern Rust (Application Logic)**
```rust
// Located in src/
pub struct User {
    pub id: u32,
    pub name: String,
    pub real_name: String,
    pub password_hash: String,
    // ... modern types
}
```

**Conversion Layer**
```rust
impl From<PascalUserRecord> for User { /* ... */ }
impl TryFrom<&User> for PascalUserRecord { /* ... */ }
```

---

## String Conversion Patterns

### Pascal String Format

Pascal `string[N]` layout:
```
Byte 0: Length (0-N)
Bytes 1-N: Data
```

### Conversion Functions

```rust
// Pascal → Rust
fn pascal_string_to_rust(bytes: &[u8]) -> String {
    let len = bytes[0] as usize;
    let data = &bytes[1..=len.min(bytes.len() - 1)];
    String::from_utf8_lossy(data).into_owned()
}

// Rust → Pascal (may truncate)
fn rust_string_to_pascal<const N: usize>(s: &str, max_len: usize) -> Result<[u8; N]> {
    let bytes = s.as_bytes();
    let len = bytes.len().min(max_len);
    let mut result = [0u8; N];
    result[0] = len as u8;
    result[1..=len].copy_from_slice(&bytes[..len]);
    Ok(result)
}
```

**Usage:**
```rust
// Reading from Pascal binary
let name = pascal_string_to_rust(&pascal_rec.name);

// Writing to Pascal binary
let pascal_name = rust_string_to_pascal::<37>(&user.name, 36)?;
```

---

## Date Conversion Patterns

### Pascal Date Formats

**Format 1: String Date (MM/DD/YY)**
```rust
// Pascal → chrono::NaiveDate
fn pascal_date_to_chrono(date_str: &str) -> Result<chrono::NaiveDate> {
    // Parse "MM/DD/YY", handle Y2K
    // 90-99 → 1990-1999
    // 00-89 → 2000-2089
}

// chrono::NaiveDate → Pascal
fn chrono_date_to_pascal(date: chrono::NaiveDate) -> String {
    format!("{:02}/{:02}/{:02}", date.month(), date.day(), date.year() % 100)
}
```

**Format 2: Packed Date/Time (6 bytes)**
```rust
#[derive(Debug, Clone, Copy)]
pub struct PackedDateTime {
    pub year: u8,   // Offset from 1900
    pub month: u8,  // 1-12
    pub day: u8,    // 1-31
    pub hour: u8,   // 0-23
    pub minute: u8, // 0-59
    pub second: u8, // 0-59
}

impl PackedDateTime {
    pub fn to_naive_datetime(&self) -> Result<chrono::NaiveDateTime> { /* ... */ }
    pub fn from_naive_datetime(dt: chrono::NaiveDateTime) -> Result<Self> { /* ... */ }
}
```

---

## Boolean Conversion

```rust
// Pascal → Rust
fn pascal_bool_to_rust(byte: u8) -> bool {
    byte != 0
}

// Rust → Pascal
fn rust_bool_to_pascal(b: bool) -> u8 {
    if b { 1 } else { 0 }
}
```

**In Structs:**
```rust
// Pascal binary layer
#[repr(C, packed)]
pub struct PascalRecord {
    pub deleted: u8,  // Boolean as byte
}

// Modern Rust layer
pub struct Record {
    pub deleted: bool,
}

// Conversion
impl From<PascalRecord> for Record {
    fn from(p: PascalRecord) -> Self {
        Record {
            deleted: pascal_bool_to_rust(p.deleted),
        }
    }
}
```

---

## Set Type Conversion

### Pascal Sets → Rust Bitsets

**Small Sets (< 256 elements):**
```rust
// Pascal: set of 1..254
// Rust: [u8; 32] (256 bits / 8 = 32 bytes)

pub type MessageScanSet = [u8; 32];

// Check if bit N is set
fn is_set(set: &MessageScanSet, n: usize) -> bool {
    let byte_index = n / 8;
    let bit_index = n % 8;
    (set[byte_index] & (1 << bit_index)) != 0
}

// Set bit N
fn set_bit(set: &mut MessageScanSet, n: usize) {
    let byte_index = n / 8;
    let bit_index = n % 8;
    set[byte_index] |= 1 << bit_index;
}

// Clear bit N
fn clear_bit(set: &mut MessageScanSet, n: usize) {
    let byte_index = n / 8;
    let bit_index = n % 8;
    set[byte_index] &= !(1 << bit_index);
}
```

**Small Enum Sets (bitflags):**
```rust
// Pascal: set of uflags (24 variants)
// Rust: bitflags u32

bitflags! {
    pub struct UserFlags: u32 {
        const RESTRICTED_LOGON = 0b00000001;
        const RESTRICTED_CHAT  = 0b00000010;
        const RESTRICTED_EMAIL = 0b00000100;
        // ... up to 32 flags
    }
}

// Usage
let mut flags = UserFlags::empty();
flags.insert(UserFlags::RESTRICTED_LOGON);
flags.insert(UserFlags::RESTRICTED_CHAT);

if flags.contains(UserFlags::RESTRICTED_LOGON) {
    // User is restricted
}
```

---

## Module Organization Quick Ref

### Core Modules

| Module | Purpose | Types |
|--------|---------|-------|
| `constants.rs` | System constants | All `MAX_*` constants |
| `pascal_compat.rs` | Helper functions | String/date/bool conversion |
| `datetime_types.rs` | Date/time types | `PackedDateTime` |
| `user.rs` | User account | `User`, `UserStats`, `SecurityLevel` |
| `user_flags.rs` | User flags | `UserFlags` bitflags |
| `security_types.rs` | Security | `SecurityRange`, levels |
| `terminal_types.rs` | Terminal | `ColorArray`, settings |
| `message_types.rs` | Message system | `MessageBoard`, `MessageIndex`, `MessageHeader` |
| `file_types.rs` | File areas | `FileArea`, `FileRecord` |
| `file_flags.rs` | File flags | `FileListFlags`, `FileBoardFlags`, `FileStatus` |
| `scan_types.rs` | NewScan | `NewScanRecord`, scan sets |
| `menu_types.rs` | Menu system | `Menu`, `Command`, flags |
| `protocol_types.rs` | Transfer protocols | `Protocol`, `ProtocolFlags` |
| `event_types.rs` | Event scheduler | `Event` |
| `network_types.rs` | Multi-node, FidoNet | `Node`, `FidoNet`, `BbsListEntry` |
| `system_config.rs` | System config | `BbsConfig`, `SystemLog` |
| `strings.rs` | UI strings | `UiStrings` (large) |

### Binary Compatibility Layer

| Module | Purpose |
|--------|---------|
| `legacy/mod.rs` | Re-exports |
| `legacy/user_record.rs` | `PascalUserRecord` |
| `legacy/systat_record.rs` | `PascalSystatRecord` |
| `legacy/board_record.rs` | `PascalBoardRecord` |
| `legacy/file_area_record.rs` | `PascalFileAreaRecord` |

---

## Common Patterns Cheat Sheet

### Creating a New Type from Pascal Record

**Step 1: Define Pascal-Compatible Binary Struct**
```rust
// src/legacy/my_record.rs
#[repr(C, packed)]
pub struct PascalMyRecord {
    pub name: [u8; 41],      // String[40]
    pub count: i16,          // Integer
    pub active: u8,          // Boolean
    pub flags: u8,           // Set of flags
}
```

**Step 2: Define Modern Rust Struct**
```rust
// src/my_types.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyRecord {
    pub name: String,
    pub count: i16,
    pub active: bool,
    pub flags: MyFlags,
}
```

**Step 3: Implement Conversion**
```rust
impl From<PascalMyRecord> for MyRecord {
    fn from(p: PascalMyRecord) -> Self {
        MyRecord {
            name: pascal_string_to_rust(&p.name),
            count: p.count,
            active: pascal_bool_to_rust(p.active),
            flags: MyFlags::from_bits_truncate(p.flags),
        }
    }
}

impl TryFrom<&MyRecord> for PascalMyRecord {
    type Error = Error;

    fn try_from(r: &MyRecord) -> Result<Self> {
        Ok(PascalMyRecord {
            name: rust_string_to_pascal::<41>(&r.name, 40)?,
            count: r.count,
            active: rust_bool_to_pascal(r.active),
            flags: r.flags.bits(),
        })
    }
}
```

**Step 4: Add Tests**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion_round_trip() {
        let original = MyRecord {
            name: "Test".into(),
            count: 42,
            active: true,
            flags: MyFlags::FLAG1 | MyFlags::FLAG2,
        };

        let pascal: PascalMyRecord = (&original).try_into().unwrap();
        let converted: MyRecord = pascal.into();

        assert_eq!(converted.name, original.name);
        assert_eq!(converted.count, original.count);
        assert_eq!(converted.active, original.active);
        assert_eq!(converted.flags, original.flags);
    }

    #[test]
    fn test_binary_serialization() {
        let record = PascalMyRecord {
            name: rust_string_to_pascal::<41>("Test", 40).unwrap(),
            count: 42,
            active: 1,
            flags: 0b0011,
        };

        let bytes = bincode::serialize(&record).unwrap();
        let deserialized: PascalMyRecord = bincode::deserialize(&bytes).unwrap();

        assert_eq!(deserialized.count, record.count);
    }
}
```

---

## Testing Patterns

### Unit Test Template
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let obj = MyType::new(...);
        assert!(obj.validate().is_ok());
    }

    #[test]
    fn test_validation_invalid_name() {
        let mut obj = MyType::new(...);
        obj.name = String::new(); // Invalid
        assert!(obj.validate().is_err());
    }

    #[test]
    fn test_json_serialization() {
        let obj = MyType::new(...);
        let json = serde_json::to_string(&obj).unwrap();
        let deserialized: MyType = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, obj);
    }

    #[test]
    fn test_bincode_serialization() {
        let obj = MyType::new(...);
        let bytes = bincode::serialize(&obj).unwrap();
        let deserialized: MyType = bincode::deserialize(&bytes).unwrap();
        assert_eq!(deserialized, obj);
    }

    #[test]
    fn test_pascal_conversion() {
        let rust_obj = MyType::new(...);
        let pascal: PascalMyType = (&rust_obj).try_into().unwrap();
        let back: MyType = pascal.into();
        assert_eq!(back.name, rust_obj.name);
    }
}
```

---

## Dependencies Cargo.toml

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
bincode = "1.3"
bitflags = "2.4"
chrono = { version = "0.4", features = ["serde"] }
thiserror = "2.0"

[dev-dependencies]
serde_test = "1.0"
```

---

## Common Pitfalls

### 1. Pascal String Length Byte
❌ **Wrong:**
```rust
let name = &pascal_rec.name[..]; // Includes length byte!
```

✅ **Correct:**
```rust
let len = pascal_rec.name[0] as usize;
let name = &pascal_rec.name[1..=len];
```

### 2. Array Indexing (Pascal 1-based, Rust 0-based)
❌ **Wrong:**
```rust
// Pascal: array[1..5] of byte
let arr: [u8; 5] = [...];
let first = arr[1]; // Wrong! Off by one
```

✅ **Correct:**
```rust
// Pascal array[1..5] → Rust [0..4]
let arr: [u8; 5] = [...];
let first = arr[0]; // Correct
```

### 3. Boolean as Byte
❌ **Wrong:**
```rust
struct PascalRec {
    pub active: bool, // Will be 1 byte in Rust, but might not match Pascal
}
```

✅ **Correct:**
```rust
#[repr(C, packed)]
struct PascalRec {
    pub active: u8, // Explicit byte, convert to bool in Rust layer
}
```

### 4. Set Binary Layout
❌ **Wrong:**
```rust
// Assuming set is stored as u32 directly
let flags: u32 = pascal_rec.flags;
```

✅ **Correct:**
```rust
// Check actual Pascal binary layout (may be byte array)
let flags = UserFlags::from_bits_truncate(u32::from_le_bytes(pascal_rec.flags));
```

### 5. String Truncation
❌ **Wrong:**
```rust
// Panic if string too long
let pascal_name = rust_string_to_pascal(&user.name, 36).unwrap();
```

✅ **Correct:**
```rust
// Truncate gracefully
let truncated = &user.name[..user.name.len().min(36)];
let pascal_name = rust_string_to_pascal(truncated, 36)?;
```

---

## Useful Commands

### Check Binary Layout Size
```bash
# In test
println!("Size: {}", std::mem::size_of::<PascalUserRecord>());
```

### Hex Dump for Debugging
```rust
fn hex_dump(data: &[u8]) {
    for (i, chunk) in data.chunks(16).enumerate() {
        print!("{:04x}: ", i * 16);
        for byte in chunk {
            print!("{:02x} ", byte);
        }
        println!();
    }
}
```

### Verify Pascal Layout
```pascal
// In Pascal test program
program TestLayout;
var
  rec: userrec;
begin
  WriteLn('Size of userrec: ', SizeOf(rec));
  WriteLn('Offset of name: ', PtrInt(@rec.name) - PtrInt(@rec));
  WriteLn('Offset of pw: ', PtrInt(@rec.pw) - PtrInt(@rec));
end.
```

---

## Key Takeaways

1. **Always use two-layer design:** Binary-compatible layer + Modern Rust layer
2. **Pascal strings include length byte:** `string[N]` = 1 + N bytes
3. **Use `#[repr(C, packed)]`** for binary compatibility structs
4. **Pascal Boolean = u8:** Use helper functions for conversion
5. **Arrays are 0-indexed in Rust:** Adjust Pascal 1-based indices
6. **Sets are bitsets:** Use byte arrays or bitflags
7. **Test binary compatibility:** Create Pascal test programs, verify with hex dumps
8. **Truncate gracefully:** Handle strings that exceed Pascal limits
9. **Y2K dates:** 90-99 = 1990-1999, 00-89 = 2000-2089
10. **Document everything:** Future developers need to understand Pascal origin

---

**For detailed information, see:**
- [records-pas-conversion-plan.md](./records-pas-conversion-plan.md) - Comprehensive conversion plan
- [type-mapping.md](./type-mapping.md) - General Pascal→Rust type mappings
- [conversion-order.md](./conversion-order.md) - Dependency-aware conversion order

**Sprint 5 Status:** Phase 1 - Analysis and Planning Complete
**Next:** Begin implementation (constants.rs, enums, PackedDateTime)
