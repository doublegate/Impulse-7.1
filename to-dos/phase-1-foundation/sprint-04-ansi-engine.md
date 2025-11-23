# Sprint 04: Storage Layer Foundation

**Phase:** Phase 1 - Foundation
**Duration:** 3 weeks
**Sprint Dates:** TBD
**Status:** Not Started

---

## Sprint Overview

Sprint 04 establishes the storage abstraction layer and implements the initial database backend using SQLite. This sprint creates the foundation for all data persistence in Impulse 7.1, including user accounts, file metadata, messages, and configuration. The abstraction allows for future database backend changes while maintaining a consistent API.

**Context:** This is the fourth sprint of Phase 1 (Foundation). The storage layer will be used by all features requiring data persistence.

**Expected Outcomes:** By the end of this sprint, the project will have a fully functional storage layer with SQLite backend, complete with schema migrations and comprehensive CRUD operations for the User table.

---

## Objectives

- [ ] Implement storage abstraction trait for data persistence
- [ ] Create SQLite backend implementation using sqlx
- [ ] Implement database schema with migration system
- [ ] Complete basic CRUD operations for User table

---

## Deliverables

| Deliverable | Type | Acceptance Criteria |
|-------------|------|---------------------|
| `impulse-storage` crate | Code | Crate compiles with Storage trait defined |
| Storage trait definition | Code | Trait covers User, File, Message operations (placeholders OK for Files/Messages) |
| SQLite implementation | Code | SqliteStorage implements all Storage trait methods |
| Database schema | Code | SQL schema files with users, files, messages, sessions tables |
| Migration system | Code | Migrations run successfully up and down without data loss |

---

## Detailed Tasks

### Task Category 1: Storage Trait Design

- [ ] **Task 1.1**: Define Storage trait interface
  - Implementation notes: Async trait with methods for CRUD operations on all entity types
  - Files affected: `crates/impulse-storage/src/traits.rs`
  - Estimated hours: 4

- [ ] **Task 1.2**: Define User operations
  - Implementation notes: get_user, create_user, update_user, delete_user, find_by_username
  - Files affected: `crates/impulse-storage/src/traits.rs`
  - Estimated hours: 2

- [ ] **Task 1.3**: Define File operations (placeholders)
  - Implementation notes: Basic signatures for get_file, list_files, add_file
  - Files affected: `crates/impulse-storage/src/traits.rs`
  - Estimated hours: 1

- [ ] **Task 1.4**: Define Message operations (placeholders)
  - Implementation notes: Basic signatures for get_message, list_messages, post_message
  - Files affected: `crates/impulse-storage/src/traits.rs`
  - Estimated hours: 1

### Task Category 2: SQLite Backend Implementation

- [ ] **Task 2.1**: Set up SQLite connection pool
  - Implementation notes: Use sqlx::SqlitePool, configure pool size and timeouts
  - Files affected: `crates/impulse-storage/src/sqlite/mod.rs`
  - Estimated hours: 3

- [ ] **Task 2.2**: Implement User CRUD operations
  - Implementation notes: Write SQL queries for all User operations
  - Files affected: `crates/impulse-storage/src/sqlite/users.rs`
  - Estimated hours: 6

- [ ] **Task 2.3**: Implement query error handling
  - Implementation notes: Convert sqlx errors to storage Error type
  - Files affected: `crates/impulse-storage/src/error.rs`
  - Estimated hours: 2

- [ ] **Task 2.4**: Add connection retry logic
  - Implementation notes: Handle transient database errors with exponential backoff
  - Files affected: `crates/impulse-storage/src/sqlite/mod.rs`
  - Estimated hours: 3

### Task Category 3: Database Schema Design

- [ ] **Task 3.1**: Design users table schema
  - Implementation notes: id, username, password_hash, security_level, stats, timestamps
  - Files affected: `migrations/001_create_users.sql`
  - Estimated hours: 2

- [ ] **Task 3.2**: Design files table schema (basic)
  - Implementation notes: id, filename, description, uploader_id, size, date, area_id
  - Files affected: `migrations/002_create_files.sql`
  - Estimated hours: 2

- [ ] **Task 3.3**: Design messages table schema (basic)
  - Implementation notes: id, from_user, to_user, subject, body, parent_id, area_id, timestamps
  - Files affected: `migrations/003_create_messages.sql`
  - Estimated hours: 2

- [ ] **Task 3.4**: Design sessions table schema
  - Implementation notes: id, user_id, token, ip_address, login_time, last_activity
  - Files affected: `migrations/004_create_sessions.sql`
  - Estimated hours: 2

- [ ] **Task 3.5**: Add indexes for common queries
  - Implementation notes: Index on username, user_id foreign keys, timestamps
  - Files affected: Migration files
  - Estimated hours: 2

### Task Category 4: Migration System

- [ ] **Task 4.1**: Set up sqlx migration infrastructure
  - Implementation notes: Use sqlx::migrate!() macro, configure migrations directory
  - Files affected: `crates/impulse-storage/src/sqlite/mod.rs`
  - Estimated hours: 2

