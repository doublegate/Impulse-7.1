# Sprint 12: Message Base - Write Functionality

**Phase:** Phase 2 - Core Features
**Duration:** Completed in 1 session (~2 hours)
**Sprint Dates:** 2025-11-25
**Status:** ✅ Complete

---

## Sprint Overview

Sprint 12 implements message posting and editing functionality, completing the messaging system. Users will be able to post new messages, reply to existing messages with threading, and use a full-screen editor with word wrap and basic editing commands.

**Context:** This is the fourth sprint of Phase 2 (Core Features). This completes the core messaging functionality started in Sprint 11.

**Expected Outcomes:** By the end of this sprint, the BBS will have a complete messaging system where users can read and write messages with proper threading.

---

## Objectives

- [ ] Implement message posting functionality
- [ ] Support replying to messages with threading
- [ ] Create full-screen message editor
- [ ] Add message validation and sanitization

---

## Deliverables

| Deliverable | Type | Acceptance Criteria |
|-------------|------|---------------------|
| Message posting implementation | Code | Write messages to JAM/Hudson formats |
| Reply functionality | Code | Maintain parent_id linkage and threading |
| Full-screen editor | UI | Line-based editing with word wrap |
| Message validation | Code | Sanitize input, check required fields |

---

## Detailed Tasks

### Task Category 1: Message Posting

- [ ] **Task 1.1**: Implement post_message() function
  - Implementation notes: Create message record, update indices
  - Files affected: `crates/impulse-message/src/post.rs`
  - Estimated hours: 4

- [ ] **Task 1.2**: JAM message writing
  - Implementation notes: Write to .JHR, .JDT, .JDX files atomically
  - Files affected: `crates/impulse-message/src/formats/jam/write.rs`
  - Estimated hours: 6

- [ ] **Task 1.3**: Hudson message writing
  - Implementation notes: Write to Hudson format files
  - Files affected: `crates/impulse-message/src/formats/hudson/write.rs`
  - Estimated hours: 5

- [ ] **Task 1.4**: Update message base indices
  - Implementation notes: Maintain index consistency, update counts
  - Files affected: `crates/impulse-message/src/formats/jam/write.rs`
  - Estimated hours: 4

- [ ] **Task 1.5**: Atomic write operations
  - Implementation notes: Use temp files, rename on success, rollback on failure
  - Files affected: `crates/impulse-message/src/atomic.rs`
  - Estimated hours: 4

### Task Category 2: Reply Functionality

- [ ] **Task 2.1**: Implement reply_to() function
  - Implementation notes: Create reply message, link to parent
  - Files affected: `crates/impulse-message/src/reply.rs`
  - Estimated hours: 3

- [ ] **Task 2.2**: Maintain parent_id linkage
  - Implementation notes: Set reply_to field, update parent's reply chain
  - Files affected: `crates/impulse-message/src/reply.rs`
  - Estimated hours: 3

- [ ] **Task 2.3**: Quote original message (optional)
  - Implementation notes: Prepend quoted text with "> " prefix
  - Files affected: `crates/impulse-message/src/quote.rs`
  - Estimated hours: 3

- [ ] **Task 2.4**: Update thread metadata
  - Implementation notes: Increment reply count, update thread depth
  - Files affected: `crates/impulse-message/src/threading.rs`
  - Estimated hours: 2

- [ ] **Task 2.5**: Notify parent message author (future)
  - Implementation notes: Placeholder for notification system
  - Files affected: `crates/impulse-message/src/notifications.rs`
  - Estimated hours: 1

### Task Category 3: Full-Screen Editor

- [ ] **Task 3.1**: Design editor ANSI screen
  - Implementation notes: Edit area, status line, help footer
  - Files affected: `assets/screens/editor.ans`, `crates/impulse-message/src/editor/mod.rs`
  - Estimated hours: 5

