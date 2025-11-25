# Migration Guide

**Project:** Impulse 7.1 BBS Modernization
**Last Updated:** 2025-11-22
**Target Audience:** System Administrators, Migration Engineers

---

## Table of Contents

1. [Migration Overview](#migration-overview)
2. [Pre-Migration Assessment](#pre-migration-assessment)
3. [Data Inventory](#data-inventory)
4. [Binary Format Analysis](#binary-format-analysis)
5. [Migration Strategies](#migration-strategies)
6. [User Data Migration](#user-data-migration)
7. [Message Base Migration](#message-base-migration)
8. [File Area Migration](#file-area-migration)
9. [Configuration Migration](#configuration-migration)
10. [Validation and Testing](#validation-and-testing)
11. [Migration Tooling](#migration-tooling)
12. [Rollback Procedures](#rollback-procedures)

---

## Migration Overview

### Migration Objectives

1. **Complete Data Preservation**: Zero data loss from legacy Pascal system
2. **Binary Compatibility**: Maintain ability to read original file formats
3. **Incremental Migration**: Support phased migration without downtime
4. **Validation**: Verify data integrity at every step
5. **Rollback Capability**: Safe fallback to Pascal version if needed

### Migration Timeline

```
┌──────────────┬───────────────┬──────────────┬──────────────┐
│ Phase 1      │ Phase 2       │ Phase 3      │ Phase 4      │
│ Assessment   │ Test Migration│ Production   │ Verification │
│ (1-2 weeks)  │ (2-4 weeks)   │ (1-2 days)   │ (1-2 weeks)  │
└──────────────┴───────────────┴──────────────┴──────────────┘
```

### Migration Approaches

| Approach | Downtime | Complexity | Risk | Best For |
|----------|----------|------------|------|----------|
| **Big Bang** | Full | Low | High | Small BBS, < 100 users |
| **Incremental** | Minimal | High | Low | Large BBS, active users |
| **Parallel Run** | None | Very High | Very Low | Mission-critical systems |
| **Phased** | Partial | Medium | Medium | Most deployments |

**Recommended**: Phased migration with parallel validation

---

## Pre-Migration Assessment

### Data Volume Assessment

```bash
# Analyze existing BBS data directory
./tools/assess_migration.sh /path/to/imp71rel/

# Expected output:
# Users: 1,234 records (512 KB)
# Messages (JAM): 45,678 messages (23 MB)
# File Areas: 2,345 files (1.2 GB)
# Configuration: 15 files (156 KB)
# Total Size: 1.24 GB
# Estimated Migration Time: 2-4 hours
```

### System Requirements Check

```bash
# Verify disk space (need 3x data size)
df -h /var/lib/impulse

# Check available memory
free -h

# Verify Rust version
rustc --version  # Should be 1.75+

# Test migration tools
cargo test -p imp-migrate --all
```

### Backup Verification

```bash
# Create complete backup BEFORE migration
./backup_bbs.sh /path/to/imp71rel/ /backup/pre-migration/

# Verify backup integrity
tar -tzf /backup/pre-migration/impulse_backup_20250122.tar.gz | wc -l

# Test restore procedure (to separate directory)
./restore_bbs.sh /backup/pre-migration/impulse_backup_20250122.tar.gz /tmp/test_restore/
```

---

## Data Inventory

### Legacy File Formats

| File | Format | Purpose | Size Range |
|------|--------|---------|------------|
| **USERS.DAT** | Binary records (512 bytes each) | User accounts | 10 KB - 10 MB |
| **USERS.IDX** | Index file | User lookup | 1 KB - 1 MB |
| ***.JHR** | JAM header | Message base headers | Variable |
| ***.JDT** | JAM text | Message body text | Variable |
| ***.JLR** | JAM last-read | User read pointers | Variable |
| ***.JDX** | JAM index | Message index | Variable |
| **FILES.DAT** | Binary records | File area metadata | 1 KB - 100 MB |
| **FILES.IDX** | Index file | File lookup | 1 KB - 10 MB |
| **CONFIG.DAT** | Binary config | BBS configuration | 4 KB - 16 KB |
| **MENUS.DAT** | Text/Binary | Menu definitions | 10 KB - 1 MB |

### Critical Fields to Preserve

**User Records:**
- Username (unique identifier)
- Password hash (security-critical)
- Security level (access control)
- Upload/download ratios (user stats)
- Last call date/time (activity tracking)
- Personal information (name, location, etc.)

**Message Records:**
- Message ID (unique identifier)
- From/To users (attribution)
- Subject and body text (content)
- Timestamp (chronology)
- Reply chain (threading)
- Read status per user (tracking)

**File Records:**
- Filename and description
- Upload date and uploader
- Download count
- File size and CRC
- Access permissions

---

## Binary Format Analysis

### User Record Structure

**Pascal Definition:**
```pascal
Type
  UserRecord = Record
    Name:          String[30];      { Offset 0, Length 31 }
    RealName:      String[30];      { Offset 31, Length 31 }
    Password:      String[15];      { Offset 62, Length 16 }
    Location:      String[30];      { Offset 78, Length 31 }
    Phone:         String[12];      { Offset 109, Length 13 }
    Security:      Byte;            { Offset 122 }
    Flags:         Word;            { Offset 123, 2 bytes }
    TotalCalls:    LongInt;         { Offset 125, 4 bytes }
    LastCall:      LongInt;         { Offset 129, 4 bytes - Unix timestamp }
    { ... additional fields ... }
  End; { Total size: 512 bytes }
```

**Rust Equivalent (using binrw):**
```rust
use binrw::{BinRead, BinWrite};

#[derive(Debug, BinRead, BinWrite)]
#[brw(little)]
pub struct UserRecord {
    #[br(count = 31)]
    name: Vec<u8>,        // Pascal String[30]

    #[br(count = 31)]
    real_name: Vec<u8>,   // Pascal String[30]

    #[br(count = 16)]
    password: Vec<u8>,    // Pascal String[15]

    #[br(count = 31)]
    location: Vec<u8>,    // Pascal String[30]

    #[br(count = 13)]
    phone: Vec<u8>,       // Pascal String[12]

    pub security: u8,
    pub flags: u16,
    pub total_calls: u32,
    pub last_call: u32,

    // Remaining fields to reach 512 bytes
    #[br(count = 512 - 129 - 4)]
    reserved: Vec<u8>,
}

impl UserRecord {
    pub fn name_str(&self) -> String {
        pascal_string_to_rust(&self.name)
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let mut cursor = Cursor::new(data);
        Self::read(&mut cursor).context("Failed to parse user record")
    }
}
```

### JAM Message Base Format

**JAM Header Structure (1024 bytes):**
```rust
#[derive(Debug, BinRead, BinWrite)]
#[brw(little)]
pub struct JamHeader {
    #[br(count = 4)]
    pub signature: [u8; 4],     // "JAM\0"

    pub revision: u32,           // Format revision
    pub base_msg_num: u32,       // Base message number
    pub active_msgs: u32,        // Active message count
    pub password_crc: u32,       // Password protection

    // ... additional fields (see 03-technical-details.md)
}

#[derive(Debug, BinRead, BinWrite)]
pub struct JamMessageHeader {
    pub signature: [u8; 4],      // "JAM\0"
    pub revision: u16,
    pub reserved: u16,
    pub subfieldlen: u32,        // Length of subfields
    pub timesread: u32,
    pub msgidcrc: u32,
    pub replycrc: u32,
    pub replyto: u32,
    pub reply1st: u32,
    pub replynext: u32,
    pub datewritten: u32,
    pub dateprocessed: u32,
    pub messagenumber: u32,
    pub attribute: u32,
    pub attribute2: u32,
    pub offset: u32,             // Text offset in .JDT
    pub txtlen: u32,             // Text length
}
```

**Subfield Format (variable length):**
```rust
#[derive(Debug, BinRead, BinWrite)]
pub struct JamSubfield {
    pub loid: u16,               // Low word of ID
    pub hiid: u16,               // High word of ID
    pub datlen: u32,             // Data length
    #[br(count = datlen)]
    pub data: Vec<u8>,           // Subfield data
}

// Common subfield types
pub const JAMSFLD_OADDRESS: u32 = 0;    // Origin address
pub const JAMSFLD_DADDRESS: u32 = 1;    // Destination address
pub const JAMSFLD_SENDERNAME: u32 = 2;  // Sender name
pub const JAMSFLD_RECVERNAME: u32 = 3;  // Receiver name
pub const JAMSFLD_MSGID: u32 = 4;       // Message ID
pub const JAMSFLD_REPLYID: u32 = 5;     // Reply ID
pub const JAMSFLD_SUBJECT: u32 = 6;     // Subject
```

### String Encoding Conversion

**Pascal Strings to Rust:**
```rust
/// Convert Pascal-style length-prefixed string to Rust String
pub fn pascal_string_to_rust(data: &[u8]) -> String {
    if data.is_empty() {
        return String::new();
    }

    let length = data[0] as usize;
    let bytes = &data[1..=length.min(data.len() - 1)];

    // Handle CP437 (DOS) encoding
    CP437_DECODER.decode(bytes, DecoderTrap::Replace)
        .unwrap_or_else(|_| String::from_utf8_lossy(bytes).to_string())
}

/// Convert Rust String to Pascal-style bytes
pub fn rust_string_to_pascal(s: &str, max_len: usize) -> Vec<u8> {
    let encoded = CP437_ENCODER.encode(s, EncoderTrap::Replace)
        .unwrap_or_else(|_| s.as_bytes().to_vec());

    let length = encoded.len().min(max_len);
    let mut result = vec![length as u8];
    result.extend_from_slice(&encoded[..length]);
    result.resize(max_len + 1, 0);  // Pad to fixed size
    result
}
```

---

## Migration Strategies

### Strategy 1: Big Bang Migration

**Use Case:** Small BBS, acceptable downtime

```bash
# 1. Stop Pascal BBS
dosbox -c "cd imp71rel && imp.exe /shutdown"

# 2. Run full migration
cargo run --bin imp-migrate -- \
    --source /path/to/imp71rel/ \
    --target /var/lib/impulse/ \
    --mode full \
    --verify

# 3. Start Rust BBS
systemctl start impulse-bbs

# Total Downtime: 1-4 hours
```

### Strategy 2: Incremental Migration

**Use Case:** Large BBS, minimal downtime acceptable

```bash
# Phase 1: Migrate users (read-only mode)
cargo run --bin imp-migrate -- \
    --source /path/to/imp71rel/ \
    --target /var/lib/impulse/ \
    --mode users \
    --read-only

# Phase 2: Migrate messages (batch processing)
cargo run --bin imp-migrate -- \
    --source /path/to/imp71rel/ \
    --target /var/lib/impulse/ \
    --mode messages \
    --batch-size 1000

# Phase 3: Migrate files
cargo run --bin imp-migrate -- \
    --source /path/to/imp71rel/ \
    --target /var/lib/impulse/ \
    --mode files

# Phase 4: Switchover (brief downtime)
# Stop Pascal, final sync, start Rust

# Total Downtime: 15-30 minutes
```

### Strategy 3: Parallel Run (Recommended)

**Use Case:** Mission-critical, zero data loss tolerance

```bash
# Run both systems simultaneously
# Pascal BBS: Port 23 (production)
# Rust BBS: Port 2323 (testing)

# Continuous sync from Pascal to Rust
cargo run --bin imp-sync -- \
    --source /path/to/imp71rel/ \
    --target /var/lib/impulse/ \
    --mode continuous \
    --interval 300  # Sync every 5 minutes

# After validation period (1-2 weeks):
# 1. Announce switch time
# 2. Stop Pascal BBS
# 3. Final sync
# 4. Switch Rust BBS to port 23
# 5. Monitor closely

# Total Downtime: 5-10 minutes
```

---

## User Data Migration

### Migration Tool Usage

```bash
# Dry run (no changes, validation only)
cargo run --bin imp-migrate-users -- \
    --source /imp71rel/USERS.DAT \
    --target /var/lib/impulse/users/ \
    --dry-run \
    --verbose

# Actual migration
cargo run --bin imp-migrate-users -- \
    --source /imp71rel/USERS.DAT \
    --target /var/lib/impulse/users/ \
    --backup \
    --verify

# Expected output:
# [INFO] Reading source: /imp71rel/USERS.DAT
# [INFO] Found 1,234 user records
# [INFO] Validating records...
# [WARN] Record 245: Invalid security level (255), clamping to 254
# [INFO] Converting records...
# [INFO] Writing to: /var/lib/impulse/users/
# [INFO] Verifying migration...
# [INFO] ✓ All 1,234 records migrated successfully
# [INFO] Migration completed in 3.2 seconds
```

### Password Migration

**Pascal (plain text or simple hash):**
```pascal
Function CheckPassword(Input: String; Stored: String): Boolean;
Begin
  CheckPassword := (Input = Stored);  { Plain text comparison }
End;
```

**Rust (upgrade to bcrypt):**
```rust
pub fn migrate_user_password(old_password: &str) -> Result<String> {
    // Hash legacy plain-text password with bcrypt
    let cost = 12;  // Bcrypt cost factor
    let hash = bcrypt::hash(old_password, cost)
        .context("Failed to hash password")?;

    Ok(hash)
}

// On first login after migration, prompt user to change password
pub fn verify_and_upgrade_password(
    username: &str,
    provided: &str,
    stored_hash: &str
) -> Result<bool> {
    if bcrypt::verify(provided, stored_hash)? {
        return Ok(true);
    }

    // Fallback: Check if legacy plain-text (migration in progress)
    if is_legacy_password_format(stored_hash) && provided == stored_hash {
        warn!("User {} using legacy password format", username);
        // Upgrade to bcrypt
        let new_hash = bcrypt::hash(provided, 12)?;
        update_user_password(username, &new_hash)?;
        Ok(true)
    } else {
        Ok(false)
    }
}
```

### User Statistics Preservation

```rust
pub fn migrate_user_statistics(legacy: &LegacyUserRecord) -> UserStatistics {
    UserStatistics {
        total_calls: legacy.total_calls,
        total_posts: legacy.total_posts,
        total_uploads: legacy.total_uploads,
        total_downloads: legacy.total_downloads,
        upload_bytes: legacy.upload_kb as u64 * 1024,
        download_bytes: legacy.download_kb as u64 * 1024,
        first_call: parse_dos_datetime(legacy.first_call),
        last_call: parse_dos_datetime(legacy.last_call),
        time_online_minutes: legacy.time_online,
        // Calculate ratios
        upload_download_ratio: calculate_ratio(
            legacy.upload_kb,
            legacy.download_kb
        ),
    }
}
```

---

## Message Base Migration

### JAM Format Migration

```rust
pub async fn migrate_jam_message_base(
    source_path: &Path,
    target_path: &Path,
) -> Result<MigrationStats> {
    let mut stats = MigrationStats::default();

    // Open legacy JAM files
    let mut jhr = File::open(source_path.join("MESSAGES.JHR"))?;
    let mut jdt = File::open(source_path.join("MESSAGES.JDT"))?;
    let mut jdx = File::open(source_path.join("MESSAGES.JDX"))?;
    let mut jlr = File::open(source_path.join("MESSAGES.JLR"))?;

    // Create new message base
    let msg_base = JamBase::create(target_path).await?;

    // Read base header
    let base_header = JamHeader::read(&mut jhr)?;
    info!("Migrating {} messages", base_header.active_msgs);

    // Migrate each message
    for msg_num in 1..=base_header.active_msgs {
        match migrate_single_message(&mut jhr, &mut jdt, msg_num).await {
            Ok(msg) => {
                msg_base.import_message(msg).await?;
                stats.messages_migrated += 1;
            }
            Err(e) => {
                error!("Failed to migrate message {}: {}", msg_num, e);
                stats.messages_failed += 1;
            }
        }

        if msg_num % 100 == 0 {
            info!("Progress: {} / {} messages", msg_num, base_header.active_msgs);
        }
    }

    // Migrate last-read pointers
    migrate_last_read_pointers(&mut jlr, &msg_base).await?;

    Ok(stats)
}

async fn migrate_single_message(
    jhr: &mut File,
    jdt: &mut File,
    msg_num: u32,
) -> Result<Message> {
    // Read message header
    let header = JamMessageHeader::read(jhr)?;

    // Read subfields
    let mut subfields = Vec::new();
    let mut bytes_read = 0;
    while bytes_read < header.subfieldlen {
        let subfield = JamSubfield::read(jhr)?;
        bytes_read += 8 + subfield.datlen;
        subfields.push(subfield);
    }

    // Read message text
    jdt.seek(SeekFrom::Start(header.offset as u64))?;
    let mut text_buf = vec![0u8; header.txtlen as usize];
    jdt.read_exact(&mut text_buf)?;

    // Extract fields from subfields
    let from = extract_subfield(&subfields, JAMSFLD_SENDERNAME)?;
    let to = extract_subfield(&subfields, JAMSFLD_RECVERNAME)?;
    let subject = extract_subfield(&subfields, JAMSFLD_SUBJECT)?;

    // Construct Message
    Ok(Message {
        id: msg_num,
        from_user: from,
        to_user: to,
        subject,
        body: String::from_utf8_lossy(&text_buf).to_string(),
        timestamp: header.datewritten,
        reply_to: if header.replyto > 0 { Some(header.replyto) } else { None },
        attributes: header.attribute,
    })
}
```

### Hudson Format Migration

```rust
// Hudson uses different structure (indexed by user number)
pub async fn migrate_hudson_message_base(
    source_path: &Path,
    target_path: &Path,
) -> Result<MigrationStats> {
    // Hudson: MSGHDR.BBS (headers), MSGTXT.BBS (text), MSGIDX.BBS (index)
    let headers = File::open(source_path.join("MSGHDR.BBS"))?;
    let text = File::open(source_path.join("MSGTXT.BBS"))?;

    // Convert to JAM format (our standard)
    let msg_base = JamBase::create(target_path).await?;

    // Migration logic specific to Hudson format...
    unimplemented!("Hudson migration in Sprint 9")
}
```

---

## File Area Migration

### File Metadata Migration

```rust
pub fn migrate_file_areas(
    source_path: &Path,
    target_path: &Path,
) -> Result<MigrationStats> {
    let mut stats = MigrationStats::default();

    // Read legacy FILES.DAT
    let files_dat = File::open(source_path.join("FILES.DAT"))?;
    let mut reader = BufReader::new(files_dat);

    // Create new file area database
    let file_mgr = FileAreaManager::new(target_path)?;

    loop {
        match FileRecord::read(&mut reader) {
            Ok(legacy_file) => {
                let new_file = FileEntry {
                    filename: legacy_file.name(),
                    description: legacy_file.description(),
                    uploader: legacy_file.uploaded_by(),
                    upload_date: parse_dos_datetime(legacy_file.upload_date),
                    size_bytes: legacy_file.size as u64,
                    download_count: legacy_file.downloads as u64,
                    crc32: legacy_file.crc,
                    area_id: legacy_file.area,
                };

                file_mgr.add_file(new_file)?;
                stats.files_migrated += 1;
            }
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
            Err(e) => {
                error!("Error reading file record: {}", e);
                stats.files_failed += 1;
            }
        }
    }

    // Verify actual files exist
    stats.files_verified = verify_file_existence(&file_mgr)?;

    Ok(stats)
}

fn verify_file_existence(file_mgr: &FileAreaManager) -> Result<u32> {
    let mut count = 0;
    for area in file_mgr.list_areas()? {
        for file in file_mgr.list_files(area.id)? {
            if file.physical_path().exists() {
                count += 1;
            } else {
                warn!("Missing file: {}", file.filename);
            }
        }
    }
    Ok(count)
}
```

### Physical File Organization

```
Legacy (DOS 8.3 filenames):
/imp71rel/
└── FILES/
    ├── UTIL/
    │   ├── PKZIP204.EXE
    │   └── README.TXT
    └── DOORS/
        └── LORD350.ZIP

Modern (long filenames, organized):
/var/lib/impulse/
└── files/
    ├── area-01-utilities/
    │   ├── pkzip-2.04g.exe       (renamed for clarity)
    │   └── readme.txt
    └── area-02-door-games/
        └── legend-of-the-red-dragon-3.50.zip
```

---

## Configuration Migration

### Config File Conversion

```rust
pub fn migrate_configuration(
    legacy_config: &Path,
    new_config: &Path,
) -> Result<()> {
    // Read legacy CONFIG.DAT (binary)
    let legacy = LegacyConfig::from_file(legacy_config)?;

    // Convert to modern TOML format
    let modern = Config {
        server: ServerConfig {
            node_count: legacy.num_nodes,
            max_connections: legacy.max_users,
            timeout_seconds: legacy.inactivity_timeout,
            ..Default::default()
        },
        telnet: TelnetConfig {
            enabled: legacy.telnet_enabled,
            port: legacy.telnet_port,
            ..Default::default()
        },
        // ... additional conversions
    };

    // Write TOML
    let toml_str = toml::to_string_pretty(&modern)?;
    std::fs::write(new_config, toml_str)?;

    info!("Configuration migrated to: {}", new_config.display());
    Ok(())
}
```

---

## Validation and Testing

### Post-Migration Validation

```bash
# Run comprehensive validation
cargo run --bin imp-validate -- \
    --data-dir /var/lib/impulse/ \
    --verbose

# Validation checks:
# ✓ User record integrity (1,234 users)
# ✓ Password hashes valid (1,234 / 1,234)
# ✓ Message base integrity (45,678 messages)
# ✓ Message threading valid (45,234 / 45,678)
# ✓ File metadata complete (2,345 files)
# ✓ Physical files exist (2,341 / 2,345) - 4 missing
# ✓ Configuration valid
# ✗ 4 files missing from disk (see warnings above)
```

### Comparison Testing

```rust
// Compare legacy vs migrated data
pub fn compare_migration_results(
    legacy_path: &Path,
    migrated_path: &Path,
) -> Result<ComparisonReport> {
    let mut report = ComparisonReport::default();

    // Compare user counts
    let legacy_users = count_legacy_users(legacy_path)?;
    let migrated_users = count_migrated_users(migrated_path)?;

    report.user_count_match = legacy_users == migrated_users;
    if !report.user_count_match {
        warn!("User count mismatch: {} legacy, {} migrated",
              legacy_users, migrated_users);
    }

    // Spot-check user data
    for username in random_sample_usernames(10) {
        let legacy = load_legacy_user(legacy_path, &username)?;
        let migrated = load_migrated_user(migrated_path, &username)?;

        if legacy.total_calls != migrated.total_calls {
            report.discrepancies.push(format!(
                "User {}: call count {} vs {}",
                username, legacy.total_calls, migrated.total_calls
            ));
        }
    }

    Ok(report)
}
```

---

## Migration Tooling

### Command-Line Tools

**imp-migrate** - Main migration orchestrator
```bash
imp-migrate --source <path> --target <path> [OPTIONS]

Options:
  --mode <MODE>           Migration mode: full, users, messages, files
  --dry-run               Validate without making changes
  --verify                Run validation after migration
  --backup                Create backup before migration
  --batch-size <N>        Process in batches of N records
  --parallel <N>          Use N parallel workers
  --verbose               Detailed logging
```

**imp-validate** - Post-migration validation
```bash
imp-validate --data-dir <path> [OPTIONS]

Options:
  --full                  Run all validation checks
  --users                 Validate user data only
  --messages              Validate message bases
  --files                 Validate file areas
  --report <PATH>         Write report to file
```

**imp-compare** - Compare legacy vs migrated
```bash
imp-compare --legacy <path> --migrated <path>

Options:
  --sample-size <N>       Number of random records to compare
  --report <PATH>         Write comparison report
```

### Progress Monitoring

```rust
use indicatif::{ProgressBar, ProgressStyle};

pub fn migrate_with_progress(total_records: u64) -> Result<()> {
    let pb = ProgressBar::new(total_records);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("=>-")
    );

    for i in 0..total_records {
        // Perform migration
        migrate_record(i)?;

        pb.set_position(i + 1);
        if i % 100 == 0 {
            pb.set_message(format!("Processing record {}", i));
        }
    }

    pb.finish_with_message("Migration complete");
    Ok(())
}
```

---

## Rollback Procedures

### Emergency Rollback

```bash
#!/bin/bash
# emergency_rollback.sh

set -e

echo "EMERGENCY ROLLBACK INITIATED"

# Stop Rust BBS immediately
systemctl stop impulse-bbs

# Restore legacy backup
tar -xzf /backup/pre-migration/impulse_backup.tar.gz -C /imp71rel/

# Restart DOS/Pascal BBS
dosbox -c "cd imp71rel && imp.exe" &

echo "Rollback complete. Legacy BBS restored."
echo "Investigate migration issues before retry."
```

### Partial Rollback

```bash
# Rollback users only (keep messages/files migrated)
imp-rollback --component users --backup /backup/users_pre_migration.dat

# Rollback to specific migration checkpoint
imp-rollback --checkpoint 2025-01-22-14:30:00
```

---

## Migration Checklist

### Pre-Migration
- [ ] Complete backup of legacy system
- [ ] Backup verification completed
- [ ] Disk space verified (3x data size available)
- [ ] Migration tools compiled and tested
- [ ] Dry run completed successfully
- [ ] Downtime window scheduled and communicated
- [ ] Rollback procedure documented and tested

### During Migration
- [ ] Legacy BBS stopped cleanly
- [ ] Migration started with logging
- [ ] Progress monitored
- [ ] No critical errors in logs
- [ ] Validation checks passing

### Post-Migration
- [ ] All data migrated (counts match)
- [ ] Validation suite passed
- [ ] Comparison testing completed
- [ ] Sample user login test successful
- [ ] Message posting test successful
- [ ] File download test successful
- [ ] Admin functions verified
- [ ] Performance acceptable
- [ ] Backups of migrated data completed

### Go-Live
- [ ] DNS/routing updated (if applicable)
- [ ] Users notified of new system
- [ ] Monitoring alerts configured
- [ ] Support team briefed
- [ ] Rollback plan ready
- [ ] First 24 hours closely monitored

---

## Summary

Successful migration requires:
- **Thorough planning** with backups and validation
- **Incremental approach** to minimize risk
- **Comprehensive testing** before production switch
- **Clear rollback plan** for emergency recovery
- **User communication** about changes and expectations

**Migration is a critical phase. Take your time, validate thoroughly, and don't rush to production.**

---

**For additional technical details:**
- [03-technical-details.md](../architecture/technical-details.md) - Binary format specifications
- [05-testing-strategy.md](../testing/testing-strategy.md) - Migration testing approach
- [06-deployment-guide.md](deployment-guide.md) - Post-migration deployment

---

**Migration questions or issues?** Document thoroughly and open an issue for team review.
