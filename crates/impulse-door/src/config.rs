//! Door configuration.
//!
//! This module provides configuration structures for door games,
//! including support for DOSBox for running legacy DOS doors.

use crate::dropfiles::DropfileType;
use crate::error::{DoorError, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Door game configuration.
///
/// This structure defines all the settings needed to execute a door game,
/// including paths, security requirements, and DOSBox configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoorConfig {
    /// Unique name/identifier for the door
    pub name: String,
    /// Human-readable description
    pub description: String,
    /// Path to the door executable
    pub executable: PathBuf,
    /// Working directory for the door
    pub directory: PathBuf,
    /// Type of dropfile to generate
    pub dropfile_type: DropfileType,
    /// Minimum security level required (0-255)
    pub min_security_level: u8,
    /// Maximum time limit in minutes (0 = unlimited)
    pub max_time_minutes: u16,
    /// Use DOSBox to run this door
    pub use_dosbox: bool,
    /// DOSBox configuration (if use_dosbox is true)
    pub dosbox_config: Option<DosBoxConfig>,
}

impl DoorConfig {
    /// Create a new door configuration.
    pub fn new(name: String, executable: PathBuf, directory: PathBuf) -> Self {
        Self {
            name,
            description: String::new(),
            executable,
            directory,
            dropfile_type: DropfileType::DoorSys,
            min_security_level: 0,
            max_time_minutes: 60,
            use_dosbox: false,
            dosbox_config: None,
        }
    }

    /// Validate the door configuration.
    pub fn validate(&self) -> Result<()> {
        if self.name.is_empty() {
            return Err(DoorError::InvalidConfig("Door name cannot be empty".to_string()));
        }

        if !self.executable.exists() {
            return Err(DoorError::ExecutableNotFound(self.executable.clone()));
        }

        if !self.directory.exists() {
            return Err(DoorError::DirectoryNotFound(self.directory.clone()));
        }

        if self.use_dosbox && self.dosbox_config.is_none() {
            return Err(DoorError::InvalidConfig(
                "DOSBox enabled but no DOSBox configuration provided".to_string(),
            ));
        }

        Ok(())
    }

    /// Load door configuration from a TOML file.
    pub fn from_file(path: &std::path::Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)?;
        config.validate()?;
        Ok(config)
    }

    /// Save door configuration to a TOML file.
    pub fn to_file(&self, path: &std::path::Path) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}

/// DOSBox configuration for running legacy DOS doors.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DosBoxConfig {
    /// CPU cycles setting ("auto", "max", or a number like "10000")
    pub cycles: String,
    /// Machine type (e.g., "svga_s3", "vga", "cga", "hercules")
    pub machine: String,
    /// Memory size in MB (typically 16)
    pub memsize: u16,
    /// Mount points as (drive_letter, path) pairs
    #[serde(default)]
    pub mount_points: Vec<(char, PathBuf)>,
}

impl DosBoxConfig {
    /// Create a new DOSBox configuration with sensible defaults.
    pub fn new() -> Self {
        Self {
            cycles: "auto".to_string(),
            machine: "svga_s3".to_string(),
            memsize: 16,
            mount_points: Vec::new(),
        }
    }

    /// Add a mount point.
    pub fn add_mount(&mut self, drive_letter: char, path: PathBuf) {
        self.mount_points.push((drive_letter, path));
    }

    /// Generate DOSBox configuration file content.
    pub fn generate_config(&self) -> String {
        let mut config = String::new();

        config.push_str("[cpu]\n");
        config.push_str(&format!("cycles={}\n\n", self.cycles));

        config.push_str("[dosbox]\n");
        config.push_str(&format!("machine={}\n", self.machine));
        config.push_str(&format!("memsize={}\n\n", self.memsize));

        config.push_str("[autoexec]\n");
        for (drive, path) in &self.mount_points {
            config.push_str(&format!(
                "mount {} \"{}\"\n",
                drive.to_uppercase(),
                path.display()
            ));
        }

        config
    }
}

impl Default for DosBoxConfig {
    fn default() -> Self {
        Self::new()
    }
}

