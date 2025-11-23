# Sprint 22: Advanced Message Base Features

**Phase:** Phase 3 - Feature Completion
**Duration:** 3 weeks
**Sprint Dates:** TBD
**Status:** Not Started

---

## Sprint Overview

Sprint 22 implements advanced messaging features including QWK offline reader support, message import/export, and FidoNet-style addressing for network message routing preparation.

**Context:** Sprint 6 of Phase 3. Enhances messaging with offline reading and network capabilities.

**Expected Outcomes:** Users can download/upload QWK packets and messages support network addressing.

---

## Objectives

- [ ] Implement QWK offline reader packet support
- [ ] Add message import/export functionality
- [ ] Support FidoNet-style addressing
- [ ] Prepare for message networking

---

## Deliverables

| Deliverable | Type | Acceptance Criteria |
|-------------|------|---------------------|
| QWK packet generation | Code | Generate QWK packets for download |
| QWK reply parsing | Code | Process uploaded reply packets |
| Import/export tools | Code | Import/export messages to text |
| Network addressing | Code | zone:net/node FidoNet format |

---

## Detailed Tasks

### Task Category 1: QWK Support

- [ ] **Task 1.1**: Generate QWK packets
  - Files affected: `crates/impulse-message/src/qwk/generate.rs`
  - Estimated hours: 8

- [ ] **Task 1.2**: Parse QWK reply packets
  - Files affected: `crates/impulse-message/src/qwk/parse.rs`
  - Estimated hours: 8

- [ ] **Task 1.3**: ZIP compression
  - Files affected: `crates/impulse-message/src/qwk/compress.rs`
  - Estimated hours: 4

### Task Category 2: Import/Export

- [ ] **Task 2.1**: Export to text files
  - Files affected: `crates/impulse-message/src/export.rs`
  - Estimated hours: 5

- [ ] **Task 2.2**: Import from standard formats
  - Files affected: `crates/impulse-message/src/import.rs`
  - Estimated hours: 6

### Task Category 3: Network Addressing

- [ ] **Task 3.1**: Implement FidoNet addressing
  - Files affected: `crates/impulse-message/src/addressing/fidonet.rs`
  - Estimated hours: 6

- [ ] **Task 3.2**: Route messages (preparation)
  - Files affected: `crates/impulse-message/src/routing/mod.rs`
  - Estimated hours: 5

### Task Category 4: Testing

- [ ] **Task 4.1**: Test QWK download/upload
  - Estimated hours: 6

- [ ] **Task 4.2**: Test import/export
  - Estimated hours: 4

---

## Acceptance Criteria

- [ ] QWK packets work with offline readers
- [ ] Reply packets process correctly
- [ ] Messages can be exported for archival
- [ ] Network addressing is supported

---

## Technical Details

### Architecture Considerations

- QWK packets are ZIP archives containing structured message files
- MESSAGES.DAT format: 128-byte header + variable-length message blocks
- CONTROL.DAT contains conference configuration
- DOOR.ID identifies the BBS system
- QWK reply packets (REP extension) use similar structure
- FidoNet addressing: zone:net/node.point format
- Message import/export supports multiple formats (text, JSON, CSV)
- Prepare routing infrastructure for future networking support

### Dependencies

**Crate-Level Dependencies:**
```toml
[dependencies]
tokio = { workspace = true }
serde = { workspace = true }
zip = "0.6"  # QWK packet compression
chrono = { workspace = true }
binrw = "0.13"  # Binary format parsing

[dev-dependencies]
tempfile = "3.8"
```

**Pascal Units Being Converted:**
- QWK.PAS (QWK packet generation and parsing)
- MSGEXP.PAS (Message export functionality)
- MSGIMP.PAS (Message import functionality)
- NETADDR.PAS (Network addressing system)

### Code Examples

