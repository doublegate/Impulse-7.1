//! Pascal-compatible system configuration record (SYSTAT.DAT format)
//!
//! This module provides the system configuration structure that matches the binary
//! layout of the original Impulse 7.1 BBS SYSTAT.DAT file.

use binrw::binrw;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

use crate::pascal_types::ArFlags;
use crate::pascal_user::PascalString;
use crate::user_flags::UserFlags;

/// System log record (ZLOG.DAT format)
///
/// Original Pascal definition (RECORDS.PAS lines 278-285):
/// ```pascal
/// zlogrec=
/// record
///   date:string[8];
///   userbaud:array[0..4] of integer;
///   active,calls,newusers,pubpost,privpost,fback,criterr:integer;
///   uploads,downloads:integer;
///   uk,dk:longint;
/// end;
/// ```
#[binrw]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZLogRec {
    /// Date of log entry (MM/DD/YY format)
    pub date: PascalString<8>,

    /// User counts by baud rate (300, 1200, 2400, 9600, 14400)
    pub userbaud: [i16; 5],

    /// Active minutes today
    pub active: i16,

    /// Number of calls today
    pub calls: i16,

    /// New users today
    pub newusers: i16,

    /// Public posts today
    pub pubpost: i16,

    /// Private posts today
    pub privpost: i16,

    /// Feedback messages today
    pub fback: i16,

    /// Critical errors today
    pub criterr: i16,

    /// Files uploaded today
    pub uploads: i16,

    /// Files downloaded today
    pub downloads: i16,

    /// Kilobytes uploaded today
    pub uk: i32,

    /// Kilobytes downloaded today
    pub dk: i32,
}

impl Default for ZLogRec {
    fn default() -> Self {
        ZLogRec {
            date: PascalString::from_string(""),
            userbaud: [0; 5],
            active: 0,
            calls: 0,
            newusers: 0,
            pubpost: 0,
            privpost: 0,
            fback: 0,
            criterr: 0,
            uploads: 0,
            downloads: 0,
            uk: 0,
            dk: 0,
        }
    }
}

/// Security range type (array of 256 integers for security level settings)
///
/// Original Pascal definition (RECORDS.PAS line 72):
/// ```pascal
/// secrange = array[0..255] of integer;
/// ```
#[binrw]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecRange {
    /// Array of 256 integers (one per security level)
    #[serde(with = "BigArray")]
    pub values: [i16; 256],
}

impl Default for SecRange {
    fn default() -> Self {
        SecRange { values: [0; 256] }
    }
}

impl SecRange {
    /// Get value for security level
    pub fn get(&self, level: u8) -> i16 {
        self.values[level as usize]
    }

    /// Set value for security level
    pub fn set(&mut self, level: u8, value: i16) {
        self.values[level as usize] = value;
    }
}

/// System configuration record (SYSTAT.DAT format)
///
/// Original Pascal definition (RECORDS.PAS lines 435-588):
/// This is the main system configuration structure containing all BBS settings,
/// paths, security levels, ratios, and system state.
#[binrw]
#[derive(Debug, Clone)]
pub struct PascalSystatRec {
    /// GFILES path (general files)
    pub gfilepath: PascalString<79>,

    /// AFILES path (text files)
    pub afilepath: PascalString<79>,

    /// MENUS path
    pub menupath: PascalString<79>,

    /// LOG path (traps, chats, SysOp logs)
    pub trappath: PascalString<79>,

    /// MSG path (private/public mail)
    pub msgpath: PascalString<79>,

    /// ISL script path
    pub tfilepath: PascalString<79>,

    /// TEMP path - temporary directory
    pub temppath: PascalString<79>,

    /// Is new user voting active?
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub usenuv: bool,

    /// Yes votes required for validation in NUV
    pub nuvyes: u8,

    /// No votes required for validation in NUV
    pub nuvno: u8,

    /// BBS name
    pub bbsname: PascalString<80>,

    /// BBS phone number
    pub bbsphone: PascalString<12>,

    /// SysOp's full name or alias
    pub sysopname: PascalString<30>,

    /// BBS registration number
    pub regnum: PascalString<10>,

