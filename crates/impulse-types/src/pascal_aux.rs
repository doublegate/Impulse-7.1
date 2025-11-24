//! Pascal-compatible auxiliary record types
//!
//! This module contains auxiliary data structures from the original Impulse 7.1 BBS:
//! - `SmalRec` - Sorted names listing (NAMES.LST)
//! - `ZScanRec` - NewScan records for message/file tracking (ZSCAN.DAT)
//! - `ZLogRec` - System usage log (ZLOG.DAT)
//! - `ModemRec` - Modem configuration settings
//!
//! All types maintain binary compatibility with Pascal records for data migration.

use binrw::binrw;
use serde::{Deserialize, Serialize};

use crate::pascal_user::PascalString;

/// Maximum number of message boards (1-254)
pub const MAX_BOARDS: usize = 254;

/// Maximum number of file boards (0-254)
pub const MAX_UBOARDS: usize = 254;

/// Packed date/time (Pascal: `cpackdatetime = array[1..6] of byte`)
///
/// Original Pascal definition (RECORDS.PAS line 74):
/// ```pascal
/// cpackdatetime=array[1..6] of byte;
/// ```
///
/// Format: [year-1900, month, day, hour, minute, second]
#[binrw]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PackedDateTime {
    /// Date/time bytes: [year-1900, month, day, hour, minute, second]
    pub data: [u8; 6],
}

impl PackedDateTime {
    /// Create new packed date/time
    pub fn new(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> Self {
        Self {
            data: [
                (year.saturating_sub(1900)) as u8,
                month,
                day,
                hour,
                minute,
                second,
            ],
        }
    }

    /// Get year (1900-2155)
    pub fn year(&self) -> u16 {
        1900 + self.data[0] as u16
    }

    /// Get month (1-12)
    pub fn month(&self) -> u8 {
        self.data[1]
    }

    /// Get day (1-31)
    pub fn day(&self) -> u8 {
        self.data[2]
    }

    /// Get hour (0-23)
    pub fn hour(&self) -> u8 {
        self.data[3]
    }

    /// Get minute (0-59)
    pub fn minute(&self) -> u8 {
        self.data[4]
    }

    /// Get second (0-59)
    pub fn second(&self) -> u8 {
        self.data[5]
    }

    /// Check if date/time is valid
    pub fn is_valid(&self) -> bool {
        self.month() >= 1
            && self.month() <= 12
            && self.day() >= 1
            && self.day() <= 31
            && self.hour() <= 23
            && self.minute() <= 59
            && self.second() <= 59
    }
}

/// Sorted names listing record (Pascal: `smalrec`)
///
/// Original Pascal definition (RECORDS.PAS lines 136-141):
/// ```pascal
/// smalrec=                          { NAMES.LST : Sorted names listing }
/// record
///   name   :string[36];             { user name }
///   deleted:boolean;                { user is deleted }
///   number :integer;                { user number }
/// end;
/// ```
///
/// Used in NAMES.LST for quick alphabetical user lookups.
#[binrw]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SmalRec {
    /// User name (36 chars max)
    pub name: PascalString<36>,

    /// Whether user account is deleted
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |b: &bool| if *b { 1u8 } else { 0u8 })]
    pub deleted: bool,

    /// User number (index into USER.LST)
    pub number: i16,
}

impl SmalRec {
    /// Check if this is a valid (non-deleted) user entry
    pub fn is_valid(&self) -> bool {
        !self.deleted && self.number > 0
    }

    /// Get the user name
    pub fn user_name(&self) -> String {
        self.name.to_string()
    }
}

/// NewScan records (Pascal: `zscanrec`)
///
/// Original Pascal definition (RECORDS.PAS lines 228-233):
/// ```pascal
/// zscanrec=                       { ZSCAN.DAT : NewScan recs (file/msg) }
/// record                          { ** b0..b3 }
///   mhiread:mhireadr;             { NewScan high message pointers }
///   mzscan:mzscanr;               { NewScan message bases }
///   fzscan:fzscanr;               { NewScan file bases }
/// end;
/// ```
///
/// Tracks which message/file bases the user wants to scan for new items.
/// mhireadr = array[1..254] of cpackdatetime (1524 bytes)
/// mzscanr = set of 1..254 (32 bytes bitset)
/// fzscanr = set of 0..254 (32 bytes bitset)
#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZScanRec {
    /// High message read pointers per board (254 * 6 bytes = 1524 bytes)
    #[br(count = MAX_BOARDS)]
    #[bw(args_raw = ())]
    pub mhiread: Vec<PackedDateTime>,

    /// Message base scan flags (32 bytes for 254 boards)
    pub mzscan: [u8; 32],

    /// File base scan flags (32 bytes for 254 boards)
    pub fzscan: [u8; 32],
}

