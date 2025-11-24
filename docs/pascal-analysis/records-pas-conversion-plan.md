# RECORDS.PAS Type Conversion Plan

**Sprint 5: Core Types Implementation**
**Source:** `/source/RECORDS.PAS` (829 lines, 40+ types)
**Target:** `crates/impulse-types/src/` (multiple modules)
**Date:** 2025-11-23

---

## Executive Summary

RECORDS.PAS is the foundational type definition unit for Impulse 7.1 BBS. It contains:
- **40+ type definitions** across 829 lines
- **10 enumeration types** (88 total variants)
- **3 major core types** (userrec, systatrec, fstringrec)
- **20+ record structures** for message, file, user, and system management
- **0 dependencies** on other Pascal units
- **93 modules depend on it** (highest impact in codebase)

**Conversion Strategy:** Phase-based modular approach with binary compatibility layer.

---

## Constants Mapping

### System Constants (lines 5-19)

| Pascal Constant | Value | Rust Mapping | Module |
|----------------|-------|--------------|--------|
| `progname` | "Impulse" | `pub const PROG_NAME: &str = "Impulse";` | `impulse-types/constants.rs` |
| `acronym` | "imp" | `pub const ACRONYM: &str = "imp";` | " |
| `ver` | "7.1" | `pub const VERSION: &str = "7.1";` | " |
| `copyright` | "(c)1993, 98 by Brandon Sneed (Nivenh)" | `pub const COPYRIGHT: &str = "...";` | " |
| `verdate` | "7/27/98" | `pub const VERSION_DATE: &str = "7/27/98";` | " |
| `maxboards` | 254 | `pub const MAX_BOARDS: usize = 254;` | " |
| `maxconfs` | 20 | `pub const MAX_CONFERENCES: usize = 20;` | " |
| `maxuboards` | 254 | `pub const MAX_FILE_AREAS: usize = 254;` | " |
| `maxprotocols` | 20 | `pub const MAX_PROTOCOLS: usize = 20;` | " |
| `maxevents` | 10 | `pub const MAX_EVENTS: usize = 10;` | " |
| `maxubatchfiles` | 20 | `pub const MAX_BATCH_FILES: usize = 20;` | " |
| `maxmenucmds` | 50 | `pub const MAX_MENU_COMMANDS: usize = 50;` | " |
| `maxBufSize` | 4096 | `pub const MAX_BUFFER_SIZE: usize = 4096;` | " |
| `maxStrings` | 255 | `pub const MAX_STRINGS: usize = 255;` | " |

**Notes:**
- All constants are compile-time constants in Rust
- Use `usize` for array sizes (required by Rust)
- Document Pascal compatibility in comments

---

## Type Aliases Mapping (lines 22-25)

| Pascal Type | Definition | Rust Type | Module | Notes |
|------------|------------|-----------|--------|-------|
| `astr` | `string[160]` | `String` or `[u8; 160]` | `impulse-types/pascal_compat.rs` | For binary compat, use `[u8; 160]` |
| `str80` | `string[80]` | `String` or `[u8; 80]` | " | Common string size |
| `acstring` | `string[20]` | `String` or `[u8; 20]` | " | Access Condition String |
| `acrq` | `'@'..'Z'` | `u8` with validation | " | AR flags (64-90 ASCII range) |

**Implementation Strategy:**
```rust
// Type aliases for Pascal compatibility
pub type AStr = [u8; 160];
pub type Str80 = [u8; 80];
pub type AcString = [u8; 20];

// AR flag type (character range '@'..'Z' = 64-90)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArFlag(u8);

impl ArFlag {
    pub fn new(c: char) -> Result<Self> {
        let byte = c as u8;
        if (64..=90).contains(&byte) { // '@' to 'Z'
            Ok(ArFlag(byte))
        } else {
            Err(Error::Validation("AR flag must be '@'-'Z'".into()))
        }
    }
}
```

---

## Enumeration Types (10 enums, 88 variants)

### 1. User Flags (`uflags`, 24 variants, lines 27-51)

**Pascal Definition:**
```pascal
uflags = (rlogon, rchat, rvalidate, rbackspace, ramsg, rpostan, rpost,
          remail, rvoting, rmsg, spcsr, onekey, avatar, pause, novice,
          ansi, color, alert, smw, nomail, fnodlratio, fnopostratio,
          fnofilepts, fnodeletion);
```

**Rust Conversion:**
```rust
use bitflags::bitflags;

bitflags! {
    /// User account flags
    ///
    /// Controls restrictions, capabilities, and preferences for user accounts.
    /// Uses bitflags for efficient storage and Pascal binary compatibility.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct UserFlags: u32 {
        /// L - Restricted to one call a day
        const RESTRICTED_LOGON      = 0b0000_0000_0000_0000_0000_0000_0000_0001;
        /// C - Can't page the SysOp
        const RESTRICTED_CHAT       = 0b0000_0000_0000_0000_0000_0000_0000_0010;
        /// V - Posts marked unvalidated
        const RESTRICTED_VALIDATE   = 0b0000_0000_0000_0000_0000_0000_0000_0100;
        /// B - Can't do ^B/^N/etc in messages
        const RESTRICTED_BACKSPACE  = 0b0000_0000_0000_0000_0000_0000_0000_1000;
        /// A - Can't change the AutoMessage
        const RESTRICTED_AUTOMSG    = 0b0000_0000_0000_0000_0000_0000_0001_0000;
        /// * - Can't post anonymously
        const RESTRICTED_POST_ANON  = 0b0000_0000_0000_0000_0000_0000_0010_0000;
        /// P - Can't post at all
        const RESTRICTED_POST       = 0b0000_0000_0000_0000_0000_0000_0100_0000;
        /// E - Can't send any e-mail
        const RESTRICTED_EMAIL      = 0b0000_0000_0000_0000_0000_0000_1000_0000;
        /// K - Can't vote
        const RESTRICTED_VOTING     = 0b0000_0000_0000_0000_0000_0001_0000_0000;
        /// M - Force e-mail deletion
        const RESTRICTED_MSG        = 0b0000_0000_0000_0000_0000_0010_0000_0000;
        /// Special cursor mode
        const SPECIAL_CURSOR        = 0b0000_0000_0000_0000_0000_0100_0000_0000;
        /// Onekey input mode
        const ONEKEY                = 0b0000_0000_0000_0000_0000_1000_0000_0000;
        /// User has AVATAR terminal emulation
        const AVATAR                = 0b0000_0000_0000_0000_0001_0000_0000_0000;
        /// Pause enabled
        const PAUSE                 = 0b0000_0000_0000_0000_0010_0000_0000_0000;
        /// User at novice help level
        const NOVICE                = 0b0000_0000_0000_0000_0100_0000_0000_0000;
        /// User has ANSI terminal emulation
        const ANSI                  = 0b0000_0000_0000_0000_1000_0000_0000_0000;
        /// User has color enabled
        const COLOR                 = 0b0000_0000_0000_0001_0000_0000_0000_0000;
        /// Alert SysOp when user logs on
        const ALERT                 = 0b0000_0000_0000_0010_0000_0000_0000_0000;
        /// Short-message waiting for user
        const SHORT_MESSAGE_WAITING = 0b0000_0000_0000_0100_0000_0000_0000_0000;
        /// User mail-box is closed
        const NO_MAIL               = 0b0000_0000_0000_1000_0000_0000_0000_0000;
        /// Flag 1 - No UL/DL ratio
        const FLAG_NO_DL_RATIO      = 0b0000_0000_0001_0000_0000_0000_0000_0000;
        /// Flag 2 - No post/call ratio
        const FLAG_NO_POST_RATIO    = 0b0000_0000_0010_0000_0000_0000_0000_0000;
        /// Flag 3 - No file points checking
        const FLAG_NO_FILE_PTS      = 0b0000_0000_0100_0000_0000_0000_0000_0000;
        /// Flag 4 - Protection from deletion
        const FLAG_NO_DELETION      = 0b0000_0000_1000_0000_0000_0000_0000_0000;
    }
}
```

**Module:** `crates/impulse-types/src/user_flags.rs`

