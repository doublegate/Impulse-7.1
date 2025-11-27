//! User profile handler

use crate::state::ServerState;
use anyhow::Result;
use impulse_telnet::TelnetConnection;
use impulse_terminal::{AnsiRenderer, Color};
use impulse_types::user::User;

/// Handle user profile
pub async fn handle_user_profile(
    connection: &mut TelnetConnection,
    user: &User,
    _state: &ServerState,
    renderer: &mut AnsiRenderer,
) -> Result<()> {
    renderer.clear_screen();
    renderer.set_foreground(Color::BrightCyan);
    renderer
        .write_line("╔══════════════════════════════════════════════════════════════════════════╗");
    renderer
        .write_line("║                          USER PROFILE                                    ║");
    renderer
        .write_line("╚══════════════════════════════════════════════════════════════════════════╝");
    renderer.reset();
    renderer.write_line("");

    // Display user profile information
    renderer.set_foreground(Color::BrightWhite);
    renderer.write_line("Personal Information:");
    renderer.reset();
    renderer.write_line(&format!("  Username:       {}", user.username()));
    renderer.write_line(&format!("  User ID:        {:?}", user.id()));
    renderer.write_line(&format!(
        "  Security Level: {}",
        user.security_level().value()
    ));
    renderer.write_line(&format!(
        "  Status:         {}",
        if user.is_active { "Active" } else { "Inactive" }
    ));

    // Display additional fields if available
    if let Some(ref real_name) = user.real_name {
        renderer.write_line(&format!("  Real Name:      {}", real_name));
    }
    if let Some(ref email) = user.email {
        renderer.write_line(&format!("  Email:          {}", email));
    }
    renderer.write_line("");

    renderer.write_line("");
    renderer.set_foreground(Color::Yellow);
    renderer.write_line("Profile Features:");
    renderer.write_line("  • User statistics tracking");
    renderer.write_line("  • Achievement system");
    renderer.write_line("  • Privacy controls");
    renderer.write_line("  • Settings editor");
    renderer.reset();
    renderer.write_line("");

    renderer.set_foreground(Color::BrightGreen);
    renderer.write_line("Full profile editor coming soon!");
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
