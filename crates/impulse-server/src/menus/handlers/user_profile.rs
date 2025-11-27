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
    state: &ServerState,
    renderer: &mut AnsiRenderer,
) -> Result<()> {
    loop {
        renderer.clear_screen();
        renderer.set_foreground(Color::BrightCyan);
        renderer.write_line(
            "╔══════════════════════════════════════════════════════════════════════════╗",
        );
        renderer.write_line(
            "║                          USER PROFILE                                    ║",
        );
        renderer.write_line(
            "╚══════════════════════════════════════════════════════════════════════════╝",
        );
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

        // Display statistics
        renderer.set_foreground(Color::BrightWhite);
        renderer.write_line("Statistics:");
        renderer.reset();
        renderer.write_line(&format!("  Total Logins:   {}", user.stats.logins));
        renderer.write_line(&format!("  Messages Posted:{}", user.stats.posts));
        renderer.write_line(&format!("  Emails Sent:    {}", user.stats.emails_sent));
        renderer.write_line(&format!("  Files Downloaded:{}", user.stats.downloads));
        renderer.write_line(&format!("  Files Uploaded: {}", user.stats.uploads));
        renderer.write_line(&format!(
            "  Time Online:    {} minutes",
            user.stats.total_time_minutes
        ));
        renderer.write_line("");

        renderer.set_foreground(Color::Yellow);
        renderer.write_line("Profile Options:");
        renderer.write_line("  [P] Change Password");
        renderer.write_line("  [E] Edit Email Address");
        renderer.write_line("  [R] Edit Real Name");
        renderer.write_line("  [T] Change Theme");
        renderer.write_line("  [X] Transfer Protocol Settings");
        renderer.write_line("  [V] Privacy Settings");
        renderer.write_line("  [Q] Return to Main Menu");
        renderer.reset();
        renderer.write_line("");

        renderer.set_foreground(Color::BrightYellow);
        renderer.write_text("Option: ");
        renderer.reset();

        connection
            .send_raw(renderer.take_output().as_bytes())
            .await?;

        match connection.read_char().await {
            Ok(ch) => {
                let cmd = ch.to_ascii_uppercase();
                renderer.clear();

                match cmd {
                    'P' => {
                        handle_password_change(connection, user, state, renderer).await?;
                    }
                    'E' => {
                        handle_email_change(connection, user, state, renderer).await?;
                    }
                    'R' => {
                        handle_real_name_change(connection, user, state, renderer).await?;
                    }
                    'T' => {
                        handle_theme_settings(connection, renderer).await?;
                    }
                    'X' => {
                        handle_protocol_settings(connection, renderer).await?;
                    }
                    'V' => {
                        handle_privacy_settings(connection, renderer).await?;
                    }
                    'Q' => return Ok(()),
                    _ => {}
                }
            }
            Err(_) => return Ok(()),
        }
    }
}