**Binary Compatibility:** Pascal `set of uflags` is stored as 32-bit bitset. Rust bitflags u32 is binary compatible.

---

### 2. File List Flags (`flistflags`, 8 variants, lines 53-61)

**Pascal Definition:**
```pascal
flistflags = (fname, fsize, fpts, fdesc, extdesc, whoul, dateul, numdl);
```

**Rust Conversion:**
```rust
bitflags! {
    /// File list display flags
    ///
    /// Controls which columns are displayed in file listings.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct FileListFlags: u8 {
        /// Display filename
        const FILENAME         = 0b0000_0001;
        /// Display file size
        const SIZE             = 0b0000_0010;
        /// Display file points
        const POINTS           = 0b0000_0100;
        /// Display file description
        const DESCRIPTION      = 0b0000_1000;
        /// Display extended description
        const EXTENDED_DESC    = 0b0001_0000;
        /// Display uploader name
        const UPLOADER         = 0b0010_0000;
        /// Display upload date
        const UPLOAD_DATE      = 0b0100_0000;
        /// Display download count
        const DOWNLOAD_COUNT   = 0b1000_0000;
    }
}
```

**Module:** `crates/impulse-types/src/file_flags.rs`

---

### 3. Anonymous Type (`anontyp`, 5 variants, lines 64-69)

**Pascal Definition:**
```pascal
anontyp = (atno, atyes, atforced, atdearabby, atanyname);
```

**Rust Conversion:**
```rust
/// Anonymous posting type for message boards
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum AnonymousType {
    /// No anonymous posts allowed
    NotAllowed = 0,
    /// Anonymous posts are allowed
    Allowed = 1,
    /// ALL posts are forced anonymous
    Forced = 2,
    /// "Dear Abby" style message base (special formatting)
    DearAbby = 3,
    /// Users can post as ANY name they want
    AnyName = 4,
}
```

**Module:** `crates/impulse-types/src/message_types.rs`

**Binary Compatibility:** Use `#[repr(u8)]` to match Pascal enum storage (0-based byte).

---

### 4. Message Index Status (`msgindexstatr`, 8 variants, lines 235-243)

**Pascal Definition:**
```pascal
msgindexstatr = (miexist, miencrypted, miunvalidated, mipermanent,
                 miallowmci, mithreads, mimassmail, miscanned);
```

**Rust Conversion:**
```rust
bitflags! {
    /// Message index status flags
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct MessageIndexStatus: u8 {
        /// Does message actually exist?
        const EXISTS        = 0b0000_0001;
        /// Is it encrypted?
        const ENCRYPTED     = 0b0000_0010;
        /// Is message unvalidated?
        const UNVALIDATED   = 0b0000_0100;
        /// Is the message permanent?
        const PERMANENT     = 0b0000_1000;
        /// Did owner have access to MCI codes?
        const ALLOW_MCI     = 0b0001_0000;
        /// Is message referenced? (threaded)
        const THREADS       = 0b0010_0000;
        /// Is it private, mass mail?
        const MASS_MAIL     = 0b0100_0000;
        /// Is message scanned for FidoNet?
        const SCANNED       = 0b1000_0000;
    }
}
```

**Module:** `crates/impulse-types/src/message_types.rs`

---

### 5. Message Board Flags (`mbflags`, 13 variants, lines 590-603)

**Pascal Definition:**
```pascal
mbflags = (mbunhidden, mbrealname, mbisdir, mbmsgpath, mbfilter,
           mbskludge, mbsseenby, mbsorigin, mbscenter, mbsbox,
           mbmcenter, mbaddtear, mbtopstar);
```

**Rust Conversion:**
```rust
bitflags! {
    /// Message board configuration flags
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct MessageBoardFlags: u16 {
        /// Whether *VISIBLE* to users without access
        const UNHIDDEN          = 0b0000_0000_0000_0001;
        /// Whether real names are forced
        const REAL_NAME         = 0b0000_0000_0000_0010;
        /// If a message base directory
        const IS_DIRECTORY      = 0b0000_0000_0000_0100;
        /// If *.BRD file stored in MSGPATH
        const IN_MSG_PATH       = 0b0000_0000_0000_1000;
        /// Whether to filter ANSI/8-bit ASCII
        const FILTER            = 0b0000_0000_0001_0000;
        /// Strip IFNA kludge lines
        const STRIP_KLUDGE      = 0b0000_0000_0010_0000;
        /// Strip SEEN-BY lines
        const STRIP_SEENBY      = 0b0000_0000_0100_0000;
        /// Strip origin lines
        const STRIP_ORIGIN      = 0b0000_0000_1000_0000;
        /// Strip centering codes
        const STRIP_CENTER      = 0b0000_0001_0000_0000;
        /// Strip box codes
        const STRIP_BOX         = 0b0000_0010_0000_0000;
        /// Center boxed/centered lines
        const CENTER_LINES      = 0b0000_0100_0000_0000;
        /// Add tear/origin lines
        const ADD_TEAR          = 0b0000_1000_0000_0000;
        /// Whether XBase for XMail base
        const TOPSTAR           = 0b0001_0000_0000_0000;
    }
}
```

**Module:** `crates/impulse-types/src/message_types.rs`

---

### 6. File Board Flags (`fbflags`, 6 variants, lines 633-639)

**Pascal Definition:**
```pascal
fbflags = (fbnoratio, fbunhidden, fbdirdlpath, fbisdir, fbusegifspecs, fbnetlink);
```

**Rust Conversion:**
```rust
bitflags! {
    /// File board configuration flags
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct FileBoardFlags: u8 {
        /// If <No Ratio> active
        const NO_RATIO          = 0b0000_0001;
        /// Whether *VISIBLE* to users without access
        const UNHIDDEN          = 0b0000_0010;
        /// If *.DIR file stored in DLPATH
        const DIR_IN_DL_PATH    = 0b0000_0100;
        /// If a file base directory
        const IS_DIRECTORY      = 0b0000_1000;
        /// Whether to use GifSpecs
        const USE_GIF_SPECS     = 0b0001_0000;
        /// Whether Net-Linked to other Impulse BBS's
        const NET_LINK          = 0b0010_0000;
    }
}
```

**Module:** `crates/impulse-types/src/file_types.rs`

---

### 7. Menu Flags (`mnuflags`, 6 variants, lines 712-718)

**Pascal Definition:**
```pascal
mnuflags = (clrscrbefore, dontcenter, nomenuprompt, forcepause,
            pulldown, autotime);
```

**Rust Conversion:**
```rust
bitflags! {
    /// Menu configuration flags
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct MenuFlags: u8 {
        /// C: Clear screen before menu display
        const CLEAR_SCREEN      = 0b0000_0001;
        /// D: Don't center the menu titles
        const DONT_CENTER       = 0b0000_0010;
        /// N: No menu prompt whatsoever
        const NO_PROMPT         = 0b0000_0100;
        /// F: Force a pause before menu display
        const FORCE_PAUSE       = 0b0000_1000;
        /// P: Pulldown menu flag
        const PULLDOWN          = 0b0001_0000;
        /// T: Is time displayed automatically?
        const AUTO_TIME         = 0b0010_0000;
    }
}
```

**Module:** `crates/impulse-types/src/menu_types.rs`

---

### 8. Command Flags (`cmdflags`, 3 variants, lines 736-739)

**Pascal Definition:**
```pascal
cmdflags = (hidden, pull, unhidden);
```

**Rust Conversion:**
```rust
bitflags! {
    /// Menu command flags
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct CommandFlags: u8 {
        /// H: Is command ALWAYS hidden?
        const HIDDEN        = 0b0000_0001;
        /// P: Is command flagged as Pulldown Active?
        const PULLDOWN      = 0b0000_0010;
        /// U: Is command ALWAYS visible?
        const UNHIDDEN      = 0b0000_0100;
    }
}
```

**Module:** `crates/impulse-types/src/menu_types.rs`

---

### 9. Protocol Flags (`xbflags`, 4 variants, lines 754-758)

**Pascal Definition:**
```pascal
xbflags = (xbactive, xbisbatch, xbisresume, xbxferokcode);
```

