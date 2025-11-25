# Type Reconciliation: Existing Rust vs Pascal Definitions

**Purpose:** Reconcile existing modern Rust types with Pascal RECORDS.PAS definitions
**Date:** 2025-11-23
**Sprint:** 5 - Core Types Implementation

## Overview

This document compares the existing Rust type implementations in `impulse-types` with the Pascal type definitions from `source/RECORDS.PAS`. It identifies gaps, conflicts, and provides a strategy for reconciliation using the two-layer architecture pattern.

## Architecture Strategy

**Two-Layer Approach:**
1. **Layer 1 (Binary Compatibility):** Pascal-compatible structs in `legacy/` module
   - `#[repr(C, packed)]` for exact binary layout
   - Fixed-size arrays for strings
   - Pascal data types (u8 for Boolean, etc.)
   - Used for reading/writing legacy `.DAT` files

2. **Layer 2 (Modern API):** Modern Rust types (existing implementations)
   - String, DateTime, Vec, Option, etc.
   - Validation methods
   - Helper functions
   - Ergonomic API for new code

3. **Conversion Layer:** `From`/`TryFrom` implementations
   - Convert between layers transparently
   - Preserve all data
   - Handle edge cases (string truncation, date conversion, etc.)

---

## 1. User Type Reconciliation

### 1.1 Current Rust Implementation

**Location:** `crates/impulse-types/src/user.rs`

```rust
pub struct User {
    pub id: u32,                                    // NOT in Pascal
    pub name: String,                               // Pascal: name: string[36]
    pub password_hash: String,                      // Pascal: pw: string[20]
    pub security_level: SecurityLevel,              // Pascal: sl: byte (0-255)
    pub real_name: Option<String>,                  // Pascal: realname: string[36]
    pub location: Option<String>,                   // Pascal: citystate: string[30]
    pub stats: UserStats,                           // Pascal: distributed fields
    pub registration_date: DateTime<Utc>,           // Pascal: firston: string[8] (MM/DD/YY)
    pub last_login: Option<DateTime<Utc>>,          // Pascal: laston: string[5] (HH:MM)
    pub email: Option<String>,                      // NOT in Pascal
    pub phone: Option<String>,                      // Pascal: ph: string[12]
    pub birthday: Option<NaiveDate>,                // Pascal: bday: string[8] (MM/DD/YY)
    pub notes: Option<String>,                      // Pascal: note: string[79]
}

pub struct UserStats {
    pub calls: u32,                                 // Pascal: timescalled: integer
    pub uploads: u32,                               // Pascal: uploads: word
    pub downloads: u32,                             // Pascal: downloads: word
    pub upload_kb: u64,                             // Pascal: uk: longint
    pub download_kb: u64,                           // Pascal: dk: longint
    pub messages_posted: u32,                       // Pascal: msgpost: word
    pub time_online: u32,                           // Pascal: ttimeon: longint
}
```

### 1.2 Pascal userrec Definition

**Location:** `source/RECORDS.PAS` (lines 143-226)

