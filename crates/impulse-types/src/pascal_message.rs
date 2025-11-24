//! Pascal-compatible message system types
//!
//! This module contains Pascal-compatible structures for the Impulse 7.1 message system,
//! supporting binary I/O with original Pascal data files (*.MIX, *.BRD, BOARDS.DAT).
//!
//! # File Formats
//!
//! - `*.MIX` - Message index files (MsgIndexRec records)
//! - `*.BRD` - Message board files (MHeaderRec records)
//! - `BOARDS.DAT` - Board configuration file (BoardRec records)
//!
//! # Key Types
//!
//! - [`CPackDateTime`] - Packed date/time (6 bytes: year, month, day, hour, min, sec)
//! - [`MsgIndexRec`] - Message index entry with ID, pointers, and metadata
//! - [`FromToInfo`] - From/To information (user, name, anonymous status)
//! - [`MHeaderRec`] - Message header with signature, pointers, and from/to info
//! - [`BoardRec`] - Message board configuration with access rules and settings

use binrw::binrw;
use serde::{Deserialize, Serialize};

use crate::board_flags::MessageBoardFlags;
use crate::message_enums::{AnonymousType, MessageIndexStatus};
use crate::pascal_user::PascalString;

/// Packed date/time (Pascal: `cpackdatetime = array[1..6] of byte`)
///
/// Stores date and time in 6 bytes:
/// - [0]: Year (0-99, relative to 1900 or 2000)
/// - [1]: Month (1-12)
/// - [2]: Day (1-31)
/// - [3]: Hour (0-23)
/// - [4]: Minute (0-59)
/// - [5]: Second (0-59)
#[binrw]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Default)]
pub struct CPackDateTime {
    /// Packed date/time bytes
    pub bytes: [u8; 6],
}


