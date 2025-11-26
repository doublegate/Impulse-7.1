//! Impulse 7.1 BBS Server
//!
//! Modern BBS server implementation in Rust

use anyhow::Result;
use impulse_session::{SessionConfig, SessionManager};
use impulse_telnet::TelnetServer;
use impulse_terminal::{AnsiRenderer, Color};
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info, warn};

/// Server configuration
struct ServerConfig {
    telnet_address: String,
    session_idle_timeout: Duration,
    max_sessions: usize,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            telnet_address: "0.0.0.0:2323".to_string(),
            session_idle_timeout: Duration::from_secs(900), // 15 minutes
            max_sessions: 100,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing/logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    info!("Impulse 7.1 BBS Server v0.1.0");
    info!("=================================");

    // Load configuration
    let config = ServerConfig::default();
    info!("Configuration loaded");
    info!("  Telnet address: {}", config.telnet_address);
    info!("  Session timeout: {:?}", config.session_idle_timeout);
    info!("  Max sessions: {}", config.max_sessions);

    // Create session manager
    let session_config = SessionConfig::new()
        .with_idle_timeout(config.session_idle_timeout)
        .with_max_total_sessions(config.max_sessions);

    let session_manager = Arc::new(SessionManager::new(session_config));
    info!("Session manager initialized");

    // Spawn session cleanup task
    let _cleanup_handle = session_manager.spawn_cleanup_task();
    info!("Session cleanup task started");

    // Bind telnet server
    info!("Binding telnet server to {}...", config.telnet_address);
    let telnet_server = TelnetServer::bind(&config.telnet_address).await?;
    info!("Telnet server listening on {}", telnet_server.local_addr());

    info!("Server initialization complete - ready to accept connections");
    info!("Press Ctrl+C to stop the server");
    println!();

    // Main server loop
    loop {
        match telnet_server.accept().await {
            Ok(mut connection) => {
                let peer_addr = connection.peer_addr();
                info!("New connection from {}", peer_addr);

                // Create session
                let session_id = match session_manager.create_session(peer_addr.to_string()).await {
                    Ok(id) => {
                        info!(session_id = %id, "Session created for {}", peer_addr);
                        id
                    }
                    Err(e) => {
                        warn!("Failed to create session for {}: {}", peer_addr, e);
                        let _ = connection
                            .send_line("Server is full. Please try again later.")
                            .await;
                        continue;
                    }
                };

                // Spawn handler for this connection
                let session_mgr = session_manager.clone();
                tokio::spawn(async move {
                    if let Err(e) = handle_connection(connection, session_id, session_mgr).await {
                        error!(session_id = %session_id, "Connection error: {}", e);
                    }
                });
            }
            Err(e) => {
                error!("Failed to accept connection: {}", e);
            }
        }
    }
}

/// Handle a single BBS connection
async fn handle_connection(
    mut connection: impulse_telnet::TelnetConnection,
    session_id: impulse_session::SessionId,
    session_manager: Arc<SessionManager>,
) -> Result<()> {
    info!(session_id = %session_id, "Starting connection handler");

    // Initialize telnet session (negotiate options)
    connection.initialize().await?;

    // Create terminal renderer
    let mut renderer = AnsiRenderer::new();

    // Display welcome screen
    display_welcome(&mut renderer);
    connection
        .send_raw(renderer.take_output().as_bytes())
        .await?;

    // Display main prompt
    renderer.clear();
    renderer.write_line("");
    renderer.set_foreground(Color::BrightCyan);
    renderer.write_text("Impulse BBS");
    renderer.reset();
    renderer.write_line(" - Main Menu");
    renderer.write_line("");
    renderer.set_foreground(Color::BrightGreen);
    renderer.write_line("Commands:");
    renderer.reset();
    renderer.write_line("  H - Help");
    renderer.write_line("  S - System Statistics");
    renderer.write_line("  W - Who's Online");
    renderer.write_line("  Q - Quit");
    renderer.write_line("");
    renderer.set_foreground(Color::BrightYellow);
    renderer.write_text("Command: ");
    renderer.reset();
    connection
        .send_raw(renderer.take_output().as_bytes())
        .await?;

    // Simple command loop
    loop {
        // Update activity
        session_manager.update_activity(session_id).await.ok();

        // Read command
        match connection.read_char().await {
            Ok(ch) => {
                let cmd = ch.to_ascii_uppercase();
                renderer.clear();

                match cmd {
                    'H' => {
                        // Help
                        renderer.write_line("\r\n");
                        renderer.set_foreground(Color::BrightWhite);
                        renderer.write_line("=== HELP ===");
                        renderer.reset();
                        renderer.write_line("This is Impulse BBS - A modern bulletin board system");
                        renderer.write_line("Built with Rust for reliability and performance");
                        renderer.write_line("\r\n");
                        connection
                            .send_raw(renderer.take_output().as_bytes())
                            .await?;
                    }
                    'S' => {
                        // Statistics
                        let session_count = session_manager.active_session_count().await;
                        renderer.write_line("\r\n");
                        renderer.set_foreground(Color::BrightWhite);
                        renderer.write_line("=== SYSTEM STATISTICS ===");
                        renderer.reset();
                        renderer.write_line(&format!("Active Sessions: {}", session_count));
                        renderer.write_line("BBS Software: Impulse 7.1 (Rust Edition)");
                        renderer.write_line("Version: 0.1.0");
                        renderer.write_line("\r\n");
                        connection
                            .send_raw(renderer.take_output().as_bytes())
                            .await?;
                    }
                    'W' => {
                        // Who's online
                        let session_count = session_manager.active_session_count().await;
                        renderer.write_line("\r\n");
                        renderer.set_foreground(Color::BrightWhite);
                        renderer.write_line("=== WHO'S ONLINE ===");
                        renderer.reset();
                        renderer.write_line(&format!("Total users online: {}", session_count));
                        renderer.write_line("\r\n");
                        connection
                            .send_raw(renderer.take_output().as_bytes())
                            .await?;
                    }
                    'Q' => {
                        // Quit
                        renderer.write_line("\r\n");
                        renderer.set_foreground(Color::BrightYellow);
                        renderer.write_line("Thank you for visiting Impulse BBS!");
                        renderer.write_line("Come back soon!");
                        renderer.reset();
                        renderer.write_line("\r\n");
                        connection
                            .send_raw(renderer.take_output().as_bytes())
                            .await?;

                        // Terminate session
                        session_manager.terminate_session(session_id).await.ok();
                        connection.close().await.ok();
                        return Ok(());
                    }
                    _ => {
                        // Unknown command
                        renderer.write_line("\r\n");
                        renderer.set_foreground(Color::BrightRed);
                        renderer.write_line("Unknown command. Press H for help.");
                        renderer.reset();
                        renderer.write_line("\r\n");
                        connection
                            .send_raw(renderer.take_output().as_bytes())
                            .await?;
                    }
                }

                // Show prompt again
                renderer.clear();
                renderer.set_foreground(Color::BrightYellow);
                renderer.write_text("Command: ");
                renderer.reset();
                connection
                    .send_raw(renderer.take_output().as_bytes())
                    .await?;
            }
            Err(e) => {
                info!(session_id = %session_id, "Client disconnected: {}", e);
                session_manager.terminate_session(session_id).await.ok();
                return Ok(());
            }
        }
    }
}

/// Display welcome screen with ANSI art
fn display_welcome(renderer: &mut AnsiRenderer) {
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