    /// Batch file to run instead of NScan
    pub startdir: PascalString<80>,

    /// Max time bank minutes
    pub max_time_bank: i16,

    /// SysOp begin minute (in minutes from midnight)
    pub lowtime: i16,

    /// SysOp end time
    pub hitime: i16,

    /// Normal downloading hours begin
    pub dllowtime: i16,

    /// Normal downloading hours end
    pub dlhitime: i16,

    /// Which editor to use (FSE/Line)
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub msgedit: bool,

    /// Is Shuttle Logon active?
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub shuttlelog: bool,

    /// Lock out 300 baud?
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub lock300: bool,

    /// SysOp password
    pub sysoppw: PascalString<20>,

    /// New user password (or NULL if none)
    pub newuserpw: PascalString<20>,

    /// Shuttle password (if Shuttle active)
    pub shuttlepw: PascalString<20>,

    /// 300 baud calling hours begin
    pub b300lowtime: i16,

    /// 300 baud calling hours end
    pub b300hitime: i16,

    /// 300 baud downloading hours begin
    pub b300dllowtime: i16,

    /// 300 baud downloading hours end
    pub b300dlhitime: i16,

    /// Don't allow new users?
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub closedsystem: bool,

    /// Is swap shell function enabled?
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub swapshell: bool,

    /// Time before event warning (minutes)
    pub eventwarningtime: i16,

    /// Last date text-files were inserted
    pub tfiledate: PascalString<8>,

    /// Last-used message ID (sequential)
    pub lastmsgid: i32,

    /// SysOp access requirement
    pub sop: PascalString<20>,

    /// Co-SysOp access requirement
    pub csop: PascalString<20>,

    /// Message SysOp access requirement
    pub msop: PascalString<20>,

    /// File SysOp access requirement
    pub fsop: PascalString<20>,

    /// SysOp PW at logon access requirement
    pub spw: PascalString<20>,

    /// See SysOp PWs remotely access requirement
    pub seepw: PascalString<20>,

    /// Make normal public posts access requirement
    pub normpubpost: PascalString<20>,

    /// Send normal e-mail access requirement
    pub normprivpost: PascalString<20>,

    /// See who posted public anon access requirement
    pub anonpubread: PascalString<20>,

    /// See who sent anon e-mail access requirement
    pub anonprivread: PascalString<20>,

    /// Make anon posts access requirement
    pub anonpubpost: PascalString<20>,

    /// Send anon e-mail access requirement
    pub anonprivpost: PascalString<20>,

    /// See unvalidated files access requirement
    pub seeunval: PascalString<20>,

    /// DL unvalidated files access requirement
    pub dlunval: PascalString<20>,

    /// No UL/DL ratio access requirement
    pub nodlratio: PascalString<20>,

    /// No post/call ratio access requirement
    pub nopostratio: PascalString<20>,

    /// No file points checking access requirement
    pub nofilepts: PascalString<20>,

    /// Uploads require validation by SysOp access requirement
    pub ulvalreq: PascalString<20>,

    /// Max e-mail can send per call
    pub maxprivpost: u8,

    /// Max feedback per call
    pub maxfback: u8,

    /// Max posts per call
    pub maxpubpost: u8,

    /// Max chat-pages per call
    pub maxchat: u8,

    /// Max mail in mail-box
    pub maxwaiting: u8,

    /// Max mail in mail-box for Co-SysOp +
    pub csmaxwaiting: u8,

    /// Tries allowed for PWs at logon
    pub maxlogontries: u8,

    /// Not used (reserved)
    pub not_used: u8,

    /// Max lines in message
    pub maxlines: u8,

    /// Max lines in message for Co-SysOp +
    pub csmaxlines: u8,

    /// SysOp color in chat mode
    pub sysopcolor: u8,

    /// User color in chat mode
    pub usercolor: u8,

    /// Minimum K drive space left to post
    pub minspaceforpost: i16,

    /// Minimum K drive space left to upload
    pub minspaceforupload: i16,

    /// Days to keep SYSOP##.LOG
    pub backsysoplogs: u8,

