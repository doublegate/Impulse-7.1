# Sprint 07: Terminal I/O Foundation

**Phase:** Phase 1 - Foundation
**Duration:** 3 weeks
**Sprint Dates:** TBD
**Status:** Not Started

---

## Sprint Overview

Sprint 07 implements the terminal I/O abstraction layer and basic ANSI rendering engine. This sprint creates the foundation for all terminal-based interaction, including ANSI color support, cursor control, and terminal capability detection. This abstraction allows the BBS to work with different terminal types while providing a consistent API.

**Context:** This is the seventh sprint of Phase 1 (Foundation). Terminal I/O is essential for all user-facing features and must support both legacy ANSI terminals and modern terminal emulators.

**Expected Outcomes:** By the end of this sprint, the project will have a complete terminal I/O abstraction that can detect terminal capabilities, render ANSI art, and provide a clean API for all screen operations.

---

## Objectives

- [ ] Implement `TerminalDriver` trait for abstraction
- [ ] Create basic ANSI sequence rendering engine
- [ ] Support terminal capability detection
- [ ] Build telnet terminal driver implementation

---

## Deliverables

| Deliverable | Type | Acceptance Criteria |
|-------------|------|---------------------|
| `impulse-terminal` crate | Code | TerminalDriver trait defined with complete API |
| ANSI renderer | Code | Parses and renders basic ANSI sequences (colors, cursor movement) |
| Terminal capability detection | Code | Detects ANSI vs ASCII, screen dimensions, color support |
| Telnet terminal driver | Code | Implements TerminalDriver for TelnetStream with IAC handling |

---

## Detailed Tasks

### Task Category 1: TerminalDriver Trait Design

- [ ] **Task 1.1**: Define TerminalDriver trait
  - Implementation notes: Async methods for read/write, screen control, capability query
  - Files affected: `crates/impulse-terminal/src/driver.rs`
  - Estimated hours: 4

- [ ] **Task 1.2**: Add basic I/O methods
  - Implementation notes: read(), write(), flush() async methods
  - Files affected: `crates/impulse-terminal/src/driver.rs`
  - Estimated hours: 2

- [ ] **Task 1.3**: Add screen control methods
  - Implementation notes: clear_screen(), move_cursor(), hide_cursor(), show_cursor()
  - Files affected: `crates/impulse-terminal/src/driver.rs`
  - Estimated hours: 3

- [ ] **Task 1.4**: Add capability detection methods
  - Implementation notes: detect_capabilities(), get_dimensions(), supports_color()
  - Files affected: `crates/impulse-terminal/src/driver.rs`
  - Estimated hours: 3

### Task Category 2: Telnet Terminal Driver Implementation

- [ ] **Task 2.1**: Create TelnetTerminal struct
  - Implementation notes: Wraps TcpStream, implements TerminalDriver trait
  - Files affected: `crates/impulse-terminal/src/telnet.rs`
  - Estimated hours: 3

- [ ] **Task 2.2**: Implement TerminalDriver for TelnetTerminal
  - Implementation notes: All trait methods with telnet-specific handling
  - Files affected: `crates/impulse-terminal/src/telnet.rs`
  - Estimated hours: 6

- [ ] **Task 2.3**: Handle telnet IAC (Interpret As Command) sequences
  - Implementation notes: Parse IAC WILL/WONT/DO/DONT, filter from user input
  - Files affected: `crates/impulse-terminal/src/telnet/iac.rs`
  - Estimated hours: 5

- [ ] **Task 2.4**: Implement NAWS (Negotiate About Window Size)
  - Implementation notes: Detect and track terminal dimensions via telnet options
  - Files affected: `crates/impulse-terminal/src/telnet/naws.rs`
  - Estimated hours: 4

- [ ] **Task 2.5**: Implement TTYPE (Terminal Type) negotiation
  - Implementation notes: Query client for terminal type (ANSI, VT100, etc.)
  - Files affected: `crates/impulse-terminal/src/telnet/ttype.rs`
  - Estimated hours: 4

### Task Category 3: ANSI Renderer Skeleton

