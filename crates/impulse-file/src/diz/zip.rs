//! FILE_ID.DIZ extraction from ZIP archives
//!
//! BBS systems traditionally use FILE_ID.DIZ (Description In ZIP) files
//! to provide file descriptions directly from the archive.

use crate::error::Result;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use zip::ZipArchive;

/// Common FILE_ID.DIZ filename variants
const DIZ_FILENAMES: &[&str] = &[
    "FILE_ID.DIZ",
    "file_id.diz",
    "DESC.SDI",
    "desc.sdi",
    "FILE_ID.DIZ",
    "DESCRIPT.ION",
    "descript.ion",
];

/// Maximum size for DIZ file (32 KB)
const MAX_DIZ_SIZE: u64 = 32 * 1024;

/// Extract FILE_ID.DIZ content from a ZIP archive
///
/// Searches for common DIZ file variants and returns the content if found.
///
/// # Arguments
///
/// * `path` - Path to the ZIP file
///
/// # Returns
///
/// The DIZ file content if found, or None if not found or not a ZIP file
///
/// # Examples
///
/// ```no_run
/// use impulse_file::diz::zip::extract_file_id_diz;
/// use std::path::Path;
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let diz = extract_file_id_diz(Path::new("/files/game.zip"))?;
/// if let Some(content) = diz {
///     println!("Description: {}", content);
/// }
/// # Ok(())
/// # }
/// ```
pub fn extract_file_id_diz(path: &Path) -> Result<Option<String>> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return Ok(None), // File doesn't exist or can't be opened
    };

    let mut archive = match ZipArchive::new(file) {
        Ok(a) => a,
        Err(_) => return Ok(None), // Not a valid ZIP file
    };

    // Try each DIZ filename variant
    for diz_name in DIZ_FILENAMES {
        if let Ok(mut entry) = archive.by_name(diz_name) {
            // Check file size
            if entry.size() > MAX_DIZ_SIZE {
                continue; // Skip overly large files
            }

            // Read content
            let mut content = String::new();
            if entry.read_to_string(&mut content).is_ok() {
                // Clean up the content
                let cleaned = clean_diz_content(&content);
                if !cleaned.is_empty() {
                    return Ok(Some(cleaned));
                }
            }
        }
    }

    Ok(None)
}

/// Clean FILE_ID.DIZ content
///
/// Removes control characters, trims whitespace, and normalizes line endings.
fn clean_diz_content(content: &str) -> String {
    content
        .lines()
        .map(|line| {
            // Remove control characters except tab and newline
            line.chars()
                .filter(|c| !c.is_control() || *c == '\t')
                .collect::<String>()
                .trim_end()
                .to_string()
        })
        .collect::<Vec<String>>()
        .join("\n")
        .trim()
        .to_string()
}

/// Check if a file is a ZIP archive
///
/// # Arguments
///
/// * `path` - Path to the file
///
/// # Returns
///
/// `true` if the file is a valid ZIP archive
pub fn is_zip_file(path: &Path) -> bool {
    if let Ok(file) = File::open(path) {
        ZipArchive::new(file).is_ok()
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_diz_content() {
        let content = "  Test File  \n  Description  \n\n  Line 3  ";
        let cleaned = clean_diz_content(content);
        assert_eq!(cleaned, "Test File\n  Description\n\n  Line 3");
    }

    #[test]
    fn test_clean_diz_removes_control_chars() {
        let content = "Test\x00File\x01\x02\x03Description";
        let cleaned = clean_diz_content(content);
        assert_eq!(cleaned, "TestFileDescription");
    }

    #[test]
    fn test_clean_diz_preserves_tabs() {
        let content = "Column1\tColumn2\tColumn3";
        let cleaned = clean_diz_content(content);
        assert_eq!(cleaned, "Column1\tColumn2\tColumn3");
    }

    #[test]
    fn test_is_zip_file_nonexistent() {
        let result = is_zip_file(Path::new("/nonexistent/file.zip"));
        assert!(!result);
    }

    // Note: Testing extract_file_id_diz requires creating actual ZIP files,
    // which is better done in integration tests with test fixtures.
}
