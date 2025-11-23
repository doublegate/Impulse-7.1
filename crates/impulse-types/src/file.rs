//! File area data types
//!
//! This module defines the core file management structures for the BBS file areas,
//! supporting file uploads, downloads, and categorization.

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};

/// File record entry
///
/// Represents a single file in a file area, with metadata about the file,
/// uploader, and download statistics.
///
/// # Examples
///
/// ```
/// use impulse_types::file::FileEntry;
/// use chrono::Utc;
///
/// let file = FileEntry {
///     id: 1,
///     filename: "document.zip".to_string(),
///     description: "Important documentation".to_string(),
///     uploader: "Alice".to_string(),
///     uploader_id: 42,
///     size_bytes: 1024000,
///     upload_date: Utc::now(),
///     area_id: 1,
///     download_count: 0,
///     is_offline: false,
///     is_missing: false,
///     password: None,
///     cost_credits: None,
/// };
///
/// assert!(file.validate().is_ok());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    /// Unique file ID
    pub id: u32,

    /// Filename - must be valid DOS 8.3 format or long filename
    pub filename: String,

    /// File description (one-line summary)
    pub description: String,

    /// Username of the uploader
    pub uploader: String,

    /// User ID of the uploader
    pub uploader_id: u32,

    /// File size in bytes
    pub size_bytes: u64,

    /// Date and time the file was uploaded
    #[serde(with = "chrono::serde::ts_seconds")]
    pub upload_date: chrono::DateTime<chrono::Utc>,

    /// File area/directory ID
    pub area_id: u32,

    /// Number of times the file has been downloaded
    pub download_count: u32,

    /// Whether the file is offline (on removable media)
    pub is_offline: bool,

    /// Whether the file is missing from disk
    pub is_missing: bool,

    /// Optional password protection
    pub password: Option<String>,

    /// Optional credit cost to download
    pub cost_credits: Option<u32>,
}

impl FileEntry {
    /// Validate the file record
    ///
    /// Ensures all required fields meet the constraints for a valid file entry.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Validation`] if:
    /// - Filename is empty or longer than 255 characters
    /// - Filename contains invalid characters
    /// - Description is empty or longer than 255 characters
    /// - Uploader field is empty or longer than 30 characters
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::file::FileEntry;
    /// use chrono::Utc;
    ///
    /// let mut file = FileEntry {
    ///     id: 1,
    ///     filename: "test.zip".to_string(),
    ///     description: "Test file".to_string(),
    ///     uploader: "Alice".to_string(),
    ///     uploader_id: 1,
    ///     size_bytes: 1024,
    ///     upload_date: Utc::now(),
    ///     area_id: 1,
    ///     download_count: 0,
    ///     is_offline: false,
    ///     is_missing: false,
    ///     password: None,
    ///     cost_credits: None,
    /// };
    ///
    /// assert!(file.validate().is_ok());
    ///
    /// // Invalid filename (too long)
    /// file.filename = "a".repeat(256);
    /// assert!(file.validate().is_err());
    /// ```
    pub fn validate(&self) -> Result<()> {
        // Validate filename
        if self.filename.is_empty() {
            return Err(Error::Validation("Filename cannot be empty".to_string()));
        }
        if self.filename.len() > 255 {
            return Err(Error::Validation(
                "Filename must be 255 characters or less".to_string(),
            ));
        }

        // Check for invalid filename characters
        let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
        if self
            .filename
            .chars()
            .any(|c| invalid_chars.contains(&c) || c.is_control())
        {
            return Err(Error::Validation(
                "Filename contains invalid characters".to_string(),
            ));
        }

        // Validate description
        if self.description.is_empty() {
            return Err(Error::Validation("Description cannot be empty".to_string()));
        }
        if self.description.len() > 255 {
            return Err(Error::Validation(
                "Description must be 255 characters or less".to_string(),
            ));
        }

        // Validate uploader
        if self.uploader.is_empty() {
            return Err(Error::Validation("Uploader cannot be empty".to_string()));
        }
        if self.uploader.len() > 30 {
            return Err(Error::Validation(
                "Uploader must be 30 characters or less".to_string(),
            ));
        }

        Ok(())
    }

    /// Get human-readable file size
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::file::FileEntry;
    /// use chrono::Utc;
    ///
    /// let file = FileEntry {
    ///     id: 1,
    ///     filename: "test.zip".to_string(),
    ///     description: "Test".to_string(),
    ///     uploader: "Alice".to_string(),
    ///     uploader_id: 1,
    ///     size_bytes: 1024000,
    ///     upload_date: Utc::now(),
    ///     area_id: 1,
    ///     download_count: 0,
    ///     is_offline: false,
    ///     is_missing: false,
    ///     password: None,
    ///     cost_credits: None,
    /// };
    ///
    /// assert_eq!(file.human_readable_size(), "1000.00 KB");
    /// ```
    pub fn human_readable_size(&self) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;