impl CPackDateTime {
    /// Create a new packed date/time from components
    pub fn new(year: u8, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> Self {
        Self {
            bytes: [year, month, day, hour, minute, second],
        }
    }

    /// Get year component (0-99)
    pub fn year(&self) -> u8 {
        self.bytes[0]
    }

    /// Get month component (1-12)
    pub fn month(&self) -> u8 {
        self.bytes[1]
    }

    /// Get day component (1-31)
    pub fn day(&self) -> u8 {
        self.bytes[2]
    }

    /// Get hour component (0-23)
    pub fn hour(&self) -> u8 {
        self.bytes[3]
    }

    /// Get minute component (0-59)
    pub fn minute(&self) -> u8 {
        self.bytes[4]
    }

    /// Get second component (0-59)
    pub fn second(&self) -> u8 {
        self.bytes[5]
    }

    /// Set year component (0-99)
    pub fn set_year(&mut self, year: u8) {
        self.bytes[0] = year;
    }

    /// Set month component (1-12)
    pub fn set_month(&mut self, month: u8) {
        self.bytes[1] = month;
    }

    /// Set day component (1-31)
    pub fn set_day(&mut self, day: u8) {
        self.bytes[2] = day;
    }

    /// Set hour component (0-23)
    pub fn set_hour(&mut self, hour: u8) {
        self.bytes[3] = hour;
    }

    /// Set minute component (0-59)
    pub fn set_minute(&mut self, minute: u8) {
        self.bytes[4] = minute;
    }

    /// Set second component (0-59)
    pub fn set_second(&mut self, second: u8) {
        self.bytes[5] = second;
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

/// Message index record (Pascal: `msgindexrec`)
///
/// Stored in `*.MIX` files, one record per message in the message base.
/// Used for quick message scanning without loading full headers.
///
/// # Pascal Definition
/// ```pascal
/// msgindexrec = record
///   messagenum: word;           { message number, tonum in EMAIL.MIX }
///   hdrptr: longint;            { pointer to message header }
///   msgid: longint;             { message ID (sequential) }
///   isreplytoid: longint;       { ID of replied message }
///   msgdate: cpackdatetime;     { message date/time (packed) }
///   msgdowk: byte;              { message day-of-week (0=Sun ...) }
///   msgindexstat: set of msgindexstatr; { status flags }
///   isreplyto: word;            { reply this message is to (-1=None) }
///   numreplys: word;            { number of replies to THIS message }
/// end;
/// ```
#[binrw]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MsgIndexRec {
    /// Message number (or recipient user number in EMAIL.MIX)
    pub messagenum: u16,

    /// File pointer to message header in *.BRD file
    pub hdrptr: i32,

    /// Unique sequential message ID
    pub msgid: i32,

    /// Message ID that this message replies to (0 if not a reply)
    pub isreplytoid: i32,

    /// Date and time message was posted (packed format)
    pub msgdate: CPackDateTime,

    /// Day of week (0=Sunday, 1=Monday, ..., 6=Saturday)
    pub msgdowk: u8,

    /// Message index status flags
    #[br(map = |b: u8| MessageIndexStatus::from_pascal_byte(b))]
    #[bw(map = |s: &MessageIndexStatus| s.to_pascal_byte())]
    pub msgindexstat: MessageIndexStatus,

    /// Message number this is a reply to (u16::MAX if not a reply)
    pub isreplyto: u16,

    /// Number of replies to this message
    pub numreplys: u16,
}

impl Default for MsgIndexRec {
    fn default() -> Self {
        Self {
            messagenum: 0,
            hdrptr: 0,
            msgid: 0,
            isreplytoid: 0,
            msgdate: CPackDateTime::default(),
            msgdowk: 0,
            msgindexstat: MessageIndexStatus::default(),
            isreplyto: u16::MAX,
            numreplys: 0,
        }
    }
}

impl MsgIndexRec {
    /// Check if this message is a reply to another message
    pub fn is_reply(&self) -> bool {
        self.isreplytoid != 0 && self.isreplyto != u16::MAX
    }

    /// Check if this message has replies
    pub fn has_replies(&self) -> bool {
        self.numreplys > 0
    }

    /// Check if message is deleted
    pub fn is_deleted(&self) -> bool {
        self.msgindexstat.is_deleted()
    }

    /// Check if message is valid (not invalid or dead)
    pub fn is_valid(&self) -> bool {
        self.msgindexstat.is_valid()
    }

    /// Get day of week as string
    pub fn day_of_week_string(&self) -> &'static str {
        match self.msgdowk {
            0 => "Sunday",
            1 => "Monday",
            2 => "Tuesday",
            3 => "Wednesday",
            4 => "Thursday",
            5 => "Friday",
            6 => "Saturday",
            _ => "Unknown",
        }
    }
}

/// From/To information (Pascal: `fromtoinfo`)
///
/// Embedded in message headers to store sender/recipient information.
/// Supports anonymous posting with multiple name display options.
///
/// # Pascal Definition
/// ```pascal
/// fromtoinfo = record
///   anon: byte;                 { anonymous type }
///   usernum: word;              { user number }
///   asn: string[42];            { given name for this case }
///   real: string[36];           { user real name }
///   alias: string[36];          { user alias }
/// end;
/// ```
#[binrw]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FromToInfo {
    /// Anonymous type (maps to AnonymousType enum via byte value)
    #[br(map = |b: u8| AnonymousType::from_pascal_byte(b))]
    #[bw(map = |a| a.to_pascal_byte())]
    pub anon: AnonymousType,

    /// User number (0 if system message or unknown)
    pub usernum: u16,

    /// Given/assigned name for this message (may be alias, pseudonym, etc.)
    pub asn: PascalString<42>,

    /// User's real name
    pub real: PascalString<36>,

    /// User's alias/handle
    pub alias: PascalString<36>,
}

impl Default for FromToInfo {
    fn default() -> Self {
        Self {
            anon: AnonymousType::NotAllowed,
            usernum: 0,
            asn: PascalString::default(),
            real: PascalString::default(),
            alias: PascalString::default(),
        }
    }
}

impl FromToInfo {
    /// Get display name based on anonymous type and preferences
    pub fn display_name(&self) -> String {
        match self.anon {
            AnonymousType::NotAllowed => {
                // Use asn if set, otherwise real name, otherwise alias
                if !self.asn.is_empty() {
                    self.asn.to_string()
                } else if !self.real.is_empty() {
                    self.real.to_string()
                } else {
                    self.alias.to_string()
                }
            }
            AnonymousType::Allowed | AnonymousType::Forced => "Anonymous".to_string(),
            AnonymousType::DearAbby => "Dear Abby".to_string(),
            AnonymousType::AnyName => {
                // Use asn for custom anonymous name
                if !self.asn.is_empty() {
                    self.asn.to_string()
                } else {
                    "Anonymous".to_string()
                }
            }
        }
    }

    /// Check if message is anonymous
    pub fn is_anonymous(&self) -> bool {
        !matches!(self.anon, AnonymousType::NotAllowed)
    }

