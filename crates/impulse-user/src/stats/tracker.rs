//! Statistics tracking implementation
//!
//! This module provides the `StatsTracker` for updating user statistics through
//! the UserManager trait, enabling consistent tracking across the application.

use crate::UserManager;
use impulse_types::{error::Result, user::UserId};

/// Statistics tracker for updating user activity
///
/// Provides high-level methods for tracking user activity statistics by
/// updating the User record through a UserManager implementation.
///
/// # Examples
///
/// ```
/// use impulse_user::{StatsTracker, InMemoryUserManager, UserManager};
/// use impulse_types::user::User;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let mut manager = InMemoryUserManager::new();
/// let user = User::new("testuser")?;
/// let user_id = user.id();
/// manager.create_user(user).await?;
///
/// let mut tracker = StatsTracker::new(manager);
/// tracker.record_login(user_id).await?;
///
/// let updated = tracker.manager().get_user(user_id).await?;
/// assert_eq!(updated.stats.logins, 1);
/// # Ok(())
/// # }
/// ```
pub struct StatsTracker<M: UserManager> {
    manager: M,
}

impl<M: UserManager> StatsTracker<M> {
    /// Create a new stats tracker with the given user manager
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_user::{StatsTracker, InMemoryUserManager};
    ///
    /// let manager = InMemoryUserManager::new();
    /// let tracker = StatsTracker::new(manager);
    /// ```
    #[must_use]
    pub fn new(manager: M) -> Self {
        StatsTracker { manager }
    }

    /// Get a reference to the underlying user manager
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_user::{StatsTracker, InMemoryUserManager};
    ///
    /// let manager = InMemoryUserManager::new();
    /// let tracker = StatsTracker::new(manager);
    /// let _manager_ref = tracker.manager();
    /// ```
    #[must_use]
    pub fn manager(&self) -> &M {
        &self.manager
    }

    /// Get a mutable reference to the underlying user manager
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_user::{StatsTracker, InMemoryUserManager};
    ///
    /// let manager = InMemoryUserManager::new();
    /// let mut tracker = StatsTracker::new(manager);
    /// let _manager_ref = tracker.manager_mut();
    /// ```
    pub fn manager_mut(&mut self) -> &mut M {
        &mut self.manager
    }

    /// Record a user login
    ///
    /// Increments login counter and updates last_login timestamp.
    ///
    /// # Errors
    ///
    /// Returns error if user not found or update fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_user::{StatsTracker, InMemoryUserManager, UserManager};
    /// use impulse_types::user::User;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut manager = InMemoryUserManager::new();
    /// let user = User::new("testuser")?;
    /// let user_id = user.id();
    /// manager.create_user(user).await?;
    ///
    /// let mut tracker = StatsTracker::new(manager);
    /// tracker.record_login(user_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn record_login(&mut self, user_id: UserId) -> Result<()> {
        let mut user = self.manager.get_user(user_id).await?;
        user.record_login();
        self.manager.update_user(user).await?;

