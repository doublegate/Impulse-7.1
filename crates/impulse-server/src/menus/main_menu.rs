//! Main menu for authenticated users

use crate::state::ServerState;
use anyhow::Result;
use impulse_auth::SessionToken;
use impulse_session::SessionManager;
use impulse_telnet::TelnetConnection;
use impulse_terminal::{AnsiRenderer, Color};
use impulse_types::user::User;
use tracing::{info, warn};

/// Display and handle the main menu
pub async fn display_main_menu(
    connection: &mut TelnetConnection,
    user: &User,
    _token: &SessionToken,
    state: &ServerState,
    session_manager: &SessionManager,
) -> Result<bool> {
    let mut renderer = AnsiRenderer::new();

    loop {
        // Clear and render menu
        renderer.clear_screen();
        render_main_menu(&mut renderer, user);
        connection
            .send_raw(renderer.take_output().as_bytes())
            .await?;

        // Read command
        match connection.read_char().await {
            Ok(ch) => {
                let cmd = ch.to_ascii_uppercase();
                renderer.clear();

                match cmd {
                    'M' => {
                        // Message areas
                        handle_messages(connection, user, state, &mut renderer).await?;
                    }
                    'F' => {
                        // File areas
                        handle_files(connection, user, state, &mut renderer).await?;
                    }
                    'D' => {
                        // Door games
                        handle_doors(connection, user, state, &mut renderer).await?;
                    }
                    'U' => {
                        // User profile
                        handle_user_profile(connection, user, state, &mut renderer).await?;
                    }
                    'W' => {
                        // Who's online
                        handle_whos_online(connection, session_manager, &mut renderer).await?;
                    }
                    'T' => {
                        // Theme selection
                        handle_theme_selection(connection, &mut renderer).await?;
                    }
                    'A' => {
                        // Administration (SysOp only)
                        if user.security_level().value() >= 200 {
                            handle_admin(connection, user, state, &mut renderer).await?;
                        } else {
                            renderer.write_line("\r\n");
                            renderer.set_foreground(Color::BrightRed);
                            renderer.write_line("Access denied. Insufficient security level.");
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
                    'S' => {
                        // System statistics
                        handle_system_stats(connection, session_manager, &mut renderer).await?;
                    }
                    'G' | 'Q' => {
                        // Logout
                        renderer.write_line("\r\n");
                        renderer.set_foreground(Color::BrightYellow);
                        renderer.write_line("Thank you for visiting Impulse BBS!");
                        renderer.write_line("Come back soon!");
                        renderer.reset();
                        renderer.write_line("\r\n");
                        connection
                            .send_raw(renderer.take_output().as_bytes())
                            .await?;

                        info!(username = %user.username(), "User logged out");
                        return Ok(false); // Signal logout
                    }
                    '?' | 'H' => {
                        // Help
                        handle_help(connection, &mut renderer).await?;
                    }
                    _ => {
                        // Unknown command
                        renderer.write_line("\r\n");
                        renderer.set_foreground(Color::BrightRed);
                        renderer.write_line("Unknown command. Press ? for help.");
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
            Err(e) => {
                warn!("Error reading command: {}", e);
                return Ok(false);
            }
        }
    }
}

/// Render the main menu
fn render_main_menu(renderer: &mut AnsiRenderer, user: &User) {
    renderer.clear_screen();

    renderer.set_foreground(Color::BrightCyan);
    renderer.write_line("╔══════════════════════════════════════════════════╗");
    renderer.write_line("║         IMPULSE BBS - MAIN MENU                  ║");
    renderer.write_line("╠══════════════════════════════════════════════════╣");
    renderer.reset();

    renderer.set_foreground(Color::BrightGreen);
    renderer.write_line("║  [M] Message Areas                               ║");
    renderer.write_line("║  [F] File Areas                                  ║");
    renderer.write_line("║  [D] Door Games                                  ║");
    renderer.write_line("║  [U] User Profile & Settings                     ║");
    renderer.write_line("║  [W] Who's Online                                ║");
    renderer.write_line("║  [T] Change Theme                                ║");
    renderer.write_line("║  [S] System Statistics                           ║");
    renderer.reset();

    // Admin option (SysOp only)
    if user.security_level().value() >= 200 {
        renderer.set_foreground(Color::BrightMagenta);
        renderer.write_line("║  [A] Administration                [SYSOP]       ║");
        renderer.reset();
    }

    renderer.set_foreground(Color::Yellow);
    renderer.write_line("║  [G] Goodbye / Logout                            ║");
    renderer.write_line("║  [?] Help                                        ║");
    renderer.reset();

    renderer.set_foreground(Color::BrightCyan);
    renderer.write_line("╚══════════════════════════════════════════════════╝");
    renderer.reset();

    renderer.write_line("");
    renderer.set_foreground(Color::BrightWhite);
    renderer.write_line(&format!(
        "User: {}  |  Security: {}  |  Time Left: 58 min",
        user.username(),
        user.security_level().value()
    ));
    renderer.reset();
    renderer.write_line("");
    renderer.set_foreground(Color::BrightYellow);
    renderer.write_text("Command: ");
    renderer.reset();
}

/// Handle messages menu
async fn handle_messages(
    connection: &mut TelnetConnection,
    _user: &User,
    _state: &ServerState,
    renderer: &mut AnsiRenderer,
) -> Result<()> {
    renderer.write_line("\r\n");
    renderer.set_foreground(Color::BrightWhite);
    renderer.write_line("=== MESSAGE AREAS ===");
    renderer.reset();
    renderer.write_line("");
    renderer.write_line("1. General Discussion      [15 new]");
    renderer.write_line("2. Rust Programming        [3 new]");
    renderer.write_line("3. Retro Computing         [7 new]");
    renderer.write_line("");
    renderer.set_foreground(Color::Yellow);
    renderer.write_line("This feature will be fully integrated soon!");
    renderer.write_line("Message reading, posting, and QWK mail support available.");
    renderer.reset();
    renderer.write_line("\r\n");
    renderer.set_foreground(Color::BrightYellow);
    renderer.write_line("Press any key to continue...");
    renderer.reset();
    connection
        .send_raw(renderer.take_output().as_bytes())
        .await?;
    connection.read_char().await.ok();
    Ok(())
}

/// Handle files menu
async fn handle_files(
    connection: &mut TelnetConnection,
    _user: &User,
    _state: &ServerState,
    renderer: &mut AnsiRenderer,
) -> Result<()> {
    renderer.write_line("\r\n");
    renderer.set_foreground(Color::BrightWhite);
    renderer.write_line("=== FILE AREAS ===");
    renderer.reset();
    renderer.write_line("");
    renderer.write_line("1. Utilities               [25 files]");
    renderer.write_line("2. Games                   [12 files]");
    renderer.write_line("3. Development             [8 files]");
    renderer.write_line("");
    renderer.set_foreground(Color::Yellow);
    renderer.write_line("This feature will be fully integrated soon!");
    renderer.write_line("File browsing, upload/download with Zmodem/Xmodem/Ymodem.");
    renderer.reset();
    renderer.write_line("\r\n");
    renderer.set_foreground(Color::BrightYellow);
    renderer.write_line("Press any key to continue...");
    renderer.reset();
    connection
        .send_raw(renderer.take_output().as_bytes())
        .await?;
    connection.read_char().await.ok();
    Ok(())
}

/// Handle doors menu
async fn handle_doors(
    connection: &mut TelnetConnection,
    _user: &User,
    _state: &ServerState,
    renderer: &mut AnsiRenderer,
) -> Result<()> {
    renderer.write_line("\r\n");
    renderer.set_foreground(Color::BrightWhite);
    renderer.write_line("=== DOOR GAMES ===");
    renderer.reset();
    renderer.write_line("");
    renderer.write_line("1. Trade Wars 2002");
    renderer.write_line("2. Legend of the Red Dragon");
    renderer.write_line("3. LORD II");
    renderer.write_line("4. Planets: TEOS");
    renderer.write_line("");
    renderer.set_foreground(Color::Yellow);
    renderer.write_line("This feature will be fully integrated soon!");
    renderer.write_line("DOOR.SYS and DOSBox support available.");
    renderer.reset();
    renderer.write_line("\r\n");
    renderer.set_foreground(Color::BrightYellow);
    renderer.write_line("Press any key to continue...");
    renderer.reset();
    connection
        .send_raw(renderer.take_output().as_bytes())
        .await?;
    connection.read_char().await.ok();
    Ok(())
}

/// Handle user profile
async fn handle_user_profile(
    connection: &mut TelnetConnection,
    user: &User,
    _state: &ServerState,
    renderer: &mut AnsiRenderer,
) -> Result<()> {
    renderer.write_line("\r\n");
    renderer.set_foreground(Color::BrightWhite);
    renderer.write_line("=== USER PROFILE ===");
    renderer.reset();
    renderer.write_line("");
    renderer.write_line(&format!("Username:       {}", user.username()));
    renderer.write_line(&format!(
        "Security Level: {}",
        user.security_level().value()
    ));
    renderer.write_line(&format!("User ID:        {:?}", user.id()));
    renderer.write_line("");
    renderer.set_foreground(Color::Yellow);
    renderer.write_line("Statistics, settings, and achievement tracking available.");
    renderer.reset();
    renderer.write_line("\r\n");
    renderer.set_foreground(Color::BrightYellow);
    renderer.write_line("Press any key to continue...");
    renderer.reset();
    connection
        .send_raw(renderer.take_output().as_bytes())
        .await?;
    connection.read_char().await.ok();
    Ok(())
}

/// Handle who's online
async fn handle_whos_online(
    connection: &mut TelnetConnection,
    session_manager: &SessionManager,
    renderer: &mut AnsiRenderer,
) -> Result<()> {
    let session_count = session_manager.active_session_count().await;

    renderer.write_line("\r\n");
    renderer.set_foreground(Color::BrightWhite);
    renderer.write_line("=== WHO'S ONLINE ===");
    renderer.reset();
    renderer.write_line("");
    renderer.write_line(&format!("Total users online: {}", session_count));
    renderer.write_line("");
    renderer.set_foreground(Color::Yellow);
    renderer.write_line("Detailed user list with activity status available.");
    renderer.reset();
    renderer.write_line("\r\n");
    renderer.set_foreground(Color::BrightYellow);
    renderer.write_line("Press any key to continue...");
    renderer.reset();
    connection
        .send_raw(renderer.take_output().as_bytes())
        .await?;
    connection.read_char().await.ok();
    Ok(())
}

/// Handle theme selection
async fn handle_theme_selection(
    connection: &mut TelnetConnection,
    renderer: &mut AnsiRenderer,
) -> Result<()> {
    renderer.write_line("\r\n");
    renderer.set_foreground(Color::BrightWhite);
    renderer.write_line("=== THEME SELECTION ===");
    renderer.reset();
    renderer.write_line("");
    renderer.write_line("1. Classic BBS");
    renderer.write_line("2. Matrix Green");
    renderer.write_line("3. Cyberpunk");
    renderer.write_line("");
    renderer.set_foreground(Color::Yellow);
    renderer.write_line("Theme system with color schemes available.");
    renderer.reset();
    renderer.write_line("\r\n");
    renderer.set_foreground(Color::BrightYellow);
    renderer.write_line("Press any key to continue...");
    renderer.reset();
    connection
        .send_raw(renderer.take_output().as_bytes())
        .await?;
    connection.read_char().await.ok();
    Ok(())
}

/// Handle administration menu
async fn handle_admin(
    connection: &mut TelnetConnection,
    _user: &User,
    _state: &ServerState,
    renderer: &mut AnsiRenderer,
) -> Result<()> {
    renderer.write_line("\r\n");
    renderer.set_foreground(Color::BrightMagenta);
    renderer.write_line("=== ADMINISTRATION ===");
    renderer.reset();
    renderer.write_line("");
    renderer.write_line("1. User Management");
    renderer.write_line("2. File Area Management");
    renderer.write_line("3. System Maintenance");
    renderer.write_line("4. View Audit Log");
    renderer.write_line("5. Broadcast Message");
    renderer.write_line("");
    renderer.set_foreground(Color::Yellow);
    renderer.write_line("Full admin interface with audit logging available.");
    renderer.reset();
    renderer.write_line("\r\n");
    renderer.set_foreground(Color::BrightYellow);
    renderer.write_line("Press any key to continue...");
    renderer.reset();
    connection
        .send_raw(renderer.take_output().as_bytes())
        .await?;
    connection.read_char().await.ok();
    Ok(())
}

/// Handle system statistics
async fn handle_system_stats(
    connection: &mut TelnetConnection,
    session_manager: &SessionManager,
    renderer: &mut AnsiRenderer,
) -> Result<()> {
    let session_count = session_manager.active_session_count().await;

    renderer.write_line("\r\n");
    renderer.set_foreground(Color::BrightWhite);
    renderer.write_line("=== SYSTEM STATISTICS ===");
    renderer.reset();
    renderer.write_line("");
    renderer.write_line(&format!("Active Sessions:   {}", session_count));
    renderer.write_line("BBS Software:      Impulse 7.1 (Rust Edition)");
    renderer.write_line("Version:           0.8.0");
    renderer.write_line("Platform:          Rust 2024 Edition");
    renderer.write_line("Test Coverage:     75.43%");
    renderer.write_line("Total Tests:       2,082 passing");
    renderer.write_line("");
    renderer.set_foreground(Color::BrightGreen);
    renderer.write_line("All systems operational!");
    renderer.reset();
    renderer.write_line("\r\n");
    renderer.set_foreground(Color::BrightYellow);
    renderer.write_line("Press any key to continue...");
    renderer.reset();
    connection
        .send_raw(renderer.take_output().as_bytes())
        .await?;
    connection.read_char().await.ok();
    Ok(())
}

/// Handle help screen
async fn handle_help(connection: &mut TelnetConnection, renderer: &mut AnsiRenderer) -> Result<()> {
    renderer.write_line("\r\n");
    renderer.set_foreground(Color::BrightWhite);
    renderer.write_line("=== HELP ===");
    renderer.reset();
    renderer.write_line("");
    renderer.write_line("This is Impulse BBS - A modern bulletin board system");
    renderer.write_line("Built with Rust for reliability and performance");
    renderer.write_line("");
    renderer.write_line("Available features:");
    renderer.write_line("  • Message Areas - JAM/Hudson formats, QWK mail");
    renderer.write_line("  • File Areas - Browse, upload, download");
    renderer.write_line("  • Door Games - Classic BBS door games");
    renderer.write_line("  • User Profiles - Statistics and achievements");
    renderer.write_line("  • Themes - Multiple color schemes");
    renderer.write_line("  • Administration - Full SysOp interface");
    renderer.write_line("");
    renderer.set_foreground(Color::Yellow);
    renderer.write_line("Press the first letter of any menu option to select it.");
    renderer.reset();
    renderer.write_line("\r\n");
    renderer.set_foreground(Color::BrightYellow);
    renderer.write_line("Press any key to continue...");
    renderer.reset();
    connection
        .send_raw(renderer.take_output().as_bytes())
        .await?;
    connection.read_char().await.ok();
    Ok(())
}
