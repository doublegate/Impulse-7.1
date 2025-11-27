//! Administration handler

use crate::state::ServerState;
use anyhow::Result;
use impulse_telnet::TelnetConnection;
use impulse_terminal::{AnsiRenderer, Color};
use impulse_types::user::User;
use impulse_user::UserManager;

/// Handle administration menu
pub async fn handle_admin(
    connection: &mut TelnetConnection,
    user: &User,
    state: &ServerState,
    renderer: &mut AnsiRenderer,
) -> Result<()> {
    // Verify admin access - create a temporary access control for the user
    let user_access = impulse_admin::AdminAccessControl::new(
        user.security_level().value(),
        200, // SysOp level
    );

    if !user_access.has_permission(impulse_admin::AdminPermission::ViewUsers) {
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
        return Ok(());
    }

    loop {
        renderer.clear_screen();
        renderer.set_foreground(Color::BrightMagenta);
        renderer.write_line(
            "╔══════════════════════════════════════════════════════════════════════════╗",
        );
        renderer.write_line(
            "║                      ADMINISTRATION PANEL                                ║",
        );
        renderer.write_line(
            "╚══════════════════════════════════════════════════════════════════════════╝",
        );
        renderer.reset();
        renderer.write_line("");

        renderer.set_foreground(Color::BrightWhite);
        renderer.write_line(&format!(
            "Administrator: {}  |  Security Level: {}",
            user.username(),
            user.security_level().value()
        ));
        renderer.reset();
        renderer.write_line("");

        renderer.set_foreground(Color::BrightYellow);
        renderer.write_line("Available Functions:");
        renderer.reset();
        renderer.write_line("");

        renderer.set_foreground(Color::BrightGreen);
        renderer.write_line("1. User Management");
        renderer.reset();
        renderer.write_line("   View, edit, and manage user accounts");
        renderer.write_line("");

        renderer.set_foreground(Color::BrightGreen);
        renderer.write_line("2. File Area Management");
        renderer.reset();
        renderer.write_line("   Create, edit, and configure file areas");
        renderer.write_line("");

        renderer.set_foreground(Color::BrightGreen);
        renderer.write_line("3. System Maintenance");
        renderer.reset();
        renderer.write_line("   View sessions, kick users, broadcast messages");
        renderer.write_line("");

        renderer.set_foreground(Color::BrightGreen);
        renderer.write_line("4. View Audit Log");
        renderer.reset();
        renderer.write_line("   Review administrative actions and system events");
        renderer.write_line("");

        renderer.set_foreground(Color::BrightGreen);
        renderer.write_line("5. Broadcast Message");
        renderer.reset();
        renderer.write_line("   Send announcement to all online users");
        renderer.write_line("");

        renderer.set_foreground(Color::BrightYellow);
        renderer.write_line("Q. Return to Main Menu");
        renderer.reset();
        renderer.write_line("");

        renderer.set_foreground(Color::Yellow);
        renderer.write_line("Administration Features:");
        renderer.write_line("  • Role-based access control");
        renderer.write_line("  • Comprehensive audit logging");
        renderer.write_line("  • User and file management");
        renderer.write_line("  • Session monitoring and control");
        renderer.reset();
        renderer.write_line("");

        renderer.set_foreground(Color::BrightYellow);
        renderer.write_text("Command (1-5, Q): ");
        renderer.reset();

        connection
            .send_raw(renderer.take_output().as_bytes())
            .await?;

        // Read command
        match connection.read_char().await {
            Ok(ch) => {
                let cmd = ch.to_ascii_uppercase();
                renderer.clear();

                match cmd {
                    '1' => {
                        show_user_management(connection, user, state, renderer).await?;
                    }
                    '2' => {
                        show_file_area_management(connection, renderer).await?;
                    }
                    '3' => {
                        show_system_maintenance(connection, renderer).await?;
                    }
                    '4' => {
                        show_audit_log(connection, user, state, renderer).await?;
                    }
                    '5' => {
                        show_broadcast(connection, renderer).await?;
                    }
                    'Q' => {
                        return Ok(());
                    }
                    _ => {
                        renderer.write_line("\r\n");
                        renderer.set_foreground(Color::BrightRed);
                        renderer.write_line("Invalid selection.");
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
            Err(_) => {
                return Ok(());
            }
        }
    }
}

/// Show user management screen
async fn show_user_management(
    connection: &mut TelnetConnection,
    _user: &User,
    state: &ServerState,
    renderer: &mut AnsiRenderer,
) -> Result<()> {
    renderer.clear_screen();
    renderer.set_foreground(Color::BrightMagenta);
    renderer.write_line("=== USER MANAGEMENT ===");
    renderer.reset();
    renderer.write_line("");

    // Get user count
    let user_manager = state.user_manager.read().await;
    let users = user_manager.list_users().await?;
    drop(user_manager);

    renderer.write_line(&format!("Total users: {}", users.len()));
    renderer.write_line("");

    renderer.set_foreground(Color::Yellow);
    renderer.write_line("User management features:");
    renderer.write_line("  • View and edit user profiles");
    renderer.write_line("  • Set security levels");
    renderer.write_line("  • Ban/unban users");
    renderer.write_line("  • View login history");
    renderer.write_line("  • Manage time limits");
    renderer.reset();
    renderer.write_line("");

    renderer.set_foreground(Color::BrightGreen);
    renderer.write_line("Full user management interface coming soon!");
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

/// Show file area management screen
async fn show_file_area_management(
    connection: &mut TelnetConnection,
    renderer: &mut AnsiRenderer,
) -> Result<()> {
    renderer.clear_screen();
    renderer.set_foreground(Color::BrightMagenta);
    renderer.write_line("=== FILE AREA MANAGEMENT ===");
    renderer.reset();
    renderer.write_line("");

    renderer.set_foreground(Color::Yellow);
    renderer.write_line("File area management features:");
    renderer.write_line("  • Create new file areas");
    renderer.write_line("  • Edit area properties");
    renderer.write_line("  • Set security levels");
    renderer.write_line("  • Configure upload/download permissions");
    renderer.write_line("  • Delete file areas");
    renderer.reset();
    renderer.write_line("");

    renderer.set_foreground(Color::BrightGreen);
    renderer.write_line("Full file area management interface coming soon!");
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

/// Show system maintenance screen
async fn show_system_maintenance(
    connection: &mut TelnetConnection,
    renderer: &mut AnsiRenderer,
) -> Result<()> {
    renderer.clear_screen();
    renderer.set_foreground(Color::BrightMagenta);
    renderer.write_line("=== SYSTEM MAINTENANCE ===");
    renderer.reset();
    renderer.write_line("");

    renderer.set_foreground(Color::Yellow);
    renderer.write_line("System maintenance features:");
    renderer.write_line("  • View active sessions");
    renderer.write_line("  • Kick individual users");
    renderer.write_line("  • Kick idle users");
    renderer.write_line("  • Broadcast messages");
    renderer.write_line("  • System shutdown");
    renderer.reset();
    renderer.write_line("");

    renderer.set_foreground(Color::BrightGreen);
    renderer.write_line("Full system maintenance interface coming soon!");
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

/// Show audit log screen
async fn show_audit_log(
    connection: &mut TelnetConnection,
    _user: &User,
    state: &ServerState,
    renderer: &mut AnsiRenderer,
) -> Result<()> {
    renderer.clear_screen();
    renderer.set_foreground(Color::BrightMagenta);
    renderer.write_line("=== AUDIT LOG ===");
    renderer.reset();
    renderer.write_line("");

    // Get recent audit entries
    let recent_entries = state.audit_logger.get_recent_entries(10).await;

    if !recent_entries.is_empty() {
        renderer.write_line(&format!("Recent {} entries:", recent_entries.len()));
        renderer.write_line("");

        for entry in recent_entries {
            renderer.set_foreground(Color::BrightYellow);
            renderer.write_text(&format!(
                "[{}] ",
                entry.timestamp.format("%Y-%m-%d %H:%M:%S")
            ));
            renderer.reset();
            renderer.set_foreground(Color::BrightWhite);
            renderer.write_text(&format!("Admin: {} ", entry.admin_user_id));
            renderer.reset();
            renderer.write_line(&format!("Action: {}", entry.action));
        }
    } else {
        renderer.set_foreground(Color::Yellow);
        renderer.write_line("No audit entries available.");
        renderer.reset();
    }

    renderer.write_line("");
    renderer.set_foreground(Color::Yellow);
    renderer.write_line("Audit logging features:");
    renderer.write_line("  • Track all administrative actions");
    renderer.write_line("  • Query by admin or action type");
    renderer.write_line("  • View detailed entry information");
    renderer.write_line("  • Export audit reports");
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

/// Show broadcast message screen
async fn show_broadcast(
    connection: &mut TelnetConnection,
    renderer: &mut AnsiRenderer,
) -> Result<()> {
    renderer.clear_screen();
    renderer.set_foreground(Color::BrightMagenta);
    renderer.write_line("=== BROADCAST MESSAGE ===");
    renderer.reset();
    renderer.write_line("");

    renderer.set_foreground(Color::Yellow);
    renderer.write_line("Broadcast features:");
    renderer.write_line("  • Send message to all online users");
    renderer.write_line("  • Target specific users or groups");
    renderer.write_line("  • Schedule broadcasts");
    renderer.write_line("  • Message templates");
    renderer.reset();
    renderer.write_line("");

    renderer.set_foreground(Color::BrightGreen);
    renderer.write_line("Full broadcast interface coming soon!");
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
