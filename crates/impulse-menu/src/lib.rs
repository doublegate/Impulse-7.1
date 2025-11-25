//! Menu system and navigation for Impulse BBS
//!
//! This crate provides a complete menu system with:
//! - TOML-based menu definitions
//! - Hot-key and full-menu interaction modes
//! - Security level filtering
//! - Command routing and handlers
//! - Navigation state machine
//! - Built-in navigation commands
//!
//! # Examples
//!
//! ## Parsing a menu
//!
//! ```
//! use impulse_menu::parser::MenuParser;
//!
//! let toml = r#"
//! [menu]
//! name = "main"
//! title = "Main Menu"
//! mode = "hotkey"
//!
//! [[option]]
//! key = "F"
//! command = "files"
//! description = "File Areas"
//! min_security = 0
//! "#;
//!
//! let menu = MenuParser::parse(toml).expect("Failed to parse menu");
//! assert_eq!(menu.menu.name, "main");
//! ```
//!
//! ## Rendering a menu
//!
//! ```
//! use impulse_menu::parser::{MenuParser, MenuDefinition, MenuMetadata, MenuMode, MenuOption};
//! use impulse_menu::renderer::MenuRenderer;
//!
//! let menu = MenuDefinition {
//!     menu: MenuMetadata {
//!         name: "test".to_string(),
//!         title: "Test Menu".to_string(),
//!         ansi_art: None,
//!         mode: MenuMode::Hotkey,
//!         inherits: None,
//!     },
//!     option: vec![
//!         MenuOption {
//!             key: "Q".to_string(),
//!             command: "quit".to_string(),
//!             description: "Quit".to_string(),
//!             min_security: 0,
//!             max_security: None,
//!         },
//!     ],
//! };
//!
//! let renderer = MenuRenderer::new();
//! let rendered = renderer.render(&menu, 10);
//!
//! assert_eq!(rendered.title, "Test Menu");
//! assert!(rendered.valid_keys.contains(&"Q".to_string()));
//! ```
//!
//! ## Command routing
//!
//! ```no_run
//! use impulse_menu::router::{CommandRouter, CommandContext};
//! use impulse_menu::commands::register_builtin_commands;
//!
//! # async fn example() {
//! let mut router = CommandRouter::new();
//! register_builtin_commands(&mut router);
//!
//! let mut ctx = CommandContext::new(10, "main".to_string());
//! let result = router.route("help", &mut ctx).await.unwrap();
//! # }
//! ```
//!
//! ## Navigation state
//!
//! ```
//! use impulse_menu::state::MenuState;
//! use impulse_menu::parser::{MenuDefinition, MenuMetadata, MenuMode};
//!
//! let mut state = MenuState::new("main");
//!
//! // Add menus
//! let main_menu = MenuDefinition {
//!     menu: MenuMetadata {
//!         name: "main".to_string(),
//!         title: "Main Menu".to_string(),
//!         ansi_art: None,
//!         mode: MenuMode::Hotkey,
//!         inherits: None,
//!     },
//!     option: vec![],
//! };
//! state.add_menu(main_menu);
//!
//! let files_menu = MenuDefinition {
//!     menu: MenuMetadata {
//!         name: "files".to_string(),
//!         title: "File Areas".to_string(),
//!         ansi_art: None,
//!         mode: MenuMode::Hotkey,
//!         inherits: None,
//!     },
//!     option: vec![],
//! };
//! state.add_menu(files_menu);
//!
//! // Navigate
//! state.navigate_to("files").unwrap();
//! assert_eq!(state.current_name(), "files");
//!
//! // Go back
//! state.go_back().unwrap();
//! assert_eq!(state.current_name(), "main");
//! ```

pub mod commands;
pub mod error;
pub mod parser;
pub mod renderer;
pub mod router;
pub mod state;