**QWK Packet Generation:**
```rust
use binrw::{BinRead, BinWrite};
use std::io::Write;
use zip::write::{FileOptions, ZipWriter};

#[derive(Debug, BinWrite)]
pub struct QwkMessageHeader {
    pub status: u8,            // Message status flags
    pub msg_number: [u8; 7],   // Message number (ASCII)
    pub date: [u8; 8],         // MM-DD-YY
    pub time: [u8; 5],         // HH:MM
    pub to: [u8; 25],          // To (padded with spaces)
    pub from: [u8; 25],        // From (padded with spaces)
    pub subject: [u8; 25],     // Subject
    pub password: [u8; 12],    // Password (usually blank)
    pub reply_to: [u8; 8],     // Message being replied to
    pub num_blocks: [u8; 6],   // Number of 128-byte blocks
    pub alive: u8,             // Active flag (0xE1 or 0xE2)
    pub conf_number: u16,      // Conference number
    pub tag_line: u16,         // Logical message number
}

impl QwkMessageHeader {
    pub fn new(msg: &Message, conf_number: u16) -> Self {
        let mut header = Self {
            status: b' ',
            msg_number: Self::format_number(msg.id as u32),
            date: Self::format_date(&msg.date),
            time: Self::format_time(&msg.date),
            to: Self::pad_field(&msg.to, 25),
            from: Self::pad_field(&msg.from, 25),
            subject: Self::pad_field(&msg.subject, 25),
            password: [b' '; 12],
            reply_to: Self::format_number(msg.reply_to.unwrap_or(0)),
            num_blocks: [b'0'; 6],
            alive: 0xE1,
            conf_number,
            tag_line: 0,
        };

        // Calculate number of 128-byte blocks needed
        let body_blocks = (msg.body.len() + 127) / 128;
        let blocks_str = format!("{:6}", body_blocks);
        header.num_blocks.copy_from_slice(blocks_str.as_bytes());

        header
    }

    fn pad_field(s: &str, len: usize) -> Vec<u8> {
        let mut buf = vec![b' '; len];
        let bytes = s.as_bytes();
        let copy_len = bytes.len().min(len);
        buf[..copy_len].copy_from_slice(&bytes[..copy_len]);
        buf.try_into().unwrap()
    }

    fn format_number(n: u32) -> [u8; 7] {
        let s = format!("{:7}", n);
        s.as_bytes().try_into().unwrap()
    }

    fn format_date(dt: &chrono::DateTime<chrono::Utc>) -> [u8; 8] {
        let s = dt.format("%m-%d-%y").to_string();
        s.as_bytes().try_into().unwrap()
    }

    fn format_time(dt: &chrono::DateTime<chrono::Utc>) -> [u8; 5] {
        let s = dt.format("%H:%M").to_string();
        s.as_bytes().try_into().unwrap()
    }
}

pub struct QwkPacketGenerator {
    bbs_id: String,
    bbs_name: String,
}

impl QwkPacketGenerator {
    pub async fn generate_packet(
        &self,
        messages: Vec<Message>,
        conferences: Vec<Conference>,
        output_path: &Path,
    ) -> anyhow::Result<()> {
        let file = std::fs::File::create(output_path)?;
        let mut zip = ZipWriter::new(file);
        let options = FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);

        // Write CONTROL.DAT
        zip.start_file("CONTROL.DAT", options)?;
        self.write_control_dat(&mut zip, &conferences)?;

        // Write DOOR.ID
        zip.start_file("DOOR.ID", options)?;
        writeln!(zip, "DOOR = Impulse BBS")?;
        writeln!(zip, "VERSION = 7.1")?;
        writeln!(zip, "SYSTEM = {}", self.bbs_name)?;
        writeln!(zip, "CONTROLNAME = IMPULSE")?;

        // Write MESSAGES.DAT
        zip.start_file("MESSAGES.DAT", options)?;
        self.write_messages_dat(&mut zip, messages)?;

        zip.finish()?;
        Ok(())
    }

    fn write_control_dat(
        &self,
        writer: &mut impl Write,
        conferences: &[Conference],
    ) -> anyhow::Result<()> {
        writeln!(writer, "{}", self.bbs_name)?;
        writeln!(writer, "{}", self.bbs_id)?;
        writeln!(writer, "00:00")?;  // Login time
        writeln!(writer, "IMPULSE")?;  // BBS software
        writeln!(writer, "0")?;  // News file

        for conf in conferences {
            writeln!(writer, "{}", conf.id)?;
            writeln!(writer, "{}", conf.name)?;
        }

        Ok(())
    }

    fn write_messages_dat(
        &self,
        writer: &mut impl Write,
        messages: Vec<Message>,
    ) -> anyhow::Result<()> {
        use binrw::BinWrite;

        // First block: BBS identifier (128 bytes)
        let mut header_block = vec![b' '; 128];
        let id_bytes = self.bbs_id.as_bytes();
        let len = id_bytes.len().min(8);
        header_block[..len].copy_from_slice(&id_bytes[..len]);
        writer.write_all(&header_block)?;

        // Write each message
        for msg in messages {
            let header = QwkMessageHeader::new(&msg, msg.conference_id);
            let mut buf = Vec::new();
            header.write(&mut std::io::Cursor::new(&mut buf))?;
            writer.write_all(&buf)?;

            // Write message body in 128-byte blocks
            let body_bytes = msg.body.as_bytes();
            for chunk in body_bytes.chunks(128) {
                let mut block = vec![b' '; 128];
                block[..chunk.len()].copy_from_slice(chunk);
                writer.write_all(&block)?;
            }
        }

        Ok(())
    }
}
```

