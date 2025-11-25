//! Menu state machine and navigation

use crate::error::{MenuLoadError, NavigationError};
use crate::parser::{MenuDefinition, MenuParser};
use std::collections::HashMap;
use std::path::Path;

/// Menu state machine managing current menu and navigation history
#[derive(Debug, Clone)]
pub struct MenuState {
    /// Name of the current menu
    current_menu: String,
    /// Navigation stack (history of visited menus)
    menu_stack: Vec<String>,
    /// All loaded menus indexed by name
    menus: HashMap<String, MenuDefinition>,
}

impl MenuState {
    /// Create a new menu state with an initial menu
    pub fn new(initial_menu: &str) -> Self {
        Self {
            current_menu: initial_menu.to_string(),
            menu_stack: Vec::new(),
            menus: HashMap::new(),
        }
    }

    /// Load all menus from a directory
    ///
    /// Reads all .toml files in the directory and parses them as menu definitions.
    ///
    /// # Errors
    /// Returns `MenuLoadError` if any menu file fails to load or validate
    pub fn load_menus(&mut self, menu_dir: &Path) -> Result<(), MenuLoadError> {
        let entries = std::fs::read_dir(menu_dir)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("toml") {
                let menu = MenuParser::parse_file(&path)?;

                // Validate the menu
                if let Err(errors) = MenuParser::validate(&menu) {
                    return Err(MenuLoadError::MultipleValidation { errors });
                }

                self.menus.insert(menu.menu.name.clone(), menu);
            }
        }

