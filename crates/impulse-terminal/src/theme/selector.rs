//! Theme selection UI

use crate::error::Result;
use std::sync::Arc;

use super::manager::ThemeManager;
use super::metadata::ThemeInfo;

/// Theme selector for interactive theme selection
pub struct ThemeSelector {
    manager: Arc<ThemeManager>,
}

impl ThemeSelector {
    /// Create a new theme selector
    pub fn new(manager: Arc<ThemeManager>) -> Self {
        Self { manager }
    }

    /// List all available themes
    pub async fn list_available_themes(&self) -> Vec<ThemeInfo> {
        self.manager.list_themes().await
    }

    /// Select a theme by name
    pub async fn select_theme(&self, name: &str) -> Result<()> {
        self.manager.switch_theme(name).await
    }

    /// Generate a selection menu display
    pub async fn generate_selection_menu(&self) -> String {
        let themes = self.list_available_themes().await;
        let mut menu = String::new();

        menu.push_str("\x1b[1;36m╔══════════════════════════════════════════════════╗\x1b[0m\n");
        menu.push_str("\x1b[1;36m║              THEME SELECTION                     ║\x1b[0m\n");
        menu.push_str("\x1b[1;36m╚══════════════════════════════════════════════════╝\x1b[0m\n\n");

        if themes.is_empty() {
            menu.push_str("\x1b[1;31mNo themes available.\x1b[0m\n");
            return menu;
        }

        for (idx, theme) in themes.iter().enumerate() {
            let number = format!("[{}]", idx + 1);
            let active_marker = if theme.is_active {
                "\x1b[1;32m ► \x1b[0m"
            } else {
                "   "
            };

            menu.push_str(&format!(
                "{}\x1b[1;33m{:<4}\x1b[0m \x1b[1;37m{:<20}\x1b[0m v{}\n",
                active_marker, number, theme.name, theme.version
            ));

            menu.push_str(&format!("     \x1b[0;36mby {:<18}\x1b[0m\n", theme.author));

            menu.push_str(&format!(
                "     \x1b[0;37m{}\x1b[0m\n\n",
                Self::truncate_description(&theme.description, 45)
            ));
        }

        menu.push_str("\x1b[1;36m══════════════════════════════════════════════════\x1b[0m\n");
        menu.push_str("\x1b[1;37mEnter number to select, [P] to preview, [Q] to quit:\x1b[0m ");

        menu
    }

    /// Generate a compact theme list
    pub async fn generate_compact_list(&self) -> String {
        let themes = self.list_available_themes().await;
        let mut list = String::new();

        list.push_str("\x1b[1;37mAvailable Themes:\x1b[0m\n");

        for theme in &themes {
            let marker = if theme.is_active { "* " } else { "  " };
            list.push_str(&format!(
                "{}\x1b[1;36m{}\x1b[0m (v{})\n",
                marker, theme.name, theme.version
            ));
        }

        list
    }

    /// Get the currently selected theme name
    pub async fn current_theme_name(&self) -> String {
        self.manager.current_theme_name().await
    }

    /// Check if a theme exists
    pub async fn theme_exists(&self, name: &str) -> bool {
        self.manager.theme_exists(name).await
    }

    /// Get theme count
    pub async fn theme_count(&self) -> usize {
        self.list_available_themes().await.len()
    }

