//! Door games handler

use crate::state::ServerState;
use anyhow::Result;
use chrono::Utc;
use impulse_door::{DoorExecutor, DoorSession};
use impulse_telnet::TelnetConnection;
use impulse_terminal::{AnsiRenderer, Color};
use impulse_types::user::User;

/// Handle doors menu
pub async fn handle_doors(
    connection: &mut TelnetConnection,
    user: &User,
    state: &ServerState,
    renderer: &mut AnsiRenderer,
) -> Result<()> {
    // Get available doors
    let doors = state.door_manager.list_doors();

    loop {
        renderer.clear_screen();
        renderer.set_foreground(Color::BrightCyan);
        renderer.write_line(
            "╔══════════════════════════════════════════════════════════════════════════╗",
        );
        renderer.write_line(
            "║                          DOOR GAMES                                      ║",
        );
        renderer.write_line(
            "╚══════════════════════════════════════════════════════════════════════════╝",
        );
        renderer.reset();
        renderer.write_line("");

        if doors.is_empty() {
            renderer.set_foreground(Color::BrightYellow);
            renderer.write_line("No door games are currently configured.");
            renderer.write_line("Contact the SysOp to add door games.");
            renderer.reset();
            renderer.write_line("");
            renderer.set_foreground(Color::Yellow);
            renderer.write_line("Door games support:");
            renderer.write_line("  - DOOR.SYS dropfile format");
            renderer.write_line("  - DORINFO1.DEF dropfile format");
            renderer.write_line("  - DOSBox integration for DOS games");
            renderer.write_line("  - Async I/O for seamless gameplay");
            renderer.reset();
            renderer.write_line("\r\n");
            renderer.set_foreground(Color::BrightYellow);
            renderer.write_line("Press any key to continue...");
            renderer.reset();
            connection
                .send_raw(renderer.take_output().as_bytes())
                .await?;
            connection.read_char().await.ok();
            return Ok(());
        }

        renderer.set_foreground(Color::BrightWhite);
        renderer.write_line("Available Door Games:");
        renderer.reset();
        renderer.write_line("");

        renderer.set_foreground(Color::BrightWhite);
        renderer.write_line("  #  Name                      Description               Min Sec");
        renderer.write_line("──────────────────────────────────────────────────────────────────");
        renderer.reset();

        for (idx, door) in doors.iter().enumerate() {
            renderer.set_foreground(Color::BrightYellow);
            renderer.write_text(&format!("{:3}. ", idx + 1));
            renderer.reset();
            renderer.set_foreground(Color::BrightWhite);
            renderer.write_text(&format!("{:25} ", door.name));
            renderer.reset();
            renderer.set_foreground(Color::Cyan);
            // Truncate description to fit
            let desc = if door.description.len() > 25 {
                format!("{}...", &door.description[..22])
            } else {
                format!("{:25}", door.description)
            };
            renderer.write_text(&desc);
            renderer.reset();
            renderer.set_foreground(Color::BrightCyan);
            renderer.write_line(&format!("  {:3}", door.min_security_level));
            renderer.reset();
        }

        renderer.write_line("");
        renderer.set_foreground(Color::Yellow);
        renderer.write_line(&format!(
            "Your security level: {}  |  Time remaining: {} minutes",
            user.security_level().value(),
            60 // Placeholder - would come from session
        ));
        renderer.reset();
        renderer.write_line("");

        renderer.set_foreground(Color::BrightYellow);
        renderer.write_text(&format!("Select door (1-{}) or [Q] to quit: ", doors.len()));
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
                }

                if let Ok(selection) = input.parse::<usize>()
                    && selection > 0
                    && selection <= doors.len()
                {
                    let door = &doors[selection - 1];

                    // Check security level
                    if user.security_level().value() < door.min_security_level {
                        renderer.clear();
                        renderer.write_line("\r\n");
                        renderer.set_foreground(Color::BrightRed);
                        renderer.write_line(&format!(
                            "Access denied! This door requires security level {}.",
                            door.min_security_level
                        ));
                        renderer.write_line(&format!(
                            "Your security level is {}.",
                            user.security_level().value()
                        ));
                        renderer.reset();
                        wait_for_key(connection, renderer).await?;
                        continue;
                    }

                    // Launch the door
                    execute_door(connection, user, state, renderer, &door.name).await?;
                }
            }
            Err(_) => {
                return Ok(());
            }
        }
    }
}

