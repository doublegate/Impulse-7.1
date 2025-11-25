//! Integration tests for the menu system

use impulse_menu::*;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

/// Helper to create a test menu file
fn create_menu_file(dir: &Path, name: &str, content: &str) {
    let path = dir.join(format!("{}.toml", name));
    fs::write(path, content).expect("Failed to write menu file");
}

#[test]
fn test_load_multiple_menus() {
    let temp_dir = TempDir::new().unwrap();

    // Create main menu
    create_menu_file(
        temp_dir.path(),
        "main",
        r#"
[menu]
name = "main"
title = "Main Menu"
mode = "hotkey"

[[option]]
key = "F"
command = "files"
description = "File Areas"
min_security = 0
"#,
    );

    // Create files menu
    create_menu_file(
        temp_dir.path(),
        "files",
        r#"
[menu]
name = "files"
title = "File Areas"
mode = "fullmenu"

[[option]]
key = "L"
command = "list"
description = "List Files"
min_security = 0
"#,
    );

    // Load menus
    let mut state = MenuState::new("main");
    state
        .load_menus(temp_dir.path())
        .expect("Failed to load menus");

    assert!(state.has_menu("main"));
    assert!(state.has_menu("files"));
    assert_eq!(state.menu_names().len(), 2);
}

#[tokio::test]
async fn test_complete_navigation_flow() {
    let temp_dir = TempDir::new().unwrap();

    // Create a hierarchy of menus
    create_menu_file(
        temp_dir.path(),
        "main",
        r#"
[menu]
name = "main"
title = "Main Menu"

[[option]]
key = "F"
command = "files"
description = "File Areas"

[[option]]
key = "M"
command = "messages"
description = "Messages"

[[option]]
key = "G"
command = "goodbye"
description = "Logoff"
"#,
    );

    create_menu_file(
        temp_dir.path(),
        "files",
        r#"
[menu]
name = "files"
title = "File Areas"

[[option]]
key = "U"
command = "upload"
description = "Upload"

[[option]]
key = "D"
command = "download"
description = "Download"

[[option]]
key = "Q"
command = "back"
description = "Back"
"#,
    );

    create_menu_file(
        temp_dir.path(),
        "messages",
        r#"
[menu]
name = "messages"
title = "Message Areas"

[[option]]
key = "R"
command = "read"
description = "Read"

[[option]]
key = "P"
command = "post"
description = "Post"

[[option]]
key = "Q"
command = "back"
description = "Back"
"#,
    );

    // Load menus
    let mut state = MenuState::new("main");
    state.load_menus(temp_dir.path()).unwrap();

    // Setup router
    let mut router = CommandRouter::new();
    commands::register_builtin_commands(&mut router);

    // Add navigation commands
    router.register_fn("files", "Go to files", 0, |_| {
        CommandResult::ChangeMenu("files".to_string())
    });
    router.register_fn("messages", "Go to messages", 0, |_| {
        CommandResult::ChangeMenu("messages".to_string())
    });

    // Test navigation: main -> files -> back -> messages -> main
    let mut ctx = CommandContext::new(10, state.current_name().to_string());

    // Navigate to files
    let result = router.route("files", &mut ctx).await.unwrap();
    if let CommandResult::ChangeMenu(menu) = result {
        state.navigate_to(&menu).unwrap();
    }
    assert_eq!(state.current_name(), "files");

    // Go back
    ctx.current_menu = state.current_name().to_string();
    let result = router.route("back", &mut ctx).await.unwrap();
    if result == CommandResult::Back {
        state.go_back().unwrap();
    }
    assert_eq!(state.current_name(), "main");

    // Navigate to messages
    ctx.current_menu = state.current_name().to_string();
    let result = router.route("messages", &mut ctx).await.unwrap();
    if let CommandResult::ChangeMenu(menu) = result {
        state.navigate_to(&menu).unwrap();
    }
    assert_eq!(state.current_name(), "messages");

    // Go to main
    let result = router.route("main", &mut ctx).await.unwrap();
    if result == CommandResult::MainMenu {
        state.go_main();
    }
    assert_eq!(state.current_name(), "main");
}