    /// Create from/to info for a user
    pub fn from_user(usernum: u16, real: &str, alias: &str) -> Self {
        Self {
            anon: AnonymousType::NotAllowed,
            usernum,
            asn: PascalString::from_string(alias), // Default to alias for display
            real: PascalString::from_string(real),
            alias: PascalString::from_string(alias),
        }
    }

    /// Create anonymous from/to info
    pub fn anonymous(anon_type: AnonymousType) -> Self {
        Self {
            anon: anon_type,
            usernum: 0,
            asn: PascalString::default(),
            real: PascalString::default(),
            alias: PascalString::default(),
        }
    }
}

/// Message header record (Pascal: `mheaderrec`)
///
/// Stored in `*.BRD` files, contains full message metadata and pointers
/// to message text. Each header is preceded by its pointer in the index.
///
/// # Pascal Definition
/// ```pascal
/// mheaderrec = record
///   signature: longint;         { header signature - $FFFFFFFF }
///   msgptr: longint;            { pointer to message text }
///   msglength: longint;         { length of message text }
///   fromi: fromtoinfo;          { from information }
///   toi: fromtoinfo;            { to information }
///   title: string[60];          { title of message }
///   origindate: string[19];     { Echo/Group original msg date }
/// end;
/// ```
#[binrw]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MHeaderRec {
    /// Header signature (should be 0xFFFFFFFF / -1)
    pub signature: i32,

    /// File pointer to message text
    pub msgptr: i32,

    /// Length of message text in bytes
    pub msglength: i32,

    /// From information (sender)
    pub fromi: FromToInfo,

    /// To information (recipient)
    pub toi: FromToInfo,

    /// Message subject/title
    pub title: PascalString<60>,

    /// Original message date for echoed/networked messages
    pub origindate: PascalString<19>,
}

impl Default for MHeaderRec {
    fn default() -> Self {
        Self {
            signature: -1, // 0xFFFFFFFF
            msgptr: 0,
            msglength: 0,
            fromi: FromToInfo::default(),
            toi: FromToInfo::default(),
            title: PascalString::default(),
            origindate: PascalString::default(),
        }
    }
}

impl MHeaderRec {
    /// Standard signature value for valid headers
    pub const SIGNATURE: i32 = -1; // 0xFFFFFFFF

    /// Check if header has valid signature
    pub fn is_valid_signature(&self) -> bool {
        self.signature == Self::SIGNATURE
    }

    /// Get message title as string
    pub fn get_title(&self) -> String {
        self.title.to_string()
    }

    /// Set message title
    pub fn set_title(&mut self, title: impl AsRef<str>) {
        self.title = PascalString::from_string(title);
    }

    /// Get sender display name
    pub fn from_name(&self) -> String {
        self.fromi.display_name()
    }

    /// Get recipient display name
    pub fn to_name(&self) -> String {
        self.toi.display_name()
    }

    /// Check if message is to/from anonymous user
    pub fn is_anonymous(&self) -> bool {
        self.fromi.is_anonymous() || self.toi.is_anonymous()
    }

    /// Get original message date (for echoed messages)
    pub fn get_origin_date(&self) -> String {
        self.origindate.to_string()
    }

    /// Set original message date
    pub fn set_origin_date(&mut self, date: impl AsRef<str>) {
        self.origindate = PascalString::from_string(date);
    }

    /// Check if message has origin date (is echoed/networked)
    pub fn is_echoed(&self) -> bool {
        !self.origindate.is_empty()
    }

    /// Create a new message header
    pub fn new(
        from_user: u16,
        from_real: &str,
        from_alias: &str,
        to_user: u16,
        to_real: &str,
        to_alias: &str,
        title: &str,
    ) -> Self {
        Self {
            signature: Self::SIGNATURE,
            msgptr: 0,
            msglength: 0,
            fromi: FromToInfo::from_user(from_user, from_real, from_alias),
            toi: FromToInfo::from_user(to_user, to_real, to_alias),
            title: PascalString::from_string(title),
            origindate: PascalString::default(),
        }
    }
}

