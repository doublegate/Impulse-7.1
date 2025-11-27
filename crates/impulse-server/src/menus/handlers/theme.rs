//! Theme selection handler

use crate::state::ServerState;
use anyhow::Result;
use impulse_telnet::TelnetConnection;
use impulse_terminal::theme::ThemePreview;
use impulse_terminal::{AnsiRenderer, Color};
use impulse_types::user::User;

/// Handle theme selection
pub async fn handle_theme_selection(
    connection: &mut TelnetConnection,
    _user: &User,
    state: &ServerState,
    renderer: &mut AnsiRenderer,
) -> Result<()> {
    // Get available themes
    let theme_manager = state.theme_manager.read().await;
    let themes = theme_manager.list_themes().await;
    drop(theme_manager);

    loop {
        renderer.clear_screen();
        renderer.set_foreground(Color::BrightCyan);
        renderer.write_line(
            "╔══════════════════════════════════════════════════════════════════════════╗",
        );
        renderer.write_line(
            "║                         THEME SELECTION                                  ║",
        );
        renderer.write_line(
            "╚══════════════════════════════════════════════════════════════════════════╝",
        );
        renderer.reset();
        renderer.write_line("");

        renderer.set_foreground(Color::BrightWhite);
        renderer.write_line("Available Themes:");
        renderer.reset();
        renderer.write_line("");

        if themes.is_empty() {
            renderer.set_foreground(Color::BrightYellow);
            renderer.write_line("No themes are currently available.");
            renderer.write_line("Using default color scheme.");
            renderer.reset();
        } else {
            for (idx, theme) in themes.iter().enumerate() {
                renderer.set_foreground(Color::BrightYellow);
                renderer.write_line(&format!("{}. {}", idx + 1, theme.name));
                renderer.reset();
                renderer.set_foreground(Color::White);
                renderer.write_line(&format!("   by {}", theme.author));
                renderer.write_line(&format!("   {}", theme.description));
                renderer.reset();
                renderer.write_line("");
            }
        }

        renderer.set_foreground(Color::Yellow);
        renderer.write_line("Theme System Features:");
        renderer.write_line("  • Color scheme customization");
        renderer.write_line("  • ANSI color support (16/256/RGB)");
        renderer.write_line("  • Theme preview before applying");
        renderer.write_line("  • Hot-reload capability");
        renderer.write_line("  • TOML-based theme configuration");
        renderer.reset();
        renderer.write_line("");

        if !themes.is_empty() {
            renderer.set_foreground(Color::BrightYellow);
            renderer.write_text(&format!(
                "Select theme (1-{}), [P]review, or [Q] to quit: ",
                themes.len()
            ));
            renderer.reset();
            connection
                .send_raw(renderer.take_output().as_bytes())
                .await?;

            // Read selection
            match connection.read_line().await {
                Ok(input) => {
                    let input = input.trim();
                    if input.eq_ignore_ascii_case("q") {
                        return Ok(());
                    } else if input.eq_ignore_ascii_case("p") {
                        // Preview current theme
                        let theme_manager = state.theme_manager.read().await;
                        let current = theme_manager.current_theme().await;
                        drop(theme_manager);

                        let previewer = ThemePreview::new();
                        match previewer.preview_theme(&current) {
                            Ok(preview_text) => {
                                renderer.clear();
                                renderer.write_line("\r\n");
                                renderer.write_text(&preview_text);
                                renderer.write_line("\r\n");
                                renderer.set_foreground(Color::BrightYellow);
                                renderer.write_line("Press any key to continue...");
                                renderer.reset();
                                connection
                                    .send_raw(renderer.take_output().as_bytes())
                                    .await?;
                                connection.read_char().await.ok();
                            }
                            Err(e) => {
                                renderer.clear();
                                renderer.write_line("\r\n");
                                renderer.set_foreground(Color::BrightRed);
                                renderer.write_line(&format!("Error generating preview: {}", e));
                                renderer.reset();
                                renderer.write_line("\r\n");
                                renderer.set_foreground(Color::BrightYellow);
                                renderer.write_line("Press any key to continue...");
                                renderer.reset();
                                connection
                                    .send_raw(renderer.take_output().as_bytes())
                                    .await?;
                                connection.read_char().await.ok();
                            }
                        }
                    } else if let Ok(selection) = input.parse::<usize>()
                        && selection > 0
                        && selection <= themes.len()
                    {
                        let theme_name = &themes[selection - 1].name;
                        let theme_manager = state.theme_manager.write().await;
                        match theme_manager.switch_theme(theme_name).await {
                            Ok(_) => {
                                renderer.clear();
                                renderer.write_line("\r\n");
                                renderer.set_foreground(Color::BrightGreen);
                                renderer.write_line(&format!(
                                    "Theme '{}' applied successfully!",
                                    theme_name
                                ));
                                renderer.reset();
                                renderer.write_line("\r\n");
                                renderer.set_foreground(Color::BrightYellow);
                                renderer.write_line("Press any key to continue...");
                                renderer.reset();
                                connection
                                    .send_raw(renderer.take_output().as_bytes())
                                    .await?;
                                connection.read_char().await.ok();
                            }
                            Err(e) => {
                                renderer.clear();
                                renderer.write_line("\r\n");
                                renderer.set_foreground(Color::BrightRed);
                                renderer.write_line(&format!("Error applying theme: {}", e));
                                renderer.reset();
                                renderer.write_line("\r\n");
                                renderer.set_foreground(Color::BrightYellow);
                                renderer.write_line("Press any key to continue...");
                                renderer.reset();
                                connection
                                    .send_raw(renderer.take_output().as_bytes())
                                    .await?;
                                connection.read_char().await.ok();
                            }
                        }
                    }
                }
                Err(_) => {
                    return Ok(());
                }
            }
        } else {
            renderer.set_foreground(Color::BrightYellow);
            renderer.write_line("Press any key to continue...");
            renderer.reset();
            connection
                .send_raw(renderer.take_output().as_bytes())
                .await?;
            connection.read_char().await.ok();
            return Ok(());
        }
    }
}
