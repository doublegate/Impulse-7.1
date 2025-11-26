//! User management and persistence
//!
//! This crate provides the user management layer for Impulse-Next_BBS, including:
//! - UserManager trait defining the API contract
//! - InMemoryUserManager for testing and development
//! - FileUserManager for Pascal .DAT file I/O
//! - Statistics tracking and display
//! - User settings and preferences management
//! - User profile display with privacy enforcement
//! - User directory with search and pagination
//! - Achievement tracking and notifications
//!
//! # Architecture
//!
//! The UserManager trait provides async CRUD operations for user accounts,
//! abstracting the underlying storage mechanism. This allows for:
//! - Easy testing with in-memory storage
//! - Backwards compatibility with Pascal USER.LST files
//! - Future support for SQL databases
//!
//! # Examples
//!
//! ```no_run
//! use impulse_user::{UserManager, InMemoryUserManager};
//! use impulse_types::user::User;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let mut manager = InMemoryUserManager::new();
//!
//! // Create a new user
//! let user = User::new("johndoe")?;
//! manager.create_user(user.clone()).await?;
//!
//! // Find user by username
//! let found = manager.find_by_username("johndoe").await?;
//! assert!(found.is_some());
//! assert_eq!(found.as_ref().map(|u| u.username()), Some("johndoe"));
//! # Ok(())
//! # }
//! ```

pub mod achievements;
pub mod directory;
pub mod privacy;
pub mod profile;
pub mod settings;
pub mod stats;

use async_trait::async_trait;
use impulse_types::{
    error::{Error, Result},
    user::{User, UserId},
};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

/// User management trait
///
/// Defines the API contract for user CRUD operations. All methods are async
/// to support both synchronous (file I/O) and asynchronous (database) backends.
#[async_trait]
pub trait UserManager: Send + Sync {
    /// Create a new user
    ///
    /// # Errors
    ///
    /// Returns `Error::UserExists` if a user with the same username already exists.
    async fn create_user(&mut self, user: User) -> Result<()>;

    /// Get user by ID
    ///
    /// # Errors
    ///
    /// Returns `Error::UserNotFound` if no user with the given ID exists.
    async fn get_user(&self, id: UserId) -> Result<User>;

    /// Find user by username (case-insensitive)
    ///
    /// Returns `None` if no user with the given username exists.
    async fn find_by_username(&self, username: &str) -> Result<Option<User>>;

    /// Update an existing user
    ///
    /// # Errors
    ///
    /// Returns `Error::UserNotFound` if the user does not exist.
    async fn update_user(&mut self, user: User) -> Result<()>;

    /// Delete a user by ID
    ///
    /// # Errors
    ///
    /// Returns `Error::UserNotFound` if the user does not exist.
    async fn delete_user(&mut self, id: UserId) -> Result<()>;

    /// List all users
    ///
    /// Returns a vector of all users in the system.
    async fn list_users(&self) -> Result<Vec<User>>;

    /// Count total users
    ///
    /// Returns the total number of users in the system.
    async fn count_users(&self) -> Result<usize>;
}

/// In-memory user manager for testing and development
///
/// Stores users in a HashMap with no persistence. Useful for:
/// - Unit tests
/// - Integration tests
/// - Development without file I/O
///
/// # Examples
///
/// ```
/// use impulse_user::{InMemoryUserManager, UserManager};
/// use impulse_types::user::User;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let mut manager = InMemoryUserManager::new();
///
/// let user = User::new("testuser")?;
/// manager.create_user(user).await?;
///
/// assert_eq!(manager.count_users().await?, 1);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Default)]
pub struct InMemoryUserManager {
    users: Arc<RwLock<HashMap<UserId, User>>>,
}