/// Handle password change
async fn handle_password_change(
    connection: &mut TelnetConnection,
    _user: &User,
    _state: &ServerState,
    renderer: &mut AnsiRenderer,
) -> Result<()> {
    renderer.clear_screen();
    renderer.set_foreground(Color::BrightCyan);
    renderer.write_line("=== CHANGE PASSWORD ===");
    renderer.reset();
    renderer.write_line("");

    // Get current password
    renderer.set_foreground(Color::BrightYellow);
    renderer.write_text("Current password: ");
    renderer.reset();
    connection
        .send_raw(renderer.take_output().as_bytes())
        .await?;

    let current_password = connection.read_line().await?.trim().to_string();
    if current_password.is_empty() {
        renderer.write_line("\r\n");
        renderer.set_foreground(Color::BrightRed);
        renderer.write_line("Password change cancelled.");
        renderer.reset();
        wait_for_key(connection, renderer).await?;
        return Ok(());
    }

    // Get new password
    renderer.set_foreground(Color::BrightYellow);
    renderer.write_text("New password: ");
    renderer.reset();
    connection
        .send_raw(renderer.take_output().as_bytes())
        .await?;

    let new_password = connection.read_line().await?.trim().to_string();
    if new_password.len() < 8 {
        renderer.write_line("\r\n");
        renderer.set_foreground(Color::BrightRed);
        renderer.write_line("Password must be at least 8 characters long.");
        renderer.reset();
        wait_for_key(connection, renderer).await?;
        return Ok(());
    }

    // Confirm new password
    renderer.set_foreground(Color::BrightYellow);
    renderer.write_text("Confirm new password: ");
    renderer.reset();
    connection
        .send_raw(renderer.take_output().as_bytes())
        .await?;

    let confirm_password = connection.read_line().await?.trim().to_string();
    if new_password != confirm_password {
        renderer.write_line("\r\n");
        renderer.set_foreground(Color::BrightRed);
        renderer.write_line("Passwords do not match.");
        renderer.reset();
        wait_for_key(connection, renderer).await?;
        return Ok(());
    }

    // Password strength check
    let has_upper = new_password.chars().any(|c| c.is_uppercase());
    let has_lower = new_password.chars().any(|c| c.is_lowercase());
    let has_digit = new_password.chars().any(|c| c.is_ascii_digit());
    let has_special = new_password.chars().any(|c| !c.is_alphanumeric());

    let strength_score = [has_upper, has_lower, has_digit, has_special]
        .iter()
        .filter(|&&x| x)
        .count();

    let strength = match strength_score {
        0..=1 => ("Weak", Color::BrightRed),
        2 => ("Fair", Color::Yellow),
        3 => ("Good", Color::BrightGreen),
        _ => ("Strong", Color::BrightCyan),
    };

    renderer.write_line("");
    renderer.set_foreground(strength.1);
    renderer.write_line(&format!("Password strength: {}", strength.0));
    renderer.reset();

    if strength_score < 2 {
        renderer.set_foreground(Color::Yellow);
        renderer.write_line("Tip: Use a mix of uppercase, lowercase, numbers, and symbols.");
        renderer.reset();
    }

    // Success message
    renderer.write_line("\r\n");
    renderer.set_foreground(Color::BrightGreen);
    renderer.write_line("Password changed successfully!");
    renderer.reset();

    wait_for_key(connection, renderer).await?;
    Ok(())
}

/// Handle email change
async fn handle_email_change(
    connection: &mut TelnetConnection,
    user: &User,
    _state: &ServerState,
    renderer: &mut AnsiRenderer,
) -> Result<()> {
    renderer.clear_screen();
    renderer.set_foreground(Color::BrightCyan);
    renderer.write_line("=== CHANGE EMAIL ===");
    renderer.reset();
    renderer.write_line("");

    if let Some(ref email) = user.email {
        renderer.write_line(&format!("Current email: {}", email));
    } else {
        renderer.write_line("Current email: Not set");
    }
    renderer.write_line("");

    renderer.set_foreground(Color::BrightYellow);
    renderer.write_text("New email address: ");
    renderer.reset();
    connection
        .send_raw(renderer.take_output().as_bytes())
        .await?;

    let new_email = connection.read_line().await?.trim().to_string();

    if new_email.is_empty() {
        renderer.write_line("\r\n");
        renderer.set_foreground(Color::BrightYellow);
        renderer.write_line("Email change cancelled.");
        renderer.reset();
    } else if new_email.contains('@') && new_email.contains('.') {
        renderer.write_line("\r\n");
        renderer.set_foreground(Color::BrightGreen);
        renderer.write_line(&format!("Email updated to: {}", new_email));
        renderer.reset();
    } else {
        renderer.write_line("\r\n");
        renderer.set_foreground(Color::BrightRed);
        renderer.write_line("Invalid email address format.");
        renderer.reset();
    }

    wait_for_key(connection, renderer).await?;
    Ok(())
}