```pascal
userrec = record
  name         :string[36];      { User name/handle }
  realname     :string[36];      { Real name }
  pw           :string[20];      { Password (plaintext in Pascal!) }
  street       :string[30];      { Street address }
  citystate    :string[30];      { City, State }
  zipcode      :string[10];      { ZIP code }
  ph           :string[12];      { Phone number }
  bday         :string[8];       { Birthday (MM/DD/YY) }
  firston      :string[8];       { First logon date (MM/DD/YY) }
  lastdate     :string[8];       { Last logon date (MM/DD/YY) }
  laston       :string[5];       { Last logon time (HH:MM) }
  note         :string[79];      { SysOp notes }

  sl           :byte;            { Security level (0-255) }
  dsl          :byte;            { Download security level }

  ar           :arf;             { Access flags (set of 'A'..'Z') }

  timebank     :integer;         { Time bank (minutes saved) }
  tltoday      :integer;         { Time left today }
  timetoday    :integer;         { Time on today }
  ttimeon      :longint;         { Total time online (minutes) }
  timescalled  :integer;         { Number of calls }

  uploads      :word;            { Files uploaded }
  downloads    :word;            { Files downloaded }
  uk           :longint;         { Upload kilobytes }
  dk           :longint;         { Download kilobytes }
  msgpost      :word;            { Messages posted }

  emailnet     :word;            { Email network (FidoNet) }
  emailnode    :word;            { Email node number }
  forusr       :byte;            { Forward to user# }

  credit       :longint;         { Credits/points }

  waiting      :byte;            { Messages waiting }
  linelen      :byte;            { Screen line length }
  pagelen      :byte;            { Screen page length }

  ontoday      :byte;            { Times on today }
  illegal      :byte;            { Illegal logon attempts }
  deleted      :boolean;         { User deleted flag }

  sstat        :smalrec;         { Message base scan status }

  lockedout    :boolean;         { User locked out }
  lockedfile   :byte;            { Locked from file areas }
  lockedforum  :byte;            { Locked from message areas }

  foreal       :boolean;         { Forwarding active }

  clr          :byte;            { Color scheme }
  usrdefr      :array[1..20] of byte;  { User-definable flags }

  chatauto     :boolean;         { Auto-chat mode }
  chatseperate :boolean;         { Separate chat window }
  chatsound    :boolean;         { Chat sound }
  chatlastmsg  :string[160];     { Last chat message }

  trapactivity :boolean;         { Trap user activity }
  trapseperate :boolean;         { Separate trap window }

  uldlratio    :boolean;         { Upload/download ratio enforced }
  ultoday      :byte;            { Uploads today }
  dltoday      :byte;            { Downloads today }

  lastfback    :integer;         { Last feedback message# }
  lastmsg      :longint;         { Last message read }
  lastfil      :integer;         { Last file area }
  lastconf     :byte;            { Last conference }

  flags        :flagrec;         { User flags (uflags set) }

  boardsconf   :array[1..maxconfs] of array[1..maxuboards] of boolean;
                                 { Board scan flags }

  ac           :array[1..6] of byte;  { Access codes }

  smw          :boolean;         { Show "Message Waiting" }
end;
```

### 1.3 Field Mapping Analysis

