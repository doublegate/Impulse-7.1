//! Door session management.
//!
//! This module provides the `DoorSession` structure for managing user session
//! state during door game execution, including time tracking and user information.

use chrono::{DateTime, Utc};

/// Door session information.
///
/// This structure contains all the information needed to execute a door game,
/// including user details, time remaining, and session state.
#[derive(Debug, Clone)]
pub struct DoorSession {
    /// Node ID (1-999)
    pub node_id: u16,
    /// User's full name
    pub user_name: String,
    /// User's alias (if any)
    pub user_alias: Option<String>,
    /// User's location (City, State)
    pub location: String,
    /// User's security level (0-255)
    pub security_level: u8,
    /// Time remaining in this session (seconds)
    pub time_remaining_seconds: u32,
    /// ANSI graphics enabled
    pub ansi_enabled: bool,
    /// Time when user logged in
    pub login_time: DateTime<Utc>,
    /// Total number of times user has called
    pub total_calls: u32,
    /// Last call date (MM/DD/YY format)
    pub last_call_date: String,
    /// Total KB uploaded
    pub upload_kb: u64,
    /// Total KB downloaded
    pub download_kb: u64,
}

impl DoorSession {
    /// Create a new door session with default values.
    ///
    /// # Arguments
    ///
    /// * `node_id` - The node number (1-999)
    /// * `user_name` - The user's full name
    pub fn new(node_id: u16, user_name: String) -> Self {
        Self {
            node_id,
            user_name,
            user_alias: None,
            location: "Unknown, XX".to_string(),
            security_level: 10,
            time_remaining_seconds: 3600, // 1 hour default
            ansi_enabled: true,
            login_time: Utc::now(),
            total_calls: 1,
            last_call_date: Utc::now().format("%m/%d/%y").to_string(),
            upload_kb: 0,
            download_kb: 0,
        }
    }

    /// Update the time remaining in the session.
    ///
    /// # Arguments
    ///
    /// * `seconds` - The new time remaining in seconds
    pub fn update_time_remaining(&mut self, seconds: u32) {
        self.time_remaining_seconds = seconds;
    }

    /// Deduct time from the session.
    ///
    /// # Arguments
    ///
    /// * `seconds` - The number of seconds to deduct
    pub fn deduct_time(&mut self, seconds: u32) {
        self.time_remaining_seconds = self.time_remaining_seconds.saturating_sub(seconds);
    }

    /// Check if the user's time has expired.
    ///
    /// # Returns
    ///
    /// `true` if time remaining is zero, `false` otherwise
    pub fn is_time_expired(&self) -> bool {
        self.time_remaining_seconds == 0
    }

    /// Get the time remaining in minutes (rounded up).
    ///
    /// # Returns
    ///
    /// The time remaining in minutes
    pub fn time_remaining_minutes(&self) -> u32 {
        self.time_remaining_seconds.div_ceil(60)
    }

    /// Get the elapsed time since login in seconds.
    ///
    /// # Returns
    ///
    /// The number of seconds since login
    pub fn elapsed_seconds(&self) -> i64 {
        let now = Utc::now();
        (now - self.login_time).num_seconds()
    }

    /// Check if the user has sufficient security level.
    ///
    /// # Arguments
    ///
    /// * `required_level` - The minimum required security level
    ///
    /// # Returns
    ///
    /// `true` if the user's security level is sufficient, `false` otherwise
    pub fn has_security_level(&self, required_level: u8) -> bool {
        self.security_level >= required_level
    }

    /// Add upload statistics.
    ///
    /// # Arguments
    ///
    /// * `kb` - The number of kilobytes uploaded
    pub fn add_upload(&mut self, kb: u64) {
        self.upload_kb += kb;
    }

    /// Add download statistics.
    ///
    /// # Arguments
    ///
    /// * `kb` - The number of kilobytes downloaded
    pub fn add_download(&mut self, kb: u64) {
        self.download_kb += kb;
    }

