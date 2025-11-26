//! File storage with atomic operations

use crate::error::{FileError, Result};
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;
use tokio::fs as async_fs;

/// Pending upload information
#[derive(Debug, Clone)]
pub struct PendingUpload {
    /// Temporary file path
    pub temp_path: PathBuf,

    /// Original filename
    pub filename: String,

    /// Target file area ID
    pub area_id: u32,

    /// Uploader user ID
    pub uploader_id: u32,

    /// Uploader username
    pub uploader_name: String,

    /// Manual description (if no FILE_ID.DIZ)
    pub manual_description: Option<String>,

    /// File size in bytes
    pub size_bytes: u64,
}

/// File storage manager
///
/// Handles atomic file storage operations with proper error handling
/// and cleanup.
pub struct FileStorage {
    /// Base directory for file areas
    base_dir: PathBuf,
}

impl FileStorage {
    /// Create a new file storage manager
    pub fn new(base_dir: impl Into<PathBuf>) -> Self {
        Self {
            base_dir: base_dir.into(),
        }
    }

    /// Get the directory path for a file area
    pub fn area_path(&self, area_id: u32) -> PathBuf {
        self.base_dir.join(format!("area_{:03}", area_id))
    }

    /// Ensure area directory exists
    pub async fn ensure_area_dir(&self, area_id: u32) -> Result<()> {
        let area_path = self.area_path(area_id);
        async_fs::create_dir_all(&area_path).await?;
        Ok(())
    }

    /// Store file atomically in target area
    ///
    /// Uses atomic rename to ensure file is either completely stored
    /// or not stored at all.
    pub async fn store_file(
        &self,
        temp_path: &Path,
        area_id: u32,
        filename: &str,
    ) -> Result<PathBuf> {
        // Ensure area directory exists
        self.ensure_area_dir(area_id).await?;

        // Build target path
        let target_path = self.area_path(area_id).join(filename);

        // Check if file already exists
        if target_path.exists() {
            return Err(FileError::InvalidPath(format!(
                "File {} already exists in area {}",
                filename, area_id
            )));
        }

        // Atomic move from temp to target
        async_fs::rename(temp_path, &target_path).await?;

        Ok(target_path)
    }

    /// Create a temporary file for upload
    pub fn create_temp_file() -> Result<NamedTempFile> {
        Ok(NamedTempFile::new()?)
    }

    /// Delete a file from storage
    pub async fn delete_file(&self, area_id: u32, filename: &str) -> Result<()> {
        let file_path = self.area_path(area_id).join(filename);
        if file_path.exists() {
            async_fs::remove_file(&file_path).await?;
        }
        Ok(())
    }

    /// Get file path
    pub fn file_path(&self, area_id: u32, filename: &str) -> PathBuf {
        self.area_path(area_id).join(filename)
    }

    /// Check if file exists
    pub fn file_exists(&self, area_id: u32, filename: &str) -> bool {
        self.file_path(area_id, filename).exists()
    }

    /// Get file size
    pub async fn file_size(&self, area_id: u32, filename: &str) -> Result<u64> {
        let path = self.file_path(area_id, filename);
        let metadata = async_fs::metadata(&path).await?;
        Ok(metadata.len())
    }
}

/// Cleanup temporary files
pub struct Cleanup {
    files: Vec<PathBuf>,
}

impl Cleanup {
    /// Create a new cleanup manager
    pub fn new() -> Self {
        Self { files: Vec::new() }
    }

    /// Add a file to cleanup list
    pub fn add(&mut self, path: PathBuf) {
        self.files.push(path);
    }

    /// Execute cleanup
    pub fn execute(self) {
        for path in self.files {
            if path.exists() {
                let _ = fs::remove_file(&path);
            }
        }
    }

    /// Cancel cleanup (keep files)
    pub fn cancel(self) {
        // Drop without executing cleanup
    }
}

impl Default for Cleanup {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[tokio::test]
    async fn test_area_path() {
        let storage = FileStorage::new("/files");
        let path = storage.area_path(1);
        assert_eq!(path, PathBuf::from("/files/area_001"));
    }

    #[tokio::test]
    async fn test_file_path() {
        let storage = FileStorage::new("/files");
        let path = storage.file_path(1, "test.zip");
        assert_eq!(path, PathBuf::from("/files/area_001/test.zip"));
    }

    #[tokio::test]
    async fn test_create_temp_file() {
        let temp = FileStorage::create_temp_file().unwrap();
        assert!(temp.path().exists());
    }

    #[tokio::test]
    async fn test_ensure_area_dir() {
        let temp_dir = tempfile::tempdir().unwrap();
        let storage = FileStorage::new(temp_dir.path());

        storage.ensure_area_dir(1).await.unwrap();

        let area_path = storage.area_path(1);
        assert!(area_path.exists());
        assert!(area_path.is_dir());
    }

