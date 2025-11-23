# Sprint 20: Theme System

**Phase:** Phase 3 - Feature Completion
**Duration:** 3 weeks
**Sprint Dates:** TBD
**Status:** Not Started

---

## Sprint Overview

Sprint 20 implements a comprehensive theme system allowing users to customize the visual appearance of the BBS. Includes theme switching, preview functionality, and multiple default themes.

**Context:** Sprint 4 of Phase 3. Enhances user experience with visual customization.

**Expected Outcomes:** Users can select and switch between polished themes that transform the BBS appearance.

---

## Objectives

- [ ] Implement theme architecture and loading system
- [ ] Create theme switching functionality
- [ ] Develop 3 default themes (Classic, Matrix, Cyberpunk)
- [ ] Support theme previews

---

## Deliverables

| Deliverable | Type | Acceptance Criteria |
|-------------|------|---------------------|
| ThemeManager | Code | Load and manage themes |
| Theme format specification | Docs | Documented theme structure |
| Default themes | Assets | 3 polished, complete themes |
| Theme switcher UI | UI | Preview and select themes |

---

## Detailed Tasks

### Task Category 1: Theme Architecture

- [ ] **Task 1.1**: Design theme directory structure
  - Files affected: `themes/` directory structure
  - Estimated hours: 3

- [ ] **Task 1.2**: Define theme metadata format
  - Files affected: `crates/impulse-terminal/src/theme/metadata.rs`
  - Estimated hours: 4

- [ ] **Task 1.3**: Implement ThemeManager
  - Files affected: `crates/impulse-terminal/src/theme/manager.rs`
  - Estimated hours: 6

- [ ] **Task 1.4**: Theme loading and caching
  - Files affected: `crates/impulse-terminal/src/theme/loader.rs`
  - Estimated hours: 5

### Task Category 2: Theme Content

- [ ] **Task 2.1**: Create Classic theme
  - Files affected: `themes/classic/`
  - Estimated hours: 12

- [ ] **Task 2.2**: Create Matrix theme
  - Files affected: `themes/matrix/`
  - Estimated hours: 12

- [ ] **Task 2.3**: Create Cyberpunk theme
  - Files affected: `themes/cyberpunk/`
  - Estimated hours: 12

### Task Category 3: Theme Switching

- [ ] **Task 3.1**: Implement theme preview
  - Files affected: `crates/impulse-terminal/src/theme/preview.rs`
  - Estimated hours: 6

- [ ] **Task 3.2**: Theme selection UI
  - Files affected: `crates/impulse-terminal/src/screens/theme_select.rs`
  - Estimated hours: 5

- [ ] **Task 3.3**: Apply theme on selection
  - Files affected: `crates/impulse-terminal/src/theme/apply.rs`
  - Estimated hours: 4

### Task Category 4: Testing

- [ ] **Task 4.1**: Test theme loading
  - Estimated hours: 3

- [ ] **Task 4.2**: Test theme switching
  - Estimated hours: 4

- [ ] **Task 4.3**: Visual QA for all themes
  - Estimated hours: 6

---

## Technical Details

### Architecture Considerations

- Theme assets organized in separate directories per theme
- Hot-reload themes without restarting BBS
- Theme metadata includes compatibility version
- Support both ANSI art and text-mode fallbacks
- Cache parsed ANSI files for performance

### Dependencies

**Crate-Level Dependencies:**
```toml
[dependencies]
tokio = { workspace = true }
serde = { workspace = true }
serde_json = "1.0"
toml = "0.8"
walkdir = "2.4"

[dev-dependencies]
tempfile = "3.8"
```

**Pascal Units Being Converted:**
- THEME.PAS (Theme management)
- COLORS.PAS (Color scheme definitions)
- SCREENS.PAS (Screen template system)

### Code Examples

