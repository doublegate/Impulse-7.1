//! Configuration reload hooks for service components

use crate::Config;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Result type for reload handlers
pub type HookResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

/// Trait for components that need to react to configuration changes
///
/// Services implement this trait to handle configuration reloads.
/// When configuration is reloaded, all registered handlers are called
/// with the old and new configuration.
///
/// # Example
/// ```
/// use impulse_config::{Config, hooks::{ReloadHandler, HookResult}};
/// use async_trait::async_trait;
///
/// struct DatabaseService {
///     // ... fields
/// }
///
/// #[async_trait]
/// impl ReloadHandler for DatabaseService {
///     async fn on_reload(&mut self, old_config: &Config, new_config: &Config) -> HookResult {
///         // Reconnect to database if connection string changed
///         if old_config.inner().paths.data_dir != new_config.inner().paths.data_dir {
///             println!("Data path changed, reconnecting...");
///             // ... reconnection logic
///         }
///         Ok(())
///     }
///
///     fn name(&self) -> &str {
///         "DatabaseService"
///     }
/// }
/// ```
#[async_trait]
pub trait ReloadHandler: Send + Sync {
    /// Called when configuration is successfully reloaded
    ///
    /// # Arguments
    /// * `old_config` - Configuration before reload
    /// * `new_config` - Configuration after reload
    ///
    /// # Returns
    /// Returns `Ok(())` if the component successfully adapted to the new configuration.
    /// Returns `Err` if the component failed to adapt (e.g., failed to reconnect).
    ///
    /// # Note
    /// This method should complete quickly. Long-running operations should be
    /// spawned as background tasks.
    async fn on_reload(&mut self, old_config: &Config, new_config: &Config) -> HookResult;

    /// Human-readable name for this handler (for logging/debugging)
    fn name(&self) -> &str;
}

/// Manager for configuration reload hooks
///
/// Maintains a registry of components that need to be notified when
/// configuration changes. Executes all handlers and aggregates errors.
///
/// # Example
/// ```
/// use impulse_config::{Config, hooks::{HookManager, ReloadHandler, HookResult}};
/// use async_trait::async_trait;
///
/// struct MyService;
///
/// #[async_trait]
/// impl ReloadHandler for MyService {
///     async fn on_reload(&mut self, _old: &Config, _new: &Config) -> HookResult {
///         println!("Config changed!");
///         Ok(())
///     }
///     fn name(&self) -> &str { "MyService" }
/// }
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let mut manager = HookManager::new();
///     manager.register("my_service", Box::new(MyService)).await;
///
///     let old_config = Config::with_defaults();
///     let new_config = Config::with_defaults();
///
///     manager.execute_all(&old_config, &new_config).await;
///
///     Ok(())
/// }
/// ```
pub struct HookManager {
    handlers: Arc<RwLock<HashMap<String, Box<dyn ReloadHandler>>>>,
}

impl HookManager {
    /// Create a new hook manager
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a reload handler
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this handler (for unregistering)
    /// * `handler` - The handler implementation
    ///
    /// # Example
    /// ```
    /// use impulse_config::hooks::{HookManager, ReloadHandler, HookResult};
    /// use impulse_config::Config;
    /// use async_trait::async_trait;
    ///
    /// struct MyHandler;
    ///
    /// #[async_trait]
    /// impl ReloadHandler for MyHandler {
    ///     async fn on_reload(&mut self, _old: &Config, _new: &Config) -> HookResult {
    ///         Ok(())
    ///     }
    ///     fn name(&self) -> &str { "MyHandler" }
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut manager = HookManager::new();
    ///     manager.register("my_handler", Box::new(MyHandler)).await;
    /// }
    /// ```
    pub async fn register(&mut self, id: impl Into<String>, handler: Box<dyn ReloadHandler>) {
        let mut handlers = self.handlers.write().await;
        handlers.insert(id.into(), handler);
    }

    /// Unregister a reload handler
    ///
    /// # Arguments
    /// * `id` - Identifier used when registering the handler
    ///
    /// # Returns
    /// Returns `true` if the handler was found and removed, `false` otherwise
    pub async fn unregister(&mut self, id: &str) -> bool {
        let mut handlers = self.handlers.write().await;
        handlers.remove(id).is_some()
    }

    /// Get the number of registered handlers
    pub async fn count(&self) -> usize {
        let handlers = self.handlers.read().await;
        handlers.len()
    }

