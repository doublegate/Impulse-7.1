//! BBS Test Fixture for isolated test environments
//!
//! Provides clean, isolated BBS instances for integration testing.
//! Each fixture gets its own temporary directory and test data.

use anyhow::Result;
use std::path::PathBuf;
use std::sync::Arc;
use tempfile::TempDir;

use super::{User, UserFactory, generate_test_username};

/// Simple test configuration for BBS fixture
#[derive(Debug, Clone)]
pub struct TestConfig {
    pub data_directory: PathBuf,
    pub file_areas_directory: PathBuf,
    pub message_base_directory: PathBuf,
    pub door_directory: PathBuf,
}

/// Test fixture providing a clean BBS environment for each test
///
/// Each fixture is isolated with:
/// - Temporary directory for file storage
/// - Separate test configuration
/// - User factory for test data generation
///
/// # Example
///
/// ```no_run
/// use impulse_integration_tests::fixtures::BbsTestFixture;
///
/// #[tokio::test]
/// async fn test_user_login() {
///     let fixture = BbsTestFixture::new().await.unwrap();
///     let user = fixture.create_test_user("testuser", 10).await.unwrap();
///     // Test user login functionality
/// }
/// ```
pub struct BbsTestFixture {
    /// Temporary directory for test data (automatically cleaned up)
    pub temp_dir: TempDir,
    /// Test configuration for this fixture
    pub config: TestConfig,
    /// User factory for creating test users
    pub user_factory: UserFactory,
    /// In-memory user storage for testing
    users: Arc<tokio::sync::RwLock<Vec<User>>>,
}

impl BbsTestFixture {
    /// Create a new isolated test fixture
    ///
    /// Sets up a temporary directory, default configuration,
    /// and initializes test infrastructure.
    pub async fn new() -> Result<Self> {
        // Create temporary directory (auto-cleanup on drop)
        let temp_dir = TempDir::new()?;

        // Configure test directories
        let config = TestConfig {
            data_directory: temp_dir.path().to_path_buf(),
            file_areas_directory: temp_dir.path().join("files"),
            message_base_directory: temp_dir.path().join("messages"),
            door_directory: temp_dir.path().join("doors"),
        };

        // Create necessary subdirectories
        std::fs::create_dir_all(&config.file_areas_directory)?;
        std::fs::create_dir_all(&config.message_base_directory)?;
        std::fs::create_dir_all(&config.door_directory)?;

        let users = Arc::new(tokio::sync::RwLock::new(Vec::new()));
        let user_factory = UserFactory::new(Arc::clone(&users));

        Ok(Self {
            temp_dir,
            config,
            user_factory,
            users,
        })
    }

    /// Create a test user with specific security level
    ///
    /// # Arguments
    ///
    /// * `username` - Username for the test user
    /// * `security_level` - Security level (0-255)
    ///
    /// # Returns
    ///
    /// Newly created User instance
    pub async fn create_test_user(&self, username: &str, security_level: u8) -> Result<User> {
        self.user_factory
            .create_user(username, security_level)
            .await
    }

    /// Create a SysOp user (security level 255)
    pub async fn create_sysop(&self) -> Result<User> {
        self.user_factory.create_sysop().await
    }

    /// Create a regular user (security level 10)
    pub async fn create_regular_user(&self) -> Result<User> {
        let username = generate_test_username();
        self.user_factory.create_user(&username, 10).await
    }

    /// Get all users created in this fixture
    pub async fn get_all_users(&self) -> Vec<User> {
        self.users.read().await.clone()
    }

    /// Clean up test data (called automatically on drop, but can be called explicitly)
    pub async fn cleanup(&self) -> Result<()> {
        self.users.write().await.clear();
        Ok(())
    }

    /// Get the path to the temporary directory
    pub fn temp_path(&self) -> &PathBuf {
        &self.config.data_directory
    }

    /// Get the file areas directory path
    pub fn file_areas_path(&self) -> &PathBuf {
        &self.config.file_areas_directory
    }

    /// Get the message base directory path
    pub fn message_base_path(&self) -> &PathBuf {
        &self.config.message_base_directory
    }

    /// Get the door games directory path
    pub fn door_directory_path(&self) -> &PathBuf {
        &self.config.door_directory
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fixture_creation() {
        let fixture = BbsTestFixture::new().await.unwrap();
        assert!(fixture.temp_path().exists());
        assert!(fixture.file_areas_path().exists());
        assert!(fixture.message_base_path().exists());
        assert!(fixture.door_directory_path().exists());
    }

    #[tokio::test]
    async fn test_fixture_isolation() {
        let fixture1 = BbsTestFixture::new().await.unwrap();
        let fixture2 = BbsTestFixture::new().await.unwrap();

        let user1 = fixture1.create_test_user("test1", 10).await.unwrap();
        let user2 = fixture2.create_test_user("test2", 10).await.unwrap();

        // Verify fixtures are isolated (different temp dirs)
        assert_ne!(fixture1.temp_path(), fixture2.temp_path());

        // Users may have same ID (1) since each fixture has its own user store
        // This is acceptable as they're in isolated fixtures
        assert_eq!(user1.username, "test1");
        assert_eq!(user2.username, "test2");
    }

    #[tokio::test]
    async fn test_create_sysop() {
        let fixture = BbsTestFixture::new().await.unwrap();
        let sysop = fixture.create_sysop().await.unwrap();
        assert_eq!(sysop.security_level, 255);
        assert_eq!(sysop.username, "sysop");
    }

    #[tokio::test]
    async fn test_create_regular_user() {
        let fixture = BbsTestFixture::new().await.unwrap();
        let user = fixture.create_regular_user().await.unwrap();
        assert_eq!(user.security_level, 10);
        assert!(user.username.starts_with("testuser_"));
    }

    #[tokio::test]
    async fn test_get_all_users() {
        let fixture = BbsTestFixture::new().await.unwrap();

        fixture.create_test_user("user1", 10).await.unwrap();
        fixture.create_test_user("user2", 20).await.unwrap();
        fixture.create_sysop().await.unwrap();

        let users = fixture.get_all_users().await;
        assert_eq!(users.len(), 3);
    }

    #[tokio::test]
    async fn test_cleanup() {
        let fixture = BbsTestFixture::new().await.unwrap();

        fixture.create_test_user("user1", 10).await.unwrap();
        fixture.create_test_user("user2", 20).await.unwrap();

        fixture.cleanup().await.unwrap();

        let users = fixture.get_all_users().await;
        assert_eq!(users.len(), 0);
    }
}
