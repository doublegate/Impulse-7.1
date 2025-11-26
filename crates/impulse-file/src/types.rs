//! File area types and structures

use chrono::{DateTime, Utc};
use impulse_types::security::SecurityLevel;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// File area structure
///
/// Represents a categorized area for file storage and downloads.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileArea {
    /// Unique area ID
    pub area_id: u32,

    /// Area name
    pub name: String,

    /// Area description
    pub description: String,

    /// Physical path on filesystem (optional, can be configured separately)
    pub path: Option<PathBuf>,

    /// Minimum security level required to access
    pub security_level: SecurityLevel,

    /// Whether area is hidden from normal listings
    pub hidden: bool,

    /// Whether uploads are allowed
    pub upload_allowed: bool,

    /// Number of files in area (cached count)
    pub file_count: u32,
}

impl FileArea {
    /// Create a new file area
    pub fn new(area_id: u32, name: String, description: String) -> Self {
        Self {
            area_id,
            name,
            description,
            path: None,
            security_level: SecurityLevel::NEW_USER,
            hidden: false,
            upload_allowed: false,
            file_count: 0,
        }
    }

    /// Set the filesystem path for this area
    pub fn with_path(mut self, path: PathBuf) -> Self {
        self.path = Some(path);
        self
    }

    /// Set the security level required to access
    pub fn with_security_level(mut self, level: SecurityLevel) -> Self {
        self.security_level = level;
        self
    }

    /// Mark as hidden
    pub fn hidden(mut self) -> Self {
        self.hidden = true;
        self
    }

    /// Allow uploads
    pub fn allow_uploads(mut self) -> Self {
        self.upload_allowed = true;
        self
    }

    /// Update file count
    pub fn set_file_count(&mut self, count: u32) {
        self.file_count = count;
    }
}

/// File status indicators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileStatus {
    /// Normal file
    Normal,

    /// Recently uploaded (within last 7 days)
    New,

    /// File is offline (on removable media)
    Offline,

    /// File is missing from disk
    Missing,

    /// Popular file (high download count)
    Popular,
}

impl FileStatus {
    /// Get display character for status
    pub fn display_char(&self) -> char {
        match self {
            FileStatus::Normal => ' ',
            FileStatus::New => 'N',
            FileStatus::Offline => 'O',
            FileStatus::Missing => '!',
            FileStatus::Popular => '*',
        }
    }

    /// Get display name for status
    pub fn display_name(&self) -> &'static str {
        match self {
            FileStatus::Normal => "",
            FileStatus::New => "New",
            FileStatus::Offline => "Offline",
            FileStatus::Missing => "Missing",
            FileStatus::Popular => "Popular",
        }
    }
}

/// Sort order for file listings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SortBy {
    /// Sort by filename (A-Z)
    NameAsc,

    /// Sort by filename (Z-A)
    NameDesc,

    /// Sort by date (newest first)
    #[default]
    DateDesc,

    /// Sort by date (oldest first)
    DateAsc,

    /// Sort by size (largest first)
    SizeDesc,

    /// Sort by size (smallest first)
    SizeAsc,

    /// Sort by download count (most downloaded first)
    DownloadsDesc,

    /// Sort by download count (least downloaded first)
    DownloadsAsc,
}

impl SortBy {
    /// Get display name for sort order
    pub fn display_name(&self) -> &'static str {
        match self {
            SortBy::NameAsc => "Name (A-Z)",
            SortBy::NameDesc => "Name (Z-A)",
            SortBy::DateDesc => "Date (Newest)",
            SortBy::DateAsc => "Date (Oldest)",
            SortBy::SizeDesc => "Size (Largest)",
            SortBy::SizeAsc => "Size (Smallest)",
            SortBy::DownloadsDesc => "Downloads (Most)",
            SortBy::DownloadsAsc => "Downloads (Least)",
        }
    }
}

/// Search criteria for finding files
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchCriteria {
    /// Search by filename pattern (supports * and ? wildcards)
    pub filename: Option<String>,

    /// Search in file description
    pub description: Option<String>,

    /// Filter by uploader username
    pub uploader: Option<String>,

    /// Filter by minimum file size (bytes)
    pub min_size: Option<u64>,

    /// Filter by maximum file size (bytes)
    pub max_size: Option<u64>,

    /// Filter by upload date range start
    pub date_from: Option<DateTime<Utc>>,

    /// Filter by upload date range end
    pub date_to: Option<DateTime<Utc>>,

    /// Filter by specific file area
    pub area_id: Option<u32>,

    /// Only show new files
    pub new_only: bool,

    /// Only show available files (not offline or missing)
    pub available_only: bool,
}