**Theme System Architecture:**
```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub metadata: ThemeMetadata,
    pub color_scheme: ColorScheme,
    pub screens: HashMap<String, String>,  // screen name -> ANSI file path
    pub prompts: HashMap<String, String>,  // prompt ID -> text template
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeMetadata {
    pub name: String,
    pub author: String,
    pub version: String,
    pub description: String,
    pub compatible_bbs_version: String,
    pub requires_ansi: bool,
    pub requires_utf8: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorScheme {
    pub primary: AnsiColor,
    pub secondary: AnsiColor,
    pub accent: AnsiColor,
    pub background: AnsiColor,
    pub text: AnsiColor,
    pub highlight: AnsiColor,
    pub error: AnsiColor,
    pub success: AnsiColor,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AnsiColor {
    pub foreground: u8,  // 0-15 ANSI color
    pub background: u8,
    pub bold: bool,
    pub blink: bool,
}

pub struct ThemeManager {
    themes: HashMap<String, Theme>,
    current_theme: String,
    theme_dir: PathBuf,
}

impl ThemeManager {
    pub async fn new(theme_dir: PathBuf) -> anyhow::Result<Self> {
        let mut themes = HashMap::new();

        // Scan theme directory
        for entry in walkdir::WalkDir::new(&theme_dir)
            .max_depth(2)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_name() == "theme.toml" {
                let theme = Self::load_theme(entry.path()).await?;
                themes.insert(theme.metadata.name.clone(), theme);
            }
        }

        Ok(Self {
            themes,
            current_theme: "classic".to_string(),
            theme_dir,
        })
    }

    pub async fn switch_theme(&mut self, theme_name: &str) -> anyhow::Result<()> {
        if !self.themes.contains_key(theme_name) {
            return Err(anyhow::anyhow!("Theme '{}' not found", theme_name));
        }

        self.current_theme = theme_name.to_string();
        Ok(())
    }

    pub fn get_screen(&self, screen_name: &str) -> Option<PathBuf> {
        self.themes
            .get(&self.current_theme)?
            .screens
            .get(screen_name)
            .map(|p| self.theme_dir.join(&self.current_theme).join(p))
    }

    pub fn get_prompt(&self, prompt_id: &str) -> Option<String> {
        self.themes
            .get(&self.current_theme)?
            .prompts
            .get(prompt_id)
            .cloned()
    }

    pub fn list_themes(&self) -> Vec<&ThemeMetadata> {
        self.themes.values().map(|t| &t.metadata).collect()
    }

    async fn load_theme(path: &std::path::Path) -> anyhow::Result<Theme> {
        let content = tokio::fs::read_to_string(path).await?;
        let theme: Theme = toml::from_str(&content)?;
        Ok(theme)
    }
}
```

**Theme Preview System:**
```rust
pub struct ThemePreview {
    terminal: Arc<TerminalDriver>,
    ansi_engine: Arc<AnsiEngine>,
}

impl ThemePreview {
    pub async fn preview_theme(
        &self,
        theme: &Theme,
    ) -> anyhow::Result<()> {
        // Display theme metadata
        self.terminal.write_line(&format!("\nTheme: {}", theme.metadata.name)).await?;
        self.terminal.write_line(&format!("Author: {}", theme.metadata.author)).await?;
        self.terminal.write_line(&format!("Description: {}\n", theme.metadata.description)).await?;

        // Display welcome screen if available
        if let Some(welcome_screen) = theme.screens.get("welcome") {
            self.ansi_engine.display_file(welcome_screen).await?;
        }

        // Show color samples
        self.display_color_samples(&theme.color_scheme).await?;

        // Show sample prompts
        self.display_sample_prompts(theme).await?;

        Ok(())
    }

    async fn display_color_samples(
        &self,
        scheme: &ColorScheme,
    ) -> anyhow::Result<()> {
        self.terminal.write_line("\n=== Color Scheme ===").await?;

        let samples = [
            ("Primary", scheme.primary),
            ("Secondary", scheme.secondary),
            ("Accent", scheme.accent),
            ("Text", scheme.text),
            ("Highlight", scheme.highlight),
            ("Error", scheme.error),
            ("Success", scheme.success),
        ];

        for (name, color) in samples {
            let ansi_code = self.color_to_ansi(color);
            self.terminal.write(&format!(
                "{}{:12} ████████ Sample Text\x1b[0m\n",
                ansi_code,
                name
            )).await?;
        }

        Ok(())
    }

    fn color_to_ansi(&self, color: AnsiColor) -> String {
        let mut code = String::from("\x1b[");

        if color.bold {
            code.push_str("1;");
        }
        if color.blink {
            code.push_str("5;");
        }

        code.push_str(&format!("{};{}m",
            30 + (color.foreground % 8),
            40 + (color.background % 8)
        ));

        code
    }

    async fn display_sample_prompts(&self, theme: &Theme) -> anyhow::Result<()> {
        self.terminal.write_line("\n=== Sample Prompts ===").await?;

        let sample_prompts = ["main_menu", "enter_password", "file_select", "message_prompt"];

        for prompt_id in sample_prompts {
            if let Some(prompt_text) = theme.prompts.get(prompt_id) {
                self.terminal.write_line(&format!("{}: {}", prompt_id, prompt_text)).await?;
            }
        }

        Ok(())
    }
}
```

