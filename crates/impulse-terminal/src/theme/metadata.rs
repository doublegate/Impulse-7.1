//! Theme metadata structures

use serde::{Deserialize, Serialize};

/// Theme metadata containing information about the theme
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ThemeMetadata {
    /// Theme name (e.g., "Matrix", "Classic BBS")
    pub name: String,
    /// Theme author name
    pub author: String,
    /// Theme version (semver format)
    pub version: String,
    /// Description of the theme
    pub description: String,
    /// Compatible BBS version (semver range)
    pub compatible_bbs_version: String,
    /// Whether the theme requires ANSI support
    #[serde(default = "default_true")]
    pub requires_ansi: bool,
    /// Whether the theme requires UTF-8 support
    #[serde(default)]
    pub requires_utf8: bool,
}

fn default_true() -> bool {
    true
}

impl ThemeMetadata {
    /// Create new theme metadata
    pub fn new(name: String, author: String, version: String, description: String) -> Self {
        Self {
            name,
            author,
            version,
            description,
            compatible_bbs_version: ">=0.1.0".to_string(),
            requires_ansi: true,
            requires_utf8: false,
        }
    }

    /// Validate the metadata
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Theme name cannot be empty".to_string());
        }
        if self.author.is_empty() {
            return Err("Theme author cannot be empty".to_string());
        }
        if self.version.is_empty() {
            return Err("Theme version cannot be empty".to_string());
        }
        Ok(())
    }

    /// Check if compatible with a BBS version
    pub fn is_compatible_with(&self, _bbs_version: &str) -> bool {
        // Simple compatibility check - in production, use semver crate
        true
    }
}

/// Information about a theme for display purposes
#[derive(Debug, Clone)]
pub struct ThemeInfo {
    /// Theme name
    pub name: String,
    /// Theme author
    pub author: String,
    /// Theme version
    pub version: String,
    /// Short description
    pub description: String,
    /// Whether currently active
    pub is_active: bool,
}

impl ThemeInfo {
    /// Create theme info from metadata
    pub fn from_metadata(metadata: &ThemeMetadata, is_active: bool) -> Self {
        Self {
            name: metadata.name.clone(),
            author: metadata.author.clone(),
            version: metadata.version.clone(),
            description: metadata.description.clone(),
            is_active,
        }
    }

    /// Format as display string
    pub fn format_display(&self) -> String {
        let active_marker = if self.is_active { " [ACTIVE]" } else { "" };
        format!(
            "{} v{} by {}{}\n  {}",
            self.name, self.version, self.author, active_marker, self.description
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_creation() {
        let metadata = ThemeMetadata::new(
            "Test Theme".to_string(),
            "Test Author".to_string(),
            "1.0.0".to_string(),
            "A test theme".to_string(),
        );

        assert_eq!(metadata.name, "Test Theme");
        assert_eq!(metadata.author, "Test Author");
        assert_eq!(metadata.version, "1.0.0");
        assert!(metadata.requires_ansi);
        assert!(!metadata.requires_utf8);
    }

    #[test]
    fn test_metadata_validation() {
        let valid = ThemeMetadata::new(
            "Valid".to_string(),
            "Author".to_string(),
            "1.0".to_string(),
            "Description".to_string(),
        );
        assert!(valid.validate().is_ok());

        let invalid = ThemeMetadata {
            name: "".to_string(),
            author: "Author".to_string(),
            version: "1.0".to_string(),
            description: "Desc".to_string(),
            compatible_bbs_version: "0.1.0".to_string(),
            requires_ansi: true,
            requires_utf8: false,
        };
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_metadata_serialization() {
        let metadata = ThemeMetadata::new(
            "Test".to_string(),
            "Author".to_string(),
            "1.0".to_string(),
            "Desc".to_string(),
        );

        let json = serde_json::to_string(&metadata).unwrap();
        let deserialized: ThemeMetadata = serde_json::from_str(&json).unwrap();
        assert_eq!(metadata, deserialized);
    }

    #[test]
    fn test_theme_info_from_metadata() {
        let metadata = ThemeMetadata::new(
            "Test".to_string(),
            "Author".to_string(),
            "1.0".to_string(),
            "Description".to_string(),
        );

        let info = ThemeInfo::from_metadata(&metadata, true);
        assert_eq!(info.name, "Test");
        assert!(info.is_active);
    }

    #[test]
    fn test_theme_info_format_display() {
        let info = ThemeInfo {
            name: "Matrix".to_string(),
            author: "Neo".to_string(),
            version: "2.0".to_string(),
            description: "Green on black".to_string(),
            is_active: true,
        };

        let display = info.format_display();
        assert!(display.contains("Matrix"));
        assert!(display.contains("[ACTIVE]"));
        assert!(display.contains("Neo"));
    }

    #[test]
    fn test_metadata_compatibility() {
        let metadata = ThemeMetadata::new(
            "Test".to_string(),
            "Author".to_string(),
            "1.0".to_string(),
            "Desc".to_string(),
        );

        assert!(metadata.is_compatible_with("0.1.0"));
        assert!(metadata.is_compatible_with("1.0.0"));
    }
}
