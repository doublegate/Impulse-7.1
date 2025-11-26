//! FILE_ID.DIZ extraction from RAR archives
//!
//! Uses system `unrar` command for extraction (fallback approach).

use crate::diz::parser::{DIZ_FILENAMES, MAX_DIZ_SIZE, clean_diz_content};
use crate::error::Result;
use std::path::Path;
use std::process::Command;
use tempfile::tempdir;

/// Extract FILE_ID.DIZ from RAR archive
///
/// Uses system `unrar` command to extract potential DIZ files.
/// Falls back to None if unrar is not available.
pub async fn extract_from_rar(path: &Path) -> Result<Option<String>> {
    // Check if unrar is available
    if !is_unrar_available() {
        return Ok(None);
    }

    // Create temp directory for extraction (unused but needed to prevent early cleanup)
    let _temp = tempdir()?;

    // Try to extract each DIZ filename variant
    for diz_name in DIZ_FILENAMES {
        let output = Command::new("unrar")
            .arg("p") // Print to stdout
            .arg("-inul") // No messages
            .arg(path)
            .arg(diz_name)
            .output();

        if let Ok(output) = output
            && output.status.success()
        {
            let content = String::from_utf8_lossy(&output.stdout);
            if content.len() <= MAX_DIZ_SIZE as usize && !content.is_empty() {
                let cleaned = clean_diz_content(&content);
                if !cleaned.is_empty() {
                    return Ok(Some(cleaned));
                }
            }
        }
    }

    Ok(None)
}

/// Check if unrar command is available
fn is_unrar_available() -> bool {
    Command::new("unrar").arg("-v").output().is_ok()
}

/// Check if file is a RAR archive (simple extension check)
pub fn is_rar_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        let ext_lower = ext.to_string_lossy().to_lowercase();
        ext_lower == "rar"
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_rar_file() {
        assert!(is_rar_file(Path::new("test.rar")));
        assert!(is_rar_file(Path::new("test.RAR")));
        assert!(!is_rar_file(Path::new("test.zip")));
        assert!(!is_rar_file(Path::new("noext")));
    }

    // Note: Testing extract_from_rar requires unrar and actual RAR files
}
