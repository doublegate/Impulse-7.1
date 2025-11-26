//! Integration tests for user profile workflows
//!
//! This module tests the complete user profile system including:
//! - Profile display with privacy enforcement
//! - User settings changes
//! - Statistics tracking
//! - Achievement awarding
//! - User directory search

use impulse_user::{
    achievements::{Achievement, AchievementTracker},
    profile::ProfileDisplay,
    settings::{PrivacySettings, UserSettings},
    stats::{Statistics, StatsTracker},
    InMemoryUserManager, UserManager,
};
use impulse_types::{security::SecurityLevel, user::User};

/// Setup a test user manager with sample users
async fn setup_user_manager() -> InMemoryUserManager {
    let mut manager = InMemoryUserManager::new();

    // Create test users
    let mut user1 = User::new("alice").expect("Failed to create user");
    user1.set_real_name("Alice Smith");
    user1.set_location("New York");
    user1.set_security_level(SecurityLevel::REGULAR_USER);

    let mut user2 = User::new("bob").expect("Failed to create user");
    user2.set_real_name("Bob Jones");
    user2.set_location("Los Angeles");
    user2.set_security_level(SecurityLevel::REGULAR_USER);

    let mut user3 = User::new("carol").expect("Failed to create user");
    user3.set_real_name("Carol White");
    user3.set_location("Chicago");
    user3.set_security_level(SecurityLevel::SYSOP);

    manager
        .create_user(user1)
        .await
        .expect("Failed to create user");
    manager
        .create_user(user2)
        .await
        .expect("Failed to create user");
    manager
        .create_user(user3)
        .await
        .expect("Failed to create user");

    manager
}

#[tokio::test]
async fn test_profile_display() {
    // Setup
    let manager = setup_user_manager().await;
    let user = manager
        .find_by_username("alice")
        .await
        .expect("Failed to find user")
        .expect("User should exist");

    // Create profile display with default privacy
    let privacy = PrivacySettings::default();
    let display = ProfileDisplay::new(&user, &privacy);

    // Verify public information is visible
    assert_eq!(display.username(), "alice");
    assert_eq!(display.security_level(), SecurityLevel::REGULAR_USER);

    // Real name should be visible with default privacy
    assert_eq!(display.real_name(), Some("Alice Smith"));

    // Location should be visible with default privacy
    assert_eq!(display.location(), Some("New York"));
}

#[tokio::test]
async fn test_privacy_settings() {
    // Setup
    let manager = setup_user_manager().await;
    let user = manager
        .find_by_username("alice")
        .await
        .expect("Failed to find user")
        .expect("User should exist");

    // Create strict privacy settings
    let mut privacy = PrivacySettings::default();
    privacy.set_hide_real_name(true);
    privacy.set_hide_location(true);
    privacy.set_hide_email(true);

    let display = ProfileDisplay::new(&user, &privacy);

    // Verify private information is hidden
    assert_eq!(display.username(), "alice"); // Username always visible
    assert_eq!(display.real_name(), None); // Real name hidden
    assert_eq!(display.location(), None); // Location hidden
}

#[tokio::test]
async fn test_user_settings_update() {
    // Setup
    let mut manager = setup_user_manager().await;
    let mut user = manager
        .find_by_username("alice")
        .await
        .expect("Failed to find user")
        .expect("User should exist");

    // Get current settings
    let mut settings = UserSettings::from_user(&user);

    // Update settings
    settings.set_screen_width(132);
    settings.set_screen_height(43);
    settings.set_color_enabled(false);

    // Apply settings back to user
    settings.apply_to_user(&mut user);

    // Update user in manager
    manager
        .update_user(user.clone())
        .await
        .expect("Failed to update user");

    // Verify settings were saved
    let updated_user = manager
        .find_by_username("alice")
        .await
        .expect("Failed to find user")
        .expect("User should exist");

    let updated_settings = UserSettings::from_user(&updated_user);
    assert_eq!(updated_settings.screen_width(), 132);
    assert_eq!(updated_settings.screen_height(), 43);
    assert_eq!(updated_settings.color_enabled(), false);
}