- [ ] **Task 3.2**: Implement line-based editing
  - Implementation notes: Insert/delete chars, move cursor, line operations
  - Files affected: `crates/impulse-message/src/editor/core.rs`
  - Estimated hours: 8

- [ ] **Task 3.3**: Add word wrap
  - Implementation notes: Automatic line break at width, preserve words
  - Files affected: `crates/impulse-message/src/editor/wordwrap.rs`
  - Estimated hours: 5

- [ ] **Task 3.4**: Implement editing commands
  - Implementation notes: Save (Ctrl+S), Abort (Ctrl+A), Delete line (Ctrl+Y)
  - Files affected: `crates/impulse-message/src/editor/commands.rs`
  - Estimated hours: 4

- [ ] **Task 3.5**: Add basic text formatting
  - Implementation notes: Support ANSI color codes (optional)
  - Files affected: `crates/impulse-message/src/editor/formatting.rs`
  - Estimated hours: 3

- [ ] **Task 3.6**: Display character/line count
  - Implementation notes: Show current position, total lines, char count
  - Files affected: `crates/impulse-message/src/editor/status.rs`
  - Estimated hours: 2

### Task Category 4: Message Header Input

- [ ] **Task 4.1**: Implement "To" field input
  - Implementation notes: Username validation, auto-complete (future)
  - Files affected: `crates/impulse-message/src/compose/to.rs`
  - Estimated hours: 3

- [ ] **Task 4.2**: Implement "Subject" field input
  - Implementation notes: Length validation, "Re: " prefix for replies
  - Files affected: `crates/impulse-message/src/compose/subject.rs`
  - Estimated hours: 2

- [ ] **Task 4.3**: Private/Public message flag
  - Implementation notes: Allow user to mark message as private
  - Files affected: `crates/impulse-message/src/compose/privacy.rs`
  - Estimated hours: 2

- [ ] **Task 4.4**: Message area selection
  - Implementation notes: Choose target area, respect user permissions
  - Files affected: `crates/impulse-message/src/compose/area.rs`
  - Estimated hours: 3

### Task Category 5: Validation and Sanitization

- [ ] **Task 5.1**: Validate required fields
  - Implementation notes: Check for non-empty to, subject, body
  - Files affected: `crates/impulse-message/src/validation.rs`
  - Estimated hours: 2

- [ ] **Task 5.2**: Sanitize message body
  - Implementation notes: Strip dangerous control sequences, limit ANSI
  - Files affected: `crates/impulse-message/src/sanitize.rs`
  - Estimated hours: 4

- [ ] **Task 5.3**: Length limits
  - Implementation notes: Enforce maximum message length, subject length
  - Files affected: `crates/impulse-message/src/validation.rs`
  - Estimated hours: 2

- [ ] **Task 5.4**: Content filtering (optional)
  - Implementation notes: Basic profanity filter, spam detection
  - Files affected: `crates/impulse-message/src/filter.rs`
  - Estimated hours: 3

### Task Category 6: Testing

- [ ] **Task 6.1**: Test message posting
  - Implementation notes: Post messages, verify file updates
  - Files affected: `tests/message_post_test.rs`
  - Estimated hours: 3

- [ ] **Task 6.2**: Test reply threading
  - Implementation notes: Create reply chains, verify linkage
  - Files affected: `tests/reply_threading_test.rs`
  - Estimated hours: 3

- [ ] **Task 6.3**: Test editor functionality
  - Implementation notes: Simulate keystrokes, verify text manipulation
  - Files affected: `tests/editor_test.rs`
  - Estimated hours: 4

- [ ] **Task 6.4**: Test atomic writes
  - Implementation notes: Simulate failures, verify rollback
  - Files affected: `tests/atomic_write_test.rs`
  - Estimated hours: 3

- [ ] **Task 6.5**: Integration test for complete posting flow
  - Implementation notes: Login → compose → post → verify → read
  - Files affected: `tests/message_post_integration_test.rs`
  - Estimated hours: 4

---

## Technical Details