| Pascal Field | Pascal Type | Rust Field | Rust Type | Status | Notes |
|--------------|-------------|------------|-----------|---------|-------|
| `name` | string[36] | `name` | String | **Match** | Modern uses String |
| `realname` | string[36] | `real_name` | Option\<String\> | **Match** | Made optional |
| `pw` | string[20] | `password_hash` | String | **Different** | Modern uses Argon2id hash |
| `street` | string[30] | - | - | **Missing** | Not in modern |
| `citystate` | string[30] | `location` | Option\<String\> | **Similar** | Simplified |
| `zipcode` | string[10] | - | - | **Missing** | Not in modern |
| `ph` | string[12] | `phone` | Option\<String\> | **Match** | Made optional |
| `bday` | string[8] | `birthday` | Option\<NaiveDate\> | **Match** | Better type |
| `firston` | string[8] | `registration_date` | DateTime\<Utc\> | **Match** | Better type |
| `lastdate` | string[8] | - | - | **Missing** | Combined with laston |
| `laston` | string[5] | `last_login` | Option\<DateTime\> | **Match** | Combined date+time |
| `note` | string[79] | `notes` | Option\<String\> | **Match** | Made optional |
| `sl` | byte | `security_level` | SecurityLevel | **Match** | Better type (enum) |
| `dsl` | byte | - | - | **Missing** | Download security level |
| `ar` | arf | - | - | **Missing** | Access flags (A-Z) |
| `timebank` | integer | - | - | **Missing** | Time banking |
| `tltoday` | integer | - | - | **Missing** | Time left today |
| `timetoday` | integer | - | - | **Missing** | Time on today |
| `ttimeon` | longint | `time_online` | u32 | **Match** | In UserStats |
| `timescalled` | integer | `calls` | u32 | **Match** | In UserStats |
| `uploads` | word | `uploads` | u32 | **Match** | In UserStats |
| `downloads` | word | `downloads` | u32 | **Match** | In UserStats |
| `uk` | longint | `upload_kb` | u64 | **Match** | In UserStats |
| `dk` | longint | `download_kb` | u64 | **Match** | In UserStats |
| `msgpost` | word | `messages_posted` | u32 | **Match** | In UserStats |
| `emailnet` | word | - | - | **Missing** | FidoNet email network |
| `emailnode` | word | - | - | **Missing** | FidoNet node |
| `forusr` | byte | - | - | **Missing** | Forward to user# |
| `credit` | longint | - | - | **Missing** | Credits/points |
| `waiting` | byte | - | - | **Missing** | Messages waiting |
| `linelen` | byte | - | - | **Missing** | Screen line length |
| `pagelen` | byte | - | - | **Missing** | Screen page length |
| `ontoday` | byte | - | - | **Missing** | Times on today |
| `illegal` | byte | - | - | **Missing** | Illegal logon attempts |
| `deleted` | boolean | - | - | **Missing** | Deleted flag |
| `sstat` | smalrec | - | - | **Missing** | Message scan status |
| `lockedout` | boolean | - | - | **Missing** | Lockout status |
| `lockedfile` | byte | - | - | **Missing** | File area lockout |
| `lockedforum` | byte | - | - | **Missing** | Forum lockout |
| `foreal` | boolean | - | - | **Missing** | Forwarding active |
| `clr` | byte | - | - | **Missing** | Color scheme |
| `usrdefr` | array[1..20] | - | - | **Missing** | User-definable flags |
| `chatauto` | boolean | - | - | **Missing** | Auto-chat |
| `chatseperate` | boolean | - | - | **Missing** | Separate chat window |
| `chatsound` | boolean | - | - | **Missing** | Chat sound |
| `chatlastmsg` | string[160] | - | - | **Missing** | Last chat message |
| `trapactivity` | boolean | - | - | **Missing** | Trap user activity |
| `trapseperate` | boolean | - | - | **Missing** | Separate trap window |
| `uldlratio` | boolean | - | - | **Missing** | UL/DL ratio enforced |
| `ultoday` | byte | - | - | **Missing** | Uploads today |
| `dltoday` | byte | - | - | **Missing** | Downloads today |
| `lastfback` | integer | - | - | **Missing** | Last feedback msg# |
| `lastmsg` | longint | - | - | **Missing** | Last message read |
| `lastfil` | integer | - | - | **Missing** | Last file area |
| `lastconf` | byte | - | - | **Missing** | Last conference |
| `flags` | flagrec | - | - | **Missing** | User flags (uflags) |
| `boardsconf` | array[...] | - | - | **Missing** | Board scan flags |
| `ac` | array[1..6] | - | - | **Missing** | Access codes |
| `smw` | boolean | - | - | **Missing** | Show message waiting |
| - | - | `id` | u32 | **Extra** | Not in Pascal (added for DB) |
| - | - | `email` | Option\<String\> | **Extra** | Not in Pascal (modern) |

### 1.4 Reconciliation Strategy

**Approach:** Keep both types with conversion layer

**Layer 1 - Pascal Compatible (`legacy/user_record.rs`):**
```rust
#[repr(C, packed)]
pub struct PascalUserRecord {
    pub name: [u8; 37],           // String[36]
    pub realname: [u8; 37],       // String[36]
    pub pw: [u8; 21],             // String[20]
    pub street: [u8; 31],         // String[30]
    pub citystate: [u8; 31],      // String[30]
    pub zipcode: [u8; 11],        // String[10]
    pub ph: [u8; 13],             // String[12]
    pub bday: [u8; 9],            // String[8]
    pub firston: [u8; 9],         // String[8]
    pub lastdate: [u8; 9],        // String[8]
    pub laston: [u8; 6],          // String[5]
    pub note: [u8; 80],           // String[79]

    pub sl: u8,                   // byte
    pub dsl: u8,                  // byte

    pub ar: [u8; 26],             // set of 'A'..'Z' → bitflags

    pub timebank: i16,            // integer
    pub tltoday: i16,             // integer
    pub timetoday: i16,           // integer
    pub ttimeon: i32,             // longint
    pub timescalled: i16,         // integer

    pub uploads: u16,             // word
    pub downloads: u16,           // word
    pub uk: i32,                  // longint
    pub dk: i32,                  // longint
    pub msgpost: u16,             // word

    pub emailnet: u16,            // word
    pub emailnode: u16,           // word
    pub forusr: u8,               // byte

    pub credit: i32,              // longint

    pub waiting: u8,              // byte
    pub linelen: u8,              // byte
    pub pagelen: u8,              // byte

    pub ontoday: u8,              // byte
    pub illegal: u8,              // byte
    pub deleted: u8,              // boolean (as u8)

    pub sstat: [u8; 104],         // smalrec (fixed size)

    pub lockedout: u8,            // boolean
    pub lockedfile: u8,           // byte
    pub lockedforum: u8,          // byte

    pub foreal: u8,               // boolean

    pub clr: u8,                  // byte
    pub usrdefr: [u8; 20],        // array[1..20] of byte

    pub chatauto: u8,             // boolean
    pub chatseperate: u8,         // boolean
    pub chatsound: u8,            // boolean
    pub chatlastmsg: [u8; 161],   // String[160]

    pub trapactivity: u8,         // boolean
    pub trapseperate: u8,         // boolean

    pub uldlratio: u8,            // boolean
    pub ultoday: u8,              // byte
    pub dltoday: u8,              // byte

    pub lastfback: i16,           // integer
    pub lastmsg: i32,             // longint
    pub lastfil: i16,             // integer
    pub lastconf: u8,             // byte

    pub flags: u32,               // flagrec → bitflags (24 flags)

    pub boardsconf: [u8; 5080],   // 20 confs × 254 boards (boolean array)

    pub ac: [u8; 6],              // array[1..6] of byte

    pub smw: u8,                  // boolean
}
```