**QWK Reply Packet Parsing:**
```rust
use binrw::BinRead;
use zip::ZipArchive;

#[derive(Debug, BinRead)]
pub struct QwkReplyHeader {
    pub conf_number: [u8; 7],
    pub tag_line: [u8; 8],
    pub date: [u8; 8],
    pub time: [u8; 5],
    pub to: [u8; 25],
    pub from: [u8; 25],
    pub subject: [u8; 25],
    pub password: [u8; 12],
    pub reply_to: [u8; 8],
    pub num_blocks: [u8; 6],
    pub alive: u8,
}

pub struct QwkReplyParser {
    bbs_id: String,
}

impl QwkReplyParser {
    pub async fn parse_reply_packet(
        &self,
        packet_path: &Path,
    ) -> anyhow::Result<Vec<ReplyMessage>> {
        let file = std::fs::File::open(packet_path)?;
        let mut archive = ZipArchive::new(file)?;

        // Find the .MSG file (usually <BBSID>.MSG)
        let msg_file_name = format!("{}.MSG", self.bbs_id);
        let mut msg_file = archive.by_name(&msg_file_name)?;

        let mut messages = Vec::new();
        let mut buffer = Vec::new();
        msg_file.read_to_end(&mut buffer)?;

        // Parse messages from buffer
        let mut cursor = std::io::Cursor::new(buffer);

        while cursor.position() < cursor.get_ref().len() as u64 {
            let header = QwkReplyHeader::read(&mut cursor)?;

            // Read message body blocks
            let num_blocks = std::str::from_utf8(&header.num_blocks)?
                .trim()
                .parse::<usize>()?;

            let mut body = Vec::new();
            for _ in 0..num_blocks {
                let mut block = vec![0u8; 128];
                cursor.read_exact(&mut block)?;
                body.extend_from_slice(&block);
            }

            // Trim trailing spaces and convert to string
            let body_str = String::from_utf8_lossy(&body)
                .trim_end()
                .to_string();

            messages.push(ReplyMessage {
                conference_id: std::str::from_utf8(&header.conf_number)?
                    .trim()
                    .parse()?,
                to: String::from_utf8_lossy(&header.to).trim().to_string(),
                from: String::from_utf8_lossy(&header.from).trim().to_string(),
                subject: String::from_utf8_lossy(&header.subject).trim().to_string(),
                body: body_str,
                reply_to: std::str::from_utf8(&header.reply_to)?
                    .trim()
                    .parse()
                    .ok(),
            });
        }

        Ok(messages)
    }
}

pub struct ReplyMessage {
    pub conference_id: u16,
    pub to: String,
    pub from: String,
    pub subject: String,
    pub body: String,
    pub reply_to: Option<u32>,
}
```

