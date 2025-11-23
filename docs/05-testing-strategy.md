# Testing Strategy

**Project:** Impulse 7.1 BBS Modernization
**Last Updated:** 2025-11-22
**Target Audience:** Developers, QA Engineers

---

## Table of Contents

1. [Testing Philosophy](#testing-philosophy)
2. [Testing Pyramid](#testing-pyramid)
3. [Unit Testing](#unit-testing)
4. [Integration Testing](#integration-testing)
5. [Property-Based Testing](#property-based-testing)
6. [Fuzz Testing](#fuzz-testing)
7. [Performance Testing](#performance-testing)
8. [Compatibility Testing](#compatibility-testing)
9. [Test Data Management](#test-data-management)
10. [Continuous Integration](#continuous-integration)
11. [Coverage Requirements](#coverage-requirements)

---

## Testing Philosophy

### Core Principles

1. **Test-Driven Development (TDD)**: Write tests before implementation
2. **Comprehensive Coverage**: Aim for 80%+ code coverage
3. **Fast Feedback**: Tests should run quickly for rapid iteration
4. **Isolation**: Tests should be independent and repeatable
5. **Realistic Scenarios**: Test against real BBS usage patterns
6. **Legacy Compatibility**: Verify behavior matches original Pascal implementation

### Quality Standards

**Every feature must include:**
- Unit tests for individual functions/modules
- Integration tests for subsystem interactions
- Documentation of test scenarios
- Performance benchmarks for critical paths

**Before merging:**
- All tests pass on all platforms
- No reduction in code coverage
- Clippy lints pass
- Code review completed

---

## Testing Pyramid

```
           ┌─────────────┐
           │  End-to-End │  (10%)  - Full BBS workflows
           │    Tests    │
           └─────────────┘
         ┌─────────────────┐
         │   Integration   │  (20%)  - Subsystem interactions
         │      Tests      │
         └─────────────────┘
       ┌─────────────────────┐
       │     Unit Tests      │  (70%)  - Individual components
       │  (Property-Based)   │
       └─────────────────────┘
```

**Target Distribution:**
- **70% Unit Tests**: Fast, isolated, comprehensive
- **20% Integration Tests**: Subsystem interactions, protocols
- **10% End-to-End Tests**: Complete user workflows

---

## Unit Testing

### Scope

Test individual functions, structs, and modules in isolation.

### Patterns and Examples

#### Basic Unit Test

```rust
// crates/imp-ansi/src/parser.rs
pub fn parse_ansi_escape(seq: &str) -> Result<AnsiCommand> {
    if !seq.starts_with("\x1b[") {
        return Err(ParseError::InvalidSequence);
    }
    // ... implementation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ansi_escape_valid_color() {
        let result = parse_ansi_escape("\x1b[31m");
        assert!(result.is_ok());

        let cmd = result.unwrap();
        assert_eq!(cmd, AnsiCommand::SetForeground(Color::Red));
    }

    #[test]
    fn test_parse_ansi_escape_invalid_sequence() {
        let result = parse_ansi_escape("not_ansi");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ParseError::InvalidSequence));
    }

    #[test]
    fn test_parse_ansi_escape_empty_input() {
        let result = parse_ansi_escape("");
        assert!(result.is_err());
    }
}
```

#### Testing with Fixtures

```rust
// crates/imp-message/tests/jam_parser.rs
#[test]
fn test_parse_jam_header_from_fixture() {
    let data = include_bytes!("../fixtures/jam_header_valid.bin");
    let header = parse_jam_header(data).expect("Failed to parse");

    assert_eq!(header.signature, b"JAM\0");
    assert_eq!(header.revision, 1);
    assert_eq!(header.base_msg_num, 1);
    assert_eq!(header.active_msgs, 42);
}

#[test]
fn test_parse_jam_header_invalid_checksum() {
    let data = include_bytes!("../fixtures/jam_header_bad_checksum.bin");
    let result = parse_jam_header(data);

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ParseError::ChecksumMismatch { .. }));
}
```

#### Testing Async Code

```rust
// crates/imp-telnet/src/connection.rs
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn test_telnet_negotiate_binary_mode() {
        let (mut client, mut server) = tokio::io::duplex(1024);

        // Spawn server task
        let server_task = tokio::spawn(async move {
            negotiate_telnet_options(&mut server).await
        });

        // Simulate client
        client.write_all(&[IAC, DO, TELOPT_BINARY]).await.unwrap();

        // Verify server response
        let result = server_task.await.unwrap();
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_connection_timeout() {
        use tokio::time::{timeout, Duration};

        let result = timeout(
            Duration::from_millis(100),
            wait_for_never_completing_operation()
        ).await;

        assert!(result.is_err()); // Should timeout
    }
}
```

#### Testing Error Conditions

```rust
#[test]
fn test_user_record_validation() {
    let mut record = UserRecord::default();

    // Invalid username (empty)
    record.name = String::new();
    assert!(record.validate().is_err());

    // Invalid username (too long)
    record.name = "a".repeat(256);
    assert!(record.validate().is_err());

    // Invalid security level
    record.security = 999;
    assert!(record.validate().is_err());

    // Valid record
    record.name = "ValidUser".to_string();
    record.security = 50;
    assert!(record.validate().is_ok());
}
```

### Mock and Stub Patterns

```rust
use mockall::*;

#[automock]
pub trait FileSystem {
    fn read_file(&self, path: &str) -> Result<Vec<u8>>;
    fn write_file(&self, path: &str, data: &[u8]) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_user_with_mock_fs() {
        let mut mock_fs = MockFileSystem::new();

        mock_fs
            .expect_read_file()
            .with(eq("users.dat"))
            .times(1)
            .returning(|_| Ok(vec![/* test data */]));

        let user_manager = UserManager::new(mock_fs);
        let user = user_manager.load_user(1).unwrap();

        assert_eq!(user.id, 1);
    }
}
```

---

## Integration Testing

### Scope

Test interactions between multiple subsystems, crates, and external dependencies.

### Example: Message Base Integration

```rust
// tests/message_integration.rs
use imp_message::{MessageBase, JamBase};
use imp_user::UserManager;
use imp_file::FileManager;
use tempfile::TempDir;

#[tokio::test]
async fn test_post_and_read_message_workflow() {
    // Setup test environment
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path().join("msgbase");

    // Initialize subsystems
    let msg_base = JamBase::create(&base_path).await.unwrap();
    let user_mgr = UserManager::new(temp_dir.path()).await.unwrap();

    // Create test user
    let user = user_mgr.create_user("TestUser", "password").await.unwrap();

    // Post message
    let msg_id = msg_base.post_message(
        &user,
        "Test Subject",
        "Test message body"
    ).await.unwrap();

    // Retrieve and verify
    let retrieved = msg_base.get_message(msg_id).await.unwrap();
    assert_eq!(retrieved.subject, "Test Subject");
    assert_eq!(retrieved.from_user, user.id);
    assert_eq!(retrieved.body, "Test message body");
}

#[tokio::test]
async fn test_concurrent_message_access() {
    let temp_dir = TempDir::new().unwrap();
    let msg_base = Arc::new(JamBase::create(temp_dir.path()).await.unwrap());

    // Spawn multiple concurrent readers
    let mut tasks = vec![];
    for i in 0..10 {
        let mb = msg_base.clone();
        tasks.push(tokio::spawn(async move {
            mb.get_message_count().await
        }));
    }

    // All should succeed
    for task in tasks {
        assert!(task.await.is_ok());
    }
}
```

### Example: Protocol Integration

```rust
// tests/telnet_integration.rs
use imp_telnet::TelnetServer;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::test]
async fn test_telnet_full_connection_lifecycle() {
    // Start test server
    let server = TelnetServer::bind("127.0.0.1:0").await.unwrap();
    let addr = server.local_addr();

    tokio::spawn(async move { server.run().await });

    // Connect client
    let mut client = TcpStream::connect(addr).await.unwrap();

    // Negotiate options
    let mut buf = vec![0u8; 1024];
    let n = client.read(&mut buf).await.unwrap();
    assert!(n > 0); // Should receive telnet negotiations

    // Send login sequence
    client.write_all(b"testuser\r\n").await.unwrap();
    client.write_all(b"testpass\r\n").await.unwrap();

    // Verify login response
    let n = client.read(&mut buf).await.unwrap();
    let response = String::from_utf8_lossy(&buf[..n]);
    assert!(response.contains("Welcome") || response.contains("Login successful"));
}
```

---

## Property-Based Testing

### Scope

Use `proptest` to generate random inputs and verify invariants hold.

### Pattern: Round-Trip Serialization

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_user_record_roundtrip(
        name in "[a-zA-Z]{1,30}",
        security in 0u16..=255,
        calls in 0u32..=999999,
    ) {
        // Create record with generated values
        let mut original = UserRecord::default();
        original.name = name;
        original.security = security;
        original.total_calls = calls;

        // Serialize to bytes
        let bytes = original.to_bytes().unwrap();

        // Deserialize back
        let deserialized = UserRecord::from_bytes(&bytes).unwrap();

        // Should match exactly
        assert_eq!(original.name, deserialized.name);
        assert_eq!(original.security, deserialized.security);
        assert_eq!(original.total_calls, deserialized.total_calls);
    }
}
```

### Pattern: Protocol Invariants

```rust
proptest! {
    #[test]
    fn test_ansi_escape_always_starts_with_esc(
        color in 0u8..=255,
        bright in proptest::bool::ANY,
    ) {
        let sequence = generate_ansi_color_sequence(color, bright);

        // Invariant: All ANSI sequences start with ESC
        assert!(sequence.starts_with("\x1b["));

        // Invariant: All sequences end with 'm' for color
        assert!(sequence.ends_with('m'));
    }

    #[test]
    fn test_jam_header_checksum_always_valid(
        msg_num in 1u32..=9999999,
        reply_to in 0u32..=9999999,
        attr in prop::collection::vec(0u8..=255, 0..10),
    ) {
        let header = JamHeader {
            base_msg_num: msg_num,
            reply_to,
            attributes: attr,
            ..Default::default()
        };

        // Calculate checksum
        let checksum = header.calculate_checksum();

        // Serialize and deserialize
        let bytes = header.to_bytes();
        let parsed = JamHeader::from_bytes(&bytes).unwrap();

        // Invariant: Checksum always validates
        assert_eq!(parsed.calculate_checksum(), checksum);
    }
}
```

### Pattern: State Machine Testing

```rust
#[derive(Debug, Clone)]
enum ConnectionAction {
    Connect,
    Login(String, String),
    SendCommand(String),
    Disconnect,
}

proptest! {
    #[test]
    fn test_connection_state_machine(
        actions in prop::collection::vec(
            prop_oneof![
                Just(ConnectionAction::Connect),
                Just(ConnectionAction::Disconnect),
                ("[a-z]{5,10}", "[a-z]{8,12}").prop_map(|(u, p)|
                    ConnectionAction::Login(u, p)
                ),
            ],
            1..20
        )
    ) {
        let mut state = ConnectionState::Disconnected;

        for action in actions {
            state = match (state, action) {
                (ConnectionState::Disconnected, ConnectionAction::Connect) => {
                    ConnectionState::Connected
                },
                (ConnectionState::Connected, ConnectionAction::Login(_, _)) => {
                    ConnectionState::Authenticated
                },
                (_, ConnectionAction::Disconnect) => {
                    ConnectionState::Disconnected
                },
                _ => state, // Invalid transitions ignored
            };

            // Invariant: State machine always in valid state
            assert!(matches!(state,
                ConnectionState::Disconnected |
                ConnectionState::Connected |
                ConnectionState::Authenticated
            ));
        }
    }
}
```

---

## Fuzz Testing

### Scope

Use `cargo-fuzz` to discover edge cases and crashes in binary parsing and protocol handling.

### Setup

```bash
cargo install cargo-fuzz
cargo fuzz init
```

### Example: JAM Header Fuzzing

```rust
// fuzz/fuzz_targets/jam_header.rs
#![no_main]
use libfuzzer_sys::fuzz_target;
use imp_message::parse_jam_header;

fuzz_target!(|data: &[u8]| {
    // Should never panic, even on random input
    let _ = parse_jam_header(data);
});
```

### Example: ANSI Parser Fuzzing

```rust
// fuzz/fuzz_targets/ansi_parser.rs
#![no_main]
use libfuzzer_sys::fuzz_target;
use imp_ansi::AnsiParser;

fuzz_target!(|data: &[u8]| {
    let mut parser = AnsiParser::new();

    // Feed random data
    if let Ok(text) = std::str::from_utf8(data) {
        let _ = parser.parse(text);
    }

    // Parser should never panic
    // Check invariants:
    assert!(parser.is_in_valid_state());
});
```

### Running Fuzz Tests

```bash
# Run fuzzer for JAM header (Ctrl+C to stop)
cargo fuzz run jam_header

# Run with corpus (saved interesting inputs)
cargo fuzz run jam_header corpus/jam/

# Minimize failing input
cargo fuzz cmin jam_header

# Run with sanitizers
cargo fuzz run jam_header -- -sanitizer=address
```

---

## Performance Testing

### Benchmarking with Criterion

```rust
// benches/ansi_rendering.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use imp_ansi::AnsiRenderer;

fn bench_render_simple(c: &mut Criterion) {
    let renderer = AnsiRenderer::new();
    let text = "Hello, \x1b[31mRed\x1b[0m World!";

    c.bench_function("render_simple_ansi", |b| {
        b.iter(|| renderer.render(black_box(text)))
    });
}

fn bench_render_complex(c: &mut Criterion) {
    let renderer = AnsiRenderer::new();
    let text = include_str!("../fixtures/complex_ansi_art.ans");

    c.bench_function("render_complex_ansi_art", |b| {
        b.iter(|| renderer.render(black_box(text)))
    });
}

criterion_group!(benches, bench_render_simple, bench_render_complex);
criterion_main!(benches);
```

### Load Testing

```rust
// tests/load_test.rs
#[tokio::test]
#[ignore] // Run with: cargo test --ignored load_test
async fn test_concurrent_connections() {
    let server = start_test_server().await;

    let mut tasks = vec![];
    for i in 0..100 {
        tasks.push(tokio::spawn(async move {
            let mut client = connect_to_server().await.unwrap();
            client.login("user", "pass").await.unwrap();
            client.send_command("LIST").await.unwrap();
            client.logout().await.unwrap();
        }));
    }

    // All connections should succeed
    for task in tasks {
        assert!(task.await.is_ok());
    }
}
```

### Memory Profiling

```rust
#[test]
fn test_message_base_memory_usage() {
    let initial = get_memory_usage();

    {
        let msg_base = create_large_message_base(10_000);
        let after_create = get_memory_usage();

        // Should not exceed reasonable limits
        assert!(after_create - initial < 100_000_000); // 100MB
    }

    let after_drop = get_memory_usage();

    // Should release most memory
    assert!(after_drop - initial < 10_000_000); // 10MB
}
```

---

## Compatibility Testing

### Testing Against Original Pascal Implementation

```rust
// tests/compatibility.rs
use std::process::Command;
use tempfile::TempDir;

#[test]
#[ignore] // Requires DOSBox and original Pascal build
fn test_user_file_compatibility() {
    let temp_dir = TempDir::new().unwrap();

    // Create user with Pascal implementation
    let pascal_output = Command::new("dosbox")
        .args(&["-c", "IMP.EXE ADDUSER TestUser password"])
        .output()
        .expect("Failed to run Pascal version");

    assert!(pascal_output.status.success());

    // Read user with Rust implementation
    let user_mgr = UserManager::new(temp_dir.path()).unwrap();
    let user = user_mgr.load_user_by_name("TestUser").unwrap();

    assert_eq!(user.name, "TestUser");
}
```

### Binary Format Verification

```rust
#[test]
fn test_jam_header_binary_layout() {
    let header = JamHeader {
        signature: *b"JAM\0",
        revision: 1,
        base_msg_num: 1,
        // ... other fields
    };

    let bytes = header.to_bytes();

    // Verify exact byte layout matches Pascal
    assert_eq!(&bytes[0..4], b"JAM\0"); // Signature at offset 0
    assert_eq!(&bytes[4..8], &1u32.to_le_bytes()); // Revision at offset 4
    assert_eq!(bytes.len(), 1024); // Exact size match
}
```

---

## Test Data Management

### Fixture Organization

```
fixtures/
├── ansi/
│   ├── simple_color.ans
│   ├── complex_art.ans
│   └── invalid_sequence.txt
├── jam/
│   ├── valid_header.bin
│   ├── corrupted_header.bin
│   └── sample_messages.jam
├── users/
│   ├── valid_user.dat
│   └── legacy_user.dat
└── protocols/
    ├── telnet_negotiate.bin
    └── zmodem_initiate.bin
```

### Fixture Loading

```rust
// tests/common/fixtures.rs
pub fn load_fixture(path: &str) -> Vec<u8> {
    let fixture_path = format!("fixtures/{}", path);
    std::fs::read(&fixture_path)
        .unwrap_or_else(|e| panic!("Failed to load fixture {}: {}", fixture_path, e))
}

pub fn load_text_fixture(path: &str) -> String {
    String::from_utf8(load_fixture(path)).expect("Invalid UTF-8 in fixture")
}

// Usage in tests:
#[test]
fn test_with_fixture() {
    let data = load_fixture("jam/valid_header.bin");
    let header = parse_jam_header(&data).unwrap();
    // ... assertions
}
```

### Test Database Seeding

```rust
pub async fn setup_test_database() -> TestDatabase {
    let db = TestDatabase::new_temp().await;

    // Seed with realistic data
    db.add_users(&[
        ("alice", "pass1", 100),
        ("bob", "pass2", 50),
        ("charlie", "pass3", 25),
    ]).await;

    db.add_messages(100).await; // Generate 100 test messages

    db
}

#[tokio::test]
async fn test_with_seeded_db() {
    let db = setup_test_database().await;

    let users = db.get_all_users().await.unwrap();
    assert_eq!(users.len(), 3);
}
```

---

## Continuous Integration

### CI Pipeline Configuration

```yaml
# .github/workflows/test.yml
name: Test Suite

on: [push, pull_request]

jobs:
  test:
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
        rust: [stable, beta]

    runs-on: ${{ matrix.os }}

    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@master
        with:
          toolchain: ${{ matrix.rust }}
          components: rustfmt, clippy

      - name: Cache cargo registry
        uses: actions/cache@v4
        with:
          path: ~/.cargo/registry
          key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}

      - name: Check formatting
        run: cargo fmt --all -- --check

      - name: Run clippy
        run: cargo clippy --all-targets --all-features -- -D warnings

      - name: Run tests
        run: cargo test --workspace --verbose

      - name: Run ignored tests
        run: cargo test --workspace --ignored --verbose

      - name: Build release
        run: cargo build --workspace --release

  coverage:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install tarpaulin
        run: cargo install cargo-tarpaulin

      - name: Generate coverage
        run: cargo tarpaulin --workspace --out Xml --output-dir coverage

      - name: Upload to codecov
        uses: codecov/codecov-action@v4
        with:
          files: ./coverage/cobertura.xml

  benchmark:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v4

      - name: Run benchmarks
        run: cargo bench --workspace -- --output-format bencher | tee output.txt

      - name: Store benchmark result
        uses: benchmark-action/github-action-benchmark@v1
        with:
          tool: 'cargo'
          output-file-path: output.txt
```

---

## Coverage Requirements

### Target Metrics

| Component | Minimum Coverage | Target Coverage |
|-----------|-----------------|-----------------|
| Core types | 90% | 95% |
| Binary parsing | 85% | 90% |
| Protocol handlers | 80% | 85% |
| ANSI rendering | 80% | 90% |
| Message base | 85% | 90% |
| User management | 90% | 95% |
| Overall project | 80% | 85% |

### Measuring Coverage

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --workspace --out Html --output-dir coverage

# Open report
xdg-open coverage/index.html

# Generate for specific crate
cargo tarpaulin -p imp-ansi --out Lcov

# Exclude test modules
cargo tarpaulin --workspace --exclude-files 'tests/*'
```

### Coverage Exemptions

Some code may be excluded from coverage requirements:

```rust
#[cfg(not(tarpaulin_include))]
fn platform_specific_code() {
    // Platform-specific code that can't be tested in CI
}

// Or with comment:
// LCOV_EXCL_START
fn unreachable_error_handling() {
    // Defensive code that should never execute
}
// LCOV_EXCL_STOP
```

---

## Test Maintenance

### Regular Review

- **Weekly**: Review failing tests, update fixtures
- **Sprint End**: Analyze coverage reports, address gaps
- **Monthly**: Audit test suite performance, prune obsolete tests
- **Quarterly**: Review test strategy, update this document

### Flaky Test Policy

- **Never ignore flaky tests** without investigation
- Add retries only as temporary measure
- Track flaky tests in issue tracker
- Root cause and fix or remove

### Test Documentation

Each test file should include module documentation:

```rust
//! Integration tests for JAM message base.
//!
//! Tests cover:
//! - Message creation and retrieval
//! - Index management
//! - Concurrent access
//! - Corruption recovery
//!
//! Fixtures: `fixtures/jam/`
```

---

## Summary

This testing strategy ensures:

- **Quality**: Comprehensive coverage catches regressions early
- **Confidence**: Property-based and fuzz testing reveal edge cases
- **Performance**: Benchmarking prevents performance degradation
- **Compatibility**: Binary format tests ensure legacy compatibility
- **Maintainability**: Clear patterns make tests easy to write and understand

**Remember:** Tests are not overhead—they are the foundation of reliable software. Invest in testing, and the project will repay that investment many times over.

---

**Questions or suggestions?** Contribute to this document via pull request.
