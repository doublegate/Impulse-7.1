# Sprint 28: Legacy Migration Tools

**Phase:** Phase 4 - Polish & Launch
**Duration:** 3 weeks
**Sprint Dates:** TBD
**Status:** Not Started

---

## Sprint Overview

Sprint 28 creates migration tools for importing data from Impulse 7.1 and other BBS systems (Renegade, Telegard). Includes data validation and migration documentation.

**Context:** Sprint 4 of Phase 4. Enables migration from legacy BBS systems.

**Expected Outcomes:** SysOps can migrate existing BBS data to the new system.

---

## Objectives

- [ ] Create import tools for Impulse 7.1 data formats
- [ ] Support other BBS format imports (Renegade, Telegard)
- [ ] Validate data integrity after migration
- [ ] Document migration process

---

## Deliverables

| Deliverable | Type | Acceptance Criteria |
|-------------|------|---------------------|
| `impulse-legacy` crate | Code | Import tools for multiple formats |
| CLI migration tool (impimport) | Binary | Command-line import utility |
| Migration validation | Code | Verify data integrity |
| Migration guide | Docs | Step-by-step instructions |

---

## Detailed Tasks

### Task Category 1: Impulse 7.1 Importer

- [ ] **Task 1.1**: Import USERS.DAT
  - Files affected: `crates/impulse-legacy/src/impulse71/users.rs`
  - Estimated hours: 8

- [ ] **Task 1.2**: Import file databases
  - Files affected: `crates/impulse-legacy/src/impulse71/files.rs`
  - Estimated hours: 6

- [ ] **Task 1.3**: Import message bases
  - Files affected: `crates/impulse-legacy/src/impulse71/messages.rs`
  - Estimated hours: 8

### Task Category 2: Other BBS Importers

- [ ] **Task 2.1**: Renegade user import
  - Files affected: `crates/impulse-legacy/src/renegade/users.rs`
  - Estimated hours: 6

- [ ] **Task 2.2**: Telegard user import
  - Files affected: `crates/impulse-legacy/src/telegard/users.rs`
  - Estimated hours: 6

### Task Category 3: Validation

- [ ] **Task 3.1**: Checksum verification
  - Files affected: `crates/impulse-legacy/src/validation/checksum.rs`
  - Estimated hours: 4

- [ ] **Task 3.2**: User count comparison
  - Files affected: `crates/impulse-legacy/src/validation/counts.rs`
  - Estimated hours: 2

- [ ] **Task 3.3**: Sample data spot-checks
  - Files affected: `crates/impulse-legacy/src/validation/spot_check.rs`
  - Estimated hours: 3

### Task Category 4: Documentation

- [ ] **Task 4.1**: Migration guide
  - Files affected: `docs/migration-guide.md`
  - Estimated hours: 6

- [ ] **Task 4.2**: Common issues & solutions
  - Files affected: `docs/migration-troubleshooting.md`
  - Estimated hours: 4

---

## Acceptance Criteria

- [ ] Impulse 7.1 data migrates successfully
- [ ] No data loss during migration
- [ ] Migrated systems functional
- [ ] Migration guide tested

---

## Technical Details

### Architecture Considerations

- Parse binary data formats from legacy BBS systems (Impulse 7.1, Renegade, Telegard)
- Validate data integrity during migration (checksums, record counts, field constraints)
- Transform legacy data structures to modern schema (users, messages, files)
- Provide CLI tool with progress reporting and rollback capability
- Support incremental migration (resume from failures)
- Generate migration reports with detailed statistics
- Preserve user passwords (migrate hashed passwords where possible)
- Handle encoding conversions (CP437 to UTF-8)

### Dependencies

**Crate-Level Dependencies:**
```toml
[dependencies]
tokio = { workspace = true }
sqlx = { workspace = true, features = ["postgres", "runtime-tokio-native-tls"] }
binrw = "0.13"
serde = { workspace = true }
anyhow = { workspace = true }
chrono = { workspace = true }
clap = { version = "4.4", features = ["derive"] }
indicatif = "0.17"
sha2 = "0.10"
encoding_rs = "0.8"
thiserror = "1.0"

[build-dependencies]
which = "5.0"
```

