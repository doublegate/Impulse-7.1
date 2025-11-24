# Sprint 6 Implementation Plan: User System & Authentication

**Project:** Impulse-Next_BBS
**Sprint:** 6 of 32
**Phase:** 1 (Foundation)
**Date:** 2025-11-23
**Status:** In Progress

---

## Executive Summary

Sprint 6 implements the complete user management system and authentication layer for Impulse-Next_BBS. This sprint builds upon Sprint 5's Pascal type foundations (particularly the PascalUserRec structure) to create a production-ready user system with modern security practices.

### Scope Overview

1. **User System** (Priority 1): Full userrec implementation with CRUD operations
2. **Authentication Layer** (Priority 2): Argon2 password hashing and session management
3. **File Parsing** (Priority 3 - DEFERRED): Deferred to Sprint 7 based on workload

### Key Metrics

- **Estimated New Code**: 2,000-2,500 lines
- **Estimated Tests**: 80-100 new tests
- **Target Test Count**: 224 → 304-324 (36-44% increase)
- **Files to Create**: ~15-20 new files
- **Files to Modify**: ~8-10 existing files

---

## Current State Analysis

### Existing Foundation (Sprint 5)

#### PascalUserRec Structure (pascal_user.rs)
**Status**: ✅ COMPLETE (535 lines, 60+ fields, 11 tests)

The complete userrec type from RECORDS.PAS is **already fully implemented** in `crates/impulse-types/src/pascal_user.rs`. Analysis shows:

**Implemented Fields** (All 60+ fields):
- Identity: name, realname, pw, ph, bday (strings with PascalString<N>)
- Dates: firston, laston (string[8] format)
- Address: street, citystate, zipcode
- User data: autosig, note, sex
- Statistics: ttimeon, uploads, downloads, uk, dk, loggedon, msgpost, emailsent
- Security: sl (security level), dsl (download security level), lockedout, deleted
- Flags: ac (UserFlags), fflag (FileListFlags), ar (ArFlags), zzdlnscn (DownloadScanFlags)
- Arrays: zzqscan[64], xqxxx[64], zzqscn[64], boardsysop[5]
- Preferences: linelen, pagelen, prompt, clsmsg, flistopt, msgorder, avadjust
- Financial: credit, filepoints, timebank, timebankadd
- Activity tracking: waiting, ontoday, illegal, tltoday
- Reserved: x1xs, x2xs, unused, unused2, unused3, x1xx, x2xx, x3xx, x4xx

**Binary Compatibility**: Full binrw derives for Pascal .DAT file format
**Tests**: 11 tests covering construction, getters, ratio calculations, sysop checks

**Gap Assessment**: The userrec type is complete, but missing:
1. ❌ User validation logic (username constraints, email validation)
2. ❌ Business logic methods (promotion, demotion, quota checks)
3. ❌ Modern Rust wrapper (separate from Pascal binary format)
4. ❌ CRUD operations in impulse-user crate

#### UserFlags (user_flags.rs)
**Status**: ✅ COMPLETE (340 lines)

All 24 Pascal user flags implemented as bitflags with:
- Pascal byte array conversion (to/from 3-byte format)
- Helper methods (is_restricted(), has_display_preference())
- Comprehensive rustdoc comments

#### Empty Crates
**Status**: ⚠️ STUB IMPLEMENTATIONS

1. **impulse-user** (1 test): Empty stub, ready for implementation
2. **impulse-auth** (1 test): Empty stub, ready for implementation

---

## Implementation Strategy

### Decision: Focus on Modern Rust Layer

Given that PascalUserRec is already complete, Sprint 6 will focus on:

1. **Modern User API** (NOT extending PascalUserRec)
   - Create `User` struct (modern Rust type, separate from Pascal format)
   - Conversion layer: User ↔ PascalUserRec (for .DAT file I/O)
   - Business logic in User methods, keep PascalUserRec as pure data

