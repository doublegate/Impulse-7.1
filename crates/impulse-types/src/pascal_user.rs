//! Pascal-compatible user record (USER.LST format)
//!
//! This module provides the `PascalUserRec` type that exactly matches the binary
//! layout of the Pascal `userrec` type from the original Impulse 7.1 BBS.
//!
//! The structure is designed for reading and writing USER.LST files in the
//! original format for backward compatibility.

#![allow(unused_variables)] // binrw temp fields trigger false positives

use binrw::binrw;
use serde::{Deserialize, Serialize};

use crate::pascal_types::{ArFlags, DownloadScanFlags, FileListFlags, UserColors};
use crate::user_flags::UserFlags;

/// Pascal string type: length byte followed by data
///
/// Pascal stores strings as [length_byte][data_bytes...]. For a `string[N]`,
/// the binary format is 1 byte for length (0..=N) plus N bytes for data.
#[binrw]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PascalString<const N: usize> {
    /// Length of the string (0..=N)
    #[br(temp)]
    #[bw(calc = data.len().min(N) as u8)]
    #[allow(unused)]
    length: u8,

    /// String data (exactly N bytes, padded with zeros)
    #[br(count = N)]
    data: Vec<u8>,
}

impl<const N: usize> PascalString<N> {
    /// Create from Rust string
    pub fn from_string(s: impl AsRef<str>) -> Self {
        let s = s.as_ref();
        let bytes = s.as_bytes();
        let len = bytes.len().min(N);

        let mut data = vec![0u8; N];
        data[..len].copy_from_slice(&bytes[..len]);

        PascalString { data }
    }

    /// Convert to Rust string
    #[allow(clippy::inherent_to_string)] // Pascal compatibility, not Display-worthy
    pub fn to_string(&self) -> String {
        // Find the actual length (first zero or N)
        let len = self.data.iter().position(|&b| b == 0).unwrap_or(N).min(N);

        String::from_utf8_lossy(&self.data[..len]).into_owned()
    }

    /// Check if string is empty (all zeros or first byte is zero)
    pub fn is_empty(&self) -> bool {
        self.data.is_empty() || self.data[0] == 0
    }

    /// Get byte array (for direct binary access)
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }
}

impl<const N: usize> Default for PascalString<N> {
    fn default() -> Self {
        PascalString { data: vec![0u8; N] }
    }
}

/// Pascal user record (userrec from RECORDS.PAS)
///
/// Original Pascal definition (RECORDS.PAS lines 143-226):
/// ```pascal
/// userrec = record
///   name         :string[36];
///   realname     :string[36];
///   pw           :string[20];
///   ph           :string[12];
///   // ... (83 lines total)
/// end;
/// ```
///
/// This structure must maintain exact binary compatibility with the Pascal
/// version for reading/writing USER.LST files.
///
/// # Binary Size
///
/// The total size must match the Pascal record size (approximately 1200-1400 bytes).
#[binrw]
#[derive(Debug, Clone)]
pub struct PascalUserRec {
    /// User name (handle) - string[36]
    pub name: PascalString<36>,

    /// Real name - string[36]
    pub realname: PascalString<36>,

    /// Password - string[20]
    pub pw: PascalString<20>,

    /// Phone number - string[12]
    pub ph: PascalString<12>,

    /// Birthday - string[8] (format: MM/DD/YY)
    pub bday: PascalString<8>,

    /// First logon date - string[8] (format: MM/DD/YY)
    pub firston: PascalString<8>,

    /// Reserved/padding - array[1..2] of byte
    pub x1xs: [u8; 2],

    /// Last logon date - string[8] (format: MM/DD/YY)
    pub laston: PascalString<8>,

    /// Reserved/padding - array[1..2] of byte
    pub x2xs: [u8; 2],

    /// Street address - string[30]
    pub street: PascalString<30>,

    /// City, state - string[30]
    pub citystate: PascalString<30>,

    /// Zip code - string[10]
    pub zipcode: PascalString<10>,

    /// Unused space - array[1..31] of byte
    pub unused: [u8; 31],

    /// Auto-signature - string[40]
    pub autosig: PascalString<40>,

    /// Unused space - array[1..41] of byte
    pub unused2: [u8; 41],

    /// SysOp note - string[39]
    pub note: PascalString<39>,

    /// Chosen menu prompt number
    pub prompt: u8,

    /// If user is locked out
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub lockedout: bool,

    /// If account is deleted
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub deleted: bool,

    /// Lockout message filename - string[8]
    pub lockedfile: PascalString<8>,

    /// Number of 'no' votes in NUV
    pub novotes: u8,

    /// Number of 'yes' votes in NUV
    pub yesvotes: u8,

    /// User flags (Pascal: set of uflags, 3 bytes)
    #[br(map = |bytes: [u8; 3]| UserFlags::from_pascal_bytes(&bytes))]
    #[bw(map = |flags: &UserFlags| flags.to_pascal_bytes())]
    pub ac: UserFlags,