impl InMemoryUserManager {
    /// Create a new empty in-memory user manager
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_user::InMemoryUserManager;
    ///
    /// let manager = InMemoryUserManager::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create a new in-memory user manager with initial users
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_user::InMemoryUserManager;
    /// use impulse_types::user::User;
    ///
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let user1 = User::new("alice")?;
    /// let user2 = User::new("bob")?;
    /// let manager = InMemoryUserManager::with_users(vec![user1, user2]);
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn with_users(users: Vec<User>) -> Self {
        let map: HashMap<UserId, User> = users.into_iter().map(|u| (u.id(), u)).collect();
        Self {
            users: Arc::new(RwLock::new(map)),
        }
    }

    /// Clear all users (useful for testing)
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_user::{InMemoryUserManager, UserManager};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut manager = InMemoryUserManager::new();
    /// // ... add users ...
    /// manager.clear().await;
    /// assert_eq!(manager.count_users().await?, 0);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn clear(&mut self) {
        self.users.write().unwrap().clear();
    }
}

#[async_trait]
impl UserManager for InMemoryUserManager {
    async fn create_user(&mut self, user: User) -> Result<()> {
        let mut users = self.users.write().unwrap();

        // Check if username already exists
        if users.values().any(|u| u.username() == user.username()) {
            tracing::warn!(
                username = %user.username(),
                "Failed to create user: username already exists"
            );
            return Err(Error::AlreadyExists(format!(
                "User '{}' already exists",
                user.username()
            )));
        }

        let user_id = user.id();
        let username = user.username().to_string();
        users.insert(user.id(), user);

        tracing::info!(
            user_id = ?user_id,
            username = %username,
            "User created successfully"
        );
        Ok(())
    }

    async fn get_user(&self, id: UserId) -> Result<User> {
        let users = self.users.read().unwrap();
        users
            .get(&id)
            .cloned()
            .ok_or_else(|| Error::NotFound(format!("User with ID {:?} not found", id)))
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        let users = self.users.read().unwrap();
        let username_lower = username.to_lowercase();
        Ok(users
            .values()
            .find(|u| u.username().to_lowercase() == username_lower)
            .cloned())
    }

    async fn update_user(&mut self, user: User) -> Result<()> {
        let mut users = self.users.write().unwrap();

        if !users.contains_key(&user.id()) {
            tracing::warn!(
                user_id = ?user.id(),
                "Failed to update user: user not found"
            );
            return Err(Error::NotFound(format!(
                "User with ID {:?} not found",
                user.id()
            )));
        }

        let user_id = user.id();
        let username = user.username().to_string();
        users.insert(user.id(), user);

        tracing::info!(
            user_id = ?user_id,
            username = %username,
            "User updated successfully"
        );
        Ok(())
    }

    async fn delete_user(&mut self, id: UserId) -> Result<()> {
        let mut users = self.users.write().unwrap();
        match users.remove(&id) {
            Some(user) => {
                tracing::info!(
                    user_id = ?id,
                    username = %user.username(),
                    "User deleted successfully"
                );
                Ok(())
            }
            None => {
                tracing::warn!(
                    user_id = ?id,
                    "Failed to delete user: user not found"
                );
                Err(Error::NotFound(format!("User with ID {:?} not found", id)))
            }
        }
    }

    async fn list_users(&self) -> Result<Vec<User>> {
        let users = self.users.read().unwrap();
        Ok(users.values().cloned().collect())
    }

    async fn count_users(&self) -> Result<usize> {
        let users = self.users.read().unwrap();
        Ok(users.len())
    }
}

/// File-based user manager for Pascal USER.LST compatibility
///
/// Reads and writes users to a Pascal-format USER.LST file using binrw.
/// Maintains backwards compatibility with the original Impulse 7.1 BBS.
///
/// # Examples
///
/// ```no_run
/// use impulse_user::FileUserManager;
/// use std::path::PathBuf;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let path = PathBuf::from("./data/USER.LST");
/// let mut manager = FileUserManager::new(path);
///
/// // Load users from file
/// manager.load().await?;
///
/// // ... modify users ...
///
/// // Save users back to file
/// manager.save().await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct FileUserManager {
    path: PathBuf,
    users: Arc<RwLock<HashMap<UserId, User>>>,
}