#[test]
fn test_menu_rendering_with_different_security_levels() {
    let menu_toml = r#"
[menu]
name = "test"
title = "Security Test Menu"

[[option]]
key = "P"
command = "public"
description = "Public Access"
min_security = 0

[[option]]
key = "U"
command = "user"
description = "User Access"
min_security = 10

[[option]]
key = "M"
command = "moderator"
description = "Moderator Access"
min_security = 50

[[option]]
key = "A"
command = "admin"
description = "Admin Access"
min_security = 100
max_security = 255
"#;

    let menu = MenuParser::parse(menu_toml).unwrap();
    let renderer = MenuRenderer::new();

    // Test different security levels
    let test_cases = vec![
        (0, 1),   // Guest: only public
        (5, 1),   // Low: only public
        (10, 2),  // User: public + user
        (50, 3),  // Moderator: public + user + moderator
        (100, 4), // Admin: all options
        (200, 4), // Super admin: all options
    ];

    for (security, expected_count) in test_cases {
        let rendered = renderer.render(&menu, security);
        assert_eq!(
            rendered.valid_keys.len(),
            expected_count,
            "Security level {} should see {} options",
            security,
            expected_count
        );
    }
}

#[test]
fn test_hotkey_vs_fullmenu_rendering() {
    let hotkey_menu = MenuParser::parse(
        r#"
[menu]
name = "hotkey"
title = "Hotkey Menu"
mode = "hotkey"

[[option]]
key = "F"
command = "files"
description = "Files"
"#,
    )
    .unwrap();

    let fullmenu_menu = MenuParser::parse(
        r#"
[menu]
name = "fullmenu"
title = "Fullmenu Menu"
mode = "fullmenu"

[[option]]
key = "F"
command = "files"
description = "Files"
"#,
    )
    .unwrap();

    let renderer = MenuRenderer::new();

    let hotkey_rendered = renderer.render(&hotkey_menu, 255);
    assert_eq!(hotkey_rendered.prompt, "Command: ");
    assert!(hotkey_rendered.content.contains("(F)"));

    let fullmenu_rendered = renderer.render(&fullmenu_menu, 255);
    assert_eq!(fullmenu_rendered.prompt, "Enter command: ");
    assert!(fullmenu_rendered.content.contains("files"));
}

#[tokio::test]
async fn test_command_handler_security() {
    let mut router = CommandRouter::new();

    // Create handlers with different security requirements
    router.register_fn("public", "Public command", 0, |_| CommandResult::Continue);
    router.register_fn("user", "User command", 10, |_| CommandResult::Continue);
    router.register_fn("admin", "Admin command", 100, |_| CommandResult::Continue);

    // Test with low security
    let mut ctx = CommandContext::new(5, "test".to_string());
    assert!(router.route("public", &mut ctx).await.is_ok());
    assert!(router.route("user", &mut ctx).await.is_err());
    assert!(router.route("admin", &mut ctx).await.is_err());

    // Test with medium security
    let mut ctx = CommandContext::new(50, "test".to_string());
    assert!(router.route("public", &mut ctx).await.is_ok());
    assert!(router.route("user", &mut ctx).await.is_ok());
    assert!(router.route("admin", &mut ctx).await.is_err());

    // Test with high security
    let mut ctx = CommandContext::new(150, "test".to_string());
    assert!(router.route("public", &mut ctx).await.is_ok());
    assert!(router.route("user", &mut ctx).await.is_ok());
    assert!(router.route("admin", &mut ctx).await.is_ok());
}