    /// Execute all registered reload handlers
    ///
    /// Calls `on_reload` for each registered handler. If any handler returns
    /// an error, execution continues for remaining handlers and all errors
    /// are collected.
    ///
    /// # Arguments
    /// * `old_config` - Configuration before reload
    /// * `new_config` - Configuration after reload
    ///
    /// # Returns
    /// Returns `Ok(())` if all handlers succeeded, or a list of errors if any failed.
    ///
    /// # Example
    /// ```no_run
    /// use impulse_config::{Config, hooks::HookManager};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let manager = HookManager::new();
    ///     let old_config = Config::load("old_config.toml")?;
    ///     let new_config = Config::load("new_config.toml")?;
    ///
    ///     if let Err(errors) = manager.execute_all(&old_config, &new_config).await {
    ///         eprintln!("Some handlers failed:");
    ///         for (name, error) in errors {
    ///             eprintln!("  {}: {}", name, error);
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn execute_all(
        &self,
        old_config: &Config,
        new_config: &Config,
    ) -> Result<(), Vec<(String, Box<dyn std::error::Error + Send + Sync>)>> {
        let mut handlers = self.handlers.write().await;
        let mut errors = Vec::new();

        for (id, handler) in handlers.iter_mut() {
            match handler.on_reload(old_config, new_config).await {
                Ok(()) => {
                    // Handler succeeded
                }
                Err(e) => {
                    errors.push((format!("{} ({})", handler.name(), id), e));
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Clear all registered handlers
    pub async fn clear(&mut self) {
        let mut handlers = self.handlers.write().await;
        handlers.clear();
    }
}

impl Default for HookManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test handler that always succeeds
    struct SuccessHandler {
        name: String,
        call_count: Arc<RwLock<usize>>,
    }

    #[async_trait]
    impl ReloadHandler for SuccessHandler {
        async fn on_reload(&mut self, _old: &Config, _new: &Config) -> HookResult {
            let mut count = self.call_count.write().await;
            *count += 1;
            Ok(())
        }

        fn name(&self) -> &str {
            &self.name
        }
    }

    // Test handler that always fails
    struct FailHandler {
        name: String,
    }

    #[async_trait]
    impl ReloadHandler for FailHandler {
        async fn on_reload(&mut self, _old: &Config, _new: &Config) -> HookResult {
            Err("Intentional test failure".into())
        }

        fn name(&self) -> &str {
            &self.name
        }
    }

    #[tokio::test]
    async fn test_new_manager() {
        let manager = HookManager::new();
        assert_eq!(manager.count().await, 0);
    }

    #[tokio::test]
    async fn test_register_handler() {
        let mut manager = HookManager::new();
        let handler = Box::new(SuccessHandler {
            name: "test".to_string(),
            call_count: Arc::new(RwLock::new(0)),
        });

        manager.register("handler1", handler).await;
        assert_eq!(manager.count().await, 1);
    }

    #[tokio::test]
    async fn test_unregister_handler() {
        let mut manager = HookManager::new();
        let handler = Box::new(SuccessHandler {
            name: "test".to_string(),
            call_count: Arc::new(RwLock::new(0)),
        });

        manager.register("handler1", handler).await;
        assert_eq!(manager.count().await, 1);

        let removed = manager.unregister("handler1").await;
        assert!(removed);
        assert_eq!(manager.count().await, 0);

        // Try removing again
        let removed = manager.unregister("handler1").await;
        assert!(!removed);
    }

    #[tokio::test]
    async fn test_execute_all_success() {
        let mut manager = HookManager::new();
        let call_count = Arc::new(RwLock::new(0));

        let handler = Box::new(SuccessHandler {
            name: "test".to_string(),
            call_count: Arc::clone(&call_count),
        });

        manager.register("handler1", handler).await;

        let old_config = Config::with_defaults();
        let new_config = Config::with_defaults();

        let result = manager.execute_all(&old_config, &new_config).await;
        assert!(result.is_ok());

        let count = *call_count.read().await;
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn test_execute_all_failure() {
        let mut manager = HookManager::new();
        let handler = Box::new(FailHandler {
            name: "failing_handler".to_string(),
        });

        manager.register("handler1", handler).await;

        let old_config = Config::with_defaults();
        let new_config = Config::with_defaults();

        let result = manager.execute_all(&old_config, &new_config).await;
        assert!(result.is_err());

        if let Err(errors) = result {
            assert_eq!(errors.len(), 1);
            assert!(errors[0].0.contains("failing_handler"));
        }
    }

    #[tokio::test]
    async fn test_execute_all_mixed() {
        let mut manager = HookManager::new();

        // Add success handler
        let success_handler = Box::new(SuccessHandler {
            name: "success".to_string(),
            call_count: Arc::new(RwLock::new(0)),
        });
        manager.register("handler1", success_handler).await;

        // Add fail handler
        let fail_handler = Box::new(FailHandler {
            name: "fail".to_string(),
        });
        manager.register("handler2", fail_handler).await;

        let old_config = Config::with_defaults();
        let new_config = Config::with_defaults();

        let result = manager.execute_all(&old_config, &new_config).await;
        assert!(result.is_err());

        // Should have exactly one error (from fail handler)
        if let Err(errors) = result {
            assert_eq!(errors.len(), 1);
        }
    }

    #[tokio::test]
    async fn test_clear() {
        let mut manager = HookManager::new();

        for i in 0..3 {
            let handler = Box::new(SuccessHandler {
                name: format!("handler{}", i),
                call_count: Arc::new(RwLock::new(0)),
            });
            manager.register(format!("handler{}", i), handler).await;
        }

        assert_eq!(manager.count().await, 3);

        manager.clear().await;
        assert_eq!(manager.count().await, 0);
    }

    #[tokio::test]
    async fn test_multiple_handlers() {
        let mut manager = HookManager::new();
        let mut call_counts = Vec::new();

        // Register multiple handlers
        for i in 0..5 {
            let call_count = Arc::new(RwLock::new(0));
            call_counts.push(Arc::clone(&call_count));

            let handler = Box::new(SuccessHandler {
                name: format!("handler{}", i),
                call_count,
            });
            manager.register(format!("handler{}", i), handler).await;
        }

        let old_config = Config::with_defaults();
        let new_config = Config::with_defaults();

        let result = manager.execute_all(&old_config, &new_config).await;
        assert!(result.is_ok());

        // All handlers should have been called once
        for call_count in call_counts {
            let count = *call_count.read().await;
            assert_eq!(count, 1);
        }
    }

    #[test]
    fn test_default() {
        let _manager = HookManager::default();
    }
}