**Pascal Units Being Migrated:**
- USERS.PAS (User database - USERS.DAT)
- MSGBASE.PAS (Message base storage)
- FILEDB.PAS (File area database)
- CONFIG.PAS (System configuration)

**External Format Specifications:**
- Impulse 7.1 binary formats (proprietary)
- Renegade .DAT files (USERS.DAT, BOARDS.DAT)
- Telegard .BBS files (USER.BBS, MBOARD.BBS)

### Code Examples

**Impulse 7.1 Binary Format Parser:**
```rust
use binrw::{BinRead, BinWrite};
use chrono::{NaiveDate, NaiveDateTime};
use encoding_rs::WINDOWS_1252;
use std::path::PathBuf;

/// Impulse 7.1 user record (USERS.DAT format)
///
/// This matches the Pascal record structure from Impulse 7.1:
/// ```pascal
/// UserRecord = record
///   Name: String[30];
///   Password: String[15];
///   SecurityLevel: Byte;
///   // ... (55 fields total)
/// end;
/// ```
#[derive(Debug, Clone, BinRead, BinWrite)]
#[brw(little)]
pub struct Impulse71UserRecord {
    /// Pascal String[30] = length byte + 30 chars
    #[br(map = |b: u8| b)]
    name_len: u8,
    #[br(count = 30)]
    name_data: Vec<u8>,

    #[br(map = |b: u8| b)]
    password_len: u8,
    #[br(count = 15)]
    password_data: Vec<u8>,

    pub security_level: u8,

    #[br(map = |b: u8| b)]
    location_len: u8,
    #[br(count = 30)]
    location_data: Vec<u8>,

    #[br(map = |b: u8| b)]
    phone_len: u8,
    #[br(count = 15)]
    phone_data: Vec<u8>,

    /// Pascal Integer (i16)
    pub login_count: i16,

    /// Pascal LongInt (i32) - Julian date
    pub last_login_date: i32,

    /// Pascal LongInt (i32) - seconds since midnight
    pub last_login_time: i32,

    pub upload_kb: i32,
    pub download_kb: i32,
    pub total_uploads: i16,
    pub total_downloads: i16,

    /// Birthdate as Julian date
    pub birthdate: i32,

    pub ansi_enabled: u8,
    pub expert_mode: u8,
    pub page_length: u8,

    #[br(map = |b: u8| b)]
    alias_len: u8,
    #[br(count = 30)]
    alias_data: Vec<u8>,

    pub last_message_read: i32,
    pub files_downloaded_today: i16,
    pub kb_downloaded_today: i32,
    pub kb_uploaded_today: i32,

    /// Padding to match 256-byte record size
    #[br(count = 64)]
    _reserved: Vec<u8>,
}

impl Impulse71UserRecord {
    /// Parse Pascal string from length-prefixed byte array
    fn parse_pascal_string(len: u8, data: &[u8]) -> String {
        let bytes = &data[..len as usize];

        // Convert from CP437/Windows-1252 to UTF-8
        let (decoded, _, _) = WINDOWS_1252.decode(bytes);
        decoded.into_owned()
    }

    /// Extract user name
    pub fn name(&self) -> String {
        Self::parse_pascal_string(self.name_len, &self.name_data)
    }

    /// Extract password (hashed)
    pub fn password(&self) -> String {
        Self::parse_pascal_string(self.password_len, &self.password_data)
    }

    /// Extract location
    pub fn location(&self) -> String {
        Self::parse_pascal_string(self.location_len, &self.location_data)
    }

    /// Extract phone number
    pub fn phone(&self) -> String {
        Self::parse_pascal_string(self.phone_len, &self.phone_data)
    }

    /// Extract alias (if set)
    pub fn alias(&self) -> Option<String> {
        if self.alias_len > 0 {
            Some(Self::parse_pascal_string(self.alias_len, &self.alias_data))
        } else {
            None
        }
    }

    /// Convert Julian date to NaiveDate
    fn julian_to_date(julian: i32) -> Option<NaiveDate> {
        if julian == 0 {
            return None;
        }

        // Julian date conversion (Pascal epoch: 1/1/1980)
        let base = NaiveDate::from_ymd_opt(1980, 1, 1)?;
        base.checked_add_signed(chrono::Duration::days(julian as i64))
    }

