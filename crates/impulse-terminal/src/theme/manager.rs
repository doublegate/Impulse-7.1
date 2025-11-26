//! Theme management and switching

use crate::error::{Result, TerminalError};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::loader::ThemeLoader;
use super::metadata::ThemeInfo;
use super::types::{ColorScheme, Theme};

/// Theme manager for loading, switching, and managing themes
pub struct ThemeManager {
    loader: Arc<RwLock<ThemeLoader>>,
    pub(crate) themes: Arc<RwLock<std::collections::HashMap<String, Theme>>>,
    current_theme: Arc<RwLock<String>>,
    theme_dir: PathBuf,
}

impl ThemeManager {
    /// Create a new theme manager
    pub async fn new(theme_dir: PathBuf) -> Result<Self> {
        let loader = Arc::new(RwLock::new(ThemeLoader::new()));
        let themes = Arc::new(RwLock::new(std::collections::HashMap::new()));
        let current_theme = Arc::new(RwLock::new(String::from("default")));

        let manager = Self {
            loader,
            themes,
            current_theme,
            theme_dir,
        };

        // Load themes from directory
        manager.reload_themes().await?;

        Ok(manager)
    }

    /// Reload all themes from the theme directory
    pub async fn reload_themes(&self) -> Result<()> {
        let mut loader = self.loader.write().await;
        let loaded_themes = loader.load_themes_from_dir(&self.theme_dir).await?;

        let mut themes = self.themes.write().await;
        themes.clear();

        for theme in loaded_themes {
            themes.insert(theme.metadata.name.clone(), theme);
        }

        // If no themes loaded, create a default theme
        if themes.is_empty() {
            let default_theme = Self::create_default_theme();
            themes.insert(default_theme.metadata.name.clone(), default_theme);
        }

        Ok(())
    }

    /// Switch to a different theme
    pub async fn switch_theme(&self, theme_name: &str) -> Result<()> {
        let themes = self.themes.read().await;

        if !themes.contains_key(theme_name) {
            return Err(TerminalError::ThemeNotFound(theme_name.to_string()));
        }

        let mut current = self.current_theme.write().await;
        *current = theme_name.to_string();

        Ok(())
    }

    /// Get the current theme
    pub async fn current_theme(&self) -> Theme {
        let current_name = self.current_theme.read().await;
        let themes = self.themes.read().await;

        themes
            .get(current_name.as_str())
            .cloned()
            .unwrap_or_else(Self::create_default_theme)
    }

    /// Get the current theme name
    pub async fn current_theme_name(&self) -> String {
        self.current_theme.read().await.clone()
    }

    /// Get a screen path from the current theme
    pub async fn get_screen(&self, screen_name: &str) -> Option<PathBuf> {
        let theme = self.current_theme().await;
        theme.get_screen(screen_name).map(|path| {
            if Path::new(path).is_absolute() {
                PathBuf::from(path)
            } else {
                self.theme_dir.join(path)
            }
        })
    }

    /// Get a prompt from the current theme
    pub async fn get_prompt(&self, prompt_id: &str) -> Option<String> {
        let theme = self.current_theme().await;
        theme.get_prompt(prompt_id).cloned()
    }

    /// List all available themes
    pub async fn list_themes(&self) -> Vec<ThemeInfo> {
        let themes = self.themes.read().await;
        let current_name = self.current_theme.read().await;

        themes
            .values()
            .map(|theme| {
                let is_active = theme.metadata.name == *current_name;
                ThemeInfo::from_metadata(&theme.metadata, is_active)
            })
            .collect()
    }

    /// Get the current color scheme
    pub async fn get_color_scheme(&self) -> ColorScheme {
        let theme = self.current_theme().await;
        theme.color_scheme
    }

    /// Check if a theme exists
    pub async fn theme_exists(&self, name: &str) -> bool {
        let themes = self.themes.read().await;
        themes.contains_key(name)
    }

    /// Get a specific theme by name
    pub async fn get_theme(&self, name: &str) -> Option<Theme> {
        let themes = self.themes.read().await;
        themes.get(name).cloned()
    }

    /// Create a default fallback theme
    fn create_default_theme() -> Theme {
        use super::metadata::ThemeMetadata;

        Theme::new(
            ThemeMetadata::new(
                "default".to_string(),
                "Impulse BBS".to_string(),
                "1.0.0".to_string(),
                "Default classic BBS theme".to_string(),
            ),
            ColorScheme::default(),
        )
    }

