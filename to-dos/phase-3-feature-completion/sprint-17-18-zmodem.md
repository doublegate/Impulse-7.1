# Sprint 17-18: Zmodem Protocol Implementation

**Phase:** Phase 3 - Feature Completion
**Duration:** 6 weeks (Double Sprint)
**Sprint Dates:** TBD
**Status:** Not Started

---

## Sprint Overview

Sprints 17-18 implement the complete Zmodem file transfer protocol, including download, upload, and crash recovery capabilities. This double sprint provides the primary file transfer mechanism for the BBS, enabling reliable file transfers with resume capability.

**Context:** This is the first sprint of Phase 3 (Feature Completion). Zmodem is the most important file transfer protocol for BBS systems.

**Expected Outcomes:** By the end of this double sprint, users will be able to download and upload files using the Zmodem protocol with full crash recovery and resume support.

---

## Objectives

- [ ] Implement complete Zmodem protocol (ZRQINIT, ZRINIT, ZFILE, ZDATA, etc.)
- [ ] Support file download via Zmodem
- [ ] Support file upload via Zmodem
- [ ] Enable crash recovery and resume functionality
- [ ] Test with multiple Zmodem clients

---

## Deliverables

| Deliverable | Type | Acceptance Criteria |
|-------------|------|---------------------|
| `impulse-protocol` crate with Zmodem | Code | Complete Zmodem implementation |
| Download functionality | Code | Send files reliably to clients |
| Upload functionality | Code | Receive files reliably from clients |
| Crash recovery | Code | Resume interrupted transfers |
| Protocol test suite | Tests | Compatible with standard clients |

---

## Detailed Tasks

### Task Category 1: Zmodem Protocol Foundation

- [ ] **Task 1.1**: Implement Zmodem frame structure
  - Implementation notes: ZDLE escaping, frame types (hex/binary)
  - Files affected: `crates/impulse-protocol/src/zmodem/frame.rs`
  - Estimated hours: 8

- [ ] **Task 1.2**: Implement CRC-16 checksum
  - Implementation notes: CRC-16/XMODEM polynomial
  - Files affected: `crates/impulse-protocol/src/zmodem/crc16.rs`
  - Estimated hours: 4

- [ ] **Task 1.3**: Implement CRC-32 checksum
  - Implementation notes: CRC-32/ZMODEM polynomial
  - Files affected: `crates/impulse-protocol/src/zmodem/crc32.rs`
  - Estimated hours: 4

- [ ] **Task 1.4**: Implement ZDLE encoding/decoding
  - Implementation notes: Escape special characters, handle control chars
  - Files affected: `crates/impulse-protocol/src/zmodem/escape.rs`
  - Estimated hours: 6

- [ ] **Task 1.5**: Implement frame parser
  - Implementation notes: Parse hex/binary frames, validate checksums
  - Files affected: `crates/impulse-protocol/src/zmodem/parser.rs`
  - Estimated hours: 8

### Task Category 2: Zmodem Handshake and Negotiation

- [ ] **Task 2.1**: Implement ZRQINIT (request init)
  - Implementation notes: Sender initiates session
  - Files affected: `crates/impulse-protocol/src/zmodem/init.rs`
  - Estimated hours: 4

- [ ] **Task 2.2**: Implement ZRINIT (receiver init)
  - Implementation notes: Receiver capabilities, buffer size
  - Files affected: `crates/impulse-protocol/src/zmodem/init.rs`
  - Estimated hours: 4

- [ ] **Task 2.3**: Negotiate protocol parameters
  - Implementation notes: Escape mode, buffer size, CRC type
  - Files affected: `crates/impulse-protocol/src/zmodem/negotiate.rs`
  - Estimated hours: 5

- [ ] **Task 2.4**: Handle ZFILE frame
  - Implementation notes: Filename, file size, modification time, transfer mode
  - Files affected: `crates/impulse-protocol/src/zmodem/file.rs`
  - Estimated hours: 6

### Task Category 3: File Download (Send)

- [ ] **Task 3.1**: Implement send file handshake
  - Implementation notes: Send ZFILE, wait for ZRPOS
  - Files affected: `crates/impulse-protocol/src/zmodem/send.rs`
  - Estimated hours: 5

- [ ] **Task 3.2**: Implement ZDATA transmission
  - Implementation notes: Send file data in blocks, ZCRC subframes
  - Files affected: `crates/impulse-protocol/src/zmodem/send.rs`
  - Estimated hours: 8

- [ ] **Task 3.3**: Handle retransmission requests
  - Implementation notes: Process ZRPOS, resend from position
  - Files affected: `crates/impulse-protocol/src/zmodem/send.rs`
  - Estimated hours: 6

- [ ] **Task 3.4**: Implement EOF handling
  - Implementation notes: Send ZEOF, wait for ZRINIT for next file
  - Files affected: `crates/impulse-protocol/src/zmodem/send.rs`
  - Estimated hours: 4

