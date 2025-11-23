# Sprint 19: Additional File Transfer Protocols

**Phase:** Phase 3 - Feature Completion
**Duration:** 3 weeks
**Sprint Dates:** TBD
**Status:** Not Started

---

## Sprint Overview

Sprint 19 adds Xmodem and Ymodem protocol support, providing users with alternative file transfer options. These simpler protocols offer compatibility with older clients and lighter-weight transfer mechanisms.

**Context:** Sprint 3 of Phase 3. Complements Zmodem with additional protocol options.

**Expected Outcomes:** Users can choose from Zmodem, Xmodem, or Ymodem for file transfers.

---

## Objectives

- [ ] Implement Xmodem protocol (128-byte and 1K blocks)
- [ ] Implement Ymodem protocol (batch transfers)
- [ ] Add protocol selection menu
- [ ] Ensure interoperability with standard clients

---

## Deliverables

| Deliverable | Type | Acceptance Criteria |
|-------------|------|---------------------|
| Xmodem implementation | Code | Working Xmodem and Xmodem-1K |
| Ymodem implementation | Code | Batch file transfers with metadata |
| Protocol selection UI | UI | Users can choose protocol |
| Protocol test suite | Tests | Compatible with clients |

---

## Detailed Tasks

### Task Category 1: Xmodem Implementation

- [ ] **Task 1.1**: Implement Xmodem (128-byte blocks)
  - Files affected: `crates/impulse-protocol/src/xmodem/basic.rs`
  - Estimated hours: 6

- [ ] **Task 1.2**: Implement Xmodem-1K (1024-byte blocks)
  - Files affected: `crates/impulse-protocol/src/xmodem/oneK.rs`
  - Estimated hours: 5

- [ ] **Task 1.3**: Implement Xmodem-CRC
  - Files affected: `crates/impulse-protocol/src/xmodem/crc.rs`
  - Estimated hours: 4

- [ ] **Task 1.4**: Implement checksum validation
  - Files affected: `crates/impulse-protocol/src/xmodem/checksum.rs`
  - Estimated hours: 3

### Task Category 2: Ymodem Implementation

- [ ] **Task 2.1**: Implement Ymodem batch transfer
  - Files affected: `crates/impulse-protocol/src/ymodem/batch.rs`
  - Estimated hours: 8

- [ ] **Task 2.2**: Handle file metadata (size, date)
  - Files affected: `crates/impulse-protocol/src/ymodem/metadata.rs`
  - Estimated hours: 4

- [ ] **Task 2.3**: Implement Ymodem-G (streaming)
  - Files affected: `crates/impulse-protocol/src/ymodem/streaming.rs`
  - Estimated hours: 6

### Task Category 3: Protocol Selection

- [ ] **Task 3.1**: Create protocol selection menu
  - Files affected: `crates/impulse-protocol/src/ui/selection.rs`
  - Estimated hours: 4

- [ ] **Task 3.2**: Auto-detect protocol if possible
  - Files affected: `crates/impulse-protocol/src/detection.rs`
  - Estimated hours: 5

- [ ] **Task 3.3**: Store user protocol preference
  - Files affected: `crates/impulse-user/src/settings.rs`
  - Estimated hours: 2

### Task Category 4: Testing

- [ ] **Task 4.1**: Test Xmodem with various clients
  - Estimated hours: 6

- [ ] **Task 4.2**: Test Ymodem batch transfers
  - Estimated hours: 5

- [ ] **Task 4.3**: Interoperability testing
  - Estimated hours: 8

---

## Technical Details

### Architecture Considerations

- Implement protocols as async state machines for non-blocking operation
- Use Tokio channels for communication between protocol handler and I/O layer
- CRC calculations must be efficient (pre-computed tables)
- Handle noisy connections with appropriate timeout and retry logic
- Support both sender and receiver roles

### Dependencies

**Crate-Level Dependencies:**
```toml
[dependencies]
tokio = { workspace = true }
crc = "3.0"  # For CRC16 calculations
bytes = "1.5"
thiserror = { workspace = true }

[dev-dependencies]
tokio-test = "0.4"
```

**Pascal Units Being Converted:**
- XMODEM.PAS (Xmodem protocol implementation)
- YMODEM.PAS (Ymodem/Ymodem-G implementation)
- PROTOCOL.PAS (Protocol selection and dispatch)

### Code Examples

