//! Configuration reload notifications and event handling

use crate::Config;
use impulse_types::config::BbsConfig;
use tokio::sync::broadcast;

/// Events emitted during configuration reload process
#[derive(Debug, Clone)]
pub enum ConfigEvent {
    /// Configuration reload is starting
    Reloading,

    /// Configuration was successfully reloaded
    ///
    /// Contains the old and new configuration for comparison
    Reloaded {
        /// Configuration before reload
        old_config: BbsConfig,
        /// Configuration after reload
        new_config: BbsConfig,
    },

    /// Configuration reload failed
    ///
    /// Contains the error message describing why reload failed.
    /// The old configuration remains in effect.
    ReloadFailed {
        /// Error message
        error: String,
    },
}

/// Configuration reload notification system
///
/// Uses tokio broadcast channels to notify multiple subscribers about
/// configuration reload events. Subscribers can react to configuration
/// changes, such as reconnecting to databases or updating runtime settings.
///
/// # Example
/// ```
/// use impulse_config::reload::{ReloadNotifier, ConfigEvent};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let notifier = ReloadNotifier::new();
///     let mut subscriber = notifier.subscribe();
///
///     // In another task, emit events
///     tokio::spawn(async move {
///         notifier.notify_reloading();
///     });
///
///     // Receive events
///     if let Ok(event) = subscriber.recv().await {
///         match event {
///             ConfigEvent::Reloading => println!("Config is reloading..."),
///             ConfigEvent::Reloaded { .. } => println!("Config reloaded!"),
///             ConfigEvent::ReloadFailed { error } => println!("Reload failed: {}", error),
///         }
///     }
///
///     Ok(())
/// }
/// ```
pub struct ReloadNotifier {
    tx: broadcast::Sender<ConfigEvent>,
}

impl ReloadNotifier {
    /// Create a new reload notifier
    ///
    /// The notifier uses a broadcast channel with a buffer of 100 events.
    /// If subscribers lag behind, oldest events will be dropped.
    pub fn new() -> Self {
        let (tx, _rx) = broadcast::channel(100);
        Self { tx }
    }

    /// Create a new reload notifier with custom buffer size
    ///
    /// # Arguments
    /// * `capacity` - Number of events to buffer before dropping old events
    pub fn with_capacity(capacity: usize) -> Self {
        let (tx, _rx) = broadcast::channel(capacity);
        Self { tx }
    }

    /// Subscribe to reload notifications
    ///
    /// Returns a receiver that will receive all future `ConfigEvent` notifications.
    /// Multiple subscribers can exist simultaneously.
    ///
    /// # Example
    /// ```
    /// use impulse_config::reload::ReloadNotifier;
    ///
    /// let notifier = ReloadNotifier::new();
    /// let subscriber1 = notifier.subscribe();
    /// let subscriber2 = notifier.subscribe();
    /// // Both subscribers will receive events independently
    /// ```
    pub fn subscribe(&self) -> broadcast::Receiver<ConfigEvent> {
        self.tx.subscribe()
    }

    /// Get the number of active subscribers
    ///
    /// Useful for monitoring how many components are listening to config changes.
    pub fn subscriber_count(&self) -> usize {
        self.tx.receiver_count()
    }

    /// Notify that configuration reload is starting
    ///
    /// Emits `ConfigEvent::Reloading` to all subscribers.
    pub fn notify_reloading(&self) {
        let _ = self.tx.send(ConfigEvent::Reloading);
    }

    /// Notify that configuration reload succeeded
    ///
    /// Emits `ConfigEvent::Reloaded` with old and new configurations to all subscribers.
    ///
    /// # Arguments
    /// * `old_config` - Configuration before reload
    /// * `new_config` - Configuration after reload
    pub fn notify_reloaded(&self, old_config: BbsConfig, new_config: BbsConfig) {
        let _ = self.tx.send(ConfigEvent::Reloaded {
            old_config,
            new_config,
        });
    }

    /// Notify that configuration reload failed
    ///
    /// Emits `ConfigEvent::ReloadFailed` with error message to all subscribers.
    ///
    /// # Arguments
    /// * `error` - Error message describing the failure
    pub fn notify_reload_failed<S: Into<String>>(&self, error: S) {
        let _ = self.tx.send(ConfigEvent::ReloadFailed {
            error: error.into(),
        });
    }

