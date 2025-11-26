//! Upload quota validation

use crate::error::{FileError, Result};
use crate::upload::UploadStats;

/// Check if upload would exceed quotas
///
/// # Arguments
///
/// * `size_bytes` - Size of file being uploaded
/// * `stats` - Current user upload statistics
/// * `max_files_per_day` - Maximum files allowed per day
/// * `max_bytes_per_day` - Maximum bytes allowed per day
///
/// # Returns
///
/// Ok if quotas would not be exceeded, error otherwise
pub fn check_quota(
    size_bytes: u64,
    stats: Option<&UploadStats>,
    max_files_per_day: u32,
    max_bytes_per_day: u64,
) -> Result<()> {
    if let Some(stats) = stats {
        // Check file count quota
        if stats.would_exceed_file_quota(max_files_per_day) {
            return Err(FileError::QuotaExceeded(format!(
                "Daily file limit of {} reached",
                max_files_per_day
            )));
        }

        // Check byte quota
        if stats.would_exceed_byte_quota(size_bytes, max_bytes_per_day) {
            return Err(FileError::QuotaExceeded(format!(
                "Daily upload size limit of {} bytes would be exceeded",
                max_bytes_per_day
            )));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_quota_no_stats() {
        // No stats = no quota exceeded
        let result = check_quota(1024, None, 10, 100 * 1024);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_quota_within_limits() {
        let mut stats = UploadStats::new();
        stats.files_today = 5;
        stats.bytes_today = 50 * 1024;

        let result = check_quota(10 * 1024, Some(&stats), 10, 100 * 1024);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_quota_file_limit_exceeded() {
        let mut stats = UploadStats::new();
        stats.files_today = 10; // At limit

        let result = check_quota(1024, Some(&stats), 10, 100 * 1024);
        assert!(matches!(result, Err(FileError::QuotaExceeded(_))));
    }

    #[test]
    fn test_check_quota_byte_limit_exceeded() {
        let mut stats = UploadStats::new();
        stats.bytes_today = 95 * 1024;

        // Uploading 10KB would exceed 100KB limit
        let result = check_quota(10 * 1024, Some(&stats), 10, 100 * 1024);
        assert!(matches!(result, Err(FileError::QuotaExceeded(_))));
    }

    #[test]
    fn test_check_quota_at_byte_limit() {
        let mut stats = UploadStats::new();
        stats.bytes_today = 90 * 1024;

        // Uploading 10KB = exactly 100KB
        let result = check_quota(10 * 1024, Some(&stats), 10, 100 * 1024);
        assert!(result.is_ok());
    }
}