impl FileUserManager {
    /// Create a new file-based user manager
    ///
    /// # Arguments
    ///
    /// * `path` - Path to USER.LST file
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_user::FileUserManager;
    /// use std::path::PathBuf;
    ///
    /// let path = PathBuf::from("./data/USER.LST");
    /// let manager = FileUserManager::new(path);
    /// ```
    #[must_use]
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Load users from USER.LST file
    ///
    /// # Errors
    ///
    /// Returns `Error::Io` if the file cannot be read or parsed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use impulse_user::FileUserManager;
    /// use std::path::PathBuf;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut manager = FileUserManager::new(PathBuf::from("./data/USER.LST"));
    /// manager.load().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn load(&mut self) -> Result<()> {
        use binrw::BinRead;
        use impulse_types::pascal_user::PascalUserRec;
        use std::fs::File;
        use std::io::{BufReader, Seek};

        tracing::debug!(
            file_path = ?self.path,
            "Loading users from file"
        );

        let file = File::open(&self.path).map_err(|e| {
            tracing::error!(
                file_path = ?self.path,
                error = %e,
                "Failed to open USER.LST file"
            );
            Error::UserManagement(format!("Failed to open USER.LST at {:?}: {}", self.path, e))
        })?;

        let mut reader = BufReader::new(file);
        let mut users_map = HashMap::new();

        // Read records until EOF
        loop {
            // Save position before read attempt
            let pos = reader.stream_position().map_err(|e| {
                Error::UserManagement(format!("Failed to get stream position: {}", e))
            })?;

            match PascalUserRec::read_le(&mut reader) {
                Ok(rec) => {
                    // Convert to modern User
                    match User::from_pascal(&rec) {
                        Ok(user) => {
                            users_map.insert(user.id(), user);
                        }
                        Err(e) => {
                            // Log warning but continue (some records might be corrupted)
                            tracing::warn!(
                                file_path = ?self.path,
                                position = pos,
                                error = %e,
                                "Failed to convert user record, skipping"
                            );
                        }
                    }
                }
                Err(e) => {
                    // Check if EOF (normal termination)
                    if reader.stream_position().map(|p| p == pos).unwrap_or(true) {
                        break; // EOF reached
                    } else {
                        tracing::error!(
                            file_path = ?self.path,
                            position = pos,
                            error = %e,
                            "Failed to read user record"
                        );
                        return Err(Error::UserManagement(format!(
                            "Failed to read user record at position {}: {}",
                            pos, e
                        )));
                    }
                }
            }
        }

        let user_count = users_map.len();
        *self.users.write().unwrap() = users_map;

        tracing::info!(
            file_path = ?self.path,
            user_count = user_count,
            "Successfully loaded users from file"
        );
        Ok(())
    }

    /// Save users to USER.LST file
    ///
    /// # Errors
    ///
    /// Returns `Error::Io` if the file cannot be written.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use impulse_user::FileUserManager;
    /// use std::path::PathBuf;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut manager = FileUserManager::new(PathBuf::from("./data/USER.LST"));
    /// manager.load().await?;
    /// // ... modify users ...
    /// manager.save().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn save(&self) -> Result<()> {
        use binrw::BinWrite;
        use std::fs::File;
        use std::io::BufWriter;

        tracing::debug!(
            file_path = ?self.path,
            "Saving users to file"
        );

        let file = File::create(&self.path).map_err(|e| {
            tracing::error!(
                file_path = ?self.path,
                error = %e,
                "Failed to create USER.LST file"
            );
            Error::UserManagement(format!(
                "Failed to create USER.LST at {:?}: {}",
                self.path, e
            ))
        })?;

        let mut writer = BufWriter::new(file);
        let users = self.users.read().unwrap();
        let user_count = users.len();

        for user in users.values() {
            let rec = user.to_pascal();
            rec.write_le(&mut writer).map_err(|e| {
                tracing::error!(
                    file_path = ?self.path,
                    username = %user.username(),
                    error = %e,
                    "Failed to write user record"
                );
                Error::UserManagement(format!(
                    "Failed to write user record for {}: {}",
                    user.username(),
                    e
                ))
            })?;
        }

        tracing::info!(
            file_path = ?self.path,
            user_count = user_count,
            "Successfully saved users to file"
        );
        Ok(())
    }

    /// Get the file path
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_user::FileUserManager;
    /// use std::path::PathBuf;
    ///
    /// let path = PathBuf::from("./data/USER.LST");
    /// let manager = FileUserManager::new(path.clone());
    /// assert_eq!(manager.path(), &path);
    /// ```
    #[must_use]
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