2. **User Management** (impulse-user crate)
   - UserManager trait (abstract interface)
   - InMemoryUserManager (development, testing)
   - FileUserManager (USER.LST .DAT file I/O using PascalUserRec)
   - CRUD operations (create, read, update, delete, search)

3. **Authentication** (impulse-auth crate)
   - Password hashing (Argon2id with configurable params)
   - Password verification
   - Legacy password migration (MD5 → Argon2)
   - Session token generation (UUID v4)
   - Session validation

4. **Testing Strategy**
   - Unit tests for each module (50+ tests)
   - Integration tests for workflows (20+ tests)
   - Serialization round-trips (10+ tests)
   - Binary compatibility tests (10+ tests)

---

## Detailed Implementation Tasks

### Phase 2: User System (Priority 1)

#### Task 2.1: Create Modern User Type
**File**: `crates/impulse-types/src/user.rs` (NEW)
**Lines**: ~300
**Tests**: 15

```rust
/// Modern User type (separate from Pascal binary format)
pub struct User {
    pub id: UserId,              // Modern UUID-based ID
    pub username: String,        // Validated username (3-36 chars)
    pub real_name: Option<String>,
    pub email: Option<String>,   // Validated email
    pub security_level: SecurityLevel,
    pub flags: UserFlags,
    pub stats: UserStats,        // Nested struct for statistics
    pub preferences: UserPreferences,
    pub created_at: SystemTime,
    pub last_login: Option<SystemTime>,
    pub is_active: bool,
}

impl User {
    pub fn new(username: impl AsRef<str>) -> Result<Self>;
    pub fn validate(&self) -> Result<()>;
    pub fn promote(&mut self) -> Result<()>;
    pub fn demote(&mut self) -> Result<()>;
    pub fn is_sysop(&self) -> bool;
    pub fn can_upload(&self) -> bool;
    pub fn can_download(&self) -> bool;
    pub fn can_post(&self) -> bool;
    pub fn get_upload_quota(&self) -> u64;
    pub fn get_download_quota(&self) -> u64;
}
```

**Deliverables**:
- User struct with all fields
- Validation methods (username, email, security level)
- Business logic methods (promotion, quotas, permissions)
- Conversion: User ↔ PascalUserRec
- 15 unit tests

---

#### Task 2.2: SecurityLevel Type
**File**: `crates/impulse-types/src/security.rs` (NEW)
**Lines**: ~150
**Tests**: 8

```rust
/// Security level (0-255, higher = more access)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SecurityLevel(u8);

impl SecurityLevel {
    pub const MIN: u8 = 0;
    pub const NEW_USER: u8 = 10;
    pub const VALIDATED: u8 = 50;
    pub const PRIVILEGED: u8 = 100;
    pub const COSYSOP: u8 = 200;
    pub const SYSOP: u8 = 255;

    pub fn new(level: u8) -> Self;
    pub fn can_access(&self, required: SecurityLevel) -> bool;
    pub fn is_sysop(&self) -> bool;
}
```

**Deliverables**:
- SecurityLevel newtype with validation
- Standard levels as constants
- Comparison and access check methods
- 8 unit tests

---

#### Task 2.3: UserStats and UserPreferences
**File**: `crates/impulse-types/src/user_stats.rs` (NEW)
**File**: `crates/impulse-types/src/user_prefs.rs` (NEW)
**Lines**: ~200 each
**Tests**: 10 each

```rust
/// User statistics (nested struct for organization)
pub struct UserStats {
    pub total_time_minutes: u32,
    pub uploads: u16,
    pub downloads: u16,
    pub upload_kb: u32,
    pub download_kb: u32,
    pub posts: u16,
    pub emails_sent: u16,
    pub logins: u16,
    pub file_points: i16,
}

/// User preferences
pub struct UserPreferences {
    pub line_length: u8,      // Terminal columns (default: 80)
    pub page_length: u8,      // Terminal rows (default: 24)
    pub ansi_enabled: bool,
    pub color_enabled: bool,
    pub pause_enabled: bool,
    pub one_key_mode: bool,
    pub avatar_enabled: bool,
}
```

