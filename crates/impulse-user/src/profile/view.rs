//! Profile viewing with privacy enforcement
//!
//! This module provides the `ProfileViewer` for viewing user profiles with
//! proper privacy settings enforcement.

use crate::{UserManager, privacy::PrivacySettings};
use impulse_types::{
    error::Result,
    user::{User, UserId},
};

/// Profile viewer with privacy enforcement
pub struct ProfileViewer<M: UserManager> {
    manager: M,
}

impl<M: UserManager> ProfileViewer<M> {
    /// Create a new profile viewer
    #[must_use]
    pub fn new(manager: M) -> Self {
        ProfileViewer { manager }
    }

    /// View own profile (no privacy filtering)
    ///
    /// # Errors
    ///
    /// Returns error if user not found.
    pub async fn view_own_profile(&self, user_id: UserId) -> Result<User> {
        self.manager.get_user(user_id).await
    }

    /// View another user's profile with privacy enforcement
    ///
    /// # Arguments
    ///
    /// * `target_user_id` - ID of user being viewed
    /// * `viewer_user_id` - ID of user viewing the profile
    /// * `privacy` - Privacy settings to enforce
    ///
    /// # Errors
    ///
    /// Returns error if either user not found.
    pub async fn view_user_profile(
        &self,
        target_user_id: UserId,
        viewer_user_id: UserId,
        privacy: &PrivacySettings,
    ) -> Result<User> {
        let target = self.manager.get_user(target_user_id).await?;
        let viewer = self.manager.get_user(viewer_user_id).await?;

        // Apply privacy filtering
        let filtered = privacy.apply(&target, viewer.is_sysop());

        tracing::debug!(
            target_user_id = ?target_user_id,
            viewer_user_id = ?viewer_user_id,
            is_sysop = viewer.is_sysop(),
            "Profile viewed with privacy enforcement"
        );

        Ok(filtered)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::InMemoryUserManager;
    use impulse_types::security::SecurityLevel;

    #[tokio::test]
    async fn test_view_own_profile() {
        let mut manager = InMemoryUserManager::new();
        let mut user = User::new("testuser").unwrap();
        user.email = Some("test@example.com".to_string());
        let user_id = user.id();
        manager.create_user(user).await.unwrap();

        let viewer = ProfileViewer::new(manager);
        let profile = viewer.view_own_profile(user_id).await.unwrap();

        assert_eq!(profile.email, Some("test@example.com".to_string()));
    }

    #[tokio::test]
    async fn test_view_user_profile_with_privacy() {
        let mut manager = InMemoryUserManager::new();

        let mut target = User::new("target").unwrap();
        target.email = Some("target@example.com".to_string());
        let target_id = target.id();
        manager.create_user(target).await.unwrap();

        let viewer = User::new("viewer").unwrap();
        let viewer_id = viewer.id();
        manager.create_user(viewer).await.unwrap();

        let profile_viewer = ProfileViewer::new(manager);
        let privacy = PrivacySettings::default(); // hide_email = true

        let filtered = profile_viewer
            .view_user_profile(target_id, viewer_id, &privacy)
            .await
            .unwrap();

        assert_eq!(filtered.email, None); // Email hidden
    }

    #[tokio::test]
    async fn test_view_user_profile_sysop_bypass() {
        let mut manager = InMemoryUserManager::new();

        let mut target = User::new("target").unwrap();
        target.email = Some("target@example.com".to_string());
        let target_id = target.id();
        manager.create_user(target).await.unwrap();

        let mut sysop = User::new("sysop").unwrap();
        sysop.set_security_level(SecurityLevel::SYSOP);
        let sysop_id = sysop.id();
        manager.create_user(sysop).await.unwrap();

        let profile_viewer = ProfileViewer::new(manager);
        let privacy = PrivacySettings::all_hidden(); // Everything hidden

        let filtered = profile_viewer
            .view_user_profile(target_id, sysop_id, &privacy)
            .await
            .unwrap();

        assert_eq!(filtered.email, Some("target@example.com".to_string())); // SysOp bypasses privacy
    }
}
