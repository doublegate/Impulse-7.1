//! Built-in navigation command handlers

use crate::error::CommandError;
use crate::router::{CommandContext, CommandHandler, CommandResult, CommandRouter};
use async_trait::async_trait;

/// Navigate back to previous menu
pub struct BackCommand;

#[async_trait]
impl CommandHandler for BackCommand {
    async fn execute(&self, _ctx: &mut CommandContext) -> Result<CommandResult, CommandError> {
        Ok(CommandResult::Back)
    }

    fn name(&self) -> &str {
        "back"
    }

    fn description(&self) -> &str {
        "Return to previous menu"
    }
}

/// Navigate to main menu
pub struct MainMenuCommand;

#[async_trait]
impl CommandHandler for MainMenuCommand {
    async fn execute(&self, _ctx: &mut CommandContext) -> Result<CommandResult, CommandError> {
        Ok(CommandResult::MainMenu)
    }

    fn name(&self) -> &str {
        "main"
    }

    fn description(&self) -> &str {
        "Return to main menu"
    }
}

/// Disconnect from the BBS
pub struct GoodbyeCommand;

#[async_trait]
impl CommandHandler for GoodbyeCommand {
    async fn execute(&self, _ctx: &mut CommandContext) -> Result<CommandResult, CommandError> {
        Ok(CommandResult::Disconnect)
    }

    fn name(&self) -> &str {
        "goodbye"
    }

    fn description(&self) -> &str {
        "Logoff and disconnect"
    }
}

/// Display help information
pub struct HelpCommand;

#[async_trait]
impl CommandHandler for HelpCommand {
    async fn execute(&self, ctx: &mut CommandContext) -> Result<CommandResult, CommandError> {
        let help_text = format!(
            "You are in menu: {}\n\
             Current navigation depth: {}\n\
             \n\
             Common commands:\n\
             - BACK: Return to previous menu\n\
             - MAIN: Return to main menu\n\
             - GOODBYE: Logoff\n\
             - HELP: Display this help",
            ctx.current_menu,
            ctx.menu_stack.len()
        );

        Ok(CommandResult::Message(help_text))
    }

    fn name(&self) -> &str {
        "help"
    }

    fn description(&self) -> &str {
        "Display help information"
    }
}

/// Display current menu breadcrumbs
pub struct WhereCommand;

#[async_trait]
impl CommandHandler for WhereCommand {
    async fn execute(&self, ctx: &mut CommandContext) -> Result<CommandResult, CommandError> {
        let breadcrumbs = if ctx.menu_stack.is_empty() {
            ctx.current_menu.clone()
        } else {
            let mut path = ctx.menu_stack.join(" > ");
            path.push_str(" > ");
            path.push_str(&ctx.current_menu);
            path
        };

        Ok(CommandResult::Message(format!(
            "Current location: {}",
            breadcrumbs
        )))
    }

    fn name(&self) -> &str {
        "where"
    }

    fn description(&self) -> &str {
        "Show current location"
    }
}

