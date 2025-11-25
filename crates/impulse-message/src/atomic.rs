//! Atomic file operations for message persistence

use crate::error::{MessageError, Result};
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::AsyncWriteExt;

/// Atomic file writer
///
/// Writes to a temporary file and atomically renames it on success.
/// This prevents corruption if the write fails partway through.
pub struct AtomicWriter {
    /// Target file path
    target_path: PathBuf,
    /// Temporary file path
    temp_path: PathBuf,
}

impl AtomicWriter {
    /// Create a new atomic writer
    ///
    /// # Arguments
    /// * `target_path` - The final destination path
    pub fn new(target_path: impl AsRef<Path>) -> Self {
        let target_path = target_path.as_ref().to_path_buf();
        let temp_path = Self::make_temp_path(&target_path);

        Self {
            target_path,
            temp_path,
        }
    }

    /// Generate a temporary file path
    fn make_temp_path(target: &Path) -> PathBuf {
        let parent = target.parent().unwrap_or(Path::new("."));
        let filename = target.file_name().unwrap_or_default();
        let temp_name = format!(
            ".tmp_{}_{}",
            std::process::id(),
            filename.to_string_lossy().replace('/', "_")
        );

        parent.join(temp_name)
    }

    /// Write data atomically
    ///
    /// # Arguments
    /// * `data` - The data to write
    ///
    /// # Errors
    /// Returns error if write or rename fails
    pub async fn write(&self, data: &[u8]) -> Result<()> {
        // Write to temporary file
        let mut file = fs::File::create(&self.temp_path)
            .await
            .map_err(|e| MessageError::WriteError(format!("Failed to create temp file: {}", e)))?;

        file.write_all(data)
            .await
            .map_err(|e| MessageError::WriteError(format!("Failed to write data: {}", e)))?;

        file.sync_all()
            .await
            .map_err(|e| MessageError::WriteError(format!("Failed to sync data: {}", e)))?;

        drop(file);

        // Ensure target directory exists
        if let Some(parent) = self.target_path.parent() {
            fs::create_dir_all(parent).await.map_err(|e| {
                MessageError::WriteError(format!("Failed to create parent directory: {}", e))
            })?;
        }

        // Atomically rename to target
        fs::rename(&self.temp_path, &self.target_path)
            .await
            .map_err(|e| {
                MessageError::AtomicFailed(format!("Failed to rename temp file: {}", e))
            })?;

        Ok(())
    }

    /// Append data atomically
    ///
    /// This reads the existing file, appends new data, and writes atomically.
    ///
    /// # Arguments
    /// * `data` - The data to append
    ///
    /// # Errors
    /// Returns error if read, write, or rename fails
    pub async fn append(&self, data: &[u8]) -> Result<()> {
        // Read existing data if file exists
        let mut existing = if self.target_path.exists() {
            fs::read(&self.target_path).await.map_err(|e| {
                MessageError::WriteError(format!("Failed to read existing file: {}", e))
            })?
        } else {
            Vec::new()
        };

        // Append new data
        existing.extend_from_slice(data);

        // Write atomically
        self.write(&existing).await
    }

    /// Clean up temporary file if it exists
    pub async fn cleanup(&self) -> Result<()> {
        if self.temp_path.exists() {
            fs::remove_file(&self.temp_path).await.map_err(|e| {
                MessageError::AtomicFailed(format!("Failed to clean up temp file: {}", e))
            })?;
        }
        Ok(())
    }
}

impl Drop for AtomicWriter {
    fn drop(&mut self) {
        // Best effort cleanup (can't be async in Drop)
        if self.temp_path.exists() {
            let _ = std::fs::remove_file(&self.temp_path);
        }
    }
}

/// Atomic multi-file writer
///
/// Writes to multiple files atomically. All succeed or all are rolled back.
pub struct AtomicMultiWriter {
    writers: Vec<(PathBuf, Vec<u8>)>,
}

impl AtomicMultiWriter {
    /// Create a new multi-file writer
    pub fn new() -> Self {
        Self {
            writers: Vec::new(),
        }
    }

    /// Add a file to write
    ///
    /// # Arguments
    /// * `path` - The file path
    /// * `data` - The data to write
    pub fn add_file(&mut self, path: impl AsRef<Path>, data: Vec<u8>) {
        self.writers.push((path.as_ref().to_path_buf(), data));
    }