    #[tokio::test]
    async fn test_store_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let storage = FileStorage::new(temp_dir.path());

        // Create temp file with content
        let mut temp = FileStorage::create_temp_file().unwrap();
        writeln!(temp, "Test content").unwrap();
        temp.flush().unwrap();
        let temp_path = temp.path().to_path_buf();

        // Store file
        let stored_path = storage.store_file(&temp_path, 1, "test.txt").await.unwrap();

        // Verify
        assert!(stored_path.exists());
        assert_eq!(
            stored_path,
            temp_dir.path().join("area_001").join("test.txt")
        );

        // Temp file should be moved (no longer at original location)
        assert!(!temp_path.exists());
    }

    #[tokio::test]
    async fn test_store_file_duplicate() {
        let temp_dir = tempfile::tempdir().unwrap();
        let storage = FileStorage::new(temp_dir.path());

        // Create and store first file
        let mut temp1 = FileStorage::create_temp_file().unwrap();
        writeln!(temp1, "Test 1").unwrap();
        temp1.flush().unwrap();
        let temp1_path = temp1.path().to_path_buf();
        storage
            .store_file(&temp1_path, 1, "test.txt")
            .await
            .unwrap();

        // Try to store duplicate
        let mut temp2 = FileStorage::create_temp_file().unwrap();
        writeln!(temp2, "Test 2").unwrap();
        temp2.flush().unwrap();
        let temp2_path = temp2.path().to_path_buf();

        let result = storage.store_file(&temp2_path, 1, "test.txt").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_file_exists() {
        let temp_dir = tempfile::tempdir().unwrap();
        let storage = FileStorage::new(temp_dir.path());

        assert!(!storage.file_exists(1, "test.txt"));

        // Create file
        let mut temp = FileStorage::create_temp_file().unwrap();
        writeln!(temp, "Test").unwrap();
        temp.flush().unwrap();
        let temp_path = temp.path().to_path_buf();
        storage.store_file(&temp_path, 1, "test.txt").await.unwrap();

        assert!(storage.file_exists(1, "test.txt"));
    }

    #[tokio::test]
    async fn test_delete_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let storage = FileStorage::new(temp_dir.path());

        // Create file
        let mut temp = FileStorage::create_temp_file().unwrap();
        writeln!(temp, "Test").unwrap();
        temp.flush().unwrap();
        let temp_path = temp.path().to_path_buf();
        storage.store_file(&temp_path, 1, "test.txt").await.unwrap();

        assert!(storage.file_exists(1, "test.txt"));

        // Delete
        storage.delete_file(1, "test.txt").await.unwrap();

        assert!(!storage.file_exists(1, "test.txt"));
    }

    #[tokio::test]
    async fn test_file_size() {
        let temp_dir = tempfile::tempdir().unwrap();
        let storage = FileStorage::new(temp_dir.path());

        // Create file with known content
        let mut temp = FileStorage::create_temp_file().unwrap();
        let content = "Test content\n";
        write!(temp, "{}", content).unwrap();
        temp.flush().unwrap();
        let temp_path = temp.path().to_path_buf();
        storage.store_file(&temp_path, 1, "test.txt").await.unwrap();

        let size = storage.file_size(1, "test.txt").await.unwrap();
        assert_eq!(size, content.len() as u64);
    }

    #[test]
    fn test_cleanup_execute() {
        // Create temp files
        let temp1 = NamedTempFile::new().unwrap();
        let temp2 = NamedTempFile::new().unwrap();
        let path1 = temp1.path().to_path_buf();
        let path2 = temp2.path().to_path_buf();

        // Keep files (don't auto-delete)
        let _keep1 = temp1.into_temp_path();
        let _keep2 = temp2.into_temp_path();

        assert!(path1.exists());
        assert!(path2.exists());

        // Cleanup
        let mut cleanup = Cleanup::new();
        cleanup.add(path1.clone());
        cleanup.add(path2.clone());
        cleanup.execute();

        assert!(!path1.exists());
        assert!(!path2.exists());
    }

    #[test]
    fn test_cleanup_cancel() {
        // Create temp file
        let temp = NamedTempFile::new().unwrap();
        let path = temp.path().to_path_buf();
        let _keep = temp.into_temp_path();

        assert!(path.exists());

        // Cancel cleanup
        let mut cleanup = Cleanup::new();
        cleanup.add(path.clone());
        cleanup.cancel();

        // File should still exist
        assert!(path.exists());

        // Manual cleanup
        fs::remove_file(&path).unwrap();
    }
}