    /// Convenience method to reload configuration and notify subscribers
    ///
    /// Attempts to reload configuration from the specified path and emits
    /// appropriate events based on success or failure.
    ///
    /// # Arguments
    /// * `path` - Path to configuration file
    /// * `current_config` - Current configuration (cloned for comparison)
    ///
    /// # Returns
    /// Returns `Some(Config)` if reload succeeded, `None` if it failed.
    ///
    /// # Example
    /// ```no_run
    /// use impulse_config::{Config, reload::ReloadNotifier};
    /// use std::path::Path;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let notifier = ReloadNotifier::new();
    ///     let current = Config::load("config.toml")?;
    ///
    ///     if let Some(new_config) = notifier.reload_and_notify(Path::new("config.toml"), &current) {
    ///         println!("Config reloaded successfully!");
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn reload_and_notify<P: AsRef<std::path::Path>>(
        &self,
        path: P,
        current_config: &Config,
    ) -> Option<Config> {
        self.notify_reloading();

        match Config::load(path) {
            Ok(new_config) => {
                let old_config = current_config.inner().clone();
                let new_config_inner = new_config.inner().clone();

                self.notify_reloaded(old_config, new_config_inner);
                Some(new_config)
            }
            Err(e) => {
                self.notify_reload_failed(e.to_string());
                None
            }
        }
    }
}

impl Default for ReloadNotifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{Duration, timeout};

    #[test]
    fn test_new_notifier() {
        let notifier = ReloadNotifier::new();
        assert_eq!(notifier.subscriber_count(), 0);
    }

    #[test]
    fn test_subscribe() {
        let notifier = ReloadNotifier::new();
        let _sub1 = notifier.subscribe();
        assert_eq!(notifier.subscriber_count(), 1);

        let _sub2 = notifier.subscribe();
        assert_eq!(notifier.subscriber_count(), 2);
    }

    #[tokio::test]
    async fn test_notify_reloading() {
        let notifier = ReloadNotifier::new();
        let mut subscriber = notifier.subscribe();

        notifier.notify_reloading();

        let result = timeout(Duration::from_millis(100), subscriber.recv()).await;
        assert!(result.is_ok());

        if let Ok(Ok(event)) = result {
            assert!(matches!(event, ConfigEvent::Reloading));
        } else {
            panic!("Expected Reloading event");
        }
    }

    #[tokio::test]
    async fn test_notify_reloaded() {
        let notifier = ReloadNotifier::new();
        let mut subscriber = notifier.subscribe();

        let old_config = BbsConfig::default();
        let mut new_config = BbsConfig::default();
        new_config.name = "Updated BBS".to_string();

        notifier.notify_reloaded(old_config.clone(), new_config.clone());

        let result = timeout(Duration::from_millis(100), subscriber.recv()).await;
        assert!(result.is_ok());

        if let Ok(Ok(ConfigEvent::Reloaded {
            old_config: old,
            new_config: new,
        })) = result
        {
            assert_eq!(old.name, "Impulse BBS");
            assert_eq!(new.name, "Updated BBS");
        } else {
            panic!("Expected Reloaded event");
        }
    }

    #[tokio::test]
    async fn test_notify_reload_failed() {
        let notifier = ReloadNotifier::new();
        let mut subscriber = notifier.subscribe();

        notifier.notify_reload_failed("Test error");

        let result = timeout(Duration::from_millis(100), subscriber.recv()).await;
        assert!(result.is_ok());

        if let Ok(Ok(ConfigEvent::ReloadFailed { error })) = result {
            assert_eq!(error, "Test error");
        } else {
            panic!("Expected ReloadFailed event");
        }
    }

    #[tokio::test]
    async fn test_multiple_subscribers() {
        let notifier = ReloadNotifier::new();
        let mut sub1 = notifier.subscribe();
        let mut sub2 = notifier.subscribe();

        notifier.notify_reloading();

        // Both subscribers should receive the event
        let result1 = timeout(Duration::from_millis(100), sub1.recv()).await;
        let result2 = timeout(Duration::from_millis(100), sub2.recv()).await;

        assert!(result1.is_ok());
        assert!(result2.is_ok());
    }

    #[test]
    fn test_reload_and_notify_invalid_path() {
        let notifier = ReloadNotifier::new();
        let current = Config::with_defaults();

        let result = notifier.reload_and_notify("/nonexistent/config.toml", &current);
        assert!(result.is_none());
    }

    #[test]
    fn test_reload_and_notify_success() {
        use tempfile::NamedTempFile;

        let notifier = ReloadNotifier::new();
        let temp_file = NamedTempFile::new().unwrap();

        // Create a valid config file
        let config = Config::with_defaults();
        config.save(temp_file.path()).unwrap();

        let result = notifier.reload_and_notify(temp_file.path(), &config);
        assert!(result.is_some());
    }

    #[test]
    fn test_with_capacity() {
        let notifier = ReloadNotifier::with_capacity(50);
        assert_eq!(notifier.subscriber_count(), 0);
    }

    #[test]
    fn test_default() {
        let notifier = ReloadNotifier::default();
        assert_eq!(notifier.subscriber_count(), 0);
    }
}
