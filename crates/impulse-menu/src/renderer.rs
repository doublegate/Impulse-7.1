//! Menu rendering engine

use crate::parser::{MenuDefinition, MenuMode, MenuOption};

/// Rendered menu ready for display
#[derive(Debug, Clone, PartialEq)]
pub struct RenderedMenu {
    /// Menu title
    pub title: String,
    /// Rendered menu content (body)
    pub content: String,
    /// Prompt to display
    pub prompt: String,
    /// List of valid keys/commands
    pub valid_keys: Vec<String>,
}

/// Menu renderer with support for different display modes
#[derive(Debug, Default)]
pub struct MenuRenderer {
    /// Width of the menu (for formatting)
    pub width: usize,
}

impl MenuRenderer {
    /// Create a new menu renderer with default settings
    pub fn new() -> Self {
        Self { width: 80 }
    }

    /// Create a menu renderer with specified width
    pub fn with_width(width: usize) -> Self {
        Self { width }
    }

    /// Render menu for display
    ///
    /// Filters options by user's security level and formats according to menu mode.
    pub fn render(&self, menu: &MenuDefinition, user_security: u8) -> RenderedMenu {
        let visible_options = self.filter_options(&menu.option, user_security);

        let content = match menu.menu.mode {
            MenuMode::Hotkey => self.format_hotkey(menu, &visible_options),
            MenuMode::Fullmenu => self.format_fullmenu(menu, &visible_options),
        };

        let prompt = match menu.menu.mode {
            MenuMode::Hotkey => "Command: ".to_string(),
            MenuMode::Fullmenu => "Enter command: ".to_string(),
        };

        let valid_keys = visible_options
            .iter()
            .map(|opt| opt.key.to_uppercase())
            .collect();

        RenderedMenu {
            title: menu.menu.title.clone(),
            content,
            prompt,
            valid_keys,
        }
    }