**Xmodem Protocol Implementation:**
```rust
use bytes::{Buf, BufMut, BytesMut};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::time::{timeout, Duration};

const SOH: u8 = 0x01;  // Start of 128-byte block
const STX: u8 = 0x02;  // Start of 1K block
const EOT: u8 = 0x04;  // End of transmission
const ACK: u8 = 0x06;  // Acknowledge
const NAK: u8 = 0x15;  // Not acknowledge
const CAN: u8 = 0x18;  // Cancel
const CRC_MODE: u8 = b'C';

#[derive(Debug, thiserror::Error)]
pub enum XmodemError {
    #[error("Transfer cancelled by remote")]
    Cancelled,
    #[error("Too many errors, aborting")]
    TooManyErrors,
    #[error("Timeout waiting for response")]
    Timeout,
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("CRC mismatch")]
    CrcMismatch,
}

pub struct XmodemSender<W> {
    writer: W,
    use_crc: bool,
    use_1k_blocks: bool,
}

impl<W: AsyncWrite + Unpin> XmodemSender<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            use_crc: false,
            use_1k_blocks: false,
        }
    }

    pub async fn send_file(
        &mut self,
        data: &[u8],
    ) -> Result<usize, XmodemError> {
        // Wait for receiver to request transfer (NAK or 'C' for CRC)
        let start_char = self.wait_for_start().await?;
        self.use_crc = start_char == CRC_MODE;

        let block_size = if self.use_1k_blocks { 1024 } else { 128 };
        let mut block_num: u8 = 1;
        let mut total_sent = 0;

        for chunk in data.chunks(block_size) {
            let mut block = BytesMut::with_capacity(block_size + 5);

            // Header: SOH/STX, block number, complement
            block.put_u8(if block_size == 1024 { STX } else { SOH });
            block.put_u8(block_num);
            block.put_u8(!block_num);

            // Data (pad with SUB if needed)
            block.put_slice(chunk);
            if chunk.len() < block_size {
                block.put_bytes(0x1A, block_size - chunk.len());
            }

            // Checksum or CRC
            if self.use_crc {
                let crc = calculate_crc16(&block[3..]);
                block.put_u16(crc);
            } else {
                let checksum = calculate_checksum(&block[3..]);
                block.put_u8(checksum);
            }

            // Send block and wait for ACK
            self.send_block_with_retry(&block, 10).await?;

            block_num = block_num.wrapping_add(1);
            total_sent += chunk.len();
        }

        // Send EOT
        self.send_eot().await?;
        Ok(total_sent)
    }

    async fn wait_for_start(&mut self) -> Result<u8, XmodemError> {
        // Implementation details...
        Ok(CRC_MODE)
    }

    async fn send_block_with_retry(
        &mut self,
        block: &[u8],
        max_retries: usize,
    ) -> Result<(), XmodemError> {
        // Implementation details...
        Ok(())
    }

    async fn send_eot(&mut self) -> Result<(), XmodemError> {
        // Send EOT, expect ACK
        Ok(())
    }
}

fn calculate_crc16(data: &[u8]) -> u16 {
    let mut crc: u16 = 0;
    for &byte in data {
        crc ^= (byte as u16) << 8;
        for _ in 0..8 {
            if crc & 0x8000 != 0 {
                crc = (crc << 1) ^ 0x1021;
            } else {
                crc <<= 1;
            }
        }
    }
    crc
}

fn calculate_checksum(data: &[u8]) -> u8 {
    data.iter().fold(0u8, |acc, &b| acc.wrapping_add(b))
}
```

**Ymodem Batch Transfer:**
```rust
pub struct YmodemSender<W> {
    writer: W,
    use_crc: bool,
}

impl<W: AsyncWrite + Unpin> YmodemSender<W> {
    pub async fn send_batch(
        &mut self,
        files: &[(String, Vec<u8>)],
    ) -> Result<usize, XmodemError> {
        let mut total_sent = 0;

        for (filename, data) in files {
            // Send file header block (block 0)
            self.send_file_header(filename, data.len()).await?;

            // Send file data using Xmodem
            let mut xmodem = XmodemSender::new(&mut self.writer);
            total_sent += xmodem.send_file(data).await?;
        }

        // Send null header to indicate batch end
        self.send_null_header().await?;
        Ok(total_sent)
    }

    async fn send_file_header(
        &mut self,
        filename: &str,
        size: usize,
    ) -> Result<(), XmodemError> {
        let mut header = BytesMut::with_capacity(128);

        // Filename (null-terminated)
        header.put_slice(filename.as_bytes());
        header.put_u8(0);

        // File size and modification time
        let meta = format!("{} 0", size);
        header.put_slice(meta.as_bytes());

        // Pad to 128 bytes
        if header.len() < 128 {
            header.put_bytes(0, 128 - header.len());
        }

        // Send as block 0
        self.send_block_0(&header).await
    }

    async fn send_block_0(&mut self, data: &[u8]) -> Result<(), XmodemError> {
        // Implementation details...
        Ok(())
    }

    async fn send_null_header(&mut self) -> Result<(), XmodemError> {
        // Send empty block 0 to signal end of batch
        let null_block = vec![0u8; 128];
        self.send_block_0(&null_block).await
    }
}
```

