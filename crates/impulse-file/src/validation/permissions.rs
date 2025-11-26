//! Area upload permission validation

use crate::error::{FileError, Result};
use crate::types::FileArea;

/// Check if uploads are allowed in the area
///
/// # Arguments
///
/// * `area` - The file area to check
///
/// # Returns
///
/// Ok if uploads are allowed, error otherwise
pub fn check_permissions(area: &FileArea) -> Result<()> {
    if !area.upload_allowed {
        Err(FileError::UploadNotAllowed(area.area_id))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_permissions_allowed() {
        let area = FileArea::new(1, "Test".to_string(), "Test area".to_string()).allow_uploads();

        let result = check_permissions(&area);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_permissions_not_allowed() {
        let area = FileArea::new(1, "Test".to_string(), "Test area".to_string());
        // upload_allowed defaults to false

        let result = check_permissions(&area);
        assert!(matches!(result, Err(FileError::UploadNotAllowed(1))));
    }
}
