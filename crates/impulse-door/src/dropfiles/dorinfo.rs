//! DORINFO1.DEF dropfile format implementation.
//!
//! DORINFO1.DEF is a 13-line dropfile format used by many door games,
//! particularly those that support multiple BBS systems.

use crate::error::{DoorError, Result};
use crate::session::DoorSession;
use std::fs;
use std::path::Path;

/// DORINFO1.DEF dropfile format - 13 line format.
///
/// This structure represents the DORINFO1.DEF dropfile specification,
/// which is a simpler alternative to DOOR.SYS used by many door games.
#[derive(Debug, Clone)]
pub struct DorinfoDropfile {
    /// Line 1: BBS name
    pub bbs_name: String,
    /// Line 2: Sysop first name
    pub sysop_first: String,
    /// Line 3: Sysop last name
    pub sysop_last: String,
    /// Line 4: COM port (COM1-4 or LOCAL)
    pub com_port: String,
    /// Line 5: Baud rate or "FOSSIL" for FOSSIL driver
    pub baud_rate: String,
    /// Line 6: Network type (0 = local, 1 = network)
    pub network_type: u8,
    /// Line 7: User first name
    pub user_first: String,
    /// Line 8: User last name
    pub user_last: String,
    /// Line 9: Location (City, State)
    pub location: String,
    /// Line 10: Graphics mode (0 = ASCII, 1 = ANSI, 2 = Avatar, 3 = RIP)
    pub graphics_mode: u8,
    /// Line 11: Security level (0-255)
    pub security_level: u8,
    /// Line 12: Time remaining (minutes)
    pub time_remaining: u32,
    /// Line 13: FOSSIL flag (-1 = no FOSSIL, 0 = FOSSIL available)
    pub fossil_flag: i8,
}

impl DorinfoDropfile {
    /// Create a new DORINFO1.DEF dropfile with default values.
    pub fn new() -> Self {
        Self {
            bbs_name: "Impulse-Next BBS".to_string(),
            sysop_first: "System".to_string(),
            sysop_last: "Operator".to_string(),
            com_port: "COM1".to_string(),
            baud_rate: "38400".to_string(),
            network_type: 0,
            user_first: "Guest".to_string(),
            user_last: "User".to_string(),
            location: "Unknown, XX".to_string(),
            graphics_mode: 1, // ANSI
            security_level: 10,
            time_remaining: 60,
            fossil_flag: -1, // No FOSSIL
        }
    }

    /// Create a DORINFO1.DEF dropfile from a door session.
    pub fn from_session(session: &DoorSession) -> Self {
        let mut dropfile = Self::new();

        // Split user name into first and last
        let name_parts: Vec<&str> = session.user_name.split_whitespace().collect();
        if !name_parts.is_empty() {
            dropfile.user_first = name_parts[0].to_string();
            if name_parts.len() > 1 {
                dropfile.user_last = name_parts[1..].join(" ");
            } else {
                dropfile.user_last = String::new(); // Empty last name for single names
            }
        }

        dropfile.location = session.location.clone();
        dropfile.graphics_mode = if session.ansi_enabled { 1 } else { 0 };
        dropfile.security_level = session.security_level;
        dropfile.time_remaining = session.time_remaining_seconds / 60;

        dropfile
    }

    /// Write the DORINFO1.DEF dropfile to a file.
    pub fn write_to_file(&self, path: &Path) -> Result<()> {
        let content = self.format_dropfile();
        fs::write(path, content).map_err(|e| {
            DoorError::DropfileCreation(format!("Failed to write DORINFO1.DEF: {}", e))
        })?;
        Ok(())
    }

    /// Format the dropfile as a 13-line string.
    pub fn format_dropfile(&self) -> String {
        format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
            self.bbs_name,
            self.sysop_first,
            self.sysop_last,
            self.com_port,
            self.baud_rate,
            self.network_type,
            self.user_first,
            self.user_last,
            self.location,
            self.graphics_mode,
            self.security_level,
            self.time_remaining,
            self.fossil_flag
        )
    }
}

impl Default for DorinfoDropfile {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_dorinfo_new() {
        let dropfile = DorinfoDropfile::new();
        assert_eq!(dropfile.bbs_name, "Impulse-Next BBS");
        assert_eq!(dropfile.sysop_first, "System");
        assert_eq!(dropfile.sysop_last, "Operator");
        assert_eq!(dropfile.com_port, "COM1");
        assert_eq!(dropfile.baud_rate, "38400");
        assert_eq!(dropfile.network_type, 0);
        assert_eq!(dropfile.user_first, "Guest");
        assert_eq!(dropfile.user_last, "User");
    }

    #[test]
    fn test_dorinfo_default() {
        let dropfile = DorinfoDropfile::default();
        assert_eq!(dropfile.graphics_mode, 1);
        assert_eq!(dropfile.security_level, 10);
        assert_eq!(dropfile.time_remaining, 60);
        assert_eq!(dropfile.fossil_flag, -1);
    }

