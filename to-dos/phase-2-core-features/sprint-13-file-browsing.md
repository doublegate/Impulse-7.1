# Sprint 13: File Areas - Browsing

**Phase:** Phase 2 - Core Features
**Duration:** 3 weeks
**Sprint Dates:** TBD
**Status:** Not Started

---

## Sprint Overview

Sprint 13 implements file area management and browsing functionality. Users will be able to navigate file areas, view file lists with descriptions, see file details, and search for files by various criteria.

**Context:** This is the fifth sprint of Phase 2 (Core Features). File browsing is the first half of the file transfer system.

**Expected Outcomes:** By the end of this sprint, users will be able to browse file areas and view file information, preparing for download functionality in later sprints.

---

## Objectives

- [ ] Implement file area management system
- [ ] Create file list and details screens
- [ ] Add comprehensive file search functionality
- [ ] Support FILE_ID.DIZ display

---

## Deliverables

| Deliverable | Type | Acceptance Criteria |
|-------------|------|---------------------|
| File area database schema | Database | Tables for file areas, files, and metadata |
| File list screen | UI | Paginated list with descriptions and stats |
| File details screen | UI | Extended info including FILE_ID.DIZ |
| Search functionality | Code | Search by name, description, uploader, date |

---

## Detailed Tasks

### Task Category 1: File Area Management

- [ ] **Task 1.1**: Define file area schema
  - Implementation notes: area_id, name, path, description, security_level
  - Files affected: `crates/impulse-storage/migrations/004_file_areas.sql`
  - Estimated hours: 2

- [ ] **Task 1.2**: Define file record schema
  - Implementation notes: file_id, area_id, filename, description, size, upload_date, uploader
  - Files affected: `crates/impulse-storage/migrations/004_file_areas.sql`
  - Estimated hours: 2

- [ ] **Task 1.3**: Implement FileArea struct
  - Implementation notes: Core file area type with metadata
  - Files affected: `crates/impulse-core/src/types/file_area.rs`
  - Estimated hours: 2

- [ ] **Task 1.4**: Implement FileRecord struct
  - Implementation notes: File metadata, download stats, FILE_ID.DIZ content
  - Files affected: `crates/impulse-core/src/types/file.rs`
  - Estimated hours: 3

- [ ] **Task 1.5**: Map areas to physical directories
  - Implementation notes: Validate paths, handle relative and absolute paths
  - Files affected: `crates/impulse-files/src/areas.rs`
  - Estimated hours: 3

### Task Category 2: File Area Navigation

- [ ] **Task 2.1**: Design area selection ANSI screen
  - Implementation notes: List of areas, file count, description
  - Files affected: `assets/screens/file_areas.ans`, `crates/impulse-files/src/screens/areas.rs`
  - Estimated hours: 4

- [ ] **Task 2.2**: Implement area list rendering
  - Implementation notes: Display areas, show counts, highlight selection
  - Files affected: `crates/impulse-files/src/screens/areas.rs`
  - Estimated hours: 3

- [ ] **Task 2.3**: Check user area permissions
  - Implementation notes: Verify security level, hidden areas, upload rights
  - Files affected: `crates/impulse-files/src/permissions.rs`
  - Estimated hours: 3

- [ ] **Task 2.4**: Navigate between areas
  - Implementation notes: Arrow keys, number selection, search
  - Files affected: `crates/impulse-files/src/screens/areas.rs`
  - Estimated hours: 2

### Task Category 3: File List Screen

- [ ] **Task 3.1**: Design file list ANSI screen
  - Implementation notes: Columns: #, filename, size, date, downloads, description
  - Files affected: `assets/screens/file_list.ans`, `crates/impulse-files/src/screens/list.rs`
  - Estimated hours: 5

- [ ] **Task 3.2**: Implement file list rendering
  - Implementation notes: Format sizes (KB/MB), dates, truncate descriptions
  - Files affected: `crates/impulse-files/src/screens/list.rs`
  - Estimated hours: 4

