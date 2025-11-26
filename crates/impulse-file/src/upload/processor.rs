//! Upload processing pipeline

use crate::error::{FileError, Result};
use crate::scanning::VirusScanner;
use crate::traits::FileAreaManager;
use crate::upload::cleanup::UploadRollback;
use crate::upload::config::UploadConfig;
use crate::upload::metadata::create_file_entry;
use crate::upload::stats::{AreaStats, UploadStats};
use crate::upload::storage::{FileStorage, PendingUpload};
use crate::validation::{
    check_duplicate, check_extension, check_permissions, check_quota, check_size,
};
use impulse_types::file::FileEntry;
use std::sync::Arc;

/// Upload processor
///
/// Orchestrates the complete upload pipeline:
/// 1. Validation (size, extension, quota, permissions)
/// 2. Duplicate detection (SHA-256 hash)
/// 3. Virus scanning (ClamAV)
/// 4. FILE_ID.DIZ extraction
/// 5. File storage (atomic)
/// 6. Database record creation
/// 7. Statistics update
pub struct UploadProcessor {
    /// Virus scanner
    scanner: Arc<dyn VirusScanner>,

    /// File area manager
    file_manager: Arc<dyn FileAreaManager>,

    /// Upload configuration
    config: UploadConfig,

    /// File storage
    storage: FileStorage,

    /// Upload statistics (per-user)
    user_stats: std::collections::HashMap<u32, UploadStats>,
}

impl UploadProcessor {
    /// Create a new upload processor
    pub fn new(
        scanner: Arc<dyn VirusScanner>,
        file_manager: Arc<dyn FileAreaManager>,
        config: UploadConfig,
        storage_base: impl Into<std::path::PathBuf>,
    ) -> Self {
        Self {
            scanner,
            file_manager,
            config,
            storage: FileStorage::new(storage_base),
            user_stats: std::collections::HashMap::new(),
        }
    }

    /// Process a pending upload through the complete pipeline
    pub async fn process(&mut self, upload: PendingUpload) -> Result<FileEntry> {
        // Create rollback manager
        let mut rollback = UploadRollback::new();
        rollback.add_temp_file(upload.temp_path.clone());

        // 1. Validate upload
        self.validate(&upload).await?;

        // 2. Check for duplicates (if enabled)
        if self.config.enable_duplicate_check {
            check_duplicate(&upload.temp_path, &*self.file_manager).await?;
        }

        // 3. Scan for viruses (if enabled)
        if self.config.enable_virus_scan {
            let scan_result = self.scanner.scan_file(&upload.temp_path).await?;

            if !scan_result.is_clean {
                // Quarantine the file
                if let Some(threat) = &scan_result.threat_name {
                    return Err(FileError::VirusDetected(threat.clone()));
                } else {
                    return Err(FileError::VirusDetected("Unknown threat".to_string()));
                }
            }
        }

        // 4. Extract FILE_ID.DIZ (will be implemented in enhanced diz module)
        let description = self.extract_description(&upload)?;

        // 5. Store file atomically
        let final_path = self
            .storage
            .store_file(&upload.temp_path, upload.area_id, &upload.filename)
            .await?;

        rollback.add_stored_file(final_path.clone());

        // 6. Create database record
        let file_id = self.generate_file_id().await?;
        let file_entry = create_file_entry(&upload, file_id, description)?;

        // 7. Add to file manager
        let mut fm = self.file_manager.clone();
        Arc::get_mut(&mut fm)
            .ok_or_else(|| FileError::InvalidPath("File manager is locked".to_string()))?
            .add_file(file_entry.clone())
            .await?;

        // 8. Update area statistics
        if let Some(mut area) = self.file_manager.get_area(upload.area_id).await? {
            AreaStats::increment_file_count(&mut area);
        }

        // 9. Update user statistics
        let stats = self.user_stats.entry(upload.uploader_id).or_default();
        stats.record_upload(upload.size_bytes);

        // Success - don't rollback
        rollback.success();

        Ok(file_entry)
    }

    /// Validate upload against all constraints
    async fn validate(&self, upload: &PendingUpload) -> Result<()> {
        // Check file size
        check_size(upload.size_bytes, self.config.max_file_size)?;

        // Check file extension
        if let Some(ext) = upload.filename.rsplit('.').next() {
            check_extension(ext, &self.config)?;
        }

        // Get user stats
        let stats = self.user_stats.get(&upload.uploader_id);

        // Check quota
        check_quota(
            upload.size_bytes,
            stats,
            self.config.max_files_per_day,
            self.config.max_bytes_per_day,
        )?;

        // Check area permissions
        if let Some(area) = self.file_manager.get_area(upload.area_id).await? {
            check_permissions(&area)?;
        } else {
            return Err(FileError::AreaNotFound(upload.area_id));
        }

        Ok(())
    }

