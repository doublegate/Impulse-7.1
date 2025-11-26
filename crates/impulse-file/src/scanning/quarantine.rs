//! Quarantine management for infected files

use crate::error::Result;
use chrono::Utc;
use std::path::{Path, PathBuf};
use tokio::fs;

/// Quarantine manager
///
/// Handles moving infected files to a quarantine directory
/// and tracking quarantined files.
pub struct QuarantineManager {
    /// Base quarantine directory
    quarantine_dir: PathBuf,
}

impl QuarantineManager {
    /// Create a new quarantine manager
    pub fn new(quarantine_dir: impl Into<PathBuf>) -> Self {
        Self {
            quarantine_dir: quarantine_dir.into(),
        }
    }

    /// Ensure quarantine directory exists
    pub async fn ensure_dir(&self) -> Result<()> {
        fs::create_dir_all(&self.quarantine_dir).await?;
        Ok(())
    }

    /// Quarantine a file
    ///
    /// Moves the file to the quarantine directory with a timestamp
    /// and threat name in the filename.
    pub async fn quarantine_file(&self, file_path: &Path, threat_name: &str) -> Result<PathBuf> {
        self.ensure_dir().await?;

        // Build quarantine filename: original_name.threat_name.timestamp
        let original_name = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let safe_threat_name = threat_name.replace(['/', '\\', ' '], "_");

        let quarantine_filename = format!("{}.{}.{}", original_name, safe_threat_name, timestamp);

        let quarantine_path = self.quarantine_dir.join(quarantine_filename);

        // Move file to quarantine
        fs::rename(file_path, &quarantine_path).await?;

        Ok(quarantine_path)
    }

    /// List quarantined files
    pub async fn list_quarantined_files(&self) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        if !self.quarantine_dir.exists() {
            return Ok(files);
        }

        let mut entries = fs::read_dir(&self.quarantine_dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            if entry.file_type().await?.is_file() {
                files.push(entry.path());
            }
        }

        Ok(files)
    }

    /// Delete a quarantined file
    pub async fn delete_quarantined(&self, filename: &str) -> Result<()> {
        let path = self.quarantine_dir.join(filename);
        if path.exists() {
            fs::remove_file(&path).await?;
        }
        Ok(())
    }

    /// Delete all quarantined files
    pub async fn purge_quarantine(&self) -> Result<usize> {
        let files = self.list_quarantined_files().await?;
        let count = files.len();

        for file in files {
            fs::remove_file(&file).await?;
        }

        Ok(count)
    }

    /// Get quarantine directory path
    pub fn quarantine_dir(&self) -> &Path {
        &self.quarantine_dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_ensure_dir() {
        let temp = tempdir().unwrap();
        let quar_dir = temp.path().join("quarantine");

        assert!(!quar_dir.exists());

        let manager = QuarantineManager::new(&quar_dir);
        manager.ensure_dir().await.unwrap();

        assert!(quar_dir.exists());
        assert!(quar_dir.is_dir());
    }

    #[tokio::test]
    async fn test_quarantine_file() {
        let temp = tempdir().unwrap();
        let quar_dir = temp.path().join("quarantine");
        let manager = QuarantineManager::new(&quar_dir);

        // Create test file
        let test_file = temp.path().join("infected.exe");
        let mut file = std::fs::File::create(&test_file).unwrap();
        writeln!(file, "EICAR test").unwrap();
        drop(file);

        assert!(test_file.exists());

        // Quarantine it
        let quarantined_path = manager
            .quarantine_file(&test_file, "Eicar-Test-Signature")
            .await
            .unwrap();

        // Original file should be moved
        assert!(!test_file.exists());
        assert!(quarantined_path.exists());

        // Check filename format
        let filename = quarantined_path.file_name().unwrap().to_str().unwrap();
        assert!(filename.starts_with("infected.exe.Eicar-Test-Signature."));
    }

    #[tokio::test]
    async fn test_list_quarantined_files() {
        let temp = tempdir().unwrap();
        let quar_dir = temp.path().join("quarantine");
        let manager = QuarantineManager::new(&quar_dir);

        // Initially empty
        let files = manager.list_quarantined_files().await.unwrap();
        assert_eq!(files.len(), 0);

        // Create test files
        for i in 1..=3 {
            let test_file = temp.path().join(format!("test{}.txt", i));
            std::fs::File::create(&test_file).unwrap();
            manager
                .quarantine_file(&test_file, "Test.Virus")
                .await
                .unwrap();
        }

        // Should have 3 files
        let files = manager.list_quarantined_files().await.unwrap();
        assert_eq!(files.len(), 3);
    }

    #[tokio::test]
    async fn test_delete_quarantined() {
        let temp = tempdir().unwrap();
        let quar_dir = temp.path().join("quarantine");
        let manager = QuarantineManager::new(&quar_dir);

        // Create and quarantine file
        let test_file = temp.path().join("test.txt");
        std::fs::File::create(&test_file).unwrap();
        let quarantined_path = manager
            .quarantine_file(&test_file, "Test.Virus")
            .await
            .unwrap();

        assert!(quarantined_path.exists());

        // Delete it
        let filename = quarantined_path.file_name().unwrap().to_str().unwrap();
        manager.delete_quarantined(filename).await.unwrap();

        assert!(!quarantined_path.exists());
    }

    #[tokio::test]
    async fn test_purge_quarantine() {
        let temp = tempdir().unwrap();
        let quar_dir = temp.path().join("quarantine");
        let manager = QuarantineManager::new(&quar_dir);

        // Create multiple quarantined files
        for i in 1..=5 {
            let test_file = temp.path().join(format!("test{}.txt", i));
            std::fs::File::create(&test_file).unwrap();
            manager
                .quarantine_file(&test_file, "Test.Virus")
                .await
                .unwrap();
        }

        let files_before = manager.list_quarantined_files().await.unwrap();
        assert_eq!(files_before.len(), 5);

        // Purge
        let count = manager.purge_quarantine().await.unwrap();
        assert_eq!(count, 5);

        let files_after = manager.list_quarantined_files().await.unwrap();
        assert_eq!(files_after.len(), 0);
    }
}