    /// Truncate description to fit display width
    fn truncate_description(desc: &str, max_len: usize) -> String {
        if desc.len() <= max_len {
            desc.to_string()
        } else {
            format!("{}...", &desc[..max_len.saturating_sub(3)])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::metadata::ThemeMetadata;
    use crate::theme::{ColorScheme, Theme};

    async fn create_test_manager() -> Arc<ThemeManager> {
        let temp_dir =
            std::env::temp_dir().join(format!("impulse_test_selector_{}", rand::random::<u32>()));
        let _ = std::fs::create_dir_all(&temp_dir);

        let manager = ThemeManager::new(temp_dir).await.unwrap();

        // Add some test themes
        {
            let mut themes = manager.themes.write().await;

            let theme1 = Theme::new(
                ThemeMetadata::new(
                    "Classic".to_string(),
                    "BBS Author".to_string(),
                    "1.0.0".to_string(),
                    "Classic BBS theme with blue and white colors".to_string(),
                ),
                ColorScheme::default(),
            );

            let theme2 = Theme::new(
                ThemeMetadata::new(
                    "Matrix".to_string(),
                    "Neo".to_string(),
                    "2.0.0".to_string(),
                    "Green on black hacker theme".to_string(),
                ),
                ColorScheme::default(),
            );

            themes.insert("Classic".to_string(), theme1);
            themes.insert("Matrix".to_string(), theme2);
        }

        Arc::new(manager)
    }

    #[tokio::test]
    async fn test_selector_creation() {
        let manager = create_test_manager().await;
        let selector = ThemeSelector::new(manager);

        let themes = selector.list_available_themes().await;
        assert!(!themes.is_empty());
    }

    #[tokio::test]
    async fn test_list_available_themes() {
        let manager = create_test_manager().await;
        let selector = ThemeSelector::new(manager);

        let themes = selector.list_available_themes().await;
        assert!(themes.len() >= 2); // At least Classic and Matrix

        let theme_names: Vec<_> = themes.iter().map(|t| t.name.as_str()).collect();
        assert!(theme_names.contains(&"Classic"));
        assert!(theme_names.contains(&"Matrix"));
    }

    #[tokio::test]
    async fn test_select_theme() {
        let manager = create_test_manager().await;
        let selector = ThemeSelector::new(manager);

        let result = selector.select_theme("Matrix").await;
        assert!(result.is_ok());

        let current = selector.current_theme_name().await;
        assert_eq!(current, "Matrix");
    }

    #[tokio::test]
    async fn test_select_nonexistent_theme() {
        let manager = create_test_manager().await;
        let selector = ThemeSelector::new(manager);

        let result = selector.select_theme("Nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_generate_selection_menu() {
        let manager = create_test_manager().await;
        let selector = ThemeSelector::new(manager);

        let menu = selector.generate_selection_menu().await;

        assert!(menu.contains("THEME SELECTION"));
        assert!(menu.contains("Classic"));
        assert!(menu.contains("Matrix"));
        assert!(menu.contains("[1]"));
        assert!(menu.contains("[2]") || menu.contains("[3]")); // Depends on how many themes
    }

    #[tokio::test]
    async fn test_generate_compact_list() {
        let manager = create_test_manager().await;
        let selector = ThemeSelector::new(manager);

        let list = selector.generate_compact_list().await;

        assert!(list.contains("Available Themes"));
        assert!(list.contains("Classic"));
        assert!(list.contains("Matrix"));
    }

    #[tokio::test]
    async fn test_current_theme_name() {
        let manager = create_test_manager().await;
        let selector = ThemeSelector::new(manager);

        let current = selector.current_theme_name().await;
        assert!(!current.is_empty());
    }

    #[tokio::test]
    async fn test_theme_exists() {
        let manager = create_test_manager().await;
        let selector = ThemeSelector::new(manager);

        assert!(selector.theme_exists("Classic").await);
        assert!(selector.theme_exists("Matrix").await);
        assert!(!selector.theme_exists("Nonexistent").await);
    }

    #[tokio::test]
    async fn test_theme_count() {
        let manager = create_test_manager().await;
        let selector = ThemeSelector::new(manager);

        let count = selector.theme_count().await;
        assert!(count >= 2); // At least Classic and Matrix
    }

    #[test]
    fn test_truncate_description() {
        let desc = "This is a very long description that needs to be truncated";
        let truncated = ThemeSelector::truncate_description(desc, 20);

        assert!(truncated.len() <= 20);
        assert!(truncated.ends_with("..."));
    }

    #[test]
    fn test_truncate_short_description() {
        let desc = "Short desc";
        let truncated = ThemeSelector::truncate_description(desc, 20);

        assert_eq!(truncated, "Short desc");
        assert!(!truncated.ends_with("..."));
    }

    #[tokio::test]
    async fn test_menu_shows_active_marker() {
        let manager = create_test_manager().await;
        manager.switch_theme("Matrix").await.unwrap();

        let selector = ThemeSelector::new(manager);
        let menu = selector.generate_selection_menu().await;

        // Should contain active marker (►)
        assert!(menu.contains("►"));
    }
}