- [ ] **Task 3.5**: Implement batch file sends
  - Implementation notes: Send multiple files in one session
  - Files affected: `crates/impulse-protocol/src/zmodem/batch.rs`
  - Estimated hours: 6

### Task Category 4: File Upload (Receive)

- [ ] **Task 4.1**: Implement receive file handshake
  - Implementation notes: Receive ZFILE, send ZRPOS
  - Files affected: `crates/impulse-protocol/src/zmodem/receive.rs`
  - Estimated hours: 5

- [ ] **Task 4.2**: Implement ZDATA reception
  - Implementation notes: Receive file data, validate CRCs
  - Files affected: `crates/impulse-protocol/src/zmodem/receive.rs`
  - Estimated hours: 8

- [ ] **Task 4.3**: Request retransmission on errors
  - Implementation notes: Send ZRPOS for bad data
  - Files affected: `crates/impulse-protocol/src/zmodem/receive.rs`
  - Estimated hours: 6

- [ ] **Task 4.4**: Handle EOF and completion
  - Implementation notes: Receive ZEOF, send ZRINIT
  - Files affected: `crates/impulse-protocol/src/zmodem/receive.rs`
  - Estimated hours: 4

- [ ] **Task 4.5**: Implement batch file receives
  - Implementation notes: Receive multiple files in one session
  - Files affected: `crates/impulse-protocol/src/zmodem/batch.rs`
  - Estimated hours: 6

### Task Category 5: Crash Recovery and Resume

- [ ] **Task 5.1**: Implement ZRPOS (position)
  - Implementation notes: Resume from specific byte position
  - Files affected: `crates/impulse-protocol/src/zmodem/resume.rs`
  - Estimated hours: 6

- [ ] **Task 5.2**: Handle partial file uploads
  - Implementation notes: Detect existing partial file, resume
  - Files affected: `crates/impulse-protocol/src/zmodem/resume.rs`
  - Estimated hours: 5

- [ ] **Task 5.3**: Handle partial file downloads
  - Implementation notes: Request resume from last received position
  - Files affected: `crates/impulse-protocol/src/zmodem/resume.rs`
  - Estimated hours: 5

- [ ] **Task 5.4**: Verify file integrity on resume
  - Implementation notes: CRC check up to resume point
  - Files affected: `crates/impulse-protocol/src/zmodem/verify.rs`
  - Estimated hours: 4

### Task Category 6: Integration and UI

- [ ] **Task 6.1**: Integrate download with file browser
  - Implementation notes: User selects file → initiate Zmodem send
  - Files affected: `crates/impulse-files/src/download.rs`
  - Estimated hours: 4

- [ ] **Task 6.2**: Integrate upload with file areas
  - Implementation notes: User uploads → receive via Zmodem → trigger FILE_ID.DIZ extraction
  - Files affected: `crates/impulse-files/src/upload/protocol.rs`
  - Estimated hours: 4

- [ ] **Task 6.3**: Display transfer progress bar
  - Implementation notes: ANSI progress bar, percentage, ETA, speed
  - Files affected: `crates/impulse-protocol/src/ui/progress.rs`
  - Estimated hours: 6

- [ ] **Task 6.4**: Update download statistics
  - Implementation notes: Increment download count, update bytes downloaded
  - Files affected: `crates/impulse-files/src/stats.rs`
  - Estimated hours: 3

- [ ] **Task 6.5**: Update upload statistics
  - Implementation notes: Increment upload count, update bytes uploaded
  - Files affected: `crates/impulse-files/src/stats.rs`
  - Estimated hours: 3

### Task Category 7: Testing

- [ ] **Task 7.1**: Unit tests for frame encoding/decoding
  - Implementation notes: Test all frame types, ZDLE escaping
  - Files affected: `tests/zmodem_frame_test.rs`
  - Estimated hours: 6

- [ ] **Task 7.2**: Unit tests for CRC calculations
  - Implementation notes: Test CRC-16 and CRC-32 with known values
  - Files affected: `tests/zmodem_crc_test.rs`
  - Estimated hours: 3

- [ ] **Task 7.3**: Test with minicom client
  - Implementation notes: Download and upload files using minicom
  - Files affected: Manual testing documentation
  - Estimated hours: 4

- [ ] **Task 7.4**: Test with PuTTY client
  - Implementation notes: Windows Zmodem testing
  - Files affected: Manual testing documentation
  - Estimated hours: 4

- [ ] **Task 7.5**: Test with SecureCRT client
  - Implementation notes: Commercial client testing
  - Files affected: Manual testing documentation
  - Estimated hours: 4

- [ ] **Task 7.6**: Test large file transfers (>100MB)
  - Implementation notes: Verify performance and reliability
  - Files affected: Performance test report
  - Estimated hours: 4

- [ ] **Task 7.7**: Test crash recovery
  - Implementation notes: Interrupt transfer, resume, verify completion
  - Files affected: `tests/zmodem_resume_test.rs`
  - Estimated hours: 6

- [ ] **Task 7.8**: Test batch transfers
  - Implementation notes: Send/receive multiple files
  - Files affected: `tests/zmodem_batch_test.rs`
  - Estimated hours: 4