        tracing::info!(user_id = ?user_id, "Login recorded");
        Ok(())
    }

    /// Record file upload(s)
    ///
    /// # Arguments
    ///
    /// * `user_id` - User performing the upload
    /// * `count` - Number of files uploaded
    /// * `kilobytes` - Total size in kilobytes
    ///
    /// # Errors
    ///
    /// Returns error if user not found or update fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_user::{StatsTracker, InMemoryUserManager, UserManager};
    /// use impulse_types::user::User;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut manager = InMemoryUserManager::new();
    /// let user = User::new("testuser")?;
    /// let user_id = user.id();
    /// manager.create_user(user).await?;
    ///
    /// let mut tracker = StatsTracker::new(manager);
    /// tracker.record_upload(user_id, 1, 1024).await?; // 1 file, 1024 KB
    /// # Ok(())
    /// # }
    /// ```
    pub async fn record_upload(
        &mut self,
        user_id: UserId,
        count: u16,
        kilobytes: u32,
    ) -> Result<()> {
        let mut user = self.manager.get_user(user_id).await?;
        user.stats.record_upload(count, kilobytes);
        self.manager.update_user(user).await?;

        tracing::info!(
            user_id = ?user_id,
            count = count,
            kb = kilobytes,
            "Upload recorded"
        );
        Ok(())
    }

    /// Record file download(s)
    ///
    /// # Arguments
    ///
    /// * `user_id` - User performing the download
    /// * `count` - Number of files downloaded
    /// * `kilobytes` - Total size in kilobytes
    ///
    /// # Errors
    ///
    /// Returns error if user not found or update fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_user::{StatsTracker, InMemoryUserManager, UserManager};
    /// use impulse_types::user::User;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut manager = InMemoryUserManager::new();
    /// let user = User::new("testuser")?;
    /// let user_id = user.id();
    /// manager.create_user(user).await?;
    ///
    /// let mut tracker = StatsTracker::new(manager);
    /// tracker.record_download(user_id, 1, 512).await?; // 1 file, 512 KB
    /// # Ok(())
    /// # }
    /// ```
    pub async fn record_download(
        &mut self,
        user_id: UserId,
        count: u16,
        kilobytes: u32,
    ) -> Result<()> {
        let mut user = self.manager.get_user(user_id).await?;
        user.stats.record_download(count, kilobytes);
        self.manager.update_user(user).await?;

        tracing::info!(
            user_id = ?user_id,
            count = count,
            kb = kilobytes,
            "Download recorded"
        );
        Ok(())
    }

    /// Record a message post
    ///
    /// # Errors
    ///
    /// Returns error if user not found or update fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_user::{StatsTracker, InMemoryUserManager, UserManager};
    /// use impulse_types::user::User;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut manager = InMemoryUserManager::new();
    /// let user = User::new("testuser")?;
    /// let user_id = user.id();
    /// manager.create_user(user).await?;
    ///
    /// let mut tracker = StatsTracker::new(manager);
    /// tracker.record_post(user_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn record_post(&mut self, user_id: UserId) -> Result<()> {
        let mut user = self.manager.get_user(user_id).await?;
        user.stats.record_post();
        self.manager.update_user(user).await?;

        tracing::info!(user_id = ?user_id, "Post recorded");
        Ok(())
    }

    /// Record an email sent
    ///
    /// # Errors
    ///
    /// Returns error if user not found or update fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_user::{StatsTracker, InMemoryUserManager, UserManager};
    /// use impulse_types::user::User;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut manager = InMemoryUserManager::new();
    /// let user = User::new("testuser")?;
    /// let user_id = user.id();
    /// manager.create_user(user).await?;
    ///
    /// let mut tracker = StatsTracker::new(manager);
    /// tracker.record_email(user_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn record_email(&mut self, user_id: UserId) -> Result<()> {
        let mut user = self.manager.get_user(user_id).await?;
        user.stats.record_email();
        self.manager.update_user(user).await?;

        tracing::info!(user_id = ?user_id, "Email recorded");
        Ok(())
    }

    /// Update time spent online
    ///
    /// # Arguments
    ///
    /// * `user_id` - User ID
    /// * `minutes` - Number of minutes to add
    ///
    /// # Errors
    ///
    /// Returns error if user not found or update fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_user::{StatsTracker, InMemoryUserManager, UserManager};
    /// use impulse_types::user::User;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut manager = InMemoryUserManager::new();
    /// let user = User::new("testuser")?;
    /// let user_id = user.id();
    /// manager.create_user(user).await?;
    ///
    /// let mut tracker = StatsTracker::new(manager);
    /// tracker.update_time_online(user_id, 60).await?; // 1 hour
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update_time_online(&mut self, user_id: UserId, minutes: u32) -> Result<()> {
        let mut user = self.manager.get_user(user_id).await?;
        user.stats.record_time(minutes);
        self.manager.update_user(user).await?;

        tracing::info!(user_id = ?user_id, minutes = minutes, "Time online updated");
        Ok(())
    }

    /// Award file points to a user
    ///
    /// # Arguments
    ///
    /// * `user_id` - User ID
    /// * `points` - Number of points to award
    ///
    /// # Errors
    ///
    /// Returns error if user not found or update fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_user::{StatsTracker, InMemoryUserManager, UserManager};
    /// use impulse_types::user::User;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut manager = InMemoryUserManager::new();
    /// let user = User::new("testuser")?;
    /// let user_id = user.id();
    /// manager.create_user(user).await?;
    ///
    /// let mut tracker = StatsTracker::new(manager);
    /// tracker.award_file_points(user_id, 100).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn award_file_points(&mut self, user_id: UserId, points: i16) -> Result<()> {
        let mut user = self.manager.get_user(user_id).await?;
        user.stats.award_file_points(points);
        self.manager.update_user(user).await?;

        tracing::info!(user_id = ?user_id, points = points, "File points awarded");
        Ok(())
    }

    /// Deduct file points from a user
    ///
    /// # Arguments
    ///
    /// * `user_id` - User ID
    /// * `points` - Number of points to deduct
    ///
    /// # Errors
    ///
    /// Returns error if user not found or update fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_user::{StatsTracker, InMemoryUserManager, UserManager};
    /// use impulse_types::user::User;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut manager = InMemoryUserManager::new();
    /// let mut user = User::new("testuser")?;
    /// user.stats.file_points = 100;
    /// let user_id = user.id();
    /// manager.create_user(user).await?;
    ///
    /// let mut tracker = StatsTracker::new(manager);
    /// tracker.deduct_file_points(user_id, 30).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn deduct_file_points(&mut self, user_id: UserId, points: i16) -> Result<()> {
        let mut user = self.manager.get_user(user_id).await?;
        user.stats.deduct_file_points(points);
        self.manager.update_user(user).await?;

        tracing::info!(user_id = ?user_id, points = points, "File points deducted");
        Ok(())
    }

    /// Get the upload/download ratio for a user
    ///
    /// Returns `None` if the user has no downloads (division by zero).
    ///
    /// # Errors
    ///
    /// Returns error if user not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_user::{StatsTracker, InMemoryUserManager, UserManager};
    /// use impulse_types::user::User;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut manager = InMemoryUserManager::new();
    /// let mut user = User::new("testuser")?;
    /// user.stats.record_upload(1, 1000);
    /// user.stats.record_download(1, 500);
    /// let user_id = user.id();
    /// manager.create_user(user).await?;
    ///
    /// let tracker = StatsTracker::new(manager);
    /// let ratio = tracker.get_ratio(user_id).await?;
    /// assert_eq!(ratio, Some(2.0));
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_ratio(&self, user_id: UserId) -> Result<Option<f64>> {
        let user = self.manager.get_user(user_id).await?;
        Ok(user.stats.ul_dl_ratio())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::InMemoryUserManager;
    use impulse_types::user::User;

    #[tokio::test]
    async fn test_record_login() {
        let mut manager = InMemoryUserManager::new();
        let user = User::new("testuser").unwrap();
        let user_id = user.id();
        manager.create_user(user).await.unwrap();

        let mut tracker = StatsTracker::new(manager);
        tracker.record_login(user_id).await.unwrap();

        let updated = tracker.manager().get_user(user_id).await.unwrap();
        assert_eq!(updated.stats.logins, 1);
        assert_eq!(updated.stats.logins_today, 1);
        assert!(updated.last_login.is_some());
    }

    #[tokio::test]
    async fn test_record_upload() {
        let mut manager = InMemoryUserManager::new();
        let user = User::new("testuser").unwrap();
        let user_id = user.id();
        manager.create_user(user).await.unwrap();

        let mut tracker = StatsTracker::new(manager);
        tracker.record_upload(user_id, 1, 1024).await.unwrap();

        let updated = tracker.manager().get_user(user_id).await.unwrap();
        assert_eq!(updated.stats.uploads, 1);
        assert_eq!(updated.stats.upload_kb, 1024);
    }

    #[tokio::test]
    async fn test_record_download() {
        let mut manager = InMemoryUserManager::new();
        let user = User::new("testuser").unwrap();
        let user_id = user.id();
        manager.create_user(user).await.unwrap();

        let mut tracker = StatsTracker::new(manager);
        tracker.record_download(user_id, 2, 512).await.unwrap();

        let updated = tracker.manager().get_user(user_id).await.unwrap();
        assert_eq!(updated.stats.downloads, 2);
        assert_eq!(updated.stats.download_kb, 512);
    }

    #[tokio::test]
    async fn test_record_post() {
        let mut manager = InMemoryUserManager::new();
        let user = User::new("testuser").unwrap();
        let user_id = user.id();
        manager.create_user(user).await.unwrap();

        let mut tracker = StatsTracker::new(manager);
        tracker.record_post(user_id).await.unwrap();

        let updated = tracker.manager().get_user(user_id).await.unwrap();
        assert_eq!(updated.stats.posts, 1);
    }

    #[tokio::test]
    async fn test_record_email() {
        let mut manager = InMemoryUserManager::new();
        let user = User::new("testuser").unwrap();
        let user_id = user.id();
        manager.create_user(user).await.unwrap();

        let mut tracker = StatsTracker::new(manager);
        tracker.record_email(user_id).await.unwrap();

        let updated = tracker.manager().get_user(user_id).await.unwrap();
        assert_eq!(updated.stats.emails_sent, 1);
    }

    #[tokio::test]
    async fn test_update_time_online() {
        let mut manager = InMemoryUserManager::new();
        let user = User::new("testuser").unwrap();
        let user_id = user.id();
        manager.create_user(user).await.unwrap();

        let mut tracker = StatsTracker::new(manager);
        tracker.update_time_online(user_id, 60).await.unwrap();

        let updated = tracker.manager().get_user(user_id).await.unwrap();
        assert_eq!(updated.stats.total_time_minutes, 60);
    }

    #[tokio::test]
    async fn test_award_file_points() {
        let mut manager = InMemoryUserManager::new();
        let user = User::new("testuser").unwrap();
        let user_id = user.id();
        manager.create_user(user).await.unwrap();

        let mut tracker = StatsTracker::new(manager);
        tracker.award_file_points(user_id, 100).await.unwrap();

        let updated = tracker.manager().get_user(user_id).await.unwrap();
        assert_eq!(updated.stats.file_points, 100);
    }

    #[tokio::test]
    async fn test_deduct_file_points() {
        let mut manager = InMemoryUserManager::new();
        let mut user = User::new("testuser").unwrap();
        user.stats.file_points = 100;
        let user_id = user.id();
        manager.create_user(user).await.unwrap();

        let mut tracker = StatsTracker::new(manager);
        tracker.deduct_file_points(user_id, 30).await.unwrap();

        let updated = tracker.manager().get_user(user_id).await.unwrap();
        assert_eq!(updated.stats.file_points, 70);
    }

    #[tokio::test]
    async fn test_get_ratio() {
        let mut manager = InMemoryUserManager::new();
        let mut user = User::new("testuser").unwrap();
        user.stats.record_upload(1, 1000);
        user.stats.record_download(1, 500);
        let user_id = user.id();
        manager.create_user(user).await.unwrap();

        let tracker = StatsTracker::new(manager);
        let ratio = tracker.get_ratio(user_id).await.unwrap();

        assert_eq!(ratio, Some(2.0));
    }

    #[tokio::test]
    async fn test_get_ratio_no_downloads() {
        let mut manager = InMemoryUserManager::new();
        let user = User::new("testuser").unwrap();
        let user_id = user.id();
        manager.create_user(user).await.unwrap();

        let tracker = StatsTracker::new(manager);
        let ratio = tracker.get_ratio(user_id).await.unwrap();

        assert_eq!(ratio, None);
    }
}