- [ ] **Task 3.1**: Create ANSI sequence parser
  - Implementation notes: Parse ESC sequences, extract commands and parameters
  - Files affected: `crates/impulse-terminal/src/ansi/parser.rs`
  - Estimated hours: 6

- [ ] **Task 3.2**: Implement color rendering (SGR codes)
  - Implementation notes: 16-color ANSI (30-37, 40-47), reset, bold, underline
  - Files affected: `crates/impulse-terminal/src/ansi/colors.rs`
  - Estimated hours: 4

- [ ] **Task 3.3**: Implement cursor movement
  - Implementation notes: CUU, CUD, CUF, CUB (up, down, forward, back)
  - Files affected: `crates/impulse-terminal/src/ansi/cursor.rs`
  - Estimated hours: 4

- [ ] **Task 3.4**: Implement screen clearing
  - Implementation notes: Clear entire screen, clear to end of line, clear to end of screen
  - Files affected: `crates/impulse-terminal/src/ansi/screen.rs`
  - Estimated hours: 3

- [ ] **Task 3.5**: Add ANSI art file loader
  - Implementation notes: Load .ANS files, render with proper CP437 encoding
  - Files affected: `crates/impulse-terminal/src/ansi/loader.rs`
  - Estimated hours: 4

### Task Category 4: Capability Detection

- [ ] **Task 4.1**: Implement terminal type detection
  - Implementation notes: Query TERM environment (via telnet), detect ANSI support
  - Files affected: `crates/impulse-terminal/src/capabilities.rs`
  - Estimated hours: 3

- [ ] **Task 4.2**: Detect color support
  - Implementation notes: Check terminal type, fall back to monochrome if unsupported
  - Files affected: `crates/impulse-terminal/src/capabilities.rs`
  - Estimated hours: 2

- [ ] **Task 4.3**: Detect screen dimensions
  - Implementation notes: Use NAWS or fallback to 80x24
  - Files affected: `crates/impulse-terminal/src/capabilities.rs`
  - Estimated hours: 2

- [ ] **Task 4.4**: Create capability flags struct
  - Implementation notes: Store detected capabilities for session use
  - Files affected: `crates/impulse-terminal/src/capabilities.rs`
  - Estimated hours: 2

### Task Category 5: Testing and Validation

- [ ] **Task 5.1**: Test with real telnet clients
  - Implementation notes: PuTTY, SecureCRT, standard telnet, SyncTERM
  - Files affected: Manual testing, document results
  - Estimated hours: 4

- [ ] **Task 5.2**: Test ANSI art rendering
  - Implementation notes: Load sample .ANS files, verify rendering
  - Files affected: Integration tests, sample files
  - Estimated hours: 3

- [ ] **Task 5.3**: Test terminal detection
  - Implementation notes: Various terminal types, verify correct detection
  - Files affected: `tests/detection_test.rs`
  - Estimated hours: 3

- [ ] **Task 5.4**: Write unit tests for ANSI parser
  - Implementation notes: Test all supported sequences, malformed input
  - Files affected: `tests/ansi_parser_test.rs`
  - Estimated hours: 5

---

## Technical Details

### Architecture Considerations

- Abstract terminal I/O for future SSH, web terminal support
- Use buffering to minimize write syscalls
- Handle partial ANSI sequences across read boundaries
- Support CP437 encoding for authentic BBS experience

### Dependencies

**Crate-Level Dependencies:**
```toml
[dependencies]
tokio = { workspace = true }
anyhow = { workspace = true }
thiserror = { workspace = true }
bytes = "1.5"
encoding_rs = "0.8"  # For CP437 support

[dev-dependencies]
tokio-test = "0.4"
```

### Code Patterns

**TerminalDriver Trait:**
```rust
use async_trait::async_trait;

#[async_trait]
pub trait TerminalDriver: Send + Sync {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
    async fn write(&mut self, data: &[u8]) -> Result<()>;
    async fn flush(&mut self) -> Result<()>;

    async fn clear_screen(&mut self) -> Result<()>;
    async fn move_cursor(&mut self, x: u16, y: u16) -> Result<()>;
    async fn hide_cursor(&mut self) -> Result<()>;
    async fn show_cursor(&mut self) -> Result<()>;

    fn capabilities(&self) -> &Capabilities;
    fn dimensions(&self) -> (u16, u16);
}
```

