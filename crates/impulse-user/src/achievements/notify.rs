//! Achievement notification system
//!
//! This module provides functionality for notifying users when they earn achievements.

use super::types::Achievement;
use serde::{Deserialize, Serialize};

/// Achievement notification
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AchievementNotification {
    /// The achievement earned
    pub achievement: Achievement,

    /// Notification message
    pub message: String,
}

impl AchievementNotification {
    /// Create a new achievement notification
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_user::achievements::types::Achievement;
    /// use impulse_user::achievements::notify::AchievementNotification;
    ///
    /// let notification = AchievementNotification::new(Achievement::FirstPost);
    /// assert_eq!(notification.achievement, Achievement::FirstPost);
    /// assert!(notification.message.contains("Achievement Unlocked"));
    /// ```
    #[must_use]
    pub fn new(achievement: Achievement) -> Self {
        let message = format!(
            "\n*** Achievement Unlocked! ***\n\n\
             {} {}\n\
             {}\n\n\
             Press any key to continue...",
            achievement.badge(),
            achievement.name(),
            achievement.description()
        );

        AchievementNotification {
            achievement,
            message,
        }
    }

    /// Format as ANSI colored message
    #[must_use]
    pub fn format_ansi(&self) -> String {
        format!(
            "\x1b[1;33m\n*** Achievement Unlocked! ***\x1b[0m\n\n\
             \x1b[1;32m{} {}\x1b[0m\n\
             \x1b[0;37m{}\x1b[0m\n\n\
             \x1b[0;36mPress any key to continue...\x1b[0m",
            self.achievement.badge(),
            self.achievement.name(),
            self.achievement.description()
        )
    }
}

/// Notification manager for achievements
pub struct NotificationManager {
    notifications: Vec<AchievementNotification>,
}

impl NotificationManager {
    /// Create a new notification manager
    #[must_use]
    pub fn new() -> Self {
        NotificationManager {
            notifications: Vec::new(),
        }
    }

    /// Add a notification for an achievement
    pub fn notify(&mut self, achievement: Achievement) {
        self.notifications
            .push(AchievementNotification::new(achievement));
    }

    /// Get pending notifications and clear the queue
    pub fn take_pending(&mut self) -> Vec<AchievementNotification> {
        std::mem::take(&mut self.notifications)
    }

    /// Get the number of pending notifications
    #[must_use]
    pub fn pending_count(&self) -> usize {
        self.notifications.len()
    }

    /// Check if there are pending notifications
    #[must_use]
    pub fn has_pending(&self) -> bool {
        !self.notifications.is_empty()
    }
}

impl Default for NotificationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_achievement_notification_new() {
        let notification = AchievementNotification::new(Achievement::FirstPost);
        assert_eq!(notification.achievement, Achievement::FirstPost);
        assert!(notification.message.contains("Achievement Unlocked"));
        assert!(notification.message.contains("First Post"));
    }

    #[test]
    fn test_achievement_notification_format_ansi() {
        let notification = AchievementNotification::new(Achievement::FirstPost);
        let ansi = notification.format_ansi();
        assert!(ansi.contains("\x1b[")); // Contains ANSI codes
        assert!(ansi.contains("Achievement Unlocked"));
    }

    #[test]
    fn test_notification_manager_new() {
        let manager = NotificationManager::new();
        assert_eq!(manager.pending_count(), 0);
        assert!(!manager.has_pending());
    }

    #[test]
    fn test_notification_manager_notify() {
        let mut manager = NotificationManager::new();
        manager.notify(Achievement::FirstPost);

        assert_eq!(manager.pending_count(), 1);
        assert!(manager.has_pending());
    }

    #[test]
    fn test_notification_manager_take_pending() {
        let mut manager = NotificationManager::new();
        manager.notify(Achievement::FirstPost);
        manager.notify(Achievement::Upload10);

        assert_eq!(manager.pending_count(), 2);

        let pending = manager.take_pending();
        assert_eq!(pending.len(), 2);
        assert_eq!(manager.pending_count(), 0);
    }
}