    #[test]
    fn test_dorinfo_from_session() {
        let session = DoorSession {
            node_id: 3,
            user_name: "John Doe".to_string(),
            user_alias: None,
            location: "Portland, OR".to_string(),
            security_level: 75,
            time_remaining_seconds: 2400, // 40 minutes
            ansi_enabled: true,
            login_time: Utc::now(),
            total_calls: 25,
            last_call_date: "11/26/25".to_string(),
            upload_kb: 512,
            download_kb: 1024,
        };

        let dropfile = DorinfoDropfile::from_session(&session);
        assert_eq!(dropfile.user_first, "John");
        assert_eq!(dropfile.user_last, "Doe");
        assert_eq!(dropfile.location, "Portland, OR");
        assert_eq!(dropfile.graphics_mode, 1); // ANSI
        assert_eq!(dropfile.security_level, 75);
        assert_eq!(dropfile.time_remaining, 40);
    }

    #[test]
    fn test_dorinfo_from_session_no_ansi() {
        let session = DoorSession {
            node_id: 1,
            user_name: "Jane Smith".to_string(),
            user_alias: None,
            location: "Austin, TX".to_string(),
            security_level: 50,
            time_remaining_seconds: 1800, // 30 minutes
            ansi_enabled: false,
            login_time: Utc::now(),
            total_calls: 10,
            last_call_date: "11/26/25".to_string(),
            upload_kb: 0,
            download_kb: 0,
        };

        let dropfile = DorinfoDropfile::from_session(&session);
        assert_eq!(dropfile.graphics_mode, 0); // ASCII
    }

    #[test]
    fn test_dorinfo_from_session_single_name() {
        let session = DoorSession {
            node_id: 1,
            user_name: "Alice".to_string(),
            user_alias: None,
            location: "Unknown".to_string(),
            security_level: 10,
            time_remaining_seconds: 3600,
            ansi_enabled: true,
            login_time: Utc::now(),
            total_calls: 1,
            last_call_date: "11/26/25".to_string(),
            upload_kb: 0,
            download_kb: 0,
        };

        let dropfile = DorinfoDropfile::from_session(&session);
        assert_eq!(dropfile.user_first, "Alice");
        assert_eq!(dropfile.user_last, ""); // Empty last name
    }

    #[test]
    fn test_dorinfo_to_string_has_13_lines() {
        let dropfile = DorinfoDropfile::new();
        let content = dropfile.format_dropfile();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines.len(), 13, "DORINFO1.DEF must have exactly 13 lines");
    }

    #[test]
    fn test_dorinfo_line_1_bbs_name() {
        let dropfile = DorinfoDropfile::new();
        let content = dropfile.format_dropfile();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines[0], "Impulse-Next BBS");
    }

    #[test]
    fn test_dorinfo_line_4_com_port() {
        let mut dropfile = DorinfoDropfile::new();
        dropfile.com_port = "COM2".to_string();
        let content = dropfile.format_dropfile();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines[3], "COM2");
    }

    #[test]
    fn test_dorinfo_line_5_fossil_mode() {
        let mut dropfile = DorinfoDropfile::new();
        dropfile.baud_rate = "FOSSIL".to_string();
        let content = dropfile.format_dropfile();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines[4], "FOSSIL");
    }

    #[test]
    fn test_dorinfo_line_6_network_type() {
        let dropfile = DorinfoDropfile::new();
        let content = dropfile.format_dropfile();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines[5], "0"); // Local
    }

    #[test]
    fn test_dorinfo_line_10_graphics_modes() {
        // Test ASCII mode
        let mut dropfile = DorinfoDropfile::new();
        dropfile.graphics_mode = 0;
        let content = dropfile.format_dropfile();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines[9], "0");

        // Test ANSI mode
        dropfile.graphics_mode = 1;
        let content = dropfile.format_dropfile();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines[9], "1");
    }

    #[test]
    fn test_dorinfo_line_13_fossil_flag() {
        let dropfile = DorinfoDropfile::new();
        let content = dropfile.format_dropfile();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines[12], "-1"); // No FOSSIL
    }

    #[test]
    fn test_dorinfo_write_to_file() {
        let dropfile = DorinfoDropfile::new();
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("DORINFO1.DEF");

        let result = dropfile.write_to_file(&file_path);
        assert!(result.is_ok());
        assert!(file_path.exists());

        let content = fs::read_to_string(&file_path).unwrap();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines.len(), 13);
    }

    #[test]
    fn test_dorinfo_local_connection() {
        let mut dropfile = DorinfoDropfile::new();
        dropfile.com_port = "LOCAL".to_string();
        dropfile.network_type = 0;
        let content = dropfile.format_dropfile();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines[3], "LOCAL");
        assert_eq!(lines[5], "0");
    }

    #[test]
    fn test_dorinfo_security_level_range() {
        let mut dropfile = DorinfoDropfile::new();
        dropfile.security_level = 255;
        let content = dropfile.format_dropfile();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines[10], "255");
    }

    #[test]
    fn test_dorinfo_time_remaining_calculation() {
        let session = DoorSession {
            node_id: 1,
            user_name: "Test User".to_string(),
            user_alias: None,
            location: "Unknown".to_string(),
            security_level: 10,
            time_remaining_seconds: 4500, // 75 minutes
            ansi_enabled: true,
            login_time: Utc::now(),
            total_calls: 1,
            last_call_date: "11/26/25".to_string(),
            upload_kb: 0,
            download_kb: 0,
        };

        let dropfile = DorinfoDropfile::from_session(&session);
        assert_eq!(dropfile.time_remaining, 75);
    }
}