    /// Write all files atomically
    ///
    /// # Errors
    /// Returns error if any write fails. All writes are rolled back on error.
    pub async fn write_all(&self) -> Result<()> {
        let mut temp_files = Vec::new();

        // Phase 1: Write all temp files
        for (target_path, data) in &self.writers {
            // Generate temp path manually (not using AtomicWriter to avoid Drop cleanup)
            let parent = target_path.parent().unwrap_or(Path::new("."));
            let filename = target_path.file_name().unwrap_or_default();
            let temp_name = format!(
                ".tmp_{}_{}",
                std::process::id(),
                filename.to_string_lossy().replace('/', "_")
            );
            let temp_path = parent.join(temp_name);

            // Write to temp file
            let mut file = fs::File::create(&temp_path).await.map_err(|e| {
                MessageError::WriteError(format!("Failed to create temp file: {}", e))
            })?;

            file.write_all(data)
                .await
                .map_err(|e| MessageError::WriteError(format!("Failed to write data: {}", e)))?;

            file.sync_all()
                .await
                .map_err(|e| MessageError::WriteError(format!("Failed to sync data: {}", e)))?;

            drop(file);

            temp_files.push((temp_path, target_path.clone()));
        }

        // Phase 2: Atomically rename all files
        let mut renamed = Vec::new();
        for (temp_path, target_path) in &temp_files {
            // Ensure target directory exists
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent).await.map_err(|e| {
                    // Clean up on error
                    for (temp, _) in &temp_files {
                        let _ = std::fs::remove_file(temp);
                    }
                    MessageError::WriteError(format!("Failed to create parent directory: {}", e))
                })?;
            }

            match fs::rename(temp_path, target_path).await {
                Ok(()) => {
                    renamed.push((temp_path.clone(), target_path.clone()));
                }
                Err(e) => {
                    // Rollback: restore previous files
                    for (prev_temp, prev_target) in renamed.iter().rev() {
                        let _ = fs::rename(prev_target, prev_temp).await;
                    }

                    // Clean up temp files
                    for (temp, _) in &temp_files {
                        let _ = fs::remove_file(temp).await;
                    }

                    return Err(MessageError::AtomicFailed(format!(
                        "Failed to rename temp file: {}",
                        e
                    )));
                }
            }
        }

        Ok(())
    }
}

impl Default for AtomicMultiWriter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_atomic_write() {
        let temp_dir = TempDir::new().unwrap();
        let target_path = temp_dir.path().join("test.dat");

        let writer = AtomicWriter::new(&target_path);
        let data = b"Hello, world!";

        writer.write(data).await.unwrap();

        // Verify file was written
        assert!(target_path.exists());
        let content = fs::read(&target_path).await.unwrap();
        assert_eq!(content, data);

        // Verify temp file was cleaned up
        assert!(!writer.temp_path.exists());
    }

    #[tokio::test]
    async fn test_atomic_append() {
        let temp_dir = TempDir::new().unwrap();
        let target_path = temp_dir.path().join("test.dat");

        // Write initial data
        fs::write(&target_path, b"Initial ").await.unwrap();

        // Append new data
        let writer = AtomicWriter::new(&target_path);
        writer.append(b"appended data").await.unwrap();

        // Verify data was appended
        let content = fs::read(&target_path).await.unwrap();
        assert_eq!(content, b"Initial appended data");
    }

    #[tokio::test]
    async fn test_atomic_multi_writer() {
        let temp_dir = TempDir::new().unwrap();
        let file1 = temp_dir.path().join("file1.dat");
        let file2 = temp_dir.path().join("file2.dat");

        let mut writer = AtomicMultiWriter::new();
        writer.add_file(&file1, b"Data 1".to_vec());
        writer.add_file(&file2, b"Data 2".to_vec());

        writer.write_all().await.unwrap();

        // Verify both files were written
        assert!(file1.exists());
        assert!(file2.exists());

        let content1 = fs::read(&file1).await.unwrap();
        let content2 = fs::read(&file2).await.unwrap();

        assert_eq!(content1, b"Data 1");
        assert_eq!(content2, b"Data 2");
    }

    #[tokio::test]
    async fn test_atomic_cleanup() {
        let temp_dir = TempDir::new().unwrap();
        let target_path = temp_dir.path().join("test.dat");

        let writer = AtomicWriter::new(&target_path);

        // Create temp file manually
        fs::write(&writer.temp_path, b"temp data").await.unwrap();
        assert!(writer.temp_path.exists());

        // Cleanup should remove it
        writer.cleanup().await.unwrap();
        assert!(!writer.temp_path.exists());
    }
}
