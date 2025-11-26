//! Upload confirmation screen

use impulse_types::file::FileEntry;

/// Upload confirmation screen
pub struct ConfirmationScreen {
    /// Uploaded file entry
    pub file_entry: FileEntry,

    /// Whether FILE_ID.DIZ was found
    pub diz_found: bool,
}

impl ConfirmationScreen {
    /// Create a new confirmation screen
    pub fn new(file_entry: FileEntry, diz_found: bool) -> Self {
        Self {
            file_entry,
            diz_found,
        }
    }

    /// Render confirmation display
    pub fn render(&self) -> String {
        let mut output = String::new();

        output.push_str("Upload Successful!\n\n");

        output.push_str(&format!("Filename: {}\n", self.file_entry.filename));
        output.push_str(&format!(
            "Size: {}\n",
            self.file_entry.human_readable_size()
        ));
        output.push_str(&format!("Description: {}\n", self.file_entry.description));

        if self.diz_found {
            output.push_str("\nFILE_ID.DIZ extracted successfully.\n");
        } else {
            output.push_str("\nNo FILE_ID.DIZ found - using manual description.\n");
        }

        output.push_str(&format!(
            "\nFile has been added to area {} and is now available for download.\n",
            self.file_entry.area_id
        ));

        output.push_str("\nThank you for your contribution!\n");

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn create_test_file() -> FileEntry {
        FileEntry {
            id: 1,
            filename: "test.zip".to_string(),
            description: "Test file".to_string(),
            uploader: "testuser".to_string(),
            uploader_id: 42,
            size_bytes: 1024000,
            upload_date: Utc::now(),
            area_id: 1,
            download_count: 0,
            is_offline: false,
            is_missing: false,
            password: None,
            cost_credits: None,
        }
    }

    #[test]
    fn test_confirmation_screen_new() {
        let file = create_test_file();
        let screen = ConfirmationScreen::new(file, true);

        assert_eq!(screen.file_entry.filename, "test.zip");
        assert!(screen.diz_found);
    }

    #[test]
    fn test_confirmation_screen_render_with_diz() {
        let file = create_test_file();
        let screen = ConfirmationScreen::new(file, true);

        let output = screen.render();

        assert!(output.contains("Upload Successful"));
        assert!(output.contains("test.zip"));
        assert!(output.contains("FILE_ID.DIZ extracted"));
    }

    #[test]
    fn test_confirmation_screen_render_without_diz() {
        let file = create_test_file();
        let screen = ConfirmationScreen::new(file, false);

        let output = screen.render();

        assert!(output.contains("No FILE_ID.DIZ"));
        assert!(output.contains("manual description"));
    }
}
