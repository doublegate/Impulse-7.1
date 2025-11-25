# Sprint 11: Message Base - Read Functionality

**Phase:** Phase 2 - Core Features
**Duration:** Completed in 1 session (~3 hours)
**Sprint Dates:** 2025-11-25
**Status:** ✅ Complete

---

## Sprint Overview

Sprint 11 implements the message reading functionality, supporting both JAM and Hudson message formats. This sprint creates the message list and read screens with threading support, allowing users to browse and read messages in a threaded conversation view.

**Context:** This is the third sprint of Phase 2 (Core Features). Message reading is the first half of the messaging system.

**Expected Outcomes:** By the end of this sprint, users will be able to browse message areas, view message lists, and read messages with proper threading.

---

## Objectives

- [x] Implement MessageBase trait for reading messages
- [x] Support JAM and Hudson message formats
- [x] Create message list and read screens
- [x] Display threaded conversations

---

## Deliverables

| Deliverable | Type | Acceptance Criteria |
|-------------|------|---------------------|
| `impulse-message` crate with MessageBase trait | Code | Trait defining read operations for message bases |
| JAM format reader | Code | Parse .JHR, .JDT, .JDX files correctly |
| Hudson format reader | Code | Parse Hudson message format |
| Message list screen | UI | Display paginated list of messages |
| Message read screen | UI | Display full message with threading |

---

## Detailed Tasks

### Task Category 1: MessageBase Trait Design

- [ ] **Task 1.1**: Define MessageBase trait
  - Implementation notes: read_message(), message_count(), search(), get_thread()
  - Files affected: `crates/impulse-message/src/trait.rs`
  - Estimated hours: 3

- [ ] **Task 1.2**: Define Message struct
  - Implementation notes: header fields (from, to, subject, date), body, thread info
  - Files affected: `crates/impulse-message/src/types.rs`
  - Estimated hours: 2

- [ ] **Task 1.3**: Define MessageThread struct
  - Implementation notes: parent_id, reply_count, depth, children
  - Files affected: `crates/impulse-message/src/types.rs`
  - Estimated hours: 2

- [ ] **Task 1.4**: Create error types for message operations
  - Implementation notes: MessageNotFound, AreaNotFound, CorruptMessage
  - Files affected: `crates/impulse-message/src/error.rs`
  - Estimated hours: 1

### Task Category 2: JAM Format Implementation

- [ ] **Task 2.1**: Implement JAM header parsing
  - Implementation notes: Parse .JHR file (message header records)
  - Files affected: `crates/impulse-message/src/formats/jam/header.rs`
  - Estimated hours: 6

- [ ] **Task 2.2**: Implement JAM data parsing
  - Implementation notes: Parse .JDT file (message text data)
  - Files affected: `crates/impulse-message/src/formats/jam/data.rs`
  - Estimated hours: 4

- [ ] **Task 2.3**: Implement JAM index parsing
  - Implementation notes: Parse .JDX file (index records)
  - Files affected: `crates/impulse-message/src/formats/jam/index.rs`
  - Estimated hours: 4

- [ ] **Task 2.4**: Implement JAM MessageBase trait
  - Implementation notes: JamMessageBase struct implementing MessageBase
  - Files affected: `crates/impulse-message/src/formats/jam/mod.rs`
  - Estimated hours: 5

- [ ] **Task 2.5**: Handle JAM kludge lines
  - Implementation notes: Parse control information (MSGID, REPLY, etc.)
  - Files affected: `crates/impulse-message/src/formats/jam/kludge.rs`
  - Estimated hours: 3

### Task Category 3: Hudson Format Implementation

- [ ] **Task 3.1**: Implement Hudson format reader
  - Implementation notes: Parse Hudson binary format
  - Files affected: `crates/impulse-message/src/formats/hudson/mod.rs`
  - Estimated hours: 6

- [ ] **Task 3.2**: Hudson MessageBase trait implementation
  - Implementation notes: HudsonMessageBase struct
  - Files affected: `crates/impulse-message/src/formats/hudson/mod.rs`
  - Estimated hours: 4

- [ ] **Task 3.3**: Hudson index management
  - Implementation notes: Handle Hudson index files
  - Files affected: `crates/impulse-message/src/formats/hudson/index.rs`
  - Estimated hours: 3

### Task Category 4: Message List Screen

- [ ] **Task 4.1**: Design message list ANSI screen
  - Implementation notes: Header, message rows (num, from, to, subject, date), footer
  - Files affected: `assets/screens/message_list.ans`, `crates/impulse-message/src/screens/list.rs`
  - Estimated hours: 4

- [ ] **Task 4.2**: Implement pagination
  - Implementation notes: Page up/down, jump to page, show page indicator
  - Files affected: `crates/impulse-message/src/screens/list.rs`
  - Estimated hours: 4

- [ ] **Task 4.3**: Implement message list navigation
  - Implementation notes: Arrow keys, number selection, search shortcut
  - Files affected: `crates/impulse-message/src/screens/list.rs`
  - Estimated hours: 3

- [ ] **Task 4.4**: Add search functionality
  - Implementation notes: Search by subject, from, to, content
  - Files affected: `crates/impulse-message/src/screens/list.rs`
  - Estimated hours: 4

- [ ] **Task 4.5**: Display message status indicators
  - Implementation notes: New/read, replied, private/public
  - Files affected: `crates/impulse-message/src/screens/list.rs`
  - Estimated hours: 2

### Task Category 5: Message Read Screen

- [ ] **Task 5.1**: Design message read ANSI screen
  - Implementation notes: Header (from/to/subject/date), body, thread indicator, footer
  - Files affected: `assets/screens/message_read.ans`, `crates/impulse-message/src/screens/read.rs`
  - Estimated hours: 5

