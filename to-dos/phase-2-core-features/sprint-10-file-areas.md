# Sprint 10: Menu System & Navigation

**Phase:** Phase 2 - Core Features
**Duration:** 3 weeks
**Sprint Dates:** TBD
**Status:** Not Started

---

## Sprint Overview

Sprint 10 implements the menu system and navigation framework that will be the primary user interface for the BBS. This includes parsing menu files, rendering menus with ANSI support, handling both hot-key and full-menu modes, and routing commands to appropriate handlers.

**Context:** This is the second sprint of Phase 2 (Core Features). The menu system is how users navigate all BBS features.

**Expected Outcomes:** By the end of this sprint, users will be able to navigate through a complete menu hierarchy using both hot-key single-press and full-menu typed commands.

---

## Objectives

- [ ] Implement menu file parser for human-editable menu definitions
- [ ] Create menu rendering engine with ANSI support
- [ ] Support both hot-key and full-menu interaction modes
- [ ] Implement command routing to handler functions

---

## Deliverables

| Deliverable | Type | Acceptance Criteria |
|-------------|------|---------------------|
| Menu file format parser | Code | Parses TOML or custom menu format |
| Menu rendering with ANSI | Code | Displays menus with color and formatting |
| Command routing system | Code | Maps menu selections to Rust handlers |
| Standard menu set | Configuration | Main, file, message, user settings menus |

---

## Detailed Tasks

### Task Category 1: Menu File Format Design

- [ ] **Task 1.1**: Design menu file format (TOML-based)
  - Implementation notes: Menu structure, options, commands, security levels, ANSI art
  - Files affected: `config/menus/*.toml`
  - Estimated hours: 4

- [ ] **Task 1.2**: Implement menu file parser
  - Implementation notes: Load menu definitions, validate structure
  - Files affected: `crates/impulse-menu/src/parser.rs`
  - Estimated hours: 5

- [ ] **Task 1.3**: Add menu inheritance and includes
  - Implementation notes: Allow menus to extend/include other menus
  - Files affected: `crates/impulse-menu/src/parser.rs`
  - Estimated hours: 3

### Task Category 2: Menu Rendering

- [ ] **Task 2.1**: Create MenuRenderer
  - Implementation notes: Render menu to terminal using TerminalDriver
  - Files affected: `crates/impulse-menu/src/renderer.rs`
  - Estimated hours: 5

- [ ] **Task 2.2**: Support hot-key mode (single keypress)
  - Implementation notes: Display keys, accept single character input
  - Files affected: `crates/impulse-menu/src/renderer.rs`
  - Estimated hours: 3

- [ ] **Task 2.3**: Support full-menu mode (typed commands)
  - Implementation notes: Show command prompt, accept text input
  - Files affected: `crates/impulse-menu/src/renderer.rs`
  - Estimated hours: 3

- [ ] **Task 2.4**: Add ANSI art header support
  - Implementation notes: Load and display .ANS files for menu headers
  - Files affected: `crates/impulse-menu/src/renderer.rs`
  - Estimated hours: 2

### Task Category 3: Command Routing

- [ ] **Task 3.1**: Design command handler trait
  - Implementation notes: Trait for menu command handlers
  - Files affected: `crates/impulse-menu/src/handlers.rs`
  - Estimated hours: 3

- [ ] **Task 3.2**: Implement command registry
  - Implementation notes: Map command strings to handlers
  - Files affected: `crates/impulse-menu/src/registry.rs`
  - Estimated hours: 4

- [ ] **Task 3.3**: Add menu state machine
  - Implementation notes: Track current menu, navigation stack, breadcrumbs
  - Files affected: `crates/impulse-menu/src/state.rs`
  - Estimated hours: 5

- [ ] **Task 3.4**: Implement navigation commands (back, main, quit)
  - Implementation notes: Standard navigation, menu stack management
  - Files affected: `crates/impulse-menu/src/navigation.rs`
  - Estimated hours: 3

### Task Category 4: Standard Menus