impl Default for ZScanRec {
    fn default() -> Self {
        Self {
            mhiread: vec![PackedDateTime::default(); MAX_BOARDS],
            mzscan: [0u8; 32],
            fzscan: [0u8; 32],
        }
    }
}

impl ZScanRec {
    /// Check if message board is enabled for scanning
    pub fn is_msg_board_enabled(&self, board: usize) -> bool {
        if board == 0 || board > MAX_BOARDS {
            return false;
        }
        let byte_idx = (board - 1) / 8;
        let bit_idx = (board - 1) % 8;
        (self.mzscan[byte_idx] & (1 << bit_idx)) != 0
    }

    /// Enable message board for scanning
    pub fn enable_msg_board(&mut self, board: usize) {
        if board > 0 && board <= MAX_BOARDS {
            let byte_idx = (board - 1) / 8;
            let bit_idx = (board - 1) % 8;
            self.mzscan[byte_idx] |= 1 << bit_idx;
        }
    }

    /// Disable message board from scanning
    pub fn disable_msg_board(&mut self, board: usize) {
        if board > 0 && board <= MAX_BOARDS {
            let byte_idx = (board - 1) / 8;
            let bit_idx = (board - 1) % 8;
            self.mzscan[byte_idx] &= !(1 << bit_idx);
        }
    }

    /// Check if file board is enabled for scanning
    pub fn is_file_board_enabled(&self, board: usize) -> bool {
        if board > MAX_UBOARDS {
            return false;
        }
        let byte_idx = board / 8;
        let bit_idx = board % 8;
        (self.fzscan[byte_idx] & (1 << bit_idx)) != 0
    }

    /// Enable file board for scanning
    pub fn enable_file_board(&mut self, board: usize) {
        if board <= MAX_UBOARDS {
            let byte_idx = board / 8;
            let bit_idx = board % 8;
            self.fzscan[byte_idx] |= 1 << bit_idx;
        }
    }

    /// Disable file board from scanning
    pub fn disable_file_board(&mut self, board: usize) {
        if board <= MAX_UBOARDS {
            let byte_idx = board / 8;
            let bit_idx = board % 8;
            self.fzscan[byte_idx] &= !(1 << bit_idx);
        }
    }

    /// Get high read pointer for message board
    pub fn get_msg_high_read(&self, board: usize) -> Option<&PackedDateTime> {
        if board > 0 && board <= MAX_BOARDS {
            Some(&self.mhiread[board - 1])
        } else {
            None
        }
    }

    /// Set high read pointer for message board
    pub fn set_msg_high_read(&mut self, board: usize, datetime: PackedDateTime) {
        if board > 0 && board <= MAX_BOARDS {
            self.mhiread[board - 1] = datetime;
        }
    }
}

/// System usage log record (Pascal: `zlogrec`)
///
/// Original Pascal definition (RECORDS.PAS lines 278-285):
/// ```pascal
/// zlogrec=                        { ZLOG.DAT : System log }
/// record
///   date:string[8];
///   userbaud:array[0..4] of integer;
///   active,calls,newusers,pubpost,privpost,fback,criterr:integer;
///   uploads,downloads:integer;
///   uk,dk:longint;
/// end;
/// ```
///
/// Daily system usage statistics stored in ZLOG.DAT.
#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZLogRec {
    /// Date string (MM/DD/YY format, 8 chars)
    pub date: PascalString<8>,

    /// User baud rate counts [0]=300, [1]=1200, [2]=2400, [3]=9600, [4]=other
    pub userbaud: [i16; 5],

    /// Minutes system was active
    pub active: i16,

    /// Number of calls received
    pub calls: i16,

    /// Number of new users registered
    pub newusers: i16,

    /// Number of public posts
    pub pubpost: i16,

    /// Number of private posts
    pub privpost: i16,

    /// Number of feedback messages
    pub fback: i16,

    /// Number of critical errors
    pub criterr: i16,

    /// Number of file uploads
    pub uploads: i16,

    /// Number of file downloads
    pub downloads: i16,

    /// Kilobytes uploaded
    pub uk: i32,

    /// Kilobytes downloaded
    pub dk: i32,
}

impl ZLogRec {
    /// Get the log date string
    pub fn log_date(&self) -> String {
        self.date.to_string()
    }

    /// Get total messages posted (public + private + feedback)
    pub fn total_messages(&self) -> i16 {
        self.pubpost
            .saturating_add(self.privpost)
            .saturating_add(self.fback)
    }

    /// Get total file transfers (uploads + downloads)
    pub fn total_transfers(&self) -> i16 {
        self.uploads.saturating_add(self.downloads)
    }

