//! User statistics tracking
//!
//! This module provides the `UserStats` type for tracking user activity
//! statistics such as uploads, downloads, message posts, and time online.

use serde::{Deserialize, Serialize};

/// User activity statistics
///
/// Tracks all quantifiable user activity including file transfers, message posts,
/// time spent online, and other metrics used for ratios and system reports.
///
/// # Examples
///
/// ```
/// use impulse_types::user_stats::UserStats;
///
/// let mut stats = UserStats::default();
///
/// // Record an upload
/// stats.record_upload(1, 1024); // 1 file, 1024 KB
/// assert_eq!(stats.uploads, 1);
/// assert_eq!(stats.upload_kb, 1024);
///
/// // Calculate upload/download ratio
/// stats.record_download(2, 512); // 2 files, 512 KB
/// let ratio = stats.ul_dl_ratio().unwrap();
/// assert_eq!(ratio, 2.0); // 1024 KB uploaded / 512 KB downloaded
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct UserStats {
    /// Total time online in minutes
    pub total_time_minutes: u32,

    /// Number of uploads
    pub uploads: u16,

    /// Number of downloads
    pub downloads: u16,

    /// Upload kilobytes
    pub upload_kb: u32,

    /// Download kilobytes
    pub download_kb: u32,

    /// Number of public message posts
    pub posts: u16,

    /// Number of emails sent
    pub emails_sent: u16,

    /// Number of feedback messages sent
    pub feedback_sent: u16,

    /// Number of times logged in
    pub logins: u16,

    /// File points (earned from uploads, used for downloads)
    pub file_points: i16,

    /// Time left today in minutes (resets daily)
    pub time_left_today: i16,

    /// Number of times logged in today
    pub logins_today: u8,

    /// Number of illegal login attempts
    pub illegal_attempts: u8,
}

impl UserStats {
    /// Create new user statistics with default values
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::user_stats::UserStats;
    ///
    /// let stats = UserStats::new();
    /// assert_eq!(stats.uploads, 0);
    /// assert_eq!(stats.downloads, 0);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Record an upload
    ///
    /// # Arguments
    ///
    /// * `count` - Number of files uploaded
    /// * `kilobytes` - Size of files in kilobytes
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::user_stats::UserStats;
    ///
    /// let mut stats = UserStats::new();
    /// stats.record_upload(1, 1024);
    /// assert_eq!(stats.uploads, 1);
    /// assert_eq!(stats.upload_kb, 1024);
    /// ```
    pub fn record_upload(&mut self, count: u16, kilobytes: u32) {
        self.uploads = self.uploads.saturating_add(count);
        self.upload_kb = self.upload_kb.saturating_add(kilobytes);
    }

    /// Record a download
    ///
    /// # Arguments
    ///
    /// * `count` - Number of files downloaded
    /// * `kilobytes` - Size of files in kilobytes
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::user_stats::UserStats;
    ///
    /// let mut stats = UserStats::new();
    /// stats.record_download(1, 512);
    /// assert_eq!(stats.downloads, 1);
    /// assert_eq!(stats.download_kb, 512);
    /// ```
    pub fn record_download(&mut self, count: u16, kilobytes: u32) {
        self.downloads = self.downloads.saturating_add(count);
        self.download_kb = self.download_kb.saturating_add(kilobytes);
    }

    /// Record a message post
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::user_stats::UserStats;
    ///
    /// let mut stats = UserStats::new();
    /// stats.record_post();
    /// assert_eq!(stats.posts, 1);
    /// ```
    pub fn record_post(&mut self) {
        self.posts = self.posts.saturating_add(1);
    }

    /// Record an email sent
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::user_stats::UserStats;
    ///
    /// let mut stats = UserStats::new();
    /// stats.record_email();
    /// assert_eq!(stats.emails_sent, 1);
    /// ```
    pub fn record_email(&mut self) {
        self.emails_sent = self.emails_sent.saturating_add(1);
    }

    /// Record a login
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::user_stats::UserStats;
    ///
    /// let mut stats = UserStats::new();
    /// stats.record_login();
    /// assert_eq!(stats.logins, 1);
    /// assert_eq!(stats.logins_today, 1);
    /// ```
    pub fn record_login(&mut self) {
        self.logins = self.logins.saturating_add(1);
        self.logins_today = self.logins_today.saturating_add(1);
    }