/// Message board record (Pascal: `boardrec`)
///
/// Stored in `BOARDS.DAT`, one record per message board/conference.
/// Contains board configuration, access rules, and FidoNet settings.
///
/// # Pascal Definition
/// ```pascal
/// boardrec = record
///   name: string[40];              { message base description }
///   filename: string[8];           { BRD/MIX data filename }
///   lastmsgid: longint;            { last message ID number }
///   msgpath: string[40];           { messages pathname }
///   acs, postacs, mciacs: acstring; { access requirements }
///   maxmsgs: word;                 { max message count }
///   anonymous: anontyp;            { anonymous type }
///   password: string[20];          { base password }
///   mbstat: set of mbflags;        { message base status vars }
///   permindx: longint;             { permanent index # }
///   mbtype: integer;               { base type (0=Local,1=Echo,2=XMail) }
///   origin: string[50];            { origin line }
///   text_color, quote_color, tear_color, origin_color: byte;
///   zone, net, node, point: integer; { FidoNet address }
///   conf: byte;
///   res: array[1..2] of byte;      { RESERVED }
/// end;
/// ```
#[binrw]
#[derive(Debug, Clone)]
pub struct BoardRec {
    /// Board name/description
    pub name: PascalString<40>,

    /// Base filename for .BRD/.MIX files (without extension)
    pub filename: PascalString<8>,

    /// Last assigned message ID (sequential)
    pub lastmsgid: i32,

    /// Path to message files
    pub msgpath: PascalString<40>,

    /// Access requirement string (read access)
    pub acs: PascalString<20>,

    /// Post access requirement string
    pub postacs: PascalString<20>,

    /// MCI usage requirement string
    pub mciacs: PascalString<20>,

    /// Maximum message count (0 = unlimited)
    pub maxmsgs: u16,

    /// Anonymous message type allowed
    #[br(map = |b: u8| AnonymousType::from_pascal_byte(b))]
    #[bw(map = |a| a.to_pascal_byte())]
    pub anonymous: AnonymousType,

    /// Board password (optional)
    pub password: PascalString<20>,

    /// Message base status flags
    #[br(map = |b: u8| MessageBoardFlags::from_pascal_byte(b))]
    #[bw(map = |f: &MessageBoardFlags| f.to_pascal_byte())]
    pub mbstat: MessageBoardFlags,

    /// Permanent index number (for renumbering)
    pub permindx: i32,

    /// Board type: 0=Local, 1=EchoMail, 2=NetMail
    pub mbtype: i16,

    /// Origin line for FidoNet messages
    pub origin: PascalString<50>,

    /// Text color (ANSI color code)
    pub text_color: u8,

    /// Quote color (ANSI color code)
    pub quote_color: u8,

    /// Tear line color (ANSI color code)
    pub tear_color: u8,

    /// Origin line color (ANSI color code)
    pub origin_color: u8,

    /// FidoNet zone number
    pub zone: i16,

    /// FidoNet net number
    pub net: i16,

    /// FidoNet node number
    pub node: i16,

    /// FidoNet point number
    pub point: i16,

    /// Conference number
    pub conf: u8,

    /// Reserved bytes
    pub res: [u8; 2],
}

impl Default for BoardRec {
    fn default() -> Self {
        Self {
            name: PascalString::default(),
            filename: PascalString::default(),
            lastmsgid: 0,
            msgpath: PascalString::default(),
            acs: PascalString::default(),
            postacs: PascalString::default(),
            mciacs: PascalString::default(),
            maxmsgs: 0,
            anonymous: AnonymousType::NotAllowed,
            password: PascalString::default(),
            mbstat: MessageBoardFlags::default(),
            permindx: 0,
            mbtype: 0,
            origin: PascalString::default(),
            text_color: 7,    // Default white
            quote_color: 3,   // Default cyan
            tear_color: 8,    // Default dark gray
            origin_color: 14, // Default yellow
            zone: 0,
            net: 0,
            node: 0,
            point: 0,
            conf: 0,
            res: [0, 0],
        }
    }
}

impl BoardRec {
    /// Board type: Local message board
    pub const TYPE_LOCAL: i16 = 0;
    /// Board type: EchoMail (networked)
    pub const TYPE_ECHO: i16 = 1;
    /// Board type: NetMail (private networked)
    pub const TYPE_NETMAIL: i16 = 2;

    /// Get board name
    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    /// Set board name
    pub fn set_name(&mut self, name: impl AsRef<str>) {
        self.name = PascalString::from_string(name);
    }

    /// Get filename (without extension)
    pub fn get_filename(&self) -> String {
        self.filename.to_string()
    }

    /// Set filename
    pub fn set_filename(&mut self, filename: impl AsRef<str>) {
        self.filename = PascalString::from_string(filename);
    }

