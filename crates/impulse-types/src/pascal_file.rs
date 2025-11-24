//! Pascal-compatible file system records
//!
//! This module provides binary-compatible representations of the file system records
//! from the original Pascal RECORDS.PAS file (lines 641-691).
//!
//! # File Format Overview
//!
//! - **UPLOADS.DAT**: File base configuration records (UlRec)
//! - **\*.DIR files**: Individual file records (UlFRec)
//! - **VERBOSE.DAT**: Extended file descriptions (VerbRec)
//!
//! # Binary Compatibility
//!
//! All structures maintain exact field ordering and sizing as the Pascal originals
//! for direct binary I/O with legacy BBS data files.

use binrw::binrw;
use serde::{Deserialize, Serialize};

use crate::board_flags::FileBoardFlags;
use crate::pascal_user::PascalString;

/// File status flags (Pascal: `filstat`)
///
/// Original Pascal definition (RECORDS.PAS lines 661-665):
/// ```pascal
/// filstat =
///    (notval,                       { if file is NOT validated }
///     isrequest,                    { if file is REQUEST }
///     resumelater);                 { if file is RESUME-LATER }
/// ```
///
/// Status flags for uploaded files. These are stored as bitflags in Pascal
/// `set of filstat`, which we represent as a u8 bitfield.
///
/// # Examples
///
/// ```
/// use impulse_types::pascal_file::FileStatus;
///
/// let status = FileStatus::NOT_VALIDATED | FileStatus::IS_REQUEST;
/// assert!(status.needs_validation());
/// assert!(status.is_request());
/// ```
use bitflags::bitflags;

bitflags! {
    /// File status flags
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct FileStatus: u8 {
        /// File is not yet validated by sysop
        const NOT_VALIDATED = 0b0000_0001;

        /// File is a download request (not uploaded yet)
        const IS_REQUEST = 0b0000_0010;

        /// File download should be resumed later
        const RESUME_LATER = 0b0000_0100;
    }
}

impl Default for FileStatus {
    fn default() -> Self {
        FileStatus::NOT_VALIDATED
    }
}

impl FileStatus {
    /// Create from Pascal byte
    pub fn from_pascal_byte(byte: u8) -> Self {
        FileStatus::from_bits_truncate(byte)
    }

    /// Convert to Pascal byte
    pub fn to_pascal_byte(self) -> u8 {
        self.bits()
    }

    /// Check if file needs validation
    pub fn needs_validation(self) -> bool {
        self.contains(FileStatus::NOT_VALIDATED)
    }

    /// Check if file is a request
    pub fn is_request(self) -> bool {
        self.contains(FileStatus::IS_REQUEST)
    }

    /// Check if file should be resumed later
    pub fn should_resume_later(self) -> bool {
        self.contains(FileStatus::RESUME_LATER)
    }

    /// Check if file is validated and ready
    pub fn is_ready(self) -> bool {
        !self.intersects(FileStatus::NOT_VALIDATED | FileStatus::IS_REQUEST)
    }
}

/// File base record (Pascal: `ulrec`)
///
/// Original Pascal definition (RECORDS.PAS lines 641-660):
/// ```pascal
/// ulrec=                          { UPLOADS.DAT : File base records }
/// record
///   name:string[40];              { area description  }
///   filename:string[12];          { filename + ".DIR" }
///   dlpath,                       { download path     }
///   ulpath:string[40];            { upload path       }
///   maxfiles:integer;             { max files allowed }
///   password:string[20];          { password required }
///   arctype,                      { wanted archive type (1..max,0=inactive) }
///   cmttype:byte;                 { wanted comment type (1..3,0=inactive) }
///   fbdepth:integer;              { file base dir depth }
///   fbstat:set of fbflags;        { file base status vars }
///   acs,                          { access requirements }
///   ulacs,                        { upload requirements }
///   nameacs:acstring;             { see-names requirements }
///   permindx:longint;             { permanent index # }
///   Conf:byte;                    { fbase conference }
///   res:array[1..5] of byte;      { RESERVED }
/// end;
/// ```
///
/// Represents a file area/base configuration in UPLOADS.DAT.
#[binrw]
#[derive(Debug, Clone, Default)]
pub struct UlRec {
    /// Area description (display name)
    pub name: PascalString<40>,

    /// Filename for DIR file (e.g., "FILES.DIR")
    pub filename: PascalString<12>,

    /// Download path (where files are stored)
    pub dlpath: PascalString<40>,

    /// Upload path (where uploads go initially)
    pub ulpath: PascalString<40>,

    /// Maximum files allowed in this area
    pub maxfiles: i16,

