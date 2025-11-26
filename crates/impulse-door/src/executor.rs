//! Door executor for running door game processes.
//!
//! This module provides the `DoorExecutor` for executing door games,
//! including support for native executables and DOSBox for legacy DOS doors.

use crate::config::DoorConfig;
use crate::dropfiles::{DropfileGenerator, DropfileType};
use crate::error::{DoorError, Result};
use crate::io::DoorIoHandler;
use crate::manager::DoorManager;
use crate::session::DoorSession;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::process::Command;
use tracing::{debug, info, warn};

/// Result of a door execution.
#[derive(Debug, Clone)]
pub struct DoorResult {
    /// Exit code from the door process
    pub exit_code: i32,
    /// Runtime in seconds
    pub runtime_seconds: u64,
    /// Whether user statistics were updated
    pub user_stats_updated: bool,
}

impl DoorResult {
    /// Check if the door execution was successful.
    pub fn is_success(&self) -> bool {
        self.exit_code == 0
    }
}

/// Door executor for running door games.
///
/// The executor handles the complete lifecycle of door execution:
/// 1. Validate user access
/// 2. Create node directory and dropfiles
/// 3. Launch the door process (native or via DOSBox)
/// 4. Monitor execution and time limits
/// 5. Clean up resources
pub struct DoorExecutor {
    /// Door manager for accessing door configurations
    door_manager: Arc<DoorManager>,
    /// Path to DOSBox executable (if available)
    dosbox_path: Option<PathBuf>,
}

impl DoorExecutor {
    /// Create a new door executor.
    ///
    /// # Arguments
    ///
    /// * `door_manager` - The door manager to use for door configurations
    pub fn new(door_manager: Arc<DoorManager>) -> Self {
        // Try to find DOSBox in PATH
        let dosbox_path = which::which("dosbox").ok();

        if dosbox_path.is_some() {
            info!("DOSBox found at: {:?}", dosbox_path);
        } else {
            warn!("DOSBox not found in PATH - DOS doors will not be available");
        }

        Self {
            door_manager,
            dosbox_path,
        }
    }

    /// Execute a door game.
    ///
    /// # Arguments
    ///
    /// * `door_name` - The name of the door to execute
    /// * `session` - The door session containing user information
    ///
    /// # Returns
    ///
    /// The result of the door execution
    pub async fn execute(&self, door_name: &str, session: &mut DoorSession) -> Result<DoorResult> {
        // Get door configuration
        let config = self
            .door_manager
            .get_door(door_name)
            .ok_or_else(|| DoorError::DoorNotFound(door_name.to_string()))?;

        // Validate user access
        if !session.has_security_level(config.min_security_level) {
            return Err(DoorError::InsufficientSecurity {
                required: config.min_security_level,
                actual: session.security_level,
            });
        }

        // Check if user has time remaining
        if session.is_time_expired() {
            return Err(DoorError::TimeExpired);
        }

        info!(
            "Executing door '{}' for user '{}' on node {}",
            door_name, session.user_name, session.node_id
        );

        // Execute based on configuration
        if config.use_dosbox {
            self.execute_dosbox(config, session).await
        } else {
            self.execute_native(config, session).await
        }
    }

    /// Execute a native door (not via DOSBox).
    async fn execute_native(
        &self,
        config: &DoorConfig,
        session: &mut DoorSession,
    ) -> Result<DoorResult> {
        debug!("Executing native door: {}", config.name);

        // Prepare node directory
        let node_dir = self.door_manager.get_node_dir(session.node_id);
        tokio::fs::create_dir_all(&node_dir).await?;

        // Generate dropfiles
        self.generate_dropfiles(config.dropfile_type, session, &node_dir)
            .await?;

        // Start the door process
        let start_time = Instant::now();
        let mut child = Command::new(&config.executable)
            .current_dir(&config.directory)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .spawn()?;

        // Set up I/O handler
        let _io_handler = DoorIoHandler::new(&mut child).await?;

        // Wait for process with timeout
        let timeout_duration = if config.max_time_minutes > 0 {
            Duration::from_secs(config.max_time_minutes as u64 * 60)
        } else {
            Duration::from_secs(session.time_remaining_seconds as u64)
        };

        let exit_status = tokio::time::timeout(timeout_duration, child.wait()).await;

        let exit_code = match exit_status {
            Ok(Ok(status)) => status.code().unwrap_or(-1),
            Ok(Err(e)) => {
                warn!("Door process error: {}", e);
                -1
            }
            Err(_) => {
                warn!("Door execution timeout, killing process");
                let _ = child.kill().await;
                return Err(DoorError::Timeout(timeout_duration.as_secs()));
            }
        };

        let runtime = start_time.elapsed();

        // Update session time
        session.deduct_time(runtime.as_secs() as u32);

        // Clean up dropfiles
        self.cleanup_dropfiles(&node_dir).await;

        info!(
            "Door '{}' exited with code {} after {}s",
            config.name,
            exit_code,
            runtime.as_secs()
        );

        Ok(DoorResult {
            exit_code,
            runtime_seconds: runtime.as_secs(),
            user_stats_updated: false,
        })
    }

