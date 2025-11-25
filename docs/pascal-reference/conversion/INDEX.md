# Pascal to Rust Conversion Guides

Practical guides for converting Pascal code to Rust, including type mappings and conversion plans.

**Last Updated:** 2025-11-24

---

## Overview

This directory contains practical conversion guides, type mapping references, and specific conversion plans for transforming the Pascal codebase into idiomatic Rust.

---

## Documentation Files

### [conversion-order.md](conversion-order.md)

**Recommended conversion sequence based on dependency analysis**

Topological ordering of the 96 Pascal units to ensure dependencies are converted first.

**Conversion Phases:**
1. **Foundation Units (Convert First):**
   - RECORDS.PAS ✅ Complete (Sprint 5)
   - COMMON.PAS
   - CONFIG.PAS ✅ Complete (Sprint 4)
   - INIT.PAS

2. **Core Services:**
   - USER.PAS ✅ Complete (Sprint 6)
   - AUTH.PAS ✅ Complete (Sprint 6)
   - SESSION.PAS
   - DATABASE.PAS

3. **I/O Layer:**
   - COMMS.PAS (high-risk)
   - FOSSIL.PAS (high-risk)
   - SERIAL.PAS

4. **Business Logic:**
   - MESSAGE.PAS
   - FILE.PAS
   - DOOR.PAS
   - MENU.PAS

5. **UI Layer:**
   - ANSI.PAS
   - TERMINAL.PAS
   - SCREEN.PAS

**Critical Path:** Foundation → Core Services → I/O Layer → Business Logic → UI Layer

### [type-mapping.md](type-mapping.md)

**Comprehensive Pascal to Rust type mappings**

Complete reference for converting Pascal types to Rust equivalents.

**Basic Types:**
- Byte → u8
- ShortInt → i8
- Word → u16
- Integer → i16
- LongInt → i32
- Char → u8 (ASCII) or char (Unicode)
- Boolean → bool
- Real → f32
- String → String or &str

**Structured Types:**
- Array → [T; N] or Vec<T>
- Record → struct
- Set → BitFlags or HashSet<T>
- File → std::fs::File
- Text → std::io::BufReader/Writer

**Pointer Types:**
- Pointer → Box<T>, Rc<T>, Arc<T>
- ^ (Pascal pointer) → &T, &mut T, *const T, *mut T

**Special Cases:**
- String[N] → String (heap) or [u8; N+1] (stack)
- PChar → *const c_char or CString
- Variant records → enum with data

### [type-reconciliation.md](type-reconciliation.md)

**Type system reconciliation and migration notes**

Handling complex type system differences between Pascal and Rust.

**Topics:**
- **String Handling:**
  - Pascal length-prefixed strings
  - Null-terminated C strings
  - Rust UTF-8 strings
  - Conversion strategies

- **Memory Layout:**
  - Record packing in Pascal
  - Struct alignment in Rust
  - repr(C) for binary compatibility
  - Padding and size considerations

- **Pointer Semantics:**
  - Pascal pointer arithmetic
  - Rust references and lifetimes
  - Unsafe pointer operations
  - Safe abstractions

- **Type Safety:**
  - Pascal weak typing
  - Rust strong typing
  - Newtype pattern for type safety
  - From/Into trait implementations

### [quick-reference-pascal-to-rust.md](quick-reference-pascal-to-rust.md)

**Quick reference cheat sheet for common conversions**

Fast lookup guide for everyday conversion patterns.

**Categories:**
- Variable declarations
- Function definitions
- Control flow (if/case/for/while)
- String operations
- File I/O
- Error handling
- Memory management
- Array/collection operations

**Format:** Side-by-side Pascal and Rust examples.

**Example:**
```pascal
{ Pascal }
procedure DoSomething(var x: Integer);
begin
  Inc(x);
end;
```

```rust
// Rust
fn do_something(x: &mut i32) {
    *x += 1;
}
```

### [pascal-binary-formats.md](pascal-binary-formats.md)

**Binary data format specifications for Impulse 7.1**

Detailed documentation of all binary file formats used by Impulse.

**File Formats:**
- **USER.LST:** User database format
  - PascalUserRec structure (293 bytes)
  - Field layouts and offsets
  - String encoding
  - Binary compatibility requirements

- **MESSAGE.DAT:** Message base format
  - JAM format specification
  - Hudson format specification
  - Message header structure
  - Index file format

- **FILE.DAT:** File area catalog
  - FileEntry structure
  - File description format
  - Upload/download tracking

- **CONFIG.DAT:** BBS configuration
  - BbsConfig structure
  - Default values
  - Validation rules

**Conversion Strategy:**
- Use binrw crate for binary I/O
- repr(C) for layout control
- Explicit padding for alignment
- Endianness handling
- Round-trip testing

### [records-pas-conversion-plan.md](records-pas-conversion-plan.md)

**Detailed conversion plan for RECORDS.PAS** ✅ Complete

Post-completion reference documenting the RECORDS.PAS conversion.

**Completed Sprint 5:**
- 11 Pascal record types converted
- 195 tests implemented
- Binary compatibility validated
- Round-trip serialization tested

**Conversion Approach:**
- Modern Rust types (User, FileEntry, Message, BbsConfig)
- Pascal compatibility types (PascalUserRec, etc.)
- Conversion traits (from_pascal, to_pascal)
- Serialization support (serde, binrw)

**Lessons Learned:**
- Start with type definitions
- Implement Pascal compatibility layer
- Comprehensive serialization tests
- Document binary format explicitly

---

## Conversion Best Practices

### Before Converting a Unit

1. **Read Analysis:** Check pascal-unit-analysis.md
2. **Check Dependencies:** Verify all dependencies converted
3. **Review Types:** Consult type-mapping.md
4. **Assess Risk:** Check risk-assessment docs
5. **Plan Tests:** Identify test cases

### During Conversion

1. **Semantic Rewrite:** Understand intent, not literal translation
2. **Modern Idioms:** Use Rust best practices
3. **Type Safety:** Leverage Rust's type system
4. **Error Handling:** Use Result<T, E> not error codes
5. **Documentation:** rustdoc for all public APIs
6. **Tests:** Unit tests for all functionality

### After Conversion

1. **Code Review:** Minimum 2 reviewers
2. **Integration Tests:** Test with dependent modules
3. **Binary Compatibility:** Validate data format compatibility
4. **Performance:** Benchmark against Pascal (if relevant)
5. **Documentation:** Update conversion docs

---

## Testing Strategy for Conversions

**Unit Tests:**
- Test each function in isolation
- Cover edge cases and error conditions
- Property-based tests for invariants

**Integration Tests:**
- Test module interactions
- Validate data flow
- Ensure API contracts met

**Compatibility Tests:**
- Read Pascal-generated files
- Write files Pascal can read
- Round-trip serialization
- Binary format validation

**Regression Tests:**
- Compare behavior to Pascal reference
- Test vectors from original
- Edge cases from original code

---

## Related Documentation

- **[Analysis](../analysis/)** - Pascal source code analysis
- **[Risk Assessment](../risk-assessment/)** - Conversion risks
- **[Planning](../../planning/)** - Sprint plans and schedule
- **[Implementation](../../implementation/)** - Development guides

---

[← Back to Pascal Reference](../INDEX.md) | [← Back to Documentation Index](../../INDEX.md)