**Deliverables**:
- UserStats struct with calculation methods (ul/dl ratio, etc.)
- UserPreferences struct with defaults
- 20 unit tests total

---

#### Task 2.4: UserManager Trait
**File**: `crates/impulse-user/src/manager.rs` (NEW)
**Lines**: ~200
**Tests**: 0 (trait definition)

```rust
#[async_trait]
pub trait UserManager: Send + Sync {
    async fn create(&self, user: User) -> Result<UserId>;
    async fn get(&self, id: UserId) -> Result<Option<User>>;
    async fn get_by_username(&self, username: &str) -> Result<Option<User>>;
    async fn update(&self, user: &User) -> Result<()>;
    async fn delete(&self, id: UserId) -> Result<()>;
    async fn list(&self, limit: usize, offset: usize) -> Result<Vec<User>>;
    async fn search(&self, query: &str) -> Result<Vec<User>>;
    async fn count(&self) -> Result<usize>;
}
```

**Deliverables**:
- UserManager trait with async methods
- Comprehensive rustdoc with usage examples

---

#### Task 2.5: InMemoryUserManager
**File**: `crates/impulse-user/src/memory.rs` (NEW)
**Lines**: ~250
**Tests**: 15

```rust
/// In-memory user storage (for development and testing)
pub struct InMemoryUserManager {
    users: Arc<RwLock<HashMap<UserId, User>>>,
    username_index: Arc<RwLock<HashMap<String, UserId>>>,
}
```

**Deliverables**:
- Full UserManager implementation using HashMap
- Username uniqueness enforcement
- Search by username (case-insensitive)
- 15 integration tests

---

#### Task 2.6: FileUserManager (Pascal .DAT)
**File**: `crates/impulse-user/src/file.rs` (NEW)
**Lines**: ~350
**Tests**: 12

```rust
/// User storage using Pascal USER.LST format
pub struct FileUserManager {
    file_path: PathBuf,
}

impl FileUserManager {
    pub async fn load_all(&self) -> Result<Vec<PascalUserRec>>;
    pub async fn save_all(&self, users: &[PascalUserRec]) -> Result<()>;

    // Converts between User and PascalUserRec internally
}
```

**Deliverables**:
- UserManager implementation using PascalUserRec I/O
- Conversion layer (User ↔ PascalUserRec)
- Binary file handling with error recovery
- 12 tests (including binary compatibility)

---

### Phase 4: Authentication Layer (Priority 2)

#### Task 4.1: Password Hashing
**File**: `crates/impulse-auth/src/password.rs` (NEW)
**Lines**: ~300
**Tests**: 20

```rust
pub struct PasswordHasher {
    params: Argon2Params,
}

impl PasswordHasher {
    pub fn new() -> Self;
    pub fn with_params(params: Argon2Params) -> Self;

    pub fn hash(&self, password: &str) -> Result<String>;
    pub fn verify(&self, password: &str, hash: &str) -> Result<bool>;
    pub fn needs_rehash(&self, hash: &str) -> bool;

    // Legacy migration
    pub fn verify_legacy_md5(&self, password: &str, md5_hash: &str) -> Result<bool>;
    pub fn migrate_from_md5(&self, password: &str, md5_hash: &str) -> Result<Option<String>>;
}
```

**Configuration**:
- Algorithm: Argon2id (secure against GPU/ASIC attacks)
- Memory: 64 MiB (configurable)
- Iterations: 3 (configurable)
- Parallelism: 4 threads (configurable)
- Salt: 16 bytes (random)

**Deliverables**:
- Argon2id password hashing
- Configurable parameters (via BbsConfig)
- Legacy MD5 migration support
- 20 unit tests (hash, verify, migration)

---

#### Task 4.2: Session Management
**File**: `crates/impulse-auth/src/session.rs` (NEW)
**Lines**: ~250
**Tests**: 15