#[tokio::test]
async fn test_statistics_tracking() {
    // Setup
    let mut tracker = StatsTracker::new();

    // Track some activity
    tracker.record_login();
    tracker.record_login();
    tracker.record_message_post();
    tracker.record_message_read(5);
    tracker.record_file_upload(1024);
    tracker.record_file_download(2048);

    // Get statistics
    let stats = tracker.get_stats();

    assert_eq!(stats.login_count(), 2, "Should have 2 logins");
    assert_eq!(stats.messages_posted(), 1, "Should have 1 message posted");
    assert_eq!(stats.messages_read(), 5, "Should have 5 messages read");
    assert_eq!(stats.files_uploaded(), 1, "Should have 1 file uploaded");
    assert_eq!(stats.files_downloaded(), 1, "Should have 1 file downloaded");
    assert_eq!(
        stats.upload_bytes(),
        1024,
        "Should have 1024 bytes uploaded"
    );
    assert_eq!(
        stats.download_bytes(),
        2048,
        "Should have 2048 bytes downloaded"
    );
}

#[tokio::test]
async fn test_statistics_ratio_calculation() {
    // Setup
    let mut tracker = StatsTracker::new();

    // Upload 10 KB, download 5 KB (2:1 ratio)
    tracker.record_file_upload(10240);
    tracker.record_file_download(5120);

    let stats = tracker.get_stats();

    // Calculate upload/download ratio
    let ratio = stats.upload_bytes() as f64 / stats.download_bytes() as f64;
    assert!(
        (ratio - 2.0).abs() < 0.01,
        "Ratio should be approximately 2.0"
    );
}

#[tokio::test]
async fn test_achievement_tracking() {
    // Setup
    let mut tracker = AchievementTracker::new();

    // Award some achievements
    let ach1 = Achievement::new("first_post", "First Post", "Posted your first message");
    let ach2 = Achievement::new("10_posts", "10 Posts", "Posted 10 messages");
    let ach3 = Achievement::new("100_logins", "Dedicated User", "Logged in 100 times");

    tracker.award(&ach1).expect("Failed to award achievement");
    tracker.award(&ach2).expect("Failed to award achievement");

    // Get awarded achievements
    let awarded = tracker.get_achievements();
    assert_eq!(awarded.len(), 2, "Should have 2 achievements");

    // Check if specific achievement is awarded
    assert!(
        tracker.has_achievement("first_post"),
        "Should have first_post"
    );
    assert!(tracker.has_achievement("10_posts"), "Should have 10_posts");
    assert!(
        !tracker.has_achievement("100_logins"),
        "Should not have 100_logins"
    );
}

#[tokio::test]
async fn test_achievement_no_duplicates() {
    // Setup
    let mut tracker = AchievementTracker::new();
    let ach = Achievement::new("first_post", "First Post", "Posted your first message");

    // Award achievement twice
    tracker.award(&ach).expect("Failed to award achievement");
    let result = tracker.award(&ach);

    // Second award should fail (already awarded)
    assert!(
        result.is_err(),
        "Should not be able to award same achievement twice"
    );

    // Should still have exactly one achievement
    assert_eq!(tracker.get_achievements().len(), 1);
}

#[tokio::test]
async fn test_user_directory_listing() {
    // Setup
    let manager = setup_user_manager().await;

    // List all users
    let users = manager.list_users().await.expect("Failed to list users");

    assert_eq!(users.len(), 3, "Should have 3 users");

    // Verify users are present
    let usernames: Vec<&str> = users.iter().map(|u| u.username()).collect();
    assert!(usernames.contains(&"alice"));
    assert!(usernames.contains(&"bob"));
    assert!(usernames.contains(&"carol"));
}

#[tokio::test]
async fn test_user_search_by_location() {
    // Setup
    let manager = setup_user_manager().await;
    let users = manager.list_users().await.expect("Failed to list users");

    // Search for users in "New York"
    let ny_users: Vec<_> = users
        .iter()
        .filter(|u| u.location() == Some("New York"))
        .collect();

    assert_eq!(ny_users.len(), 1, "Should find 1 user in New York");
    assert_eq!(ny_users[0].username(), "alice");
}