/// Register all built-in commands with a command router
pub fn register_builtin_commands(router: &mut CommandRouter) {
    router.register(std::sync::Arc::new(BackCommand));
    router.register(std::sync::Arc::new(MainMenuCommand));
    router.register(std::sync::Arc::new(GoodbyeCommand));
    router.register(std::sync::Arc::new(HelpCommand));
    router.register(std::sync::Arc::new(WhereCommand));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_back_command() {
        let cmd = BackCommand;
        let mut ctx = CommandContext::new(0, "test".to_string());

        let result = cmd.execute(&mut ctx).await.unwrap();
        assert_eq!(result, CommandResult::Back);
        assert_eq!(cmd.name(), "back");
    }

    #[tokio::test]
    async fn test_main_menu_command() {
        let cmd = MainMenuCommand;
        let mut ctx = CommandContext::new(0, "test".to_string());

        let result = cmd.execute(&mut ctx).await.unwrap();
        assert_eq!(result, CommandResult::MainMenu);
        assert_eq!(cmd.name(), "main");
    }

    #[tokio::test]
    async fn test_goodbye_command() {
        let cmd = GoodbyeCommand;
        let mut ctx = CommandContext::new(0, "test".to_string());

        let result = cmd.execute(&mut ctx).await.unwrap();
        assert_eq!(result, CommandResult::Disconnect);
        assert_eq!(cmd.name(), "goodbye");
    }

    #[tokio::test]
    async fn test_help_command() {
        let cmd = HelpCommand;
        let mut ctx = CommandContext::new(0, "test_menu".to_string());

        let result = cmd.execute(&mut ctx).await.unwrap();

        if let CommandResult::Message(msg) = result {
            assert!(msg.contains("test_menu"));
            assert!(msg.contains("BACK"));
            assert!(msg.contains("MAIN"));
            assert!(msg.contains("GOODBYE"));
        } else {
            panic!("Expected Message result");
        }
    }

    #[tokio::test]
    async fn test_where_command_single_menu() {
        let cmd = WhereCommand;
        let mut ctx = CommandContext::new(0, "main".to_string());

        let result = cmd.execute(&mut ctx).await.unwrap();

        if let CommandResult::Message(msg) = result {
            assert!(msg.contains("main"));
            assert!(!msg.contains(">"));
        } else {
            panic!("Expected Message result");
        }
    }

    #[tokio::test]
    async fn test_where_command_with_stack() {
        let cmd = WhereCommand;
        let mut ctx = CommandContext::new(0, "files".to_string());
        ctx.menu_stack.push("main".to_string());

        let result = cmd.execute(&mut ctx).await.unwrap();

        if let CommandResult::Message(msg) = result {
            assert!(msg.contains("main > files"));
        } else {
            panic!("Expected Message result");
        }
    }

    #[tokio::test]
    async fn test_where_command_deep_stack() {
        let cmd = WhereCommand;
        let mut ctx = CommandContext::new(0, "upload".to_string());
        ctx.menu_stack.push("main".to_string());
        ctx.menu_stack.push("files".to_string());

        let result = cmd.execute(&mut ctx).await.unwrap();

        if let CommandResult::Message(msg) = result {
            assert!(msg.contains("main > files > upload"));
        } else {
            panic!("Expected Message result");
        }
    }

    #[tokio::test]
    async fn test_register_builtin_commands() {
        let mut router = CommandRouter::new();
        register_builtin_commands(&mut router);

        assert!(router.has_handler("back"));
        assert!(router.has_handler("main"));
        assert!(router.has_handler("goodbye"));
        assert!(router.has_handler("help"));
        assert!(router.has_handler("where"));
    }

    #[tokio::test]
    async fn test_builtin_commands_via_router() {
        let mut router = CommandRouter::new();
        register_builtin_commands(&mut router);

        let mut ctx = CommandContext::new(0, "test".to_string());

        // Test back
        let result = router.route("back", &mut ctx).await.unwrap();
        assert_eq!(result, CommandResult::Back);

        // Test main
        let result = router.route("main", &mut ctx).await.unwrap();
        assert_eq!(result, CommandResult::MainMenu);

        // Test goodbye
        let result = router.route("goodbye", &mut ctx).await.unwrap();
        assert_eq!(result, CommandResult::Disconnect);

        // Test help
        let result = router.route("help", &mut ctx).await.unwrap();
        assert!(matches!(result, CommandResult::Message(_)));

        // Test where
        let result = router.route("where", &mut ctx).await.unwrap();
        assert!(matches!(result, CommandResult::Message(_)));
    }

    #[tokio::test]
    async fn test_command_descriptions() {
        let back = BackCommand;
        let main = MainMenuCommand;
        let goodbye = GoodbyeCommand;
        let help = HelpCommand;
        let where_cmd = WhereCommand;

        assert!(!back.description().is_empty());
        assert!(!main.description().is_empty());
        assert!(!goodbye.description().is_empty());
        assert!(!help.description().is_empty());
        assert!(!where_cmd.description().is_empty());
    }

    #[tokio::test]
    async fn test_command_names() {
        let commands = vec![
            BackCommand.name(),
            MainMenuCommand.name(),
            GoodbyeCommand.name(),
            HelpCommand.name(),
            WhereCommand.name(),
        ];

        // All names should be lowercase
        for name in &commands {
            assert_eq!(name, &name.to_lowercase());
        }

        // All names should be unique
        let unique_count = commands
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len();
        assert_eq!(unique_count, commands.len());
    }
}
