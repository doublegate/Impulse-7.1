//! Upload prompt screen

use crate::types::FileArea;

/// Upload prompt screen
///
/// Displays upload form and prompts for file selection and description.
pub struct UploadScreen {
    /// Target file area
    pub area: FileArea,

    /// User-provided description (optional)
    pub description: Option<String>,
}

impl UploadScreen {
    /// Create a new upload screen
    pub fn new(area: FileArea) -> Self {
        Self {
            area,
            description: None,
        }
    }

    /// Set manual description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Render upload prompt
    pub fn render(&self) -> String {
        let mut output = String::new();

        output.push_str(&format!("Upload to: {}\n", self.area.name));
        output.push_str(&format!("Area: {}\n\n", self.area.description));

        output.push_str("File upload will be processed after selection.\n");
        output.push_str("The file will be scanned for viruses before being added.\n\n");

        if let Some(ref desc) = self.description {
            output.push_str(&format!("Description: {}\n", desc));
        } else {
            output.push_str("Description: (will be extracted from FILE_ID.DIZ if available)\n");
        }

        output.push_str("\nReady to upload file...\n");

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upload_screen_new() {
        let area = FileArea::new(1, "General".to_string(), "General files".to_string());
        let screen = UploadScreen::new(area);

        assert_eq!(screen.area.area_id, 1);
        assert!(screen.description.is_none());
    }

    #[test]
    fn test_upload_screen_with_description() {
        let area = FileArea::new(1, "General".to_string(), "General files".to_string());
        let screen = UploadScreen::new(area).with_description("Test file".to_string());

        assert_eq!(screen.description, Some("Test file".to_string()));
    }

    #[test]
    fn test_upload_screen_render() {
        let area = FileArea::new(1, "General".to_string(), "General files".to_string());
        let screen = UploadScreen::new(area);

        let output = screen.render();

        assert!(output.contains("Upload to: General"));
        assert!(output.contains("will be scanned"));
    }
}