    /// Password required to access (optional)
    pub password: PascalString<20>,

    /// Wanted archive type (1..max, 0=inactive)
    pub arctype: u8,

    /// Wanted comment type (1..3, 0=inactive)
    pub cmttype: u8,

    /// File base directory depth
    pub fbdepth: i16,

    /// File base status flags
    #[br(map = |b: u16| FileBoardFlags::from_pascal_word(b))]
    #[bw(map = |f: &FileBoardFlags| f.to_pascal_word())]
    pub fbstat: FileBoardFlags,

    /// Access requirements string
    pub acs: PascalString<20>,

    /// Upload requirements string
    pub ulacs: PascalString<20>,

    /// See-names requirements string
    pub nameacs: PascalString<20>,

    /// Permanent index number
    pub permindx: i32,

    /// File base conference number
    pub conf: u8,

    /// Reserved bytes
    pub res: [u8; 5],
}

impl UlRec {
    /// Check if area has password protection
    pub fn has_password(&self) -> bool {
        !self.password.is_empty()
    }

    /// Verify password (case-insensitive)
    pub fn verify_password(&self, password: &str) -> bool {
        if !self.has_password() {
            return true; // No password required
        }
        self.password.to_string().eq_ignore_ascii_case(password)
    }

    /// Check if area is full (reached max files)
    pub fn is_full(&self, current_count: i16) -> bool {
        self.maxfiles > 0 && current_count >= self.maxfiles
    }

    /// Check if area requires specific archive type
    pub fn requires_archive_type(&self) -> bool {
        self.arctype > 0
    }

    /// Check if area requires comment file
    pub fn requires_comment(&self) -> bool {
        self.cmttype > 0
    }

    /// Get display name
    pub fn display_name(&self) -> String {
        self.name.to_string()
    }

    /// Get full download path
    pub fn download_path(&self) -> String {
        self.dlpath.to_string()
    }

    /// Get full upload path
    pub fn upload_path(&self) -> String {
        self.ulpath.to_string()
    }
}

/// File record (Pascal: `ulfrec`)
///
/// Original Pascal definition (RECORDS.PAS lines 666-683):
/// ```pascal
/// ulfrec=                         { *.DIR : File records }
/// record
///   filename:string[12];          { Filename }
///   description:string[60];       { File description }
///   filepoints:integer;           { File points }
///   nacc:integer;                 { Number DLs }
///   ft:byte;                      { File type (useless?) }
///   blocks:integer;               { # 128 byte blks }
///   owner:integer;                { ULer of file }
///   stowner:string[36];           { ULer's name }
///   date:string[8];               { Date ULed }
///   daten:integer;                { Numeric date ULed }
///   vpointer:longint;             { Pointer to verbose descr, -1 if none }
///   filestat:set of filstat;      { File status }
///   priv:boolean;                 { private for someone? }
///   privfor:string[8];            { a short string of who its for }
/// end;
/// ```
///
/// Represents an individual file entry in a \*.DIR file.
#[binrw]
#[derive(Debug, Clone)]
pub struct UlFRec {
    /// Filename (e.g., "FILE.ZIP")
    pub filename: PascalString<12>,

    /// Short description (one line)
    pub description: PascalString<60>,

    /// File points/credits cost
    pub filepoints: i16,

    /// Number of downloads
    pub nacc: i16,

    /// File type indicator (legacy, mostly unused)
    pub ft: u8,

    /// File size in 128-byte blocks
    pub blocks: i16,

    /// User number who uploaded the file
    pub owner: i16,

    /// Uploader's name
    pub stowner: PascalString<36>,

    /// Upload date (MM/DD/YY format)
    pub date: PascalString<8>,

    /// Numeric date value (days since epoch)
    pub daten: i16,

    /// Pointer to verbose description in VERBOSE.DAT (-1 if none)
    pub vpointer: i32,

    /// File status flags
    #[br(map = |b: u8| FileStatus::from_pascal_byte(b))]
    #[bw(map = |f: &FileStatus| f.to_pascal_byte())]
    pub filestat: FileStatus,

    /// Is file private for someone?
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |b: &bool| if *b { 1u8 } else { 0u8 })]
    pub priv_flag: bool,

    /// Who file is private for (if priv is true)
    pub privfor: PascalString<8>,
}

impl Default for UlFRec {
    fn default() -> Self {
        Self {
            filename: PascalString::default(),
            description: PascalString::default(),
            filepoints: 0,
            nacc: 0,
            ft: 0,
            blocks: 0,
            owner: 0,
            stowner: PascalString::default(),
            date: PascalString::default(),
            daten: 0,
            vpointer: -1,
            filestat: FileStatus::default(),
            priv_flag: false,
            privfor: PascalString::default(),
        }
    }
}