// Custom serialization for DropfileType
impl Serialize for DropfileType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = match self {
            DropfileType::DoorSys => "DoorSys",
            DropfileType::Dorinfo1Def => "Dorinfo1Def",
            DropfileType::DorinfoDef => "DorinfoDef",
            DropfileType::ChainTxt => "ChainTxt",
            DropfileType::CallInfo => "CallInfo",
        };
        serializer.serialize_str(s)
    }
}

// Custom deserialization for DropfileType
impl<'de> Deserialize<'de> for DropfileType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "DoorSys" => Ok(DropfileType::DoorSys),
            "Dorinfo1Def" => Ok(DropfileType::Dorinfo1Def),
            "DorinfoDef" => Ok(DropfileType::DorinfoDef),
            "ChainTxt" => Ok(DropfileType::ChainTxt),
            "CallInfo" => Ok(DropfileType::CallInfo),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown dropfile type: {}",
                s
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_door_config_new() {
        let config = DoorConfig::new(
            "test-door".to_string(),
            PathBuf::from("/bin/ls"),
            PathBuf::from("/tmp"),
        );

        assert_eq!(config.name, "test-door");
        assert_eq!(config.executable, PathBuf::from("/bin/ls"));
        assert_eq!(config.directory, PathBuf::from("/tmp"));
        assert_eq!(config.min_security_level, 0);
        assert_eq!(config.max_time_minutes, 60);
        assert!(!config.use_dosbox);
        assert!(config.dosbox_config.is_none());
    }

    #[test]
    fn test_door_config_validate_empty_name() {
        let config = DoorConfig {
            name: String::new(),
            description: String::new(),
            executable: PathBuf::from("/bin/ls"),
            directory: PathBuf::from("/tmp"),
            dropfile_type: DropfileType::DoorSys,
            min_security_level: 0,
            max_time_minutes: 60,
            use_dosbox: false,
            dosbox_config: None,
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_door_config_validate_missing_executable() {
        let config = DoorConfig::new(
            "test".to_string(),
            PathBuf::from("/nonexistent/executable"),
            PathBuf::from("/tmp"),
        );

        let result = config.validate();
        assert!(result.is_err());
        assert!(matches!(result, Err(DoorError::ExecutableNotFound(_))));
    }

    #[test]
    fn test_door_config_validate_missing_directory() {
        let config = DoorConfig::new(
            "test".to_string(),
            PathBuf::from("/bin/ls"),
            PathBuf::from("/nonexistent/directory"),
        );

        let result = config.validate();
        assert!(result.is_err());
        assert!(matches!(result, Err(DoorError::DirectoryNotFound(_))));
    }

    #[test]
    fn test_door_config_validate_dosbox_no_config() {
        let mut config = DoorConfig::new(
            "test".to_string(),
            PathBuf::from("/bin/ls"),
            PathBuf::from("/tmp"),
        );
        config.use_dosbox = true;

        let result = config.validate();
        assert!(result.is_err());
        assert!(matches!(result, Err(DoorError::InvalidConfig(_))));
    }

    #[test]
    fn test_dosbox_config_new() {
        let config = DosBoxConfig::new();
        assert_eq!(config.cycles, "auto");
        assert_eq!(config.machine, "svga_s3");
        assert_eq!(config.memsize, 16);
        assert!(config.mount_points.is_empty());
    }

    #[test]
    fn test_dosbox_config_default() {
        let config = DosBoxConfig::default();
        assert_eq!(config.cycles, "auto");
        assert_eq!(config.machine, "svga_s3");
        assert_eq!(config.memsize, 16);
    }

    #[test]
    fn test_dosbox_config_add_mount() {
        let mut config = DosBoxConfig::new();
        config.add_mount('C', PathBuf::from("/dos/games"));
        config.add_mount('D', PathBuf::from("/dos/doors"));

        assert_eq!(config.mount_points.len(), 2);
        assert_eq!(config.mount_points[0].0, 'C');
        assert_eq!(config.mount_points[1].0, 'D');
    }

    #[test]
    fn test_dosbox_config_generate_config() {
        let mut config = DosBoxConfig::new();
        config.cycles = "10000".to_string();
        config.machine = "vga".to_string();
        config.memsize = 32;
        config.add_mount('C', PathBuf::from("/dos/games"));

        let content = config.generate_config();

        assert!(content.contains("cycles=10000"));
        assert!(content.contains("machine=vga"));
        assert!(content.contains("memsize=32"));
        assert!(content.contains("mount C"));
    }

    #[test]
    fn test_dropfile_type_serialize() {
        // Test within a struct context (TOML requires key-value pairs)
        #[derive(serde::Serialize)]
        struct TestConfig {
            dropfile_type: DropfileType,
        }

        let config = TestConfig {
            dropfile_type: DropfileType::DoorSys,
        };
        let serialized = toml::to_string(&config).unwrap();
        assert!(serialized.contains("dropfile_type = \"DoorSys\""));
    }

    #[test]
    fn test_dropfile_type_deserialize() {
        #[derive(serde::Deserialize)]
        struct TestConfig {
            dropfile_type: DropfileType,
        }

        let toml_str = "dropfile_type = \"DoorSys\"";
        let config: TestConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.dropfile_type, DropfileType::DoorSys);
    }

    #[test]
    fn test_dropfile_type_deserialize_invalid() {
        #[derive(serde::Deserialize)]
        struct TestConfig {
            #[allow(dead_code)]
            dropfile_type: DropfileType,
        }

        let toml_str = "dropfile_type = \"InvalidType\"";
        let result: std::result::Result<TestConfig, _> = toml::from_str(toml_str);
        assert!(result.is_err());
    }

    #[test]
    fn test_door_config_serialize() {
        let config = DoorConfig::new(
            "test-door".to_string(),
            PathBuf::from("/bin/ls"),
            PathBuf::from("/tmp"),
        );

        let serialized = toml::to_string(&config).unwrap();
        assert!(serialized.contains("name = \"test-door\""));
        assert!(serialized.contains("dropfile_type = \"DoorSys\""));
    }

    #[test]
    fn test_door_config_roundtrip() {
        let config = DoorConfig::new(
            "test-door".to_string(),
            PathBuf::from("/bin/ls"),
            PathBuf::from("/tmp"),
        );

        let serialized = toml::to_string(&config).unwrap();
        let deserialized: DoorConfig = toml::from_str(&serialized).unwrap();

        assert_eq!(config.name, deserialized.name);
        assert_eq!(config.min_security_level, deserialized.min_security_level);
        assert_eq!(config.use_dosbox, deserialized.use_dosbox);
    }

    #[test]
    fn test_door_config_to_from_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path().join("door.toml");

        let mut config = DoorConfig::new(
            "test-door".to_string(),
            PathBuf::from("/bin/ls"),
            PathBuf::from("/tmp"),
        );
        config.description = "Test door game".to_string();
        config.min_security_level = 50;

        // Write to file
        config.to_file(&config_path).unwrap();
        assert!(config_path.exists());

        // Read from file
        let loaded = DoorConfig::from_file(&config_path).unwrap();
        assert_eq!(loaded.name, "test-door");
        assert_eq!(loaded.description, "Test door game");
        assert_eq!(loaded.min_security_level, 50);
    }

    #[test]
    fn test_dosbox_config_clone() {
        let mut config = DosBoxConfig::new();
        config.add_mount('C', PathBuf::from("/dos"));

        let cloned = config.clone();
        assert_eq!(config.cycles, cloned.cycles);
        assert_eq!(config.mount_points.len(), cloned.mount_points.len());
    }

    #[test]
    fn test_door_config_with_dosbox() {
        let mut config = DoorConfig::new(
            "dos-door".to_string(),
            PathBuf::from("/bin/ls"),
            PathBuf::from("/tmp"),
        );
        config.use_dosbox = true;
        config.dosbox_config = Some(DosBoxConfig::new());

        let result = config.validate();
        assert!(result.is_ok());
    }
}