    /// Get last login datetime
    pub fn last_login(&self) -> Option<NaiveDateTime> {
        let date = Self::julian_to_date(self.last_login_date)?;
        let time = chrono::Duration::seconds(self.last_login_time as i64);
        date.and_hms_opt(0, 0, 0)?
            .checked_add_signed(time)
    }

    /// Get birthdate
    pub fn birth_date(&self) -> Option<NaiveDate> {
        Self::julian_to_date(self.birthdate)
    }
}

/// Impulse 7.1 data importer
pub struct Impulse71Importer {
    data_dir: PathBuf,
}

impl Impulse71Importer {
    pub fn new(data_dir: PathBuf) -> Self {
        Self { data_dir }
    }

    /// Import all users from USERS.DAT
    pub async fn import_users(&self) -> anyhow::Result<Vec<Impulse71UserRecord>> {
        use binrw::BinReaderExt;
        use std::io::{BufReader, Seek};

        let users_path = self.data_dir.join("USERS.DAT");

        let file = std::fs::File::open(&users_path)
            .map_err(|e| anyhow::anyhow!("Failed to open {}: {}", users_path.display(), e))?;

        let mut reader = BufReader::new(file);
        let mut users = Vec::new();

        // Each record is exactly 256 bytes
        let record_size = 256u64;
        let mut record_num = 0u32;

        loop {
            // Seek to record position
            let pos = record_num as u64 * record_size;
            match reader.seek(std::io::SeekFrom::Start(pos)) {
                Ok(_) => {},
                Err(_) => break, // EOF
            }

            // Try to read record
            match reader.read_le::<Impulse71UserRecord>() {
                Ok(user) => {
                    // Skip deleted records (name_len == 0)
                    if user.name_len > 0 {
                        users.push(user);
                    }
                    record_num += 1;
                }
                Err(_) => break, // EOF or parse error
            }
        }

        Ok(users)
    }

    /// Calculate checksum for validation
    pub fn calculate_users_checksum(&self, users: &[Impulse71UserRecord]) -> String {
        use sha2::{Sha256, Digest};

        let mut hasher = Sha256::new();

        for user in users {
            hasher.update(user.name().as_bytes());
            hasher.update(&user.security_level.to_le_bytes());
            hasher.update(&user.login_count.to_le_bytes());
        }

        format!("{:x}", hasher.finalize())
    }
}

/// Message base record format
#[derive(Debug, Clone, BinRead, BinWrite)]
#[brw(little)]
pub struct Impulse71MessageHeader {
    pub area_id: i16,
    pub message_num: i32,

    #[br(map = |b: u8| b)]
    from_len: u8,
    #[br(count = 30)]
    from_data: Vec<u8>,

    #[br(map = |b: u8| b)]
    to_len: u8,
    #[br(count = 30)]
    to_data: Vec<u8>,

    #[br(map = |b: u8| b)]
    subject_len: u8,
    #[br(count = 60)]
    subject_data: Vec<u8>,

    pub posted_date: i32,
    pub posted_time: i32,

    pub reply_to: i32,
    pub is_private: u8,
    pub is_read: u8,

    /// Offset to message text in MESSAGES.TXT
    pub text_offset: i32,
    pub text_length: i32,

    #[br(count = 32)]
    _reserved: Vec<u8>,
}

impl Impulse71MessageHeader {
    pub fn from_user(&self) -> String {
        Impulse71UserRecord::parse_pascal_string(self.from_len, &self.from_data)
    }

    pub fn to_user(&self) -> String {
        Impulse71UserRecord::parse_pascal_string(self.to_len, &self.to_data)
    }

    pub fn subject(&self) -> String {
        Impulse71UserRecord::parse_pascal_string(self.subject_len, &self.subject_data)
    }
}
```

**Migration Validator:**
```rust
use anyhow::{Context, Result};
use sha2::{Sha256, Digest};
use std::path::PathBuf;
use sqlx::PgPool;

/// Migration validation report
#[derive(Debug, Clone)]
pub struct MigrationValidation {
    pub source: String,
    pub total_records: usize,
    pub migrated_records: usize,
    pub failed_records: usize,
    pub source_checksum: String,
    pub target_checksum: String,
    pub spot_checks: Vec<SpotCheck>,
    pub errors: Vec<String>,
}