#[async_trait]
impl UserManager for FileUserManager {
    async fn create_user(&mut self, user: User) -> Result<()> {
        let user_id = user.id();
        let username = user.username().to_string();

        {
            let mut users = self.users.write().unwrap();

            // Check if username already exists
            if users.values().any(|u| u.username() == user.username()) {
                tracing::warn!(
                    username = %username,
                    file_path = ?self.path,
                    "Failed to create user: username already exists"
                );
                return Err(Error::AlreadyExists(format!(
                    "User '{}' already exists",
                    user.username()
                )));
            }

            users.insert(user.id(), user);
        } // Lock released here

        self.save().await?;

        tracing::info!(
            user_id = ?user_id,
            username = %username,
            file_path = ?self.path,
            "User created and saved to file"
        );
        Ok(())
    }

    async fn get_user(&self, id: UserId) -> Result<User> {
        let users = self.users.read().unwrap();
        users
            .get(&id)
            .cloned()
            .ok_or_else(|| Error::NotFound(format!("User with ID {:?} not found", id)))
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        let users = self.users.read().unwrap();
        let username_lower = username.to_lowercase();
        Ok(users
            .values()
            .find(|u| u.username().to_lowercase() == username_lower)
            .cloned())
    }

    async fn update_user(&mut self, user: User) -> Result<()> {
        let user_id = user.id();
        let username = user.username().to_string();

        {
            let mut users = self.users.write().unwrap();

            if !users.contains_key(&user.id()) {
                tracing::warn!(
                    user_id = ?user_id,
                    file_path = ?self.path,
                    "Failed to update user: user not found"
                );
                return Err(Error::NotFound(format!(
                    "User with ID {:?} not found",
                    user.id()
                )));
            }

            users.insert(user.id(), user);
        } // Lock released here

        self.save().await?;

        tracing::info!(
            user_id = ?user_id,
            username = %username,
            file_path = ?self.path,
            "User updated and saved to file"
        );
        Ok(())
    }

    async fn delete_user(&mut self, id: UserId) -> Result<()> {
        let username = {
            let mut users = self.users.write().unwrap();
            match users.remove(&id) {
                Some(user) => user.username().to_string(),
                None => {
                    tracing::warn!(
                        user_id = ?id,
                        file_path = ?self.path,
                        "Failed to delete user: user not found"
                    );
                    return Err(Error::NotFound(format!("User with ID {:?} not found", id)));
                }
            }
        }; // Lock released here

        self.save().await?;

        tracing::info!(
            user_id = ?id,
            username = %username,
            file_path = ?self.path,
            "User deleted and changes saved to file"
        );
        Ok(())
    }

    async fn list_users(&self) -> Result<Vec<User>> {
        let users = self.users.read().unwrap();
        Ok(users.values().cloned().collect())
    }

    async fn count_users(&self) -> Result<usize> {
        let users = self.users.read().unwrap();
        Ok(users.len())
    }
}

// Re-export commonly used types
pub use achievements::{Achievement, AchievementChecker, AchievementProgress, UserAchievement};
pub use directory::UserDirectory;
pub use privacy::PrivacySettings;
pub use profile::{ProfileDisplayOptions, ProfileViewer};
pub use settings::{PasswordStrength, ProtocolSettings, SettingsManager, Theme};
pub use stats::StatsTracker;

#[cfg(test)]
mod tests {
    use super::*;
    use impulse_types::security::SecurityLevel;