**Layer 2 - Modern Rust (`user.rs` - EXTEND existing):**
```rust
pub struct User {
    // Existing fields (keep as-is)
    pub id: u32,
    pub name: String,
    pub password_hash: String,
    pub security_level: SecurityLevel,
    pub real_name: Option<String>,
    pub location: Option<String>,
    pub stats: UserStats,
    pub registration_date: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub birthday: Option<NaiveDate>,
    pub notes: Option<String>,

    // NEW: Pascal-specific fields (for full compatibility)
    pub street: Option<String>,
    pub zipcode: Option<String>,
    pub download_security_level: u8,
    pub access_flags: AccessFlags,
    pub time_bank: i16,
    pub time_left_today: i16,
    pub time_on_today: i16,
    pub email_network: Option<u16>,
    pub email_node: Option<u16>,
    pub forward_to_user: Option<u8>,
    pub credits: i32,
    pub messages_waiting: u8,
    pub screen_line_length: u8,
    pub screen_page_length: u8,
    pub times_on_today: u8,
    pub illegal_logon_attempts: u8,
    pub deleted: bool,
    pub message_scan_status: MessageScanStatus,
    pub locked_out: bool,
    pub locked_from_files: u8,
    pub locked_from_forums: u8,
    pub forwarding_active: bool,
    pub color_scheme: u8,
    pub user_defined_flags: [u8; 20],
    pub chat_auto: bool,
    pub chat_separate: bool,
    pub chat_sound: bool,
    pub chat_last_message: Option<String>,
    pub trap_activity: bool,
    pub trap_separate: bool,
    pub ul_dl_ratio_enforced: bool,
    pub uploads_today: u8,
    pub downloads_today: u8,
    pub last_feedback_number: i16,
    pub last_message_read: i32,
    pub last_file_area: i16,
    pub last_conference: u8,
    pub user_flags: UserFlags,
    pub board_scan_flags: Vec<Vec<bool>>,  // 20 × 254
    pub access_codes: [u8; 6],
    pub show_message_waiting: bool,
}
```

