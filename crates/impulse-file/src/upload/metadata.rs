//! FileEntry metadata creation

use crate::error::Result;
use crate::upload::storage::PendingUpload;
use chrono::Utc;
use impulse_types::file::FileEntry;
use std::path::Path;

/// Metadata builder for FileEntry
pub struct MetadataBuilder {
    upload: PendingUpload,
    description: Option<String>,
    final_path: Option<String>,
}

impl MetadataBuilder {
    /// Create a new metadata builder
    pub fn new(upload: PendingUpload) -> Self {
        Self {
            upload,
            description: None,
            final_path: None,
        }
    }

    /// Set the description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Set the final storage path
    pub fn with_final_path(mut self, path: impl AsRef<Path>) -> Self {
        self.final_path = Some(path.as_ref().to_string_lossy().to_string());
        self
    }

    /// Build FileEntry record
    pub fn build(self, file_id: u32) -> Result<FileEntry> {
        let description = self
            .description
            .or_else(|| self.upload.manual_description.clone())
            .unwrap_or_else(|| format!("Uploaded by {}", self.upload.uploader_name));

        let entry = FileEntry {
            id: file_id,
            filename: self.upload.filename.clone(),
            description,
            uploader: self.upload.uploader_name.clone(),
            uploader_id: self.upload.uploader_id,
            size_bytes: self.upload.size_bytes,
            upload_date: Utc::now(),
            area_id: self.upload.area_id,
            download_count: 0,
            is_offline: false,
            is_missing: false,
            password: None,
            cost_credits: None,
        };

        // Validate before returning
        entry.validate()?;

        Ok(entry)
    }
}

/// Create FileEntry from upload with auto-generated ID
pub fn create_file_entry(
    upload: &PendingUpload,
    file_id: u32,
    description: Option<String>,
) -> Result<FileEntry> {
    let builder = MetadataBuilder::new(upload.clone());

    let builder = if let Some(desc) = description {
        builder.with_description(desc)
    } else {
        builder
    };

    builder.build(file_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn create_test_upload() -> PendingUpload {
        PendingUpload {
            temp_path: PathBuf::from("/tmp/test.zip"),
            filename: "test.zip".to_string(),
            area_id: 1,
            uploader_id: 42,
            uploader_name: "testuser".to_string(),
            manual_description: Some("Test file".to_string()),
            size_bytes: 1024,
        }
    }

    #[test]
    fn test_metadata_builder() {
        let upload = create_test_upload();
        let entry = MetadataBuilder::new(upload.clone())
            .with_description("Custom description".to_string())
            .build(1)
            .unwrap();

        assert_eq!(entry.id, 1);
        assert_eq!(entry.filename, "test.zip");
        assert_eq!(entry.description, "Custom description");
        assert_eq!(entry.uploader, "testuser");
        assert_eq!(entry.uploader_id, 42);
        assert_eq!(entry.size_bytes, 1024);
        assert_eq!(entry.area_id, 1);
        assert_eq!(entry.download_count, 0);
        assert!(!entry.is_offline);
        assert!(!entry.is_missing);
    }

    #[test]
    fn test_metadata_uses_manual_description() {
        let upload = create_test_upload();
        let entry = MetadataBuilder::new(upload.clone()).build(1).unwrap();

        assert_eq!(entry.description, "Test file");
    }

    #[test]
    fn test_metadata_fallback_description() {
        let mut upload = create_test_upload();
        upload.manual_description = None;

        let entry = MetadataBuilder::new(upload.clone()).build(1).unwrap();

        assert_eq!(entry.description, "Uploaded by testuser");
    }

    #[test]
    fn test_create_file_entry_with_description() {
        let upload = create_test_upload();
        let entry = create_file_entry(&upload, 1, Some("Custom".to_string())).unwrap();

        assert_eq!(entry.description, "Custom");
    }

    #[test]
    fn test_create_file_entry_without_description() {
        let upload = create_test_upload();
        let entry = create_file_entry(&upload, 1, None).unwrap();

        assert_eq!(entry.description, "Test file"); // Uses manual_description
    }

    #[test]
    fn test_metadata_validation_fails_on_invalid() {
        let mut upload = create_test_upload();
        upload.filename = String::new(); // Invalid

        let result = MetadataBuilder::new(upload).build(1);

        assert!(result.is_err());
    }
}
