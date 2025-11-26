//! Settings management implementation
//!
//! This module provides the `SettingsManager` for updating user preferences
//! through the UserManager trait.

use crate::UserManager;
use impulse_types::{
    error::{Error, Result},
    user::UserId,
    user_prefs::UserPreferences,
};

/// Settings manager for updating user preferences
pub struct SettingsManager<M: UserManager> {
    manager: M,
}

impl<M: UserManager> SettingsManager<M> {
    /// Create a new settings manager
    #[must_use]
    pub fn new(manager: M) -> Self {
        SettingsManager { manager }
    }

    /// Update user preferences
    ///
    /// # Errors
    ///
    /// Returns error if user not found or update fails.
    pub async fn update_preferences(
        &mut self,
        user_id: UserId,
        preferences: UserPreferences,
    ) -> Result<()> {
        let mut user = self.manager.get_user(user_id).await?;
        user.preferences = preferences;
        self.manager.update_user(user).await?;

        tracing::info!(user_id = ?user_id, "Preferences updated");
        Ok(())
    }

    /// Set terminal dimensions
    ///
    /// # Errors
    ///
    /// Returns error if dimensions invalid or update fails.
    pub async fn set_terminal_dimensions(
        &mut self,
        user_id: UserId,
        columns: u8,
        rows: u8,
    ) -> Result<()> {
        let mut user = self.manager.get_user(user_id).await?;

        if !user.preferences.set_dimensions(columns, rows) {
            return Err(Error::Validation("Invalid terminal dimensions".to_string()));
        }

        self.manager.update_user(user).await?;

        tracing::info!(
            user_id = ?user_id,
            columns = columns,
            rows = rows,
            "Terminal dimensions updated"
        );
        Ok(())
    }

    /// Enable or disable graphics
    ///
    /// # Errors
    ///
    /// Returns error if user not found or update fails.
    pub async fn set_graphics(&mut self, user_id: UserId, enabled: bool) -> Result<()> {
        let mut user = self.manager.get_user(user_id).await?;

        if enabled {
            user.preferences.enable_graphics();
        } else {
            user.preferences.disable_graphics();
        }

        self.manager.update_user(user).await?;

        tracing::info!(
            user_id = ?user_id,
            enabled = enabled,
            "Graphics settings updated"
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::InMemoryUserManager;
    use impulse_types::user::User;

    #[tokio::test]
    async fn test_update_preferences() {
        let mut manager = InMemoryUserManager::new();
        let user = User::new("testuser").unwrap();
        let user_id = user.id();
        manager.create_user(user).await.unwrap();

        let mut settings_mgr = SettingsManager::new(manager);
        let prefs = UserPreferences::expert();
        settings_mgr
            .update_preferences(user_id, prefs.clone())
            .await
            .unwrap();

        let updated = settings_mgr.manager.get_user(user_id).await.unwrap();
        assert_eq!(updated.preferences, prefs);
    }

    #[tokio::test]
    async fn test_set_terminal_dimensions() {
        let mut manager = InMemoryUserManager::new();
        let user = User::new("testuser").unwrap();
        let user_id = user.id();
        manager.create_user(user).await.unwrap();

        let mut settings_mgr = SettingsManager::new(manager);
        settings_mgr
            .set_terminal_dimensions(user_id, 132, 43)
            .await
            .unwrap();

        let updated = settings_mgr.manager.get_user(user_id).await.unwrap();
        assert_eq!(updated.preferences.line_length, 132);
        assert_eq!(updated.preferences.page_length, 43);
    }

    #[tokio::test]
    async fn test_set_terminal_dimensions_invalid() {
        let mut manager = InMemoryUserManager::new();
        let user = User::new("testuser").unwrap();
        let user_id = user.id();
        manager.create_user(user).await.unwrap();

        let mut settings_mgr = SettingsManager::new(manager);
        let result = settings_mgr.set_terminal_dimensions(user_id, 20, 5).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_graphics() {
        let mut manager = InMemoryUserManager::new();
        let user = User::new("testuser").unwrap();
        let user_id = user.id();
        manager.create_user(user).await.unwrap();

        let mut settings_mgr = SettingsManager::new(manager);
        settings_mgr.set_graphics(user_id, false).await.unwrap();

        let updated = settings_mgr.manager.get_user(user_id).await.unwrap();
        assert!(!updated.preferences.ansi_enabled);
        assert!(!updated.preferences.color_enabled);
        assert!(!updated.preferences.avatar_enabled);
    }
}
