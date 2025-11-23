# Sprint 14: File Upload Functionality

**Phase:** Phase 2 - Core Features
**Duration:** 3 weeks
**Sprint Dates:** TBD
**Status:** Not Started

---

## Sprint Overview

Sprint 14 implements file upload handling, including FILE_ID.DIZ extraction, virus scanning with ClamAV, and upload validation. This completes the basic file management system started in Sprint 13.

**Context:** This is the sixth sprint of Phase 2 (Core Features). File upload completes the essential file management system.

**Expected Outcomes:** By the end of this sprint, users will be able to upload files with automatic virus scanning, description extraction, and validation.

---

## Objectives

- [ ] Implement file upload mechanism
- [ ] Extract and parse FILE_ID.DIZ from archives
- [ ] Integrate ClamAV virus scanning
- [ ] Add upload validation and quotas

---

## Deliverables

| Deliverable | Type | Acceptance Criteria |
|-------------|------|---------------------|
| File upload handler | Code | Receive and store uploaded files |
| FILE_ID.DIZ extraction | Code | Parse .DIZ from ZIP/RAR/7Z archives |
| Virus scanning integration | Code | Scan uploads with ClamAV |
| Upload validation | Code | Size limits, duplicates, quotas |

---

## Detailed Tasks

### Task Category 1: Upload Handling

- [ ] **Task 1.1**: Implement upload receiver (placeholder)
  - Implementation notes: Placeholder for protocol upload (actual protocol in Phase 3)
  - Files affected: `crates/impulse-files/src/upload/receiver.rs`
  - Estimated hours: 4

- [ ] **Task 1.2**: Store file in appropriate area
  - Implementation notes: Move file to area directory, set permissions
  - Files affected: `crates/impulse-files/src/upload/storage.rs`
  - Estimated hours: 3

- [ ] **Task 1.3**: Create FileRecord in database
  - Implementation notes: Insert metadata, set upload date, uploader
  - Files affected: `crates/impulse-files/src/upload/metadata.rs`
  - Estimated hours: 3

- [ ] **Task 1.4**: Handle upload failures
  - Implementation notes: Cleanup partial uploads, rollback database changes
  - Files affected: `crates/impulse-files/src/upload/cleanup.rs`
  - Estimated hours: 3

- [ ] **Task 1.5**: Update area statistics
  - Implementation notes: Increment file count, update total size
  - Files affected: `crates/impulse-files/src/upload/stats.rs`
  - Estimated hours: 2

### Task Category 2: FILE_ID.DIZ Extraction

- [ ] **Task 2.1**: Support ZIP archive extraction
  - Implementation notes: Use zip crate to read ZIP files
  - Files affected: `crates/impulse-files/src/diz/zip.rs`
  - Estimated hours: 4

- [ ] **Task 2.2**: Support RAR archive extraction
  - Implementation notes: Use unrar or system unrar command
  - Files affected: `crates/impulse-files/src/diz/rar.rs`
  - Estimated hours: 4

- [ ] **Task 2.3**: Support 7Z archive extraction
  - Implementation notes: Use 7z command or lzma crate
  - Files affected: `crates/impulse-files/src/diz/sevenz.rs`
  - Estimated hours: 4

- [ ] **Task 2.4**: Parse FILE_ID.DIZ content
  - Implementation notes: Normalize line endings, strip control chars, validate length
  - Files affected: `crates/impulse-files/src/diz/parser.rs`
  - Estimated hours: 3

- [ ] **Task 2.5**: Store description in database
  - Implementation notes: Save .DIZ content to FileRecord.description field
  - Files affected: `crates/impulse-files/src/upload/metadata.rs`
  - Estimated hours: 2

- [ ] **Task 2.6**: Handle missing FILE_ID.DIZ
  - Implementation notes: Prompt user for manual description
  - Files affected: `crates/impulse-files/src/upload/manual_desc.rs`
  - Estimated hours: 3

### Task Category 3: Virus Scanning

- [ ] **Task 3.1**: Integrate ClamAV clamd socket
  - Implementation notes: Connect to clamd via TCP or Unix socket
  - Files affected: `crates/impulse-files/src/scanning/clamav.rs`
  - Estimated hours: 5

- [ ] **Task 3.2**: Implement file scanning
  - Implementation notes: Send file to clamd, receive scan result
  - Files affected: `crates/impulse-files/src/scanning/scan.rs`
  - Estimated hours: 4

