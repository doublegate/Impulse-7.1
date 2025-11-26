//! File system watching for configuration hot-reload

use crate::error::{ConfigError, Result};
use notify::{
    Config as NotifyConfig, Event, EventKind, RecommendedWatcher, RecursiveMode,
    Watcher as NotifyWatcher,
};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time;

/// Configuration file watcher for hot-reload functionality
///
/// Watches a configuration file for modifications and emits notifications
/// when changes are detected. Includes debouncing to handle rapid successive
/// changes (e.g., editor save operations that write multiple times).
///
/// # Example
/// ```no_run
/// use impulse_config::watcher::ConfigWatcher;
/// use std::path::Path;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let (watcher, mut rx) = ConfigWatcher::new(Path::new("config.toml"))?;
///
///     // Start watching in a background task
///     tokio::spawn(async move {
///         watcher.watch().await;
///     });
///
///     // Receive change notifications
///     while let Some(_) = rx.recv().await {
///         println!("Config file changed!");
///     }
///
///     Ok(())
/// }
/// ```
pub struct ConfigWatcher {
    path: PathBuf,
    debounce_duration: Duration,
    tx: mpsc::Sender<()>,
}

impl ConfigWatcher {
    /// Create a new configuration file watcher
    ///
    /// Returns a tuple of (watcher, receiver) where:
    /// - `watcher` is the `ConfigWatcher` instance to call `watch()` on
    /// - `receiver` receives notifications when the file changes
    ///
    /// # Arguments
    /// * `path` - Path to the configuration file to watch
    ///
    /// # Errors
    /// Returns `ConfigError` if the path does not exist
    pub fn new<P: AsRef<Path>>(path: P) -> Result<(Self, mpsc::Receiver<()>)> {
        let path = path.as_ref().to_path_buf();

        // Verify path exists
        if !path.exists() {
            return Err(ConfigError::PathNotFound(path));
        }

        let (tx, rx) = mpsc::channel(10);

        let watcher = Self {
            path,
            debounce_duration: Duration::from_millis(500),
            tx,
        };

        Ok((watcher, rx))
    }

    /// Create a watcher with custom debounce duration
    ///
    /// # Arguments
    /// * `path` - Path to the configuration file to watch
    /// * `debounce_duration` - Time to wait after last change before notifying
    ///
    /// # Errors
    /// Returns `ConfigError` if the path does not exist
    pub fn with_debounce<P: AsRef<Path>>(
        path: P,
        debounce_duration: Duration,
    ) -> Result<(Self, mpsc::Receiver<()>)> {
        let path = path.as_ref().to_path_buf();

        if !path.exists() {
            return Err(ConfigError::PathNotFound(path));
        }

        let (tx, rx) = mpsc::channel(10);

        let watcher = Self {
            path,
            debounce_duration,
            tx,
        };

        Ok((watcher, rx))
    }

    /// Start watching the configuration file
    ///
    /// This method runs indefinitely and should be spawned as a background task.
    /// It watches for file modifications and sends notifications through the
    /// channel after debouncing rapid changes.
    ///
    /// # Example
    /// ```no_run
    /// use impulse_config::watcher::ConfigWatcher;
    /// use std::path::Path;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let (watcher, mut rx) = ConfigWatcher::new(Path::new("config.toml"))?;
    ///
    ///     tokio::spawn(async move {
    ///         watcher.watch().await;
    ///     });
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn watch(self) {
        let path = Arc::new(self.path);
        let tx = self.tx;
        let debounce_duration = self.debounce_duration;

        // Create channel for notify events
        let (notify_tx, mut notify_rx) = mpsc::channel(100);

        // Set up file watcher
        let notify_path = Arc::clone(&path);
        let mut watcher = match RecommendedWatcher::new(
            move |res: notify::Result<Event>| {
                if let Ok(event) = res {
                    // Only care about modify events
                    if matches!(event.kind, EventKind::Modify(_)) {
                        let _ = notify_tx.blocking_send(event);
                    }
                }
            },
            NotifyConfig::default(),
        ) {
            Ok(w) => w,
            Err(e) => {
                eprintln!("Failed to create file watcher: {}", e);
                return;
            }
        };

        // Start watching the file
        if let Err(e) = watcher.watch(&notify_path, RecursiveMode::NonRecursive) {
            eprintln!("Failed to watch file {}: {}", notify_path.display(), e);
            return;
        }

        // Debouncing logic: wait for a quiet period before notifying
        let mut last_event: Option<tokio::time::Instant> = None;

        loop {
            tokio::select! {
                // Receive events from notify
                Some(_event) = notify_rx.recv() => {
                    last_event = Some(tokio::time::Instant::now());
                }

                // Check debounce timeout
                _ = time::sleep(Duration::from_millis(100)) => {
                    if let Some(last) = last_event
                        && last.elapsed() >= debounce_duration
                    {
                        // Send notification
                        if tx.send(()).await.is_err() {
                            // Receiver dropped, stop watching
                            break;
                        }
                        last_event = None;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use tokio::time::timeout;

    #[test]
    fn test_new_nonexistent_path() {
        let result = ConfigWatcher::new("/nonexistent/path/config.toml");
        assert!(result.is_err());
    }

    #[test]
    fn test_new_valid_path() {
        let temp_file = NamedTempFile::new().unwrap();
        let result = ConfigWatcher::new(temp_file.path());
        assert!(result.is_ok());
    }

    #[test]
    fn test_with_debounce() {
        let temp_file = NamedTempFile::new().unwrap();
        let debounce = Duration::from_millis(1000);
        let result = ConfigWatcher::with_debounce(temp_file.path(), debounce);
        assert!(result.is_ok());

        let (watcher, _rx) = result.unwrap();
        assert_eq!(watcher.debounce_duration, debounce);
    }

    #[tokio::test]
    async fn test_watch_detects_changes() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_path_buf();

        // Create watcher with short debounce for testing
        let (watcher, mut rx) =
            ConfigWatcher::with_debounce(&path, Duration::from_millis(100)).unwrap();

        // Start watching in background
        tokio::spawn(async move {
            watcher.watch().await;
        });

        // Give watcher time to start
        tokio::time::sleep(Duration::from_millis(50)).await;

        // Modify the file
        std::fs::write(&path, "test content").unwrap();

        // Should receive notification within reasonable time
        let result = timeout(Duration::from_secs(2), rx.recv()).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[tokio::test]
    async fn test_debouncing() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_path_buf();

        // Create watcher with longer debounce
        let (watcher, mut rx) =
            ConfigWatcher::with_debounce(&path, Duration::from_millis(300)).unwrap();

        tokio::spawn(async move {
            watcher.watch().await;
        });

        tokio::time::sleep(Duration::from_millis(50)).await;

        // Write multiple times rapidly
        std::fs::write(&path, "content 1").unwrap();
        tokio::time::sleep(Duration::from_millis(50)).await;
        std::fs::write(&path, "content 2").unwrap();
        tokio::time::sleep(Duration::from_millis(50)).await;
        std::fs::write(&path, "content 3").unwrap();

        // Should only receive ONE notification after debounce period
        let result = timeout(Duration::from_secs(2), rx.recv()).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());

        // Second receive should timeout (no second notification)
        let result2 = timeout(Duration::from_millis(200), rx.recv()).await;
        assert!(result2.is_err()); // Timeout = good, means only one notification
    }
}
