//! Privacy settings and enforcement
//!
//! This module provides privacy controls for user profiles, allowing users to
//! hide specific information from other users while keeping it visible to themselves
//! and system operators.

use impulse_types::user::User;
use serde::{Deserialize, Serialize};

/// Privacy settings for user profiles
///
/// Controls what information is visible to other users when viewing a profile.
/// SysOps always bypass privacy settings.
///
/// # Examples
///
/// ```
/// use impulse_user::privacy::PrivacySettings;
///
/// let settings = PrivacySettings::default();
/// assert!(settings.hide_email); // Email hidden by default for privacy
/// assert!(!settings.hide_stats); // Stats visible by default
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrivacySettings {
    /// Hide email address from other users
    pub hide_email: bool,

    /// Hide real name from other users
    pub hide_real_name: bool,

    /// Hide statistics from other users
    pub hide_stats: bool,

    /// Hide online status from other users
    pub hide_online: bool,

    /// Hide last login time from other users
    pub hide_last_login: bool,

    /// Hide signature from other users
    pub hide_signature: bool,
}

impl Default for PrivacySettings {
    /// Default privacy settings
    ///
    /// By default, email is hidden but other information is visible.
    fn default() -> Self {
        PrivacySettings {
            hide_email: true,       // Email hidden by default for privacy
            hide_real_name: false,  // Real name visible
            hide_stats: false,      // Stats visible
            hide_online: false,     // Online status visible
            hide_last_login: false, // Last login visible
            hide_signature: false,  // Signature visible
        }
    }
}

impl PrivacySettings {
    /// Create new privacy settings with all fields visible
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_user::privacy::PrivacySettings;
    ///
    /// let settings = PrivacySettings::all_visible();
    /// assert!(!settings.hide_email);
    /// assert!(!settings.hide_real_name);
    /// assert!(!settings.hide_stats);
    /// ```
    #[must_use]
    pub fn all_visible() -> Self {
        PrivacySettings {
            hide_email: false,
            hide_real_name: false,
            hide_stats: false,
            hide_online: false,
            hide_last_login: false,
            hide_signature: false,
        }
    }

    /// Create new privacy settings with all fields hidden
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_user::privacy::PrivacySettings;
    ///
    /// let settings = PrivacySettings::all_hidden();
    /// assert!(settings.hide_email);
    /// assert!(settings.hide_real_name);
    /// assert!(settings.hide_stats);
    /// ```
    #[must_use]
    pub fn all_hidden() -> Self {
        PrivacySettings {
            hide_email: true,
            hide_real_name: true,
            hide_stats: true,
            hide_online: true,
            hide_last_login: true,
            hide_signature: true,
        }
    }

    /// Apply privacy settings to a user profile for viewing by another user
    ///
    /// Returns a sanitized clone of the user with hidden fields set to None.
    /// If the viewer is a SysOp, all fields are visible.
    ///
    /// # Arguments
    ///
    /// * `user` - The user whose profile is being viewed
    /// * `viewer_is_sysop` - Whether the viewer is a SysOp
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_user::privacy::PrivacySettings;
    /// use impulse_types::user::User;
    ///
    /// let mut user = User::new("testuser").unwrap();
    /// user.email = Some("test@example.com".to_string());
    ///
    /// let settings = PrivacySettings::default(); // hide_email = true
    /// let sanitized = settings.apply(&user, false);
    ///
    /// assert_eq!(sanitized.email, None); // Email hidden
    /// assert_eq!(sanitized.username(), "testuser"); // Username always visible
    /// ```
    #[must_use]
    pub fn apply(&self, user: &User, viewer_is_sysop: bool) -> User {
        // SysOps bypass all privacy settings
        if viewer_is_sysop {
            return user.clone();
        }

        let mut sanitized = user.clone();

        if self.hide_email {
            sanitized.email = None;
        }

        if self.hide_real_name {
            sanitized.real_name = None;
        }

        if self.hide_stats {
            sanitized.stats = Default::default();
        }

        if self.hide_last_login {
            sanitized.last_login = None;
        }

        if self.hide_signature {
            sanitized.sysop_note = None; // Using sysop_note as signature field
        }

        sanitized
    }