    /// Execute a door via DOSBox.
    async fn execute_dosbox(
        &self,
        config: &DoorConfig,
        session: &mut DoorSession,
    ) -> Result<DoorResult> {
        debug!("Executing DOSBox door: {}", config.name);

        // Check if DOSBox is available
        let dosbox_path = self
            .dosbox_path
            .as_ref()
            .ok_or_else(|| DoorError::DosBoxNotFound(PathBuf::from("dosbox")))?;

        // Prepare node directory
        let node_dir = self.door_manager.get_node_dir(session.node_id);
        tokio::fs::create_dir_all(&node_dir).await?;

        // Generate dropfiles
        self.generate_dropfiles(config.dropfile_type, session, &node_dir)
            .await?;

        // Create DOSBox configuration
        let dosbox_config_path = self.create_dosbox_config(config, &node_dir).await?;

        // Start DOSBox with the configuration
        let start_time = Instant::now();
        let mut child = Command::new(dosbox_path)
            .arg("-conf")
            .arg(&dosbox_config_path)
            .current_dir(&config.directory)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .spawn()?;

        // Set up I/O handler
        let _io_handler = DoorIoHandler::new(&mut child).await?;

        // Wait for process with timeout
        let timeout_duration = if config.max_time_minutes > 0 {
            Duration::from_secs(config.max_time_minutes as u64 * 60)
        } else {
            Duration::from_secs(session.time_remaining_seconds as u64)
        };

        let exit_status = tokio::time::timeout(timeout_duration, child.wait()).await;

        let exit_code = match exit_status {
            Ok(Ok(status)) => status.code().unwrap_or(-1),
            Ok(Err(e)) => {
                warn!("DOSBox process error: {}", e);
                -1
            }
            Err(_) => {
                warn!("DOSBox execution timeout, killing process");
                let _ = child.kill().await;
                return Err(DoorError::Timeout(timeout_duration.as_secs()));
            }
        };

        let runtime = start_time.elapsed();

        // Update session time
        session.deduct_time(runtime.as_secs() as u32);

        // Clean up
        self.cleanup_dropfiles(&node_dir).await;
        let _ = tokio::fs::remove_file(dosbox_config_path).await;

        info!(
            "DOSBox door '{}' exited with code {} after {}s",
            config.name,
            exit_code,
            runtime.as_secs()
        );

        Ok(DoorResult {
            exit_code,
            runtime_seconds: runtime.as_secs(),
            user_stats_updated: false,
        })
    }

    /// Create a DOSBox configuration file for the door.
    async fn create_dosbox_config(&self, config: &DoorConfig, node_path: &Path) -> Result<PathBuf> {
        let dosbox_config = config
            .dosbox_config
            .as_ref()
            .ok_or_else(|| DoorError::Config("DOSBox configuration missing".to_string()))?;

        let config_path = node_path.join("dosbox.conf");
        let config_content = dosbox_config.generate_config();

        tokio::fs::write(&config_path, config_content).await?;

        Ok(config_path)
    }

    /// Generate dropfiles for the door.
    async fn generate_dropfiles(
        &self,
        dropfile_type: DropfileType,
        session: &DoorSession,
        node_dir: &Path,
    ) -> Result<()> {
        debug!("Generating {} dropfile", dropfile_type.description());
        DropfileGenerator::generate(dropfile_type, session, node_dir)?;
        Ok(())
    }