    /// Minutes after which to blank WFC menu
    pub wfcblanktime: u8,

    /// Default video line length
    pub linelen: u8,

    /// Default video page length
    pub pagelen: u8,

    /// Use iCEcolor?
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub specialfx: bool,

    /// Make use of FOSSIL comm driver
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub fossil: bool,

    /// Allow aliases? (handles)
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub allowalias: bool,

    /// Use phone number password in logon?
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub phonepw: bool,

    /// Is local security ON?
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub localsec: bool,

    /// Is local screen-security ON?
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub localscreensec: bool,

    /// Trap ALL USER'S activity?
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub globaltrap: bool,

    /// Does chat buffer auto-open?
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub autochatopen: bool,

    /// Auto-Message in logon?
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub autominlogon: bool,

    /// Bulletins in logon?
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub bullinlogon: bool,

    /// Last Callers list in logon?
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub lcallinlogon: bool,

    /// User Stats in logon?
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub yourinfoinlogon: bool,

    /// Is BBS multitasking?
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub multitask: bool,

    /// Take phone off-hook for local logons?
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub offhooklocallogon: bool,

    /// Is mandatory logon voting active?
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub forcevoting: bool,

    /// "Compress" file/message base numbers?
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub compressbases: bool,

    /// Search for dup. filenames when UL?
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub searchdup: bool,

    /// SysOp log type: File/Printer/Both
    pub slogtype: u8,

    /// Strip colors from SysOp log output?
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub stripclog: bool,

    /// User# to send new user application to
    pub newapp: u8,

    /// Guest user number (borrowed f2 status bar var)
    pub guestuser: u8,

    /// Minutes before time-out bell
    pub timeoutbell: u8,

    /// Minutes before timeout (logoff)
    pub timeout: u8,

    /// Whether to use WFC menu logo
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub usewfclogo: bool,

    /// Attempt to use EMS for overlay
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub useems: bool,

    /// Use ROM BIOS for local video output
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub usebios: bool,

    /// Suppress snow on CGA systems
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub use_scroll_back: bool,

    /// BBS comments for archives (3 lines)
    pub filearccomment: [PascalString<80>; 3],

    /// Use Internal Upload Checker?
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub ulcheckit: bool,

    /// Are UL/DL ratios active?
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub uldlratio: bool,

    /// Is auto file-pt compensation active?
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub fileptratio: bool,

    /// File point compensation ratio
    pub fileptcomp: u8,

    /// File point "base compensation size"
    pub fileptcompbasesize: u8,

    /// Percent time refund for ULs
    pub ulrefund: u8,

    /// "To SysOp" file base
    pub tosysopdir: u8,

    /// Validate ALL FILES automatically?
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub validateallfiles: bool,

    /// Remote output device (GATEx, COMx, etc)
    pub remdevice: PascalString<10>,

    /// Max K allowed in TEMP\3\
    pub maxintemp: i16,

    /// Min K to allow resume-later
    pub minresume: i16,

    /// Max files in DL batch queue
    pub maxdbatch: u8,

    /// Max files in UL batch queue
    pub maxubatch: u8,

    /// New user SL
    pub newsl: u8,

    /// New user DSL
    pub newdsl: u8,

    /// New user AR (access requirements)
    #[br(map = |n: u32| ArFlags::from_pascal_u32(n))]
    #[bw(map = |flags: &ArFlags| flags.to_pascal_u32())]
    pub newar: ArFlags,

    /// New user AC (access flags)
    #[br(map = |bytes: [u8; 3]| UserFlags::from_pascal_bytes(&bytes))]
    #[bw(map = |flags: &UserFlags| flags.to_pascal_bytes())]
    pub newac: UserFlags,

    /// New user file points
    pub newfp: i16,

    /// Auto-validation SL
    pub autosl: u8,

    /// Auto-validation DSL
    pub autodsl: u8,

    /// Auto-validation AR
    #[br(map = |n: u32| ArFlags::from_pascal_u32(n))]
    #[bw(map = |flags: &ArFlags| flags.to_pascal_u32())]
    pub autoar: ArFlags,