impl UlFRec {
    /// Calculate file size in bytes (blocks * 128)
    pub fn file_size_bytes(&self) -> usize {
        (self.blocks as usize) * 128
    }

    /// Calculate file size in KB
    pub fn file_size_kb(&self) -> usize {
        self.file_size_bytes() / 1024
    }

    /// Check if file has verbose description
    pub fn has_verbose_description(&self) -> bool {
        self.vpointer >= 0
    }

    /// Check if file is validated
    pub fn is_validated(&self) -> bool {
        self.filestat.is_ready()
    }

    /// Check if file is private
    pub fn is_private(&self) -> bool {
        self.priv_flag
    }

    /// Get private recipient name
    pub fn private_for(&self) -> Option<String> {
        if self.priv_flag && !self.privfor.is_empty() {
            Some(self.privfor.to_string())
        } else {
            None
        }
    }

    /// Get uploader name
    pub fn uploader(&self) -> String {
        self.stowner.to_string()
    }

    /// Get short description
    pub fn short_description(&self) -> String {
        self.description.to_string()
    }

    /// Get filename
    pub fn name(&self) -> String {
        self.filename.to_string()
    }

    /// Check if file costs points/credits
    pub fn has_cost(&self) -> bool {
        self.filepoints > 0
    }

    /// Get download count
    pub fn download_count(&self) -> i16 {
        self.nacc
    }

    /// Increment download counter
    pub fn record_download(&mut self) {
        self.nacc = self.nacc.saturating_add(1);
    }
}

/// Verbose description record (Pascal: `verbrec`)
///
/// Original Pascal definition (RECORDS.PAS lines 685-688):
/// ```pascal
/// verbrec=                        { VERBOSE.DAT : Verbose descriptions }
/// record
///   descr:array[1..20] of string[50];
/// end;
/// ```
///
/// Extended file description (up to 20 lines of 50 characters each).
#[binrw]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerbRec {
    /// Array of 20 description lines (50 chars each)
    #[br(count = 20)]
    #[bw(args_raw = ())]
    pub descr: Vec<PascalString<50>>,
}

impl Default for VerbRec {
    fn default() -> Self {
        Self {
            descr: vec![PascalString::default(); 20],
        }
    }
}

impl VerbRec {
    /// Create new verbose description
    pub fn new() -> Self {
        Self::default()
    }

    /// Get full description as multi-line string
    pub fn full_description(&self) -> String {
        self.descr
            .iter()
            .map(|line| line.to_string())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Set description from lines
    pub fn set_description(&mut self, lines: &[&str]) {
        for (i, line) in lines.iter().enumerate().take(20) {
            let ps = PascalString::from_string(line);
            self.descr[i] = ps;
        }
    }

    /// Get line count (non-empty lines)
    pub fn line_count(&self) -> usize {
        self.descr.iter().filter(|line| !line.is_empty()).count()
    }

    /// Get specific line
    pub fn get_line(&self, index: usize) -> Option<String> {
        self.descr.get(index).map(|line| line.to_string())
    }
}

#[cfg(test)]
#[allow(clippy::field_reassign_with_default)]
mod tests {
    use super::*;

    // FileStatus tests
    #[test]
    fn test_file_status_default() {
        let status = FileStatus::default();
        assert!(status.needs_validation());
        assert!(!status.is_ready());
    }

    #[test]
    fn test_file_status_flags() {
        let mut status = FileStatus::NOT_VALIDATED;
        assert!(status.needs_validation());
        assert!(!status.is_request());

        status.insert(FileStatus::IS_REQUEST);
        assert!(status.is_request());
        assert!(status.needs_validation());

        status.remove(FileStatus::NOT_VALIDATED);
        assert!(!status.needs_validation());
        assert!(status.is_request());
    }

    #[test]
    fn test_file_status_ready() {
        let status = FileStatus::empty();
        assert!(status.is_ready());

        let status = FileStatus::NOT_VALIDATED;
        assert!(!status.is_ready());

        let status = FileStatus::IS_REQUEST;
        assert!(!status.is_ready());
    }

    // UlRec tests
    #[test]
    fn test_ulrec_default() {
        let ulrec = UlRec::default();
        assert!(!ulrec.has_password());
        assert!(!ulrec.requires_archive_type());
        assert!(!ulrec.requires_comment());
    }