impl MigrationValidation {
    /// Print validation summary
    pub fn print_summary(&self) {
        println!("\n=== Migration Validation Report ===");
        println!("Source: {}", self.source);
        println!("Total records: {}", self.total_records);
        println!("Migrated: {}", self.migrated_records);
        println!("Failed: {}", self.failed_records);
        println!("Success rate: {:.1}%",
            (self.migrated_records as f64 / self.total_records as f64) * 100.0);

        println!("\nChecksums:");
        println!("  Source: {}", self.source_checksum);
        println!("  Target: {}", self.target_checksum);

        if self.source_checksum == self.target_checksum {
            println!("  ✓ Checksums match - data integrity verified");
        } else {
            println!("  ✗ Checksums differ - manual review required");
        }

        if !self.spot_checks.is_empty() {
            println!("\nSpot Checks ({} samples):", self.spot_checks.len());
            for check in &self.spot_checks {
                if check.passed {
                    println!("  ✓ Record #{}: {}", check.record_num, check.description);
                } else {
                    println!("  ✗ Record #{}: {} - {}",
                        check.record_num, check.description, check.error.as_ref().unwrap());
                }
            }
        }

        if !self.errors.is_empty() {
            println!("\nErrors ({}):", self.errors.len());
            for (i, error) in self.errors.iter().take(10).enumerate() {
                println!("  {}. {}", i + 1, error);
            }
            if self.errors.len() > 10 {
                println!("  ... and {} more errors", self.errors.len() - 10);
            }
        }

        println!("===================================\n");
    }
}

#[derive(Debug, Clone)]
pub struct SpotCheck {
    pub record_num: usize,
    pub description: String,
    pub passed: bool,
    pub error: Option<String>,
}

/// Migration validator
pub struct MigrationValidator {
    db: PgPool,
}

impl MigrationValidator {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    /// Validate Impulse 7.1 user migration
    pub async fn validate_impulse71_users(
        &self,
        source_users: &[Impulse71UserRecord],
        source_checksum: String,
    ) -> Result<MigrationValidation> {
        let mut validation = MigrationValidation {
            source: "Impulse 7.1 USERS.DAT".to_string(),
            total_records: source_users.len(),
            migrated_records: 0,
            failed_records: 0,
            source_checksum: source_checksum.clone(),
            target_checksum: String::new(),
            spot_checks: Vec::new(),
            errors: Vec::new(),
        };

        // Count migrated users
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE migrated_from = 'impulse71'")
            .fetch_one(&self.db)
            .await?;

        validation.migrated_records = count as usize;
        validation.failed_records = validation.total_records - validation.migrated_records;

        // Calculate target checksum
        validation.target_checksum = self.calculate_target_checksum().await?;

        // Perform spot checks on sample records
        validation.spot_checks = self.perform_spot_checks(source_users).await?;

        // Collect any migration errors
        validation.errors = self.collect_migration_errors().await?;

        Ok(validation)
    }

    async fn calculate_target_checksum(&self) -> Result<String> {
        let users: Vec<(String, i16, i32)> = sqlx::query_as(
            "SELECT name, security_level, login_count
             FROM users
             WHERE migrated_from = 'impulse71'
             ORDER BY id"
        )
        .fetch_all(&self.db)
        .await?;

        let mut hasher = Sha256::new();

        for (name, security, logins) in users {
            hasher.update(name.as_bytes());
            hasher.update(&(security as u8).to_le_bytes());
            hasher.update(&(logins as i16).to_le_bytes());
        }

        Ok(format!("{:x}", hasher.finalize()))
    }