**Layer 3 - Conversion (`legacy/conversions.rs`):**
```rust
impl From<PascalUserRecord> for User {
    fn from(pascal: PascalUserRecord) -> Self {
        User {
            // Generate new ID (will be assigned by DB)
            id: 0,

            // Convert Pascal strings to Rust
            name: pascal_string_to_rust(&pascal.name),
            password_hash: hash_legacy_password(&pascal.pw),  // Convert plaintext!

            real_name: Some(pascal_string_to_rust(&pascal.realname)),
            location: Some(pascal_string_to_rust(&pascal.citystate)),

            // Convert stats
            stats: UserStats {
                calls: pascal.timescalled as u32,
                uploads: pascal.uploads as u32,
                downloads: pascal.downloads as u32,
                upload_kb: pascal.uk as u64,
                download_kb: pascal.dk as u64,
                messages_posted: pascal.msgpost as u32,
                time_online: pascal.ttimeon as u32,
            },

            // Convert dates
            registration_date: parse_pascal_date(&pascal.firston).unwrap(),
            last_login: Some(combine_pascal_datetime(&pascal.lastdate, &pascal.laston)),
            birthday: parse_pascal_date(&pascal.bday).ok(),

            // Convert phone
            phone: Some(pascal_string_to_rust(&pascal.ph)),

            // Convert notes
            notes: Some(pascal_string_to_rust(&pascal.note)),

            // Pascal-specific fields
            street: Some(pascal_string_to_rust(&pascal.street)),
            zipcode: Some(pascal_string_to_rust(&pascal.zipcode)),
            download_security_level: pascal.dsl,
            security_level: SecurityLevel::from_byte(pascal.sl),
            access_flags: AccessFlags::from_bytes(&pascal.ar),

            // ... (convert all other fields)

            // Modern-only fields
            email: None,  // Not in Pascal
        }
    }
}

impl TryFrom<&User> for PascalUserRecord {
    type Error = Error;

    fn try_from(user: &User) -> Result<Self> {
        Ok(PascalUserRecord {
            name: rust_string_to_pascal(&user.name, 36)?,
            realname: rust_string_to_pascal(
                &user.real_name.as_deref().unwrap_or(""), 36
            )?,
            pw: [0u8; 21],  // Don't export plaintext passwords!

            // ... (convert all fields back)
        })
    }
}
```

### 1.5 Action Items

1. **Create `legacy/user_record.rs`:**
   - Define `PascalUserRecord` with exact binary layout
   - Add serde Serialize/Deserialize for bincode
   - Calculate and verify struct size matches Pascal

2. **Extend existing `user.rs`:**
   - Add Pascal-specific fields to `User` struct
   - Keep all existing fields and methods
   - Update validation to handle new fields
   - Mark new fields as `#[serde(skip_serializing_if = "Option::is_none")]`

3. **Create conversion layer:**
   - Implement `From<PascalUserRecord> for User`
   - Implement `TryFrom<&User> for PascalUserRecord`
   - Add helper functions for string/date conversion
   - Add password migration logic (hash plaintext on import)

4. **Write tests:**
   - Binary serialization round-trip
   - String conversion edge cases
   - Date conversion edge cases
   - Password hashing
   - Field mapping completeness

---

## 2. FileEntry Type Reconciliation

### 2.1 Current Rust Implementation

**Location:** `crates/impulse-types/src/file.rs`

```rust
pub struct FileEntry {
    pub id: u32,                    // NOT in Pascal
    pub filename: String,           // Pascal: filename: string[12]
    pub description: String,        // Pascal: description: string[58]
    pub uploader: String,           // Pascal: stowner: string[29]
    pub uploader_id: u32,           // Pascal: owner: integer (but as word)
    pub size_bytes: u64,            // Pascal: blocks: word (512-byte blocks)
    pub upload_date: DateTime<Utc>, // Pascal: uldate: string[8] (MM/DD/YY)
    pub area_id: u32,               // NOT in Pascal (implicit by file location)
    pub download_count: u32,        // Pascal: downloaded: word
    pub is_offline: bool,           // Pascal: offline: boolean
    pub is_missing: bool,           // Pascal: notval: boolean
    pub password: Option<String>,   // NOT in Pascal
    pub cost_credits: Option<u32>,  // NOT in Pascal
}
```

### 2.2 Pascal ulrec Definition

**Location:** `source/RECORDS.PAS` (lines 413-438)

```pascal
ulrec=  { EXTENDED.DIR : Extended file information }
record
  name         :string[58];      { File description }
  filename     :string[12];      { Filename (8.3 format) }
  ft           :byte;            { File type/category }
  blocks       :word;            { Size in 512-byte blocks }
  owner        :word;            { Uploader user number }
  stowner      :string[29];      { Uploader name }
  ultime       :string[5];       { Upload time (HH:MM) }
  uldate       :string[8];       { Upload date (MM/DD/YY) }
  downloaded   :word;            { Download count }
  notval       :boolean;         { Not validated }
  offline      :boolean;         { Offline (on tape/disk) }
  credit       :word;            { Credit value }
  res          :byte;            { Reserved }
  lastdl       :string[8];       { Last download date }

  { Note: Pascal also has ulfrec for extended metadata }
end;
```

### 2.3 Field Mapping Analysis