    #[test]
    fn test_ulrec_password() {
        let mut ulrec = UlRec::default();
        assert!(ulrec.verify_password("")); // No password set

        ulrec.password = PascalString::from_string("SECRET");
        assert!(ulrec.has_password());
        assert!(ulrec.verify_password("secret")); // Case insensitive
        assert!(ulrec.verify_password("SECRET"));
        assert!(!ulrec.verify_password("wrong"));
    }

    #[test]
    fn test_ulrec_full() {
        let mut ulrec = UlRec::default();
        ulrec.maxfiles = 100;

        assert!(!ulrec.is_full(50));
        assert!(!ulrec.is_full(99));
        assert!(ulrec.is_full(100));
        assert!(ulrec.is_full(150));
    }

    #[test]
    fn test_ulrec_requirements() {
        let mut ulrec = UlRec::default();
        assert!(!ulrec.requires_archive_type());
        assert!(!ulrec.requires_comment());

        ulrec.arctype = 1;
        assert!(ulrec.requires_archive_type());

        ulrec.cmttype = 1;
        assert!(ulrec.requires_comment());
    }

    // UlFRec tests
    #[test]
    fn test_ulfrec_default() {
        let ulfrec = UlFRec::default();
        assert_eq!(ulfrec.file_size_bytes(), 0);
        assert!(!ulfrec.has_verbose_description());
        assert!(!ulfrec.is_private());
        assert!(!ulfrec.has_cost());
    }

    #[test]
    fn test_ulfrec_file_size() {
        let mut ulfrec = UlFRec::default();
        ulfrec.blocks = 10;

        assert_eq!(ulfrec.file_size_bytes(), 1280); // 10 * 128
        assert_eq!(ulfrec.file_size_kb(), 1); // 1280 / 1024
    }

    #[test]
    fn test_ulfrec_verbose() {
        let mut ulfrec = UlFRec::default();
        assert!(!ulfrec.has_verbose_description());

        ulfrec.vpointer = 0;
        assert!(ulfrec.has_verbose_description());

        ulfrec.vpointer = 100;
        assert!(ulfrec.has_verbose_description());

        ulfrec.vpointer = -1;
        assert!(!ulfrec.has_verbose_description());
    }

    #[test]
    fn test_ulfrec_private() {
        let mut ulfrec = UlFRec::default();
        assert!(!ulfrec.is_private());
        assert_eq!(ulfrec.private_for(), None);

        ulfrec.priv_flag = true;
        ulfrec.privfor = PascalString::from_string("JOHN");
        assert!(ulfrec.is_private());
        assert_eq!(ulfrec.private_for(), Some("JOHN".to_string()));
    }

    #[test]
    fn test_ulfrec_downloads() {
        let mut ulfrec = UlFRec::default();
        assert_eq!(ulfrec.download_count(), 0);

        ulfrec.record_download();
        assert_eq!(ulfrec.download_count(), 1);

        ulfrec.record_download();
        assert_eq!(ulfrec.download_count(), 2);
    }

    #[test]
    fn test_ulfrec_validation() {
        let mut ulfrec = UlFRec::default();
        assert!(!ulfrec.is_validated()); // Default has NOT_VALIDATED

        ulfrec.filestat = FileStatus::empty();
        assert!(ulfrec.is_validated());
    }

    // VerbRec tests
    #[test]
    fn test_verbrec_default() {
        let verbrec = VerbRec::default();
        assert_eq!(verbrec.line_count(), 0);
        assert_eq!(verbrec.full_description(), "");
    }

    #[test]
    fn test_verbrec_set_description() {
        let mut verbrec = VerbRec::new();
        let lines = vec!["Line 1", "Line 2", "Line 3"];
        verbrec.set_description(&lines);

        assert_eq!(verbrec.line_count(), 3);
        assert_eq!(verbrec.get_line(0), Some("Line 1".to_string()));
        assert_eq!(verbrec.get_line(1), Some("Line 2".to_string()));
        assert_eq!(verbrec.get_line(2), Some("Line 3".to_string()));
    }

    #[test]
    fn test_verbrec_full_description() {
        let mut verbrec = VerbRec::new();
        let lines = vec!["First line", "Second line", "Third line"];
        verbrec.set_description(&lines);

        let full = verbrec.full_description();
        assert_eq!(full, "First line\nSecond line\nThird line");
    }

    #[test]
    fn test_verbrec_max_lines() {
        let mut verbrec = VerbRec::new();
        let lines: Vec<&str> = (0..25).map(|_| "Line").collect();
        verbrec.set_description(&lines);

        assert_eq!(verbrec.line_count(), 20); // Max 20 lines
    }
}
