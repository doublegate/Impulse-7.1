//! User directory with search and pagination
//!
//! This module provides functionality for listing and searching users with
//! pagination support.

use crate::UserManager;
use impulse_types::{error::Result, user::User};

/// User directory for listing and searching users
pub struct UserDirectory<M: UserManager> {
    manager: M,
}

impl<M: UserManager> UserDirectory<M> {
    /// Create a new user directory
    #[must_use]
    pub fn new(manager: M) -> Self {
        UserDirectory { manager }
    }

    /// List all users with pagination
    ///
    /// # Arguments
    ///
    /// * `page` - Page number (0-based)
    /// * `per_page` - Number of users per page
    ///
    /// # Returns
    ///
    /// Tuple of (users, total_count)
    ///
    /// # Errors
    ///
    /// Returns error if listing fails.
    pub async fn list_users(&self, page: usize, per_page: usize) -> Result<(Vec<User>, usize)> {
        let all_users = self.manager.list_users().await?;
        let total = all_users.len();

        let start = page * per_page;
        let end = (start + per_page).min(total);

        let page_users = if start < total {
            all_users[start..end].to_vec()
        } else {
            Vec::new()
        };

        Ok((page_users, total))
    }

    /// Search users by username or real name
    ///
    /// # Arguments
    ///
    /// * `query` - Search query (case-insensitive, partial match)
    ///
    /// # Returns
    ///
    /// Vector of users matching the query
    ///
    /// # Errors
    ///
    /// Returns error if search fails.
    pub async fn search_users(&self, query: &str) -> Result<Vec<User>> {
        let all_users = self.manager.list_users().await?;
        let query_lower = query.to_lowercase();

        let matches: Vec<User> = all_users
            .into_iter()
            .filter(|user| {
                user.username().to_lowercase().contains(&query_lower)
                    || user
                        .real_name
                        .as_ref()
                        .map(|name| name.to_lowercase().contains(&query_lower))
                        .unwrap_or(false)
            })
            .collect();

        Ok(matches)
    }

    /// Get total user count
    ///
    /// # Errors
    ///
    /// Returns error if count fails.
    pub async fn count_users(&self) -> Result<usize> {
        self.manager.count_users().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::InMemoryUserManager;

    #[tokio::test]
    async fn test_list_users() {
        let mut manager = InMemoryUserManager::new();

        for i in 0..25 {
            let user = User::new(format!("user{}", i)).unwrap();
            manager.create_user(user).await.unwrap();
        }

        let directory = UserDirectory::new(manager);

        // Page 0 (first 10 users)
        let (users, total) = directory.list_users(0, 10).await.unwrap();
        assert_eq!(users.len(), 10);
        assert_eq!(total, 25);

        // Page 1 (next 10 users)
        let (users, total) = directory.list_users(1, 10).await.unwrap();
        assert_eq!(users.len(), 10);
        assert_eq!(total, 25);

        // Page 2 (last 5 users)
        let (users, total) = directory.list_users(2, 10).await.unwrap();
        assert_eq!(users.len(), 5);
        assert_eq!(total, 25);

        // Page 3 (beyond end)
        let (users, total) = directory.list_users(3, 10).await.unwrap();
        assert_eq!(users.len(), 0);
        assert_eq!(total, 25);
    }

    #[tokio::test]
    async fn test_search_users_by_username() {
        let mut manager = InMemoryUserManager::new();

        manager
            .create_user(User::new("alice").unwrap())
            .await
            .unwrap();
        manager
            .create_user(User::new("bob").unwrap())
            .await
            .unwrap();
        manager
            .create_user(User::new("charlie").unwrap())
            .await
            .unwrap();

        let directory = UserDirectory::new(manager);

        let results = directory.search_users("ali").await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].username(), "alice");

        let results = directory.search_users("b").await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].username(), "bob");
    }

    #[tokio::test]
    async fn test_search_users_by_real_name() {
        let mut manager = InMemoryUserManager::new();

        let mut user1 = User::new("alice").unwrap();
        user1.real_name = Some("Alice Smith".to_string());
        manager.create_user(user1).await.unwrap();

        let mut user2 = User::new("bob").unwrap();
        user2.real_name = Some("Bob Jones".to_string());
        manager.create_user(user2).await.unwrap();

        let directory = UserDirectory::new(manager);

        let results = directory.search_users("Smith").await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].username(), "alice");

        let results = directory.search_users("jones").await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].username(), "bob");
    }

    #[tokio::test]
    async fn test_search_users_case_insensitive() {
        let mut manager = InMemoryUserManager::new();
        manager
            .create_user(User::new("TestUser").unwrap())
            .await
            .unwrap();

        let directory = UserDirectory::new(manager);

        let results = directory.search_users("testuser").await.unwrap();
        assert_eq!(results.len(), 1);

        let results = directory.search_users("TESTUSER").await.unwrap();
        assert_eq!(results.len(), 1);
    }

    #[tokio::test]
    async fn test_count_users() {
        let mut manager = InMemoryUserManager::new();

        for i in 0..10 {
            let user = User::new(format!("user{}", i)).unwrap();
            manager.create_user(user).await.unwrap();
        }

        let directory = UserDirectory::new(manager);
        let count = directory.count_users().await.unwrap();
        assert_eq!(count, 10);
    }
}
