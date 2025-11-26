//! Duplicate file detection using SHA-256 hashing

use crate::error::Result;
use crate::traits::FileAreaManager;
use hex;
use sha2::{Digest, Sha256};
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

/// Calculate SHA-256 hash of a file
///
/// # Arguments
///
/// * `path` - Path to the file
///
/// # Returns
///
/// Hexadecimal string representation of the hash
pub async fn calculate_file_hash(path: &Path) -> Result<String> {
    let mut file = File::open(path).await?;
    let mut hasher = Sha256::new();
    let mut buffer = vec![0u8; 8192];

    loop {
        let bytes_read = file.read(&mut buffer).await?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let hash = hasher.finalize();
    Ok(hex::encode(hash))
}

/// Check if file is a duplicate based on hash
///
/// In a real implementation, this would query a database of file hashes.
/// This is a placeholder that always returns Ok (no duplicate).
///
/// # Current Behavior
///
/// This function currently:
/// 1. Calculates the SHA-256 hash of the file
/// 2. Returns Ok without checking for duplicates
///
/// # Production Implementation
///
/// For production use, this should:
/// 1. Query the database for existing files with the calculated hash
/// 2. Compare file sizes as a quick preliminary check
/// 3. Return Err(FileError::DuplicateFile(hash)) if duplicate found
/// 4. Consider implementing configurable duplicate handling policies:
///    - Reject duplicates (current planned behavior)
///    - Allow duplicates with different names
///    - Replace existing file
pub async fn check_duplicate(path: &Path, _file_manager: &dyn FileAreaManager) -> Result<()> {
    // Calculate hash
    let _hash = calculate_file_hash(path).await?;

    // NOTE: Stub implementation - always returns Ok (no duplicate detected).
    // See function documentation for production implementation requirements.

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_calculate_file_hash() {
        let mut temp = NamedTempFile::new().unwrap();
        writeln!(temp, "Test content").unwrap();
        temp.flush().unwrap();

        let hash = calculate_file_hash(temp.path()).await.unwrap();

        // Hash should be 64 hex characters (SHA-256)
        assert_eq!(hash.len(), 64);
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[tokio::test]
    async fn test_calculate_file_hash_consistency() {
        let mut temp = NamedTempFile::new().unwrap();
        writeln!(temp, "Test content").unwrap();
        temp.flush().unwrap();

        let hash1 = calculate_file_hash(temp.path()).await.unwrap();
        let hash2 = calculate_file_hash(temp.path()).await.unwrap();

        // Same file should produce same hash
        assert_eq!(hash1, hash2);
    }

    #[tokio::test]
    async fn test_calculate_file_hash_different_files() {
        let mut temp1 = NamedTempFile::new().unwrap();
        writeln!(temp1, "Content 1").unwrap();
        temp1.flush().unwrap();

        let mut temp2 = NamedTempFile::new().unwrap();
        writeln!(temp2, "Content 2").unwrap();
        temp2.flush().unwrap();

        let hash1 = calculate_file_hash(temp1.path()).await.unwrap();
        let hash2 = calculate_file_hash(temp2.path()).await.unwrap();

        // Different files should produce different hashes
        assert_ne!(hash1, hash2);
    }

    #[tokio::test]
    async fn test_calculate_file_hash_empty_file() {
        let temp = NamedTempFile::new().unwrap();

        let hash = calculate_file_hash(temp.path()).await.unwrap();

        // Empty file has known SHA-256 hash
        assert_eq!(
            hash,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }
}
