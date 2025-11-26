//! FILE_ID.DIZ extraction from multiple archive formats
//!
//! Supports ZIP, RAR, and 7Z archives with automatic format detection.

pub mod parser;
pub mod rar;
pub mod sevenz;
pub mod zip;

pub use parser::{DIZ_FILENAMES, MAX_DIZ_SIZE, clean_diz_content};
pub use rar::{extract_from_rar, is_rar_file};
pub use sevenz::{extract_from_7z, is_7z_file};
pub use zip::{extract_file_id_diz as extract_from_zip, is_zip_file};

use crate::error::Result;
use std::path::Path;

/// Extract FILE_ID.DIZ from any supported archive format
///
/// Automatically detects archive format and extracts DIZ content.
/// Supports ZIP, RAR, and 7Z archives.
///
/// # Arguments
///
/// * `path` - Path to the archive file
///
/// # Returns
///
/// DIZ content if found, None if not found or unsupported format
pub async fn extract_file_id_diz(path: &Path) -> Result<Option<String>> {
    // Try ZIP first (most common, pure Rust implementation)
    if is_zip_file(path) {
        return extract_from_zip(path);
    }

    // Try RAR (system command fallback)
    if is_rar_file(path) {
        return extract_from_rar(path).await;
    }

    // Try 7Z (system command fallback)
    if is_7z_file(path) {
        return extract_from_7z(path).await;
    }

    // Unsupported format
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_extract_unsupported_format() {
        let path = Path::new("test.unknown");
        let result = extract_file_id_diz(path).await.unwrap();
        assert!(result.is_none());
    }
}