- [ ] **Task 3.3**: Add pagination support
  - Implementation notes: Page up/down, page size config, page indicator
  - Files affected: `crates/impulse-files/src/screens/list.rs`
  - Estimated hours: 3

- [ ] **Task 3.4**: Implement sorting options
  - Implementation notes: Sort by name, size, date, downloads
  - Files affected: `crates/impulse-files/src/screens/list.rs`
  - Estimated hours: 3

- [ ] **Task 3.5**: Show file status indicators
  - Implementation notes: New, offline, missing, popular (high downloads)
  - Files affected: `crates/impulse-files/src/screens/list.rs`
  - Estimated hours: 2

### Task Category 4: File Details Screen

- [ ] **Task 4.1**: Design file details ANSI screen
  - Implementation notes: Full description, FILE_ID.DIZ, stats, commands
  - Files affected: `assets/screens/file_details.ans`, `crates/impulse-files/src/screens/details.rs`
  - Estimated hours: 5

- [ ] **Task 4.2**: Display extended description
  - Implementation notes: Word-wrapped full description, ANSI support
  - Files affected: `crates/impulse-files/src/screens/details.rs`
  - Estimated hours: 3

- [ ] **Task 4.3**: Show FILE_ID.DIZ content
  - Implementation notes: Extract and display .DIZ file from archive
  - Files affected: `crates/impulse-files/src/diz.rs`
  - Estimated hours: 4

- [ ] **Task 4.4**: Display file statistics
  - Implementation notes: Upload date, uploader, downloads, last downloaded
  - Files affected: `crates/impulse-files/src/screens/details.rs`
  - Estimated hours: 2

- [ ] **Task 4.5**: Show archive contents (optional)
  - Implementation notes: List files inside ZIP/RAR/7Z archives
  - Files affected: `crates/impulse-files/src/archive.rs`
  - Estimated hours: 4

### Task Category 5: Search Functionality

- [ ] **Task 5.1**: Design search interface
  - Implementation notes: Search prompt, criteria selection, results display
  - Files affected: `assets/screens/file_search.ans`, `crates/impulse-files/src/screens/search.rs`
  - Estimated hours: 4

- [ ] **Task 5.2**: Implement filename search
  - Implementation notes: Wildcard support (* and ?), case-insensitive
  - Files affected: `crates/impulse-files/src/search.rs`
  - Estimated hours: 4

- [ ] **Task 5.3**: Implement description search
  - Implementation notes: Keyword matching, full-text search
  - Files affected: `crates/impulse-files/src/search.rs`
  - Estimated hours: 4

- [ ] **Task 5.4**: Filter by uploader
  - Implementation notes: Search by username
  - Files affected: `crates/impulse-files/src/search.rs`
  - Estimated hours: 2

- [ ] **Task 5.5**: Filter by date range
  - Implementation notes: Upload date between start and end dates
  - Files affected: `crates/impulse-files/src/search.rs`
  - Estimated hours: 3

- [ ] **Task 5.6**: Filter by file size
  - Implementation notes: Size range (e.g., 1-10MB)
  - Files affected: `crates/impulse-files/src/search.rs`
  - Estimated hours: 2

### Task Category 6: Testing

- [ ] **Task 6.1**: Create test file areas
  - Implementation notes: Sample areas with various file types
  - Files affected: `tests/fixtures/file_areas/`
  - Estimated hours: 3

- [ ] **Task 6.2**: Test area navigation
  - Implementation notes: Switch areas, verify permissions
  - Files affected: `tests/area_navigation_test.rs`
  - Estimated hours: 3

- [ ] **Task 6.3**: Test file list rendering
  - Implementation notes: Various list sizes, pagination, sorting
  - Files affected: `tests/file_list_test.rs`
  - Estimated hours: 3

- [ ] **Task 6.4**: Test search functionality
  - Implementation notes: All search criteria, edge cases
  - Files affected: `tests/file_search_test.rs`
  - Estimated hours: 4

- [ ] **Task 6.5**: Integration test for file browsing
  - Implementation notes: Full flow: select area → browse → search → view details
  - Files affected: `tests/file_browsing_integration_test.rs`
  - Estimated hours: 4

