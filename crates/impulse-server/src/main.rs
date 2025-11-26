//! Impulse 7.1 BBS Server
//!
//! Modern BBS server implementation in Rust

mod auth;
mod menus;
mod state;

use anyhow::Result;
use auth::{authenticate, AuthResult};
use impulse_session::{SessionConfig, SessionManager};
use impulse_telnet::TelnetServer;
use menus::display_main_menu;
use state::ServerState;
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

    info!("Impulse 7.1 BBS Server v0.8.0");
    info!("=================================");

    // Load configuration
    let config = ServerConfig::default();
    info!("Configuration loaded");
    info!("  Telnet address: {}", config.telnet_address);
    info!("  Session timeout: {:?}", config.session_idle_timeout);
    info!("  Max sessions: {}", config.max_sessions);

    // Initialize server state (user managers, message bases, etc.)
    info!("Initializing server state...");
    let server_state = Arc::new(ServerState::new().await?);
    info!("Server state initialized");

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
    println!("Demo credentials:");
    println!("  Username: sysop   (security level 255)");
    println!("  Username: testuser (security level 10)");
    println!("  Password: demo123 (for any user)");
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
                let state = server_state.clone();
                tokio::spawn(async move {
                    if let Err(e) = handle_connection(connection, session_id, session_mgr, state).await {
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
    state: Arc<ServerState>,
) -> Result<()> {
    info!(session_id = %session_id, "Starting connection handler");

    // Initialize telnet session (negotiate options)
    connection.initialize().await?;

    // Authentication phase
    info!(session_id = %session_id, "Starting authentication");
    match authenticate(&mut connection, &state).await? {
        AuthResult::Authenticated { user, token } => {
            info!(
                session_id = %session_id,
                username = %user.username(),
                user_id = ?user.id(),
                "User authenticated successfully"
            );

            // Main menu loop
            loop {
                // Update activity
                session_manager.update_activity(session_id).await.ok();

                // Display main menu and handle commands
                match display_main_menu(&mut connection, &user, &token, &state, &session_manager)
                    .await
                {
                    Ok(should_continue) => {
                        if !should_continue {
                            // User logged out
                            break;
                        }
                    }
                    Err(e) => {
                        error!(
                            session_id = %session_id,
                            error = %e,
                            "Error in main menu"
                        );
                        break;
                    }
                }
            }

            // Logout
            state.auth_service.logout(&token).await;
            info!(
                session_id = %session_id,
                username = %user.username(),
                "User logged out"
            );
        }
        AuthResult::Quit => {
            info!(session_id = %session_id, "User quit during authentication");
        }
    }

    // Terminate session
    session_manager.terminate_session(session_id).await.ok();
    connection.close().await.ok();

    Ok(())
}