**Rust Conversion:**
```rust
bitflags! {
    /// Transfer protocol flags
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ProtocolFlags: u8 {
        /// Protocol is active
        const ACTIVE            = 0b0000_0001;
        /// Protocol supports batch transfers
        const BATCH             = 0b0000_0010;
        /// Protocol supports resume
        const RESUME            = 0b0000_0100;
        /// Transfer OK code
        const XFER_OK_CODE      = 0b0000_1000;
    }
}
```

**Module:** `crates/impulse-types/src/protocol_types.rs`

---

### 10. File Status (`filstat`, 3 variants, lines 661-664)

**Pascal Definition:**
```pascal
filstat = (notval, isrequest, resumelater);
```

**Rust Conversion:**
```rust
bitflags! {
    /// File status flags
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct FileStatus: u8 {
        /// File is NOT validated
        const NOT_VALIDATED     = 0b0000_0001;
        /// File is REQUEST
        const IS_REQUEST        = 0b0000_0010;
        /// File is RESUME-LATER
        const RESUME_LATER      = 0b0000_0100;
    }
}
```

**Module:** `crates/impulse-types/src/file_types.rs`

---

## Array and Specialized Types

### Color Records (`clrs`, line 71)

**Pascal Definition:**
```pascal
clrs = array[FALSE..TRUE,0..9] of byte;
```

**Rust Conversion:**
```rust
/// User color configuration (2x10 byte array)
///
/// First dimension: [Normal, Bright] = [false, true]
/// Second dimension: [0..9] = color codes
pub type ColorArray = [[u8; 10]; 2];
```

**Module:** `crates/impulse-types/src/terminal_types.rs`

---

### Security Range (`secrange`, line 72)

**Pascal Definition:**
```pascal
secrange = array[0..255] of integer;
```

**Rust Conversion:**
```rust
/// Security level table (256 entries of i16)
pub type SecurityRange = [i16; 256];
```

**Module:** `crates/impulse-types/src/security_types.rs`

---

### Packed Date/Time (`cpackdatetime`, line 74)

**Pascal Definition:**
```pascal
cpackdatetime = array[1..6] of byte;
```

**Rust Conversion:**
```rust
/// Packed date/time (6 bytes: year, month, day, hour, minute, second)
///
/// Pascal uses 1-based indexing, Rust uses 0-based.
/// Binary layout: [year, month, day, hour, minute, second]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackedDateTime {
    /// Year (1980-2099, stored as offset from 1900)
    pub year: u8,
    /// Month (1-12)
    pub month: u8,
    /// Day (1-31)
    pub day: u8,
    /// Hour (0-23)
    pub hour: u8,
    /// Minute (0-59)
    pub minute: u8,
    /// Second (0-59)
    pub second: u8,
}

impl PackedDateTime {
    /// Convert to chrono::NaiveDateTime
    pub fn to_naive_datetime(&self) -> Result<chrono::NaiveDateTime> {
        let year = 1900 + self.year as i32;
        chrono::NaiveDate::from_ymd_opt(year, self.month as u32, self.day as u32)
            .and_then(|date| date.and_hms_opt(self.hour as u32, self.minute as u32, self.second as u32))
            .ok_or_else(|| Error::Validation("Invalid packed datetime".into()))
    }

    /// Create from chrono::NaiveDateTime
    pub fn from_naive_datetime(dt: chrono::NaiveDateTime) -> Result<Self> {
        let year = dt.year() - 1900;
        if !(0..=255).contains(&year) {
            return Err(Error::Validation("Year out of range (1900-2155)".into()));
        }
        Ok(PackedDateTime {
            year: year as u8,
            month: dt.month() as u8,
            day: dt.day() as u8,
            hour: dt.hour() as u8,
            minute: dt.minute() as u8,
            second: dt.second() as u8,
        })
    }
}
```

**Module:** `crates/impulse-types/src/datetime_types.rs`

---

### Message/File Scan Sets (lines 76-78)

**Pascal Definition:**
```pascal
mzscanr = set of 1..maxboards;      // Message board scan set
fzscanr = set of 0..maxuboards;     // File area scan set
mhireadr = array[1..maxboards] of cpackdatetime;  // High-read pointers
```

**Rust Conversion:**
```rust
/// Message board scan set (254 boards, bitset)
///
/// Pascal `set of 1..254` requires 32 bytes (256 bits).
/// We use bit_vec or simple byte array.
pub type MessageScanSet = [u8; 32]; // 256 bits / 8 = 32 bytes

/// File area scan set (254 areas, bitset)
pub type FileScanSet = [u8; 32];

/// Message high-read pointers (array of packed datetimes)
pub type MessageHighReadArray = [PackedDateTime; MAX_BOARDS];
```

**Module:** `crates/impulse-types/src/scan_types.rs`

**Note:** Pascal sets use bitsets. For binary compatibility, use byte arrays with bit manipulation.

---

## Major Record Types

### Priority Classification

**Tier 1 - CRITICAL (implement first):**
1. **userrec** (lines 143-226, 83 lines) - User account record
2. **systatrec** (lines 435-588, 153 lines) - System configuration
3. **boardrec** (lines 605-631) - Message board configuration
4. **ulrec** (lines 641-659) - File area configuration

**Tier 2 - HIGH (implement second):**
5. **msgindexrec** (lines 245-256) - Message index
6. **mheaderrec** (lines 267-276) - Message header
7. **ulfrec** (lines 666-683) - File record
8. **zscanrec** (lines 228-233) - NewScan data
9. **eventrec** (lines 698-710) - Event scheduler

**Tier 3 - MEDIUM (implement third):**
10. **fstringrec** (lines 307-430, 123 lines) - UI strings (HUGE)
11. **menurec** (lines 721-734) - Menu definition
12. **commandrec** (lines 741-752) - Menu command
13. **protrec** (lines 760-776) - Protocol configuration
14. **modemrec** (lines 287-305) - Modem configuration

**Tier 4 - LOW (implement last):**
15-35. Supporting records (smalrec, zlogrec, noderec, etc.)

---

## Tier 1: CRITICAL Records

### 1. User Record (`userrec`, lines 143-226, 83 lines)

**Pascal Structure:**
```pascal
userrec = record
  name         :string[36];              { user name        }
  realname     :string[36];              { real name        }
  pw           :string[20];              { user password    }
  ph           :string[12];              { user phone #     }
  bday         :string[8];               { user birthdate   }
  firston      :string[8];               { firston date     }
  x1xs         :array[1..2] of byte;
  laston       :string[8];               { laston date      }
  x2xs         :array[1..2] of byte;
  street       :string[30];              { mailing address  }
  citystate    :string[30];              { city, state      }
  zipcode      :string[10];              { zipcode          }
  unused       :array [1..31] of byte;
  autosig      :string[40];              { autosig          }
  unused2      :array [1..41] of byte;
  note         :string[39];              { SysOp note       }
  prompt       :byte;                    { Chosen Menu Prompt }
  lockedout    :boolean;                 { if locked out    }
  deleted      :boolean;                 { if deleted       }
  lockedfile   :string[8];               { lockout msg to print }
  novotes, yesvotes :byte;               { # of yes/no votes in NUV }
  ac           :set of uflags;           { user flags   }
  fflag        :set of flistflags;       { users configable file list }
  ar           :set of acrq;             { AR flags     }
  zzqscan      :array[1..64] of word;    { last read msg pointers }
  xqxxx        :array[1..64] of word;
  zzqscn       :array[1..64] of boolean; { scan boards flags  }
  zzdlnscn     :dlnscan;                 { scan uboards flags }
  unused3      :array[1..20] of byte;
  sex          :char;                    { user sex }
  ttimeon      :longint;                 { total mins spent on  }
  x1xx         :integer;
  uk           :longint;                 { UL k                 }
  x2xx         :integer;
  dk           :longint;                 { DL k                 }
  x3xx         :integer;
  uploads, downloads, loggedon, tltoday, msgpost, emailsent,
  feedback, forusr, filepoints :integer;
  waiting, linelen, pagelen, ontoday, illegal, sl, dsl :byte;
  cols         :clrs;                    { user colors }
  lastmsg, lastfil :byte;                { last msg/file areas   }
  credit       :longint;                 { $$$ credit in dollars }
  x4xx         :integer;
  timebank     :integer;                 { # mins in Time Bank   }
  boardsysop   :array[1..5] of byte;     { msg board SysOp       }
  trapactivity, trapseperate :boolean;
  timebankadd  :integer;                 { time added to timebank TODAY }
  mpointer     :longint;                 { *REMOVED* }
  chatauto, chatseperate :boolean;
  userstartmenu:string[8];               { menu to start user out on }
  slogseperate :boolean;                 { seperate SysOp log? }
  clsmsg       :byte;                    { 1 if clear-screen msg, 2 if not }
  flistopt     :byte;                    { type of file list type to use }
  msgorder     :byte;                    { 0:Chrono, 1:ReplyTree }
  avadjust     :byte;                    { AVATAR color adjust: 1=no, 2=yes }
end;
```

