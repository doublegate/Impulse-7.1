//! File details screen

use crate::types::FileStatus;
use chrono::{DateTime, Utc};
use impulse_types::file::FileEntry;
use std::fmt;

/// File details screen
///
/// Displays detailed information about a single file.
pub struct FileDetailsScreen {
    file: FileEntry,
    diz_content: Option<String>,
}

impl FileDetailsScreen {
    /// Create a new file details screen
    pub fn new(file: FileEntry) -> Self {
        Self {
            file,
            diz_content: None,
        }
    }

    /// Set FILE_ID.DIZ content
    pub fn with_diz(mut self, diz: String) -> Self {
        self.diz_content = Some(diz);
        self
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

    /// Format date and time
    fn format_datetime(date: &DateTime<Utc>) -> String {
        date.format("%Y-%m-%d %H:%M:%S UTC").to_string()
    }

    /// Wrap long text to specified width
    fn wrap_text(text: &str, width: usize) -> Vec<String> {
        let mut lines = Vec::new();
        for paragraph in text.split('\n') {
            if paragraph.is_empty() {
                lines.push(String::new());
                continue;
            }

            let mut current_line = String::new();
            for word in paragraph.split_whitespace() {
                if current_line.is_empty() {
                    current_line = word.to_string();
                } else if current_line.len() + word.len() < width {
                    current_line.push(' ');
                    current_line.push_str(word);
                } else {
                    lines.push(current_line);
                    current_line = word.to_string();
                }
            }
            if !current_line.is_empty() {
                lines.push(current_line);
            }
        }
        lines
    }

    /// Render the screen
    pub fn render(&self) -> String {
        let mut output = String::new();

        // Header
        output.push_str("=== File Details ===\n\n");

        // File information
        output.push_str(&format!("Filename:    {}\n", self.file.filename));
        output.push_str(&format!(
            "Size:        {}\n",
            self.file.human_readable_size()
        ));
        output.push_str(&format!(
            "Uploaded:    {}\n",
            Self::format_datetime(&self.file.upload_date)
        ));
        output.push_str(&format!("Uploader:    {}\n", self.file.uploader));
        output.push_str(&format!("Downloads:   {}\n", self.file.download_count));

        // Status
        let status = Self::get_file_status(&self.file);
        if status != FileStatus::Normal {
            output.push_str(&format!("Status:      {}\n", status.display_name()));
        }

        // Password protected
        if self.file.is_protected() {
            output.push_str("Protected:   Yes (password required)\n");
        }

        // Cost in credits
        if let Some(cost) = self.file.cost_credits {
            output.push_str(&format!("Cost:        {} credits\n", cost));
        }

        output.push('\n');

        // Description
        output.push_str("Description:\n");
        for line in Self::wrap_text(&self.file.description, 76) {
            output.push_str(&format!("  {}\n", line));
        }

        // FILE_ID.DIZ content
        if let Some(ref diz) = self.diz_content {
            output.push_str("\nFILE_ID.DIZ:\n");
            output.push_str(&format!("{}\n", "-".repeat(78)));
            for line in diz.lines() {
                output.push_str(&format!("{}\n", line));
            }
            output.push_str(&format!("{}\n", "-".repeat(78)));
        }

        // Footer
        output.push_str("\n[D]ownload [B]ack to list [Q]uit\n");

        output
    }
}

impl fmt::Display for FileDetailsScreen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_file() -> FileEntry {
        FileEntry {
            id: 1,
            filename: "testfile.zip".to_string(),
            description: "This is a test file with a description".to_string(),
            uploader: "Alice".to_string(),
            uploader_id: 42,
            size_bytes: 1024 * 1024,
            upload_date: Utc::now(),
            area_id: 1,
            download_count: 25,
            is_offline: false,
            is_missing: false,
            password: None,
            cost_credits: None,
        }
    }

    #[test]
    fn test_file_details_new() {
        let file = create_test_file();
        let screen = FileDetailsScreen::new(file.clone());

        assert_eq!(screen.file.filename, file.filename);
        assert!(screen.diz_content.is_none());
    }

    #[test]
    fn test_with_diz() {
        let file = create_test_file();
        let diz = "Test DIZ content".to_string();
        let screen = FileDetailsScreen::new(file).with_diz(diz.clone());

        assert_eq!(screen.diz_content, Some(diz));
    }

    #[test]
    fn test_render() {
        let file = create_test_file();
        let screen = FileDetailsScreen::new(file);
        let output = screen.render();

        assert!(output.contains("File Details"));
        assert!(output.contains("testfile.zip"));
        assert!(output.contains("Alice"));
        assert!(output.contains("25")); // Download count
        assert!(output.contains("Description:"));
    }

    #[test]
    fn test_render_with_diz() {
        let file = create_test_file();
        let diz =
            "Amazing Game Pack v1.0\nBy Cool Developer\n\nContains 10 awesome games!".to_string();
        let screen = FileDetailsScreen::new(file).with_diz(diz.clone());
        let output = screen.render();

        assert!(output.contains("FILE_ID.DIZ:"));
        assert!(output.contains("Amazing Game Pack"));
        assert!(output.contains("Cool Developer"));
    }

    #[test]
    fn test_password_protected() {
        let mut file = create_test_file();
        file.password = Some("secret".to_string());

        let screen = FileDetailsScreen::new(file);
        let output = screen.render();

        assert!(output.contains("Protected:"));
        assert!(output.contains("password required"));
    }

    #[test]
    fn test_credit_cost() {
        let mut file = create_test_file();
        file.cost_credits = Some(100);

        let screen = FileDetailsScreen::new(file);
        let output = screen.render();

        assert!(output.contains("Cost:"));
        assert!(output.contains("100 credits"));
    }

    #[test]
    fn test_wrap_text() {
        let text = "This is a long line that should be wrapped at the specified width to make it easier to read";
        let wrapped = FileDetailsScreen::wrap_text(text, 40);

        // Should be split into multiple lines
        assert!(wrapped.len() > 1);

        // No line should exceed the width
        for line in &wrapped {
            assert!(line.len() <= 40);
        }
    }

    #[test]
    fn test_wrap_text_with_newlines() {
        let text = "First paragraph\n\nSecond paragraph";
        let wrapped = FileDetailsScreen::wrap_text(text, 80);

        assert_eq!(wrapped.len(), 3); // Two paragraphs + empty line
        assert_eq!(wrapped[0], "First paragraph");
        assert_eq!(wrapped[1], "");
        assert_eq!(wrapped[2], "Second paragraph");
    }
}