    async fn perform_spot_checks(
        &self,
        source_users: &[Impulse71UserRecord],
    ) -> Result<Vec<SpotCheck>> {
        use rand::seq::SliceRandom;
        use rand::thread_rng;

        let mut checks = Vec::new();
        let mut rng = thread_rng();

        // Sample 10 random users for detailed comparison
        let sample_size = 10.min(source_users.len());
        let samples: Vec<_> = source_users
            .choose_multiple(&mut rng, sample_size)
            .collect();

        for (i, source_user) in samples.iter().enumerate() {
            let name = source_user.name();

            // Query migrated user
            let result: Option<(i32, String, i16, i32, String)> = sqlx::query_as(
                "SELECT id, name, security_level, login_count, location
                 FROM users
                 WHERE name = $1 AND migrated_from = 'impulse71'"
            )
            .bind(&name)
            .fetch_optional(&self.db)
            .await?;

            let check = match result {
                Some((id, db_name, db_security, db_logins, db_location)) => {
                    // Verify key fields match
                    let source_security = source_user.security_level as i16;
                    let source_logins = source_user.login_count as i32;
                    let source_location = source_user.location();

                    if db_security != source_security {
                        SpotCheck {
                            record_num: i + 1,
                            description: format!("User '{}' (ID {})", name, id),
                            passed: false,
                            error: Some(format!(
                                "Security level mismatch: source={}, target={}",
                                source_security, db_security
                            )),
                        }
                    } else if db_logins != source_logins {
                        SpotCheck {
                            record_num: i + 1,
                            description: format!("User '{}' (ID {})", name, id),
                            passed: false,
                            error: Some(format!(
                                "Login count mismatch: source={}, target={}",
                                source_logins, db_logins
                            )),
                        }
                    } else if db_location != source_location {
                        SpotCheck {
                            record_num: i + 1,
                            description: format!("User '{}' (ID {})", name, id),
                            passed: false,
                            error: Some(format!(
                                "Location mismatch: source='{}', target='{}'",
                                source_location, db_location
                            )),
                        }
                    } else {
                        SpotCheck {
                            record_num: i + 1,
                            description: format!("User '{}' (ID {})", name, id),
                            passed: true,
                            error: None,
                        }
                    }
                }
                None => SpotCheck {
                    record_num: i + 1,
                    description: format!("User '{}'", name),
                    passed: false,
                    error: Some("User not found in migrated database".to_string()),
                },
            };

            checks.push(check);
        }

        Ok(checks)
    }

    async fn collect_migration_errors(&self) -> Result<Vec<String>> {
        // Check migration_log table for errors
        let errors: Vec<(String,)> = sqlx::query_as(
            "SELECT error_message
             FROM migration_log
             WHERE status = 'failed'
             ORDER BY created_at DESC
             LIMIT 100"
        )
        .fetch_all(&self.db)
        .await?;

        Ok(errors.into_iter().map(|(msg,)| msg).collect())
    }
}
```

**CLI Migration Tool (impimport):**
```rust
use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle};
use sqlx::PgPool;
use std::path::PathBuf;
use anyhow::{Context, Result};

/// Impulse 7.1 BBS data migration tool
#[derive(Parser)]
#[command(name = "impimport")]
#[command(about = "Import data from legacy BBS systems", long_about = None)]
struct Cli {
    /// Database connection string
    #[arg(short, long, env = "DATABASE_URL")]
    database: String,

    /// Source BBS data directory
    #[arg(short, long)]
    source: PathBuf,

    /// Dry run (don't commit changes)
    #[arg(long)]
    dry_run: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Import from Impulse 7.1
    Impulse71 {
        /// Import users
        #[arg(long)]
        users: bool,

        /// Import messages
        #[arg(long)]
        messages: bool,

        /// Import file areas
        #[arg(long)]
        files: bool,

        /// Import all data
        #[arg(long)]
        all: bool,
    },

    /// Import from Renegade
    Renegade {
        #[arg(long)]
        users: bool,
    },

    /// Import from Telegard
    Telegard {
        #[arg(long)]
        users: bool,
    },