**Theme Selection UI:**
```rust
pub struct ThemeSelector {
    theme_manager: Arc<ThemeManager>,
    preview: ThemePreview,
}

impl ThemeSelector {
    pub async fn select_theme(
        &self,
        terminal: &TerminalDriver,
    ) -> anyhow::Result<String> {
        let themes = self.theme_manager.list_themes();

        terminal.write_line("\n=== Available Themes ===\n").await?;

        for (idx, theme) in themes.iter().enumerate() {
            terminal.write_line(&format!(
                "({}) {} by {} - {}",
                idx + 1,
                theme.name,
                theme.author,
                theme.description
            )).await?;
        }

        terminal.write("\nSelect theme number (P to preview): ").await?;

        let input = terminal.read_line().await?;

        if input.to_uppercase().starts_with('P') {
            // Preview mode
            terminal.write("Preview which theme? ").await?;
            let preview_num = terminal.read_line().await?.trim().parse::<usize>()?;

            if preview_num > 0 && preview_num <= themes.len() {
                let theme_name = &themes[preview_num - 1].name;
                if let Some(theme) = self.theme_manager.themes.get(theme_name) {
                    self.preview.preview_theme(theme).await?;
                }
            }

            // Recursively call to select after preview
            return self.select_theme(terminal).await;
        }

        // Parse selection
        let selection = input.trim().parse::<usize>()?;

        if selection > 0 && selection <= themes.len() {
            Ok(themes[selection - 1].name.clone())
        } else {
            terminal.write_line("Invalid selection.").await?;
            Ok("classic".to_string())
        }
    }
}
```

---

## Dependencies

### Upstream Dependencies
- **Sprint 04**: ANSI engine for rendering themed screens
- **Sprint 06**: User system for storing theme preferences
- **Sprint 10**: Menu system to apply themes

### Blocks Downstream
- **Sprint 23**: Admin interface for theme management
- **Sprint 30**: Beta testing includes theme feedback

---

## Acceptance Criteria

- [ ] Themes change entire visual experience
- [ ] Theme switching is seamless (no restart)
- [ ] Default themes (Classic, Matrix, Cyberpunk) are polished and functional
- [ ] Preview works before selection
- [ ] User preference persists in database
- [ ] Themes hot-reload when files change
- [ ] Fallback to text mode if ANSI not supported
- [ ] Theme metadata validates on load
- [ ] All screens have themed versions
- [ ] Color schemes apply consistently

---

## Testing Requirements

### Unit Tests
- [ ] Theme loading from TOML
- [ ] Theme validation (required fields)
- [ ] Color code generation (ANSI escape sequences)
- [ ] Screen path resolution
- [ ] Prompt template substitution

### Integration Tests
- [ ] Switch between all three default themes
- [ ] Preview theme and cancel
- [ ] Preview theme and apply
- [ ] Invalid theme file handling
- [ ] Missing screen fallback

### Visual QA Tests
- [ ] Classic theme renders correctly on all screens
- [ ] Matrix theme renders correctly on all screens
- [ ] Cyberpunk theme renders correctly on all screens
- [ ] Colors display properly on different terminals (xterm, PuTTY, Terminal.app)
- [ ] Text-mode fallback works without ANSI

### Performance Tests
- [ ] Theme switching < 100ms
- [ ] Theme loading on startup < 200ms
- [ ] ANSI file caching reduces repeated renders
- [ ] Memory usage stable with multiple themes

---

## Notes and Decisions

### Design Decisions
*To be populated during sprint*

**Proposed Decisions:**
- Use TOML for theme metadata (human-editable, structured)
- Organize themes in `themes/[theme-name]/` directories
- Default theme is "Classic" (reminiscent of original Impulse 7.1)
- Matrix theme uses green-on-black color scheme
- Cyberpunk theme uses magenta/cyan neon aesthetics
- Support hot-reload for theme development
- Cache parsed ANSI files in memory

### Lessons Learned
*To be populated during sprint*

### Risks and Mitigations
- **Risk**: ANSI art may not render on all terminals
- **Mitigation**: Detect terminal capabilities; fallback to text mode; test on multiple emulators
- **Risk**: Theme files may be malformed or corrupt
- **Mitigation**: Strict validation on load; helpful error messages; schema documentation
- **Risk**: Creating high-quality ANSI art is time-consuming
- **Mitigation**: Start with simple themes; community contributions; ANSI art tools
- **Risk**: Themes may break with BBS updates
- **Mitigation**: Version compatibility checking; migration guide for theme authors

---

## Progress Log

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