    /// Get total kilobytes transferred (uploaded + downloaded)
    pub fn total_kb(&self) -> i32 {
        self.uk.saturating_add(self.dk)
    }

    /// Get baud rate distribution percentages
    pub fn baud_distribution(&self) -> [f32; 5] {
        let total: i16 = self.userbaud.iter().sum();
        if total == 0 {
            return [0.0; 5];
        }
        [
            (self.userbaud[0] as f32 / total as f32) * 100.0,
            (self.userbaud[1] as f32 / total as f32) * 100.0,
            (self.userbaud[2] as f32 / total as f32) * 100.0,
            (self.userbaud[3] as f32 / total as f32) * 100.0,
            (self.userbaud[4] as f32 / total as f32) * 100.0,
        ]
    }
}

/// Modem configuration record (Pascal: `modemrec`)
///
/// Original Pascal definition (RECORDS.PAS lines 287-305):
/// ```pascal
/// modemrec=
/// record
///   waitbaud:longint;            { wait baud }
///   comport:byte;                { comport number }
///   init:string[80];             { initialization string }
///   answer:string[40];           { answer string }
///   hangup:string[40];           { hangup string }
///   offhook:string[40];          { phone off-hook string }
///   nocallinittime:byte;         { reinit modem after x mins of inactivity }
///   hardware:boolean;            { use hardware handshaking }
/// end;
/// ```
///
/// Modem/serial port configuration settings.
#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModemRec {
    /// Baud rate to wait at (e.g., 2400, 9600, 19200)
    pub waitbaud: i32,

    /// COM port number (1-4)
    pub comport: u8,

    /// Modem initialization string (80 chars max)
    pub init: PascalString<80>,

    /// Modem answer string (40 chars max)
    pub answer: PascalString<40>,

    /// Modem hangup string (40 chars max)
    pub hangup: PascalString<40>,

    /// Phone off-hook string (40 chars max)
    pub offhook: PascalString<40>,

    /// Minutes of inactivity before reinitializing modem
    pub nocallinittime: u8,

    /// Use hardware handshaking (RTS/CTS)
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |b: &bool| if *b { 1u8 } else { 0u8 })]
    pub hardware: bool,
}

impl Default for ModemRec {
    fn default() -> Self {
        Self {
            waitbaud: 2400,
            comport: 1,
            init: PascalString::default(),
            answer: PascalString::default(),
            hangup: PascalString::default(),
            offhook: PascalString::default(),
            nocallinittime: 60,
            hardware: true,
        }
    }
}

impl ModemRec {
    /// Check if COM port number is valid (1-4)
    pub fn is_valid_port(&self) -> bool {
        self.comport >= 1 && self.comport <= 4
    }

    /// Check if baud rate is valid (common rates)
    pub fn is_valid_baud(&self) -> bool {
        matches!(
            self.waitbaud,
            300 | 1200 | 2400 | 4800 | 9600 | 14400 | 19200 | 28800 | 38400 | 57600 | 115200
        )
    }

    /// Get modem init string
    pub fn init_string(&self) -> String {
        self.init.to_string()
    }

    /// Get modem answer string
    pub fn answer_string(&self) -> String {
        self.answer.to_string()
    }

    /// Get modem hangup string
    pub fn hangup_string(&self) -> String {
        self.hangup.to_string()
    }

