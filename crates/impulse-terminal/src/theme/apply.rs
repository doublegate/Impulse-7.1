//! Theme application and persistence

use crate::error::{Result, TerminalError};
use std::path::{Path, PathBuf};
use tokio::fs;

/// Apply a theme and persist user preference
pub async fn apply_theme(theme_name: &str, config_dir: &Path) -> Result<()> {
    // Validate theme name
    if theme_name.is_empty() {
        return Err(TerminalError::ThemeValidation(
            "Theme name cannot be empty".to_string(),
        ));
    }

    // Ensure config directory exists
    if !config_dir.exists() {
        fs::create_dir_all(config_dir)
            .await
            .map_err(TerminalError::Io)?;
    }

    // Write theme preference to config file
    let pref_file = config_dir.join("theme.txt");
    fs::write(&pref_file, theme_name)
        .await
        .map_err(TerminalError::Io)?;

    Ok(())
}

/// Load the saved theme preference
#[allow(dead_code)]
pub async fn load_theme_preference(config_dir: &Path) -> Result<Option<String>> {
    let pref_file = config_dir.join("theme.txt");

    if !pref_file.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(&pref_file)
        .await
        .map_err(TerminalError::Io)?;

    let theme_name = content.trim().to_string();

    if theme_name.is_empty() {
        Ok(None)
    } else {
        Ok(Some(theme_name))
    }
}

/// Clear the theme preference (revert to default)
#[allow(dead_code)]
pub async fn clear_theme_preference(config_dir: &Path) -> Result<()> {
    let pref_file = config_dir.join("theme.txt");

    if pref_file.exists() {
        fs::remove_file(&pref_file)
            .await
            .map_err(TerminalError::Io)?;
    }

    Ok(())
}

/// Check if a theme preference file exists
#[allow(dead_code)]
pub fn has_theme_preference(config_dir: &Path) -> bool {
    config_dir.join("theme.txt").exists()
}

/// Get the theme preference file path
#[allow(dead_code)]
pub fn get_preference_path(config_dir: &Path) -> PathBuf {
    config_dir.join("theme.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn create_test_config_dir() -> PathBuf {
        let dir = std::env::temp_dir().join(format!("impulse_test_apply_{}", rand::random::<u32>()));
        let _ = fs::create_dir_all(&dir).await;
        dir
    }

    #[tokio::test]
    async fn test_apply_theme() {
        let config_dir = create_test_config_dir().await;

        let result = apply_theme("matrix", &config_dir).await;
        assert!(result.is_ok());

        // Verify file was created
        let pref_file = config_dir.join("theme.txt");
        assert!(pref_file.exists());

        let content = fs::read_to_string(&pref_file).await.unwrap();
        assert_eq!(content, "matrix");

        let _ = fs::remove_dir_all(&config_dir).await;
    }

    #[tokio::test]
    async fn test_apply_empty_theme_name() {
        let config_dir = create_test_config_dir().await;

        let result = apply_theme("", &config_dir).await;
        assert!(result.is_err());

        if let Err(TerminalError::ThemeValidation(_)) = result {
            // Expected error
        } else {
            panic!("Expected ThemeValidation error");
        }

        let _ = fs::remove_dir_all(&config_dir).await;
    }

    #[tokio::test]
    async fn test_load_theme_preference() {
        let config_dir = create_test_config_dir().await;

        // Apply a theme first
        apply_theme("cyberpunk", &config_dir).await.unwrap();

        // Load preference
        let preference = load_theme_preference(&config_dir).await.unwrap();
        assert_eq!(preference, Some("cyberpunk".to_string()));

        let _ = fs::remove_dir_all(&config_dir).await;
    }

    #[tokio::test]
    async fn test_load_nonexistent_preference() {
        let config_dir = create_test_config_dir().await;

        let preference = load_theme_preference(&config_dir).await.unwrap();
        assert_eq!(preference, None);

        let _ = fs::remove_dir_all(&config_dir).await;
    }

    #[tokio::test]
    async fn test_clear_theme_preference() {
        let config_dir = create_test_config_dir().await;

        // Apply and then clear
        apply_theme("matrix", &config_dir).await.unwrap();
        assert!(has_theme_preference(&config_dir));

        clear_theme_preference(&config_dir).await.unwrap();
        assert!(!has_theme_preference(&config_dir));

        let _ = fs::remove_dir_all(&config_dir).await;
    }

    #[tokio::test]
    async fn test_clear_nonexistent_preference() {
        let config_dir = create_test_config_dir().await;

        // Should not error when clearing non-existent preference
        let result = clear_theme_preference(&config_dir).await;
        assert!(result.is_ok());

        let _ = fs::remove_dir_all(&config_dir).await;
    }

    #[test]
    fn test_has_theme_preference() {
        let temp_dir = std::env::temp_dir();
        let config_dir = temp_dir.join("nonexistent_config_dir");

        assert!(!has_theme_preference(&config_dir));
    }

    #[test]
    fn test_get_preference_path() {
        let config_dir = PathBuf::from("/test/config");
        let pref_path = get_preference_path(&config_dir);

        assert_eq!(pref_path, PathBuf::from("/test/config/theme.txt"));
    }

    #[tokio::test]
    async fn test_apply_creates_config_dir() {
        let base_dir = std::env::temp_dir().join(format!("impulse_test_apply_dir_{}", rand::random::<u32>()));
        let config_dir = base_dir.join("nested").join("config");

        // Config dir doesn't exist yet
        assert!(!config_dir.exists());

        let result = apply_theme("classic", &config_dir).await;
        assert!(result.is_ok());

        // Config dir should now exist
        assert!(config_dir.exists());
        assert!(config_dir.join("theme.txt").exists());

        let _ = fs::remove_dir_all(&base_dir).await;
    }

    #[tokio::test]
    async fn test_load_empty_preference_file() {
        let config_dir = create_test_config_dir().await;
        let pref_file = config_dir.join("theme.txt");

        // Create empty preference file
        fs::write(&pref_file, "").await.unwrap();

        let preference = load_theme_preference(&config_dir).await.unwrap();
        assert_eq!(preference, None);

        let _ = fs::remove_dir_all(&config_dir).await;
    }

    #[tokio::test]
    async fn test_load_whitespace_preference() {
        let config_dir = create_test_config_dir().await;
        let pref_file = config_dir.join("theme.txt");

        // Create preference file with whitespace
        fs::write(&pref_file, "  matrix  \n").await.unwrap();

        let preference = load_theme_preference(&config_dir).await.unwrap();
        assert_eq!(preference, Some("matrix".to_string()));

        let _ = fs::remove_dir_all(&config_dir).await;
    }
}
