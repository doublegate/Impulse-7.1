//! User achievements and badges system
//!
//! This module provides an achievement tracking system with automated condition
//! checking and user notifications.

pub mod checker;
pub mod notify;
pub mod types;

pub use checker::AchievementChecker;
pub use notify::{AchievementNotification, NotificationManager};
pub use types::{Achievement, AchievementProgress, UserAchievement};