### Architecture Considerations

- Use atomic file operations to prevent corruption
- Implement auto-save for editor (periodic backup)
- Support draft messages (save and continue later)
- Handle concurrent posts to same message base

### Dependencies

**Crate-Level Dependencies:**
```toml
[dependencies]
tokio = { workspace = true }
crossterm = "0.27"
unicode-width = "0.1"
tempfile = "3.8"
```

### Code Patterns

**Message Posting:**
```rust
pub struct MessagePoster {
    base: Arc<dyn MessageBase>,
    validator: MessageValidator,
}

impl MessagePoster {
    pub async fn post_message(&self, msg: NewMessage) -> Result<u32> {
        // Validate
        self.validator.validate(&msg)?;

        // Sanitize
        let sanitized = self.sanitize_content(&msg);

        // Write atomically
        let msg_num = self.base.write_message(sanitized).await?;

        // Update indices
        self.base.update_index(msg_num).await?;

        Ok(msg_num)
    }
}
```

**Full-Screen Editor:**
```rust
pub struct Editor {
    lines: Vec<String>,
    cursor: CursorPosition,
    max_width: usize,
    modified: bool,
}

impl Editor {
    pub fn insert_char(&mut self, ch: char) {
        let line = &mut self.lines[self.cursor.row];
        line.insert(self.cursor.col, ch);
        self.cursor.col += 1;

        // Check word wrap
        if line.len() > self.max_width {
            self.wrap_line(self.cursor.row);
        }

        self.modified = true;
    }

    fn wrap_line(&mut self, row: usize) {
        let line = &self.lines[row];
        if let Some(wrap_point) = find_wrap_point(line, self.max_width) {
            let remainder = line[wrap_point..].to_string();
            self.lines[row].truncate(wrap_point);
            self.lines.insert(row + 1, remainder);
        }
    }
}
```

---

## Dependencies

### Upstream Dependencies
- **Sprint 11**: Message reading required for reply functionality
- **Sprint 09**: User authentication for author information
- **Sprint 04**: Storage layer for message persistence

### Blocks Downstream
- **Sprint 22**: Advanced messaging builds on basic posting
- **Future**: Email gateway, network message routing

---

## Acceptance Criteria

- [ ] Users can post new messages
- [ ] Replies are properly threaded
- [ ] Editor is usable and intuitive
- [ ] Word wrap works correctly
- [ ] Messages persist correctly to JAM format
- [ ] Messages persist correctly to Hudson format
- [ ] Required fields are validated
- [ ] Content is sanitized for safety
- [ ] Atomic writes prevent corruption
- [ ] Editor supports undo (bonus feature)

---

## Testing Requirements

### Unit Tests
- [ ] Message validation logic
- [ ] Content sanitization
- [ ] Editor text operations
- [ ] Word wrap algorithm
- [ ] Atomic write operations

### Integration Tests
- [ ] Complete compose flow
- [ ] Reply with quoting
- [ ] Multi-user concurrent posting
- [ ] Editor save and resume

### User Acceptance Tests
- [ ] SysOp can post announcement
- [ ] User can reply to message
- [ ] Editor handles long messages
- [ ] Crash recovery works

---

## Notes and Decisions

### Design Decisions
*To be populated during sprint*

**Proposed Decisions:**
- Maximum message length: 64KB (configurable)
- Maximum subject length: 72 characters
- Word wrap at 79 characters
- Auto-save every 60 seconds
- Support basic ANSI codes (colors only, no cursor movement)

### Lessons Learned
*To be populated during sprint*

### Risks and Mitigations
- **Risk**: Editor may be complex to implement well
- **Mitigation**: Start with minimal viable editor, iterate based on feedback
- **Risk**: Concurrent writes may cause corruption
- **Mitigation**: File locking, atomic operations, retry logic
- **Risk**: Word wrap may be buggy
- **Mitigation**: Extensive testing with various text patterns, edge cases

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