// Re-export commonly used types
pub use error::{CommandError, MenuLoadError, MenuParseError, NavigationError, ValidationError};
pub use parser::{MenuDefinition, MenuMetadata, MenuMode, MenuOption, MenuParser};
pub use renderer::{MenuRenderer, RenderedMenu};
pub use router::{CommandContext, CommandHandler, CommandResult, CommandRouter};
pub use state::MenuState;

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_complete_menu_flow() {
        // Create menu definitions
        let main_toml = r#"
[menu]
name = "main"
title = "Main Menu"
mode = "hotkey"

[[option]]
key = "F"
command = "files"
description = "File Areas"
min_security = 0

[[option]]
key = "M"
command = "messages"
description = "Message Areas"
min_security = 10

[[option]]
key = "G"
command = "goodbye"
description = "Logoff"
min_security = 0
"#;

        let files_toml = r#"
[menu]
name = "files"
title = "File Areas"
mode = "fullmenu"

[[option]]
key = "L"
command = "list"
description = "List Files"
min_security = 0

[[option]]
key = "Q"
command = "back"
description = "Back to Main"
min_security = 0
"#;

        // Parse menus
        let main_menu = MenuParser::parse(main_toml).unwrap();
        let files_menu = MenuParser::parse(files_toml).unwrap();

        // Initialize state
        let mut state = MenuState::new("main");
        state.add_menu(main_menu);
        state.add_menu(files_menu);

        // Initialize router
        let mut router = CommandRouter::new();
        commands::register_builtin_commands(&mut router);

        // Add custom file command
        router.register_fn("files", "Navigate to files", 0, |_ctx| {
            CommandResult::ChangeMenu("files".to_string())
        });

        // Initialize renderer
        let renderer = MenuRenderer::new();

        // Render main menu for user with security level 15
        let rendered = renderer.render(state.current().unwrap(), 15);
        assert_eq!(rendered.title, "Main Menu");
        assert_eq!(rendered.valid_keys.len(), 3); // F, M, G

        // User selects "files" command
        let mut ctx = CommandContext::new(15, state.current_name().to_string());
        ctx.menu_stack = state.breadcrumbs().iter().map(|s| s.to_string()).collect();

        let result = router.route("files", &mut ctx).await.unwrap();

        // Handle navigation
        if let CommandResult::ChangeMenu(menu_name) = result {
            state.navigate_to(&menu_name).unwrap();
        }

        assert_eq!(state.current_name(), "files");
        assert_eq!(state.stack_depth(), 1);

        // Render files menu
        let rendered = renderer.render(state.current().unwrap(), 15);
        assert_eq!(rendered.title, "File Areas");
        assert_eq!(rendered.prompt, "Enter command: "); // fullmenu mode

        // User goes back
        let mut ctx = CommandContext::new(15, state.current_name().to_string());
        let result = router.route("back", &mut ctx).await.unwrap();

        if result == CommandResult::Back {
            state.go_back().unwrap();
        }

        assert_eq!(state.current_name(), "main");
        assert_eq!(state.stack_depth(), 0);
    }

    #[test]
    fn test_menu_validation_and_rendering() {
        let menu_toml = r#"
[menu]
name = "test"
title = "Test Menu"
mode = "hotkey"

[[option]]
key = "A"
command = "admin"
description = "Admin Panel"
min_security = 100
max_security = 255

[[option]]
key = "U"
command = "user"
description = "User Settings"
min_security = 10

[[option]]
key = "Q"
command = "quit"
description = "Quit"
min_security = 0
"#;

        let menu = MenuParser::parse(menu_toml).unwrap();

        // Validate menu
        MenuParser::validate(&menu).expect("Menu should be valid");

        let renderer = MenuRenderer::new();

        // Low security user
        let rendered = renderer.render(&menu, 5);
        assert_eq!(rendered.valid_keys.len(), 1); // Only Q

        // Medium security user
        let rendered = renderer.render(&menu, 50);
        assert_eq!(rendered.valid_keys.len(), 2); // U and Q

        // High security user
        let rendered = renderer.render(&menu, 150);
        assert_eq!(rendered.valid_keys.len(), 3); // A, U, and Q
    }

    #[tokio::test]
    async fn test_command_security() {
        let mut router = CommandRouter::new();

        // Register a secure command
        router.register_fn("admin", "Admin panel", 100, |_ctx| {
            CommandResult::Message("Welcome to admin panel".to_string())
        });

        // Low privilege user
        let mut ctx = CommandContext::new(50, "main".to_string());
        let result = router.route("admin", &mut ctx).await;
        assert!(matches!(
            result,
            Err(CommandError::InsufficientPrivileges { .. })
        ));

        // High privilege user
        let mut ctx = CommandContext::new(150, "main".to_string());
        let result = router.route("admin", &mut ctx).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_breadcrumbs_tracking() {
        let mut state = MenuState::new("main");

        // Create menu hierarchy
        let menus = vec![
            ("main", "Main Menu"),
            ("files", "File Areas"),
            ("upload", "Upload Files"),
        ];

        for (name, title) in menus {
            state.add_menu(MenuDefinition {
                menu: MenuMetadata {
                    name: name.to_string(),
                    title: title.to_string(),
                    ansi_art: None,
                    mode: MenuMode::Hotkey,
                    inherits: None,
                },
                option: vec![],
            });
        }

        // Navigate through hierarchy
        state.navigate_to("files").unwrap();
        state.navigate_to("upload").unwrap();

        let breadcrumbs = state.breadcrumbs();
        assert_eq!(breadcrumbs, vec!["main", "files", "upload"]);

        // Go back
        state.go_back().unwrap();
        let breadcrumbs = state.breadcrumbs();
        assert_eq!(breadcrumbs, vec!["main", "files"]);

        // Go to main
        state.go_main();
        let breadcrumbs = state.breadcrumbs();
        assert_eq!(breadcrumbs, vec!["main"]);
    }
}