**Rust Conversion (Two Layers):**

**Layer 1: Pascal-Compatible Binary Structure**
```rust
/// Pascal-compatible user record for binary file I/O
///
/// This struct maintains exact binary layout compatibility with Pascal's
/// userrec for reading/writing USER.LST files.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(C, packed)]
pub struct PascalUserRecord {
    pub name: [u8; 37],           // String[36] = 1 length byte + 36 chars
    pub realname: [u8; 37],       // String[36]
    pub pw: [u8; 21],             // String[20]
    pub ph: [u8; 13],             // String[12]
    pub bday: [u8; 9],            // String[8]
    pub firston: [u8; 9],         // String[8]
    pub x1xs: [u8; 2],            // Unused
    pub laston: [u8; 9],          // String[8]
    pub x2xs: [u8; 2],            // Unused
    pub street: [u8; 31],         // String[30]
    pub citystate: [u8; 31],      // String[30]
    pub zipcode: [u8; 11],        // String[10]
    pub unused: [u8; 31],         // Unused
    pub autosig: [u8; 41],        // String[40]
    pub unused2: [u8; 41],        // Unused
    pub note: [u8; 40],           // String[39]
    pub prompt: u8,               // Byte
    pub lockedout: u8,            // Boolean (0/1)
    pub deleted: u8,              // Boolean (0/1)
    pub lockedfile: [u8; 9],      // String[8]
    pub novotes: u8,              // Byte
    pub yesvotes: u8,             // Byte
    pub ac: u32,                  // Set of uflags (bitset)
    pub fflag: u8,                // Set of flistflags (bitset)
    pub ar: [u8; 32],             // Set of '@'..'Z' (bitset, 91 bits -> 12 bytes rounded)
    pub zzqscan: [u16; 64],       // Array of Word
    pub xqxxx: [u16; 64],         // Array of Word
    pub zzqscn: [u8; 64],         // Array of Boolean (1 byte each)
    pub zzdlnscn: [u8; 13],       // Set of 0..96 (97 bits -> 13 bytes)
    pub unused3: [u8; 20],        // Unused
    pub sex: u8,                  // Char
    pub ttimeon: i32,             // Longint
    pub x1xx: i16,                // Integer
    pub uk: i32,                  // Longint (Upload KB)
    pub x2xx: i16,                // Integer
    pub dk: i32,                  // Longint (Download KB)
    pub x3xx: i16,                // Integer
    pub uploads: i16,             // Integer
    pub downloads: i16,           // Integer
    pub loggedon: i16,            // Integer
    pub tltoday: i16,             // Integer (Time Left Today)
    pub msgpost: i16,             // Integer
    pub emailsent: i16,           // Integer
    pub feedback: i16,            // Integer
    pub forusr: i16,              // Integer (Forward mail to user #)
    pub filepoints: i16,          // Integer
    pub waiting: u8,              // Byte (Mail waiting)
    pub linelen: u8,              // Byte (Line length)
    pub pagelen: u8,              // Byte (Page length)
    pub ontoday: u8,              // Byte (Times on today)
    pub illegal: u8,              // Byte (Illegal logon attempts)
    pub sl: u8,                   // Byte (Security Level)
    pub dsl: u8,                  // Byte (Download Security Level)
    pub cols: [[u8; 10]; 2],      // clrs (2x10 byte array)
    pub lastmsg: u8,              // Byte (Last message area)
    pub lastfil: u8,              // Byte (Last file area)
    pub credit: i32,              // Longint ($$$ credit)
    pub x4xx: i16,                // Integer
    pub timebank: i16,            // Integer (Mins in Time Bank)
    pub boardsysop: [u8; 5],      // Array[1..5] of Byte
    pub trapactivity: u8,         // Boolean
    pub trapseperate: u8,         // Boolean
    pub timebankadd: i16,         // Integer
    pub mpointer: i32,            // Longint (*REMOVED*)
    pub chatauto: u8,             // Boolean
    pub chatseperate: u8,         // Boolean
    pub userstartmenu: [u8; 9],   // String[8]
    pub slogseperate: u8,         // Boolean
    pub clsmsg: u8,               // Byte
    pub flistopt: u8,             // Byte
    pub msgorder: u8,             // Byte
    pub avadjust: u8,             // Byte
}
```

**Layer 2: Modern Rust Type**
```rust
/// Modern user account record
///
/// This is the primary user type used in the application. It provides:
/// - Proper Rust types (String, chrono::DateTime, etc.)
/// - Type safety and validation
/// - Convenient methods and trait implementations
///
/// Conversion from/to PascalUserRecord is provided for binary file I/O.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// User ID (1-based, matches record number in USER.LST)
    pub id: u32,

    /// Username (handle) - 1-36 characters
    pub name: String,

    /// Real name - 1-36 characters
    pub real_name: String,

    /// Hashed password (Argon2id for new passwords, legacy for old)
    pub password_hash: String,

    /// Phone number (optional, 12 chars max)
    pub phone: Option<String>,

    /// Birthday
    pub birthday: Option<chrono::NaiveDate>,

    /// First logon date
    pub first_on: chrono::NaiveDate,

    /// Last logon date
    pub last_on: chrono::NaiveDate,

    /// Street address (optional, 30 chars max)
    pub street: Option<String>,

    /// City, State (optional, 30 chars max)
    pub city_state: Option<String>,

    /// ZIP code (optional, 10 chars max)
    pub zip_code: Option<String>,

    /// Auto-signature (optional, 40 chars max)
    pub auto_sig: Option<String>,

    /// SysOp note about user (optional, 39 chars max)
    pub sysop_note: Option<String>,

    /// Chosen menu prompt style
    pub prompt: u8,

    /// Locked out flag
    pub locked_out: bool,

    /// Deleted flag (soft delete)
    pub deleted: bool,

    /// Lockout message filename (8 chars max)
    pub lockout_file: Option<String>,

    /// New user voting - No votes received
    pub no_votes: u8,

    /// New user voting - Yes votes received
    pub yes_votes: u8,

    /// User flags (restrictions, capabilities, preferences)
    pub flags: UserFlags,

    /// File list display flags
    pub file_list_flags: FileListFlags,

    /// AR flags (A-Z access requirements)
    pub ar_flags: HashSet<char>,

    /// Last-read message pointers (per board)
    pub last_read_ptrs: Vec<u16>,

    /// Message board scan flags (which boards to scan)
    pub msg_scan_flags: HashSet<u8>,

    /// File area scan flags (which areas to scan)
    pub file_scan_flags: HashSet<u8>,

    /// Sex ('M', 'F', 'O', or ' ' for unspecified)
    pub sex: char,

    /// Total time online (minutes)
    pub total_time_on: u32,

    /// Upload kilobytes
    pub upload_kb: u32,

    /// Download kilobytes
    pub download_kb: u32,

    /// Total uploads
    pub uploads: u16,

    /// Total downloads
    pub downloads: u16,

    /// Times logged on
    pub times_logged_on: u16,

    /// Time left today (minutes)
    pub time_left_today: u16,

    /// Messages posted
    pub messages_posted: u16,

    /// Email sent
    pub email_sent: u16,

    /// Feedback sent
    pub feedback: u16,

    /// Forward mail to user number
    pub forward_to_user: Option<u16>,

    /// File points
    pub file_points: i16,

    /// Mail waiting count
    pub mail_waiting: u8,

    /// Line length (columns)
    pub line_len: u8,

    /// Page length (rows)
    pub page_len: u8,

    /// Times on today
    pub times_on_today: u8,

    /// Illegal logon attempts
    pub illegal_logons: u8,

    /// Security level (SL)
    pub security_level: u8,

    /// Download security level (DSL)
    pub download_sl: u8,

    /// User color configuration
    pub colors: ColorArray,

    /// Last message area visited
    pub last_msg_area: u8,

    /// Last file area visited
    pub last_file_area: u8,

    /// Credit balance (dollars)
    pub credit: i32,

    /// Time bank balance (minutes)
    pub time_bank: i16,

    /// Message board SysOp privileges (up to 5 boards)
    pub board_sysop: Vec<u8>,

    /// Trap activity to log file
    pub trap_activity: bool,

    /// Trap to separate file
    pub trap_separate: bool,

    /// Time added to time bank today
    pub time_bank_add_today: i16,

    /// Auto-chat trapping
    pub chat_auto: bool,

    /// Separate chat file
    pub chat_separate: bool,

    /// User start menu (8 chars max)
    pub user_start_menu: Option<String>,

    /// Separate SysOp log
    pub sysop_log_separate: bool,

    /// Clear screen message flag (1=yes, 2=no)
    pub clear_screen_msg: u8,

    /// File list option (display style)
    pub file_list_opt: u8,

    /// Message order (0=Chrono, 1=ReplyTree)
    pub msg_order: u8,

    /// AVATAR color adjust (1=no, 2=yes)
    pub avatar_adjust: u8,
}

impl User {
    /// Convert from Pascal-compatible binary record
    pub fn from_pascal(pascal: &PascalUserRecord, id: u32) -> Result<Self> {
        // TODO: Implement conversion from Pascal record
        // Extract strings, convert dates, parse flags, etc.
        todo!("Implement Pascal->Rust conversion")
    }

    /// Convert to Pascal-compatible binary record
    pub fn to_pascal(&self) -> Result<PascalUserRecord> {
        // TODO: Implement conversion to Pascal record
        // Truncate strings to fit, pack dates, convert flags, etc.
        todo!("Implement Rust->Pascal conversion")
    }

    /// Validate user record
    pub fn validate(&self) -> Result<()> {
        // Validate username, phone, dates, etc.
        todo!("Implement validation")
    }
}
```