| Pascal Field | Pascal Type | Rust Field | Rust Type | Status | Notes |
|--------------|-------------|------------|-----------|---------|-------|
| `name` | string[58] | `description` | String | **Match** | Description/name |
| `filename` | string[12] | `filename` | String | **Match** | Filename |
| `ft` | byte | - | - | **Missing** | File type/category |
| `blocks` | word | `size_bytes` | u64 | **Different** | Blocks × 512 = bytes |
| `owner` | word | `uploader_id` | u32 | **Match** | User ID |
| `stowner` | string[29] | `uploader` | String | **Match** | Username |
| `ultime` | string[5] | - | - | **Missing** | Upload time |
| `uldate` | string[8] | `upload_date` | DateTime\<Utc\> | **Match** | Combined date+time |
| `downloaded` | word | `download_count` | u32 | **Match** | Download count |
| `notval` | boolean | `is_missing` | bool | **Similar** | Validation status |
| `offline` | boolean | `is_offline` | bool | **Match** | Offline flag |
| `credit` | word | `cost_credits` | Option\<u32\> | **Match** | Credit cost |
| `res` | byte | - | - | **Missing** | Reserved byte |
| `lastdl` | string[8] | - | - | **Missing** | Last download date |
| - | - | `id` | u32 | **Extra** | Database ID (not in Pascal) |
| - | - | `area_id` | u32 | **Extra** | File area ID (implicit) |
| - | - | `password` | Option\<String\> | **Extra** | Password protection |

### 2.4 Reconciliation Strategy

**Approach:** Similar to User - two-layer with conversion

**Action Items:**
1. Create `legacy/file_record.rs` with `PascalFileRecord`
2. Extend `file.rs` with missing Pascal fields
3. Create conversion layer
4. Write tests for binary compatibility

---

## 3. Message Type Reconciliation

### 3.1 Current Rust Implementation

**Location:** `crates/impulse-types/src/message.rs`

```rust
pub struct Message {
    pub id: u32,                    // NOT in Pascal
    pub from: String,               // Pascal: from: string[35]
    pub to: String,                 // Pascal: _to: string[35]
    pub subject: String,            // Pascal: subj: string[65]
    pub body: String,               // Pascal: (stored separately in .MSG file)
    pub date: DateTime<Utc>,        // Pascal: date: string[8] + time: string[5]
    pub area_id: u32,               // NOT in Pascal (implicit by board)
    pub parent_id: Option<u32>,     // Pascal: replyto: word
    pub is_read: bool,              // Pascal: (tracked in user's smalrec)
    pub is_private: bool,           // Pascal: validated: integer (flags)
    pub is_deleted: bool,           // Pascal: (marked by deletion)
}
```

### 3.2 Pascal msgindexrec Definition

**Location:** `source/RECORDS.PAS` (lines 295-313)

```pascal
msgindexrec=  { MSGINDEX.BBS : Message index }
record
  msgnum       :longint;         { Message number }
  from         :string[35];      { Sender name }
  _to          :string[35];      { Recipient name }
  date         :string[8];       { Date posted (MM/DD/YY) }
  time         :string[5];       { Time posted (HH:MM) }
  subj         :string[65];      { Subject }
  replyto      :word;            { Reply-to message# }
  msgptr       :longint;         { Pointer to message text }
  msgid        :longint;         { Message ID (for threading) }
  validated    :integer;         { Status flags }
  network      :byte;            { Network type }
end;
```

### 3.3 Field Mapping Analysis

| Pascal Field | Pascal Type | Rust Field | Rust Type | Status | Notes |
|--------------|-------------|------------|-----------|---------|-------|
| `msgnum` | longint | `id` | u32 | **Match** | Message number |
| `from` | string[35] | `from` | String | **Match** | Sender |
| `_to` | string[35] | `to` | String | **Match** | Recipient |
| `date` | string[8] | `date` | DateTime\<Utc\> | **Match** | Combined date+time |
| `time` | string[5] | - | - | **Missing** | Part of date |
| `subj` | string[65] | `subject` | String | **Match** | Subject |
| `replyto` | word | `parent_id` | Option\<u32\> | **Match** | Reply-to |
| `msgptr` | longint | - | - | **Missing** | Text file pointer |
| `msgid` | longint | - | - | **Missing** | Threading ID |
| `validated` | integer | - | - | **Missing** | Status flags |
| `network` | byte | - | - | **Missing** | Network type |
| - | - | `body` | String | **Extra** | Loaded from .MSG file |
| - | - | `area_id` | u32 | **Extra** | Board/area ID |
| - | - | `is_read` | bool | **Extra** | Read status (per-user) |
| - | - | `is_private` | bool | **Extra** | Privacy flag |
| - | - | `is_deleted` | bool | **Extra** | Deletion flag |