    /// File list flags (Pascal: set of flistflags, 1 byte)
    #[br(map = |b: u8| FileListFlags::from_pascal_byte(b))]
    #[bw(map = |flags: &FileListFlags| flags.to_pascal_byte())]
    pub fflag: FileListFlags,

    /// AR flags (Pascal: set of acrq, 4 bytes)
    #[br(map = |n: u32| ArFlags::from_pascal_u32(n))]
    #[bw(map = |flags: &ArFlags| flags.to_pascal_u32())]
    pub ar: ArFlags,

    /// Last read message pointers - array[1..64] of word
    pub zzqscan: [u16; 64],

    /// Reserved - array[1..64] of word
    pub xqxxx: [u16; 64],

    /// Scan boards flags - array[1..64] of boolean
    #[br(map = |bytes: [u8; 64]| bytes.map(|b| b != 0))]
    #[bw(map = |flags: &[bool; 64]| flags.map(|b| if b { 1u8 } else { 0u8 }))]
    pub zzqscn: [bool; 64],

    /// Download scan flags (Pascal: dlnscan, 16 bytes for set of 0..96)
    #[br(map = |bytes: [u8; 16]| DownloadScanFlags::from_pascal_bytes(bytes))]
    #[bw(map = |flags: &DownloadScanFlags| flags.to_pascal_bytes())]
    pub zzdlnscn: DownloadScanFlags,

    /// Unused space - array[1..20] of byte
    pub unused3: [u8; 20],

    /// User sex/gender ('M', 'F', or space)
    pub sex: u8,

    /// Total time online (minutes) - longint (i32)
    pub ttimeon: i32,

    /// Reserved - integer (i16)
    pub x1xx: i16,

    /// Upload kilobytes - longint (i32)
    pub uk: i32,

    /// Reserved - integer (i16)
    pub x2xx: i16,

    /// Download kilobytes - longint (i32)
    pub dk: i32,

    /// Reserved - integer (i16)
    pub x3xx: i16,

    /// Number of uploads
    pub uploads: i16,

    /// Number of downloads
    pub downloads: i16,

    /// Number of times logged on
    pub loggedon: i16,

    /// Time left today (minutes)
    pub tltoday: i16,

    /// Number of public message posts
    pub msgpost: i16,

    /// Number of emails sent
    pub emailsent: i16,

    /// Number of feedback messages sent
    pub feedback: i16,

    /// Forward mail to user number
    pub forusr: i16,

    /// File points
    pub filepoints: i16,

    /// Mail waiting
    pub waiting: u8,

    /// Line length (columns)
    pub linelen: u8,

    /// Page length (rows)
    pub pagelen: u8,

    /// Times on today
    pub ontoday: u8,

    /// Illegal logon attempts
    pub illegal: u8,

    /// Security level
    pub sl: u8,

    /// Download security level
    pub dsl: u8,

    /// User colors (Pascal: clrs = array[FALSE..TRUE, 0..9] of byte, 20 bytes)
    #[br(map = |bytes: [u8; 20]| {
        let mut arr = [[0u8; 10]; 2];
        arr[0].copy_from_slice(&bytes[0..10]);
        arr[1].copy_from_slice(&bytes[10..20]);
        UserColors::from_pascal_array(arr)
    })]
    #[bw(map = |colors: &UserColors| {
        let arr = colors.to_pascal_array();
        let mut bytes = [0u8; 20];
        bytes[0..10].copy_from_slice(&arr[0]);
        bytes[10..20].copy_from_slice(&arr[1]);
        bytes
    })]
    pub cols: UserColors,

    /// Last message area
    pub lastmsg: u8,

    /// Last file area
    pub lastfil: u8,

    /// Credit (dollars) - longint (i32)
    pub credit: i32,

    /// Reserved - integer (i16)
    pub x4xx: i16,

    /// Time bank (minutes) - integer (i16)
    pub timebank: i16,

    /// Board SysOp assignments - array[1..5] of byte
    pub boardsysop: [u8; 5],

    /// If trapping user activity
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub trapactivity: bool,

    /// If trap to separate TRAP file
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub trapseperate: bool,

    /// Time added to time bank today
    pub timebankadd: i16,

    /// Message pointer (*REMOVED*)
    pub mpointer: i32,

    /// If auto chat trapping
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub chatauto: bool,

    /// If separate chat file
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub chatseperate: bool,

    /// Menu to start user on - string[8]
    pub userstartmenu: PascalString<8>,

    /// Separate SysOp log
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub slogseperate: bool,

    /// Clear screen message option (1=clear, 2=don't clear)
    pub clsmsg: u8,

    /// File list option type
    pub flistopt: u8,

    /// Message order (0=Chronological, 1=ReplyTree)
    pub msgorder: u8,

    /// AVATAR color adjust (1=no, 2=yes)
    pub avadjust: u8,
}