    /// Check if board is local (not networked)
    pub fn is_local(&self) -> bool {
        self.mbtype == Self::TYPE_LOCAL
    }

    /// Check if board is EchoMail
    pub fn is_echo(&self) -> bool {
        self.mbtype == Self::TYPE_ECHO
    }

    /// Check if board is NetMail
    pub fn is_netmail(&self) -> bool {
        self.mbtype == Self::TYPE_NETMAIL
    }

    /// Check if board is networked (Echo or NetMail)
    pub fn is_networked(&self) -> bool {
        self.is_echo() || self.is_netmail()
    }

    /// Check if board allows anonymous posting
    pub fn allows_anonymous(&self) -> bool {
        !matches!(self.anonymous, AnonymousType::NotAllowed)
    }

    /// Check if board forces anonymous posting
    pub fn forces_anonymous(&self) -> bool {
        matches!(self.anonymous, AnonymousType::Forced)
    }

    /// Check if board has password
    pub fn has_password(&self) -> bool {
        !self.password.is_empty()
    }

    /// Verify board password
    pub fn verify_password(&self, password: &str) -> bool {
        self.password.to_string().eq_ignore_ascii_case(password)
    }

    /// Check if board has message limit
    pub fn has_message_limit(&self) -> bool {
        self.maxmsgs > 0
    }

    /// Get FidoNet address as string (zone:net/node.point)
    pub fn fido_address(&self) -> String {
        if self.point > 0 {
            format!("{}:{}/{}.{}", self.zone, self.net, self.node, self.point)
        } else {
            format!("{}:{}/{}", self.zone, self.net, self.node)
        }
    }

    /// Check if board is a file board (repurposed message base)
    pub fn is_file_board(&self) -> bool {
        self.mbstat.contains(MessageBoardFlags::FILE_BOARD)
    }

    /// Check if board auto-validates posts
    pub fn auto_validates(&self) -> bool {
        self.mbstat.contains(MessageBoardFlags::AUTO)
    }

