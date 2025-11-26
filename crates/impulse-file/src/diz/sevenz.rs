//! FILE_ID.DIZ extraction from 7Z archives
//!
//! Uses system `7z` command for extraction (fallback approach).

use crate::diz::parser::{DIZ_FILENAMES, MAX_DIZ_SIZE, clean_diz_content};
use crate::error::Result;
use std::path::Path;
use std::process::Command;

/// Extract FILE_ID.DIZ from 7Z archive
///
/// Uses system `7z` command to extract potential DIZ files.
/// Falls back to None if 7z is not available.
pub async fn extract_from_7z(path: &Path) -> Result<Option<String>> {
    // Check if 7z is available
    if !is_7z_available() {
        return Ok(None);
    }

    // Try to extract each DIZ filename variant
    for diz_name in DIZ_FILENAMES {
        let output = Command::new("7z")
            .arg("e") // Extract
            .arg("-so") // Write to stdout
            .arg(path)
            .arg(diz_name)
            .output();

        if let Ok(output) = output {
            if output.status.success() {
                let content = String::from_utf8_lossy(&output.stdout);
                if content.len() <= MAX_DIZ_SIZE as usize && !content.is_empty() {
                    let cleaned = clean_diz_content(&content);
                    if !cleaned.is_empty() {
                        return Ok(Some(cleaned));
                    }
                }
            }
        }
    }

    Ok(None)
}

/// Check if 7z command is available
fn is_7z_available() -> bool {
    Command::new("7z").arg("-h").output().is_ok()
}

/// Check if file is a 7Z archive (simple extension check)
pub fn is_7z_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        let ext_lower = ext.to_string_lossy().to_lowercase();
        ext_lower == "7z"
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_7z_file() {
        assert!(is_7z_file(Path::new("test.7z")));
        assert!(is_7z_file(Path::new("test.7Z")));
        assert!(!is_7z_file(Path::new("test.zip")));
        assert!(!is_7z_file(Path::new("noext")));
    }

    // Note: Testing extract_from_7z requires 7z and actual 7Z files
}
