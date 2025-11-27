//! Authentication flow for BBS connections
//!
//! Handles login, registration, and authentication UI.

use crate::state::ServerState;
use anyhow::Result;
use impulse_auth::SessionToken;
use impulse_telnet::TelnetConnection;
use impulse_terminal::{AnsiRenderer, Color};
use impulse_types::user::User;
use impulse_user::UserManager;
use tracing::{info, warn};

/// Authentication result
pub enum AuthResult {
    /// User authenticated successfully (User is boxed to reduce enum size variance)
    Authenticated {
        user: Box<User>,
        token: SessionToken,
    },

    /// User quit during authentication
    Quit,
}

/// Handle authentication flow (login or new user registration)
pub async fn authenticate(
    connection: &mut TelnetConnection,
    state: &ServerState,
) -> Result<AuthResult> {
    let mut renderer = AnsiRenderer::new();

    loop {
        // Display welcome screen
        renderer.clear_screen();
        display_welcome_screen(&mut renderer);
        connection
            .send_raw(renderer.take_output().as_bytes())
            .await?;

        // Display login menu
        renderer.clear();
        renderer.write_line("");
        renderer.set_foreground(Color::BrightCyan);
        renderer.write_line("╔════════════════════════════════════╗");
        renderer.write_line("║           LOGIN MENU               ║");
        renderer.write_line("╠════════════════════════════════════╣");
        renderer.reset();
        renderer.write_line("║  [L] Login                         ║");
        renderer.write_line("║  [N] New User Registration         ║");
        renderer.write_line("║  [Q] Quit                          ║");
        renderer.set_foreground(Color::BrightCyan);
        renderer.write_line("╚════════════════════════════════════╝");
        renderer.reset();
        renderer.write_line("");
        renderer.set_foreground(Color::BrightYellow);
        renderer.write_text("Choice: ");
        renderer.reset();
        connection
            .send_raw(renderer.take_output().as_bytes())
            .await?;

        // Read choice
        match connection.read_char().await {
            Ok(ch) => {
                let choice = ch.to_ascii_uppercase();
                renderer.clear();

                match choice {
                    'L' => {
                        // Login flow
                        if let Some(result) = handle_login(connection, state, &mut renderer).await?
                        {
                            return Ok(result);
                        }
                        // If login failed or cancelled, loop back to menu
                    }
                    'N' => {
                        // Registration flow
                        renderer.write_line("\r\n");
                        renderer.set_foreground(Color::BrightRed);
                        renderer.write_line("New user registration is not yet implemented.");
                        renderer.write_line("Please contact the SysOp for an account.");
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
                    'Q' => {
                        // Quit
                        renderer.write_line("\r\n");
                        renderer.set_foreground(Color::BrightYellow);
                        renderer.write_line("Goodbye! Come back soon!");
                        renderer.reset();
                        renderer.write_line("\r\n");
                        connection
                            .send_raw(renderer.take_output().as_bytes())
                            .await?;
                        return Ok(AuthResult::Quit);
                    }
                    _ => {
                        // Invalid choice
                        renderer.write_line("\r\n");
                        renderer.set_foreground(Color::BrightRed);
                        renderer.write_line("Invalid choice. Please try again.");
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
                warn!("Error reading authentication choice: {}", e);
                return Ok(AuthResult::Quit);
            }
        }
    }
}

/// Handle login flow
async fn handle_login(
    connection: &mut TelnetConnection,
    state: &ServerState,
    renderer: &mut AnsiRenderer,
) -> Result<Option<AuthResult>> {
    // Prompt for username
    renderer.write_line("\r\n");
    renderer.set_foreground(Color::BrightWhite);
    renderer.write_line("=== LOGIN ===");
    renderer.reset();
    renderer.write_line("");
    renderer.set_foreground(Color::BrightGreen);
    renderer.write_text("Username: ");
    renderer.reset();
    connection
        .send_raw(renderer.take_output().as_bytes())
        .await?;

    let username = match connection.read_line().await {
        Ok(line) => line.trim().to_string(),
        Err(_) => return Ok(Some(AuthResult::Quit)),
    };

    if username.is_empty() {
        renderer.clear();
        renderer.write_line("\r\n");
        renderer.set_foreground(Color::BrightRed);
        renderer.write_line("Login cancelled.");
        renderer.reset();
        renderer.write_line("\r\n");
        renderer.set_foreground(Color::BrightYellow);
        renderer.write_line("Press any key to continue...");
        renderer.reset();
        connection
            .send_raw(renderer.take_output().as_bytes())
            .await?;
        connection.read_char().await.ok();
        return Ok(None);
    }

    // Prompt for password (add newline after username input)
    renderer.clear();
    renderer.write_line(""); // Add newline to move to next line
    renderer.set_foreground(Color::BrightGreen);
    renderer.write_text("Password: ");
    renderer.reset();
    connection
        .send_raw(renderer.take_output().as_bytes())
        .await?;

    // Read password securely (without echoing)
    let _password = match connection.read_password(true).await {
        Ok(line) => line.trim().to_string(),
        Err(_) => return Ok(Some(AuthResult::Quit)),
    };

    renderer.clear();

    // Look up user
    let user_manager = state.user_manager.read().await;
    let user_opt = user_manager.find_by_username(&username).await?;

    match user_opt {
        Some(user) => {
            // For demo purposes, accept any password for existing users
            // In production, use state.auth_service.login(&user, &password, &stored_hash).await

            // Create mock hash for demonstration
            let hash = state.auth_service.hash_password("demo123").unwrap();

            // Authenticate with demo password
            match state.auth_service.login(&user, "demo123", &hash).await {
                Ok(token) => {
                    info!(
                        username = %username,
                        user_id = ?user.id(),
                        "User logged in successfully"
                    );

                    renderer.write_line("\r\n");
                    renderer.set_foreground(Color::BrightGreen);
                    renderer.write_line(&format!("Login successful! Welcome back, {}!", username));
                    renderer.reset();
                    renderer.write_line("");
                    renderer.set_foreground(Color::Yellow);
                    renderer.write_line(&format!(
                        "Security Level: {}",
                        user.security_level().value()
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

                    Ok(Some(AuthResult::Authenticated {
                        user: Box::new(user),
                        token,
                    }))
                }
                Err(e) => {
                    warn!(username = %username, error = %e, "Login failed");

                    renderer.write_line("\r\n");
                    renderer.set_foreground(Color::BrightRed);
                    renderer.write_line("Login failed: Invalid credentials");
                    renderer.reset();
                    renderer.write_line("\r\n");
                    renderer.set_foreground(Color::BrightYellow);
                    renderer.write_line("Press any key to continue...");
                    renderer.reset();
                    connection
                        .send_raw(renderer.take_output().as_bytes())
                        .await?;
                    connection.read_char().await.ok();

                    Ok(None)
                }
            }
        }
        None => {
            warn!(username = %username, "Login failed: user not found");

            renderer.write_line("\r\n");
            renderer.set_foreground(Color::BrightRed);
            renderer.write_line("Login failed: User not found");
            renderer.reset();
            renderer.write_line("\r\n");
            renderer.set_foreground(Color::BrightYellow);
            renderer.write_line("Press any key to continue...");
            renderer.reset();
            connection
                .send_raw(renderer.take_output().as_bytes())
                .await?;
            connection.read_char().await.ok();

            Ok(None)
        }
    }
}

/// Display welcome screen
fn display_welcome_screen(renderer: &mut AnsiRenderer) {
    renderer.clear_screen();

    // ANSI Art Banner
    renderer.set_foreground(Color::BrightCyan);
    renderer.write_line("  ___                 _           ____  ____  ____  ");
    renderer.write_line(" |_ _|_ __ ___  _ __ | |__   ___ | __ )| __ )/ ___| ");
    renderer.write_line("  | || '_ ` _ \\| '_ \\| '_ \\ / _ \\|  _ \\|  _ \\\\___ \\ ");
    renderer.write_line("  | || | | | | | |_) | | | |  __/| |_) | |_) |___) |");
    renderer.write_line(" |___|_| |_| |_| .__/|_| |_|\\___||____/|____/|____/ ");
    renderer.write_line("               |_|                                   ");
    renderer.reset();
    renderer.write_line("");

    renderer.set_foreground(Color::BrightWhite);
    renderer.write_line("              Welcome to Impulse BBS");
    renderer.reset();
    renderer.set_foreground(Color::BrightGreen);
    renderer.write_line("                    Version 7.1");
    renderer.reset();
    renderer.write_line("");

    renderer.set_foreground(Color::Yellow);
    renderer.write_line("  A Modern Bulletin Board System");
    renderer.write_line("  Powered by Rust - Fast, Safe, Reliable");
    renderer.reset();
    renderer.write_line("");

    renderer.set_foreground(Color::BrightBlue);
    renderer.write_line("  ===========================================");
    renderer.reset();
    renderer.write_line("");
}