---

## Technical Details

### Architecture Considerations

- Index file metadata in database for fast searching
- Keep file system in sync with database
- Support offline files (in database but not on disk)
- Handle missing files gracefully

### Dependencies

**Crate-Level Dependencies:**
```toml
[dependencies]
tokio = { workspace = true }
sqlx = { workspace = true }
zip = "0.6"
chrono = "0.4"
humansize = "2.1"
wildmatch = "2.1"
```

### Code Patterns

**File Area Management:**
```rust
pub struct FileAreaManager {
    storage: Arc<dyn Storage>,
}

impl FileAreaManager {
    pub async fn list_areas(&self, user_level: SecurityLevel) -> Result<Vec<FileArea>> {
        let areas = self.storage.get_all_file_areas().await?;
        Ok(areas.into_iter()
            .filter(|a| a.security_level <= user_level)
            .collect())
    }

    pub async fn get_files(&self, area_id: u32, page: u32, page_size: u32)
        -> Result<(Vec<FileRecord>, u32)> {
        let offset = page * page_size;
        let files = self.storage.get_files_in_area(area_id, offset, page_size).await?;
        let total = self.storage.count_files_in_area(area_id).await?;
        Ok((files, total))
    }
}
```

**FILE_ID.DIZ Extraction:**
```rust
pub fn extract_file_id_diz(path: &Path) -> Result<Option<String>> {
    let file = File::open(path)?;
    let mut archive = ZipArchive::new(file)?;

    // Look for FILE_ID.DIZ, DESC.SDI, or similar
    for name in &["FILE_ID.DIZ", "file_id.diz", "DESC.SDI", "desc.sdi"] {
        if let Ok(mut entry) = archive.by_name(name) {
            let mut content = String::new();
            entry.read_to_string(&mut content)?;
            return Ok(Some(content));
        }
    }

    Ok(None)
}
```

---

## Dependencies

### Upstream Dependencies
- **Sprint 04**: Storage layer for file metadata
- **Sprint 09**: User authentication for access control
- **Sprint 07**: Terminal I/O for rendering screens

### Blocks Downstream
- **Sprint 14**: File upload requires area management
- **Sprint 17-18**: Download requires file browsing
- **Sprint 23**: Admin interface needs file area management

---

## Acceptance Criteria

- [ ] File areas are navigable
- [ ] File list displays correctly with pagination
- [ ] Sorting works for all columns
- [ ] Search returns relevant results
- [ ] FILE_ID.DIZ is displayed when available
- [ ] Permissions are enforced (users only see allowed areas)
- [ ] Performance is acceptable for areas with 10,000+ files
- [ ] Missing files are handled gracefully

---

## Testing Requirements

### Unit Tests
- [ ] File area permissions
- [ ] Search algorithms
- [ ] FILE_ID.DIZ extraction
- [ ] File size formatting
- [ ] Wildcard matching

### Integration Tests
- [ ] Area navigation flow
- [ ] File list pagination
- [ ] Search with multiple criteria
- [ ] Details screen display

### Performance Tests
- [ ] List rendering with large file counts
- [ ] Search performance on large datasets
- [ ] Database query optimization

---

## Notes and Decisions

### Design Decisions
*To be populated during sprint*

**Proposed Decisions:**
- Default page size: 20 files per page
- Support ZIP, RAR, 7Z, ARJ, LZH archive formats
- Cache FILE_ID.DIZ in database after first extraction
- Default sort: newest files first
- Show "(Offline)" indicator for missing files

### Lessons Learned
*To be populated during sprint*

### Risks and Mitigations
- **Risk**: Database and filesystem may become out of sync
- **Mitigation**: Periodic sync job, manual rescan command for SysOps
- **Risk**: FILE_ID.DIZ extraction may be slow
- **Mitigation**: Cache in database, extract async, background processing
- **Risk**: Large file areas may cause slow queries
- **Mitigation**: Add database indices, implement query pagination

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