- [ ] **Task 4.1**: Create main menu
  - Implementation notes: Top-level menu with all major sections
  - Files affected: `config/menus/main.toml`, `assets/screens/main.ans`
  - Estimated hours: 3

- [ ] **Task 4.2**: Create file areas menu
  - Implementation notes: Browse files, search, upload/download
  - Files affected: `config/menus/files.toml`
  - Estimated hours: 2

- [ ] **Task 4.3**: Create message areas menu
  - Implementation notes: Read messages, post, search
  - Files affected: `config/menus/messages.toml`
  - Estimated hours: 2

- [ ] **Task 4.4**: Create user settings menu
  - Implementation notes: Change password, preferences, profile
  - Files affected: `config/menus/settings.toml`
  - Estimated hours: 2

### Task Category 5: Testing and Integration

- [ ] **Task 5.1**: Test menu parsing with various formats
  - Implementation notes: Valid, invalid, edge cases
  - Files affected: `tests/menu_parser_test.rs`
  - Estimated hours: 3

- [ ] **Task 5.2**: Test menu rendering
  - Implementation notes: Different terminal sizes, capabilities
  - Files affected: `tests/menu_render_test.rs`
  - Estimated hours: 3

- [ ] **Task 5.3**: Test command routing
  - Implementation notes: All commands route correctly
  - Files affected: `tests/command_routing_test.rs`
  - Estimated hours: 3

- [ ] **Task 5.4**: Integration test: navigate menu hierarchy
  - Implementation notes: Main → files → back → messages
  - Files affected: `tests/menu_navigation_test.rs`
  - Estimated hours: 4

---

## Technical Details

### Dependencies

```toml
[dependencies]
toml = "0.8"
serde = { workspace = true }
tokio = { workspace = true }
```

### Menu File Format Example

```toml
[menu]
name = "main"
title = "Main Menu"
ansi_art = "main.ans"
mode = "hotkey"  # or "fullmenu"

[[option]]
key = "F"
command = "files"
description = "File Areas"
min_security = 0

[[option]]
key = "M"
command = "messages"
description = "Message Areas"
min_security = 0

[[option]]
key = "G"
command = "goodbye"
description = "Logoff"
min_security = 0
```

**Menu Data Structures (Rust):**
```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Menu {
    pub name: String,
    pub title: String,
    pub ansi_art: Option<String>,
    pub mode: MenuMode,
    pub options: Vec<MenuOption>,
    #[serde(default)]
    pub inherits: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MenuMode {
    Hotkey,
    Fullmenu,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuOption {
    pub key: String,
    pub command: String,
    pub description: String,
    pub min_security: u8,
    #[serde(default)]
    pub max_security: Option<u8>,
}

#[async_trait::async_trait]
pub trait CommandHandler: Send + Sync {
    async fn execute(&self, ctx: &mut SessionContext) -> anyhow::Result<CommandResult>;
    fn name(&self) -> &str;
}

pub enum CommandResult {
    Continue,
    ChangeMenu(String),
    Disconnect,
}

pub struct MenuRenderer {
    terminal: Arc<TerminalDriver>,
    ansi_engine: Arc<AnsiEngine>,
}

impl MenuRenderer {
    pub async fn render_menu(
        &self,
        menu: &Menu,
        user_security: u8,
    ) -> anyhow::Result<()> {
        // Display ANSI art header if present
        if let Some(ansi_file) = &menu.ansi_art {
            self.ansi_engine.display_file(ansi_file).await?;
        } else {
            // Fallback to text header
            self.terminal.write_line(&format!("\n=== {} ===\n", menu.title)).await?;
        }

        // Filter and display menu options based on security level
        let visible_options: Vec<_> = menu.options.iter()
            .filter(|opt| {
                opt.min_security <= user_security &&
                opt.max_security.map_or(true, |max| user_security <= max)
            })
            .collect();

        match menu.mode {
            MenuMode::Hotkey => {
                for option in visible_options {
                    self.terminal.write_line(&format!(
                        "({}) {}",
                        option.key,
                        option.description
                    )).await?;
                }
                self.terminal.write("\nCommand: ").await?;
            }
            MenuMode::Fullmenu => {
                for option in visible_options {
                    self.terminal.write_line(&format!(
                        "{:<15} - {}",
                        option.command,
                        option.description
                    )).await?;
                }
                self.terminal.write("\nEnter command: ").await?;
            }
        }

        Ok(())
    }
}
```