    /// Get phone off-hook string
    pub fn offhook_string(&self) -> String {
        self.offhook.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // PackedDateTime tests
    #[test]
    fn test_packed_datetime_new() {
        let dt = PackedDateTime::new(2025, 11, 23, 14, 30, 45);
        assert_eq!(dt.year(), 2025);
        assert_eq!(dt.month(), 11);
        assert_eq!(dt.day(), 23);
        assert_eq!(dt.hour(), 14);
        assert_eq!(dt.minute(), 30);
        assert_eq!(dt.second(), 45);
    }

    #[test]
    fn test_packed_datetime_valid() {
        let dt = PackedDateTime::new(2025, 11, 23, 14, 30, 45);
        assert!(dt.is_valid());

        let invalid = PackedDateTime::new(2025, 13, 32, 25, 60, 60);
        assert!(!invalid.is_valid());
    }

    #[test]
    fn test_packed_datetime_default() {
        let dt = PackedDateTime::default();
        assert_eq!(dt.year(), 1900);
        assert_eq!(dt.month(), 0);
        assert!(!dt.is_valid());
    }

    // SmalRec tests
    #[test]
    fn test_smalrec_default() {
        let rec = SmalRec::default();
        assert!(rec.name.is_empty());
        assert!(!rec.deleted);
        assert_eq!(rec.number, 0);
        assert!(!rec.is_valid());
    }

    #[test]
    fn test_smalrec_valid() {
        let mut rec = SmalRec::default();
        rec.name = PascalString::from_string("JOHN DOE");
        rec.number = 100;
        assert!(rec.is_valid());
        assert_eq!(rec.user_name(), "JOHN DOE");
    }

    #[test]
    fn test_smalrec_deleted() {
        let mut rec = SmalRec::default();
        rec.name = PascalString::from_string("JOHN DOE");
        rec.number = 100;
        rec.deleted = true;
        assert!(!rec.is_valid()); // Deleted users are not valid
    }

    // ZScanRec tests
    #[test]
    fn test_zscanrec_default() {
        let rec = ZScanRec::default();
        assert_eq!(rec.mhiread.len(), MAX_BOARDS);
        assert!(!rec.is_msg_board_enabled(1));
        assert!(!rec.is_file_board_enabled(0));
    }

    #[test]
    fn test_zscanrec_msg_board_enable() {
        let mut rec = ZScanRec::default();
        assert!(!rec.is_msg_board_enabled(1));
        rec.enable_msg_board(1);
        assert!(rec.is_msg_board_enabled(1));
        rec.disable_msg_board(1);
        assert!(!rec.is_msg_board_enabled(1));
    }

    #[test]
    fn test_zscanrec_file_board_enable() {
        let mut rec = ZScanRec::default();
        assert!(!rec.is_file_board_enabled(10));
        rec.enable_file_board(10);
        assert!(rec.is_file_board_enabled(10));
        rec.disable_file_board(10);
        assert!(!rec.is_file_board_enabled(10));
    }

    #[test]
    fn test_zscanrec_high_read() {
        let mut rec = ZScanRec::default();
        let dt = PackedDateTime::new(2025, 11, 23, 14, 30, 0);
        rec.set_msg_high_read(5, dt);
        assert_eq!(rec.get_msg_high_read(5), Some(&dt));
        assert_eq!(rec.get_msg_high_read(0), None); // Invalid board
    }

    // ZLogRec tests
    #[test]
    fn test_zlogrec_default() {
        let rec = ZLogRec::default();
        assert!(rec.date.is_empty());
        assert_eq!(rec.calls, 0);
        assert_eq!(rec.total_messages(), 0);
        assert_eq!(rec.total_transfers(), 0);
    }

    #[test]
    fn test_zlogrec_totals() {
        let mut rec = ZLogRec::default();
        rec.pubpost = 10;
        rec.privpost = 5;
        rec.fback = 2;
        rec.uploads = 3;
        rec.downloads = 7;
        rec.uk = 1000;
        rec.dk = 2000;

        assert_eq!(rec.total_messages(), 17);
        assert_eq!(rec.total_transfers(), 10);
        assert_eq!(rec.total_kb(), 3000);
    }

    #[test]
    fn test_zlogrec_baud_distribution() {
        let mut rec = ZLogRec::default();
        rec.userbaud = [10, 20, 30, 35, 5]; // Total: 100
        let dist = rec.baud_distribution();
        // Use approximate comparison for floating point
        assert!((dist[0] - 10.0).abs() < 0.01); // 300 baud: 10%
        assert!((dist[1] - 20.0).abs() < 0.01); // 1200 baud: 20%
        assert!((dist[2] - 30.0).abs() < 0.01); // 2400 baud: 30%
        assert!((dist[3] - 35.0).abs() < 0.01); // 9600 baud: 35%
        assert!((dist[4] - 5.0).abs() < 0.01); // other: 5%
    }

    // ModemRec tests
    #[test]
    fn test_modemrec_default() {
        let rec = ModemRec::default();
        assert_eq!(rec.waitbaud, 2400);
        assert_eq!(rec.comport, 1);
        assert_eq!(rec.nocallinittime, 60);
        assert!(rec.hardware);
        assert!(rec.is_valid_port());
        assert!(rec.is_valid_baud());
    }

    #[test]
    fn test_modemrec_strings() {
        let mut rec = ModemRec::default();
        rec.init = PascalString::from_string("AT&F");
        rec.answer = PascalString::from_string("ATA");
        rec.hangup = PascalString::from_string("ATH");
        rec.offhook = PascalString::from_string("ATH1");

        assert_eq!(rec.init_string(), "AT&F");
        assert_eq!(rec.answer_string(), "ATA");
        assert_eq!(rec.hangup_string(), "ATH");
        assert_eq!(rec.offhook_string(), "ATH1");
    }

    #[test]
    fn test_modemrec_validation() {
        let mut rec = ModemRec::default();
        assert!(rec.is_valid_port());
        assert!(rec.is_valid_baud());

        rec.comport = 5; // Invalid port
        assert!(!rec.is_valid_port());

        rec.comport = 2; // Valid port
        rec.waitbaud = 12345; // Invalid baud
        assert!(!rec.is_valid_baud());
    }
}
