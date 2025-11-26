//! File size validation

use crate::error::{FileError, Result};

/// Check if file size is within limits
///
/// # Arguments
///
/// * `size_bytes` - Size of the file in bytes
/// * `max_size` - Maximum allowed size in bytes
///
/// # Returns
///
/// Ok if size is within limits, error otherwise
pub fn check_size(size_bytes: u64, max_size: u64) -> Result<()> {
    if size_bytes > max_size {
        Err(FileError::FileTooLarge(size_bytes, max_size))
    } else {
        Ok(())
    }
}

/// Format size for display
pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_size_within_limit() {
        let result = check_size(1024, 2048);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_size_at_limit() {
        let result = check_size(2048, 2048);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_size_exceeds_limit() {
        let result = check_size(3000, 2048);
        assert!(matches!(result, Err(FileError::FileTooLarge(3000, 2048))));
    }

    #[test]
    fn test_format_size_bytes() {
        assert_eq!(format_size(500), "500 bytes");
    }

    #[test]
    fn test_format_size_kb() {
        assert_eq!(format_size(1024), "1.00 KB");
        assert_eq!(format_size(2048), "2.00 KB");
    }

    #[test]
    fn test_format_size_mb() {
        assert_eq!(format_size(1024 * 1024), "1.00 MB");
        assert_eq!(format_size(5 * 1024 * 1024), "5.00 MB");
    }

    #[test]
    fn test_format_size_gb() {
        assert_eq!(format_size(1024 * 1024 * 1024), "1.00 GB");
        assert_eq!(format_size(2 * 1024 * 1024 * 1024), "2.00 GB");
    }
}