**ANSI Sequence Rendering:**
```rust
pub struct AnsiRenderer {
    capabilities: Capabilities,
}

impl AnsiRenderer {
    pub async fn render<T: TerminalDriver>(
        &self,
        terminal: &mut T,
        ansi_text: &str,
    ) -> Result<()> {
        let sequences = self.parse_ansi(ansi_text)?;

        for seq in sequences {
            match seq {
                AnsiSequence::Text(text) => {
                    terminal.write(text.as_bytes()).await?;
                }
                AnsiSequence::Color(color) => {
                    if self.capabilities.supports_color {
                        terminal.write(&color.to_escape_code()).await?;
                    }
                }
                AnsiSequence::CursorMove { x, y } => {
                    terminal.move_cursor(x, y).await?;
                }
                // ... other sequences
            }
        }

        terminal.flush().await?;
        Ok(())
    }
}
```

**Telnet IAC Handling:**
```rust
pub struct IacHandler {
    state: IacState,
}

impl IacHandler {
    pub fn process(&mut self, byte: u8) -> Option<IacCommand> {
        match self.state {
            IacState::Normal => {
                if byte == IAC {
                    self.state = IacState::Iac;
                    None
                } else {
                    Some(IacCommand::Data(byte))
                }
            }
            IacState::Iac => {
                match byte {
                    WILL | WONT | DO | DONT => {
                        self.state = IacState::Option(byte);
                        None
                    }
                    _ => {
                        self.state = IacState::Normal;
                        Some(IacCommand::Command(byte))
                    }
                }
            }
            // ... other states
        }
    }
}
```

---

## Dependencies

### Upstream Dependencies
- **Sprint 06**: Session management provides connection streams

### Blocks Downstream
- **Sprint 08**: Integration testing needs terminal I/O
- **Sprint 10**: Menu system requires ANSI rendering
- **All UI features**: Terminal abstraction is foundation

---

## Acceptance Criteria

- [ ] Can display colored text via telnet
- [ ] Basic ANSI art files render correctly
- [ ] Terminal type detection works
- [ ] IAC sequences handled properly
- [ ] Screen dimensions detected (NAWS or fallback)
- [ ] CP437 encoding supported
- [ ] All TerminalDriver methods implemented for telnet

---

## Testing Requirements

### Unit Tests
- [ ] ANSI parser handles all standard sequences
- [ ] IAC state machine processes commands correctly
- [ ] Color codes generate correct escape sequences
- [ ] Cursor movement commands work

### Integration Tests
- [ ] Load and render ANSI art files
- [ ] Terminal capability detection with mock clients
- [ ] Full terminal session (connect → render → disconnect)

### Manual Testing
- [ ] Test with PuTTY (Windows)
- [ ] Test with standard telnet (Linux)
- [ ] Test with SyncTERM (DOS BBS terminal)
- [ ] Test with modern terminal emulators (iTerm2, Windows Terminal)

---

## Notes and Decisions

### Design Decisions
*To be populated during sprint*

**Proposed Decisions:**
- Support CP437 for authentic BBS experience
- Use async trait for all I/O operations
- Buffer writes for performance
- Default to 80x24 if dimensions unknown

### Lessons Learned
*To be populated during sprint*

### Risks and Mitigations
- **Risk**: Telnet IAC handling complex and error-prone
- **Mitigation**: Comprehensive test suite, reference existing implementations
- **Risk**: ANSI parsing may have edge cases
- **Mitigation**: Test with large corpus of ANSI art, fuzz testing
- **Risk**: Terminal detection may fail for some clients
- **Mitigation**: Sensible fallbacks, allow manual override in config

---

## Progress Log

### Week 1
- *Date*: Progress notes will be added here as sprint progresses

### Week 2
- *Date*: Progress notes will be added here as sprint progresses

### Week 3
- *Date*: Progress notes will be added here as sprint progresses

### Sprint Completion
- **Completed**: TBD
- **Velocity**: TBD
- **Burndown**: TBD
