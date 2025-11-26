//! DOOR.SYS dropfile format implementation.
//!
//! DOOR.SYS is a 52-line standard dropfile format used by many BBS door games.
//! Each line contains specific information about the user and system configuration.

use crate::error::{DoorError, Result};
use crate::session::DoorSession;
use std::fs;
use std::path::Path;

/// DOOR.SYS dropfile format - 52 line standard format.
///
/// This structure represents the complete DOOR.SYS dropfile specification,
/// which is used by most BBS door games to receive user and system information.
#[derive(Debug, Clone)]
pub struct DoorSysDropfile {
    /// Line 1: COM port (COM1, COM2, COM0 for local)
    pub com_port: String,
    /// Line 2: Baud rate (300-115200)
    pub baud_rate: u32,
    /// Line 3: Parity (7 or 8)
    pub parity: u8,
    /// Line 4: Node number (1-999)
    pub node_number: u16,
    /// Line 5: DTR drop time in seconds (0 = disabled)
    pub dtr_drop_time: u16,
    /// Line 6: Screen display (Y/N)
    pub screen_display: bool,
    /// Line 7: Printer toggle (Y/N)
    pub printer_toggle: bool,
    /// Line 8: Page bell (Y/N)
    pub page_bell: bool,
    /// Line 9: Caller alarm (Y/N)
    pub caller_alarm: bool,
    /// Line 10: User's full name
    pub user_name: String,
    /// Line 11: Location (City, State)
    pub location: String,
    /// Line 12: Home phone number
    pub phone_home: String,
    /// Line 13: Work/data phone number
    pub phone_work: String,
    /// Line 14: Password (masked)
    pub password: String,
    /// Line 15: Security level (0-255)
    pub security_level: u8,
    /// Line 16: Total times called
    pub total_calls: u32,
    /// Line 17: Last call date (MM/DD/YY)
    pub last_call_date: String,
    /// Line 18: Seconds remaining this session
    pub seconds_remaining: u32,
    /// Line 19: Minutes remaining this session
    pub minutes_remaining: u32,
    /// Line 20: Graphics mode (GR=Graphics, NG=No Graphics, 7E=7-bit)
    pub graphics_mode: String,
    /// Line 21: Page length (lines per page)
    pub page_length: u16,
    /// Line 22: Expert mode (Y/N)
    pub expert_mode: bool,
    /// Line 23: Conference areas (1-5)
    pub conferences: String,
    /// Line 24: Conference area the user was in
    pub current_conference: u8,
    /// Line 25: Expiration date (MM/DD/YY)
    pub expiration_date: String,
    /// Line 26: User record number
    pub user_record_number: u32,
    /// Line 27: Default protocol (X=Xmodem, Y=Ymodem, Z=Zmodem, etc.)
    pub default_protocol: char,
    /// Line 28: Total uploads
    pub total_uploads: u32,
    /// Line 29: Total downloads
    pub total_downloads: u32,
    /// Line 30: Daily download limit (KB)
    pub daily_download_kb: u32,
    /// Line 31: Daily download KB so far
    pub daily_downloaded_kb: u32,
    /// Line 32: Birthdate (MM/DD/YY)
    pub birthdate: String,
    /// Line 33: Path to the user's directory
    pub user_directory: String,
    /// Line 34: Path to the BBS directory
    pub bbs_directory: String,
    /// Line 35: Sysop (Y/N)
    pub is_sysop: bool,
    /// Line 36: Co-Sysop (Y/N)
    pub is_co_sysop: bool,
    /// Line 37: ANSI support (Y/N)
    pub ansi_enabled: bool,
    /// Line 38: Use full screen editor (Y/N)
    pub use_full_editor: bool,
    /// Line 39: Screen clearing codes (Y/N)
    pub screen_clearing: bool,
    /// Line 40: Mail waiting (Y/N)
    pub mail_waiting: bool,
    /// Line 41: Current conference area name
    pub conference_name: String,
    /// Line 42: Screen width (characters)
    pub screen_width: u16,
    /// Line 43: BBS software name
    pub bbs_software: String,
    /// Line 44: User's alias (if any)
    pub user_alias: String,
    /// Line 45: Time of login (HH:MM)
    pub login_time: String,
    /// Line 46: Time limit for this session (minutes)
    pub time_limit: u32,
    /// Line 47: Upload KB limit
    pub upload_kb_limit: u32,
    /// Line 48: Download KB limit
    pub download_kb_limit: u32,
    /// Line 49: Upload/download ratio
    pub ul_dl_ratio: String,
    /// Line 50: Upload KB total
    pub upload_kb_total: u64,
    /// Line 51: Download KB total
    pub download_kb_total: u64,
    /// Line 52: User's comment (optional)
    pub user_comment: String,
}

