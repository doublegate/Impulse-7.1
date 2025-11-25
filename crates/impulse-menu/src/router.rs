//! Command routing and handler system

use crate::error::CommandError;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

/// Result of executing a command
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandResult {
    /// Continue in current menu
    Continue,
    /// Navigate to a different menu
    ChangeMenu(String),
    /// Go back to previous menu
    Back,
    /// Return to main menu
    MainMenu,
    /// Disconnect user
    Disconnect,
    /// Display a message and continue
    Message(String),
}

/// Context passed to command handlers
#[derive(Debug, Clone)]
pub struct CommandContext {
    /// Optional user ID
    pub user_id: Option<String>,
    /// User's security level
    pub user_security: u8,
    /// Current menu name
    pub current_menu: String,
    /// Navigation stack (menu history)
    pub menu_stack: Vec<String>,
}

impl CommandContext {
    /// Create a new command context
    pub fn new(user_security: u8, current_menu: String) -> Self {
        Self {
            user_id: None,
            user_security,
            current_menu,
            menu_stack: Vec::new(),
        }
    }

    /// Create a context with user ID
    pub fn with_user(user_id: String, user_security: u8, current_menu: String) -> Self {
        Self {
            user_id: Some(user_id),
            user_security,
            current_menu,
            menu_stack: Vec::new(),
        }
    }
}

/// Trait for command handlers
#[async_trait]
pub trait CommandHandler: Send + Sync {
    /// Execute the command
    async fn execute(&self, ctx: &mut CommandContext) -> Result<CommandResult, CommandError>;

    /// Get command name
    fn name(&self) -> &str;

    /// Get command description
    fn description(&self) -> &str;

    /// Get minimum security level required
    fn min_security(&self) -> u8 {
        0
    }
}

/// Command registry and router
pub struct CommandRouter {
    handlers: HashMap<String, Arc<dyn CommandHandler>>,
}

impl CommandRouter {
    /// Create a new command router
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    /// Register a command handler
    pub fn register(&mut self, handler: Arc<dyn CommandHandler>) {
        self.handlers.insert(handler.name().to_string(), handler);
    }

    /// Register a command handler with ownership transfer
    pub fn register_boxed(&mut self, handler: Box<dyn CommandHandler>) {
        let name = handler.name().to_string();
        self.handlers.insert(name, Arc::from(handler));
    }

    /// Register a simple function-based command handler
    pub fn register_fn<F>(&mut self, name: &str, desc: &str, min_security: u8, f: F)
    where
        F: Fn(&mut CommandContext) -> CommandResult + Send + Sync + 'static,
    {
        let handler = FunctionCommandHandler {
            name: name.to_string(),
            description: desc.to_string(),
            min_security,
            func: Arc::new(f),
        };
        self.register(Arc::new(handler));
    }

    /// Route a command to its handler
    pub async fn route(
        &self,
        command: &str,
        ctx: &mut CommandContext,
    ) -> Result<CommandResult, CommandError> {
        let command_lower = command.to_lowercase();

        match self.handlers.get(&command_lower) {
            Some(handler) => {
                // Check security level
                if ctx.user_security < handler.min_security() {
                    return Err(CommandError::InsufficientPrivileges {
                        command: command.to_string(),
                    });
                }

                handler.execute(ctx).await
            }
            None => Err(CommandError::UnknownCommand {
                command: command.to_string(),
            }),
        }
    }

    /// Check if a handler exists for a command
    pub fn has_handler(&self, command: &str) -> bool {
        self.handlers.contains_key(&command.to_lowercase())
    }

    /// Get list of registered command names
    pub fn commands(&self) -> Vec<String> {
        self.handlers.keys().cloned().collect()
    }

    /// Get handler by name
    pub fn get_handler(&self, command: &str) -> Option<Arc<dyn CommandHandler>> {
        self.handlers.get(&command.to_lowercase()).cloned()
    }
}

impl Default for CommandRouter {
    fn default() -> Self {
        Self::new()
    }
}

/// Function-based command handler
struct FunctionCommandHandler<F>
where
    F: Fn(&mut CommandContext) -> CommandResult + Send + Sync,
{
    name: String,
    description: String,
    min_security: u8,
    func: Arc<F>,
}