**Module:** `crates/impulse-types/src/user.rs` (update existing file)

**Notes:**
- **Two-layer design:** Binary-compatible layer + Modern Rust layer
- **Existing `User` struct** in user.rs needs to be reconciled with this design
- **Pascal String encoding:** Pascal `string[N]` = 1 length byte + N chars
- **Boolean encoding:** Pascal Boolean = 1 byte (0=false, 1=true)
- **Set encoding:** Pascal sets are bitsets, size depends on range
- **#[repr(C, packed)]:** Required for exact binary layout matching

---

### 2. System Configuration Record (`systatrec`, lines 435-588, 153 lines)

**Pascal Structure:** (153 lines - abbreviated here)
```pascal
systatrec = record
  gfilepath:string[79];             { GFILES path }
  afilepath:string[79];             { AFILES path (text files path) }
  menupath:string[79];              { MENUS path  }
  trappath:string[79];              { LOG path (traps, chats, SysOp logs) }
  msgpath:string[79];               { MSG path (priv/pub mail path) }
  tfilepath:string[79];             { ISL script path }
  temppath:string[79];              { TEMP path - "temp" directory }
  usenuv:boolean;                   { Is new user voting active? }
  nuvyes, nuvno:byte;               { Yes/No votes required for validation }
  bbsname:string[80];               { BBS name }
  bbsphone:string[12];              { BBS phone number }
  sysopname:string[30];             { SysOp's full name or alias }
  regnum:string[10];                { BBS Registration Number }
  // ... 140 more lines of configuration fields
end;
```

**Rust Conversion Strategy:**
- Similar two-layer design (PascalSystatRecord + BbsConfig)
- **Existing `BbsConfig`** in config.rs is already a modern design
- Create `PascalSystatRecord` for binary I/O compatibility
- Implement conversion between layers

**Module:** `crates/impulse-types/src/system_config.rs` (new)

**Note:** Detailed conversion deferred to Task 6 (153 lines is a large structure).

---

### 3. Message Board Record (`boardrec`, lines 605-631)

**Pascal Structure:**
```pascal
boardrec = record
  name:string[40];              { message base description }
  filename:string[8];           { BRD/MIX data filename }
  lastmsgid:longint;            { last message ID number }
  msgpath:string[40];           { messages pathname   }
  acs, postacs, mciacs:acstring; { access requirements }
  maxmsgs:word;                 { max message count }
  anonymous:anontyp;            { anonymous type }
  password:string[20];          { base password }
  mbstat:set of mbflags;        { message base status vars }
  permindx:longint;             { permanent index # }
  mbtype:integer;               { base type (0=Local,1=Echo,2=XMail) }
  origin:string[50];            { origin line }
  text_color, quote_color, tear_color, origin_color:byte;
  zone, net, node, point:integer; { FidoNet address }
  conf:byte;                    { Conference number }
  res:array[1..2] of byte;      { RESERVED }
end;
```

**Rust Conversion:**
```rust
/// Message board configuration record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageBoard {
    /// Message base description (40 chars max)
    pub name: String,

    /// BRD/MIX data filename (8 chars max)
    pub filename: String,

    /// Last message ID number (sequential)
    pub last_msg_id: i32,

    /// Messages pathname (40 chars max)
    pub msg_path: String,

    /// Access requirement string (20 chars max)
    pub acs: String,

    /// Post access requirement (20 chars max)
    pub post_acs: String,

    /// MCI usage requirement (20 chars max)
    pub mci_acs: String,

    /// Max message count
    pub max_msgs: u16,

    /// Anonymous type
    pub anonymous: AnonymousType,

    /// Base password (20 chars max, optional)
    pub password: Option<String>,

    /// Message base status flags
    pub status: MessageBoardFlags,

    /// Permanent index number
    pub perm_index: i32,

    /// Base type (0=Local, 1=Echo, 2=XMail)
    pub mb_type: MessageBoardType,

    /// Origin line (50 chars max)
    pub origin: String,

    /// Text color
    pub text_color: u8,

    /// Quote color
    pub quote_color: u8,

    /// Tear line color
    pub tear_color: u8,

    /// Origin line color
    pub origin_color: u8,

    /// FidoNet address (zone:net/node.point)
    pub fidonet_addr: FidoNetAddress,

    /// Conference number
    pub conference: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum MessageBoardType {
    Local = 0,
    Echo = 1,
    XMail = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FidoNetAddress {
    pub zone: i16,
    pub net: i16,
    pub node: i16,
    pub point: i16,
}
```

**Module:** `crates/impulse-types/src/message_types.rs`

---

### 4. File Area Record (`ulrec`, lines 641-659)

**Pascal Structure:**
```pascal
ulrec = record
  name:string[40];              { area description  }
  filename:string[12];          { filename + ".DIR" }
  dlpath, ulpath:string[40];    { download/upload paths }
  maxfiles:integer;             { max files allowed }
  password:string[20];          { password required }
  arctype, cmttype:byte;        { archive/comment types }
  fbdepth:integer;              { file base dir depth }
  fbstat:set of fbflags;        { file base status vars }
  acs, ulacs, nameacs:acstring; { access requirements }
  permindx:longint;             { permanent index # }
  Conf:byte;                    { fbase conference }
  res:array[1..5] of byte;      { RESERVED }
end;
```