    /// Record time spent online
    ///
    /// # Arguments
    ///
    /// * `minutes` - Number of minutes to add
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::user_stats::UserStats;
    ///
    /// let mut stats = UserStats::new();
    /// stats.record_time(60); // 1 hour
    /// assert_eq!(stats.total_time_minutes, 60);
    /// ```
    pub fn record_time(&mut self, minutes: u32) {
        self.total_time_minutes = self.total_time_minutes.saturating_add(minutes);
    }

    /// Calculate upload/download ratio
    ///
    /// Returns `None` if no downloads have occurred (division by zero).
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::user_stats::UserStats;
    ///
    /// let mut stats = UserStats::new();
    /// stats.record_upload(1, 1000);
    /// stats.record_download(1, 500);
    ///
    /// let ratio = stats.ul_dl_ratio().unwrap();
    /// assert_eq!(ratio, 2.0);
    /// ```
    #[must_use]
    pub fn ul_dl_ratio(&self) -> Option<f64> {
        if self.download_kb == 0 {
            None
        } else {
            Some(self.upload_kb as f64 / self.download_kb as f64)
        }
    }

    /// Calculate post/read ratio (posts per 100 reads)
    ///
    /// This requires login count as a proxy for message reads.
    /// Returns `None` if logins is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::user_stats::UserStats;
    ///
    /// let mut stats = UserStats::new();
    /// stats.posts = 10;
    /// stats.logins = 20; // Proxy for reads
    ///
    /// let ratio = stats.post_read_ratio().unwrap();
    /// assert_eq!(ratio, 50.0); // 10 posts per 20 logins = 50%
    /// ```
    #[must_use]
    pub fn post_read_ratio(&self) -> Option<f64> {
        if self.logins == 0 {
            None
        } else {
            Some((self.posts as f64 / self.logins as f64) * 100.0)
        }
    }

    /// Check if user meets upload/download ratio requirement
    ///
    /// # Arguments
    ///
    /// * `required_ratio` - Minimum required ratio
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::user_stats::UserStats;
    ///
    /// let mut stats = UserStats::new();
    /// stats.record_upload(1, 1000);
    /// stats.record_download(1, 500);
    ///
    /// assert!(stats.meets_ratio_requirement(1.5));
    /// assert!(!stats.meets_ratio_requirement(2.5));
    /// ```
    #[must_use]
    pub fn meets_ratio_requirement(&self, required_ratio: f64) -> bool {
        self.ul_dl_ratio()
            .map(|r| r >= required_ratio)
            .unwrap_or(true) // If no downloads, ratio requirement is met
    }

    /// Award file points
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::user_stats::UserStats;
    ///
    /// let mut stats = UserStats::new();
    /// stats.award_file_points(100);
    /// assert_eq!(stats.file_points, 100);
    /// ```
    pub fn award_file_points(&mut self, points: i16) {
        self.file_points = self.file_points.saturating_add(points);
    }

    /// Deduct file points
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::user_stats::UserStats;
    ///
    /// let mut stats = UserStats::new();
    /// stats.file_points = 100;
    /// stats.deduct_file_points(30);
    /// assert_eq!(stats.file_points, 70);
    /// ```
    pub fn deduct_file_points(&mut self, points: i16) {
        self.file_points = self.file_points.saturating_sub(points);
    }

    /// Reset daily statistics (called at midnight)
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::user_stats::UserStats;
    ///
    /// let mut stats = UserStats::new();
    /// stats.logins_today = 5;
    /// stats.time_left_today = 60;
    ///
    /// stats.reset_daily();
    /// assert_eq!(stats.logins_today, 0);
    /// assert_eq!(stats.time_left_today, 0);
    /// ```
    pub fn reset_daily(&mut self) {
        self.logins_today = 0;
        self.time_left_today = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let stats = UserStats::default();
        assert_eq!(stats.uploads, 0);
        assert_eq!(stats.downloads, 0);
        assert_eq!(stats.total_time_minutes, 0);
    }