impl DoorSysDropfile {
    /// Create a new DOOR.SYS dropfile with default values.
    pub fn new() -> Self {
        Self {
            com_port: "COM1".to_string(),
            baud_rate: 38400,
            parity: 8,
            node_number: 1,
            dtr_drop_time: 0,
            screen_display: true,
            printer_toggle: false,
            page_bell: true,
            caller_alarm: true,
            user_name: "Guest User".to_string(),
            location: "Unknown, XX".to_string(),
            phone_home: "000-000-0000".to_string(),
            phone_work: "000-000-0000".to_string(),
            password: "XXXX".to_string(),
            security_level: 10,
            total_calls: 1,
            last_call_date: "01/01/25".to_string(),
            seconds_remaining: 3600,
            minutes_remaining: 60,
            graphics_mode: "GR".to_string(),
            page_length: 23,
            expert_mode: false,
            conferences: "1;2;3;4;5".to_string(),
            current_conference: 1,
            expiration_date: "12/31/99".to_string(),
            user_record_number: 1,
            default_protocol: 'Z',
            total_uploads: 0,
            total_downloads: 0,
            daily_download_kb: 10240,
            daily_downloaded_kb: 0,
            birthdate: "01/01/90".to_string(),
            user_directory: "/home/bbs/users".to_string(),
            bbs_directory: "/home/bbs".to_string(),
            is_sysop: false,
            is_co_sysop: false,
            ansi_enabled: true,
            use_full_editor: true,
            screen_clearing: true,
            mail_waiting: false,
            conference_name: "Main".to_string(),
            screen_width: 80,
            bbs_software: "Impulse-Next BBS".to_string(),
            user_alias: String::new(),
            login_time: "00:00".to_string(),
            time_limit: 60,
            upload_kb_limit: 10240,
            download_kb_limit: 10240,
            ul_dl_ratio: "1:10".to_string(),
            upload_kb_total: 0,
            download_kb_total: 0,
            user_comment: String::new(),
        }
    }

    /// Create a DOOR.SYS dropfile from a door session.
    pub fn from_session(session: &DoorSession) -> Self {
        let mut dropfile = Self::new();
        dropfile.node_number = session.node_id;
        dropfile.user_name = session.user_name.clone();
        dropfile.location = session.location.clone();
        dropfile.security_level = session.security_level;
        dropfile.total_calls = session.total_calls;
        dropfile.last_call_date = session.last_call_date.clone();
        dropfile.seconds_remaining = session.time_remaining_seconds;
        dropfile.minutes_remaining = session.time_remaining_seconds / 60;
        dropfile.ansi_enabled = session.ansi_enabled;
        dropfile.upload_kb_total = session.upload_kb;
        dropfile.download_kb_total = session.download_kb;

        if let Some(alias) = &session.user_alias {
            dropfile.user_alias = alias.clone();
        }

        // Format login time
        dropfile.login_time = session.login_time.format("%H:%M").to_string();

        dropfile
    }

    /// Write the DOOR.SYS dropfile to a file.
    pub fn write_to_file(&self, path: &Path) -> Result<()> {
        let content = self.format_dropfile();
        fs::write(path, content).map_err(|e| {
            DoorError::DropfileCreation(format!("Failed to write DOOR.SYS: {}", e))
        })?;
        Ok(())
    }

    /// Format the dropfile as a 52-line string.
    pub fn format_dropfile(&self) -> String {
        format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n\
             {}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n\
             {}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n\
             {}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n\
             {}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n\
             {}\n{}\n",
            self.com_port,
            self.baud_rate,
            self.parity,
            self.node_number,
            self.dtr_drop_time,
            if self.screen_display { "Y" } else { "N" },
            if self.printer_toggle { "Y" } else { "N" },
            if self.page_bell { "Y" } else { "N" },
            if self.caller_alarm { "Y" } else { "N" },
            self.user_name,
            self.location,
            self.phone_home,
            self.phone_work,
            self.password,
            self.security_level,
            self.total_calls,
            self.last_call_date,
            self.seconds_remaining,
            self.minutes_remaining,
            self.graphics_mode,
            self.page_length,
            if self.expert_mode { "Y" } else { "N" },
            self.conferences,
            self.current_conference,
            self.expiration_date,
            self.user_record_number,
            self.default_protocol,
            self.total_uploads,
            self.total_downloads,
            self.daily_download_kb,
            self.daily_downloaded_kb,
            self.birthdate,
            self.user_directory,
            self.bbs_directory,
            if self.is_sysop { "Y" } else { "N" },
            if self.is_co_sysop { "Y" } else { "N" },
            if self.ansi_enabled { "Y" } else { "N" },
            if self.use_full_editor { "Y" } else { "N" },
            if self.screen_clearing { "Y" } else { "N" },
            if self.mail_waiting { "Y" } else { "N" },
            self.conference_name,
            self.screen_width,
            self.bbs_software,
            self.user_alias,
            self.login_time,
            self.time_limit,
            self.upload_kb_limit,
            self.download_kb_limit,
            self.ul_dl_ratio,
            self.upload_kb_total,
            self.download_kb_total,
            self.user_comment
        )
    }
}