**Rust Conversion:**
```rust
/// File area configuration record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileArea {
    /// Area description (40 chars max)
    pub name: String,

    /// Filename + ".DIR" (12 chars max)
    pub filename: String,

    /// Download path (40 chars max)
    pub download_path: String,

    /// Upload path (40 chars max)
    pub upload_path: String,

    /// Max files allowed
    pub max_files: i16,

    /// Password required (20 chars max, optional)
    pub password: Option<String>,

    /// Wanted archive type (1..max, 0=inactive)
    pub arc_type: u8,

    /// Wanted comment type (1..3, 0=inactive)
    pub comment_type: u8,

    /// File base directory depth
    pub depth: i16,

    /// File base status flags
    pub status: FileBoardFlags,

    /// Access requirements (20 chars max)
    pub acs: String,

    /// Upload requirements (20 chars max)
    pub upload_acs: String,

    /// See-names requirements (20 chars max)
    pub name_acs: String,

    /// Permanent index number
    pub perm_index: i32,

    /// File base conference
    pub conference: u8,
}
```

**Module:** `crates/impulse-types/src/file_types.rs`

---

## Tier 2: HIGH Priority Records

### 5. Message Index Record (`msgindexrec`, lines 245-256)

**Rust Conversion:**
```rust
/// Message index record (stored in *.MIX files)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageIndex {
    /// Message number (tonum in EMAIL.MIX for email)
    pub message_num: u16,

    /// Pointer to message header in *.BRD file
    pub header_ptr: i32,

    /// Message ID (sequential, unique across all bases)
    pub msg_id: i32,

    /// ID of message this is a reply to (-1 if none)
    pub is_reply_to_id: i32,

    /// Message date/time (packed format)
    pub msg_date: PackedDateTime,

    /// Message day-of-week (0=Sunday, 6=Saturday)
    pub msg_day_of_week: u8,

    /// Message status flags
    pub status: MessageIndexStatus,

    /// Reply this message is to (-1 if none)
    pub is_reply_to: u16,

    /// Number of replies to THIS message
    pub num_replies: u16,
}
```

**Module:** `crates/impulse-types/src/message_types.rs`

---

### 6. Message Header Record (`mheaderrec`, lines 267-276)

**Rust Conversion:**
```rust
/// From/To information for message headers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FromToInfo {
    /// Anonymous type (0=not anon, 1=anon, etc.)
    pub anon: u8,

    /// User number
    pub user_num: u16,

    /// Given name for this case (42 chars max)
    pub as_name: String,

    /// User real name (36 chars max)
    pub real_name: String,

    /// User alias (36 chars max)
    pub alias: String,
}

/// Message header record (stored in *.BRD files)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageHeader {
    /// Header signature (0xFFFFFFFF = -1 as i32)
    pub signature: i32,

    /// Pointer to message text in *.BRD file
    pub msg_ptr: i32,

    /// Length of message text
    pub msg_length: i32,

    /// From information
    pub from: FromToInfo,

    /// To information
    pub to: FromToInfo,

    /// Message title (60 chars max)
    pub title: String,

    /// Echo/Group original message date (19 chars max)
    pub origin_date: String,
}
```

**Module:** `crates/impulse-types/src/message_types.rs`

---

### 7. File Record (`ulfrec`, lines 666-683)

**Rust Conversion:**
```rust
/// File record (stored in *.DIR files)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRecord {
    /// Filename (12 chars max)
    pub filename: String,

    /// File description (60 chars max)
    pub description: String,

    /// File points required
    pub file_points: i16,

    /// Number of downloads
    pub download_count: i16,

    /// File type (useless?)
    pub file_type: u8,

    /// Size in 128-byte blocks
    pub blocks: i16,

    /// Uploader user number
    pub owner: i16,

    /// Uploader's name (36 chars max)
    pub owner_name: String,

    /// Date uploaded (8 chars max, MM/DD/YY format)
    pub date: String,

    /// Numeric date uploaded (Julian date)
    pub date_num: i16,

    /// Pointer to verbose description (-1 if none)
    pub verbose_ptr: i32,

    /// File status flags
    pub status: FileStatus,

    /// Private file flag
    pub private: bool,

    /// Private for (short user identifier, 8 chars max)
    pub private_for: Option<String>,
}
```

**Module:** `crates/impulse-types/src/file_types.rs`

---

### 8. NewScan Record (`zscanrec`, lines 228-233)

**Rust Conversion:**
```rust
/// NewScan record (ZSCAN.DAT, per-user newscan data)
///
/// Tracks which message boards and file areas the user has scanned,
/// and the high-read pointers for each message board.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewScanRecord {
    /// NewScan high message read pointers (one per board)
    pub msg_high_read: MessageHighReadArray,

    /// NewScan message bases (bitset of which boards to scan)
    pub msg_scan: MessageScanSet,

    /// NewScan file bases (bitset of which areas to scan)
    pub file_scan: FileScanSet,
}
```

**Module:** `crates/impulse-types/src/scan_types.rs`

---

### 9. Event Record (`eventrec`, lines 698-710)

**Rust Conversion:**
```rust
/// Event scheduler record (EVENTS.DAT)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Whether active
    pub active: bool,

    /// Event description for logs (30 chars max)
    pub description: String,

    /// Event type: 'A'=CS, 'C'=hat, 'D'=os call, 'E'=xternal
    pub event_type: char,

    /// Error level if "E", command line if "D" (20 chars max)
    pub exec_data: String,

    /// Off-hook time before event (0 if none)
    pub busy_time: i16,

    /// Time of execution (minutes since midnight)
    pub exec_time: i16,

    /// Busy phone DURING event?
    pub busy_during: bool,

    /// Length of time event takes (minutes)
    pub duration: i16,

    /// Bitwise execution days OR day of month if monthly
    pub exec_days: u8,

    /// Monthly event flag
    pub monthly: bool,
}
```

**Module:** `crates/impulse-types/src/event_types.rs` (new)

---

## Module Organization

### Target Module Structure

```
crates/impulse-types/src/
├── lib.rs                     # Module declarations, re-exports
├── error.rs                   # Error types (existing)
├── constants.rs               # System constants (NEW)
├── pascal_compat.rs           # Pascal type aliases (NEW)
├── datetime_types.rs          # PackedDateTime, conversions (NEW)
├── user.rs                    # User types (UPDATE existing)
├── user_flags.rs              # UserFlags bitflags (NEW)
├── security_types.rs          # SecurityRange, security levels (NEW)
├── terminal_types.rs          # ColorArray, terminal settings (NEW)
├── message_types.rs           # Message, board, index types (NEW)
├── file_types.rs              # File area, file record types (NEW)
├── file_flags.rs              # File-related flags (NEW)
├── scan_types.rs              # NewScan, scan sets (NEW)
├── menu_types.rs              # Menu, command types (NEW)
├── protocol_types.rs          # Protocol configuration (NEW)
├── event_types.rs             # Event scheduler types (NEW)
├── network_types.rs           # FidoNet, node types (NEW)
├── system_config.rs           # System configuration (NEW)
├── strings.rs                 # UI strings (NEW, large)
└── legacy/                    # Pascal-compatible binary structures
    ├── mod.rs                 # Re-exports
    ├── user_record.rs         # PascalUserRecord
    ├── systat_record.rs       # PascalSystatRecord
    ├── board_record.rs        # PascalBoardRecord
    └── ...
```

---

## Binary Compatibility Strategy

### Two-Layer Architecture

**Layer 1: Pascal-Compatible Binary Structures**
- Located in `src/legacy/` submodule
- Use `#[repr(C, packed)]` for exact memory layout
- Use fixed-size arrays `[u8; N]` for Pascal strings
- Use byte arrays for Pascal sets
- Minimal validation, raw access
- **Purpose:** Read/write existing Pascal data files (USER.LST, BOARDS.DAT, etc.)

**Layer 2: Modern Rust Types**
- Located in main `src/` modules
- Use `String`, `chrono::DateTime`, `HashSet`, etc.
- Comprehensive validation and error handling
- Convenient methods and trait implementations
- **Purpose:** Application logic and new functionality

**Conversion Layer:**
- `impl From<PascalXxxRecord> for Xxx` - Pascal → Rust
- `impl TryFrom<&Xxx> for PascalXxxRecord` - Rust → Pascal (fallible)
- Handles string truncation, date conversion, set mapping, etc.

### Example Pattern

