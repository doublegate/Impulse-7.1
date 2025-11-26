//! User Factory for creating test users with various configurations

use anyhow::Result;
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

use super::{User, generate_test_email, test_password_hash};

/// Factory for creating test users with customizable properties
///
/// Uses in-memory storage for testing without requiring database setup.
/// Creates realistic User instances with appropriate default values.
pub struct UserFactory {
    users: Arc<tokio::sync::RwLock<Vec<User>>>,
}

impl UserFactory {
    /// Create a new UserFactory with shared user storage
    pub fn new(users: Arc<tokio::sync::RwLock<Vec<User>>>) -> Self {
        Self { users }
    }

    /// Create a test user with specified username and security level
    ///
    /// # Arguments
    ///
    /// * `username` - Username for the new user
    /// * `security_level` - Security level (0-255)
    ///
    /// # Returns
    ///
    /// Newly created User instance with default test values
    pub async fn create_user(&self, username: &str, security_level: u8) -> Result<User> {
        let mut users = self.users.write().await;

        // Generate unique ID
        let id = users.len() as i32 + 1;

        let user = User {
            id,
            username: username.to_string(),
            password_hash: test_password_hash(),
            email: generate_test_email(username),
            security_level,
            time_limit: 60,
            download_limit_kb: 10240,
            upload_limit_kb: 10240,
            total_calls: 0,
            last_login: None,
            created_at: Utc::now(),
        };

        users.push(user.clone());
        Ok(user)
    }

    /// Create a SysOp user (security level 255)
    pub async fn create_sysop(&self) -> Result<User> {
        self.create_user("sysop", 255).await
    }

    /// Create a regular user (security level 10)
    pub async fn create_regular_user(&self) -> Result<User> {
        let username = format!("user_{}", &Uuid::new_v4().to_string()[..8]);
        self.create_user(&username, 10).await
    }

    /// Create a privileged user (security level 100)
    pub async fn create_privileged_user(&self) -> Result<User> {
        let username = format!("privuser_{}", &Uuid::new_v4().to_string()[..8]);
        self.create_user(&username, 100).await
    }

    /// Create multiple test users with sequential names
    ///
    /// # Arguments
    ///
    /// * `count` - Number of users to create
    /// * `security_level` - Security level for all users
    ///
    /// # Returns
    ///
    /// Vector of created User instances
    pub async fn create_users_batch(&self, count: usize, security_level: u8) -> Result<Vec<User>> {
        let mut created_users = Vec::with_capacity(count);

        for i in 0..count {
            let username = format!("batchuser_{}", i);
            let user = self.create_user(&username, security_level).await?;
            created_users.push(user);
        }

        Ok(created_users)
    }

    /// Find user by username
    pub async fn find_by_username(&self, username: &str) -> Option<User> {
        let users = self.users.read().await;
        users.iter().find(|u| u.username == username).cloned()
    }

    /// Get total user count
    pub async fn count(&self) -> usize {
        self.users.read().await.len()
    }

    /// Clear all users
    pub async fn clear(&self) {
        self.users.write().await.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn setup_factory() -> UserFactory {
        let users = Arc::new(tokio::sync::RwLock::new(Vec::new()));
        UserFactory::new(users)
    }

    #[tokio::test]
    async fn test_create_user() {
        let factory = setup_factory().await;
        let user = factory.create_user("testuser", 50).await.unwrap();

        assert_eq!(user.username, "testuser");
        assert_eq!(user.security_level, 50);
        assert_eq!(user.email, "testuser@test.local");
        assert!(user.password_hash.starts_with("$argon2id$"));
    }

    #[tokio::test]
    async fn test_create_sysop() {
        let factory = setup_factory().await;
        let sysop = factory.create_sysop().await.unwrap();

        assert_eq!(sysop.username, "sysop");
        assert_eq!(sysop.security_level, 255);
    }

    #[tokio::test]
    async fn test_create_regular_user() {
        let factory = setup_factory().await;
        let user = factory.create_regular_user().await.unwrap();

        assert_eq!(user.security_level, 10);
        assert!(user.username.starts_with("user_"));
    }

    #[tokio::test]
    async fn test_create_privileged_user() {
        let factory = setup_factory().await;
        let user = factory.create_privileged_user().await.unwrap();

        assert_eq!(user.security_level, 100);
        assert!(user.username.starts_with("privuser_"));
    }

    #[tokio::test]
    async fn test_create_users_batch() {
        let factory = setup_factory().await;
        let users = factory.create_users_batch(5, 20).await.unwrap();

        assert_eq!(users.len(), 5);
        for (i, user) in users.iter().enumerate() {
            assert_eq!(user.username, format!("batchuser_{}", i));
            assert_eq!(user.security_level, 20);
        }
    }

    #[tokio::test]
    async fn test_find_by_username() {
        let factory = setup_factory().await;
        factory.create_user("findme", 30).await.unwrap();
        factory.create_user("other", 30).await.unwrap();

        let found = factory.find_by_username("findme").await;
        assert!(found.is_some());
        assert_eq!(found.unwrap().username, "findme");

        let not_found = factory.find_by_username("notexist").await;
        assert!(not_found.is_none());
    }

    #[tokio::test]
    async fn test_count() {
        let factory = setup_factory().await;
        assert_eq!(factory.count().await, 0);

        factory.create_user("user1", 10).await.unwrap();
        assert_eq!(factory.count().await, 1);

        factory.create_users_batch(3, 10).await.unwrap();
        assert_eq!(factory.count().await, 4);
    }

    #[tokio::test]
    async fn test_clear() {
        let factory = setup_factory().await;
        factory.create_users_batch(5, 10).await.unwrap();
        assert_eq!(factory.count().await, 5);

        factory.clear().await;
        assert_eq!(factory.count().await, 0);
    }

    #[tokio::test]
    async fn test_unique_ids() {
        let factory = setup_factory().await;
        let user1 = factory.create_user("user1", 10).await.unwrap();
        let user2 = factory.create_user("user2", 10).await.unwrap();
        let user3 = factory.create_user("user3", 10).await.unwrap();

        assert_ne!(user1.id, user2.id);
        assert_ne!(user2.id, user3.id);
        assert_ne!(user1.id, user3.id);
    }
}