```rust
pub struct Session {
    pub id: SessionId,           // UUID v4
    pub user_id: UserId,
    pub created_at: SystemTime,
    pub expires_at: SystemTime,
    pub last_activity: SystemTime,
    pub ip_address: Option<IpAddr>,
}

pub struct SessionManager {
    sessions: Arc<RwLock<HashMap<SessionId, Session>>>,
    timeout: Duration,
}

impl SessionManager {
    pub async fn create(&self, user_id: UserId) -> Result<Session>;
    pub async fn get(&self, id: SessionId) -> Result<Option<Session>>;
    pub async fn validate(&self, id: SessionId) -> Result<bool>;
    pub async fn refresh(&self, id: SessionId) -> Result<()>;
    pub async fn revoke(&self, id: SessionId) -> Result<()>;
    pub async fn revoke_all_for_user(&self, user_id: UserId) -> Result<()>;
    pub async fn cleanup_expired(&self) -> Result<usize>;
}
```

**Deliverables**:
- Session struct with UUID-based IDs
- SessionManager with timeout handling
- Automatic expiration cleanup
- Multi-session support (same user, multiple logins)
- 15 tests (create, validate, refresh, revoke, cleanup)

---

#### Task 4.3: Authentication Service
**File**: `crates/impulse-auth/src/auth.rs` (NEW)
**Lines**: ~200
**Tests**: 10

```rust
pub struct AuthService {
    password_hasher: PasswordHasher,
    session_manager: SessionManager,
}

impl AuthService {
    pub async fn login(
        &self,
        username: &str,
        password: &str,
        user_manager: &dyn UserManager,
    ) -> Result<Session>;

    pub async fn logout(&self, session_id: SessionId) -> Result<()>;

    pub async fn validate_session(&self, session_id: SessionId) -> Result<Option<UserId>>;

    pub async fn change_password(
        &self,
        user_id: UserId,
        old_password: &str,
        new_password: &str,
        user_manager: &dyn UserManager,
    ) -> Result<()>;
}
```

**Deliverables**:
- Complete authentication workflow
- Login with automatic password migration (MD5 → Argon2)
- Session validation
- Password change with old password verification
- 10 integration tests

---

#### Task 4.4: Security Utilities
**File**: `crates/impulse-auth/src/security.rs` (NEW)
**Lines**: ~200
**Tests**: 10

```rust
pub struct PasswordStrength {
    pub score: u8,              // 0-100
    pub has_lowercase: bool,
    pub has_uppercase: bool,
    pub has_digits: bool,
    pub has_special: bool,
    pub length: usize,
}

pub fn assess_password_strength(password: &str) -> PasswordStrength;
pub fn is_password_secure(password: &str) -> bool;
pub fn generate_random_password(length: usize) -> String;

pub struct RateLimiter {
    attempts: Arc<RwLock<HashMap<String, Vec<SystemTime>>>>,
    max_attempts: usize,
    window: Duration,
}

impl RateLimiter {
    pub async fn check(&self, key: &str) -> Result<()>;
    pub async fn record_attempt(&self, key: &str);
    pub async fn reset(&self, key: &str);
}
```

**Deliverables**:
- Password strength assessment
- Random password generation
- Rate limiting for authentication (5 attempts per 15 minutes)
- 10 tests

---

### Phase 5: Comprehensive Testing (80+ new tests)

#### Test Categories

1. **Unit Tests** (50 tests)
   - User construction and validation (10)
   - SecurityLevel operations (8)
   - UserStats calculations (10)
   - Password hashing/verification (12)
   - Session lifecycle (10)

2. **Integration Tests** (20 tests)
   - Full authentication flow (5)
   - User CRUD operations (8)
   - Password migration (3)
   - Rate limiting (4)

3. **Serialization Tests** (10 tests)
   - User ↔ PascalUserRec conversion (5)
   - JSON round-trips (3)
   - Binary .DAT file I/O (2)

4. **Edge Case Tests** (10 tests)
   - Empty usernames (2)
   - Password edge cases (3)
   - Session expiration (2)
   - Concurrent access (3)