```rust
// Layer 1: Binary-compatible (legacy/user_record.rs)
#[repr(C, packed)]
pub struct PascalUserRecord {
    pub name: [u8; 37],  // String[36]
    pub realname: [u8; 37],
    pub pw: [u8; 21],
    // ... all fields with exact Pascal layout
}

// Layer 2: Modern Rust (user.rs)
pub struct User {
    pub id: u32,
    pub name: String,
    pub real_name: String,
    pub password_hash: String,
    // ... modern types
}

// Conversion (user.rs)
impl From<PascalUserRecord> for User {
    fn from(p: PascalUserRecord) -> Self {
        User {
            id: 0, // Assigned when reading from file
            name: pascal_string_to_rust(&p.name),
            real_name: pascal_string_to_rust(&p.realname),
            password_hash: pascal_string_to_rust(&p.pw),
            // ... convert all fields
        }
    }
}

impl TryFrom<&User> for PascalUserRecord {
    type Error = Error;

    fn try_from(u: &User) -> Result<Self> {
        Ok(PascalUserRecord {
            name: rust_string_to_pascal(&u.name, 36)?,
            realname: rust_string_to_pascal(&u.real_name, 36)?,
            pw: rust_string_to_pascal(&u.password_hash, 20)?,
            // ... convert all fields, truncating where needed
        })
    }
}
```

### Helper Functions for String Conversion

```rust
// src/pascal_compat.rs

/// Convert Pascal string (length byte + data) to Rust String
///
/// Pascal strings: byte 0 = length, bytes 1..N = data
pub fn pascal_string_to_rust(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return String::new();
    }
    let len = bytes[0] as usize;
    let data = &bytes[1..=len.min(bytes.len() - 1)];
    String::from_utf8_lossy(data).into_owned()
}

/// Convert Rust String to Pascal string format
///
/// Returns fixed-size array [length_byte, data...], truncating if needed
pub fn rust_string_to_pascal<const N: usize>(s: &str, max_len: usize) -> Result<[u8; N]> {
    if N < max_len + 1 {
        return Err(Error::Validation("Buffer too small for Pascal string".into()));
    }

    let bytes = s.as_bytes();
    let len = bytes.len().min(max_len);

    let mut result = [0u8; N];
    result[0] = len as u8;
    result[1..=len].copy_from_slice(&bytes[..len]);

    Ok(result)
}

/// Convert Pascal boolean (0/1 byte) to Rust bool
pub fn pascal_bool_to_rust(byte: u8) -> bool {
    byte != 0
}

/// Convert Rust bool to Pascal boolean byte
pub fn rust_bool_to_pascal(b: bool) -> u8 {
    if b { 1 } else { 0 }
}

/// Convert Pascal date string (MM/DD/YY) to chrono::NaiveDate
pub fn pascal_date_to_chrono(date_str: &str) -> Result<chrono::NaiveDate> {
    // Parse MM/DD/YY format, handle Y2K (90-99 = 1990-1999, 00-89 = 2000-2089)
    let parts: Vec<&str> = date_str.split('/').collect();
    if parts.len() != 3 {
        return Err(Error::Validation("Invalid Pascal date format".into()));
    }

    let month: u32 = parts[0].parse().map_err(|_| Error::Validation("Invalid month".into()))?;
    let day: u32 = parts[1].parse().map_err(|_| Error::Validation("Invalid day".into()))?;
    let year: i32 = parts[2].parse().map_err(|_| Error::Validation("Invalid year".into()))?;

    // Y2K handling
    let full_year = if year >= 90 { 1900 + year } else { 2000 + year };

    chrono::NaiveDate::from_ymd_opt(full_year, month, day)
        .ok_or_else(|| Error::Validation("Invalid date".into()))
}

/// Convert chrono::NaiveDate to Pascal date string (MM/DD/YY)
pub fn chrono_date_to_pascal(date: chrono::NaiveDate) -> String {
    format!("{:02}/{:02}/{:02}", date.month(), date.day(), date.year() % 100)
}
```

---

## Dependencies Between Types

### Internal Dependencies (within RECORDS.PAS)

**Zero Dependencies (can be implemented first):**
- Constants (`MAX_BOARDS`, etc.)
- Type aliases (`astr`, `str80`, `acstring`, `acrq`)
- Simple enums (`AnonymousType`, `MessageBoardType`, etc.)
- `PackedDateTime`
- `ColorArray`
- `SecurityRange`

**Low Dependencies (require simple types):**
- `UserFlags`, `FileListFlags`, etc. (bitflags enums)
- `MessageScanSet`, `FileScanSet` (bitsets)
- `FidoNetAddress` (simple struct)

**Medium Dependencies (require low-dependency types):**
- `FromToInfo` (requires no deps)
- `MessageBoard` (requires `AnonymousType`, `MessageBoardFlags`, `FidoNetAddress`)
- `FileArea` (requires `FileBoardFlags`)
- `FileRecord` (requires `FileStatus`)

**High Dependencies (require medium types):**
- `MessageIndex` (requires `PackedDateTime`, `MessageIndexStatus`)
- `MessageHeader` (requires `FromToInfo`)
- `User` (requires `UserFlags`, `FileListFlags`, `ColorArray`, many others)
- `NewScanRecord` (requires `MessageScanSet`, `FileScanSet`, `MessageHighReadArray`)

### Conversion Order (by dependency)

**Phase A: Foundation (no dependencies)**
1. Constants
2. Type aliases
3. Simple enums (10 enums)
4. `PackedDateTime`
5. `ColorArray`, `SecurityRange`
6. Bitflags enums (10 bitflag types)

**Phase B: Basic Structures**
7. `FidoNetAddress`
8. Scan sets (`MessageScanSet`, `FileScanSet`)
9. `FromToInfo`

**Phase C: Core Records**
10. `MessageBoard`
11. `FileArea`
12. `FileRecord`
13. `MessageIndex`
14. `MessageHeader`

**Phase D: Complex Records**
15. `NewScanRecord`
16. `User` (most complex)
17. `BbsConfig` / `systatrec` (largest)

**Phase E: Auxiliary Records**
18-35. Supporting records (events, menus, protocols, etc.)

---

## Testing Strategy

### Unit Tests (per type)

**For Each Type:**
1. **Construction Tests**
   - Valid construction
   - Invalid construction (validation)
   - Default values

2. **Serialization Tests**
   - JSON round-trip
   - Bincode round-trip
   - Field preservation

3. **Binary Compatibility Tests**
   - Pascal → Rust conversion
   - Rust → Pascal conversion
   - Round-trip conversion
   - String truncation handling
   - Date conversion edge cases

4. **Validation Tests**
   - Field constraints
   - Cross-field validation
   - Error messages

### Integration Tests

**Binary File I/O:**
1. Read existing Pascal data files (if available)
2. Verify all fields parsed correctly
3. Write data back to binary format
4. Verify byte-for-byte match (or acceptable differences)

**End-to-End:**
1. Load Pascal user file → Convert to Rust → Save to new file → Load new file → Compare
2. Same for boards, file areas, messages, etc.

### Test Data

**Create Test Fixtures:**
- Sample `userrec` with known values
- Sample `boardrec` for each anonymous type
- Edge cases: max-length strings, boundary dates, flag combinations

**Pascal Test Program:**
- Write small Pascal program to generate binary test files
- Ensure test files have known, predictable values
- Use for verification testing

---

## Documentation Requirements

### For Each Type:

1. **Module-level documentation**
   - Overview of types in module
   - Pascal source reference (file, line numbers)
   - Binary compatibility notes
   - Usage examples

2. **Type-level documentation**
   - Purpose and usage
   - Pascal equivalent (with line reference)
   - Field descriptions
   - Binary layout notes
   - Example construction

3. **Method documentation**
   - All public methods documented
   - Parameters explained
   - Return values described
   - Errors listed
   - Examples provided

4. **Cross-references**
   - Link related types
   - Reference conversion functions
   - Link to Pascal source analysis docs

### Documentation Files

**Create:**
1. `docs/pascal-analysis/records-pas-field-guide.md` - Field-by-field reference
2. `docs/implementation/binary-file-format.md` - Binary file format specs
3. `docs/implementation/type-conversion-guide.md` - How to convert Pascal types
4. `crates/impulse-types/README.md` - Crate overview and usage guide