    /// Auto-validation AC
    #[br(map = |bytes: [u8; 3]| UserFlags::from_pascal_bytes(&bytes))]
    #[bw(map = |flags: &UserFlags| flags.to_pascal_bytes())]
    pub autoac: UserFlags,

    /// Logon menu to start ALL users on
    pub allstartmenu: PascalString<8>,

    /// Default bulletins filename prefix
    pub bulletprefix: PascalString<8>,

    /// Time allowance by security level
    pub timeallow: SecRange,

    /// Call allowance by security level
    pub callallow: SecRange,

    /// # ULs/# DLs ratios by security level
    pub dlratio: SecRange,

    /// DLk/ULk ratios by security level
    pub dlkratio: SecRange,

    /// Post/call ratios by security level
    pub postratio: SecRange,

    /// Last system date
    pub lastdate: PascalString<8>,

    /// Type of SysOp window currently in use
    pub curwindow: u8,

    /// Is SysOp window on top of screen?
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub istopwindow: bool,

    /// Total number of callers
    pub callernum: i32,

    /// Number of users
    pub numusers: i16,

    /// TODAY's ZLOG record
    pub todayzlog: ZLogRec,

    /// File points/upload credit compensation for posts
    pub postcredits: i16,

    /// Display "ansi missing" messages?
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub displayansimsg: bool,

    /// Disable/enable screen pauses
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub scrpause: bool,

    /// Window on flag
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |&b| if b { 1u8 } else { 0u8 })]
    pub windowon: bool,

    /// Swap shell path
    pub swappath: PascalString<79>,
}

impl Default for PascalSystatRec {
    fn default() -> Self {
        PascalSystatRec {
            gfilepath: PascalString::from_string(""),
            afilepath: PascalString::from_string(""),
            menupath: PascalString::from_string(""),
            trappath: PascalString::from_string(""),
            msgpath: PascalString::from_string(""),
            tfilepath: PascalString::from_string(""),
            temppath: PascalString::from_string(""),
            usenuv: false,
            nuvyes: 0,
            nuvno: 0,
            bbsname: PascalString::from_string(""),
            bbsphone: PascalString::from_string(""),
            sysopname: PascalString::from_string(""),
            regnum: PascalString::from_string(""),
            startdir: PascalString::from_string(""),
            max_time_bank: 0,
            lowtime: 0,
            hitime: 0,
            dllowtime: 0,
            dlhitime: 0,
            msgedit: false,
            shuttlelog: false,
            lock300: false,
            sysoppw: PascalString::from_string(""),
            newuserpw: PascalString::from_string(""),
            shuttlepw: PascalString::from_string(""),
            b300lowtime: 0,
            b300hitime: 0,
            b300dllowtime: 0,
            b300dlhitime: 0,
            closedsystem: false,
            swapshell: false,
            eventwarningtime: 0,
            tfiledate: PascalString::from_string(""),
            lastmsgid: 0,
            sop: PascalString::from_string(""),
            csop: PascalString::from_string(""),
            msop: PascalString::from_string(""),
            fsop: PascalString::from_string(""),
            spw: PascalString::from_string(""),
            seepw: PascalString::from_string(""),
            normpubpost: PascalString::from_string(""),
            normprivpost: PascalString::from_string(""),
            anonpubread: PascalString::from_string(""),
            anonprivread: PascalString::from_string(""),
            anonpubpost: PascalString::from_string(""),
            anonprivpost: PascalString::from_string(""),
            seeunval: PascalString::from_string(""),
            dlunval: PascalString::from_string(""),
            nodlratio: PascalString::from_string(""),
            nopostratio: PascalString::from_string(""),
            nofilepts: PascalString::from_string(""),
            ulvalreq: PascalString::from_string(""),
            maxprivpost: 0,
            maxfback: 0,
            maxpubpost: 0,
            maxchat: 0,
            maxwaiting: 0,
            csmaxwaiting: 0,
            maxlogontries: 0,
            not_used: 0,
            maxlines: 0,
            csmaxlines: 0,
            sysopcolor: 0,
            usercolor: 0,
            minspaceforpost: 0,
            minspaceforupload: 0,
            backsysoplogs: 0,
            wfcblanktime: 0,
            linelen: 80,
            pagelen: 25,
            specialfx: false,
            fossil: false,
            allowalias: false,
            phonepw: false,
            localsec: false,
            localscreensec: false,
            globaltrap: false,
            autochatopen: false,
            autominlogon: false,
            bullinlogon: false,
            lcallinlogon: false,
            yourinfoinlogon: false,
            multitask: false,
            offhooklocallogon: false,
            forcevoting: false,
            compressbases: false,
            searchdup: false,
            slogtype: 0,
            stripclog: false,
            newapp: 0,
            guestuser: 0,
            timeoutbell: 0,
            timeout: 0,
            usewfclogo: false,
            useems: false,
            usebios: false,
            use_scroll_back: false,
            filearccomment: [
                PascalString::from_string(""),
                PascalString::from_string(""),
                PascalString::from_string(""),
            ],
            ulcheckit: false,
            uldlratio: false,
            fileptratio: false,
            fileptcomp: 0,
            fileptcompbasesize: 0,
            ulrefund: 0,
            tosysopdir: 0,
            validateallfiles: false,
            remdevice: PascalString::from_string(""),
            maxintemp: 0,
            minresume: 0,
            maxdbatch: 0,
            maxubatch: 0,
            newsl: 0,
            newdsl: 0,
            newar: ArFlags::empty(),
            newac: UserFlags::empty(),
            newfp: 0,
            autosl: 0,
            autodsl: 0,
            autoar: ArFlags::empty(),
            autoac: UserFlags::empty(),
            allstartmenu: PascalString::from_string(""),
            bulletprefix: PascalString::from_string(""),
            timeallow: SecRange::default(),
            callallow: SecRange::default(),
            dlratio: SecRange::default(),
            dlkratio: SecRange::default(),
            postratio: SecRange::default(),
            lastdate: PascalString::from_string(""),
            curwindow: 0,
            istopwindow: false,
            callernum: 0,
            numusers: 0,
            todayzlog: ZLogRec::default(),
            postcredits: 0,
            displayansimsg: false,
            scrpause: false,
            windowon: false,
            swappath: PascalString::from_string(""),
        }
    }
}

