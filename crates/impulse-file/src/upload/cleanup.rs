//! Upload cleanup and rollback

use std::fs;
use std::path::PathBuf;

/// Upload rollback manager
///
/// Tracks operations that need to be rolled back if upload fails.
/// Uses RAII pattern - drops execute cleanup automatically.
pub struct UploadRollback {
    /// Temporary files to delete
    temp_files: Vec<PathBuf>,

    /// Stored files to delete (if we need to rollback after storage)
    stored_files: Vec<PathBuf>,

    /// Whether rollback should execute (false = success, keep files)
    should_rollback: bool,
}

impl UploadRollback {
    /// Create a new rollback manager
    pub fn new() -> Self {
        Self {
            temp_files: Vec::new(),
            stored_files: Vec::new(),
            should_rollback: true,
        }
    }

    /// Add a temporary file to cleanup
    pub fn add_temp_file(&mut self, path: PathBuf) {
        self.temp_files.push(path);
    }

    /// Add a stored file to rollback
    pub fn add_stored_file(&mut self, path: PathBuf) {
        self.stored_files.push(path);
    }

    /// Mark upload as successful (don't rollback)
    pub fn success(mut self) {
        self.should_rollback = false;
        // Drop without executing rollback
    }

    /// Execute rollback manually
    pub fn rollback(mut self) {
        self.should_rollback = true;
        // Drop will execute cleanup
    }

    /// Internal cleanup implementation
    fn execute_cleanup(&self) {
        if !self.should_rollback {
            // Only cleanup temp files on success
            for path in &self.temp_files {
                if path.exists() {
                    let _ = fs::remove_file(path);
                }
            }
        } else {
            // Rollback everything on failure
            for path in &self.temp_files {
                if path.exists() {
                    let _ = fs::remove_file(path);
                }
            }
            for path in &self.stored_files {
                if path.exists() {
                    let _ = fs::remove_file(path);
                }
            }
        }
    }
}

impl Default for UploadRollback {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for UploadRollback {
    fn drop(&mut self) {
        self.execute_cleanup();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_rollback_on_failure() {
        let dir = tempdir().unwrap();

        // Create test files
        let temp_path = dir.path().join("temp.txt");
        let stored_path = dir.path().join("stored.txt");

        File::create(&temp_path)
            .unwrap()
            .write_all(b"temp")
            .unwrap();
        File::create(&stored_path)
            .unwrap()
            .write_all(b"stored")
            .unwrap();

        assert!(temp_path.exists());
        assert!(stored_path.exists());

        {
            let mut rollback = UploadRollback::new();
            rollback.add_temp_file(temp_path.clone());
            rollback.add_stored_file(stored_path.clone());

            // Rollback drops here with should_rollback = true
        }

        // Both files should be deleted
        assert!(!temp_path.exists());
        assert!(!stored_path.exists());
    }

    #[test]
    fn test_no_rollback_on_success() {
        let dir = tempdir().unwrap();

        // Create test files
        let temp_path = dir.path().join("temp.txt");
        let stored_path = dir.path().join("stored.txt");

        File::create(&temp_path)
            .unwrap()
            .write_all(b"temp")
            .unwrap();
        File::create(&stored_path)
            .unwrap()
            .write_all(b"stored")
            .unwrap();

        assert!(temp_path.exists());
        assert!(stored_path.exists());

        {
            let mut rollback = UploadRollback::new();
            rollback.add_temp_file(temp_path.clone());
            rollback.add_stored_file(stored_path.clone());

            rollback.success(); // Mark as successful
        }

        // Temp file deleted, stored file kept
        assert!(!temp_path.exists());
        assert!(stored_path.exists());

        // Cleanup
        fs::remove_file(&stored_path).unwrap();
    }

    #[test]
    fn test_manual_rollback() {
        let dir = tempdir().unwrap();

        // Create test files
        let temp_path = dir.path().join("temp.txt");
        let stored_path = dir.path().join("stored.txt");

        File::create(&temp_path)
            .unwrap()
            .write_all(b"temp")
            .unwrap();
        File::create(&stored_path)
            .unwrap()
            .write_all(b"stored")
            .unwrap();

        {
            let mut rollback = UploadRollback::new();
            rollback.add_temp_file(temp_path.clone());
            rollback.add_stored_file(stored_path.clone());

            rollback.rollback(); // Explicit rollback
        }

        assert!(!temp_path.exists());
        assert!(!stored_path.exists());
    }

    #[test]
    fn test_handles_missing_files() {
        // Should not panic if files don't exist
        let mut rollback = UploadRollback::new();
        rollback.add_temp_file(PathBuf::from("/nonexistent/temp.txt"));
        rollback.add_stored_file(PathBuf::from("/nonexistent/stored.txt"));

        // Drop should not panic
        drop(rollback);
    }

    #[test]
    fn test_empty_rollback() {
        // Should handle empty rollback gracefully
        let rollback = UploadRollback::new();
        drop(rollback);
    }
}