### 3.4 Reconciliation Strategy

**Notes:**
- Pascal splits message metadata (index) from body (separate file)
- Modern design combines them for simplicity
- Need separate `PascalMessageIndex` and body loading logic

**Action Items:**
1. Create `legacy/message_index.rs` with `PascalMessageIndex`
2. Create message body loader (reads `.MSG` files)
3. Extend `message.rs` with Pascal fields
4. Create conversion layer

---

## 4. BbsConfig Type Reconciliation

### 4.1 Current Rust Implementation

**Location:** `crates/impulse-types/src/config.rs`

```rust
pub struct BbsConfig {
    pub name: String,
    pub sysop: String,
    pub sysop_email: Option<String>,
    pub location: Option<String>,
    pub servers: Vec<ServerConfig>,
    pub paths: BbsPaths,
    pub limits: SystemLimits,
    pub security: SecuritySettings,
    pub enable_web_admin: bool,
    pub web_admin_port: u16,
    pub enable_ansi: bool,
    pub enable_utf8: bool,
    pub tagline: Option<String>,
}
```

### 4.2 Pascal systatrec Definition

**Location:** `source/RECORDS.PAS` (lines 568-721, 153 lines!)

```pascal
systatrec=  { STATUS.DAT : System configuration }
record
  gfilepath     :string[79];      { General files path }
  afilepath     :string[79];      { Archive files path }
  // ... 150+ more lines of configuration fields
  bbsname       :string[40];      { BBS name }
  bbsphone      :string[12];      { BBS phone number }
  sysopname     :string[30];      { SysOp name }
  // ... many, many more fields
end;
```

### 4.3 Field Mapping Analysis

**Status:** **COMPLEX - Requires detailed analysis**

Pascal `systatrec` is 153 lines with 100+ configuration fields covering:
- File paths (20+ fields)
- Message board settings (30+ fields)
- Security settings (15+ fields)
- Display settings (10+ fields)
- Network settings (20+ fields)
- System limits (15+ fields)

Current Rust `BbsConfig` has ~13 top-level fields organized into nested structures.

### 4.4 Reconciliation Strategy

**Approach:**
1. Current `BbsConfig` is well-structured for modern use
2. Create separate `PascalSystemConfig` for exact binary compatibility
3. Map Pascal fields to modern nested structure during conversion
4. Many Pascal fields are obsolete (DOS paths, modem settings) - make optional

**Action Items:**
1. Create `legacy/system_config.rs` with full Pascal layout
2. Document field-by-field mapping
3. Create conversion layer with intelligent defaults
4. Mark obsolete fields as optional in modern type

---

## 5. Implementation Plan

### Phase 1: Foundation (Days 1-2)
1. Create `legacy/` module structure
2. Create helper functions:
   - `pascal_string_to_rust()`
   - `rust_string_to_pascal()`
   - `parse_pascal_date()`
   - `combine_pascal_datetime()`
   - `hash_legacy_password()`

### Phase 2: User Type (Days 3-5)
1. Create `legacy/user_record.rs`
2. Extend existing `user.rs`
3. Implement conversions
4. Write tests

### Phase 3: FileEntry Type (Days 6-7)
1. Create `legacy/file_record.rs`
2. Extend existing `file.rs`
3. Implement conversions
4. Write tests

### Phase 4: Message Type (Days 8-9)
1. Create `legacy/message_index.rs`
2. Create message body loader
3. Extend existing `message.rs`
4. Implement conversions
5. Write tests

### Phase 5: BbsConfig Type (Days 10-12)
1. Create `legacy/system_config.rs`
2. Document full field mapping
3. Extend existing `config.rs` (selective)
4. Implement conversions
5. Write tests