    /// Get the upload/download ratio.
    ///
    /// # Returns
    ///
    /// The ratio of uploads to downloads, or 0.0 if no downloads
    pub fn ul_dl_ratio(&self) -> f64 {
        if self.download_kb == 0 {
            0.0
        } else {
            self.upload_kb as f64 / self.download_kb as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_session() {
        let session = DoorSession::new(3, "John Doe".to_string());
        assert_eq!(session.node_id, 3);
        assert_eq!(session.user_name, "John Doe");
        assert_eq!(session.security_level, 10);
        assert_eq!(session.time_remaining_seconds, 3600);
        assert!(session.ansi_enabled);
        assert_eq!(session.total_calls, 1);
        assert_eq!(session.upload_kb, 0);
        assert_eq!(session.download_kb, 0);
    }

    #[test]
    fn test_update_time_remaining() {
        let mut session = DoorSession::new(1, "Test User".to_string());
        session.update_time_remaining(1800);
        assert_eq!(session.time_remaining_seconds, 1800);
    }

    #[test]
    fn test_deduct_time() {
        let mut session = DoorSession::new(1, "Test User".to_string());
        session.time_remaining_seconds = 3600;
        session.deduct_time(600);
        assert_eq!(session.time_remaining_seconds, 3000);
    }

    #[test]
    fn test_deduct_time_saturating() {
        let mut session = DoorSession::new(1, "Test User".to_string());
        session.time_remaining_seconds = 100;
        session.deduct_time(200);
        assert_eq!(session.time_remaining_seconds, 0);
    }

    #[test]
    fn test_is_time_expired() {
        let mut session = DoorSession::new(1, "Test User".to_string());
        assert!(!session.is_time_expired());

        session.time_remaining_seconds = 0;
        assert!(session.is_time_expired());
    }

    #[test]
    fn test_time_remaining_minutes() {
        let mut session = DoorSession::new(1, "Test User".to_string());

        session.time_remaining_seconds = 3600;
        assert_eq!(session.time_remaining_minutes(), 60);

        session.time_remaining_seconds = 3601;
        assert_eq!(session.time_remaining_minutes(), 61);

        session.time_remaining_seconds = 59;
        assert_eq!(session.time_remaining_minutes(), 1);

        session.time_remaining_seconds = 0;
        assert_eq!(session.time_remaining_minutes(), 0);
    }

    #[test]
    fn test_elapsed_seconds() {
        let session = DoorSession::new(1, "Test User".to_string());
        let elapsed = session.elapsed_seconds();
        assert!((0..2).contains(&elapsed)); // Should be very small
    }

    #[test]
    fn test_has_security_level() {
        let mut session = DoorSession::new(1, "Test User".to_string());
        session.security_level = 50;

        assert!(session.has_security_level(10));
        assert!(session.has_security_level(50));
        assert!(!session.has_security_level(100));
    }

    #[test]
    fn test_has_security_level_edge_cases() {
        let mut session = DoorSession::new(1, "Test User".to_string());

        session.security_level = 0;
        assert!(session.has_security_level(0));
        assert!(!session.has_security_level(1));

        session.security_level = 255;
        assert!(session.has_security_level(255));
        assert!(session.has_security_level(0));
    }

    #[test]
    fn test_add_upload() {
        let mut session = DoorSession::new(1, "Test User".to_string());
        session.add_upload(100);
        assert_eq!(session.upload_kb, 100);

        session.add_upload(50);
        assert_eq!(session.upload_kb, 150);
    }

    #[test]
    fn test_add_download() {
        let mut session = DoorSession::new(1, "Test User".to_string());
        session.add_download(200);
        assert_eq!(session.download_kb, 200);

        session.add_download(100);
        assert_eq!(session.download_kb, 300);
    }

    #[test]
    fn test_ul_dl_ratio_no_downloads() {
        let mut session = DoorSession::new(1, "Test User".to_string());
        session.upload_kb = 100;
        assert_eq!(session.ul_dl_ratio(), 0.0);
    }

    #[test]
    fn test_ul_dl_ratio_with_downloads() {
        let mut session = DoorSession::new(1, "Test User".to_string());
        session.upload_kb = 100;
        session.download_kb = 200;
        assert!((session.ul_dl_ratio() - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_ul_dl_ratio_equal() {
        let mut session = DoorSession::new(1, "Test User".to_string());
        session.upload_kb = 100;
        session.download_kb = 100;
        assert!((session.ul_dl_ratio() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_session_clone() {
        let session = DoorSession::new(5, "Clone Test".to_string());
        let cloned = session.clone();

        assert_eq!(session.node_id, cloned.node_id);
        assert_eq!(session.user_name, cloned.user_name);
        assert_eq!(session.security_level, cloned.security_level);
    }

    #[test]
    fn test_session_with_alias() {
        let mut session = DoorSession::new(1, "John Doe".to_string());
        session.user_alias = Some("JDoe".to_string());

        assert!(session.user_alias.is_some());
        assert_eq!(session.user_alias.unwrap(), "JDoe");
    }

    #[test]
    fn test_last_call_date_format() {
        let session = DoorSession::new(1, "Test User".to_string());
        // Should be in MM/DD/YY format
        assert!(session.last_call_date.len() == 8);
        assert_eq!(session.last_call_date.chars().nth(2).unwrap(), '/');
        assert_eq!(session.last_call_date.chars().nth(5).unwrap(), '/');
    }
}