**FidoNet Addressing:**
```rust
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FidoAddress {
    pub zone: u16,
    pub net: u16,
    pub node: u16,
    pub point: Option<u16>,
}

impl FidoAddress {
    pub fn new(zone: u16, net: u16, node: u16, point: Option<u16>) -> Self {
        Self { zone, net, node, point }
    }

    pub fn parse(s: &str) -> anyhow::Result<Self> {
        // Format: zone:net/node[.point]
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err(anyhow::anyhow!("Invalid FidoNet address format"));
        }

        let zone = parts[0].parse()?;

        let net_node_point: Vec<&str> = parts[1].split('/').collect();
        if net_node_point.len() != 2 {
            return Err(anyhow::anyhow!("Invalid net/node format"));
        }

        let net = net_node_point[0].parse()?;

        let node_point: Vec<&str> = net_node_point[1].split('.').collect();
        let node = node_point[0].parse()?;
        let point = if node_point.len() > 1 {
            Some(node_point[1].parse()?)
        } else {
            None
        };

        Ok(Self { zone, net, node, point })
    }

    pub fn is_local(&self, other: &FidoAddress) -> bool {
        self.zone == other.zone && self.net == other.net
    }

    pub fn is_same_zone(&self, other: &FidoAddress) -> bool {
        self.zone == other.zone
    }
}

impl fmt::Display for FidoAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}/{}", self.zone, self.net, self.node)?;
        if let Some(point) = self.point {
            write!(f, ".{}", point)?;
        }
        Ok(())
    }
}

pub struct MessageRouter {
    local_address: FidoAddress,
}

impl MessageRouter {
    pub fn new(local_address: FidoAddress) -> Self {
        Self { local_address }
    }

    pub fn determine_route(&self, dest: &FidoAddress) -> RoutingDecision {
        if dest == &self.local_address {
            return RoutingDecision::Local;
        }

        if dest.is_local(&self.local_address) {
            return RoutingDecision::DirectRoute(dest.clone());
        }

        if dest.is_same_zone(&self.local_address) {
            // Route through zone coordinator
            return RoutingDecision::ViaHub(FidoAddress::new(
                dest.zone,
                dest.net,
                0,  // Net coordinator
                None,
            ));
        }

        // Different zone - route through zone gate
        RoutingDecision::ViaGate(FidoAddress::new(
            dest.zone,
            0,  // Zone coordinator
            0,
            None,
        ))
    }
}

#[derive(Debug, PartialEq)]
pub enum RoutingDecision {
    Local,
    DirectRoute(FidoAddress),
    ViaHub(FidoAddress),
    ViaGate(FidoAddress),
}
```

**Message Import/Export:**
```rust
use std::path::Path;
use tokio::fs;
use tokio::io::AsyncWriteExt;

pub struct MessageExporter {
    format: ExportFormat,
}

#[derive(Debug, Clone, Copy)]
pub enum ExportFormat {
    Text,
    Json,
    Csv,
}

impl MessageExporter {
    pub async fn export_messages(
        &self,
        messages: Vec<Message>,
        output_path: &Path,
    ) -> anyhow::Result<()> {
        match self.format {
            ExportFormat::Text => self.export_text(messages, output_path).await,
            ExportFormat::Json => self.export_json(messages, output_path).await,
            ExportFormat::Csv => self.export_csv(messages, output_path).await,
        }
    }

    async fn export_text(
        &self,
        messages: Vec<Message>,
        output_path: &Path,
    ) -> anyhow::Result<()> {
        let mut file = fs::File::create(output_path).await?;

        for msg in messages {
            let text = format!(
                "Message #{}\n\
                 From: {} <{}>\n\
                 To: {}\n\
                 Date: {}\n\
                 Subject: {}\n\
                 {}\n\
                 {}\n\n",
                msg.id,
                msg.from,
                msg.from_address.as_deref().unwrap_or(""),
                msg.to,
                msg.date.format("%Y-%m-%d %H:%M:%S"),
                msg.subject,
                "-".repeat(70),
                msg.body
            );
            file.write_all(text.as_bytes()).await?;
        }

        Ok(())
    }

    async fn export_json(
        &self,
        messages: Vec<Message>,
        output_path: &Path,
    ) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(&messages)?;
        fs::write(output_path, json).await?;
        Ok(())
    }

    async fn export_csv(
        &self,
        messages: Vec<Message>,
        output_path: &Path,
    ) -> anyhly::Result<()> {
        let mut wtr = csv::Writer::from_path(output_path)?;

        for msg in messages {
            wtr.serialize(msg)?;
        }

        wtr.flush()?;
        Ok(())
    }
}

pub struct MessageImporter {
    format: ImportFormat,
}

#[derive(Debug, Clone, Copy)]
pub enum ImportFormat {
    Text,
    Json,
}

impl MessageImporter {
    pub async fn import_messages(
        &self,
        input_path: &Path,
    ) -> anyhow::Result<Vec<Message>> {
        match self.format {
            ImportFormat::Text => self.import_text(input_path).await,
            ImportFormat::Json => self.import_json(input_path).await,
        }
    }

    async fn import_json(&self, input_path: &Path) -> anyhow::Result<Vec<Message>> {
        let content = fs::read_to_string(input_path).await?;
        let messages: Vec<Message> = serde_json::from_str(&content)?;
        Ok(messages)
    }

    async fn import_text(&self, input_path: &Path) -> anyhow::Result<Vec<Message>> {
        // Parse text format (basic implementation)
        let content = fs::read_to_string(input_path).await?;
        // Implementation would parse structured text format
        Ok(Vec::new())
    }
}
```