#[async_trait]
impl<F> CommandHandler for FunctionCommandHandler<F>
where
    F: Fn(&mut CommandContext) -> CommandResult + Send + Sync,
{
    async fn execute(&self, ctx: &mut CommandContext) -> Result<CommandResult, CommandError> {
        Ok((self.func)(ctx))
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn min_security(&self) -> u8 {
        self.min_security
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestHandler {
        name: String,
        result: CommandResult,
    }

    #[async_trait]
    impl CommandHandler for TestHandler {
        async fn execute(&self, _ctx: &mut CommandContext) -> Result<CommandResult, CommandError> {
            Ok(self.result.clone())
        }

        fn name(&self) -> &str {
            &self.name
        }

        fn description(&self) -> &str {
            "Test handler"
        }
    }

    #[tokio::test]
    async fn test_register_and_route() {
        let mut router = CommandRouter::new();

        let handler = TestHandler {
            name: "test".to_string(),
            result: CommandResult::Continue,
        };

        router.register(Arc::new(handler));

        let mut ctx = CommandContext::new(0, "main".to_string());
        let result = router.route("test", &mut ctx).await.unwrap();

        assert_eq!(result, CommandResult::Continue);
    }

    #[tokio::test]
    async fn test_route_unknown_command() {
        let router = CommandRouter::new();
        let mut ctx = CommandContext::new(0, "main".to_string());

        let result = router.route("nonexistent", &mut ctx).await;
        assert!(matches!(result, Err(CommandError::UnknownCommand { .. })));
    }

    #[tokio::test]
    async fn test_route_case_insensitive() {
        let mut router = CommandRouter::new();

        let handler = TestHandler {
            name: "files".to_string(),
            result: CommandResult::ChangeMenu("files".to_string()),
        };

        router.register(Arc::new(handler));

        let mut ctx = CommandContext::new(0, "main".to_string());

        // Test different cases
        let result = router.route("FILES", &mut ctx).await.unwrap();
        assert_eq!(result, CommandResult::ChangeMenu("files".to_string()));

        let result = router.route("Files", &mut ctx).await.unwrap();
        assert_eq!(result, CommandResult::ChangeMenu("files".to_string()));

        let result = router.route("files", &mut ctx).await.unwrap();
        assert_eq!(result, CommandResult::ChangeMenu("files".to_string()));
    }

    #[tokio::test]
    async fn test_register_fn() {
        let mut router = CommandRouter::new();

        router.register_fn("hello", "Say hello", 0, |_ctx| {
            CommandResult::Message("Hello!".to_string())
        });

        let mut ctx = CommandContext::new(0, "main".to_string());
        let result = router.route("hello", &mut ctx).await.unwrap();

        assert_eq!(result, CommandResult::Message("Hello!".to_string()));
    }

    #[tokio::test]
    async fn test_has_handler() {
        let mut router = CommandRouter::new();

        let handler = TestHandler {
            name: "test".to_string(),
            result: CommandResult::Continue,
        };

        router.register(Arc::new(handler));

        assert!(router.has_handler("test"));
        assert!(router.has_handler("TEST"));
        assert!(!router.has_handler("nonexistent"));
    }

    #[tokio::test]
    async fn test_commands_list() {
        let mut router = CommandRouter::new();

        router.register(Arc::new(TestHandler {
            name: "cmd1".to_string(),
            result: CommandResult::Continue,
        }));

        router.register(Arc::new(TestHandler {
            name: "cmd2".to_string(),
            result: CommandResult::Continue,
        }));

        let commands = router.commands();
        assert_eq!(commands.len(), 2);
        assert!(commands.contains(&"cmd1".to_string()));
        assert!(commands.contains(&"cmd2".to_string()));
    }

    #[tokio::test]
    async fn test_command_context() {
        let ctx = CommandContext::new(10, "main".to_string());
        assert_eq!(ctx.user_security, 10);
        assert_eq!(ctx.current_menu, "main");
        assert!(ctx.user_id.is_none());

        let ctx = CommandContext::with_user("user123".to_string(), 20, "files".to_string());
        assert_eq!(ctx.user_id, Some("user123".to_string()));
        assert_eq!(ctx.user_security, 20);
        assert_eq!(ctx.current_menu, "files");
    }

    #[tokio::test]
    async fn test_command_result_types() {
        let results = vec![
            CommandResult::Continue,
            CommandResult::ChangeMenu("test".to_string()),
            CommandResult::Back,
            CommandResult::MainMenu,
            CommandResult::Disconnect,
            CommandResult::Message("msg".to_string()),
        ];

        for result in results {
            // Just ensure they can be created and cloned
            let _cloned = result.clone();
        }
    }

    struct SecureHandler;

    #[async_trait]
    impl CommandHandler for SecureHandler {
        async fn execute(&self, _ctx: &mut CommandContext) -> Result<CommandResult, CommandError> {
            Ok(CommandResult::Continue)
        }

        fn name(&self) -> &str {
            "secure"
        }

        fn description(&self) -> &str {
            "Secure command"
        }

        fn min_security(&self) -> u8 {
            100
        }
    }

    #[tokio::test]
    async fn test_security_check() {
        let mut router = CommandRouter::new();
        router.register(Arc::new(SecureHandler));

        // User with insufficient privileges
        let mut ctx = CommandContext::new(50, "main".to_string());
        let result = router.route("secure", &mut ctx).await;
        assert!(matches!(
            result,
            Err(CommandError::InsufficientPrivileges { .. })
        ));

        // User with sufficient privileges
        let mut ctx = CommandContext::new(150, "main".to_string());
        let result = router.route("secure", &mut ctx).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_handler() {
        let mut router = CommandRouter::new();

        router.register(Arc::new(TestHandler {
            name: "test".to_string(),
            result: CommandResult::Continue,
        }));

        let handler = router.get_handler("test");
        assert!(handler.is_some());
        assert_eq!(handler.unwrap().name(), "test");

        let handler = router.get_handler("nonexistent");
        assert!(handler.is_none());
    }

    #[tokio::test]
    async fn test_multiple_handlers() {
        let mut router = CommandRouter::new();

        router.register_fn("cmd1", "Command 1", 0, |_| CommandResult::Continue);
        router.register_fn("cmd2", "Command 2", 10, |_| CommandResult::Back);
        router.register_fn("cmd3", "Command 3", 20, |_| CommandResult::MainMenu);

        let mut ctx = CommandContext::new(50, "test".to_string());

        let r1 = router.route("cmd1", &mut ctx).await.unwrap();
        assert_eq!(r1, CommandResult::Continue);

        let r2 = router.route("cmd2", &mut ctx).await.unwrap();
        assert_eq!(r2, CommandResult::Back);

        let r3 = router.route("cmd3", &mut ctx).await.unwrap();
        assert_eq!(r3, CommandResult::MainMenu);
    }
}