impl SearchCriteria {
    /// Create empty search criteria
    pub fn new() -> Self {
        Self::default()
    }

    /// Set filename pattern
    pub fn with_filename(mut self, pattern: impl Into<String>) -> Self {
        self.filename = Some(pattern.into());
        self
    }

    /// Set description search
    pub fn with_description(mut self, text: impl Into<String>) -> Self {
        self.description = Some(text.into());
        self
    }

    /// Set uploader filter
    pub fn with_uploader(mut self, username: impl Into<String>) -> Self {
        self.uploader = Some(username.into());
        self
    }

    /// Set size range
    pub fn with_size_range(mut self, min: Option<u64>, max: Option<u64>) -> Self {
        self.min_size = min;
        self.max_size = max;
        self
    }

    /// Set date range
    pub fn with_date_range(
        mut self,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
    ) -> Self {
        self.date_from = from;
        self.date_to = to;
        self
    }

    /// Set area filter
    pub fn in_area(mut self, area_id: u32) -> Self {
        self.area_id = Some(area_id);
        self
    }

    /// Only show new files
    pub fn new_only(mut self) -> Self {
        self.new_only = true;
        self
    }

    /// Only show available files
    pub fn available_only(mut self) -> Self {
        self.available_only = true;
        self
    }

    /// Check if criteria is empty
    pub fn is_empty(&self) -> bool {
        self.filename.is_none()
            && self.description.is_none()
            && self.uploader.is_none()
            && self.min_size.is_none()
            && self.max_size.is_none()
            && self.date_from.is_none()
            && self.date_to.is_none()
            && self.area_id.is_none()
            && !self.new_only
            && !self.available_only
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_area_new() {
        let area = FileArea::new(1, "General".to_string(), "General files".to_string());
        assert_eq!(area.area_id, 1);
        assert_eq!(area.name, "General");
        assert_eq!(area.description, "General files");
        assert_eq!(area.security_level, SecurityLevel::NEW_USER);
        assert!(!area.hidden);
        assert!(!area.upload_allowed);
        assert_eq!(area.file_count, 0);
    }

    #[test]
    fn test_file_area_builder() {
        let area = FileArea::new(1, "Test".to_string(), "Test area".to_string())
            .with_path(PathBuf::from("/files/test"))
            .with_security_level(SecurityLevel::VALIDATED)
            .hidden()
            .allow_uploads();

        assert_eq!(area.path, Some(PathBuf::from("/files/test")));
        assert_eq!(area.security_level, SecurityLevel::VALIDATED);
        assert!(area.hidden);
        assert!(area.upload_allowed);
    }

    #[test]
    fn test_file_status_display() {
        assert_eq!(FileStatus::Normal.display_char(), ' ');
        assert_eq!(FileStatus::New.display_char(), 'N');
        assert_eq!(FileStatus::Offline.display_char(), 'O');
        assert_eq!(FileStatus::Missing.display_char(), '!');
        assert_eq!(FileStatus::Popular.display_char(), '*');
    }

    #[test]
    fn test_sort_by_default() {
        assert_eq!(SortBy::default(), SortBy::DateDesc);
    }

    #[test]
    fn test_search_criteria_empty() {
        let criteria = SearchCriteria::new();
        assert!(criteria.is_empty());

        let criteria = criteria.with_filename("*.zip");
        assert!(!criteria.is_empty());
    }

    #[test]
    fn test_search_criteria_builder() {
        let criteria = SearchCriteria::new()
            .with_filename("*.zip")
            .with_description("game")
            .with_uploader("alice")
            .new_only()
            .available_only();

        assert_eq!(criteria.filename, Some("*.zip".to_string()));
        assert_eq!(criteria.description, Some("game".to_string()));
        assert_eq!(criteria.uploader, Some("alice".to_string()));
        assert!(criteria.new_only);
        assert!(criteria.available_only);
    }
}