- [ ] **Task 3.3**: Quarantine infected files
  - Implementation notes: Move to quarantine directory, mark in database
  - Files affected: `crates/impulse-files/src/scanning/quarantine.rs`
  - Estimated hours: 3

- [ ] **Task 3.4**: Notify SysOp of infections
  - Implementation notes: Log infection, send alert to SysOp (email or message)
  - Files affected: `crates/impulse-files/src/scanning/notify.rs`
  - Estimated hours: 3

- [ ] **Task 3.5**: Handle scan failures
  - Implementation notes: Retry on timeout, fallback behavior if clamd unavailable
  - Files affected: `crates/impulse-files/src/scanning/fallback.rs`
  - Estimated hours: 3

### Task Category 4: Upload Validation

- [ ] **Task 4.1**: Check file size limits
  - Implementation notes: Enforce per-file and total upload size limits
  - Files affected: `crates/impulse-files/src/validation/size.rs`
  - Estimated hours: 2

- [ ] **Task 4.2**: Detect duplicate files
  - Implementation notes: Compare SHA-256 hashes, check for identical files
  - Files affected: `crates/impulse-files/src/validation/duplicates.rs`
  - Estimated hours: 4

- [ ] **Task 4.3**: Enforce user upload quotas
  - Implementation notes: Track user upload count/size per day/month
  - Files affected: `crates/impulse-files/src/validation/quotas.rs`
  - Estimated hours: 4

- [ ] **Task 4.4**: Validate file extensions
  - Implementation notes: Allow/block specific file types per area
  - Files affected: `crates/impulse-files/src/validation/extensions.rs`
  - Estimated hours: 3

- [ ] **Task 4.5**: Check area permissions
  - Implementation notes: Verify user has upload rights to selected area
  - Files affected: `crates/impulse-files/src/validation/permissions.rs`
  - Estimated hours: 2

### Task Category 5: Upload UI

- [ ] **Task 5.1**: Design upload prompt screen
  - Implementation notes: Select area, enter description, confirm upload
  - Files affected: `assets/screens/upload.ans`, `crates/impulse-files/src/screens/upload.rs`
  - Estimated hours: 4

- [ ] **Task 5.2**: Show upload progress (placeholder)
  - Implementation notes: Progress bar for upload (actual protocol in Phase 3)
  - Files affected: `crates/impulse-files/src/screens/progress.rs`
  - Estimated hours: 3

- [ ] **Task 5.3**: Display scanning status
  - Implementation notes: "Scanning for viruses..." indicator
  - Files affected: `crates/impulse-files/src/screens/scanning.rs`
  - Estimated hours: 2

- [ ] **Task 5.4**: Show upload confirmation
  - Implementation notes: Success message, file details, area, description
  - Files affected: `crates/impulse-files/src/screens/confirmation.rs`
  - Estimated hours: 2

### Task Category 6: Testing

- [ ] **Task 6.1**: Create test files and archives
  - Implementation notes: Sample ZIP/RAR/7Z files with .DIZ files
  - Files affected: `tests/fixtures/uploads/`
  - Estimated hours: 3

- [ ] **Task 6.2**: Test FILE_ID.DIZ extraction
  - Implementation notes: Various archive formats, missing .DIZ
  - Files affected: `tests/diz_extraction_test.rs`
  - Estimated hours: 3

- [ ] **Task 6.3**: Test virus scanning
  - Implementation notes: Mock ClamAV responses, infected files
  - Files affected: `tests/virus_scan_test.rs`
  - Estimated hours: 4

- [ ] **Task 6.4**: Test upload validation
  - Implementation notes: Size limits, duplicates, quotas
  - Files affected: `tests/upload_validation_test.rs`
  - Estimated hours: 4

- [ ] **Task 6.5**: Integration test for upload flow
  - Implementation notes: Upload → scan → extract → store → confirm
  - Files affected: `tests/upload_integration_test.rs`
  - Estimated hours: 5

---

## Technical Details

### Architecture Considerations