**Total**: 90 new tests

---

## Test Implementation Strategy

### Test File Organization

```
crates/impulse-types/tests/
├── user_tests.rs              (15 tests)
├── security_tests.rs          (8 tests)
├── user_stats_tests.rs        (10 tests)
└── serialization_tests.rs     (10 tests)

crates/impulse-user/tests/
├── memory_manager_tests.rs    (15 tests)
├── file_manager_tests.rs      (12 tests)
└── integration_tests.rs       (8 tests)

crates/impulse-auth/tests/
├── password_tests.rs          (20 tests)
├── session_tests.rs           (15 tests)
├── auth_service_tests.rs      (10 tests)
└── security_tests.rs          (10 tests)
```

---

## File Modification Plan

### Files to Create (18 new files)

**impulse-types crate** (4 files):
1. `src/user.rs` - Modern User type (~300 lines)
2. `src/security.rs` - SecurityLevel (~150 lines)
3. `src/user_stats.rs` - UserStats (~200 lines)
4. `src/user_prefs.rs` - UserPreferences (~200 lines)

**impulse-user crate** (4 files):
5. `src/manager.rs` - UserManager trait (~200 lines)
6. `src/memory.rs` - InMemoryUserManager (~250 lines)
7. `src/file.rs` - FileUserManager (~350 lines)
8. `src/error.rs` - User-specific errors (~100 lines)

**impulse-auth crate** (5 files):
9. `src/password.rs` - Password hashing (~300 lines)
10. `src/session.rs` - Session management (~250 lines)
11. `src/auth.rs` - AuthService (~200 lines)
12. `src/security.rs` - Security utilities (~200 lines)
13. `src/error.rs` - Auth-specific errors (~100 lines)

**Test files** (5 files):
14. `crates/impulse-types/tests/user_tests.rs` (15 tests)
15. `crates/impulse-user/tests/integration_tests.rs` (8 tests)
16. `crates/impulse-auth/tests/password_tests.rs` (20 tests)
17. `crates/impulse-auth/tests/session_tests.rs` (15 tests)
18. `crates/impulse-auth/tests/auth_service_tests.rs` (10 tests)

### Files to Modify (4 files)

1. `crates/impulse-types/src/lib.rs` - Add new module exports
2. `crates/impulse-user/src/lib.rs` - Replace stub with real API
3. `crates/impulse-auth/src/lib.rs` - Replace stub with real API
4. `crates/impulse-types/src/pascal_user.rs` - Add conversion methods

---

## Code Size Estimates

| Module | Files | Lines | Tests |
|--------|-------|-------|-------|
| User types (impulse-types) | 4 | 850 | 43 |
| User management (impulse-user) | 4 | 900 | 35 |
| Authentication (impulse-auth) | 5 | 1050 | 55 |
| Test infrastructure | 5 | 500 | - |
| **Total** | **18** | **3300** | **133** |

**Note**: Estimates include comprehensive rustdoc comments and examples.

---

## Dependencies

### New Dependencies

**None required** - All dependencies already in workspace:
- ✅ argon2 = "0.5" (already in Cargo.toml)
- ✅ sha2 = "0.10" (already in Cargo.toml)
- ✅ tokio (async runtime)
- ✅ async-trait (trait abstractions)
- ✅ uuid (session IDs) - Need to add to workspace
- ✅ rand (password generation, salts) - Need to add to workspace

### Workspace Cargo.toml Updates

```toml
[workspace.dependencies]
uuid = { version = "1.6", features = ["v4", "serde"] }
rand = "0.8"
```

---

## Risk Assessment

### Technical Risks

1. **Risk**: Binary compatibility with Pascal .DAT files
   - **Mitigation**: Comprehensive round-trip tests, use existing PascalUserRec
   - **Severity**: Medium
   - **Likelihood**: Low (PascalUserRec already tested)

