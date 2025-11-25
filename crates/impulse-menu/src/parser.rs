//! Menu file parser for TOML-based menu definitions

use crate::error::{MenuParseError, ValidationError};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::Path;

/// Complete menu definition loaded from a TOML file
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MenuDefinition {
    /// Menu metadata
    pub menu: MenuMetadata,
    /// Menu options (array of options)
    #[serde(default)]
    pub option: Vec<MenuOption>,
}

/// Menu metadata section
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MenuMetadata {
    /// Unique menu name (used for navigation)
    pub name: String,
    /// Display title
    pub title: String,
    /// Optional ANSI art file to display
    #[serde(default)]
    pub ansi_art: Option<String>,
    /// Menu interaction mode
    #[serde(default)]
    pub mode: MenuMode,
    /// Optional menu to inherit from
    #[serde(default)]
    pub inherits: Option<String>,
}

/// Menu interaction mode
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum MenuMode {
    /// Hot-key mode: single keypress selects option
    #[default]
    Hotkey,
    /// Full-menu mode: user types complete command
    Fullmenu,
}

/// Individual menu option
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MenuOption {
    /// Hot-key (single character in hotkey mode)
    pub key: String,
    /// Command to execute when selected
    pub command: String,
    /// Description displayed to user
    pub description: String,
    /// Minimum security level required
    #[serde(default)]
    pub min_security: u8,
    /// Maximum security level allowed (None = no limit)
    #[serde(default)]
    pub max_security: Option<u8>,
}

/// Menu file parser
pub struct MenuParser;

impl MenuParser {
    /// Parse menu definition from TOML string
    ///
    /// # Errors
    /// Returns `MenuParseError` if TOML is invalid
    pub fn parse(content: &str) -> Result<MenuDefinition, MenuParseError> {
        toml::from_str(content).map_err(|source| MenuParseError::TomlParse {
            path: "<string>".into(),
            source,
        })
    }

    /// Parse menu definition from TOML file
    ///
    /// # Errors
    /// Returns `MenuParseError` if file cannot be read or parsed
    pub fn parse_file(path: &Path) -> Result<MenuDefinition, MenuParseError> {
        let content = std::fs::read_to_string(path).map_err(|source| MenuParseError::FileRead {
            path: path.to_path_buf(),
            source,
        })?;

        toml::from_str(&content).map_err(|source| MenuParseError::TomlParse {
            path: path.to_path_buf(),
            source,
        })
    }

    /// Validate menu definition structure
    ///
    /// # Errors
    /// Returns vector of validation errors if menu is invalid
    pub fn validate(menu: &MenuDefinition) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();

        // Validate menu metadata
        if menu.menu.name.trim().is_empty() {
            errors.push(ValidationError::EmptyMenuName);
        }

        if menu.menu.title.trim().is_empty() {
            errors.push(ValidationError::EmptyMenuTitle);
        }

        // Validate options
        let mut seen_keys = HashSet::new();