---

## Technical Details

### Architecture Considerations

- Implement state machine for protocol flow
- Use async I/O for efficient transfers
- Buffer data appropriately for performance
- Handle timeouts and retransmissions gracefully

### Dependencies

**Crate-Level Dependencies:**
```toml
[dependencies]
tokio = { workspace = true }
crc = "3.0"
bytes = "1.5"
tracing = "0.1"
```

### Code Patterns

**Zmodem State Machine:**
```rust
pub enum ZmodemState {
    Idle,
    Handshake,
    FileHeader,
    DataTransfer,
    EOF,
    Complete,
}

pub struct ZmodemSession {
    state: ZmodemState,
    stream: Box<dyn AsyncRead + AsyncWrite + Unpin>,
    current_file: Option<FileInfo>,
    position: u64,
    buffer_size: usize,
}

impl ZmodemSession {
    pub async fn send_file(&mut self, path: &Path) -> Result<()> {
        // Send ZFILE
        self.send_frame(ZFrame::ZFile { ... }).await?;
        self.state = ZmodemState::FileHeader;

        // Wait for ZRPOS
        let frame = self.recv_frame().await?;
        match frame {
            ZFrame::ZRPos { position } => {
                self.position = position;
                self.state = ZmodemState::DataTransfer;
            }
            _ => return Err(anyhow!("Unexpected frame"))
        }

        // Send data
        self.send_data(path).await?;

        // Send EOF
        self.send_frame(ZFrame::ZEof { position }).await?;
        self.state = ZmodemState::Complete;

        Ok(())
    }
}
```

**Progress Display:**
```rust
pub struct TransferProgress {
    pub filename: String,
    pub total_size: u64,
    pub transferred: u64,
    pub start_time: Instant,
}

impl TransferProgress {
    pub fn render(&self) -> String {
        let percent = (self.transferred as f64 / self.total_size as f64 * 100.0) as u32;
        let elapsed = self.start_time.elapsed().as_secs_f64();
        let speed = self.transferred as f64 / elapsed;
        let eta = ((self.total_size - self.transferred) as f64 / speed) as u64;

        format!(
            "Transferring: {} [{:>3}%] {} / {} | Speed: {}/s | ETA: {}",
            self.filename,
            percent,
            humansize(self.transferred),
            humansize(self.total_size),
            humansize(speed as u64),
            format_duration(eta)
        )
    }
}
```

---

## Dependencies

### Upstream Dependencies
- **Sprint 14**: File upload infrastructure
- **Sprint 13**: File browsing for download
- **Sprint 07**: Terminal I/O for protocol communication

### Blocks Downstream
- **Sprint 19**: Other protocols may build on this infrastructure
- **User downloads/uploads**: Primary transfer mechanism

---

## Acceptance Criteria

- [ ] Zmodem downloads work reliably
- [ ] Zmodem uploads work reliably
- [ ] Resume works after connection interruption
- [ ] Compatible with minicom, PuTTY, SecureCRT
- [ ] Large files (>100MB) transfer successfully
- [ ] Batch transfers work correctly
- [ ] Transfer progress displays accurately
- [ ] CRC errors are detected and corrected
- [ ] Timeouts are handled gracefully

---

## Testing Requirements

### Unit Tests
- [ ] Frame encoding/decoding
- [ ] CRC calculations
- [ ] ZDLE escaping
- [ ] State machine transitions

### Integration Tests
- [ ] Complete download flow
- [ ] Complete upload flow
- [ ] Crash recovery
- [ ] Batch transfers

### Compatibility Tests
- [ ] minicom (Linux)
- [ ] PuTTY (Windows)
- [ ] SecureCRT (Cross-platform)
- [ ] Tera Term (Windows)
- [ ] ZOC (macOS)

### Performance Tests
- [ ] Large file transfers (1GB+)
- [ ] Transfer speed benchmarks
- [ ] Memory usage during transfers

---

## Notes and Decisions

### Design Decisions
*To be populated during sprint*

**Proposed Decisions:**
- Use binary headers for efficiency (not hex)
- Default buffer size: 8KB
- Use CRC-32 for reliability
- Escape mode: minimal (ESC only)
- Timeout: 30 seconds per frame

### Lessons Learned
*To be populated during sprint*

### Risks and Mitigations
- **Risk**: Zmodem specification may be ambiguous
- **Mitigation**: Study existing implementations (lrzsz, sexyz), extensive testing
- **Risk**: Performance may be poor on slow connections
- **Mitigation**: Optimize buffering, minimize overhead
- **Risk**: Resume may fail with corrupted partial files
- **Mitigation**: CRC verification before resume, option to restart

---

## Progress Log

### Week 1-2
- *Date*: Progress notes will be added here as sprint progresses

### Week 3-4
- *Date*: Progress notes will be added here as sprint progresses

### Week 5-6
- *Date*: Progress notes will be added here as sprint progresses

### Sprint Completion
- **Completed**: TBD
- **Velocity**: TBD
- **Burndown**: TBD
