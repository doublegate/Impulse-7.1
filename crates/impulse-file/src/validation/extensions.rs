//! File extension validation

use crate::error::{FileError, Result};
use crate::upload::UploadConfig;

/// Check if file extension is allowed
///
/// # Arguments
///
/// * `extension` - File extension (without dot)
/// * `config` - Upload configuration
///
/// # Returns
///
/// Ok if extension is allowed, error otherwise
pub fn check_extension(extension: &str, config: &UploadConfig) -> Result<()> {
    if !config.is_extension_allowed(extension) {
        Err(FileError::ExtensionNotAllowed(extension.to_string()))
    } else {
        Ok(())
    }
}

/// Extract extension from filename
pub fn extract_extension(filename: &str) -> Option<&str> {
    filename.rsplit('.').next().filter(|ext| !ext.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_extension_allowed() {
        let config = UploadConfig::new().allow_extension("zip");
        let result = check_extension("zip", &config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_extension_blocked() {
        let config = UploadConfig::new(); // exe is blocked by default
        let result = check_extension("exe", &config);
        assert!(matches!(
            result,
            Err(FileError::ExtensionNotAllowed(ext)) if ext == "exe"
        ));
    }

    #[test]
    fn test_check_extension_not_in_allowlist() {
        let config = UploadConfig::new().allow_extension("zip");
        let result = check_extension("txt", &config);
        assert!(matches!(
            result,
            Err(FileError::ExtensionNotAllowed(ext)) if ext == "txt"
        ));
    }

    #[test]
    fn test_extract_extension() {
        assert_eq!(extract_extension("test.zip"), Some("zip"));
        assert_eq!(extract_extension("document.tar.gz"), Some("gz"));
        assert_eq!(extract_extension("noextension"), Some("noextension"));
        assert_eq!(extract_extension("file."), None);
    }
}