    /// Check if any privacy settings are enabled
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_user::privacy::PrivacySettings;
    ///
    /// let default_settings = PrivacySettings::default();
    /// assert!(default_settings.has_any_privacy()); // Email is hidden by default
    ///
    /// let all_visible = PrivacySettings::all_visible();
    /// assert!(!all_visible.has_any_privacy());
    /// ```
    #[must_use]
    pub fn has_any_privacy(&self) -> bool {
        self.hide_email
            || self.hide_real_name
            || self.hide_stats
            || self.hide_online
            || self.hide_last_login
            || self.hide_signature
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_privacy() {
        let settings = PrivacySettings::default();
        assert!(settings.hide_email); // Email hidden by default
        assert!(!settings.hide_real_name);
        assert!(!settings.hide_stats);
        assert!(!settings.hide_online);
        assert!(!settings.hide_last_login);
        assert!(!settings.hide_signature);
    }

    #[test]
    fn test_all_visible() {
        let settings = PrivacySettings::all_visible();
        assert!(!settings.hide_email);
        assert!(!settings.hide_real_name);
        assert!(!settings.hide_stats);
        assert!(!settings.hide_online);
        assert!(!settings.hide_last_login);
        assert!(!settings.hide_signature);
    }

    #[test]
    fn test_all_hidden() {
        let settings = PrivacySettings::all_hidden();
        assert!(settings.hide_email);
        assert!(settings.hide_real_name);
        assert!(settings.hide_stats);
        assert!(settings.hide_online);
        assert!(settings.hide_last_login);
        assert!(settings.hide_signature);
    }

    #[test]
    fn test_apply_hide_email() {
        let mut user = User::new("testuser").unwrap();
        user.email = Some("test@example.com".to_string());

        let settings = PrivacySettings {
            hide_email: true,
            ..Default::default()
        };

        let sanitized = settings.apply(&user, false);
        assert_eq!(sanitized.email, None);
        assert_eq!(sanitized.username(), "testuser"); // Username always visible
    }

    #[test]
    fn test_apply_hide_real_name() {
        let mut user = User::new("testuser").unwrap();
        user.real_name = Some("Test User".to_string());

        let settings = PrivacySettings {
            hide_real_name: true,
            ..Default::default()
        };

        let sanitized = settings.apply(&user, false);
        assert_eq!(sanitized.real_name, None);
    }

    #[test]
    fn test_apply_hide_stats() {
        let mut user = User::new("testuser").unwrap();
        user.stats.record_upload(10, 1000);
        user.stats.record_download(5, 500);

        let settings = PrivacySettings {
            hide_stats: true,
            ..Default::default()
        };

        let sanitized = settings.apply(&user, false);
        assert_eq!(sanitized.stats.uploads, 0);
        assert_eq!(sanitized.stats.downloads, 0);
    }

    #[test]
    fn test_apply_hide_last_login() {
        let mut user = User::new("testuser").unwrap();
        user.record_login(); // Sets last_login

        let settings = PrivacySettings {
            hide_last_login: true,
            ..Default::default()
        };

        let sanitized = settings.apply(&user, false);
        assert_eq!(sanitized.last_login, None);
    }

    #[test]
    fn test_sysop_bypass_privacy() {
        let mut user = User::new("testuser").unwrap();
        user.email = Some("test@example.com".to_string());
        user.real_name = Some("Test User".to_string());
        user.stats.record_upload(10, 1000);

        let settings = PrivacySettings::all_hidden();

        // SysOp can see everything
        let sanitized = settings.apply(&user, true);
        assert_eq!(sanitized.email, Some("test@example.com".to_string()));
        assert_eq!(sanitized.real_name, Some("Test User".to_string()));
        assert_eq!(sanitized.stats.uploads, 10);
    }

    #[test]
    fn test_has_any_privacy() {
        let all_visible = PrivacySettings::all_visible();
        assert!(!all_visible.has_any_privacy());

        let default_settings = PrivacySettings::default();
        assert!(default_settings.has_any_privacy()); // Email is hidden

        let all_hidden = PrivacySettings::all_hidden();
        assert!(all_hidden.has_any_privacy());
    }

    #[test]
    fn test_serialization() {
        let settings = PrivacySettings::default();
        let json = serde_json::to_string(&settings).unwrap();
        let deserialized: PrivacySettings = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, settings);
    }
}