    /// Filter options by security level
    ///
    /// Returns only options the user has permission to see.
    pub fn filter_options<'a>(
        &self,
        options: &'a [MenuOption],
        security: u8,
    ) -> Vec<&'a MenuOption> {
        options
            .iter()
            .filter(|opt| {
                opt.min_security <= security && opt.max_security.is_none_or(|max| security <= max)
            })
            .collect()
    }

    /// Format menu for hotkey mode
    ///
    /// Displays options as "(key) description" format.
    pub fn format_hotkey(&self, menu: &MenuDefinition, options: &[&MenuOption]) -> String {
        let mut output = String::new();

        // Title header
        output.push_str(&self.format_title(&menu.menu.title));
        output.push('\n');

        // Options
        for option in options {
            output.push_str(&format!("({}) {}\n", option.key, option.description));
        }

        output
    }

    /// Format menu for fullmenu mode
    ///
    /// Displays options as "command - description" format.
    pub fn format_fullmenu(&self, menu: &MenuDefinition, options: &[&MenuOption]) -> String {
        let mut output = String::new();

        // Title header
        output.push_str(&self.format_title(&menu.menu.title));
        output.push('\n');

        // Options with aligned descriptions
        let max_cmd_len = options
            .iter()
            .map(|opt| opt.command.len())
            .max()
            .unwrap_or(15);

        for option in options {
            output.push_str(&format!(
                "{:<width$} - {}\n",
                option.command,
                option.description,
                width = max_cmd_len
            ));
        }

        output
    }

    /// Format title with decorative border
    fn format_title(&self, title: &str) -> String {
        let border = "=".repeat(self.width.min(80));
        format!("{}\n{}\n{}", border, self.center_text(title), border)
    }

    /// Center text within the menu width
    fn center_text(&self, text: &str) -> String {
        let width = self.width.min(80);
        let padding = (width.saturating_sub(text.len())) / 2;
        format!("{}{}", " ".repeat(padding), text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{MenuMetadata, MenuParser};

    fn create_test_menu() -> MenuDefinition {
        MenuDefinition {
            menu: MenuMetadata {
                name: "test".to_string(),
                title: "Test Menu".to_string(),
                ansi_art: None,
                mode: MenuMode::Hotkey,
                inherits: None,
            },
            option: vec![
                MenuOption {
                    key: "F".to_string(),
                    command: "files".to_string(),
                    description: "File Areas".to_string(),
                    min_security: 0,
                    max_security: None,
                },
                MenuOption {
                    key: "M".to_string(),
                    command: "messages".to_string(),
                    description: "Message Areas".to_string(),
                    min_security: 10,
                    max_security: None,
                },
                MenuOption {
                    key: "A".to_string(),
                    command: "admin".to_string(),
                    description: "Admin Panel".to_string(),
                    min_security: 100,
                    max_security: Some(255),
                },
            ],
        }
    }

    #[test]
    fn test_render_hotkey_mode() {
        let renderer = MenuRenderer::new();
        let menu = create_test_menu();

        let rendered = renderer.render(&menu, 50);

        assert_eq!(rendered.title, "Test Menu");
        assert!(rendered.content.contains("(F) File Areas"));
        assert!(rendered.content.contains("(M) Message Areas"));
        assert_eq!(rendered.prompt, "Command: ");
        assert!(rendered.valid_keys.contains(&"F".to_string()));
        assert!(rendered.valid_keys.contains(&"M".to_string()));
    }

    #[test]
    fn test_render_fullmenu_mode() {
        let renderer = MenuRenderer::new();
        let mut menu = create_test_menu();
        menu.menu.mode = MenuMode::Fullmenu;

        let rendered = renderer.render(&menu, 50);

        assert!(rendered.content.contains("files"));
        assert!(rendered.content.contains("messages"));
        assert!(rendered.content.contains(" - "));
        assert_eq!(rendered.prompt, "Enter command: ");
    }

    #[test]
    fn test_filter_options_by_security() {
        let renderer = MenuRenderer::new();
        let menu = create_test_menu();

        // Security level 5: should see only first option (min_security = 0)
        let filtered = renderer.filter_options(&menu.option, 5);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].key, "F");

        // Security level 50: should see first two options
        let filtered = renderer.filter_options(&menu.option, 50);
        assert_eq!(filtered.len(), 2);

        // Security level 150: should see all options
        let filtered = renderer.filter_options(&menu.option, 150);
        assert_eq!(filtered.len(), 3);
    }

    #[test]
    fn test_filter_options_with_max_security() {
        let renderer = MenuRenderer::new();
        let menu = MenuDefinition {
            menu: MenuMetadata {
                name: "test".to_string(),
                title: "Test".to_string(),
                ansi_art: None,
                mode: MenuMode::Hotkey,
                inherits: None,
            },
            option: vec![MenuOption {
                key: "X".to_string(),
                command: "restricted".to_string(),
                description: "Restricted".to_string(),
                min_security: 10,
                max_security: Some(50),
            }],
        };

        // Should see option at security 30 (within range)
        let filtered = renderer.filter_options(&menu.option, 30);
        assert_eq!(filtered.len(), 1);

        // Should not see option at security 5 (below min)
        let filtered = renderer.filter_options(&menu.option, 5);
        assert_eq!(filtered.len(), 0);

        // Should not see option at security 100 (above max)
        let filtered = renderer.filter_options(&menu.option, 100);
        assert_eq!(filtered.len(), 0);
    }

    #[test]
    fn test_format_hotkey() {
        let renderer = MenuRenderer::new();
        let menu = create_test_menu();
        let options = renderer.filter_options(&menu.option, 255);

        let formatted = renderer.format_hotkey(&menu, &options);

        assert!(formatted.contains("Test Menu"));
        assert!(formatted.contains("(F) File Areas"));
        assert!(formatted.contains("(M) Message Areas"));
        assert!(formatted.contains("(A) Admin Panel"));
    }

    #[test]
    fn test_format_fullmenu() {
        let renderer = MenuRenderer::new();
        let menu = create_test_menu();
        let options = renderer.filter_options(&menu.option, 255);

        let formatted = renderer.format_fullmenu(&menu, &options);

        assert!(formatted.contains("Test Menu"));
        assert!(formatted.contains("files"));
        assert!(formatted.contains("messages"));
        assert!(formatted.contains("admin"));
        assert!(formatted.contains(" - "));
    }

    #[test]
    fn test_center_text() {
        let renderer = MenuRenderer::with_width(80);

        let centered = renderer.center_text("Test");
        assert!(centered.starts_with("    ")); // Should have padding

        let centered = renderer.center_text("A very long text that should be centered");
        assert!(centered.len() >= 40);
    }

    #[test]
    fn test_format_title() {
        let renderer = MenuRenderer::with_width(40);

        let title = renderer.format_title("Test Menu");

        assert!(title.contains("="));
        assert!(title.contains("Test Menu"));
        assert_eq!(title.lines().count(), 3); // Border, title, border
    }

    #[test]
    fn test_valid_keys_uppercase() {
        let renderer = MenuRenderer::new();
        let menu = create_test_menu();

        let rendered = renderer.render(&menu, 255);

        // All keys should be uppercase
        for key in &rendered.valid_keys {
            assert_eq!(key, &key.to_uppercase());
        }
    }

    #[test]
    fn test_custom_width() {
        let renderer = MenuRenderer::with_width(60);
        assert_eq!(renderer.width, 60);

        let renderer = MenuRenderer::with_width(100);
        assert_eq!(renderer.width, 100);
    }

    #[test]
    fn test_empty_menu() {
        let renderer = MenuRenderer::new();
        let menu = MenuDefinition {
            menu: MenuMetadata {
                name: "empty".to_string(),
                title: "Empty Menu".to_string(),
                ansi_art: None,
                mode: MenuMode::Hotkey,
                inherits: None,
            },
            option: vec![],
        };

        let rendered = renderer.render(&menu, 255);

        assert_eq!(rendered.title, "Empty Menu");
        assert!(rendered.valid_keys.is_empty());
    }

    #[test]
    fn test_integration_with_parser() {
        let toml = r#"
[menu]
name = "main"
title = "Main Menu"
mode = "hotkey"

[[option]]
key = "F"
command = "files"
description = "File Areas"
min_security = 0

[[option]]
key = "M"
command = "messages"
description = "Messages"
min_security = 10
"#;

        let menu = MenuParser::parse(toml).unwrap();
        let renderer = MenuRenderer::new();
        let rendered = renderer.render(&menu, 50);

        assert_eq!(rendered.title, "Main Menu");
        assert_eq!(rendered.valid_keys.len(), 2);
    }
}