/// Execute a door game
async fn execute_door(
    connection: &mut TelnetConnection,
    user: &User,
    state: &ServerState,
    renderer: &mut AnsiRenderer,
    door_name: &str,
) -> Result<()> {
    renderer.clear_screen();
    renderer.set_foreground(Color::BrightCyan);
    renderer.write_line("=== DOOR GAME LAUNCHER ===");
    renderer.reset();
    renderer.write_line("");

    // Get door info
    let door = match state.door_manager.get_door(door_name) {
        Some(d) => d,
        None => {
            renderer.set_foreground(Color::BrightRed);
            renderer.write_line(&format!("Door '{}' not found.", door_name));
            renderer.reset();
            wait_for_key(connection, renderer).await?;
            return Ok(());
        }
    };

    renderer.set_foreground(Color::BrightGreen);
    renderer.write_line(&format!("Launching: {}", door.name));
    renderer.write_line(&format!("Description: {}", door.description));
    renderer.reset();
    renderer.write_line("");

    renderer.set_foreground(Color::BrightYellow);
    renderer.write_line("Preparing door environment...");
    renderer.reset();
    connection
        .send_raw(renderer.take_output().as_bytes())
        .await?;

    // Create door session
    let mut door_session = DoorSession {
        node_id: 1, // Would come from actual session
        user_name: user
            .real_name
            .clone()
            .unwrap_or_else(|| user.username().to_string()),
        user_alias: Some(user.username().to_string()),
        location: "Online".to_string(),
        security_level: user.security_level().value(),
        time_remaining_seconds: 3600, // 60 minutes default
        ansi_enabled: true,
        login_time: Utc::now(),
        total_calls: user.stats.logins as u32,
        last_call_date: Utc::now().format("%m/%d/%y").to_string(),
        upload_kb: user.stats.upload_kb as u64,
        download_kb: user.stats.download_kb as u64,
    };

    // Create door executor
    let executor = DoorExecutor::new(state.door_manager.clone());

    renderer.write_line("  - Dropfile created");
    renderer.write_line("  - Environment configured");
    renderer.write_line("");
    renderer.set_foreground(Color::BrightCyan);
    renderer.write_line("Starting door game...");
    renderer.write_line("(Press ESC to exit the door at any time)");
    renderer.reset();
    renderer.write_line("");
    connection
        .send_raw(renderer.take_output().as_bytes())
        .await?;

    // Execute the door
    match executor.execute(door_name, &mut door_session).await {
        Ok(result) => {
            renderer.write_line("");
            renderer.set_foreground(Color::BrightGreen);
            renderer.write_line(&format!("Door exited with code: {}", result.exit_code));
            renderer.write_line(&format!(
                "Time spent in door: {} seconds",
                result.runtime_seconds
            ));
            if result.user_stats_updated {
                renderer.write_line("User statistics updated.");
            }
            renderer.reset();
        }
        Err(e) => {
            renderer.write_line("");
            renderer.set_foreground(Color::BrightRed);
            renderer.write_line(&format!("Door execution error: {}", e));
            renderer.reset();

            // Provide helpful error message
            let error_str = e.to_string();
            if error_str.contains("not found") {
                renderer.set_foreground(Color::Yellow);
                renderer.write_line("");
                renderer.write_line("The door executable was not found.");
                renderer.write_line("Please contact the SysOp to verify door configuration.");
                renderer.reset();
            } else if error_str.contains("DOSBox") {
                renderer.set_foreground(Color::Yellow);
                renderer.write_line("");
                renderer.write_line("This is a DOS door that requires DOSBox.");
                renderer.write_line("DOSBox may not be installed on this system.");
                renderer.reset();
            }
        }
    }

    wait_for_key(connection, renderer).await?;
    Ok(())
}

/// Helper to wait for key press
async fn wait_for_key(
    connection: &mut TelnetConnection,
    renderer: &mut AnsiRenderer,
) -> Result<()> {
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
