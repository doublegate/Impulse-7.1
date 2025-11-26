//! File area permissions checking

use crate::types::FileArea;
use impulse_types::security::SecurityLevel;

/// Check if a user can access a file area
///
/// # Arguments
///
/// * `area` - The file area to check
/// * `user_level` - User's security level
///
/// # Returns
///
/// `true` if the user can access the area
pub fn can_access_area(area: &FileArea, user_level: SecurityLevel) -> bool {
    // Check security level
    if !user_level.can_access(area.security_level) {
        return false;
    }

    // Hidden areas require operator status
    if area.hidden && !user_level.is_operator() {
        return false;
    }

    true
}

/// Check if a user can upload to a file area
///
/// # Arguments
///
/// * `area` - The file area to check
/// * `user_level` - User's security level
///
/// # Returns
///
/// `true` if the user can upload to the area
pub fn can_upload_to_area(area: &FileArea, user_level: SecurityLevel) -> bool {
    // Must be able to access the area
    if !can_access_area(area, user_level) {
        return false;
    }

    // Uploads must be allowed in the area
    if !area.upload_allowed {
        return false;
    }

    true
}

/// Check if a user can view hidden areas
///
/// # Arguments
///
/// * `user_level` - User's security level
///
/// # Returns
///
/// `true` if the user can view hidden areas
pub fn can_view_hidden_areas(user_level: SecurityLevel) -> bool {
    user_level.is_operator()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_area() -> FileArea {
        FileArea::new(1, "Test".to_string(), "Test area".to_string())
    }

    #[test]
    fn test_can_access_area_basic() {
        let area = create_test_area();
        assert!(can_access_area(&area, SecurityLevel::NEW_USER));
        assert!(can_access_area(&area, SecurityLevel::SYSOP));
    }

    #[test]
    fn test_can_access_area_security_level() {
        let area = create_test_area().with_security_level(SecurityLevel::VALIDATED);

        assert!(!can_access_area(&area, SecurityLevel::NEW_USER));
        assert!(can_access_area(&area, SecurityLevel::VALIDATED));
        assert!(can_access_area(&area, SecurityLevel::SYSOP));
    }

    #[test]
    fn test_can_access_hidden_area() {
        let area = create_test_area().hidden();

        assert!(!can_access_area(&area, SecurityLevel::NEW_USER));
        assert!(!can_access_area(&area, SecurityLevel::VALIDATED));
        assert!(can_access_area(&area, SecurityLevel::SYSOP));
        assert!(can_access_area(&area, SecurityLevel::COSYSOP));
    }

    #[test]
    fn test_can_upload_to_area() {
        let area = create_test_area().allow_uploads();

        assert!(can_upload_to_area(&area, SecurityLevel::NEW_USER));
        assert!(can_upload_to_area(&area, SecurityLevel::SYSOP));
    }

    #[test]
    fn test_cannot_upload_if_not_allowed() {
        let area = create_test_area(); // Uploads not allowed

        assert!(!can_upload_to_area(&area, SecurityLevel::NEW_USER));
        assert!(!can_upload_to_area(&area, SecurityLevel::SYSOP));
    }

    #[test]
    fn test_cannot_upload_if_cannot_access() {
        let area = create_test_area()
            .with_security_level(SecurityLevel::VALIDATED)
            .allow_uploads();

        assert!(!can_upload_to_area(&area, SecurityLevel::NEW_USER));
        assert!(can_upload_to_area(&area, SecurityLevel::VALIDATED));
    }

    #[test]
    fn test_can_view_hidden_areas() {
        assert!(!can_view_hidden_areas(SecurityLevel::NEW_USER));
        assert!(!can_view_hidden_areas(SecurityLevel::VALIDATED));
        assert!(!can_view_hidden_areas(SecurityLevel::PRIVILEGED));
        assert!(can_view_hidden_areas(SecurityLevel::COSYSOP));
        assert!(can_view_hidden_areas(SecurityLevel::SYSOP));
    }
}
