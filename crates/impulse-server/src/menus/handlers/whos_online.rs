//! Who's online handler

use anyhow::Result;
use impulse_session::SessionManager;
use impulse_telnet::TelnetConnection;
use impulse_terminal::{AnsiRenderer, Color};

/// Handle who's online
pub async fn handle_whos_online(
    connection: &mut TelnetConnection,
    session_manager: &SessionManager,
    renderer: &mut AnsiRenderer,
) -> Result<()> {
    renderer.clear_screen();
    renderer.set_foreground(Color::BrightCyan);
    renderer
        .write_line("╔══════════════════════════════════════════════════════════════════════════╗");
    renderer
        .write_line("║                         WHO'S ONLINE                                     ║");
    renderer
        .write_line("╚══════════════════════════════════════════════════════════════════════════╝");
    renderer.reset();
    renderer.write_line("");

    // Get all active sessions
    let sessions = session_manager.list_all_sessions().await;

    renderer.set_foreground(Color::BrightWhite);
    renderer.write_line(&format!("Total users online: {}", sessions.len()));
    renderer.reset();
    renderer.write_line("");

    if !sessions.is_empty() {
        renderer.set_foreground(Color::BrightYellow);
        renderer.write_line(&format!(
            "{:20} {:20} {:15} {:20}",
            "Username", "Location", "State", "Connected"
        ));
        renderer.write_line(&"-".repeat(76));
        renderer.reset();

        for session in sessions {
            let username = session.username().unwrap_or("<authenticating>");
            let location = session.remote_addr();
            let state = format!("{:?}", session.state());
            // Show idle time as duration
            let idle = session.idle_time();
            let idle_mins = idle.as_secs() / 60;

            renderer.set_foreground(Color::BrightGreen);
            renderer.write_text(&format!("{:20} ", username));
            renderer.reset();
            renderer.write_text(&format!("{:20} ", location));
            renderer.set_foreground(Color::Yellow);
            renderer.write_text(&format!("{:15} ", state));
            renderer.reset();
            renderer.write_line(&format!("idle {}m", idle_mins));
        }
    } else {
        renderer.set_foreground(Color::Yellow);
        renderer.write_line("No users currently online.");
        renderer.reset();
    }

    renderer.write_line("");
    renderer.set_foreground(Color::Yellow);
    renderer.write_line("Session Management:");
    renderer.write_line("  • Real-time activity tracking");
    renderer.write_line("  • Idle timeout detection");
    renderer.write_line("  • Privacy controls");
    renderer.write_line("  • User directory search");
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