#[tokio::test]
async fn test_user_search_by_security_level() {
    // Setup
    let manager = setup_user_manager().await;
    let users = manager.list_users().await.expect("Failed to list users");

    // Find sysops
    let sysops: Vec<_> = users
        .iter()
        .filter(|u| u.security_level() == SecurityLevel::SYSOP)
        .collect();

    assert_eq!(sysops.len(), 1, "Should find 1 sysop");
    assert_eq!(sysops[0].username(), "carol");

    // Find regular users
    let regulars: Vec<_> = users
        .iter()
        .filter(|u| u.security_level() == SecurityLevel::REGULAR_USER)
        .collect();

    assert_eq!(regulars.len(), 2, "Should find 2 regular users");
}

#[tokio::test]
async fn test_profile_complete_flow() {
    // Setup
    let mut manager = setup_user_manager().await;

    // Find user
    let mut user = manager
        .find_by_username("alice")
        .await
        .expect("Failed to find user")
        .expect("User should exist");

    // Display initial profile
    let privacy = PrivacySettings::default();
    let display1 = ProfileDisplay::new(&user, &privacy);
    assert_eq!(display1.real_name(), Some("Alice Smith"));

    // Update user settings
    let mut settings = UserSettings::from_user(&user);
    settings.set_screen_width(132);
    settings.apply_to_user(&mut user);

    // Update privacy settings
    let mut privacy = PrivacySettings::default();
    privacy.set_hide_real_name(true);

    // Track some activity
    let mut tracker = StatsTracker::new();
    tracker.record_login();
    tracker.record_message_post();

    // Update user
    manager
        .update_user(user.clone())
        .await
        .expect("Failed to update");

    // Display updated profile
    let updated_user = manager
        .find_by_username("alice")
        .await
        .expect("Failed to find user")
        .expect("User should exist");

    let display2 = ProfileDisplay::new(&updated_user, &privacy);

    // Real name should be hidden now
    assert_eq!(display2.real_name(), None);

    // Settings should be updated
    let updated_settings = UserSettings::from_user(&updated_user);
    assert_eq!(updated_settings.screen_width(), 132);

    // Statistics should be tracked
    let stats = tracker.get_stats();
    assert_eq!(stats.login_count(), 1);
    assert_eq!(stats.messages_posted(), 1);
}

#[tokio::test]
async fn test_statistics_time_tracking() {
    // Setup
    let mut tracker = StatsTracker::new();

    // Record session time
    tracker.record_session_duration(std::time::Duration::from_secs(600)); // 10 minutes
    tracker.record_session_duration(std::time::Duration::from_secs(900)); // 15 minutes

    let stats = tracker.get_stats();

    // Total time should be 25 minutes (1500 seconds)
    assert_eq!(
        stats.total_time_seconds(),
        1500,
        "Should have 1500 seconds total"
    );
}

#[tokio::test]
async fn test_achievement_progression() {
    // Setup
    let mut tracker = AchievementTracker::new();

    // Define achievement progression: 1, 10, 100 posts
    let ach1 = Achievement::new("1_post", "First Post", "Posted 1 message");
    let ach10 = Achievement::new("10_posts", "Regular Poster", "Posted 10 messages");
    let ach100 = Achievement::new("100_posts", "Prolific Writer", "Posted 100 messages");

    // Award achievements in order
    tracker.award(&ach1).expect("Failed to award");
    assert_eq!(tracker.get_achievements().len(), 1);

    tracker.award(&ach10).expect("Failed to award");
    assert_eq!(tracker.get_achievements().len(), 2);

    tracker.award(&ach100).expect("Failed to award");
    assert_eq!(tracker.get_achievements().len(), 3);

    // Verify all are present
    assert!(tracker.has_achievement("1_post"));
    assert!(tracker.has_achievement("10_posts"));
    assert!(tracker.has_achievement("100_posts"));
}
