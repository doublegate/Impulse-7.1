//! File list screen with pagination and sorting

use crate::types::{FileStatus, SortBy};
use chrono::{DateTime, Utc};
use impulse_types::file::FileEntry;
use std::fmt;

/// File list screen
///
/// Displays a paginated list of files with sorting options.
pub struct FileListScreen {
    files: Vec<FileEntry>,
    total_files: u32,
    page: u32,
    page_size: u32,
    sort_by: SortBy,
    area_name: String,
}

impl FileListScreen {
    /// Create a new file list screen
    pub fn new(files: Vec<FileEntry>, total_files: u32, page: u32, page_size: u32) -> Self {
        Self {
            files,
            total_files,
            page,
            page_size,
            sort_by: SortBy::default(),
            area_name: "Files".to_string(),
        }
    }

    /// Set the area name
    pub fn with_area_name(mut self, name: String) -> Self {
        self.area_name = name;
        self
    }

    /// Set the sort order
    pub fn with_sort_by(mut self, sort_by: SortBy) -> Self {
        self.sort_by = sort_by;
        self
    }

    /// Get current page number
    pub fn page(&self) -> u32 {
        self.page
    }

    /// Get total pages
    pub fn total_pages(&self) -> u32 {
        self.total_files.div_ceil(self.page_size)
    }

    /// Get file status
    fn get_file_status(file: &FileEntry) -> FileStatus {
        if file.is_missing {
            FileStatus::Missing
        } else if file.is_offline {
            FileStatus::Offline
        } else {
            let days_old = (Utc::now() - file.upload_date).num_days();
            if days_old <= 7 {
                FileStatus::New
            } else if file.download_count >= 100 {
                FileStatus::Popular
            } else {
                FileStatus::Normal
            }
        }
    }

    /// Format file date
    fn format_date(date: &DateTime<Utc>) -> String {
        date.format("%Y-%m-%d").to_string()
    }

    /// Render the screen
    pub fn render(&self) -> String {
        let mut output = String::new();

        // Header
        output.push_str(&format!(
            "=== {} - Page {}/{} ({} files) ===\n",
            self.area_name,
            self.page + 1,
            self.total_pages(),
            self.total_files
        ));
        output.push_str(&format!("Sort: {}\n\n", self.sort_by.display_name()));

        // Column headers
        output.push_str(&format!(
            "{:>3} {:1} {:25} {:10} {:12} {:>6} {}\n",
            "#", "S", "Filename", "Size", "Date", "DL", "Description"
        ));
        output.push_str(&format!("{}\n", "-".repeat(80)));

        // File list
        let start_num = (self.page * self.page_size) + 1;
        for (i, file) in self.files.iter().enumerate() {
            let num = start_num + i as u32;
            let status = Self::get_file_status(file);
            let status_char = status.display_char();

            output.push_str(&format!(
                "{:>3} {:1} {:25} {:10} {:12} {:>6} {}\n",
                num,
                status_char,
                truncate(&file.filename, 25),
                file.human_readable_size(),
                Self::format_date(&file.upload_date),
                file.download_count,
                truncate(&file.description, 35)
            ));
        }

        // Footer
        output.push('\n');
        output.push_str("[#]=View Details [S]=Sort [/]=Search [N]ext [P]rev [Q]uit\n");

        // Status indicators legend
        output.push_str("\nStatus: [N]ew [*]Popular [O]ffline [!]Missing\n");

        output
    }
}

impl Default for FileListScreen {
    fn default() -> Self {
        Self::new(Vec::new(), 0, 0, 20)
    }
}

impl fmt::Display for FileListScreen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

/// Truncate a string to a maximum length
fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        format!("{:width$}", s, width = max_len)
    } else {
        format!("{:width$}...", &s[..max_len - 3], width = max_len - 3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_file(id: u32, filename: &str) -> FileEntry {
        FileEntry {
            id,
            filename: filename.to_string(),
            description: format!("Description for {}", filename),
            uploader: "testuser".to_string(),
            uploader_id: 1,
            size_bytes: 1024 * 1024,
            upload_date: Utc::now(),
            area_id: 1,
            download_count: 10,
            is_offline: false,
            is_missing: false,
            password: None,
            cost_credits: None,
        }
    }

    #[test]
    fn test_file_list_new() {
        let files = vec![
            create_test_file(1, "file1.zip"),
            create_test_file(2, "file2.zip"),
        ];

        let screen = FileListScreen::new(files, 2, 0, 20);
        assert_eq!(screen.page, 0);
        assert_eq!(screen.total_files, 2);
        assert_eq!(screen.files.len(), 2);
    }

    #[test]
    fn test_total_pages() {
        let screen = FileListScreen::new(Vec::new(), 25, 0, 10);
        assert_eq!(screen.total_pages(), 3); // 25 files / 10 per page = 3 pages

        let screen = FileListScreen::new(Vec::new(), 20, 0, 10);
        assert_eq!(screen.total_pages(), 2); // Exactly 2 pages

        let screen = FileListScreen::new(Vec::new(), 5, 0, 10);
        assert_eq!(screen.total_pages(), 1); // Less than page size = 1 page
    }

    #[test]
    fn test_get_file_status_missing() {
        let mut file = create_test_file(1, "test.zip");
        file.is_missing = true;

        assert_eq!(FileListScreen::get_file_status(&file), FileStatus::Missing);
    }

    #[test]
    fn test_get_file_status_offline() {
        let mut file = create_test_file(1, "test.zip");
        file.is_offline = true;

        assert_eq!(FileListScreen::get_file_status(&file), FileStatus::Offline);
    }

    #[test]
    fn test_get_file_status_new() {
        let file = create_test_file(1, "test.zip");
        // Recently uploaded
        assert_eq!(FileListScreen::get_file_status(&file), FileStatus::New);
    }

    #[test]
    fn test_get_file_status_popular() {
        let mut file = create_test_file(1, "test.zip");
        file.upload_date = Utc::now() - chrono::Duration::days(30);
        file.download_count = 150;

        assert_eq!(FileListScreen::get_file_status(&file), FileStatus::Popular);
    }

    #[test]
    fn test_render() {
        let files = vec![
            create_test_file(1, "file1.zip"),
            create_test_file(2, "file2.zip"),
        ];

        let screen = FileListScreen::new(files, 2, 0, 20).with_area_name("Test Area".to_string());

        let output = screen.render();

        assert!(output.contains("Test Area"));
        assert!(output.contains("file1.zip"));
        assert!(output.contains("file2.zip"));
        assert!(output.contains("Page 1/1"));
        assert!(output.contains("2 files"));
    }
}