    #[test]
    fn test_record_upload() {
        let mut stats = UserStats::new();
        stats.record_upload(1, 1024);
        assert_eq!(stats.uploads, 1);
        assert_eq!(stats.upload_kb, 1024);

        stats.record_upload(2, 512);
        assert_eq!(stats.uploads, 3);
        assert_eq!(stats.upload_kb, 1536);
    }

    #[test]
    fn test_record_download() {
        let mut stats = UserStats::new();
        stats.record_download(1, 512);
        assert_eq!(stats.downloads, 1);
        assert_eq!(stats.download_kb, 512);

        stats.record_download(2, 1024);
        assert_eq!(stats.downloads, 3);
        assert_eq!(stats.download_kb, 1536);
    }

    #[test]
    fn test_record_post() {
        let mut stats = UserStats::new();
        stats.record_post();
        assert_eq!(stats.posts, 1);

        stats.record_post();
        assert_eq!(stats.posts, 2);
    }

    #[test]
    fn test_record_email() {
        let mut stats = UserStats::new();
        stats.record_email();
        assert_eq!(stats.emails_sent, 1);
    }

    #[test]
    fn test_record_login() {
        let mut stats = UserStats::new();
        stats.record_login();
        assert_eq!(stats.logins, 1);
        assert_eq!(stats.logins_today, 1);

        stats.record_login();
        assert_eq!(stats.logins, 2);
        assert_eq!(stats.logins_today, 2);
    }

    #[test]
    fn test_record_time() {
        let mut stats = UserStats::new();
        stats.record_time(60);
        assert_eq!(stats.total_time_minutes, 60);

        stats.record_time(30);
        assert_eq!(stats.total_time_minutes, 90);
    }

    #[test]
    fn test_ul_dl_ratio() {
        let mut stats = UserStats::new();

        // No downloads, returns None
        assert_eq!(stats.ul_dl_ratio(), None);

        stats.record_upload(1, 1000);
        stats.record_download(1, 500);
        assert_eq!(stats.ul_dl_ratio(), Some(2.0));

        stats.record_upload(1, 500);
        assert_eq!(stats.ul_dl_ratio(), Some(3.0));
    }

    #[test]
    fn test_post_read_ratio() {
        let mut stats = UserStats::new();

        // No logins, returns None
        assert_eq!(stats.post_read_ratio(), None);

        stats.posts = 10;
        stats.logins = 20;
        assert_eq!(stats.post_read_ratio(), Some(50.0));
    }

    #[test]
    fn test_meets_ratio_requirement() {
        let mut stats = UserStats::new();

        // No downloads, requirement is met
        assert!(stats.meets_ratio_requirement(1.0));

        stats.record_upload(1, 1000);
        stats.record_download(1, 500);

        assert!(stats.meets_ratio_requirement(1.5));
        assert!(stats.meets_ratio_requirement(2.0));
        assert!(!stats.meets_ratio_requirement(2.5));
    }

    #[test]
    fn test_file_points() {
        let mut stats = UserStats::new();

        stats.award_file_points(100);
        assert_eq!(stats.file_points, 100);

        stats.deduct_file_points(30);
        assert_eq!(stats.file_points, 70);

        stats.deduct_file_points(100);
        assert_eq!(stats.file_points, -30);
    }

    #[test]
    fn test_reset_daily() {
        let mut stats = UserStats::new();
        stats.logins_today = 5;
        stats.time_left_today = 60;

        stats.reset_daily();
        assert_eq!(stats.logins_today, 0);
        assert_eq!(stats.time_left_today, 0);
    }

    #[test]
    fn test_serialization() {
        let stats = UserStats {
            total_time_minutes: 120,
            uploads: 10,
            downloads: 5,
            upload_kb: 1024,
            download_kb: 512,
            posts: 20,
            emails_sent: 15,
            feedback_sent: 3,
            logins: 50,
            file_points: 100,
            time_left_today: 60,
            logins_today: 2,
            illegal_attempts: 0,
        };

        let json = serde_json::to_string(&stats).unwrap();
        let deserialized: UserStats = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, stats);
    }
}