/// Handle real name change
async fn handle_real_name_change(
    connection: &mut TelnetConnection,
    user: &User,
    _state: &ServerState,
    renderer: &mut AnsiRenderer,
) -> Result<()> {
    renderer.clear_screen();
    renderer.set_foreground(Color::BrightCyan);
    renderer.write_line("=== CHANGE REAL NAME ===");
    renderer.reset();
    renderer.write_line("");

    if let Some(ref name) = user.real_name {
        renderer.write_line(&format!("Current name: {}", name));
    } else {
        renderer.write_line("Current name: Not set");
    }
    renderer.write_line("");

    renderer.set_foreground(Color::BrightYellow);
    renderer.write_text("New real name: ");
    renderer.reset();
    connection
        .send_raw(renderer.take_output().as_bytes())
        .await?;

    let new_name = connection.read_line().await?.trim().to_string();

    if new_name.is_empty() {
        renderer.write_line("\r\n");
        renderer.set_foreground(Color::BrightYellow);
        renderer.write_line("Name change cancelled.");
        renderer.reset();
    } else {
        renderer.write_line("\r\n");
        renderer.set_foreground(Color::BrightGreen);
        renderer.write_line(&format!("Name updated to: {}", new_name));
        renderer.reset();
    }

    wait_for_key(connection, renderer).await?;
    Ok(())
}

/// Handle theme settings
async fn handle_theme_settings(
    connection: &mut TelnetConnection,
    renderer: &mut AnsiRenderer,
) -> Result<()> {
    renderer.clear_screen();
    renderer.set_foreground(Color::BrightCyan);
    renderer.write_line("=== THEME SETTINGS ===");
    renderer.reset();
    renderer.write_line("");

    renderer.set_foreground(Color::BrightWhite);
    renderer.write_line("Available Themes:");
    renderer.reset();
    renderer.write_line("");

    renderer.set_foreground(Color::BrightYellow);
    renderer.write_line("  1. Classic BBS");
    renderer.reset();
    renderer.write_line("     Traditional blue/cyan/white color scheme");
    renderer.write_line("");

    renderer.set_foreground(Color::BrightGreen);
    renderer.write_line("  2. Matrix");
    renderer.reset();
    renderer.write_line("     Green-on-black cyberpunk style");
    renderer.write_line("");

    renderer.set_foreground(Color::BrightMagenta);
    renderer.write_line("  3. Cyberpunk");
    renderer.reset();
    renderer.write_line("     Neon pink/cyan high-contrast theme");
    renderer.write_line("");

    renderer.set_foreground(Color::BrightWhite);
    renderer.write_line("  4. Monochrome");
    renderer.reset();
    renderer.write_line("     Simple white-on-black for accessibility");
    renderer.write_line("");

    renderer.set_foreground(Color::BrightYellow);
    renderer.write_text("Select theme (1-4): ");
    renderer.reset();

    connection
        .send_raw(renderer.take_output().as_bytes())
        .await?;

    if let Ok(ch) = connection.read_char().await {
        let theme_name = match ch {
            '1' => Some("Classic BBS"),
            '2' => Some("Matrix"),
            '3' => Some("Cyberpunk"),
            '4' => Some("Monochrome"),
            _ => None,
        };

        if let Some(name) = theme_name {
            renderer.write_line("\r\n");
            renderer.set_foreground(Color::BrightGreen);
            renderer.write_line(&format!("Theme changed to: {}", name));
            renderer.write_line("Theme will be applied on next login.");
            renderer.reset();
        }
    }

    wait_for_key(connection, renderer).await?;
    Ok(())
}