---

## Dependencies

### Upstream Dependencies
- **Sprint 11**: Message base system provides core message structures
- **Sprint 06**: User system for message ownership
- **Sprint 13-14**: File areas for QWK packet upload/download

### Blocks Downstream
- **Sprint 24**: Integration testing includes QWK packet verification
- **Future**: Message networking (FidoNet, BinkleyTerm)

---

## Testing Requirements

### Unit Tests
- [ ] QWK header generation matches specification
- [ ] QWK packet structure validates with offline readers
- [ ] QWK reply parsing handles malformed packets
- [ ] FidoNet address parsing (valid and invalid formats)
- [ ] Message routing logic for all zones
- [ ] Export formats (text, JSON, CSV) are valid
- [ ] Import handles corrupt data gracefully

### Integration Tests
- [ ] Generate QWK packet, download, verify with offline reader
- [ ] Upload QWK reply packet, parse, verify messages imported
- [ ] Export messages to file, reimport, verify integrity
- [ ] FidoNet addressing integrates with message headers
- [ ] QWK packets compress/decompress correctly

### Compatibility Tests
- [ ] QWK packets work with Blue Wave offline reader
- [ ] QWK packets work with MultiMail
- [ ] QWK packets work with OLX (offline express)
- [ ] Reply packets from different readers parse correctly
- [ ] ZIP compression compatible with standard tools

### Performance Tests
- [ ] Generate QWK packet with 1000 messages < 3 seconds
- [ ] Parse reply packet with 100 messages < 1 second
- [ ] Export 10,000 messages to JSON < 5 seconds
- [ ] FidoNet routing decision < 1ms per message

---

## Notes and Decisions

### Design Decisions
*To be populated during sprint*

**Proposed Decisions:**
- Use standard QWK format (128-byte blocks) for compatibility
- Support both QWK and REP file extensions
- ZIP compression for packet size reduction
- FidoNet addressing for future networking (even if not networked yet)
- Export formats: text (human-readable), JSON (structured), CSV (spreadsheet)
- Import only from trusted sources (validate structure)
- Store original FidoNet addresses in message headers
- Prepare routing infrastructure but don't implement actual networking yet

### Lessons Learned
*To be populated during sprint*

### Risks and Mitigations
- **Risk**: QWK format is complex and error-prone
- **Mitigation**: Test with multiple offline readers; strict validation; reference implementations
- **Risk**: Reply packets may be malformed or malicious
- **Mitigation**: Strict parsing with error handling; size limits; sanitize input
- **Risk**: Export file corruption
- **Mitigation**: Atomic writes; verify after export; backup before import
- **Risk**: FidoNet addressing may not be needed
- **Mitigation**: Implement infrastructure anyway; enables future networking; minimal overhead
- **Risk**: QWK readers may not handle large packets
- **Mitigation**: Split packets at 4MB; document limits; test with large message bases

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