    /// Clean up dropfiles after door execution.
    async fn cleanup_dropfiles(&self, node_dir: &Path) {
        let dropfiles = ["DOOR.SYS", "DORINFO1.DEF", "CHAIN.TXT", "CALLINFO.BBS"];

        for filename in &dropfiles {
            let path = node_dir.join(filename);
            if path.exists() && tokio::fs::remove_file(&path).await.is_err() {
                warn!("Failed to remove dropfile {:?}", path);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    // Helper function to get a platform-specific test executable path
    fn test_executable() -> PathBuf {
        #[cfg(unix)]
        {
            PathBuf::from("/bin/ls")
        }
        #[cfg(windows)]
        {
            PathBuf::from("C:\\Windows\\System32\\cmd.exe")
        }
    }

    // Helper function to get a platform-specific test directory path
    fn test_directory() -> PathBuf {
        std::env::temp_dir()
    }

    async fn create_test_executor() -> DoorExecutor {
        let temp_dir = tempfile::tempdir().unwrap();
        let door_dir = temp_dir.path().join("doors");
        let node_dir = temp_dir.path().join("nodes");

        let manager = Arc::new(DoorManager::new(door_dir, node_dir).await.unwrap());
        DoorExecutor::new(manager)
    }

    fn create_test_session() -> DoorSession {
        DoorSession {
            node_id: 1,
            user_name: "Test User".to_string(),
            user_alias: None,
            location: "Test Location".to_string(),
            security_level: 100,
            time_remaining_seconds: 3600,
            ansi_enabled: true,
            login_time: Utc::now(),
            total_calls: 1,
            last_call_date: "11/26/25".to_string(),
            upload_kb: 0,
            download_kb: 0,
        }
    }

    #[tokio::test]
    async fn test_executor_new() {
        let executor = create_test_executor().await;
        // Executor should be created successfully - verify non-existent door returns error
        let mut session = create_test_session();
        let result = executor.execute("__nonexistent__", &mut session).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_execute_door_not_found() {
        let executor = create_test_executor().await;
        let mut session = create_test_session();

        let result = executor.execute("nonexistent-door", &mut session).await;
        assert!(result.is_err());
        assert!(matches!(result, Err(DoorError::DoorNotFound(_))));
    }

    #[tokio::test]
    async fn test_execute_insufficient_security() {
        let temp_dir = tempfile::tempdir().unwrap();
        let door_dir = temp_dir.path().join("doors");
        let node_dir = temp_dir.path().join("nodes");

        let mut manager = DoorManager::new(door_dir, node_dir).await.unwrap();

        // Add a door with high security requirement
        let mut config = DoorConfig::new(
            "secure-door".to_string(),
            test_executable(),
            test_directory(),
        );
        config.min_security_level = 200;

        manager.add_door(config).unwrap();

        let executor = DoorExecutor::new(Arc::new(manager));

        // Create session with low security level
        let mut session = create_test_session();
        session.security_level = 50;

        let result = executor.execute("secure-door", &mut session).await;
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(DoorError::InsufficientSecurity { .. })
        ));
    }

    #[tokio::test]
    async fn test_execute_time_expired() {
        let temp_dir = tempfile::tempdir().unwrap();
        let door_dir = temp_dir.path().join("doors");
        let node_dir = temp_dir.path().join("nodes");

        let mut manager = DoorManager::new(door_dir, node_dir).await.unwrap();

        let config = DoorConfig::new("test-door".to_string(), test_executable(), test_directory());

        manager.add_door(config).unwrap();

        let executor = DoorExecutor::new(Arc::new(manager));

        // Create session with no time remaining
        let mut session = create_test_session();
        session.time_remaining_seconds = 0;

        let result = executor.execute("test-door", &mut session).await;
        assert!(result.is_err());
        assert!(matches!(result, Err(DoorError::TimeExpired)));
    }

    #[tokio::test]
    async fn test_door_result_is_success() {
        let result = DoorResult {
            exit_code: 0,
            runtime_seconds: 10,
            user_stats_updated: false,
        };
        assert!(result.is_success());

        let result = DoorResult {
            exit_code: 1,
            runtime_seconds: 10,
            user_stats_updated: false,
        };
        assert!(!result.is_success());
    }

    #[tokio::test]
    async fn test_door_result_clone() {
        let result = DoorResult {
            exit_code: 0,
            runtime_seconds: 10,
            user_stats_updated: true,
        };

        let cloned = result.clone();
        assert_eq!(result.exit_code, cloned.exit_code);
        assert_eq!(result.runtime_seconds, cloned.runtime_seconds);
        assert_eq!(result.user_stats_updated, cloned.user_stats_updated);
    }

    #[tokio::test]
    async fn test_executor_cleanup_dropfiles() {
        let temp_dir = tempfile::tempdir().unwrap();
        let door_dir = temp_dir.path().join("doors");
        let node_dir = temp_dir.path().join("nodes");
        let test_node_dir = node_dir.join("node1");

        tokio::fs::create_dir_all(&test_node_dir).await.unwrap();

        // Create some test dropfiles
        tokio::fs::write(test_node_dir.join("DOOR.SYS"), "test")
            .await
            .unwrap();
        tokio::fs::write(test_node_dir.join("DORINFO1.DEF"), "test")
            .await
            .unwrap();

        let manager = Arc::new(DoorManager::new(door_dir, node_dir).await.unwrap());
        let executor = DoorExecutor::new(manager);

        executor.cleanup_dropfiles(&test_node_dir).await;

        // Files should be removed
        assert!(!test_node_dir.join("DOOR.SYS").exists());
        assert!(!test_node_dir.join("DORINFO1.DEF").exists());
    }
}