#[test]
fn test_menu_validation_errors() {
    // Test duplicate keys
    let duplicate_keys = MenuParser::parse(
        r#"
[menu]
name = "test"
title = "Test"

[[option]]
key = "F"
command = "files"
description = "Files"

[[option]]
key = "F"
command = "files2"
description = "Files 2"
"#,
    )
    .unwrap();

    let errors = MenuParser::validate(&duplicate_keys).unwrap_err();
    assert!(!errors.is_empty());

    // Test invalid security range
    let invalid_security = MenuParser::parse(
        r#"
[menu]
name = "test"
title = "Test"

[[option]]
key = "F"
command = "files"
description = "Files"
min_security = 100
max_security = 50
"#,
    )
    .unwrap();

    let errors = MenuParser::validate(&invalid_security).unwrap_err();
    assert!(!errors.is_empty());
}

#[test]
fn test_breadcrumb_generation() {
    let mut state = MenuState::new("main");

    // Create menu hierarchy
    for (name, title) in &[
        ("main", "Main"),
        ("files", "Files"),
        ("upload", "Upload"),
        ("preview", "Preview"),
    ] {
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

    // Navigate: main -> files -> upload -> preview
    state.navigate_to("files").unwrap();
    state.navigate_to("upload").unwrap();
    state.navigate_to("preview").unwrap();

    let breadcrumbs = state.breadcrumbs();
    assert_eq!(breadcrumbs, vec!["main", "files", "upload", "preview"]);

    // Go back twice
    state.go_back().unwrap();
    state.go_back().unwrap();

    let breadcrumbs = state.breadcrumbs();
    assert_eq!(breadcrumbs, vec!["main", "files"]);
}

#[tokio::test]
async fn test_where_command_accuracy() {
    let mut router = CommandRouter::new();
    commands::register_builtin_commands(&mut router);

    // Single menu
    let mut ctx = CommandContext::new(0, "main".to_string());
    let result = router.route("where", &mut ctx).await.unwrap();
    if let CommandResult::Message(msg) = result {
        assert!(msg.contains("main"));
        assert!(!msg.contains(">"));
    }

    // Multiple menus
    ctx.menu_stack.push("main".to_string());
    ctx.menu_stack.push("files".to_string());
    ctx.current_menu = "upload".to_string();

    let result = router.route("where", &mut ctx).await.unwrap();
    if let CommandResult::Message(msg) = result {
        assert!(msg.contains("main > files > upload"));
    }
}

#[test]
fn test_menu_inheritance_field() {
    let menu_toml = r#"
[menu]
name = "submenu"
title = "Sub Menu"
inherits = "main"
"#;

    let menu = MenuParser::parse(menu_toml).unwrap();
    assert_eq!(menu.menu.inherits, Some("main".to_string()));
}

#[test]
fn test_ansi_art_field() {
    let menu_toml = r#"
[menu]
name = "main"
title = "Main Menu"
ansi_art = "main.ans"
"#;

    let menu = MenuParser::parse(menu_toml).unwrap();
    assert_eq!(menu.menu.ansi_art, Some("main.ans".to_string()));
}

#[test]
fn test_empty_menu_options() {
    let menu_toml = r#"
[menu]
name = "empty"
title = "Empty Menu"
"#;

    let menu = MenuParser::parse(menu_toml).unwrap();
    assert_eq!(menu.option.len(), 0);

    let renderer = MenuRenderer::new();
    let rendered = renderer.render(&menu, 255);
    assert!(rendered.valid_keys.is_empty());
}

#[tokio::test]
async fn test_case_insensitive_command_routing() {
    let mut router = CommandRouter::new();
    router.register_fn("files", "Files", 0, |_| {
        CommandResult::ChangeMenu("files".to_string())
    });

    let mut ctx = CommandContext::new(0, "main".to_string());

    // Test various cases
    for variant in &["files", "FILES", "Files", "FiLeS"] {
        let result = router.route(variant, &mut ctx).await.unwrap();
        assert_eq!(result, CommandResult::ChangeMenu("files".to_string()));
    }
}