        Ok(())
    }

    /// Add a menu definition to the state
    pub fn add_menu(&mut self, menu: MenuDefinition) {
        self.menus.insert(menu.menu.name.clone(), menu);
    }

    /// Get the current menu definition
    pub fn current(&self) -> Option<&MenuDefinition> {
        self.menus.get(&self.current_menu)
    }

    /// Get the current menu name
    pub fn current_name(&self) -> &str {
        &self.current_menu
    }

    /// Navigate to a menu (push current to stack)
    ///
    /// # Errors
    /// Returns `NavigationError::MenuNotFound` if the target menu doesn't exist
    pub fn navigate_to(&mut self, menu_name: &str) -> Result<(), NavigationError> {
        if !self.menus.contains_key(menu_name) {
            return Err(NavigationError::MenuNotFound {
                menu: menu_name.to_string(),
            });
        }

        // Push current menu to stack
        self.menu_stack.push(self.current_menu.clone());

        // Navigate to new menu
        self.current_menu = menu_name.to_string();

        Ok(())
    }

    /// Go back to previous menu
    ///
    /// # Errors
    /// Returns `NavigationError::CannotGoBack` if already at main menu
    pub fn go_back(&mut self) -> Result<(), NavigationError> {
        match self.menu_stack.pop() {
            Some(previous_menu) => {
                self.current_menu = previous_menu;
                Ok(())
            }
            None => Err(NavigationError::CannotGoBack),
        }
    }

    /// Return to main menu (clear stack)
    ///
    /// Sets current menu to the bottom of the stack (main menu) or the current
    /// menu if stack is empty.
    pub fn go_main(&mut self) {
        if let Some(main_menu) = self.menu_stack.first() {
            self.current_menu = main_menu.clone();
        }
        self.menu_stack.clear();
    }

    /// Get navigation breadcrumbs
    ///
    /// Returns the menu path from main menu to current menu.
    pub fn breadcrumbs(&self) -> Vec<&str> {
        let mut crumbs: Vec<&str> = self.menu_stack.iter().map(|s| s.as_str()).collect();
        crumbs.push(&self.current_menu);
        crumbs
    }

    /// Get menu by name
    pub fn get_menu(&self, name: &str) -> Option<&MenuDefinition> {
        self.menus.get(name)
    }

    /// Get all menu names
    pub fn menu_names(&self) -> Vec<String> {
        self.menus.keys().cloned().collect()
    }

    /// Get the depth of the navigation stack
    pub fn stack_depth(&self) -> usize {
        self.menu_stack.len()
    }

    /// Check if a menu exists
    pub fn has_menu(&self, name: &str) -> bool {
        self.menus.contains_key(name)
    }

    /// Clear the navigation stack without changing current menu
    pub fn clear_stack(&mut self) {
        self.menu_stack.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{MenuMetadata, MenuMode, MenuOption};

    fn create_test_menu(name: &str, title: &str) -> MenuDefinition {
        MenuDefinition {
            menu: MenuMetadata {
                name: name.to_string(),
                title: title.to_string(),
                ansi_art: None,
                mode: MenuMode::Hotkey,
                inherits: None,
            },
            option: vec![MenuOption {
                key: "Q".to_string(),
                command: "quit".to_string(),
                description: "Quit".to_string(),
                min_security: 0,
                max_security: None,
            }],
        }
    }

    #[test]
    fn test_new_menu_state() {
        let state = MenuState::new("main");
        assert_eq!(state.current_name(), "main");
        assert_eq!(state.stack_depth(), 0);
    }

    #[test]
    fn test_add_and_get_menu() {
        let mut state = MenuState::new("main");
        let menu = create_test_menu("test", "Test Menu");

        state.add_menu(menu.clone());

        assert!(state.has_menu("test"));
        let retrieved = state.get_menu("test").unwrap();
        assert_eq!(retrieved.menu.name, "test");
    }

    #[test]
    fn test_navigate_to() {
        let mut state = MenuState::new("main");

        state.add_menu(create_test_menu("main", "Main Menu"));
        state.add_menu(create_test_menu("files", "File Areas"));

        state.navigate_to("files").unwrap();

        assert_eq!(state.current_name(), "files");
        assert_eq!(state.stack_depth(), 1);
        assert_eq!(state.menu_stack[0], "main");
    }

    #[test]
    fn test_navigate_to_nonexistent() {
        let mut state = MenuState::new("main");

        let result = state.navigate_to("nonexistent");
        assert!(matches!(result, Err(NavigationError::MenuNotFound { .. })));
    }

    #[test]
    fn test_go_back() {
        let mut state = MenuState::new("main");

        state.add_menu(create_test_menu("main", "Main Menu"));
        state.add_menu(create_test_menu("files", "File Areas"));

        state.navigate_to("files").unwrap();
        assert_eq!(state.current_name(), "files");

        state.go_back().unwrap();
        assert_eq!(state.current_name(), "main");
        assert_eq!(state.stack_depth(), 0);
    }

    #[test]
    fn test_go_back_at_main() {
        let mut state = MenuState::new("main");

        let result = state.go_back();
        assert!(matches!(result, Err(NavigationError::CannotGoBack)));
    }

    #[test]
    fn test_go_main() {
        let mut state = MenuState::new("main");

        state.add_menu(create_test_menu("main", "Main Menu"));
        state.add_menu(create_test_menu("files", "File Areas"));
        state.add_menu(create_test_menu("messages", "Messages"));

        // Navigate: main -> files -> messages
        state.navigate_to("files").unwrap();
        state.navigate_to("messages").unwrap();

        assert_eq!(state.stack_depth(), 2);
        assert_eq!(state.current_name(), "messages");

        // Go back to main
        state.go_main();

        assert_eq!(state.current_name(), "main");
        assert_eq!(state.stack_depth(), 0);
    }

    #[test]
    fn test_breadcrumbs() {
        let mut state = MenuState::new("main");

        state.add_menu(create_test_menu("main", "Main Menu"));
        state.add_menu(create_test_menu("files", "File Areas"));
        state.add_menu(create_test_menu("upload", "Upload"));

        // Navigate: main -> files -> upload
        state.navigate_to("files").unwrap();
        state.navigate_to("upload").unwrap();

        let crumbs = state.breadcrumbs();
        assert_eq!(crumbs, vec!["main", "files", "upload"]);
    }

    #[test]
    fn test_breadcrumbs_single_menu() {
        let state = MenuState::new("main");
        let crumbs = state.breadcrumbs();
        assert_eq!(crumbs, vec!["main"]);
    }

    #[test]
    fn test_menu_names() {
        let mut state = MenuState::new("main");

        state.add_menu(create_test_menu("main", "Main"));
        state.add_menu(create_test_menu("files", "Files"));
        state.add_menu(create_test_menu("messages", "Messages"));

        let names = state.menu_names();
        assert_eq!(names.len(), 3);
        assert!(names.contains(&"main".to_string()));
        assert!(names.contains(&"files".to_string()));
        assert!(names.contains(&"messages".to_string()));
    }

    #[test]
    fn test_current() {
        let mut state = MenuState::new("main");
        let menu = create_test_menu("main", "Main Menu");

        state.add_menu(menu);

        let current = state.current().unwrap();
        assert_eq!(current.menu.name, "main");
    }

    #[test]
    fn test_current_none() {
        let state = MenuState::new("nonexistent");
        assert!(state.current().is_none());
    }

    #[test]
    fn test_clear_stack() {
        let mut state = MenuState::new("main");

        state.add_menu(create_test_menu("main", "Main Menu"));
        state.add_menu(create_test_menu("files", "File Areas"));

        state.navigate_to("files").unwrap();
        assert_eq!(state.stack_depth(), 1);

        state.clear_stack();
        assert_eq!(state.stack_depth(), 0);
        assert_eq!(state.current_name(), "files"); // Current menu unchanged
    }

    #[test]
    fn test_navigation_sequence() {
        let mut state = MenuState::new("main");

        state.add_menu(create_test_menu("main", "Main"));
        state.add_menu(create_test_menu("files", "Files"));
        state.add_menu(create_test_menu("messages", "Messages"));
        state.add_menu(create_test_menu("settings", "Settings"));

        // Complex navigation
        state.navigate_to("files").unwrap();
        state.navigate_to("messages").unwrap();
        state.go_back().unwrap(); // back to files
        state.navigate_to("settings").unwrap();
        state.go_back().unwrap(); // back to files
        state.go_back().unwrap(); // back to main

        assert_eq!(state.current_name(), "main");
        assert_eq!(state.stack_depth(), 0);
    }

    #[test]
    fn test_load_menus_empty_directory() {
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let mut state = MenuState::new("main");

        let result = state.load_menus(temp_dir.path());
        assert!(result.is_ok());
        assert_eq!(state.menu_names().len(), 0);
    }

    #[test]
    fn test_load_menus_with_files() {
        use std::fs::write;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let menu_path = temp_dir.path().join("test.toml");

        let menu_toml = r#"
[menu]
name = "test"
title = "Test Menu"

[[option]]
key = "Q"
command = "quit"
description = "Quit"
min_security = 0
"#;

        write(&menu_path, menu_toml).unwrap();

        let mut state = MenuState::new("main");
        state.load_menus(temp_dir.path()).unwrap();

        assert!(state.has_menu("test"));
    }

    #[test]
    fn test_load_menus_ignores_non_toml() {
        use std::fs::write;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let txt_path = temp_dir.path().join("readme.txt");

        write(&txt_path, "This is not a menu").unwrap();

        let mut state = MenuState::new("main");
        let result = state.load_menus(temp_dir.path());

        assert!(result.is_ok());
        assert_eq!(state.menu_names().len(), 0);
    }
}