    /// Validate migration
    Validate {
        /// Source system
        #[arg(long)]
        system: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Connect to database
    println!("Connecting to database...");
    let db = PgPool::connect(&cli.database)
        .await
        .context("Failed to connect to database")?;

    // Execute command
    match cli.command {
        Commands::Impulse71 { users, messages, files, all } => {
            import_impulse71(&db, &cli.source, users || all, messages || all, files || all, cli.dry_run, cli.verbose).await?;
        }
        Commands::Renegade { users } => {
            import_renegade_users(&db, &cli.source, cli.dry_run, cli.verbose).await?;
        }
        Commands::Telegard { users } => {
            import_telegard_users(&db, &cli.source, cli.dry_run, cli.verbose).await?;
        }
        Commands::Validate { system } => {
            validate_migration(&db, &system).await?;
        }
    }

    db.close().await;
    Ok(())
}

async fn import_impulse71(
    db: &PgPool,
    source_dir: &PathBuf,
    import_users: bool,
    import_messages: bool,
    import_files: bool,
    dry_run: bool,
    verbose: bool,
) -> Result<()> {
    println!("\n=== Impulse 7.1 Data Migration ===");
    println!("Source directory: {}", source_dir.display());
    println!("Dry run: {}", dry_run);
    println!();

    let importer = Impulse71Importer::new(source_dir.clone());

    if import_users {
        println!("Importing users...");

        // Read source data
        let users = importer.import_users().await
            .context("Failed to read USERS.DAT")?;

        println!("Found {} users in source database", users.len());

        // Calculate source checksum
        let source_checksum = importer.calculate_users_checksum(&users);
        if verbose {
            println!("Source checksum: {}", source_checksum);
        }

        // Create progress bar
        let pb = ProgressBar::new(users.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")
                .unwrap()
                .progress_chars("#>-")
        );

        // Begin transaction
        let mut tx = db.begin().await?;

        let mut imported = 0;
        let mut failed = 0;

        for user in &users {
            let name = user.name();
            pb.set_message(format!("Importing '{}'", name));

            // Convert to modern user record
            let result = sqlx::query(
                "INSERT INTO users (
                    name, password_hash, security_level, location, phone,
                    login_count, last_login, upload_kb, download_kb,
                    total_uploads, total_downloads, birthdate,
                    ansi_enabled, expert_mode, page_length, alias,
                    last_message_read, migrated_from, created_at, updated_at
                 ) VALUES (
                    $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12,
                    $13, $14, $15, $16, $17, 'impulse71', NOW(), NOW()
                 )
                 ON CONFLICT (name) DO NOTHING"
            )
            .bind(&name)
            .bind(&user.password()) // TODO: Re-hash with Argon2id
            .bind(user.security_level as i16)
            .bind(&user.location())
            .bind(&user.phone())
            .bind(user.login_count as i32)
            .bind(user.last_login())
            .bind(user.upload_kb)
            .bind(user.download_kb)
            .bind(user.total_uploads as i32)
            .bind(user.total_downloads as i32)
            .bind(user.birth_date())
            .bind(user.ansi_enabled != 0)
            .bind(user.expert_mode != 0)
            .bind(user.page_length as i16)
            .bind(&user.alias())
            .bind(user.last_message_read)
            .execute(&mut *tx)
            .await;

            match result {
                Ok(_) => imported += 1,
                Err(e) => {
                    if verbose {
                        eprintln!("Failed to import user '{}': {}", name, e);
                    }

                    // Log error
                    let _ = sqlx::query(
                        "INSERT INTO migration_log (source_system, record_type, record_id, status, error_message)
                         VALUES ('impulse71', 'user', $1, 'failed', $2)"
                    )
                    .bind(&name)
                    .bind(e.to_string())
                    .execute(&mut *tx)
                    .await;

                    failed += 1;
                }
            }

            pb.inc(1);
        }

        pb.finish_with_message(format!("Imported {} users ({} failed)", imported, failed));

        if dry_run {
            println!("\nDry run - rolling back transaction");
            tx.rollback().await?;
        } else {
            println!("\nCommitting transaction...");
            tx.commit().await?;

            // Validate migration
            println!("\nValidating migration...");
            let validator = MigrationValidator::new(db.clone());
            let validation = validator.validate_impulse71_users(&users, source_checksum).await?;
            validation.print_summary();
        }
    }

    if import_messages {
        println!("\nMessage import not yet implemented");
        // TODO: Implement message migration
    }

    if import_files {
        println!("\nFile area import not yet implemented");
        // TODO: Implement file area migration
    }

    Ok(())
}

async fn import_renegade_users(
    db: &PgPool,
    source_dir: &PathBuf,
    dry_run: bool,
    verbose: bool,
) -> Result<()> {
    println!("\n=== Renegade User Migration ===");
    println!("Source directory: {}", source_dir.display());

    // TODO: Implement Renegade USERS.DAT parser
    println!("Not yet implemented");

    Ok(())
}

async fn import_telegard_users(
    db: &PgPool,
    source_dir: &PathBuf,
    dry_run: bool,
    verbose: bool,
) -> Result<()> {
    println!("\n=== Telegard User Migration ===");
    println!("Source directory: {}", source_dir.display());

    // TODO: Implement Telegard USER.BBS parser
    println!("Not yet implemented");

    Ok(())
}