- [ ] **Task 5.2**: Implement message body rendering
  - Implementation notes: Word wrap, ANSI color support, quote highlighting
  - Files affected: `crates/impulse-message/src/screens/read.rs`
  - Estimated hours: 5

- [ ] **Task 5.3**: Show reply threading
  - Implementation notes: Thread tree, parent/child indicators, depth visualization
  - Files affected: `crates/impulse-message/src/screens/read.rs`
  - Estimated hours: 4

- [ ] **Task 5.4**: Implement navigation controls
  - Implementation notes: Next/prev message, jump to reply, back to list
  - Files affected: `crates/impulse-message/src/screens/read.rs`
  - Estimated hours: 3

- [ ] **Task 5.5**: Add reply preview
  - Implementation notes: Show snippet of replies, jump to full reply
  - Files affected: `crates/impulse-message/src/screens/read.rs`
  - Estimated hours: 3

### Task Category 6: Testing

- [ ] **Task 6.1**: Create test JAM message bases
  - Implementation notes: Generate sample .JHR/.JDT/.JDX files with test data
  - Files affected: `tests/fixtures/jam/`
  - Estimated hours: 3

- [ ] **Task 6.2**: Create test Hudson message bases
  - Implementation notes: Generate sample Hudson files with test data
  - Files affected: `tests/fixtures/hudson/`
  - Estimated hours: 3

- [ ] **Task 6.3**: Test JAM parsing
  - Implementation notes: Unit tests for header, data, index parsing
  - Files affected: `tests/jam_format_test.rs`
  - Estimated hours: 4

- [ ] **Task 6.4**: Test message threading
  - Implementation notes: Verify thread tree construction, depth calculation
  - Files affected: `tests/threading_test.rs`
  - Estimated hours: 3

- [ ] **Task 6.5**: Integration test for message reading
  - Implementation notes: Full flow: open area → list → read → navigate
  - Files affected: `tests/message_read_integration_test.rs`
  - Estimated hours: 4

---

## Technical Details

### Architecture Considerations

- Use memory-mapped files for large message bases (performance)
- Cache parsed headers in memory with LRU eviction
- Support lazy loading of message bodies (read headers, load body on demand)
- Handle corrupted messages gracefully

### Dependencies

**Crate-Level Dependencies:**
```toml
[dependencies]
tokio = { workspace = true }
serde = { workspace = true }
bincode = "1.3"
memmap2 = "0.9"
lru = "0.12"
chrono = "0.4"
```

### Code Patterns

**MessageBase Trait:**
```rust
#[async_trait]
pub trait MessageBase: Send + Sync {
    async fn read_message(&self, msg_num: u32) -> Result<Message>;
    async fn message_count(&self) -> Result<u32>;
    async fn search(&self, criteria: SearchCriteria) -> Result<Vec<u32>>;
    async fn get_thread(&self, msg_num: u32) -> Result<MessageThread>;
    async fn list_messages(&self, start: u32, count: u32) -> Result<Vec<MessageHeader>>;
}
```

**JAM Format Structures:**
```rust
#[repr(C)]
struct JamHeader {
    signature: [u8; 4],  // "JAM\0"
    created: u32,
    modified: u32,
    active: u32,
    password_crc: u32,
    base_msg_num: u32,
}

#[repr(C)]
struct JamMessageHeader {
    signature: [u8; 4],
    revision: u16,
    reserved: u16,
    subfield_len: u32,
    times_read: u32,
    msg_id_crc: u32,
    reply_id_crc: u32,
    reply_to: u32,
    reply_1st: u32,
    reply_next: u32,
    date_written: u32,
    date_received: u32,
    date_processed: u32,
    msg_num: u32,
    attribute: u32,
    attribute2: u32,
    offset: u32,
    text_len: u32,
}
```

---

## Dependencies

### Upstream Dependencies
- **Sprint 09**: User authentication for message access control
- **Sprint 04**: Storage layer for message area configuration

### Blocks Downstream
- **Sprint 12**: Message writing requires read functionality
- **Sprint 22**: Advanced message features build on basic reading

---

## Acceptance Criteria

- [ ] Can read existing JAM message bases
- [ ] Can read existing Hudson message bases
- [ ] Message list displays correctly with pagination
- [ ] Threading is properly visualized
- [ ] Search finds messages by various criteria
- [ ] Navigation works smoothly between list and read screens
- [ ] Handles corrupted messages without crashing
- [ ] Performance is acceptable for bases with 10,000+ messages

---

## Testing Requirements

### Unit Tests
- [ ] JAM header parsing
- [ ] JAM data extraction
- [ ] Hudson format parsing
- [ ] Thread tree construction
- [ ] Search functionality

### Integration Tests
- [ ] Complete message reading flow
- [ ] Multi-area navigation
- [ ] Threading across multiple messages
- [ ] Search with various criteria

### Performance Tests
- [ ] Load time for large message bases (10,000+ messages)
- [ ] Search performance
- [ ] Memory usage with cached headers

---

## Notes and Decisions

### Design Decisions
*To be populated during sprint*

**Proposed Decisions:**
- Use JAM format as primary, Hudson for backward compatibility
- Cache last 1000 message headers in LRU cache
- Use memory-mapped files for bases over 100MB
- Support ANSI in message bodies, sanitize control sequences

### Lessons Learned
*To be populated during sprint*

### Risks and Mitigations
- **Risk**: JAM format documentation may be incomplete
- **Mitigation**: Study existing JAM implementations, reverse engineer if needed
- **Risk**: Large message bases may cause memory issues
- **Mitigation**: Implement lazy loading, pagination, LRU caching
- **Risk**: Corrupted message bases may crash reader
- **Mitigation**: Extensive error handling, checksum validation, recovery mode

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
