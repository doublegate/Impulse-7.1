//! Theme loading from TOML files

use crate::error::{Result, TerminalError};
use std::path::{Path, PathBuf};
use tokio::fs;

use super::types::Theme;

/// Theme loader for loading themes from TOML files
pub struct ThemeLoader {
    cache: std::collections::HashMap<String, Theme>,
}

impl ThemeLoader {
    /// Create a new theme loader
    pub fn new() -> Self {
        Self {
            cache: std::collections::HashMap::new(),
        }
    }

    /// Load a theme from a TOML file
    pub async fn load_theme<P: AsRef<Path>>(&mut self, path: P) -> Result<Theme> {
        let path = path.as_ref();
        let content = fs::read_to_string(path).await.map_err(TerminalError::Io)?;

        self.parse_theme(&content, path)
    }

    /// Parse theme from TOML string
    fn parse_theme(&mut self, content: &str, _path: &Path) -> Result<Theme> {
        let theme: Theme =
            toml::from_str(content).map_err(|e| TerminalError::TomlParse(e.to_string()))?;

        // Validate the theme
        theme.validate().map_err(TerminalError::ThemeValidation)?;

        // Cache the theme
        self.cache
            .insert(theme.metadata.name.clone(), theme.clone());

        Ok(theme)
    }

    /// Load all themes from a directory
    pub async fn load_themes_from_dir<P: AsRef<Path>>(&mut self, dir: P) -> Result<Vec<Theme>> {
        let dir = dir.as_ref();
        if !dir.exists() {
            return Err(TerminalError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Theme directory not found: {}", dir.display()),
            )));
        }

        let mut themes = Vec::new();
        let mut entries = fs::read_dir(dir).await.map_err(TerminalError::Io)?;

        while let Some(entry) = entries.next_entry().await.map_err(TerminalError::Io)? {
            let path = entry.path();

            // Check if it's a directory
            if path.is_dir() {
                // Look for theme.toml in the directory
                let theme_file = path.join("theme.toml");
                if theme_file.exists() {
                    match self.load_theme(&theme_file).await {
                        Ok(theme) => themes.push(theme),
                        Err(e) => {
                            eprintln!(
                                "Warning: Failed to load theme from {}: {}",
                                theme_file.display(),
                                e
                            );
                        }
                    }
                }
            }
        }

        Ok(themes)
    }

    /// Get a cached theme by name
    pub fn get_cached(&self, name: &str) -> Option<&Theme> {
        self.cache.get(name)
    }

    /// Clear the cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// Get number of cached themes
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }

    /// Validate a theme file without loading it into cache
    pub async fn validate_theme_file<P: AsRef<Path>>(path: P) -> Result<()> {
        let path = path.as_ref();
        let content = fs::read_to_string(path).await.map_err(TerminalError::Io)?;

        let theme: Theme =
            toml::from_str(&content).map_err(|e| TerminalError::TomlParse(e.to_string()))?;

        theme.validate().map_err(TerminalError::ThemeValidation)?;

        Ok(())
    }
}

impl Default for ThemeLoader {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to find theme directories
#[allow(dead_code)]
pub async fn find_theme_dirs<P: AsRef<Path>>(base_dir: P) -> Result<Vec<PathBuf>> {
    let base_dir = base_dir.as_ref();
    let mut theme_dirs = Vec::new();

    if !base_dir.exists() {
        return Ok(theme_dirs);
    }

    let mut entries = fs::read_dir(base_dir).await.map_err(TerminalError::Io)?;

    while let Some(entry) = entries.next_entry().await.map_err(TerminalError::Io)? {
        let path = entry.path();
        if path.is_dir() && path.join("theme.toml").exists() {
            theme_dirs.push(path);
        }
    }

    Ok(theme_dirs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::{ColorScheme, ThemeMetadata};

    #[test]
    fn test_theme_loader_creation() {
        let loader = ThemeLoader::new();
        assert_eq!(loader.cache_size(), 0);
    }

    #[test]
    fn test_theme_loader_cache() {
        let mut loader = ThemeLoader::new();
        let theme = Theme::new(
            ThemeMetadata::new(
                "Test".to_string(),
                "Author".to_string(),
                "1.0".to_string(),
                "Desc".to_string(),
            ),
            ColorScheme::default(),
        );

        loader.cache.insert("Test".to_string(), theme);
        assert_eq!(loader.cache_size(), 1);
        assert!(loader.get_cached("Test").is_some());
    }

    #[test]
    fn test_theme_loader_clear_cache() {
        let mut loader = ThemeLoader::new();
        let theme = Theme::new(
            ThemeMetadata::new(
                "Test".to_string(),
                "Author".to_string(),
                "1.0".to_string(),
                "Desc".to_string(),
            ),
            ColorScheme::default(),
        );

        loader.cache.insert("Test".to_string(), theme);
        assert_eq!(loader.cache_size(), 1);

        loader.clear_cache();
        assert_eq!(loader.cache_size(), 0);
    }

    #[tokio::test]
    async fn test_parse_theme_toml() {
        let toml_content = r#"
[metadata]
name = "Test Theme"
author = "Test Author"
version = "1.0.0"
description = "A test theme"
compatible_bbs_version = ">=0.1.0"
requires_ansi = true
requires_utf8 = false

[color_scheme]
primary = { foreground = "bright_blue", background = "black", bold = false, blink = false }
secondary = { foreground = "cyan", background = "black", bold = false, blink = false }
accent = { foreground = "bright_cyan", background = "black", bold = true, blink = false }
background = { foreground = "white", background = "black", bold = false, blink = false }
text = { foreground = "white", background = "black", bold = false, blink = false }
highlight = { foreground = "bright_yellow", background = "black", bold = true, blink = false }
error = { foreground = "bright_red", background = "black", bold = true, blink = false }
success = { foreground = "bright_green", background = "black", bold = true, blink = false }
"#;

        let mut loader = ThemeLoader::new();
        let theme = loader
            .parse_theme(toml_content, Path::new("test.toml"))
            .unwrap();

        assert_eq!(theme.metadata.name, "Test Theme");
        assert_eq!(theme.metadata.author, "Test Author");
    }

    #[tokio::test]
    async fn test_parse_invalid_toml() {
        let invalid_toml = "this is not valid toml {[}";

        let mut loader = ThemeLoader::new();
        let result = loader.parse_theme(invalid_toml, Path::new("test.toml"));

        assert!(result.is_err());
        if let Err(TerminalError::TomlParse(_)) = result {
            // Expected error
        } else {
            panic!("Expected TomlParse error");
        }
    }

    #[tokio::test]
    async fn test_load_nonexistent_directory() {
        let mut loader = ThemeLoader::new();
        let result = loader
            .load_themes_from_dir("/nonexistent/path/to/themes")
            .await;

        assert!(result.is_err());
    }
}