- Process uploads asynchronously (don't block user session)
- Use temporary upload directory during processing
- Atomic move to final location after successful processing
- Generate SHA-256 hash for duplicate detection

### Dependencies

**Crate-Level Dependencies:**
```toml
[dependencies]
tokio = { workspace = true }
sha2 = "0.10"
zip = "0.6"
tempfile = "3.8"

[dev-dependencies]
mockito = "1.2"
```

### Code Patterns

**Upload Processing Pipeline:**
```rust
pub struct UploadProcessor {
    scanner: Arc<dyn VirusScanner>,
    storage: Arc<dyn Storage>,
    config: UploadConfig,
}

impl UploadProcessor {
    pub async fn process_upload(&self, upload: Upload) -> Result<FileRecord> {
        // Validate
        self.validate_upload(&upload).await?;

        // Check duplicates
        if let Some(existing) = self.find_duplicate(&upload).await? {
            return Err(anyhow!("Duplicate of file #{}", existing.id));
        }

        // Scan for viruses
        let scan_result = self.scanner.scan_file(&upload.temp_path).await?;
        if !scan_result.is_clean {
            self.quarantine_file(&upload).await?;
            return Err(anyhow!("File contains virus: {}", scan_result.threat));
        }

        // Extract FILE_ID.DIZ
        let description = extract_file_id_diz(&upload.temp_path)?
            .unwrap_or_else(|| upload.manual_description.clone());

        // Store file
        let final_path = self.store_file(&upload).await?;

        // Create database record
        let record = self.create_file_record(&upload, &description, &final_path).await?;

        Ok(record)
    }
}
```

**ClamAV Integration:**
```rust
pub struct ClamAVScanner {
    socket_path: PathBuf,
}

#[async_trait]
impl VirusScanner for ClamAVScanner {
    async fn scan_file(&self, path: &Path) -> Result<ScanResult> {
        let stream = UnixStream::connect(&self.socket_path).await?;
        let mut stream = BufStream::new(stream);

        // Send INSTREAM command
        stream.write_all(b"nINSTREAM\n").await?;
        stream.flush().await?;

        // Stream file contents
        let mut file = File::open(path).await?;
        let mut buffer = vec![0u8; 8192];
        loop {
            let n = file.read(&mut buffer).await?;
            if n == 0 { break; }

            // Send chunk size and data
            stream.write_u32(n as u32).await?;
            stream.write_all(&buffer[..n]).await?;
        }

        // Send zero-length chunk to signal end
        stream.write_u32(0).await?;
        stream.flush().await?;

        // Read response
        let mut response = String::new();
        stream.read_line(&mut response).await?;

        Ok(Self::parse_response(&response))
    }
}
```

---

## Dependencies

### Upstream Dependencies
- **Sprint 13**: File area management required
- **Sprint 09**: User authentication for uploader tracking
- **Sprint 04**: Storage layer for file metadata

### Blocks Downstream
- **Sprint 17-18**: File transfer protocols use upload infrastructure
- **Sprint 23**: Admin needs upload management

---

## Acceptance Criteria

- [ ] Files can be uploaded successfully
- [ ] FILE_ID.DIZ is extracted and displayed
- [ ] Infected files are quarantined
- [ ] Duplicates are detected
- [ ] Size limits are enforced
- [ ] Upload quotas work correctly
- [ ] SysOp receives infection notifications
- [ ] Upload process is transactional (all-or-nothing)

---

## Testing Requirements

### Unit Tests
- [ ] FILE_ID.DIZ extraction (all formats)
- [ ] Duplicate detection
- [ ] Size validation
- [ ] Quota enforcement
- [ ] Extension filtering

### Integration Tests
- [ ] Complete upload flow
- [ ] Virus scanning integration
- [ ] Rollback on failure
- [ ] Multi-user concurrent uploads

### Security Tests
- [ ] Path traversal attacks
- [ ] Malicious archive files
- [ ] Oversized files
- [ ] Quota bypass attempts

---

## Notes and Decisions

### Design Decisions
*To be populated during sprint*

**Proposed Decisions:**
- Default max file size: 50MB (configurable)
- Default upload quota: 10 files or 100MB per day
- Supported archive formats: ZIP, RAR, 7Z
- Scan all uploads with ClamAV (mandatory)
- Store quarantined files for 30 days before deletion

### Lessons Learned
*To be populated during sprint*

### Risks and Mitigations
- **Risk**: ClamAV may be slow for large files
- **Mitigation**: Async scanning, show progress, configurable timeout
- **Risk**: Archive extraction may consume excessive CPU
- **Mitigation**: Limit extraction time, extract in separate process
- **Risk**: Users may abuse upload quotas
- **Mitigation**: Strict validation, SysOp override, quota reset schedules

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