    /// Get the theme directory path
    pub fn theme_dir(&self) -> &PathBuf {
        &self.theme_dir
    }
}

use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::metadata::ThemeMetadata;

    #[tokio::test]
    async fn test_theme_manager_default_theme() {
        let temp_dir = std::env::temp_dir().join("impulse_test_themes_mgr");
        let _ = std::fs::create_dir_all(&temp_dir);

        let manager = ThemeManager::new(temp_dir.clone()).await.unwrap();
        let themes = manager.list_themes().await;

        assert!(!themes.is_empty());
        assert_eq!(themes[0].name, "default");

        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[tokio::test]
    async fn test_theme_manager_switch_theme() {
        let temp_dir = std::env::temp_dir().join("impulse_test_themes_switch");
        let _ = std::fs::create_dir_all(&temp_dir);

        let manager = ThemeManager::new(temp_dir.clone()).await.unwrap();

        // Should have default theme
        let current = manager.current_theme_name().await;
        assert_eq!(current, "default");

        // Add another theme manually
        {
            let mut themes = manager.themes.write().await;
            let new_theme = Theme::new(
                ThemeMetadata::new(
                    "matrix".to_string(),
                    "Neo".to_string(),
                    "1.0".to_string(),
                    "Matrix theme".to_string(),
                ),
                ColorScheme::default(),
            );
            themes.insert("matrix".to_string(), new_theme);
        }

        // Switch to new theme
        manager.switch_theme("matrix").await.unwrap();
        let current = manager.current_theme_name().await;
        assert_eq!(current, "matrix");

        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[tokio::test]
    async fn test_theme_manager_switch_nonexistent() {
        let temp_dir = std::env::temp_dir().join("impulse_test_themes_nonexist");
        let _ = std::fs::create_dir_all(&temp_dir);

        let manager = ThemeManager::new(temp_dir.clone()).await.unwrap();

        let result = manager.switch_theme("nonexistent").await;
        assert!(result.is_err());

        if let Err(TerminalError::ThemeNotFound(_)) = result {
            // Expected error
        } else {
            panic!("Expected ThemeNotFound error");
        }

        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[tokio::test]
    async fn test_theme_manager_get_color_scheme() {
        let temp_dir = std::env::temp_dir().join("impulse_test_themes_colors");
        let _ = std::fs::create_dir_all(&temp_dir);

        let manager = ThemeManager::new(temp_dir.clone()).await.unwrap();
        let colors = manager.get_color_scheme().await;

        // Should have default color scheme
        assert_eq!(
            colors.primary().foreground,
            super::super::types::ThemeColor::BrightBlue
        );

        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[tokio::test]
    async fn test_theme_manager_theme_exists() {
        let temp_dir = std::env::temp_dir().join("impulse_test_themes_exists");
        let _ = std::fs::create_dir_all(&temp_dir);

        let manager = ThemeManager::new(temp_dir.clone()).await.unwrap();

        assert!(manager.theme_exists("default").await);
        assert!(!manager.theme_exists("nonexistent").await);

        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[tokio::test]
    async fn test_theme_manager_get_theme() {
        let temp_dir = std::env::temp_dir().join("impulse_test_themes_get");
        let _ = std::fs::create_dir_all(&temp_dir);

        let manager = ThemeManager::new(temp_dir.clone()).await.unwrap();

        let theme = manager.get_theme("default").await;
        assert!(theme.is_some());
        assert_eq!(theme.unwrap().metadata.name, "default");

        let nonexistent = manager.get_theme("nonexistent").await;
        assert!(nonexistent.is_none());

        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[tokio::test]
    async fn test_theme_manager_list_themes() {
        let temp_dir = std::env::temp_dir().join("impulse_test_themes_list");
        let _ = std::fs::create_dir_all(&temp_dir);

        let manager = ThemeManager::new(temp_dir.clone()).await.unwrap();

        let themes = manager.list_themes().await;
        assert!(!themes.is_empty());

        // Check that current theme is marked as active
        let active_themes: Vec<_> = themes.iter().filter(|t| t.is_active).collect();
        assert_eq!(active_themes.len(), 1);

        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}