---

## Implementation Phases

### Phase 1: Foundation (Tasks 2-4)
**Duration:** 2 days
**Deliverables:**
- Constants module
- Type aliases module
- 10 enum types
- `PackedDateTime` type
- Helper functions for string/date conversion
- Unit tests (50+ tests)

### Phase 2: Core Enums and Flags (Task 4)
**Duration:** 1 day
**Deliverables:**
- 10 bitflags types (UserFlags, MessageBoardFlags, etc.)
- Serialization tests
- Binary compatibility tests

### Phase 3: User Record (Task 5)
**Duration:** 3 days
**Deliverables:**
- `PascalUserRecord` (binary-compatible)
- `User` (modern Rust)
- Conversion layer
- Reconciliation with existing `user.rs`
- Comprehensive tests (80+ tests)
- Pascal data file I/O verification

### Phase 4: System Configuration (Task 6)
**Duration:** 3 days
**Deliverables:**
- `PascalSystatRecord` (binary-compatible)
- Update existing `BbsConfig`
- Conversion layer
- Tests

### Phase 5: Message Types (Task 7)
**Duration:** 2 days
**Deliverables:**
- `MessageBoard`, `MessageIndex`, `MessageHeader`, `FromToInfo`
- Binary-compatible layer
- Conversion layer
- Tests

### Phase 6: File Types (Task 8)
**Duration:** 2 days
**Deliverables:**
- `FileArea`, `FileRecord`
- Binary-compatible layer
- Conversion layer
- Tests

### Phase 7: Auxiliary Types (Tasks 9-12)
**Duration:** 3 days
**Deliverables:**
- NewScan, Event, Menu, Command, Protocol types
- Network types (FidoNet, node records)
- Tests

### Phase 8: Integration & Testing (Tasks 13-17)
**Duration:** 3 days
**Deliverables:**
- Bincode serialization for all types
- Binary compatibility verification
- Pascal data file round-trip tests
- 80%+ code coverage
- Performance benchmarks

### Phase 9: Documentation (Tasks 18-19)
**Duration:** 2 days
**Deliverables:**
- Comprehensive rustdoc (100% public API)
- Pascal conversion reference document
- Binary file format specification
- Migration guide

### Phase 10: Quality & Completion (Tasks 20-22)
**Duration:** 1 day
**Deliverables:**
- README.md and CHANGELOG.md updates
- Full quality checks (fmt, clippy, test, build)
- Commit Sprint 5 completion
- Sprint completion report

**Total Duration:** 22 days (Sprint 5 scope)

---

## Risk Assessment

### High Risks

**1. Binary Compatibility Complexity**
- **Risk:** Pascal's exact binary layout may be compiler-dependent
- **Mitigation:** Create reference Pascal program to generate test data, verify with hex dumps
- **Severity:** HIGH
- **Probability:** MEDIUM

**2. String Encoding Issues**
- **Risk:** Pascal strings may use non-UTF8 encodings (CP437, etc.)
- **Mitigation:** Use `String::from_utf8_lossy()` for reading, document encoding
- **Severity:** MEDIUM
- **Probability:** HIGH

**3. Set Type Binary Layout**
- **Risk:** Pascal `set of` binary layout may vary by range
- **Mitigation:** Test with known Pascal data files, document bit ordering
- **Severity:** MEDIUM
- **Probability:** MEDIUM

**4. Date Conversion Edge Cases**
- **Risk:** Y2K handling, leap years, invalid dates in old data
- **Mitigation:** Comprehensive date validation and error handling
- **Severity:** LOW
- **Probability:** HIGH

### Medium Risks

**5. Reconciliation with Existing Types**
- **Risk:** Existing `User`, `BbsConfig` types may conflict with Pascal design
- **Mitigation:** Two-layer architecture allows coexistence
- **Severity:** MEDIUM
- **Probability:** LOW

**6. Performance of Conversion Layer**
- **Risk:** Frequent conversions between layers may impact performance
- **Mitigation:** Minimize conversions, benchmark critical paths
- **Severity:** LOW
- **Probability:** LOW

### Low Risks

**7. Test Data Availability**
- **Risk:** May not have access to original Impulse 7.1 data files
- **Mitigation:** Generate synthetic test data with Pascal program
- **Severity:** LOW
- **Probability:** MEDIUM

---

## Quality Metrics

### Code Coverage Target: 80%+

**Per Module:**
- Constants: N/A (no logic to test)
- Type aliases: N/A
- Enums: 90%+ (construction, validation, serialization)
- User types: 85%+ (complex validation and conversion)
- Message types: 80%+
- File types: 80%+
- Auxiliary types: 75%+

### Test Counts (Estimated)

**Total Tests:** 400+

**Breakdown:**
- Enum construction/validation: 80 tests (10 enums × 8 tests each)
- Bitflags operations: 50 tests
- PackedDateTime: 20 tests
- User type: 100 tests (validation, conversion, binary I/O)
- System config: 60 tests
- Message types: 50 tests
- File types: 40 tests
- Auxiliary types: 60 tests
- Integration tests: 40 tests

### Performance Benchmarks

**Measure:**
1. Pascal → Rust conversion time (per record)
2. Rust → Pascal conversion time (per record)
3. Binary file read time (1000 records)
4. Binary file write time (1000 records)
5. Memory footprint (Pascal struct vs Rust struct)

**Targets:**
- Conversion: < 1µs per record
- File I/O: < 1ms per 1000 records
- Memory: < 2x Pascal size (acceptable for modern types)

---

## Acceptance Criteria

### Sprint 5 Completion Checklist

**Code:**
- [ ] All 40+ types converted from Pascal to Rust
- [ ] Binary-compatible layer implemented for all file-based types
- [ ] Modern Rust layer with proper types (String, DateTime, etc.)
- [ ] Conversion functions (Pascal ↔ Rust) for all types
- [ ] All types support Serde JSON serialization
- [ ] All types support bincode binary serialization

**Testing:**
- [ ] 400+ unit tests passing
- [ ] 80%+ code coverage achieved
- [ ] Binary compatibility verified with test files
- [ ] Round-trip conversion tests passing
- [ ] Serialization tests (JSON + bincode) passing
- [ ] Edge case handling tested (truncation, dates, etc.)

**Documentation:**
- [ ] 100% rustdoc coverage on public APIs
- [ ] Module-level documentation complete
- [ ] Pascal conversion reference document created
- [ ] Binary file format specification written
- [ ] Usage examples in documentation
- [ ] README.md updated with Sprint 5 completion

**Quality:**
- [ ] 0 clippy warnings
- [ ] `cargo fmt` passing
- [ ] `cargo build --workspace` succeeds
- [ ] `cargo test --workspace` passes (all 482+ tests)
- [ ] No regressions in existing code

**Integration:**
- [ ] Reconciliation with existing types (User, BbsConfig)
- [ ] CHANGELOG.md updated
- [ ] Git commit created with proper message
- [ ] Sprint completion report written

---

## Next Steps After This Document

**Immediate Actions:**
1. Review and validate this conversion plan
2. Create GitHub issues for each implementation phase (if desired)
3. Begin Phase 1: Foundation (constants, enums, PackedDateTime)
4. Set up test infrastructure for binary compatibility testing
5. Create Pascal test program to generate reference data files

**First Implementation:**
- Start with `constants.rs` (simplest, no dependencies)
- Then `pascal_compat.rs` (helper functions)
- Then simple enums (AnonymousType, etc.)
- Then `PackedDateTime` (critical for many types)
- Build up to complex types (User, BbsConfig)

**Continuous:**
- Write tests alongside implementation (TDD approach)
- Document as you go (don't defer rustdoc)
- Verify binary compatibility early and often
- Keep this plan updated as issues are discovered

---

**Document Status:** DRAFT - Ready for Review
**Author:** Claude Code (AI Assistant)
**Date:** 2025-11-23
**Sprint:** 5 - Core Types Implementation
**Next Task:** Begin Phase 1 implementation (constants.rs, enums)