2. **Risk**: Performance of Argon2 hashing
   - **Mitigation**: Configurable parameters, benchmark tests
   - **Severity**: Low
   - **Likelihood**: Low (Argon2 is designed for server use)

3. **Risk**: Session memory leak
   - **Mitigation**: Automatic cleanup task, expiration tests
   - **Severity**: Medium
   - **Likelihood**: Low (proper cleanup implementation)

### Schedule Risks

1. **Risk**: Scope larger than expected (90 vs 80 tests)
   - **Mitigation**: Tests are parallelizable, defer file parsing
   - **Severity**: Low
   - **Likelihood**: Medium

2. **Risk**: Authentication edge cases require iteration
   - **Mitigation**: Follow established patterns, comprehensive tests
   - **Severity**: Low
   - **Likelihood**: Low

---

## Success Criteria

### Functional Requirements

- ✅ Modern User type with validation
- ✅ SecurityLevel with permission checking
- ✅ UserStats and UserPreferences types
- ✅ InMemoryUserManager (CRUD operations)
- ✅ FileUserManager (Pascal .DAT I/O)
- ✅ Argon2 password hashing
- ✅ Session management with timeout
- ✅ AuthService with login/logout
- ✅ Password strength validation
- ✅ Rate limiting for auth attempts

### Quality Requirements

- ✅ 80+ new tests (target: 90+)
- ✅ 0 clippy warnings
- ✅ All builds succeed (Linux, macOS, Windows)
- ✅ 100% test pass rate
- ✅ Comprehensive rustdoc comments
- ✅ Binary compatibility verified

### Documentation Requirements

- ✅ README.md updated (sprint status, test count)
- ✅ CHANGELOG.md updated (all new types/modules)
- ✅ CLAUDE.local.md updated (sprint completion)
- ✅ Comprehensive completion report
- ✅ Rustdoc examples for all public APIs

---

## Timeline Estimate

### Phase Breakdown

| Phase | Duration | Tasks |
|-------|----------|-------|
| Analysis & Planning | 0.5 hours | ✅ Complete |
| User Types (2.1-2.3) | 2 hours | User, SecurityLevel, Stats, Prefs |
| User Management (2.4-2.6) | 3 hours | Manager trait, implementations |
| Authentication (4.1-4.3) | 3 hours | Password, sessions, AuthService |
| Security Utilities (4.4) | 1 hour | Validation, rate limiting |
| Testing | 3 hours | 90 tests across 3 crates |
| Quality Checks | 0.5 hours | Clippy, tests, build |
| Documentation | 1 hour | README, CHANGELOG, reports |
| **Total** | **14 hours** | Full sprint implementation |

---

## Deferred Work (Sprint 7+)

### File Parsing (Original Priority 3)

**Rationale**: User system scope is sufficient for one sprint (90 tests, 3,300 lines).

**Deferred to Sprint 7**:
- FILE.LST parser (Pascal file entries)
- File area management
- File search/filtering
- Estimated: 20 tests, ~800 lines

---

## Next Steps

1. ✅ **Implementation Plan Complete**
2. ⏭️ **Phase 2**: Implement User Types (user.rs, security.rs, stats, prefs)
3. ⏭️ **Phase 3**: Implement User Management (manager trait, implementations)
4. ⏭️ **Phase 4**: Implement Authentication (password, sessions, auth service)
5. ⏭️ **Phase 5**: Write Comprehensive Tests (90 tests)
6. ⏭️ **Phase 6**: Quality Checks (clippy, tests, build)
7. ⏭️ **Phase 7**: Update Documentation
8. ⏭️ **Phase 8**: Generate Completion Report

---

## Approval

**Plan Status**: ✅ READY FOR IMPLEMENTATION
**Estimated Completion**: Same session (14 hours total)
**Risk Level**: LOW
**Confidence**: HIGH (95%)

**Key Decision**: Modern User API separate from PascalUserRec maintains clean architecture while preserving binary compatibility.

---

**Document Version**: 1.0
**Last Updated**: 2025-11-23 (Analysis Phase)
