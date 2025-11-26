//! Area statistics updates

use crate::types::FileArea;

/// Statistics updater for file areas
pub struct AreaStats;

impl AreaStats {
    /// Update file count for an area
    ///
    /// This is typically called after a successful upload to increment
    /// the cached file count.
    pub fn increment_file_count(area: &mut FileArea) {
        area.set_file_count(area.file_count + 1);
    }

    /// Decrement file count for an area
    ///
    /// This is typically called after a file deletion.
    pub fn decrement_file_count(area: &mut FileArea) {
        if area.file_count > 0 {
            area.set_file_count(area.file_count - 1);
        }
    }

    /// Recalculate file count from actual files
    ///
    /// This can be used to correct cached counts if they drift.
    pub fn recalculate_file_count(area: &mut FileArea, actual_count: u32) {
        area.set_file_count(actual_count);
    }
}

/// Upload statistics tracking
#[derive(Debug, Clone, Default)]
pub struct UploadStats {
    /// Total files uploaded
    pub total_files: u32,

    /// Total bytes uploaded
    pub total_bytes: u64,

    /// Files uploaded today
    pub files_today: u32,

    /// Bytes uploaded today
    pub bytes_today: u64,
}

impl UploadStats {
    /// Create new upload statistics
    pub fn new() -> Self {
        Self::default()
    }

    /// Record an upload
    pub fn record_upload(&mut self, size_bytes: u64) {
        self.total_files += 1;
        self.total_bytes += size_bytes;
        self.files_today += 1;
        self.bytes_today += size_bytes;
    }

    /// Reset daily statistics
    pub fn reset_daily(&mut self) {
        self.files_today = 0;
        self.bytes_today = 0;
    }

    /// Check if daily file quota would be exceeded
    pub fn would_exceed_file_quota(&self, max_files: u32) -> bool {
        self.files_today >= max_files
    }

    /// Check if daily byte quota would be exceeded
    pub fn would_exceed_byte_quota(&self, size_bytes: u64, max_bytes: u64) -> bool {
        self.bytes_today + size_bytes > max_bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::FileArea;

    #[test]
    fn test_increment_file_count() {
        let mut area = FileArea::new(1, "Test".to_string(), "Test area".to_string());
        assert_eq!(area.file_count, 0);

        AreaStats::increment_file_count(&mut area);
        assert_eq!(area.file_count, 1);

        AreaStats::increment_file_count(&mut area);
        assert_eq!(area.file_count, 2);
    }

    #[test]
    fn test_decrement_file_count() {
        let mut area = FileArea::new(1, "Test".to_string(), "Test area".to_string());
        area.set_file_count(5);

        AreaStats::decrement_file_count(&mut area);
        assert_eq!(area.file_count, 4);

        AreaStats::decrement_file_count(&mut area);
        assert_eq!(area.file_count, 3);
    }

    #[test]
    fn test_decrement_file_count_at_zero() {
        let mut area = FileArea::new(1, "Test".to_string(), "Test area".to_string());
        assert_eq!(area.file_count, 0);

        AreaStats::decrement_file_count(&mut area);
        assert_eq!(area.file_count, 0); // Should not go negative
    }

    #[test]
    fn test_recalculate_file_count() {
        let mut area = FileArea::new(1, "Test".to_string(), "Test area".to_string());
        area.set_file_count(100); // Incorrect count

        AreaStats::recalculate_file_count(&mut area, 42);
        assert_eq!(area.file_count, 42);
    }

    #[test]
    fn test_upload_stats_new() {
        let stats = UploadStats::new();
        assert_eq!(stats.total_files, 0);
        assert_eq!(stats.total_bytes, 0);
        assert_eq!(stats.files_today, 0);
        assert_eq!(stats.bytes_today, 0);
    }

    #[test]
    fn test_upload_stats_record_upload() {
        let mut stats = UploadStats::new();

        stats.record_upload(1024);
        assert_eq!(stats.total_files, 1);
        assert_eq!(stats.total_bytes, 1024);
        assert_eq!(stats.files_today, 1);
        assert_eq!(stats.bytes_today, 1024);

        stats.record_upload(2048);
        assert_eq!(stats.total_files, 2);
        assert_eq!(stats.total_bytes, 3072);
        assert_eq!(stats.files_today, 2);
        assert_eq!(stats.bytes_today, 3072);
    }

    #[test]
    fn test_upload_stats_reset_daily() {
        let mut stats = UploadStats::new();
        stats.record_upload(1024);
        stats.record_upload(2048);

        assert_eq!(stats.total_files, 2);
        assert_eq!(stats.total_bytes, 3072);
        assert_eq!(stats.files_today, 2);
        assert_eq!(stats.bytes_today, 3072);

        stats.reset_daily();

        assert_eq!(stats.total_files, 2); // Preserved
        assert_eq!(stats.total_bytes, 3072); // Preserved
        assert_eq!(stats.files_today, 0); // Reset
        assert_eq!(stats.bytes_today, 0); // Reset
    }

    #[test]
    fn test_would_exceed_file_quota() {
        let mut stats = UploadStats::new();
        stats.files_today = 9;

        assert!(!stats.would_exceed_file_quota(10));

        stats.files_today = 10;
        assert!(stats.would_exceed_file_quota(10));
    }

    #[test]
    fn test_would_exceed_byte_quota() {
        let mut stats = UploadStats::new();
        stats.bytes_today = 5000;

        assert!(!stats.would_exceed_byte_quota(4000, 10000));
        assert!(stats.would_exceed_byte_quota(6000, 10000));
    }
}