        for option in &menu.option {
            // Check for empty fields
            if option.key.trim().is_empty() {
                errors.push(ValidationError::EmptyOptionKey);
            }

            if option.command.trim().is_empty() {
                errors.push(ValidationError::EmptyOptionCommand);
            }

            if option.description.trim().is_empty() {
                errors.push(ValidationError::EmptyOptionDescription);
            }

            // Check for duplicate keys
            if !seen_keys.insert(option.key.to_uppercase()) {
                errors.push(ValidationError::DuplicateOptionKey {
                    key: option.key.clone(),
                });
            }

            // Validate security level range
            if let Some(max) = option.max_security {
                if option.min_security > max {
                    errors.push(ValidationError::InvalidSecurityRange {
                        min: option.min_security,
                        max,
                    });
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_menu_toml() -> &'static str {
        r#"
[menu]
name = "main"
title = "Main Menu"
ansi_art = "main.ans"
mode = "hotkey"

[[option]]
key = "F"
command = "files"
description = "File Areas"
min_security = 0

[[option]]
key = "M"
command = "messages"
description = "Message Areas"
min_security = 10

[[option]]
key = "G"
command = "goodbye"
description = "Logoff"
min_security = 0
max_security = 255
"#
    }

    #[test]
    fn test_parse_valid_menu() {
        let menu = MenuParser::parse(sample_menu_toml()).expect("Should parse valid menu");

        assert_eq!(menu.menu.name, "main");
        assert_eq!(menu.menu.title, "Main Menu");
        assert_eq!(menu.menu.ansi_art, Some("main.ans".to_string()));
        assert_eq!(menu.menu.mode, MenuMode::Hotkey);
        assert_eq!(menu.option.len(), 3);
    }

    #[test]
    fn test_parse_menu_options() {
        let menu = MenuParser::parse(sample_menu_toml()).unwrap();

        let file_opt = &menu.option[0];
        assert_eq!(file_opt.key, "F");
        assert_eq!(file_opt.command, "files");
        assert_eq!(file_opt.description, "File Areas");
        assert_eq!(file_opt.min_security, 0);
        assert_eq!(file_opt.max_security, None);

        let msg_opt = &menu.option[1];
        assert_eq!(msg_opt.min_security, 10);

        let goodbye_opt = &menu.option[2];
        assert_eq!(goodbye_opt.max_security, Some(255));
    }

    #[test]
    fn test_parse_fullmenu_mode() {
        let toml = r#"
[menu]
name = "test"
title = "Test Menu"
mode = "fullmenu"
"#;

        let menu = MenuParser::parse(toml).unwrap();
        assert_eq!(menu.menu.mode, MenuMode::Fullmenu);
    }

    #[test]
    fn test_parse_default_mode() {
        let toml = r#"
[menu]
name = "test"
title = "Test Menu"
"#;

        let menu = MenuParser::parse(toml).unwrap();
        assert_eq!(menu.menu.mode, MenuMode::Hotkey);
    }

    #[test]
    fn test_parse_with_inherits() {
        let toml = r#"
[menu]
name = "submenu"
title = "Sub Menu"
inherits = "main"
"#;

        let menu = MenuParser::parse(toml).unwrap();
        assert_eq!(menu.menu.inherits, Some("main".to_string()));
    }

    #[test]
    fn test_parse_invalid_toml() {
        let invalid_toml = "this is not valid toml [[[";
        let result = MenuParser::parse(invalid_toml);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_missing_required_field() {
        let toml = r#"
[menu]
name = "test"
# title is required but missing
"#;

        let result = MenuParser::parse(toml);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_valid_menu() {
        let menu = MenuParser::parse(sample_menu_toml()).unwrap();
        let result = MenuParser::validate(&menu);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_empty_menu_name() {
        let mut menu = MenuParser::parse(sample_menu_toml()).unwrap();
        menu.menu.name = "".to_string();

        let errors = MenuParser::validate(&menu).unwrap_err();
        assert!(
            errors
                .iter()
                .any(|e| matches!(e, ValidationError::EmptyMenuName))
        );
    }

    #[test]
    fn test_validate_empty_title() {
        let mut menu = MenuParser::parse(sample_menu_toml()).unwrap();
        menu.menu.title = "   ".to_string();

        let errors = MenuParser::validate(&menu).unwrap_err();
        assert!(
            errors
                .iter()
                .any(|e| matches!(e, ValidationError::EmptyMenuTitle))
        );
    }

    #[test]
    fn test_validate_duplicate_keys() {
        let toml = r#"
[menu]
name = "test"
title = "Test"

[[option]]
key = "F"
command = "files"
description = "Files"

[[option]]
key = "f"
command = "files2"
description = "Files 2"
"#;

        let menu = MenuParser::parse(toml).unwrap();
        let errors = MenuParser::validate(&menu).unwrap_err();
        assert!(errors.iter().any(|e| matches!(
            e,
            ValidationError::DuplicateOptionKey { key } if key == "F" || key == "f"
        )));
    }

    #[test]
    fn test_validate_empty_option_fields() {
        let toml = r#"
[menu]
name = "test"
title = "Test"

[[option]]
key = ""
command = ""
description = ""
"#;

        let menu = MenuParser::parse(toml).unwrap();
        let errors = MenuParser::validate(&menu).unwrap_err();
        assert!(
            errors
                .iter()
                .any(|e| matches!(e, ValidationError::EmptyOptionKey))
        );
        assert!(
            errors
                .iter()
                .any(|e| matches!(e, ValidationError::EmptyOptionCommand))
        );
        assert!(
            errors
                .iter()
                .any(|e| matches!(e, ValidationError::EmptyOptionDescription))
        );
    }

    #[test]
    fn test_validate_invalid_security_range() {
        let toml = r#"
[menu]
name = "test"
title = "Test"

[[option]]
key = "F"
command = "files"
description = "Files"
min_security = 100
max_security = 50
"#;

        let menu = MenuParser::parse(toml).unwrap();
        let errors = MenuParser::validate(&menu).unwrap_err();
        assert!(errors.iter().any(|e| matches!(
            e,
            ValidationError::InvalidSecurityRange { min: 100, max: 50 }
        )));
    }

    #[test]
    fn test_menu_mode_serialization() {
        // Test serialization within a struct context
        #[derive(Serialize, Deserialize)]
        struct TestWrapper {
            mode: MenuMode,
        }

        let hotkey = TestWrapper {
            mode: MenuMode::Hotkey,
        };
        let fullmenu = TestWrapper {
            mode: MenuMode::Fullmenu,
        };

        let hotkey_str = toml::to_string(&hotkey).unwrap();
        let fullmenu_str = toml::to_string(&fullmenu).unwrap();

        assert!(hotkey_str.contains("hotkey"));
        assert!(fullmenu_str.contains("fullmenu"));

        // Test deserialization
        let parsed_hotkey: TestWrapper = toml::from_str(&hotkey_str).unwrap();
        let parsed_fullmenu: TestWrapper = toml::from_str(&fullmenu_str).unwrap();

        assert_eq!(parsed_hotkey.mode, MenuMode::Hotkey);
        assert_eq!(parsed_fullmenu.mode, MenuMode::Fullmenu);
    }

    #[test]
    fn test_roundtrip_serialization() {
        let original = MenuParser::parse(sample_menu_toml()).unwrap();
        let serialized = toml::to_string(&original).unwrap();
        let deserialized = MenuParser::parse(&serialized).unwrap();

        assert_eq!(original, deserialized);
    }
}