**Protocol Selection UI:**
```rust
pub enum FileProtocol {
    Zmodem,
    YmodemG,
    Ymodem,
    Xmodem1K,
    XmodemCrc,
    XmodemChecksum,
}

pub struct ProtocolSelector {
    user_preference: Option<FileProtocol>,
}

impl ProtocolSelector {
    pub async fn select_protocol(
        &self,
        terminal: &TerminalDriver,
    ) -> anyhow::Result<FileProtocol> {
        terminal.write_line("\n=== File Transfer Protocol ===\n").await?;
        terminal.write_line("(Z) Zmodem (recommended)").await?;
        terminal.write_line("(G) Ymodem-G (fast, reliable connection)").await?;
        terminal.write_line("(Y) Ymodem batch").await?;
        terminal.write_line("(1) Xmodem-1K").await?;
        terminal.write_line("(C) Xmodem-CRC").await?;
        terminal.write_line("(X) Xmodem (checksum)").await?;
        terminal.write("\nProtocol: ").await?;

        let choice = terminal.read_key().await?;

        match choice.to_ascii_uppercase() {
            b'Z' => Ok(FileProtocol::Zmodem),
            b'G' => Ok(FileProtocol::YmodemG),
            b'Y' => Ok(FileProtocol::Ymodem),
            b'1' => Ok(FileProtocol::Xmodem1K),
            b'C' => Ok(FileProtocol::XmodemCrc),
            b'X' => Ok(FileProtocol::XmodemChecksum),
            _ => {
                terminal.write_line("\nInvalid selection, using Zmodem.").await?;
                Ok(FileProtocol::Zmodem)
            }
        }
    }
}
```

---

## Dependencies

### Upstream Dependencies
- **Sprint 17-18**: Zmodem implementation provides base protocol infrastructure
- **Sprint 14**: File upload system for received files
- **Sprint 13**: File areas for download selection

### Blocks Downstream
- **Sprint 24**: Integration testing needs all protocols working

---

## Acceptance Criteria

- [ ] Xmodem (128-byte) works correctly
- [ ] Xmodem-1K works correctly
- [ ] Xmodem-CRC works correctly
- [ ] Ymodem batch transfers work
- [ ] Ymodem-G (streaming) works
- [ ] Users can select preferred protocol
- [ ] Batch transfers work with Ymodem
- [ ] Compatible with standard clients (Tera Term, SyncTerm, etc.)
- [ ] CRC and checksum calculations verified
- [ ] Error recovery handles noisy connections
- [ ] Timeout and retry logic prevents hangs

---

## Testing Requirements

### Unit Tests
- [ ] CRC16 calculation matches reference implementation
- [ ] Checksum calculation correct
- [ ] Block formatting (SOH/STX headers, padding)
- [ ] Block number wrapping (0-255)
- [ ] File header metadata encoding (Ymodem)

### Integration Tests
- [ ] Complete file transfer (sender → receiver)
- [ ] Batch transfer with multiple files (Ymodem)
- [ ] Error injection and recovery
- [ ] Timeout handling
- [ ] Cancel mid-transfer

### Interoperability Tests
- [ ] Test with Tera Term Pro (Windows)
- [ ] Test with SyncTerm (multi-platform)
- [ ] Test with minicom (Linux)
- [ ] Test with ZOC (macOS/Windows)
- [ ] Verify binary file integrity (compare SHA-256)

### Performance Tests
- [ ] Transfer 1MB file < 60 seconds (Xmodem-1K)
- [ ] Transfer 1MB file < 30 seconds (Ymodem-G)
- [ ] Batch transfer 10 files efficient
- [ ] CPU usage < 10% during transfer

---

## Notes and Decisions

### Design Decisions
*To be populated during sprint*

**Proposed Decisions:**
- Default to CRC mode for Xmodem (more reliable than checksum)
- Recommend Ymodem-G for fast, reliable connections
- Fall back to Xmodem-CRC for maximum compatibility
- Store user's last protocol choice as preference
- Support both sender and receiver roles for all protocols

### Lessons Learned
*To be populated during sprint*

### Risks and Mitigations
- **Risk**: Xmodem/Ymodem are slower than Zmodem
- **Mitigation**: Clearly document speed differences; recommend Zmodem by default
- **Risk**: Some clients may not support all variants
- **Mitigation**: Provide fallback options; detect client capabilities if possible
- **Risk**: Binary file corruption on some terminals
- **Mitigation**: Always use CRC when available; verify file integrity post-transfer
- **Risk**: Noisy connections cause frequent retries
- **Mitigation**: Configurable timeout and retry limits; suggest switching protocols

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
