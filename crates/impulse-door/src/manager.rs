//! Door manager for managing multiple door configurations.
//!
//! This module provides the `DoorManager` for loading, managing, and
//! accessing door game configurations.

use crate::config::DoorConfig;
use crate::error::{DoorError, Result};
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::{debug, info, warn};

/// Door manager for managing multiple door configurations.
///
/// The door manager handles loading door configurations from TOML files,
/// managing the collection of available doors, and providing access to
/// door configurations by name.
pub struct DoorManager {
    /// Map of door name to door configuration
    doors: HashMap<String, DoorConfig>,
    /// Directory containing door configuration files
    door_dir: PathBuf,
    /// Directory for node-specific files (dropfiles, temp files)
    node_dir: PathBuf,
}

impl DoorManager {
    /// Create a new door manager.
    ///
    /// # Arguments
    ///
    /// * `door_dir` - Directory containing door configuration files
    /// * `node_dir` - Directory for node-specific files
    ///
    /// # Returns
    ///
    /// A new door manager with doors loaded from the door directory
    pub async fn new(door_dir: PathBuf, node_dir: PathBuf) -> Result<Self> {
        let mut manager = Self {
            doors: HashMap::new(),
            door_dir,
            node_dir,
        };

        // Create directories if they don't exist
        tokio::fs::create_dir_all(&manager.door_dir).await?;
        tokio::fs::create_dir_all(&manager.node_dir).await?;

        // Load door configurations
        manager.load_doors().await?;

        Ok(manager)
    }

    /// Load all door configurations from the door directory.
    async fn load_doors(&mut self) -> Result<()> {
        info!(
            "Loading door configurations from {}",
            self.door_dir.display()
        );

        let mut entries = tokio::fs::read_dir(&self.door_dir).await?;
        let mut loaded_count = 0;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("toml") {
                match DoorConfig::from_file(&path) {
                    Ok(config) => {
                        debug!("Loaded door configuration: {}", config.name);
                        self.doors.insert(config.name.clone(), config);
                        loaded_count += 1;
                    }
                    Err(e) => {
                        warn!("Failed to load door config from {:?}: {}", path, e);
                    }
                }
            }
        }