async fn validate_migration(db: &PgPool, system: &str) -> Result<()> {
    println!("\n=== Migration Validation ===");
    println!("System: {}", system);

    let validator = MigrationValidator::new(db.clone());

    match system {
        "impulse71" => {
            // Re-read source data and validate
            println!("Validating Impulse 7.1 migration...");
            // TODO: Re-read source and validate
        }
        _ => {
            anyhow::bail!("Unknown system: {}", system);
        }
    }

    Ok(())
}
```

**Renegade/Telegard Format Importers:**
```rust
use binrw::{BinRead, BinWrite};
use std::path::PathBuf;

/// Renegade user record (USERS.DAT format)
///
/// Renegade BBS user database format (different from Impulse 7.1)
#[derive(Debug, Clone, BinRead, BinWrite)]
#[brw(little)]
pub struct RenegadeUserRecord {
    #[br(map = |b: u8| b)]
    name_len: u8,
    #[br(count = 35)]
    name_data: Vec<u8>,

    #[br(map = |b: u8| b)]
    password_len: u8,
    #[br(count = 20)]
    password_data: Vec<u8>,

    pub security_level: u8,
    pub flags: u32,

    #[br(map = |b: u8| b)]
    city_len: u8,
    #[br(count = 30)]
    city_data: Vec<u8>,

    pub num_calls: i32,
    pub last_on: i32,
    pub time_on: i32,

    pub uploads: i16,
    pub downloads: i16,
    pub upload_kb: i32,
    pub download_kb: i32,

    pub messages_posted: i32,
    pub last_message_pointer: i32,

    pub vote: [u8; 20],

    #[br(count = 80)]
    _reserved: Vec<u8>,
}

impl RenegadeUserRecord {
    fn parse_string(len: u8, data: &[u8]) -> String {
        let bytes = &data[..len as usize];
        let (decoded, _, _) = encoding_rs::WINDOWS_1252.decode(bytes);
        decoded.into_owned()
    }

    pub fn name(&self) -> String {
        Self::parse_string(self.name_len, &self.name_data)
    }

    pub fn password(&self) -> String {
        Self::parse_string(self.password_len, &self.password_data)
    }

    pub fn city(&self) -> String {
        Self::parse_string(self.city_len, &self.city_data)
    }
}

/// Telegard user record (USER.BBS format)
#[derive(Debug, Clone, BinRead, BinWrite)]
#[brw(little)]
pub struct TelegardUserRecord {
    #[br(map = |b: u8| b)]
    name_len: u8,
    #[br(count = 30)]
    name_data: Vec<u8>,

    #[br(map = |b: u8| b)]
    password_len: u8,
    #[br(count = 15)]
    password_data: Vec<u8>,

    pub security_level: u8,

    #[br(map = |b: u8| b)]
    location_len: u8,
    #[br(count = 25)]
    location_data: Vec<u8>,

    pub times_on: i32,
    pub last_on_date: i32,

    pub ul_k: i32,
    pub dl_k: i32,
    pub uploads: i16,
    pub downloads: i16,

    pub messages_posted: i32,

    pub ansi: u8,
    pub expert: u8,

    #[br(count = 100)]
    _reserved: Vec<u8>,
}

impl TelegardUserRecord {
    fn parse_string(len: u8, data: &[u8]) -> String {
        let bytes = &data[..len as usize];
        let (decoded, _, _) = encoding_rs::WINDOWS_1252.decode(bytes);
        decoded.into_owned()
    }

    pub fn name(&self) -> String {
        Self::parse_string(self.name_len, &self.name_data)
    }

    pub fn password(&self) -> String {
        Self::parse_string(self.password_len, &self.password_data)
    }

    pub fn location(&self) -> String {
        Self::parse_string(self.location_len, &self.location_data)
    }
}

/// Renegade importer
pub struct RenegadeImporter {
    data_dir: PathBuf,
}

impl RenegadeImporter {
    pub fn new(data_dir: PathBuf) -> Self {
        Self { data_dir }
    }