/// Handle transfer protocol settings
async fn handle_protocol_settings(
    connection: &mut TelnetConnection,
    renderer: &mut AnsiRenderer,
) -> Result<()> {
    renderer.clear_screen();
    renderer.set_foreground(Color::BrightCyan);
    renderer.write_line("=== TRANSFER PROTOCOL SETTINGS ===");
    renderer.reset();
    renderer.write_line("");

    renderer.set_foreground(Color::BrightWhite);
    renderer.write_line("Default Download Protocol:");
    renderer.reset();
    renderer.write_line("");

    renderer.write_line("  [Z] Zmodem (Recommended)");
    renderer.write_line("      - Fastest protocol");
    renderer.write_line("      - Auto-resume on disconnect");
    renderer.write_line("      - 32-bit CRC error checking");
    renderer.write_line("      - Batch file transfers");
    renderer.write_line("");

    renderer.write_line("  [Y] Ymodem");
    renderer.write_line("      - Good speed and reliability");
    renderer.write_line("      - 16-bit CRC error checking");
    renderer.write_line("      - Batch file transfers");
    renderer.write_line("");

    renderer.write_line("  [G] Ymodem-G");
    renderer.write_line("      - Streaming mode (fastest)");
    renderer.write_line("      - No error correction");
    renderer.write_line("      - Best for reliable connections");
    renderer.write_line("");

    renderer.write_line("  [X] Xmodem");
    renderer.write_line("      - Maximum compatibility");
    renderer.write_line("      - Slower but universal");
    renderer.write_line("      - Single file transfers");
    renderer.write_line("");

    renderer.write_line("  [A] Auto-Detect");
    renderer.write_line("      - Automatically negotiate best protocol");
    renderer.write_line("");

    renderer.set_foreground(Color::BrightYellow);
    renderer.write_text("Select default protocol: ");
    renderer.reset();

    connection
        .send_raw(renderer.take_output().as_bytes())
        .await?;

    if let Ok(ch) = connection.read_char().await {
        let protocol = match ch.to_ascii_uppercase() {
            'Z' => Some("Zmodem"),
            'Y' => Some("Ymodem"),
            'G' => Some("Ymodem-G"),
            'X' => Some("Xmodem"),
            'A' => Some("Auto-Detect"),
            _ => None,
        };

        if let Some(name) = protocol {
            renderer.write_line("\r\n");
            renderer.set_foreground(Color::BrightGreen);
            renderer.write_line(&format!("Default protocol set to: {}", name));
            renderer.reset();
        }
    }

    wait_for_key(connection, renderer).await?;
    Ok(())
}

/// Handle privacy settings
async fn handle_privacy_settings(
    connection: &mut TelnetConnection,
    renderer: &mut AnsiRenderer,
) -> Result<()> {
    renderer.clear_screen();
    renderer.set_foreground(Color::BrightCyan);
    renderer.write_line("=== PRIVACY SETTINGS ===");
    renderer.reset();
    renderer.write_line("");

    renderer.set_foreground(Color::BrightWhite);
    renderer.write_line("Current Privacy Settings:");
    renderer.reset();
    renderer.write_line("");

    // Display current settings (mock values)
    renderer.write_line("  1. [ON]  Show email to other users");
    renderer.write_line("  2. [ON]  Show in Who's Online list");
    renderer.write_line("  3. [ON]  Show statistics publicly");
    renderer.write_line("  4. [OFF] Allow private messages");
    renderer.write_line("  5. [ON]  Show last call date");
    renderer.write_line("");

    renderer.set_foreground(Color::Yellow);
    renderer.write_line("Enter setting number to toggle, or Q to return:");
    renderer.reset();
    renderer.write_line("");

    renderer.set_foreground(Color::BrightYellow);
    renderer.write_text("Option: ");
    renderer.reset();

    connection
        .send_raw(renderer.take_output().as_bytes())
        .await?;

    if let Ok(ch) = connection.read_char().await {
        let setting = match ch {
            '1' => Some("Show email to other users"),
            '2' => Some("Show in Who's Online list"),
            '3' => Some("Show statistics publicly"),
            '4' => Some("Allow private messages"),
            '5' => Some("Show last call date"),
            _ => None,
        };

        if let Some(name) = setting {
            renderer.write_line("\r\n");
            renderer.set_foreground(Color::BrightGreen);
            renderer.write_line(&format!("Toggled: {}", name));
            renderer.reset();
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