        info!("Loaded {} door configurations", loaded_count);
        Ok(())
    }

    /// Get a list of all available doors.
    ///
    /// # Returns
    ///
    /// A vector of references to door configurations
    pub fn list_doors(&self) -> Vec<&DoorConfig> {
        self.doors.values().collect()
    }

    /// Get a door configuration by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the door to retrieve
    ///
    /// # Returns
    ///
    /// A reference to the door configuration, or `None` if not found
    pub fn get_door(&self, name: &str) -> Option<&DoorConfig> {
        self.doors.get(name)
    }

    /// Add a new door configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - The door configuration to add
    ///
    /// # Returns
    ///
    /// Ok if the door was added successfully, or an error if validation failed
    pub fn add_door(&mut self, config: DoorConfig) -> Result<()> {
        config.validate()?;
        self.doors.insert(config.name.clone(), config);
        Ok(())
    }

    /// Remove a door configuration.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the door to remove
    ///
    /// # Returns
    ///
    /// Ok if the door was removed, or an error if the door was not found
    pub fn remove_door(&mut self, name: &str) -> Result<()> {
        if self.doors.remove(name).is_some() {
            Ok(())
        } else {
            Err(DoorError::DoorNotFound(name.to_string()))
        }
    }

    /// Reload all door configurations from disk.
    ///
    /// This clears the current door list and reloads all configurations
    /// from the door directory.
    pub async fn reload_doors(&mut self) -> Result<()> {
        info!("Reloading door configurations");
        self.doors.clear();
        self.load_doors().await
    }

    /// Get the node directory path for a specific node.
    ///
    /// # Arguments
    ///
    /// * `node_id` - The node ID
    ///
    /// # Returns
    ///
    /// The path to the node directory
    pub fn get_node_dir(&self, node_id: u16) -> PathBuf {
        self.node_dir.join(format!("node{}", node_id))
    }

    /// Check if a door exists.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the door to check
    ///
    /// # Returns
    ///
    /// `true` if the door exists, `false` otherwise
    pub fn has_door(&self, name: &str) -> bool {
        self.doors.contains_key(name)
    }

    /// Get the number of configured doors.
    ///
    /// # Returns
    ///
    /// The number of doors in the manager
    pub fn door_count(&self) -> usize {
        self.doors.len()
    }

    /// Get doors that the user has access to based on security level.
    ///
    /// # Arguments
    ///
    /// * `security_level` - The user's security level
    ///
    /// # Returns
    ///
    /// A vector of references to accessible door configurations
    pub fn list_accessible_doors(&self, security_level: u8) -> Vec<&DoorConfig> {
        self.doors
            .values()
            .filter(|door| door.min_security_level <= security_level)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn create_test_manager() -> DoorManager {
        let temp_dir = tempfile::tempdir().unwrap();
        let door_dir = temp_dir.path().join("doors");
        let node_dir = temp_dir.path().join("nodes");

        DoorManager::new(door_dir, node_dir).await.unwrap()
    }

    fn create_test_config(name: &str, security_level: u8) -> DoorConfig {
        let mut config = DoorConfig::new(
            name.to_string(),
            PathBuf::from("/bin/ls"),
            PathBuf::from("/tmp"),
        );
        config.min_security_level = security_level;
        config
    }

    #[tokio::test]
    async fn test_door_manager_new() {
        let manager = create_test_manager().await;
        assert_eq!(manager.door_count(), 0);
    }

    #[tokio::test]
    async fn test_add_door() {
        let mut manager = create_test_manager().await;
        let config = create_test_config("test-door", 10);

        let result = manager.add_door(config);
        assert!(result.is_ok());
        assert_eq!(manager.door_count(), 1);
    }

    #[tokio::test]
    async fn test_get_door() {
        let mut manager = create_test_manager().await;
        let config = create_test_config("test-door", 10);
        manager.add_door(config).unwrap();

        let door = manager.get_door("test-door");
        assert!(door.is_some());
        assert_eq!(door.unwrap().name, "test-door");
    }

    #[tokio::test]
    async fn test_get_door_not_found() {
        let manager = create_test_manager().await;
        let door = manager.get_door("nonexistent");
        assert!(door.is_none());
    }

    #[tokio::test]
    async fn test_list_doors() {
        let mut manager = create_test_manager().await;
        manager.add_door(create_test_config("door1", 10)).unwrap();
        manager.add_door(create_test_config("door2", 20)).unwrap();
        manager.add_door(create_test_config("door3", 30)).unwrap();

        let doors = manager.list_doors();
        assert_eq!(doors.len(), 3);
    }

    #[tokio::test]
    async fn test_remove_door() {
        let mut manager = create_test_manager().await;
        manager
            .add_door(create_test_config("test-door", 10))
            .unwrap();

        assert_eq!(manager.door_count(), 1);

        let result = manager.remove_door("test-door");
        assert!(result.is_ok());
        assert_eq!(manager.door_count(), 0);
    }

    #[tokio::test]
    async fn test_remove_door_not_found() {
        let mut manager = create_test_manager().await;
        let result = manager.remove_door("nonexistent");
        assert!(result.is_err());
        assert!(matches!(result, Err(DoorError::DoorNotFound(_))));
    }

    #[tokio::test]
    async fn test_has_door() {
        let mut manager = create_test_manager().await;
        manager
            .add_door(create_test_config("test-door", 10))
            .unwrap();

        assert!(manager.has_door("test-door"));
        assert!(!manager.has_door("nonexistent"));
    }

    #[tokio::test]
    async fn test_door_count() {
        let mut manager = create_test_manager().await;
        assert_eq!(manager.door_count(), 0);

        manager.add_door(create_test_config("door1", 10)).unwrap();
        assert_eq!(manager.door_count(), 1);

        manager.add_door(create_test_config("door2", 20)).unwrap();
        assert_eq!(manager.door_count(), 2);
    }

    #[tokio::test]
    async fn test_get_node_dir() {
        let manager = create_test_manager().await;
        let node_dir = manager.get_node_dir(5);
        assert!(node_dir.to_string_lossy().contains("node5"));
    }

    #[tokio::test]
    async fn test_list_accessible_doors() {
        let mut manager = create_test_manager().await;
        manager.add_door(create_test_config("door1", 10)).unwrap();
        manager.add_door(create_test_config("door2", 50)).unwrap();
        manager.add_door(create_test_config("door3", 100)).unwrap();

        // User with security level 50 should see doors 1 and 2
        let accessible = manager.list_accessible_doors(50);
        assert_eq!(accessible.len(), 2);

        // User with security level 100 should see all doors
        let accessible = manager.list_accessible_doors(100);
        assert_eq!(accessible.len(), 3);

        // User with security level 5 should see no doors
        let accessible = manager.list_accessible_doors(5);
        assert_eq!(accessible.len(), 0);
    }

    #[tokio::test]
    async fn test_reload_doors() {
        let temp_dir = tempfile::tempdir().unwrap();
        let door_dir = temp_dir.path().join("doors");
        let node_dir = temp_dir.path().join("nodes");

        tokio::fs::create_dir_all(&door_dir).await.unwrap();

        // Create a door config file
        let config = create_test_config("test-door", 10);
        let config_path = door_dir.join("test-door.toml");
        config.to_file(&config_path).unwrap();

        // Create manager and verify door was loaded
        let mut manager = DoorManager::new(door_dir.clone(), node_dir).await.unwrap();
        assert_eq!(manager.door_count(), 1);

        // Add another door config file
        let config2 = create_test_config("test-door2", 20);
        let config_path2 = door_dir.join("test-door2.toml");
        config2.to_file(&config_path2).unwrap();

        // Reload and verify both doors are loaded
        manager.reload_doors().await.unwrap();
        assert_eq!(manager.door_count(), 2);
    }

    #[tokio::test]
    async fn test_add_door_validation() {
        let mut manager = create_test_manager().await;
        let mut config = create_test_config("", 10); // Empty name
        config.name = String::new();

        let result = manager.add_door(config);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_manager_creates_directories() {
        let temp_dir = tempfile::tempdir().unwrap();
        let door_dir = temp_dir.path().join("doors");
        let node_dir = temp_dir.path().join("nodes");

        assert!(!door_dir.exists());
        assert!(!node_dir.exists());

        let _manager = DoorManager::new(door_dir.clone(), node_dir.clone())
            .await
            .unwrap();

        assert!(door_dir.exists());
        assert!(node_dir.exists());
    }
}
