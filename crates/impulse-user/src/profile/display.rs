//! Profile display formatting
//!
//! This module provides functions for formatting user profiles for display.

use crate::UserManager;
use crate::stats::display::format_stats_summary;
use impulse_types::{error::Result, user::UserId};

/// Profile display options
#[derive(Debug, Clone)]
pub struct ProfileDisplayOptions {
    /// Show statistics
    pub show_stats: bool,

    /// Show personal information
    pub show_personal: bool,

    /// Show signature
    pub show_signature: bool,

    /// Show achievements
    pub show_achievements: bool,
}

impl Default for ProfileDisplayOptions {
    fn default() -> Self {
        ProfileDisplayOptions {
            show_stats: true,
            show_personal: true,
            show_signature: true,
            show_achievements: true,
        }
    }
}

/// Format a full user profile for display
///
/// # Errors
///
/// Returns error if user not found.
pub async fn format_profile<M: UserManager>(
    manager: &M,
    user_id: UserId,
    options: &ProfileDisplayOptions,
) -> Result<String> {
    let user = manager.get_user(user_id).await?;

    let mut output = String::new();

    // Header
    output.push_str(&format!(
        "══════════════════════════════════════════════════════════════\n\
         User Profile: {}\n\
         ══════════════════════════════════════════════════════════════\n\n",
        user.username()
    ));

    // Personal information
    if options.show_personal {
        output.push_str("Personal Information:\n");
        output.push_str(&format!("  Username:     {}\n", user.username()));

        if let Some(real_name) = &user.real_name {
            output.push_str(&format!("  Real Name:    {}\n", real_name));
        }

        if let Some(email) = &user.email {
            output.push_str(&format!("  Email:        {}\n", email));
        }

        output.push_str(&format!(
            "  Security:     Level {}\n",
            user.security_level().value()
        ));
        output.push_str(&format!(
            "  Status:       {}\n",
            if user.is_active { "Active" } else { "Inactive" }
        ));
        output.push('\n');
    }

    // Statistics
    if options.show_stats {
        output.push_str("Statistics:\n");
        let stats_summary = format_stats_summary(&user.stats);
        for line in stats_summary.lines() {
            output.push_str(&format!("  {}\n", line));
        }
        output.push('\n');
    }

    // Signature
    if options.show_signature
        && let Some(signature) = &user.sysop_note
    {
        output.push_str(&format!("Signature:\n  {}\n\n", signature));
    }

    output.push_str("══════════════════════════════════════════════════════════════\n");

    Ok(output)
}

/// Format a compact user signature
pub fn format_signature(username: &str, signature: Option<&str>) -> String {
    match signature {
        Some(sig) => format!("--- {} ---\n{}", username, sig),
        None => format!("--- {} ---", username),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::InMemoryUserManager;
    use impulse_types::user::User;

    #[tokio::test]
    async fn test_format_profile() {
        let mut manager = InMemoryUserManager::new();
        let mut user = User::new("testuser").unwrap();
        user.real_name = Some("Test User".to_string());
        user.email = Some("test@example.com".to_string());
        user.stats.record_upload(10, 1000);
        let user_id = user.id();
        manager.create_user(user).await.unwrap();

        let options = ProfileDisplayOptions::default();
        let profile = format_profile(&manager, user_id, &options).await.unwrap();

        assert!(profile.contains("User Profile: testuser"));
        assert!(profile.contains("Test User"));
        assert!(profile.contains("test@example.com"));
        assert!(profile.contains("Statistics:"));
    }

    #[tokio::test]
    async fn test_format_profile_minimal() {
        let mut manager = InMemoryUserManager::new();
        let user = User::new("testuser").unwrap();
        let user_id = user.id();
        manager.create_user(user).await.unwrap();

        let options = ProfileDisplayOptions {
            show_stats: false,
            show_personal: true,
            show_signature: false,
            show_achievements: false,
        };
        let profile = format_profile(&manager, user_id, &options).await.unwrap();

        assert!(profile.contains("testuser"));
        assert!(!profile.contains("Statistics:"));
    }

    #[test]
    fn test_format_signature() {
        let sig = format_signature("testuser", Some("My cool signature"));
        assert!(sig.contains("testuser"));
        assert!(sig.contains("My cool signature"));

        let no_sig = format_signature("testuser", None);
        assert!(no_sig.contains("testuser"));
        assert!(!no_sig.contains("signature"));
    }
}
