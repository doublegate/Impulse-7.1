//! Door games handler

use crate::state::ServerState;
use anyhow::Result;
use impulse_telnet::TelnetConnection;
use impulse_terminal::{AnsiRenderer, Color};
use impulse_types::user::User;

/// Handle doors menu
pub async fn handle_doors(
    connection: &mut TelnetConnection,
    _user: &User,
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
            renderer.write_line("  • DOOR.SYS dropfile format");
            renderer.write_line("  • DORINFO1.DEF dropfile format");
            renderer.write_line("  • DOSBox integration for DOS games");
            renderer.write_line("  • Async I/O for seamless gameplay");
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

        for (idx, door) in doors.iter().enumerate() {
            renderer.set_foreground(Color::BrightYellow);
            renderer.write_text(&format!("{}. ", idx + 1));
            renderer.reset();
            renderer.set_foreground(Color::BrightWhite);
            renderer.write_text(&format!("{:25} ", door.name));
            renderer.reset();
            renderer.write_line(&format!("- {}", door.description));
        }

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
                    renderer.clear();
                    renderer.write_line("\r\n");
                    renderer.set_foreground(Color::BrightGreen);
                    renderer.write_line(&format!("Launching door: {}", door.name));
                    renderer.write_line("Door executor ready with dropfile generation.");
                    renderer.write_line("Game session would start here with full I/O handling.");
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
            Err(_) => {
                return Ok(());
            }
        }
    }
}
