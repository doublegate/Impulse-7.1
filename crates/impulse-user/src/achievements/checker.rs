//! Achievement condition checking
//!
//! This module provides the `AchievementChecker` for verifying if users have
//! earned achievements based on their statistics.

use super::types::{Achievement, UserAchievement};
use impulse_types::user::User;
use std::time::SystemTime;

/// Achievement condition checker
pub struct AchievementChecker;

impl AchievementChecker {
    /// Check all achievements for a user and return newly earned ones
    ///
    /// # Arguments
    ///
    /// * `user` - The user to check
    /// * `current_achievements` - Achievements already earned by the user
    ///
    /// # Returns
    ///
    /// Vector of newly earned achievements
    pub fn check_all(user: &User, current_achievements: &[UserAchievement]) -> Vec<Achievement> {
        let current: Vec<Achievement> = current_achievements
            .iter()
            .map(|ua| ua.achievement)
            .collect();

        Achievement::all()
            .into_iter()
            .filter(|achievement| {
                !current.contains(achievement) && Self::check_achievement(user, *achievement)
            })
            .collect()
    }

    /// Check if a user has earned a specific achievement
    fn check_achievement(user: &User, achievement: Achievement) -> bool {
        match achievement {
            Achievement::FirstPost => user.stats.posts >= 1,
            Achievement::Upload10 => user.stats.uploads >= 10,
            Achievement::Upload100 => user.stats.uploads >= 100,
            Achievement::Upload1000 => user.stats.uploads >= 1000,
            Achievement::Download10 => user.stats.downloads >= 10,
            Achievement::Download100 => user.stats.downloads >= 100,
            Achievement::Download1000 => user.stats.downloads >= 1000,
            Achievement::Calls10 => user.stats.logins >= 10,
            Achievement::Calls100 => user.stats.logins >= 100,
            Achievement::Calls1000 => user.stats.logins >= 1000,
            Achievement::Member1Month => Self::check_membership_days(user, 30),
            Achievement::Member6Months => Self::check_membership_days(user, 180),
            Achievement::LoyalMember => Self::check_membership_days(user, 365),
            Achievement::VeteranMember => Self::check_membership_days(user, 1825),
            Achievement::Posts100 => user.stats.posts >= 100,
            Achievement::Posts500 => user.stats.posts >= 500,
            Achievement::Posts1000 => user.stats.posts >= 1000,
            Achievement::Time10Hours => user.stats.total_time_minutes >= 600,
            Achievement::Time100Hours => user.stats.total_time_minutes >= 6000,
            Achievement::Time1000Hours => user.stats.total_time_minutes >= 60000,
            Achievement::Ratio1to1 => user.stats.ul_dl_ratio().map(|r| r >= 1.0).unwrap_or(false),
            Achievement::Ratio1to2 => user.stats.ul_dl_ratio().map(|r| r >= 0.5).unwrap_or(false),
            Achievement::Email50 => user.stats.emails_sent >= 50,
            Achievement::FilePoints1000 => user.stats.file_points >= 1000,
            Achievement::FilePoints10000 => user.stats.file_points >= 10000,
        }
    }

    fn check_membership_days(user: &User, required_days: u64) -> bool {
        let now = SystemTime::now();
        if let Ok(duration) = now.duration_since(user.created_at) {
            let days = duration.as_secs() / 86400;
            days >= required_days
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_first_post() {
        let mut user = User::new("testuser").unwrap();
        assert!(!AchievementChecker::check_achievement(
            &user,
            Achievement::FirstPost
        ));

        user.stats.record_post();
        assert!(AchievementChecker::check_achievement(
            &user,
            Achievement::FirstPost
        ));
    }

    #[test]
    fn test_check_uploads() {
        let mut user = User::new("testuser").unwrap();
        user.stats.record_upload(10, 1000);

        assert!(AchievementChecker::check_achievement(
            &user,
            Achievement::Upload10
        ));
        assert!(!AchievementChecker::check_achievement(
            &user,
            Achievement::Upload100
        ));
    }

    #[test]
    fn test_check_ratio() {
        let mut user = User::new("testuser").unwrap();
        user.stats.record_upload(1, 1000);
        user.stats.record_download(1, 1000);

        assert!(AchievementChecker::check_achievement(
            &user,
            Achievement::Ratio1to1
        ));
        assert!(AchievementChecker::check_achievement(
            &user,
            Achievement::Ratio1to2
        ));
    }

    #[test]
    fn test_check_all() {
        let mut user = User::new("testuser").unwrap();
        user.stats.record_post();
        user.stats.record_upload(10, 1000);

        let current = vec![];
        let newly_earned = AchievementChecker::check_all(&user, &current);

        assert!(newly_earned.contains(&Achievement::FirstPost));
        assert!(newly_earned.contains(&Achievement::Upload10));
        assert!(!newly_earned.contains(&Achievement::Upload100));
    }

    #[test]
    fn test_check_all_filters_existing() {
        let mut user = User::new("testuser").unwrap();
        user.stats.record_post();

        let current = vec![UserAchievement::new(user.id(), Achievement::FirstPost)];
        let newly_earned = AchievementChecker::check_all(&user, &current);

        assert!(!newly_earned.contains(&Achievement::FirstPost));
    }
}