    #[tokio::test]
    async fn test_in_memory_create_user() {
        let mut manager = InMemoryUserManager::new();
        let user = User::new("testuser").unwrap();
        let user_id = user.id();

        manager.create_user(user).await.unwrap();
        assert_eq!(manager.count_users().await.unwrap(), 1);

        let retrieved = manager.get_user(user_id).await.unwrap();
        assert_eq!(retrieved.username(), "testuser");
    }

    #[tokio::test]
    async fn test_in_memory_duplicate_username() {
        let mut manager = InMemoryUserManager::new();
        let user1 = User::new("testuser").unwrap();
        let user2 = User::new("testuser").unwrap();

        manager.create_user(user1).await.unwrap();
        let result = manager.create_user(user2).await;
        assert!(matches!(result, Err(Error::AlreadyExists(_))));
    }

    #[tokio::test]
    async fn test_in_memory_find_by_username() {
        let mut manager = InMemoryUserManager::new();
        let user = User::new("TestUser").unwrap();

        manager.create_user(user).await.unwrap();

        // Case-insensitive search
        let found = manager.find_by_username("testuser").await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().username(), "TestUser");

        let not_found = manager.find_by_username("nobody").await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    async fn test_in_memory_update_user() {
        let mut manager = InMemoryUserManager::new();
        let mut user = User::new("testuser").unwrap();
        let user_id = user.id();

        manager.create_user(user.clone()).await.unwrap();

        // Update security level
        user.set_security_level(SecurityLevel::new(100));
        manager.update_user(user).await.unwrap();

        let retrieved = manager.get_user(user_id).await.unwrap();
        assert_eq!(retrieved.security_level().value(), 100);
    }

    #[tokio::test]
    async fn test_in_memory_delete_user() {
        let mut manager = InMemoryUserManager::new();
        let user = User::new("testuser").unwrap();
        let user_id = user.id();

        manager.create_user(user).await.unwrap();
        assert_eq!(manager.count_users().await.unwrap(), 1);

        manager.delete_user(user_id).await.unwrap();
        assert_eq!(manager.count_users().await.unwrap(), 0);

        let result = manager.get_user(user_id).await;
        assert!(matches!(result, Err(Error::NotFound(_))));
    }

    #[tokio::test]
    async fn test_in_memory_list_users() {
        let mut manager = InMemoryUserManager::new();
        let user1 = User::new("alice").unwrap();
        let user2 = User::new("bob").unwrap();

        manager.create_user(user1).await.unwrap();
        manager.create_user(user2).await.unwrap();

        let users = manager.list_users().await.unwrap();
        assert_eq!(users.len(), 2);

        let usernames: Vec<&str> = users.iter().map(|u| u.username()).collect();
        assert!(usernames.contains(&"alice"));
        assert!(usernames.contains(&"bob"));
    }

    #[tokio::test]
    async fn test_in_memory_clear() {
        let mut manager = InMemoryUserManager::new();
        let user = User::new("testuser").unwrap();

        manager.create_user(user).await.unwrap();
        assert_eq!(manager.count_users().await.unwrap(), 1);

        manager.clear().await;
        assert_eq!(manager.count_users().await.unwrap(), 0);
    }

    #[tokio::test]
    async fn test_in_memory_with_users() {
        let user1 = User::new("alice").unwrap();
        let user2 = User::new("bob").unwrap();
        let manager = InMemoryUserManager::with_users(vec![user1, user2]);

        assert_eq!(manager.count_users().await.unwrap(), 2);
    }

    #[tokio::test]
    async fn test_file_manager_new() {
        let path = PathBuf::from("/tmp/impulse-next-bbs/test-users.lst");
        let manager = FileUserManager::new(path.clone());
        assert_eq!(manager.path(), &path);
    }

    #[tokio::test]
    async fn test_file_manager_nonexistent_file() {
        let path = PathBuf::from("/tmp/impulse-next-bbs/nonexistent.lst");
        let mut manager = FileUserManager::new(path);
        let result = manager.load().await;
        assert!(matches!(result, Err(Error::UserManagement(_))));
    }
}