        if self.size_bytes >= GB {
            format!("{:.2} GB", self.size_bytes as f64 / GB as f64)
        } else if self.size_bytes >= MB {
            format!("{:.2} MB", self.size_bytes as f64 / MB as f64)
        } else if self.size_bytes >= KB {
            format!("{:.2} KB", self.size_bytes as f64 / KB as f64)
        } else {
            format!("{} bytes", self.size_bytes)
        }
    }

    /// Check if the file is available for download
    pub fn is_available(&self) -> bool {
        !self.is_offline && !self.is_missing
    }

    /// Check if the file is password protected
    pub fn is_protected(&self) -> bool {
        self.password.is_some()
    }

    /// Check if the file has a download cost
    pub fn has_cost(&self) -> bool {
        self.cost_credits.is_some()
    }

    /// Record a download
    pub fn record_download(&mut self) {
        self.download_count += 1;
    }

    /// Get file extension (if any)
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::file::FileEntry;
    /// use chrono::Utc;
    ///
    /// # let file = FileEntry {
    /// #     id: 1,
    /// #     filename: "test.zip".to_string(),
    /// #     description: "Test".to_string(),
    /// #     uploader: "Alice".to_string(),
    /// #     uploader_id: 1,
    /// #     size_bytes: 1024,
    /// #     upload_date: Utc::now(),
    /// #     area_id: 1,
    /// #     download_count: 0,
    /// #     is_offline: false,
    /// #     is_missing: false,
    /// #     password: None,
    /// #     cost_credits: None,
    /// # };
    /// assert_eq!(file.extension(), Some("zip"));
    /// ```
    pub fn extension(&self) -> Option<&str> {
        self.filename.rsplit_once('.').map(|(_, ext)| ext)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_file() -> FileEntry {
        FileEntry {
            id: 1,
            filename: "testfile.zip".to_string(),
            description: "Test file description".to_string(),
            uploader: "TestUser".to_string(),
            uploader_id: 42,
            size_bytes: 1024000,
            upload_date: chrono::Utc::now(),
            area_id: 1,
            download_count: 5,
            is_offline: false,
            is_missing: false,
            password: None,
            cost_credits: None,
        }
    }

    #[test]
    fn test_valid_file() {
        let file = create_test_file();
        assert!(file.validate().is_ok());
    }

    #[test]
    fn test_empty_filename() {
        let mut file = create_test_file();
        file.filename = String::new();
        assert!(file.validate().is_err());
    }

    #[test]
    fn test_filename_too_long() {
        let mut file = create_test_file();
        file.filename = "a".repeat(256);
        assert!(file.validate().is_err());
    }

    #[test]
    fn test_invalid_filename_chars() {
        let mut file = create_test_file();
        file.filename = "test/file.zip".to_string();
        assert!(file.validate().is_err());
    }

    #[test]
    fn test_empty_description() {
        let mut file = create_test_file();
        file.description = String::new();
        assert!(file.validate().is_err());
    }

    #[test]
    fn test_description_too_long() {
        let mut file = create_test_file();
        file.description = "a".repeat(256);
        assert!(file.validate().is_err());
    }

    #[test]
    fn test_empty_uploader() {
        let mut file = create_test_file();
        file.uploader = String::new();
        assert!(file.validate().is_err());
    }

    #[test]
    fn test_uploader_too_long() {
        let mut file = create_test_file();
        file.uploader = "a".repeat(31);
        assert!(file.validate().is_err());
    }

    #[test]
    fn test_human_readable_size() {
        let mut file = create_test_file();

        file.size_bytes = 500;
        assert_eq!(file.human_readable_size(), "500 bytes");

        file.size_bytes = 1024;
        assert_eq!(file.human_readable_size(), "1.00 KB");

        file.size_bytes = 1024 * 1024;
        assert_eq!(file.human_readable_size(), "1.00 MB");

        file.size_bytes = 1024 * 1024 * 1024;
        assert_eq!(file.human_readable_size(), "1.00 GB");
    }

    #[test]
    fn test_is_available() {
        let mut file = create_test_file();
        assert!(file.is_available());

        file.is_offline = true;
        assert!(!file.is_available());

        file.is_offline = false;
        file.is_missing = true;
        assert!(!file.is_available());
    }

    #[test]
    fn test_is_protected() {
        let mut file = create_test_file();
        assert!(!file.is_protected());

        file.password = Some("secret".to_string());
        assert!(file.is_protected());
    }

    #[test]
    fn test_has_cost() {
        let mut file = create_test_file();
        assert!(!file.has_cost());

        file.cost_credits = Some(100);
        assert!(file.has_cost());
    }

    #[test]
    fn test_record_download() {
        let mut file = create_test_file();
        let initial_count = file.download_count;

        file.record_download();
        assert_eq!(file.download_count, initial_count + 1);
    }

    #[test]
    fn test_extension() {
        let mut file = create_test_file();

        file.filename = "test.zip".to_string();
        assert_eq!(file.extension(), Some("zip"));

        file.filename = "document.tar.gz".to_string();
        assert_eq!(file.extension(), Some("gz"));

        file.filename = "noextension".to_string();
        assert_eq!(file.extension(), None);
    }
}