    /// Check if board is read-only for non-sysops
    pub fn is_read_only(&self) -> bool {
        self.mbstat.contains(MessageBoardFlags::NO_USER)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpackdatetime_new() {
        let dt = CPackDateTime::new(25, 3, 15, 14, 30, 45);
        assert_eq!(dt.year(), 25);
        assert_eq!(dt.month(), 3);
        assert_eq!(dt.day(), 15);
        assert_eq!(dt.hour(), 14);
        assert_eq!(dt.minute(), 30);
        assert_eq!(dt.second(), 45);
    }

    #[test]
    fn test_cpackdatetime_setters() {
        let mut dt = CPackDateTime::default();
        dt.set_year(24);
        dt.set_month(12);
        dt.set_day(25);
        dt.set_hour(23);
        dt.set_minute(59);
        dt.set_second(59);

        assert_eq!(dt.year(), 24);
        assert_eq!(dt.month(), 12);
        assert_eq!(dt.day(), 25);
        assert_eq!(dt.hour(), 23);
        assert_eq!(dt.minute(), 59);
        assert_eq!(dt.second(), 59);
    }

    #[test]
    fn test_cpackdatetime_validation() {
        let valid = CPackDateTime::new(25, 3, 15, 14, 30, 45);
        assert!(valid.is_valid());

        let invalid_month = CPackDateTime::new(25, 0, 15, 14, 30, 45);
        assert!(!invalid_month.is_valid());

        let invalid_day = CPackDateTime::new(25, 3, 0, 14, 30, 45);
        assert!(!invalid_day.is_valid());

        let invalid_hour = CPackDateTime::new(25, 3, 15, 24, 30, 45);
        assert!(!invalid_hour.is_valid());
    }

    #[test]
    fn test_msgindexrec_default() {
        let msg = MsgIndexRec::default();
        assert_eq!(msg.messagenum, 0);
        assert_eq!(msg.msgid, 0);
        assert!(!msg.is_reply());
        assert!(!msg.has_replies());
    }

    #[test]
    fn test_msgindexrec_reply_detection() {
        let mut msg = MsgIndexRec::default();
        msg.isreplytoid = 42;
        msg.isreplyto = 10;
        assert!(msg.is_reply());

        msg.numreplys = 3;
        assert!(msg.has_replies());
    }

    #[test]
    fn test_msgindexrec_day_of_week() {
        let mut msg = MsgIndexRec::default();
        msg.msgdowk = 0;
        assert_eq!(msg.day_of_week_string(), "Sunday");

        msg.msgdowk = 3;
        assert_eq!(msg.day_of_week_string(), "Wednesday");

        msg.msgdowk = 6;
        assert_eq!(msg.day_of_week_string(), "Saturday");

        msg.msgdowk = 99;
        assert_eq!(msg.day_of_week_string(), "Unknown");
    }

    #[test]
    fn test_fromtoinfo_display_name() {
        let info = FromToInfo::from_user(1, "John Doe", "JDoe");
        assert_eq!(info.display_name(), "JDoe");

        let anon = FromToInfo::anonymous(AnonymousType::Allowed);
        assert_eq!(anon.display_name(), "Anonymous");
        assert!(anon.is_anonymous());

        let dear_abby = FromToInfo::anonymous(AnonymousType::DearAbby);
        assert_eq!(dear_abby.display_name(), "Dear Abby");
    }

    #[test]
    fn test_mheaderrec_signature() {
        let header = MHeaderRec::default();
        assert!(header.is_valid_signature());
        assert_eq!(header.signature, MHeaderRec::SIGNATURE);
    }

    #[test]
    fn test_mheaderrec_new() {
        let header = MHeaderRec::new(1, "Alice", "A1ice", 2, "Bob", "B0b", "Test Message");

        assert!(header.is_valid_signature());
        assert_eq!(header.get_title(), "Test Message");
        assert_eq!(header.from_name(), "A1ice");
        assert_eq!(header.to_name(), "B0b");
        assert!(!header.is_anonymous());
        assert!(!header.is_echoed());
    }

    #[test]
    fn test_mheaderrec_anonymous() {
        let mut header = MHeaderRec::default();
        header.fromi = FromToInfo::anonymous(AnonymousType::Forced);
        assert!(header.is_anonymous());
    }

    #[test]
    fn test_boardrec_types() {
        let mut board = BoardRec::default();
        assert!(board.is_local());
        assert!(!board.is_networked());

        board.mbtype = BoardRec::TYPE_ECHO;
        assert!(board.is_echo());
        assert!(board.is_networked());

        board.mbtype = BoardRec::TYPE_NETMAIL;
        assert!(board.is_netmail());
        assert!(board.is_networked());
    }

    #[test]
    fn test_boardrec_anonymous() {
        let mut board = BoardRec::default();
        assert!(!board.allows_anonymous());
        assert!(!board.forces_anonymous());

        board.anonymous = AnonymousType::Allowed;
        assert!(board.allows_anonymous());
        assert!(!board.forces_anonymous());

        board.anonymous = AnonymousType::Forced;
        assert!(board.allows_anonymous());
        assert!(board.forces_anonymous());
    }

    #[test]
    fn test_boardrec_password() {
        let mut board = BoardRec::default();
        assert!(!board.has_password());

        board.password = PascalString::from_string("secret");
        assert!(board.has_password());
        assert!(board.verify_password("secret"));
        assert!(board.verify_password("SECRET")); // Case-insensitive
        assert!(!board.verify_password("wrong"));
    }

    #[test]
    fn test_boardrec_fido_address() {
        let mut board = BoardRec::default();
        board.zone = 1;
        board.net = 234;
        board.node = 567;
        assert_eq!(board.fido_address(), "1:234/567");

        board.point = 89;
        assert_eq!(board.fido_address(), "1:234/567.89");
    }

    #[test]
    fn test_boardrec_message_limit() {
        let mut board = BoardRec::default();
        assert!(!board.has_message_limit());

        board.maxmsgs = 1000;
        assert!(board.has_message_limit());
    }

    #[test]
    fn test_boardrec_flags() {
        let mut board = BoardRec::default();
        assert!(!board.is_file_board());
        assert!(!board.auto_validates());
        assert!(!board.is_read_only());

        board.mbstat.insert(MessageBoardFlags::FILE_BOARD);
        assert!(board.is_file_board());

        board.mbstat.insert(MessageBoardFlags::AUTO);
        assert!(board.auto_validates());

        board.mbstat.insert(MessageBoardFlags::NO_USER);
        assert!(board.is_read_only());
    }

    #[test]
    fn test_boardrec_name_operations() {
        let mut board = BoardRec::default();
        board.set_name("General Discussion");
        assert_eq!(board.get_name(), "General Discussion");

        board.set_filename("GENERAL");
        assert_eq!(board.get_filename(), "GENERAL");
    }
}