- [ ] **Task 4.2**: Create migration runner
  - Implementation notes: Function to run all pending migrations on startup
  - Files affected: `crates/impulse-storage/src/sqlite/migrations.rs`
  - Estimated hours: 2

- [ ] **Task 4.3**: Write rollback migrations
  - Implementation notes: DOWN migrations for all schema changes
  - Files affected: All migration files
  - Estimated hours: 3

- [ ] **Task 4.4**: Test migration up/down cycles
  - Implementation notes: Verify data integrity through migration cycles
  - Files affected: `tests/migrations_test.rs`
  - Estimated hours: 4

### Task Category 5: Testing and Integration

- [ ] **Task 5.1**: Write unit tests for User operations
  - Implementation notes: Test all CRUD operations with various inputs
  - Files affected: `tests/user_storage_test.rs`
  - Estimated hours: 5

- [ ] **Task 5.2**: Write integration tests for storage layer
  - Implementation notes: Test complete workflows (create → read → update → delete)
  - Files affected: `tests/integration_test.rs`
  - Estimated hours: 4

- [ ] **Task 5.3**: Test concurrent access scenarios
  - Implementation notes: Multiple simultaneous operations, connection pool behavior
  - Files affected: `tests/concurrency_test.rs`
  - Estimated hours: 4

- [ ] **Task 5.4**: Add documentation and examples
  - Implementation notes: Rustdoc comments, usage examples in docs
  - Files affected: All source files, `README.md`
  - Estimated hours: 3

---

## Technical Details

### Architecture Considerations

- Use async trait (`#[async_trait]`) for all Storage operations
- Design for future database backends (PostgreSQL, MySQL)
- Use connection pooling to handle concurrent requests
- Implement repository pattern for clean separation

### Dependencies

**Crate-Level Dependencies:**
```toml
[dependencies]
sqlx = { version = "0.7", features = ["runtime-tokio", "sqlite", "migrate"] }
tokio = { workspace = true }
anyhow = { workspace = true }
thiserror = { workspace = true }
async-trait = "0.1"

[dev-dependencies]
tempfile = "3.8"  # For test databases
```

### Code Patterns

**Storage Trait Pattern:**
```rust
use async_trait::async_trait;

#[async_trait]
pub trait Storage: Send + Sync {
    async fn get_user(&self, id: UserId) -> Result<Option<User>>;
    async fn create_user(&self, user: &User) -> Result<UserId>;
    async fn update_user(&self, user: &User) -> Result<()>;
    async fn delete_user(&self, id: UserId) -> Result<()>;
    async fn find_user_by_username(&self, username: &str) -> Result<Option<User>>;
}
```

**SQLite Implementation Pattern:**
```rust
pub struct SqliteStorage {
    pool: SqlitePool,
}

#[async_trait]
impl Storage for SqliteStorage {
    async fn get_user(&self, id: UserId) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE id = ?",
            id.0
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(user)
    }
}
```

---

## Dependencies

### Upstream Dependencies
- **Sprint 02**: Requires core type definitions (User, Error types)

### Blocks Downstream
- **Sprint 06**: Session management requires storage layer
- **Sprint 09**: User authentication requires user storage
- **Sprint 11**: Message reading requires storage trait
- **Sprint 13**: File areas require file storage

---

## Acceptance Criteria

- [ ] All Storage trait methods implemented for SQLite
- [ ] Integration tests for CRUD operations pass
- [ ] Schema migrations work correctly (up and down)
- [ ] No data loss during migration cycles
- [ ] Connection pool handles concurrent access
- [ ] Error handling is comprehensive with clear messages
- [ ] Documentation covers all public APIs

---

## Testing Requirements

### Unit Tests
- [ ] Each CRUD operation tested individually
- [ ] Error cases covered (duplicate username, not found, etc.)
- [ ] SQL query construction verified

### Integration Tests
- [ ] Complete user lifecycle (create → read → update → delete)
- [ ] Find operations return correct results
- [ ] Transactions work correctly

### Concurrency Tests
- [ ] Multiple simultaneous reads/writes
- [ ] Connection pool behavior under load
- [ ] Race condition scenarios

### Migration Tests
- [ ] Fresh database creation
- [ ] Upgrade from version N to N+1
- [ ] Rollback from version N to N-1
- [ ] Data integrity through migration cycles

---

## Notes and Decisions

### Design Decisions
*To be populated during sprint*

**Proposed Decisions:**
- Use SQLite for initial implementation (simple, no server required)
- Use sqlx for compile-time query verification
- Implement repository pattern for testability
- Use async/await throughout for future scalability

### Lessons Learned
*To be populated during sprint*

### Risks and Mitigations
- **Risk**: SQLite may not scale for high-traffic deployments
- **Mitigation**: Storage trait allows swapping to PostgreSQL later
- **Risk**: Schema changes may break existing data
- **Mitigation**: Comprehensive migration testing, backup recommendations
- **Risk**: Connection pool exhaustion under load
- **Mitigation**: Configure appropriate pool size, implement connection timeouts

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