    pub async fn import_users(&self) -> anyhow::Result<Vec<RenegadeUserRecord>> {
        use binrw::BinReaderExt;
        use std::io::BufReader;

        let users_path = self.data_dir.join("USERS.DAT");
        let file = std::fs::File::open(&users_path)?;
        let mut reader = BufReader::new(file);

        let mut users = Vec::new();

        // Renegade uses variable-length records
        while let Ok(user) = reader.read_le::<RenegadeUserRecord>() {
            if user.name_len > 0 {
                users.push(user);
            }
        }

        Ok(users)
    }
}

/// Telegard importer
pub struct TelegardImporter {
    data_dir: PathBuf,
}

impl TelegardImporter {
    pub fn new(data_dir: PathBuf) -> Self {
        Self { data_dir }
    }

    pub async fn import_users(&self) -> anyhow::Result<Vec<TelegardUserRecord>> {
        use binrw::BinReaderExt;
        use std::io::BufReader;

        let users_path = self.data_dir.join("USER.BBS");
        let file = std::fs::File::open(&users_path)?;
        let mut reader = BufReader::new(file);

        let mut users = Vec::new();

        while let Ok(user) = reader.read_le::<TelegardUserRecord>() {
            if user.name_len > 0 {
                users.push(user);
            }
        }

        Ok(users)
    }
}
```

---

## Dependencies

### Upstream Dependencies
- **Sprint 06**: User system provides target schema for migration
- **Sprint 07**: Database layer provides migration targets
- **Sprint 12**: Message system schema for message migration

### Blocks Downstream
- **Sprint 30**: Beta testing requires migration testing with real data

---

## Testing Requirements

### Unit Tests
- [ ] Impulse 7.1 binary format parsing (USERS.DAT, message headers)
- [ ] Pascal string conversion (length-prefixed to UTF-8)
- [ ] Julian date conversion (Pascal epoch to Rust NaiveDate)
- [ ] Renegade format parsing
- [ ] Telegard format parsing
- [ ] Checksum calculation consistency

### Integration Tests
- [ ] Full user migration cycle (read, transform, insert)
- [ ] Transaction rollback on errors
- [ ] Duplicate user handling (ON CONFLICT)
- [ ] Migration log recording
- [ ] Progress reporting accuracy

### Validation Tests
- [ ] Checksum verification (source vs target)
- [ ] Record count comparison
- [ ] Spot check sampling (random users compared field-by-field)
- [ ] Data integrity constraints (foreign keys, NOT NULL)

### Compatibility Tests
- [ ] Impulse 7.1 data (various versions)
- [ ] Renegade USERS.DAT (10-05-+)
- [ ] Telegard USER.BBS (2.7+)
- [ ] Corrupt data handling (partial records, invalid dates)

### CLI Tests
- [ ] Command-line argument parsing
- [ ] Dry run mode (no commits)
- [ ] Progress bar display
- [ ] Error reporting

---

## Notes and Decisions

### Design Decisions
*To be populated during sprint*

**Proposed Decisions:**
- Use `binrw` for binary parsing (declarative, type-safe)
- Support multiple source formats (Impulse 7.1, Renegade, Telegard)
- Calculate checksums for validation (SHA-256 of key fields)
- Use transactions for atomicity (all-or-nothing imports)
- Log migration errors to database table
- Re-hash passwords with Argon2id during migration
- Convert CP437/Windows-1252 to UTF-8 encoding
- Preserve original data in `migrated_from` field
- Provide dry-run mode for testing
- Use indicatif for progress reporting

### Lessons Learned
*To be populated during sprint*

### Risks and Mitigations
- **Risk**: Binary format mismatches (different Impulse 7.1 versions)
- **Mitigation**: Test with multiple source databases; provide manual override options
- **Risk**: Data corruption during migration
- **Mitigation**: Use transactions; validate with checksums; provide rollback capability
- **Risk**: Password migration security
- **Mitigation**: Re-hash all passwords with Argon2id; never expose plaintext passwords
- **Risk**: Large datasets exceed memory
- **Mitigation**: Stream records incrementally; use batched inserts
- **Risk**: Encoding issues (CP437 to UTF-8)
- **Mitigation**: Use encoding_rs library; test with non-ASCII usernames
- **Risk**: Duplicate records
- **Mitigation**: Use ON CONFLICT clauses; provide duplicate resolution options

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