**Command Router:**
```rust
use std::collections::HashMap;
use std::sync::Arc;

pub struct CommandRouter {
    handlers: HashMap<String, Arc<dyn CommandHandler>>,
}

impl CommandRouter {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn register<H>(&mut self, command: String, handler: H)
    where
        H: CommandHandler + 'static,
    {
        self.handlers.insert(command, Arc::new(handler));
    }

    pub async fn route(
        &self,
        command: &str,
        ctx: &mut SessionContext,
    ) -> anyhow::Result<CommandResult> {
        if let Some(handler) = self.handlers.get(command) {
            handler.execute(ctx).await
        } else {
            // Unknown command
            ctx.terminal.write_line("Unknown command. Try again.").await?;
            Ok(CommandResult::Continue)
        }
    }
}

// Example handler for "files" command
pub struct FilesCommandHandler;

#[async_trait::async_trait]
impl CommandHandler for FilesCommandHandler {
    async fn execute(&self, ctx: &mut SessionContext) -> anyhow::Result<CommandResult> {
        // Navigate to file areas menu
        Ok(CommandResult::ChangeMenu("files".to_string()))
    }

    fn name(&self) -> &str {
        "files"
    }
}
```

---

## Acceptance Criteria

- [ ] Menus render correctly with ANSI art
- [ ] Navigation works in both hot-key and full modes
- [ ] Commands route to correct handlers
- [ ] Menu stack maintains navigation history
- [ ] Security levels enforced on menu options

---

## Testing Requirements

### Unit Tests
- [ ] Menu file parser handles valid TOML
- [ ] Parser rejects invalid menu structures
- [ ] Menu inheritance works correctly
- [ ] Security level filtering functions properly
- [ ] Command routing maps correctly

### Integration Tests
- [ ] Full navigation flow (main → sub-menu → back)
- [ ] Both hot-key and full-menu modes work
- [ ] ANSI art loads and displays
- [ ] Menu state persists across screens
- [ ] Invalid commands handled gracefully

### Security Tests
- [ ] Users cannot access menus above their security level
- [ ] Menu options hidden for insufficient privileges
- [ ] No menu command injection possible

### Performance Tests
- [ ] Menu rendering < 50ms for typical menus
- [ ] Menu parsing on startup < 100ms for all menus
- [ ] Memory usage stable during navigation

---

## Notes and Decisions

### Design Decisions
*To be populated during sprint*

**Proposed Decisions:**
- Use TOML for menu files (human-readable, supports hierarchy)
- Support both hot-key (single keypress) and full-menu (typed command) modes
- Allow ANSI art headers for visual appeal
- Menu inheritance for code reuse (base menus extended by specific menus)
- Security levels enforced at menu display time

### Lessons Learned
*To be populated during sprint*

### Risks and Mitigations
- **Risk**: Menu files may be edited incorrectly by SysOps
- **Mitigation**: Strict parsing with helpful error messages; menu validator tool
- **Risk**: Complex menu hierarchies may confuse users
- **Mitigation**: Clear breadcrumbs, always show "back" and "main menu" options
- **Risk**: ANSI art may not render on all terminals
- **Mitigation**: Fallback to text-only menu if ANSI not supported

---

### Week 1
- *Date*: Progress notes will be added here as sprint progresses

### Week 2
- *Date*: Progress notes will be added here as sprint progresses

### Week 3
- *Date*: Progress notes will be added here as sprint progresses

### Sprint Completion
- **Completed**: TBD
- **Velocity**: TBD
- **Burndown**: TBD