impl Default for PascalUserRec {
    fn default() -> Self {
        PascalUserRec {
            name: PascalString::default(),
            realname: PascalString::default(),
            pw: PascalString::default(),
            ph: PascalString::default(),
            bday: PascalString::default(),
            firston: PascalString::default(),
            x1xs: [0; 2],
            laston: PascalString::default(),
            x2xs: [0; 2],
            street: PascalString::default(),
            citystate: PascalString::default(),
            zipcode: PascalString::default(),
            unused: [0; 31],
            autosig: PascalString::default(),
            unused2: [0; 41],
            note: PascalString::default(),
            prompt: 0,
            lockedout: false,
            deleted: false,
            lockedfile: PascalString::default(),
            novotes: 0,
            yesvotes: 0,
            ac: UserFlags::default(),
            fflag: FileListFlags::default(),
            ar: ArFlags::default(),
            zzqscan: [0; 64],
            xqxxx: [0; 64],
            zzqscn: [false; 64],
            zzdlnscn: DownloadScanFlags::default(),
            unused3: [0; 20],
            sex: b' ',
            ttimeon: 0,
            x1xx: 0,
            uk: 0,
            x2xx: 0,
            dk: 0,
            x3xx: 0,
            uploads: 0,
            downloads: 0,
            loggedon: 0,
            tltoday: 0,
            msgpost: 0,
            emailsent: 0,
            feedback: 0,
            forusr: 0,
            filepoints: 0,
            waiting: 0,
            linelen: 80,
            pagelen: 24,
            ontoday: 0,
            illegal: 0,
            sl: 10, // New user level
            dsl: 10,
            cols: UserColors::default(),
            lastmsg: 0,
            lastfil: 0,
            credit: 0,
            x4xx: 0,
            timebank: 0,
            boardsysop: [0; 5],
            trapactivity: false,
            trapseperate: false,
            timebankadd: 0,
            mpointer: 0,
            chatauto: false,
            chatseperate: false,
            userstartmenu: PascalString::default(),
            slogseperate: false,
            clsmsg: 1, // Clear screen
            flistopt: 0,
            msgorder: 0, // Chronological
            avadjust: 1, // No adjust
        }
    }
}

impl PascalUserRec {
    /// Create a new user record with minimal initialization
    #[allow(clippy::field_reassign_with_default)] // Pascal-style initialization pattern
    pub fn new(name: impl AsRef<str>) -> Self {
        let mut rec = Self::default();
        rec.name = PascalString::from_string(name);
        rec
    }

    /// Get the username as a Rust string
    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    /// Set the username
    pub fn set_name(&mut self, name: impl AsRef<str>) {
        self.name = PascalString::from_string(name);
    }

    /// Get the real name as a Rust string
    pub fn get_realname(&self) -> String {
        self.realname.to_string()
    }

    /// Check if user is active (not deleted and not locked out)
    pub fn is_active(&self) -> bool {
        !self.deleted && !self.lockedout
    }

    /// Calculate upload/download ratio
    pub fn ul_dl_ratio(&self) -> Option<f64> {
        if self.dk == 0 {
            None
        } else {
            Some(self.uk as f64 / self.dk as f64)
        }
    }

    /// Check if user is a SysOp (security level 255)
    pub fn is_sysop(&self) -> bool {
        self.sl == 255
    }
}

#[cfg(test)]
#[allow(clippy::field_reassign_with_default)]
mod tests {
    use super::*;

    #[test]
    fn test_pascal_string_from_string() {
        let ps: PascalString<10> = PascalString::from_string("Hello");
        assert_eq!(ps.to_string(), "Hello");

        // Test truncation
        let ps2: PascalString<5> = PascalString::from_string("HelloWorld");
        assert_eq!(ps2.to_string(), "Hello");
    }

    #[test]
    fn test_pascal_string_default() {
        let ps: PascalString<20> = PascalString::default();
        assert_eq!(ps.to_string(), "");
    }

    #[test]
    fn test_pascal_user_rec_default() {
        let user = PascalUserRec::default();
        assert_eq!(user.get_name(), "");
        assert!(!user.deleted);
        assert!(!user.lockedout);
        assert_eq!(user.sl, 10); // New user level
    }

    #[test]
    fn test_pascal_user_rec_new() {
        let user = PascalUserRec::new("TestUser");
        assert_eq!(user.get_name(), "TestUser");
        assert!(user.is_active());
    }

    #[test]
    fn test_pascal_user_rec_is_active() {
        let mut user = PascalUserRec::new("Test");
        assert!(user.is_active());

        user.deleted = true;
        assert!(!user.is_active());

        user.deleted = false;
        user.lockedout = true;
        assert!(!user.is_active());
    }

    #[test]
    fn test_pascal_user_rec_ratio() {
        let mut user = PascalUserRec::new("Test");
        assert_eq!(user.ul_dl_ratio(), None); // No downloads

        user.uk = 1000;
        user.dk = 500;
        assert_eq!(user.ul_dl_ratio(), Some(2.0));
    }

    #[test]
    fn test_pascal_user_rec_is_sysop() {
        let mut user = PascalUserRec::new("Test");
        assert!(!user.is_sysop());

        user.sl = 255;
        assert!(user.is_sysop());
    }
}