impl PascalSystatRec {
    /// Create a new system configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the BBS name
    pub fn get_bbs_name(&self) -> String {
        self.bbsname.to_string()
    }

    /// Set the BBS name
    pub fn set_bbs_name(&mut self, name: impl AsRef<str>) {
        self.bbsname = PascalString::from_string(name);
    }

    /// Get the SysOp name
    pub fn get_sysop_name(&self) -> String {
        self.sysopname.to_string()
    }

    /// Set the SysOp name
    pub fn set_sysop_name(&mut self, name: impl AsRef<str>) {
        self.sysopname = PascalString::from_string(name);
    }

    /// Check if the system is closed to new users
    pub fn is_closed(&self) -> bool {
        self.closedsystem
    }

    /// Check if UL/DL ratios are enforced
    pub fn enforces_ul_dl_ratio(&self) -> bool {
        self.uldlratio
    }

    /// Get time allowance for a security level
    pub fn get_time_allowance(&self, level: u8) -> i16 {
        self.timeallow.get(level)
    }

    /// Get call allowance for a security level
    pub fn get_call_allowance(&self, level: u8) -> i16 {
        self.callallow.get(level)
    }

    /// Get DL ratio for a security level
    pub fn get_dl_ratio(&self, level: u8) -> i16 {
        self.dlratio.get(level)
    }

    /// Get post ratio for a security level
    pub fn get_post_ratio(&self, level: u8) -> i16 {
        self.postratio.get(level)
    }

    /// Check if during SysOp hours
    pub fn is_sysop_hours(&self, current_minute: i16) -> bool {
        if self.lowtime <= self.hitime {
            current_minute >= self.lowtime && current_minute <= self.hitime
        } else {
            // Wraps around midnight
            current_minute >= self.lowtime || current_minute <= self.hitime
        }
    }