### Phase 6: Testing & Verification (Days 13-14)
1. Binary compatibility tests
2. Round-trip tests (Pascal → Rust → Pascal)
3. Integration tests
4. Performance tests

---

## 6. Testing Strategy

### 6.1 Unit Tests

**For each type:**
1. Binary serialization (bincode)
2. JSON serialization
3. Validation
4. Conversion (Pascal → Rust)
5. Conversion (Rust → Pascal)
6. String edge cases (truncation, UTF-8, empty)
7. Date edge cases (Y2K, invalid dates)
8. Boolean conversion (0/1 → false/true)

### 6.2 Integration Tests

1. Load real Pascal `.DAT` files (if available)
2. Verify all fields parse correctly
3. Round-trip: Load → Convert → Save → Load → Compare
4. Verify struct sizes match Pascal compiler output

### 6.3 Property-Based Tests

Using `proptest`:
1. Generate random Pascal structs
2. Convert to Rust
3. Convert back to Pascal
4. Verify equality (accounting for truncation)

---

## 7. Migration Path

### 7.1 For Existing BBS Operators

**Option 1: Data Migration (Recommended)**
```bash
# Use migration tool to convert all .DAT files
impulse-cli migrate --from-pascal ./data --to ./new-data

# Result: New format files with all data preserved
# Old files remain untouched
```

**Option 2: Live Conversion**
```rust
// System automatically detects and converts on-the-fly
// First read: Pascal format
// First write: Modern format
// Transparent to user
```

### 7.2 Backward Compatibility

**Reading:** Support both formats
- Try modern format first (JSON or bincode)
- Fall back to Pascal format if not found
- Convert and cache in modern format

**Writing:** Modern format only
- Always write in modern JSON/bincode
- Never write Pascal format (deprecated)
- Keep original Pascal files as backup

---

## 8. Security Considerations

### 8.1 Password Migration

**Pascal Issue:** Passwords stored in plaintext!

```pascal
pw: string[20];  // PLAINTEXT PASSWORD!
```

**Rust Solution:** Hash on first read

```rust
impl From<PascalUserRecord> for User {
    fn from(pascal: PascalUserRecord) -> Self {
        let plaintext = pascal_string_to_rust(&pascal.pw);

        User {
            // Hash the plaintext password using Argon2id
            password_hash: hash_legacy_password(&plaintext),
            // ... other fields
        }
    }
}
```

### 8.2 Data Sanitization

**Issues in Pascal:**
- No input validation
- No length checks
- No character encoding (CP437/ASCII)
- No SQL injection protection

**Rust Solution:**
- Validate all strings on import
- Sanitize special characters
- Convert to UTF-8
- Validate against schema

---

## 9. Dependencies

### 9.1 New Dependencies Required

Add to `impulse-types/Cargo.toml`:

```toml
[dependencies]
# Existing
serde = { version = "1.0", features = ["derive"] }
chrono = { version = "0.4", features = ["serde"] }
thiserror = "2.0"

# NEW for Pascal compatibility
bincode = "1.3"            # Binary serialization
bitflags = "2.4"           # For set types
argon2 = "0.5"             # Password hashing
encoding_rs = "0.8"        # CP437 → UTF-8 conversion
hex = "0.4"                # Binary debugging
```

---

## 10. Documentation Requirements

### 10.1 Rustdoc

Each type needs:
1. Module-level docs explaining Pascal origin
2. Struct-level docs with field descriptions
3. Examples showing conversion
4. Migration guide section

### 10.2 Migration Guide

Create `docs/MIGRATION_GUIDE.md`:
1. How to migrate existing BBS data
2. Password security notice
3. Backup procedures
4. Rollback procedures
5. Common issues and solutions

---

## Summary

**Reconciliation Complete:** All 4 core types analyzed

**Strategy:** Two-layer architecture
- Layer 1: Pascal-compatible binary structs
- Layer 2: Modern Rust types (existing + extended)
- Layer 3: Conversion implementations

**Next Steps:**
1. Implement helper functions (Phase 1)
2. Convert User type (Phase 2)
3. Convert FileEntry type (Phase 3)
4. Convert Message type (Phase 4)
5. Convert BbsConfig type (Phase 5)
6. Comprehensive testing (Phase 6)

**Timeline:** 14 days for full reconciliation and testing

**Risk:** LOW - Two-layer approach preserves existing code while adding compatibility