impl Default for DoorSysDropfile {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_doorsys_new() {
        let dropfile = DoorSysDropfile::new();
        assert_eq!(dropfile.com_port, "COM1");
        assert_eq!(dropfile.baud_rate, 38400);
        assert_eq!(dropfile.parity, 8);
        assert_eq!(dropfile.node_number, 1);
        assert_eq!(dropfile.user_name, "Guest User");
    }

    #[test]
    fn test_doorsys_default() {
        let dropfile = DoorSysDropfile::default();
        assert_eq!(dropfile.security_level, 10);
        assert_eq!(dropfile.total_calls, 1);
        assert!(dropfile.ansi_enabled);
    }

    #[test]
    fn test_doorsys_from_session() {
        let session = DoorSession {
            node_id: 5,
            user_name: "John Doe".to_string(),
            user_alias: Some("JDoe".to_string()),
            location: "Seattle, WA".to_string(),
            security_level: 100,
            time_remaining_seconds: 1800,
            ansi_enabled: true,
            login_time: Utc::now(),
            total_calls: 50,
            last_call_date: "11/26/25".to_string(),
            upload_kb: 1024,
            download_kb: 2048,
        };

        let dropfile = DoorSysDropfile::from_session(&session);
        assert_eq!(dropfile.node_number, 5);
        assert_eq!(dropfile.user_name, "John Doe");
        assert_eq!(dropfile.user_alias, "JDoe");
        assert_eq!(dropfile.location, "Seattle, WA");
        assert_eq!(dropfile.security_level, 100);
        assert_eq!(dropfile.seconds_remaining, 1800);
        assert_eq!(dropfile.minutes_remaining, 30);
        assert_eq!(dropfile.total_calls, 50);
        assert_eq!(dropfile.upload_kb_total, 1024);
        assert_eq!(dropfile.download_kb_total, 2048);
    }

    #[test]
    fn test_doorsys_to_string_has_52_lines() {
        let dropfile = DoorSysDropfile::new();
        let content = dropfile.format_dropfile();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines.len(), 52, "DOOR.SYS must have exactly 52 lines");
    }

    #[test]
    fn test_doorsys_line_1_com_port() {
        let dropfile = DoorSysDropfile::new();
        let content = dropfile.format_dropfile();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines[0], "COM1");
    }

    #[test]
    fn test_doorsys_line_2_baud_rate() {
        let dropfile = DoorSysDropfile::new();
        let content = dropfile.format_dropfile();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines[1], "38400");
    }

    #[test]
    fn test_doorsys_line_10_user_name() {
        let mut dropfile = DoorSysDropfile::new();
        dropfile.user_name = "Test User".to_string();
        let content = dropfile.format_dropfile();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines[9], "Test User");
    }

    #[test]
    fn test_doorsys_boolean_fields() {
        let dropfile = DoorSysDropfile::new();
        let content = dropfile.format_dropfile();
        let lines: Vec<&str> = content.lines().collect();

        // Line 6: Screen display (Y)
        assert_eq!(lines[5], "Y");
        // Line 7: Printer toggle (N)
        assert_eq!(lines[6], "N");
        // Line 8: Page bell (Y)
        assert_eq!(lines[7], "Y");
    }

    #[test]
    fn test_doorsys_write_to_file() {
        let dropfile = DoorSysDropfile::new();
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("DOOR.SYS");

        let result = dropfile.write_to_file(&file_path);
        assert!(result.is_ok());
        assert!(file_path.exists());

        let content = fs::read_to_string(&file_path).unwrap();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines.len(), 52);
    }

    #[test]
    fn test_doorsys_time_remaining_calculation() {
        let session = DoorSession {
            node_id: 1,
            user_name: "User".to_string(),
            user_alias: None,
            location: "Unknown".to_string(),
            security_level: 10,
            time_remaining_seconds: 3665, // 61 minutes, 5 seconds
            ansi_enabled: true,
            login_time: Utc::now(),
            total_calls: 1,
            last_call_date: "11/26/25".to_string(),
            upload_kb: 0,
            download_kb: 0,
        };

        let dropfile = DoorSysDropfile::from_session(&session);
        assert_eq!(dropfile.seconds_remaining, 3665);
        assert_eq!(dropfile.minutes_remaining, 61);
    }

    #[test]
    fn test_doorsys_graphics_mode() {
        let mut dropfile = DoorSysDropfile::new();
        dropfile.graphics_mode = "NG".to_string();
        let content = dropfile.format_dropfile();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines[19], "NG"); // Line 20
    }

    #[test]
    fn test_doorsys_security_level_range() {
        let mut dropfile = DoorSysDropfile::new();
        dropfile.security_level = 255;
        let content = dropfile.format_dropfile();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines[14], "255"); // Line 15
    }

    #[test]
    fn test_doorsys_bbs_software_name() {
        let dropfile = DoorSysDropfile::new();
        let content = dropfile.format_dropfile();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines[42], "Impulse-Next BBS"); // Line 43
    }

    #[test]
    fn test_doorsys_default_protocol() {
        let mut dropfile = DoorSysDropfile::new();
        dropfile.default_protocol = 'X';
        let content = dropfile.format_dropfile();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines[26], "X"); // Line 27
    }
}