    /// Check if during download hours
    pub fn is_download_hours(&self, current_minute: i16) -> bool {
        if self.dllowtime <= self.dlhitime {
            current_minute >= self.dllowtime && current_minute <= self.dlhitime
        } else {
            // Wraps around midnight
            current_minute >= self.dllowtime || current_minute <= self.dlhitime
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zlogrec_default() {
        let log = ZLogRec::default();
        assert_eq!(log.calls, 0);
        assert_eq!(log.newusers, 0);
        assert_eq!(log.uploads, 0);
        assert_eq!(log.downloads, 0);
    }

    #[test]
    fn test_secrange_default() {
        let range = SecRange::default();
        assert_eq!(range.get(0), 0);
        assert_eq!(range.get(255), 0);
    }

    #[test]
    fn test_secrange_get_set() {
        let mut range = SecRange::default();
        range.set(10, 42);
        assert_eq!(range.get(10), 42);
        assert_eq!(range.get(11), 0);
    }

    #[test]
    fn test_pascal_systatrec_default() {
        let config = PascalSystatRec::default();
        assert_eq!(config.get_bbs_name(), "");
        assert_eq!(config.linelen, 80);
        assert_eq!(config.pagelen, 25);
        assert!(!config.is_closed());
    }

    #[test]
    fn test_pascal_systatrec_bbs_name() {
        let mut config = PascalSystatRec::default();
        config.set_bbs_name("My BBS");
        assert_eq!(config.get_bbs_name(), "My BBS");
    }

    #[test]
    fn test_pascal_systatrec_sysop_name() {
        let mut config = PascalSystatRec::default();
        config.set_sysop_name("John Doe");
        assert_eq!(config.get_sysop_name(), "John Doe");
    }

    #[test]
    fn test_pascal_systatrec_closed_system() {
        let mut config = PascalSystatRec::default();
        assert!(!config.is_closed());
        config.closedsystem = true;
        assert!(config.is_closed());
    }

    #[test]
    fn test_pascal_systatrec_ul_dl_ratio() {
        let mut config = PascalSystatRec::default();
        assert!(!config.enforces_ul_dl_ratio());
        config.uldlratio = true;
        assert!(config.enforces_ul_dl_ratio());
    }

    #[test]
    fn test_pascal_systatrec_time_allowance() {
        let mut config = PascalSystatRec::default();
        config.timeallow.set(10, 60);
        assert_eq!(config.get_time_allowance(10), 60);
    }

    #[test]
    fn test_pascal_systatrec_sysop_hours() {
        let mut config = PascalSystatRec::default();
        config.lowtime = 120; // 2:00 AM
        config.hitime = 240; // 4:00 AM

        assert!(!config.is_sysop_hours(60)); // 1:00 AM - before
        assert!(config.is_sysop_hours(180)); // 3:00 AM - during
        assert!(!config.is_sysop_hours(300)); // 5:00 AM - after
    }

    #[test]
    fn test_pascal_systatrec_sysop_hours_wraparound() {
        let mut config = PascalSystatRec::default();
        config.lowtime = 1380; // 11:00 PM
        config.hitime = 60; // 1:00 AM

        assert!(config.is_sysop_hours(1400)); // 11:20 PM - during
        assert!(config.is_sysop_hours(30)); // 12:30 AM - during
        assert!(!config.is_sysop_hours(120)); // 2:00 AM - after
    }

    #[test]
    fn test_pascal_systatrec_download_hours() {
        let mut config = PascalSystatRec::default();
        config.dllowtime = 360; // 6:00 AM
        config.dlhitime = 1320; // 10:00 PM

        assert!(!config.is_download_hours(300)); // 5:00 AM - before
        assert!(config.is_download_hours(720)); // 12:00 PM - during
        assert!(!config.is_download_hours(1380)); // 11:00 PM - after
    }

    #[test]
    fn test_pascal_systatrec_new_user_settings() {
        let mut config = PascalSystatRec::default();
        config.newsl = 10;
        config.newdsl = 5;
        config.newfp = 100;

        assert_eq!(config.newsl, 10);
        assert_eq!(config.newdsl, 5);
        assert_eq!(config.newfp, 100);
    }
}