    /// Extract file description (placeholder - will use enhanced diz module)
    fn extract_description(&self, upload: &PendingUpload) -> Result<Option<String>> {
        // This will be replaced with enhanced DIZ extraction
        Ok(upload.manual_description.clone())
    }

    /// Generate unique file ID
    async fn generate_file_id(&self) -> Result<u32> {
        // Simple implementation - in production, this would query the database
        // for the next available ID
        Ok(1)
    }

    /// Get user upload statistics
    pub fn get_user_stats(&self, user_id: u32) -> Option<&UploadStats> {
        self.user_stats.get(&user_id)
    }

    /// Reset daily statistics for a user
    pub fn reset_user_daily_stats(&mut self, user_id: u32) {
        if let Some(stats) = self.user_stats.get_mut(&user_id) {
            stats.reset_daily();
        }
    }

    /// Reset all daily statistics
    pub fn reset_all_daily_stats(&mut self) {
        for stats in self.user_stats.values_mut() {
            stats.reset_daily();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manager::InMemoryFileAreaManager;
    use crate::scanning::MockScanner;
    use crate::types::FileArea;
    use std::path::PathBuf;
    use tempfile::tempdir;

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

    async fn create_test_processor() -> (UploadProcessor, tempfile::TempDir) {
        let scanner = Arc::new(MockScanner::new(true));
        let mut file_manager = InMemoryFileAreaManager::new();

        // Add test area with uploads allowed
        let area = FileArea::new(1, "Test".to_string(), "Test area".to_string()).allow_uploads();
        file_manager.add_area(area).await.unwrap();

        let file_manager: Arc<dyn FileAreaManager> = Arc::new(file_manager);
        let config = UploadConfig::new().disable_duplicate_check();
        let temp_dir = tempdir().unwrap();

        let processor = UploadProcessor::new(scanner, file_manager, config, temp_dir.path());

        (processor, temp_dir)
    }

    #[tokio::test]
    async fn test_validate_success() {
        let (processor, _temp) = create_test_processor().await;
        let upload = create_test_upload();

        let result = processor.validate(&upload).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_file_too_large() {
        let (processor, _temp) = create_test_processor().await;
        let mut upload = create_test_upload();
        upload.size_bytes = 100 * 1024 * 1024; // 100 MB, exceeds default 50 MB

        let result = processor.validate(&upload).await;
        assert!(matches!(result, Err(FileError::FileTooLarge(_, _))));
    }

    #[tokio::test]
    async fn test_validate_blocked_extension() {
        let (processor, _temp) = create_test_processor().await;
        let mut upload = create_test_upload();
        upload.filename = "malware.exe".to_string();

        let result = processor.validate(&upload).await;
        assert!(matches!(result, Err(FileError::ExtensionNotAllowed(_))));
    }

    #[tokio::test]
    async fn test_validate_area_not_found() {
        let (processor, _temp) = create_test_processor().await;
        let mut upload = create_test_upload();
        upload.area_id = 999; // Non-existent area

        let result = processor.validate(&upload).await;
        assert!(matches!(result, Err(FileError::AreaNotFound(999))));
    }

    #[test]
    fn test_get_user_stats() {
        let scanner = Arc::new(MockScanner::new(true));
        let file_manager: Arc<dyn FileAreaManager> = Arc::new(InMemoryFileAreaManager::new());
        let config = UploadConfig::new();
        let temp = tempdir().unwrap();

        let mut processor = UploadProcessor::new(scanner, file_manager, config, temp.path());

        assert!(processor.get_user_stats(42).is_none());

        // Simulate an upload recording
        let mut stats = UploadStats::new();
        stats.record_upload(1024);
        processor.user_stats.insert(42, stats);

        let user_stats = processor.get_user_stats(42).unwrap();
        assert_eq!(user_stats.total_files, 1);
        assert_eq!(user_stats.total_bytes, 1024);
    }

    #[test]
    fn test_reset_user_daily_stats() {
        let scanner = Arc::new(MockScanner::new(true));
        let file_manager: Arc<dyn FileAreaManager> = Arc::new(InMemoryFileAreaManager::new());
        let config = UploadConfig::new();
        let temp = tempdir().unwrap();

        let mut processor = UploadProcessor::new(scanner, file_manager, config, temp.path());

        let mut stats = UploadStats::new();
        stats.record_upload(1024);
        processor.user_stats.insert(42, stats);

        processor.reset_user_daily_stats(42);

        let user_stats = processor.get_user_stats(42).unwrap();
        assert_eq!(user_stats.files_today, 0);
        assert_eq!(user_stats.bytes_today, 0);
        assert_eq!(user_stats.total_files, 1); // Preserved
    }
}
