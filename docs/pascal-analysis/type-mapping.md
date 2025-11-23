# Pascal to Rust Type Mapping

**Source:** Analysis of Impulse 7.1 BBS Pascal codebase
**Date:** 2025-11-23

## Overview

This document provides comprehensive mappings from Pascal types used in the
original Impulse BBS to their Rust equivalents.

## Integer Types

| Pascal Type | Size | Range | Rust Type | Notes |
|-------------|------|-------|-----------|-------|
| `Byte` | 1 byte | 0..255 | `u8` | Unsigned 8-bit |
| `ShortInt` | 1 byte | -128..127 | `i8` | Signed 8-bit |
| `Word` | 2 bytes | 0..65535 | `u16` | Unsigned 16-bit |
| `Integer` | 2 bytes | -32768..32767 | `i16` | Signed 16-bit |
| `LongInt` | 4 bytes | -2147483648..2147483647 | `i32` | Signed 32-bit |
| `Cardinal` | 4 bytes | 0..4294967295 | `u32` | Unsigned 32-bit |
| `Comp` | 8 bytes | Large integer | `i64` | Signed 64-bit |

## Real/Float Types

| Pascal Type | Size | Rust Type | Notes |
|-------------|------|-----------|-------|
| `Real` | 6 bytes | `f32` or `f64` | Use f32 for compatibility |
| `Single` | 4 bytes | `f32` | IEEE 754 single precision |
| `Double` | 8 bytes | `f64` | IEEE 754 double precision |
| `Extended` | 10 bytes | `f64` | Map to f64 (some precision loss) |

## Boolean Type

| Pascal Type | Rust Type | Notes |
|-------------|-----------|-------|
| `Boolean` | `bool` | Direct mapping |
| `ByteBool` | `bool` | Convert byte (0/1) to bool |
| `WordBool` | `bool` | Convert word (0/1) to bool |
| `LongBool` | `bool` | Convert longint (0/1) to bool |

## Character and String Types

| Pascal Type | Rust Type | Migration Notes |
|-------------|-----------|-----------------|
| `Char` | `char` or `u8` | Use `char` for text, `u8` for binary |
| `String[N]` | `String` or `[u8; N]` | Fixed-length → dynamic or fixed array |
| `String` (ShortString) | `String` | Pascal string (255 max) → Rust String |
| `PChar` | `&str` or `CString` | Null-terminated → Rust string slice |
| `AnsiString` | `String` | Dynamic string → String |

**String Migration Examples:**
```rust
// Pascal: var name: String[80];
// Rust Option 1 (dynamic):
let mut name = String::new();

// Rust Option 2 (fixed, validated):
struct FixedString80 {
    data: [u8; 80],
    len: usize,
}

// Pascal: var ptr: PChar;
// Rust:
let ptr: &str = "...";
// or for C interop:
use std::ffi::CString;
let ptr = CString::new("...").unwrap();
```

## Pointer Types

| Pascal Type | Rust Type | Notes |
|-------------|-----------|-------|
| `^Type` | `Box<Type>` | Heap-allocated, owned |
| `^Type` | `&Type` or `&mut Type` | Borrowed reference |
| `^Type` | `Arc<Type>` | Shared ownership, thread-safe |
| `^Type` | `Rc<Type>` | Shared ownership, single-threaded |
| `Pointer` | `*const T` or `*mut T` | Raw pointer (unsafe) |
| `nil` | `None` or `null` | Use `Option<T>` for safety |

**Pointer Migration Strategy:**
```rust
// Pascal: type PUser = ^UserRecord;
// Rust (owned):
type PUser = Box<UserRecord>;

// Rust (borrowed):
fn process_user(user: &UserRecord) { ... }

// Rust (shared):
type PUser = Arc<UserRecord>;

// Pascal: if ptr <> nil then
// Rust:
if let Some(user) = user_opt { ... }
```

## Array Types

| Pascal Type | Rust Type | Notes |
|-------------|-----------|-------|
| `Array[1..N] of Type` | `[Type; N]` | Fixed-size array (0-indexed in Rust) |
| `Array[Low..High] of Type` | `[Type; SIZE]` | Calculate size: High-Low+1 |
| Dynamic arrays | `Vec<Type>` | Heap-allocated, growable |
| `Array of Const` | `&[&dyn Any]` or macros | Variable arguments |

**Array Migration Examples:**
```rust
// Pascal: var boards: Array[1..254] of BoardRec;
// Rust (fixed):
let boards: [BoardRec; 254] = [BoardRec::default(); 254];

// Rust (dynamic):
let mut boards: Vec<BoardRec> = Vec::with_capacity(254);

// Note: Pascal arrays are 1-indexed, Rust arrays are 0-indexed
// Pascal: boards[1] → Rust: boards[0]
```

## Record Types

Pascal `Record` types map directly to Rust `struct` types.

**Migration Pattern:**
```rust
// Pascal:
// type
//   UserRecord = record
//     name: String[80];
//     age: Integer;
//     active: Boolean;
//   end;

// Rust:
#[derive(Debug, Clone)]
struct UserRecord {
    name: String,  // or [u8; 80]
    age: i16,
    active: bool,
}
```

**Variant Records (case...of):**
```rust
// Pascal:
// type
//   ShapeType = (Circle, Rectangle);
//   Shape = record
//     case kind: ShapeType of
//       Circle: (radius: Real);
//       Rectangle: (width, height: Real);
//   end;

// Rust (enum with data):
enum Shape {
    Circle { radius: f32 },
    Rectangle { width: f32, height: f32 },
}
```

## Enumeration Types

Pascal enumerations map to Rust enums.

**Migration Pattern:**
```rust
// Pascal:
// type
//   Color = (Red, Green, Blue);

// Rust:
#[derive(Debug, Clone, Copy, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}
```

## Set Types

Pascal `Set of` types can be mapped to several Rust types:

| Pascal Type | Rust Type | Use Case |
|-------------|-----------|----------|
| `Set of Enum` | `HashSet<Enum>` | General purpose |
| `Set of Byte` | `BitVec` or `[bool; 256]` | Small sets |
| `Set of Char` | `HashSet<char>` | Character sets |
| Flag sets | `bitflags!` macro | Efficient flag storage |

**Set Migration Examples:**
```rust
// Pascal:
// type
//   Flags = (fRead, fWrite, fExec);
//   FlagSet = Set of Flags;

// Rust Option 1 (HashSet):
use std::collections::HashSet;
type FlagSet = HashSet<Flags>;

// Rust Option 2 (bitflags):
use bitflags::bitflags;
bitflags! {
    struct FlagSet: u8 {
        const READ  = 0b001;
        const WRITE = 0b010;
        const EXEC  = 0b100;
    }
}
```

## File Types

| Pascal Type | Rust Type | Notes |
|-------------|-----------|-------|
| `Text` | `std::fs::File` + `BufReader/Writer` | Text file I/O |
| `File of Type` | `std::fs::File` + serialization | Binary file I/O |
| `File` (untyped) | `std::fs::File` | Raw file access |

**File I/O Migration:**
```rust
// Pascal: var f: File of UserRecord;
// Rust:
use std::fs::File;
use std::io::{Read, Write};
use bincode;

// Write
let mut file = File::create("users.dat")?;
let encoded = bincode::serialize(&user)?;
file.write_all(&encoded)?;

// Read
let mut file = File::open("users.dat")?;
let mut buffer = Vec::new();
file.read_to_end(&mut buffer)?;
let user: UserRecord = bincode::deserialize(&buffer)?;
```

## Procedural Types

Pascal procedure/function pointer types map to Rust closures or function pointers.

**Migration Pattern:**
```rust
// Pascal:
// type
//   TCallback = Procedure(data: Integer);
//   TFunction = Function(x: Integer): Integer;

// Rust (function pointer):
type Callback = fn(data: i32);
type Function = fn(x: i32) -> i32;

// Rust (closure):
type Callback = Box<dyn Fn(i32)>;
type Function = Box<dyn Fn(i32) -> i32>;
```

## Special Types

| Pascal Type | Rust Type | Notes |
|-------------|-----------|-------|
| `Variant` | `Box<dyn Any>` | Dynamic typing (rarely needed) |
| `OleVariant` | N/A | Windows COM (platform-specific) |
| `Packed Record` | `#[repr(C, packed)]` | Memory layout control |
| `Absolute` | `static mut` (unsafe) | Fixed memory address |

## Type System Differences

**Key Differences:**

1. **Indexing:** Pascal arrays start at any index (often 1), Rust arrays start at 0
2. **Strings:** Pascal has fixed-length strings, Rust uses dynamic UTF-8 strings
3. **Pointers:** Pascal pointers are nullable, Rust uses `Option<T>` for safety
4. **Memory Safety:** Rust enforces ownership, Pascal allows manual management
5. **Type Safety:** Rust is stricter (no implicit type conversions)

**Migration Checklist:**

- [ ] Convert integer types with correct signedness
- [ ] Replace fixed-length strings with appropriate Rust type
- [ ] Convert pointers to owned/borrowed types
- [ ] Adjust array indices (1-based → 0-based)
- [ ] Replace nil checks with Option pattern matching
- [ ] Convert set types to HashSet or bitflags
- [ ] Replace variant records with Rust enums
- [ ] Update file I/O to use std::fs and serialization