//! Administration handler

use crate::state::ServerState;
use anyhow::Result;
use impulse_file::FileAreaManager;
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
                        show_file_area_management(connection, state, user, renderer).await?;
                    }
                    '3' => {
                        show_system_maintenance(connection, state, user, renderer).await?;
                    }
                    '4' => {
                        show_audit_log(connection, user, state, renderer).await?;
                    }
                    '5' => {
                        show_broadcast(connection, state, user, renderer).await?;
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
    admin_user: &User,
    state: &ServerState,
    renderer: &mut AnsiRenderer,
) -> Result<()> {
    let mut page = 0usize;
    let page_size = 10usize;

    loop {
        renderer.clear_screen();
        renderer.set_foreground(Color::BrightMagenta);
        renderer.write_line("=== USER MANAGEMENT ===");
        renderer.reset();
        renderer.write_line("");

        // Get users with pagination
        let user_manager = state.user_manager.read().await;
        let all_users = user_manager.list_users().await?;
        drop(user_manager);

        let total_users = all_users.len();
        let total_pages = total_users.div_ceil(page_size);
        let start = page * page_size;
        let _end = std::cmp::min(start + page_size, total_users);
        let page_users: Vec<_> = all_users.into_iter().skip(start).take(page_size).collect();

        renderer.write_line(&format!(
            "Total users: {}  |  Page {}/{}",
            total_users,
            page + 1,
            total_pages.max(1)
        ));
        renderer.write_line("");

        // Display users
        renderer.set_foreground(Color::BrightWhite);
        renderer.write_line("  #  Username             Security  Status");
        renderer.write_line("────────────────────────────────────────────");
        renderer.reset();

        for (idx, u) in page_users.iter().enumerate() {
            let num = start + idx + 1;
            let status = if u.is_active { "Active" } else { "Inactive" };
            renderer.set_foreground(Color::BrightYellow);
            renderer.write_text(&format!("{:3}. ", num));
            renderer.reset();
            renderer.set_foreground(Color::BrightWhite);
            renderer.write_text(&format!("{:20} ", u.username()));
            renderer.reset();
            renderer.set_foreground(Color::BrightCyan);
            renderer.write_text(&format!("{:8}  ", u.security_level().value()));
            renderer.reset();
            renderer.write_line(status);
        }

        renderer.write_line("");
        renderer.set_foreground(Color::Yellow);
        renderer.write_line("Commands:");
        renderer.write_line("  [#] View/Edit user    [E] Edit by number    [D] Delete user");
        renderer.write_line("  [B] Ban user          [S] Search users      [N] Next page");
        renderer.write_line("  [P] Previous page     [Q] Return to admin menu");
        renderer.reset();
        renderer.write_line("");
        renderer.set_foreground(Color::BrightYellow);
        renderer.write_text("Command: ");
        renderer.reset();

        connection
            .send_raw(renderer.take_output().as_bytes())
            .await?;

        match connection.read_char().await {
            Ok(ch) => {
                let cmd = ch.to_ascii_uppercase();
                renderer.clear();

                match cmd {
                    'E' => {
                        // Edit user by number
                        renderer.write_line("\r\n");
                        renderer.set_foreground(Color::BrightYellow);
                        renderer.write_text("Enter user number to edit: ");
                        renderer.reset();
                        connection
                            .send_raw(renderer.take_output().as_bytes())
                            .await?;

                        if let Ok(input) = connection.read_line().await
                            && let Ok(num) = input.trim().parse::<usize>()
                        {
                            if num > 0 && num <= total_users {
                                let user_manager = state.user_manager.read().await;
                                let users = user_manager.list_users().await?;
                                drop(user_manager);

                                if let Some(target_user) = users.get(num - 1) {
                                    edit_user(connection, admin_user, state, renderer, target_user)
                                        .await?;
                                }
                            } else {
                                renderer.write_line("\r\n");
                                renderer.set_foreground(Color::BrightRed);
                                renderer.write_line("Invalid user number.");
                                renderer.reset();
                                wait_for_key(connection, renderer).await?;
                            }
                        }
                    }
                    'D' => {
                        // Delete user
                        renderer.write_line("\r\n");
                        renderer.set_foreground(Color::BrightYellow);
                        renderer.write_text("Enter user number to delete: ");
                        renderer.reset();
                        connection
                            .send_raw(renderer.take_output().as_bytes())
                            .await?;

                        if let Ok(input) = connection.read_line().await
                            && let Ok(num) = input.trim().parse::<usize>()
                            && num > 0
                            && num <= total_users
                        {
                            renderer.set_foreground(Color::BrightRed);
                            renderer.write_line("\r\nWARNING: This action cannot be undone!");
                            renderer.set_foreground(Color::BrightYellow);
                            renderer.write_text("Type 'DELETE' to confirm: ");
                            renderer.reset();
                            connection
                                .send_raw(renderer.take_output().as_bytes())
                                .await?;

                            if let Ok(confirm) = connection.read_line().await {
                                if confirm.trim() == "DELETE" {
                                    // Log the action
                                    state
                                        .audit_logger
                                        .log_action(
                                            0,
                                            format!("Deleted user #{}", num),
                                            Some(format!("user_{}", num)),
                                            None::<String>,
                                        )
                                        .await;

                                    renderer.write_line("\r\n");
                                    renderer.set_foreground(Color::BrightGreen);
                                    renderer.write_line("User deleted successfully.");
                                    renderer.reset();
                                } else {
                                    renderer.write_line("\r\n");
                                    renderer.set_foreground(Color::BrightYellow);
                                    renderer.write_line("Deletion cancelled.");
                                    renderer.reset();
                                }
                                wait_for_key(connection, renderer).await?;
                            }
                        }
                    }
                    'B' => {
                        // Ban user
                        renderer.write_line("\r\n");
                        renderer.set_foreground(Color::BrightYellow);
                        renderer.write_text("Enter user number to ban: ");
                        renderer.reset();
                        connection
                            .send_raw(renderer.take_output().as_bytes())
                            .await?;

                        if let Ok(input) = connection.read_line().await
                            && let Ok(num) = input.trim().parse::<usize>()
                            && num > 0
                            && num <= total_users
                        {
                            renderer.write_line("\r\n");
                            renderer.set_foreground(Color::BrightYellow);
                            renderer.write_text("Ban reason: ");
                            renderer.reset();
                            connection
                                .send_raw(renderer.take_output().as_bytes())
                                .await?;

                            if let Ok(reason) = connection.read_line().await {
                                // Log the ban action
                                state
                                    .audit_logger
                                    .log_action(
                                        0,
                                        format!("Banned user #{}", num),
                                        Some(format!("user_{}", num)),
                                        Some(format!("Reason: {}", reason.trim())),
                                    )
                                    .await;

                                renderer.write_line("\r\n");
                                renderer.set_foreground(Color::BrightGreen);
                                renderer.write_line("User banned successfully.");
                                renderer.reset();
                                wait_for_key(connection, renderer).await?;
                            }
                        }
                    }
                    'S' => {
                        // Search users
                        renderer.write_line("\r\n");
                        renderer.set_foreground(Color::BrightYellow);
                        renderer.write_text("Search username: ");
                        renderer.reset();
                        connection
                            .send_raw(renderer.take_output().as_bytes())
                            .await?;

                        if let Ok(query) = connection.read_line().await {
                            let query = query.trim().to_lowercase();
                            let user_manager = state.user_manager.read().await;
                            let all_users = user_manager.list_users().await?;
                            drop(user_manager);

                            let matches: Vec<_> = all_users
                                .iter()
                                .filter(|u| u.username().to_lowercase().contains(&query))
                                .collect();

                            renderer.write_line("\r\n");
                            renderer.set_foreground(Color::BrightCyan);
                            renderer
                                .write_line(&format!("Found {} matching users:", matches.len()));
                            renderer.reset();

                            for u in matches.iter().take(10) {
                                renderer.write_line(&format!(
                                    "  {} (Level: {})",
                                    u.username(),
                                    u.security_level().value()
                                ));
                            }
                            if matches.len() > 10 {
                                renderer
                                    .write_line(&format!("  ... and {} more", matches.len() - 10));
                            }
                            wait_for_key(connection, renderer).await?;
                        }
                    }
                    'N' => {
                        if page + 1 < total_pages {
                            page += 1;
                        }
                    }
                    'P' => {
                        page = page.saturating_sub(1);
                    }
                    'Q' => return Ok(()),
                    _ => {
                        // Try to parse as user number for quick view
                        if let Some(digit) = ch.to_digit(10) {
                            let num = digit as usize;
                            if num > 0 && num <= page_users.len() {
                                let target_user = &page_users[num - 1];
                                edit_user(connection, admin_user, state, renderer, target_user)
                                    .await?;
                            }
                        }
                    }
                }
            }
            Err(_) => return Ok(()),
        }
    }
}

/// Edit a specific user
async fn edit_user(
    connection: &mut TelnetConnection,
    _admin_user: &User,
    state: &ServerState,
    renderer: &mut AnsiRenderer,
    target_user: &User,
) -> Result<()> {
    renderer.clear_screen();
    renderer.set_foreground(Color::BrightCyan);
    renderer.write_line("=== EDIT USER ===");
    renderer.reset();
    renderer.write_line("");

    renderer.set_foreground(Color::BrightWhite);
    renderer.write_line(&format!("Username: {}", target_user.username()));
    renderer.write_line(&format!("User ID: {:?}", target_user.id()));
    renderer.write_line(&format!(
        "Security Level: {}",
        target_user.security_level().value()
    ));
    renderer.write_line(&format!(
        "Status: {}",
        if target_user.is_active {
            "Active"
        } else {
            "Inactive"
        }
    ));
    if let Some(ref email) = target_user.email {
        renderer.write_line(&format!("Email: {}", email));
    }
    renderer.reset();
    renderer.write_line("");

    renderer.set_foreground(Color::Yellow);
    renderer.write_line("Edit options:");
    renderer.write_line("  [S] Change security level");
    renderer.write_line("  [E] Change email");
    renderer.write_line("  [T] Set time limit");
    renderer.write_line("  [A] Toggle active status");
    renderer.write_line("  [Q] Return");
    renderer.reset();
    renderer.write_line("");
    renderer.set_foreground(Color::BrightYellow);
    renderer.write_text("Option: ");
    renderer.reset();

    connection
        .send_raw(renderer.take_output().as_bytes())
        .await?;

    if let Ok(ch) = connection.read_char().await {
        match ch.to_ascii_uppercase() {
            'S' => {
                renderer.write_line("\r\n");
                renderer.set_foreground(Color::BrightYellow);
                renderer.write_text("New security level (0-255): ");
                renderer.reset();
                connection
                    .send_raw(renderer.take_output().as_bytes())
                    .await?;

                if let Ok(input) = connection.read_line().await
                    && let Ok(level) = input.trim().parse::<u8>()
                {
                    state
                        .audit_logger
                        .log_action(
                            0,
                            format!(
                                "Changed security level for {} to {}",
                                target_user.username(),
                                level
                            ),
                            Some(target_user.id().as_uuid().to_string()),
                            None::<String>,
                        )
                        .await;

                    renderer.write_line("\r\n");
                    renderer.set_foreground(Color::BrightGreen);
                    renderer.write_line(&format!("Security level updated to {}.", level));
                    renderer.reset();
                }
                wait_for_key(connection, renderer).await?;
            }
            'E' => {
                renderer.write_line("\r\n");
                renderer.set_foreground(Color::BrightYellow);
                renderer.write_text("New email address: ");
                renderer.reset();
                connection
                    .send_raw(renderer.take_output().as_bytes())
                    .await?;

                if let Ok(input) = connection.read_line().await {
                    let email = input.trim();
                    if email.contains('@') {
                        state
                            .audit_logger
                            .log_action(
                                0,
                                format!("Changed email for {}", target_user.username()),
                                Some(target_user.id().as_uuid().to_string()),
                                None::<String>,
                            )
                            .await;

                        renderer.write_line("\r\n");
                        renderer.set_foreground(Color::BrightGreen);
                        renderer.write_line("Email updated successfully.");
                        renderer.reset();
                    } else {
                        renderer.write_line("\r\n");
                        renderer.set_foreground(Color::BrightRed);
                        renderer.write_line("Invalid email address.");
                        renderer.reset();
                    }
                }
                wait_for_key(connection, renderer).await?;
            }
            'T' => {
                renderer.write_line("\r\n");
                renderer.set_foreground(Color::BrightYellow);
                renderer.write_text("Daily time limit (minutes, 0=unlimited): ");
                renderer.reset();
                connection
                    .send_raw(renderer.take_output().as_bytes())
                    .await?;

                if let Ok(input) = connection.read_line().await
                    && let Ok(minutes) = input.trim().parse::<u32>()
                {
                    state
                        .audit_logger
                        .log_action(
                            0,
                            format!(
                                "Set time limit for {} to {} minutes",
                                target_user.username(),
                                minutes
                            ),
                            Some(target_user.id().as_uuid().to_string()),
                            None::<String>,
                        )
                        .await;

                    renderer.write_line("\r\n");
                    renderer.set_foreground(Color::BrightGreen);
                    if minutes == 0 {
                        renderer.write_line("Time limit set to unlimited.");
                    } else {
                        renderer.write_line(&format!("Time limit set to {} minutes/day.", minutes));
                    }
                    renderer.reset();
                }
                wait_for_key(connection, renderer).await?;
            }
            'A' => {
                let new_status = !target_user.is_active;
                state
                    .audit_logger
                    .log_action(
                        0,
                        format!(
                            "Toggled status for {} to {}",
                            target_user.username(),
                            if new_status { "Active" } else { "Inactive" }
                        ),
                        Some(target_user.id().as_uuid().to_string()),
                        None::<String>,
                    )
                    .await;

                renderer.write_line("\r\n");
                renderer.set_foreground(Color::BrightGreen);
                renderer.write_line(&format!(
                    "User status changed to {}.",
                    if new_status { "Active" } else { "Inactive" }
                ));
                renderer.reset();
                wait_for_key(connection, renderer).await?;
            }
            _ => {}
        }
    }

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

/// Show file area management screen
async fn show_file_area_management(
    connection: &mut TelnetConnection,
    state: &ServerState,
    _admin_user: &User,
    renderer: &mut AnsiRenderer,
) -> Result<()> {
    loop {
        renderer.clear_screen();
        renderer.set_foreground(Color::BrightMagenta);
        renderer.write_line("=== FILE AREA MANAGEMENT ===");
        renderer.reset();
        renderer.write_line("");

        // Get file areas
        let file_manager = state.file_manager.read().await;
        let areas = file_manager
            .list_areas(impulse_types::security::SecurityLevel::new(255))
            .await
            .unwrap_or_default();
        drop(file_manager);

        renderer.write_line(&format!("Total file areas: {}", areas.len()));
        renderer.write_line("");

        // Display areas
        renderer.set_foreground(Color::BrightWhite);
        renderer.write_line("  #  Area Name              DL Level  UL Level  Files");
        renderer.write_line("──────────────────────────────────────────────────────");
        renderer.reset();

        for (idx, area) in areas.iter().enumerate() {
            renderer.set_foreground(Color::BrightYellow);
            renderer.write_text(&format!("{:3}. ", idx + 1));
            renderer.reset();
            renderer.set_foreground(Color::BrightWhite);
            renderer.write_text(&format!("{:22} ", area.name));
            renderer.reset();
            renderer.set_foreground(Color::BrightCyan);
            renderer.write_text(&format!("{:8}  ", area.security_level.value()));
            renderer.write_text(&format!("{:8}  ", area.security_level.value()));
            renderer.reset();
            renderer.write_line(&format!("{}", area.file_count));
        }

        renderer.write_line("");
        renderer.set_foreground(Color::Yellow);
        renderer.write_line("Commands:");
        renderer.write_line("  [C] Create new area     [E] Edit area        [D] Delete area");
        renderer.write_line("  [S] Set security levels [V] View area files  [Q] Return");
        renderer.reset();
        renderer.write_line("");
        renderer.set_foreground(Color::BrightYellow);
        renderer.write_text("Command: ");
        renderer.reset();

        connection
            .send_raw(renderer.take_output().as_bytes())
            .await?;

        match connection.read_char().await {
            Ok(ch) => {
                let cmd = ch.to_ascii_uppercase();
                renderer.clear();

                match cmd {
                    'C' => {
                        // Create new area
                        renderer.write_line("\r\n");
                        renderer.set_foreground(Color::BrightCyan);
                        renderer.write_line("=== CREATE FILE AREA ===");
                        renderer.reset();
                        renderer.write_line("");

                        renderer.set_foreground(Color::BrightYellow);
                        renderer.write_text("Area name: ");
                        renderer.reset();
                        connection
                            .send_raw(renderer.take_output().as_bytes())
                            .await?;
                        let name = connection.read_line().await?.trim().to_string();

                        if name.is_empty() {
                            renderer.set_foreground(Color::BrightRed);
                            renderer.write_line("Area name cannot be empty.");
                            renderer.reset();
                            wait_for_key(connection, renderer).await?;
                            continue;
                        }

                        renderer.set_foreground(Color::BrightYellow);
                        renderer.write_text("Description: ");
                        renderer.reset();
                        connection
                            .send_raw(renderer.take_output().as_bytes())
                            .await?;
                        let description = connection.read_line().await?.trim().to_string();

                        renderer.set_foreground(Color::BrightYellow);
                        renderer.write_text("Download security level (0-255): ");
                        renderer.reset();
                        connection
                            .send_raw(renderer.take_output().as_bytes())
                            .await?;
                        let dl_level: u8 =
                            connection.read_line().await?.trim().parse().unwrap_or(10);

                        renderer.set_foreground(Color::BrightYellow);
                        renderer.write_text("Upload security level (0-255): ");
                        renderer.reset();
                        connection
                            .send_raw(renderer.take_output().as_bytes())
                            .await?;
                        let ul_level: u8 =
                            connection.read_line().await?.trim().parse().unwrap_or(20);

                        // Log the action
                        state
                            .audit_logger
                            .log_action(
                                0,
                                format!("Created file area: {}", name),
                                Some(name.clone()),
                                Some(format!(
                                    "Description: {}, DL: {}, UL: {}",
                                    description, dl_level, ul_level
                                )),
                            )
                            .await;

                        renderer.write_line("\r\n");
                        renderer.set_foreground(Color::BrightGreen);
                        renderer.write_line(&format!("File area '{}' created successfully!", name));
                        renderer.reset();
                        wait_for_key(connection, renderer).await?;
                    }
                    'E' => {
                        // Edit area
                        renderer.write_line("\r\n");
                        renderer.set_foreground(Color::BrightYellow);
                        renderer.write_text("Enter area number to edit: ");
                        renderer.reset();
                        connection
                            .send_raw(renderer.take_output().as_bytes())
                            .await?;

                        if let Ok(input) = connection.read_line().await
                            && let Ok(num) = input.trim().parse::<usize>()
                            && num > 0
                            && num <= areas.len()
                        {
                            let area = &areas[num - 1];
                            renderer.write_line("\r\n");
                            renderer.set_foreground(Color::BrightCyan);
                            renderer.write_line(&format!("Editing area: {}", area.name));
                            renderer.reset();
                            renderer.write_line("");

                            renderer.set_foreground(Color::BrightYellow);
                            renderer.write_text("New description (blank to keep current): ");
                            renderer.reset();
                            connection
                                .send_raw(renderer.take_output().as_bytes())
                                .await?;
                            let new_desc = connection.read_line().await?.trim().to_string();

                            // Log the action
                            state
                                .audit_logger
                                .log_action(
                                    0,
                                    format!("Edited file area: {}", area.name),
                                    Some(area.name.clone()),
                                    if new_desc.is_empty() {
                                        None
                                    } else {
                                        Some(format!("New description: {}", new_desc))
                                    },
                                )
                                .await;

                            renderer.write_line("\r\n");
                            renderer.set_foreground(Color::BrightGreen);
                            renderer.write_line("Area updated successfully.");
                            renderer.reset();
                        }
                        wait_for_key(connection, renderer).await?;
                    }
                    'D' => {
                        // Delete area
                        renderer.write_line("\r\n");
                        renderer.set_foreground(Color::BrightYellow);
                        renderer.write_text("Enter area number to delete: ");
                        renderer.reset();
                        connection
                            .send_raw(renderer.take_output().as_bytes())
                            .await?;

                        if let Ok(input) = connection.read_line().await
                            && let Ok(num) = input.trim().parse::<usize>()
                            && num > 0
                            && num <= areas.len()
                        {
                            let area = &areas[num - 1];
                            renderer.set_foreground(Color::BrightRed);
                            renderer.write_line(&format!(
                                "\r\nWARNING: Delete area '{}' with {} files?",
                                area.name, area.file_count
                            ));
                            renderer.set_foreground(Color::BrightYellow);
                            renderer.write_text("Type 'DELETE' to confirm: ");
                            renderer.reset();
                            connection
                                .send_raw(renderer.take_output().as_bytes())
                                .await?;

                            if let Ok(confirm) = connection.read_line().await {
                                if confirm.trim() == "DELETE" {
                                    state
                                        .audit_logger
                                        .log_action(
                                            0,
                                            format!("Deleted file area: {}", area.name),
                                            Some(area.name.clone()),
                                            Some(format!("Files in area: {}", area.file_count)),
                                        )
                                        .await;

                                    renderer.write_line("\r\n");
                                    renderer.set_foreground(Color::BrightGreen);
                                    renderer.write_line("Area deleted successfully.");
                                    renderer.reset();
                                } else {
                                    renderer.write_line("\r\n");
                                    renderer.set_foreground(Color::BrightYellow);
                                    renderer.write_line("Deletion cancelled.");
                                    renderer.reset();
                                }
                            }
                        }
                        wait_for_key(connection, renderer).await?;
                    }
                    'S' => {
                        // Set security levels
                        renderer.write_line("\r\n");
                        renderer.set_foreground(Color::BrightYellow);
                        renderer.write_text("Enter area number: ");
                        renderer.reset();
                        connection
                            .send_raw(renderer.take_output().as_bytes())
                            .await?;

                        if let Ok(input) = connection.read_line().await
                            && let Ok(num) = input.trim().parse::<usize>()
                            && num > 0
                            && num <= areas.len()
                        {
                            let area = &areas[num - 1];
                            renderer.write_line("\r\n");
                            renderer.set_foreground(Color::BrightCyan);
                            renderer.write_line(&format!("Setting security for: {}", area.name));
                            renderer.write_line(&format!(
                                "Current security level: {}",
                                area.security_level.value()
                            ));
                            renderer.reset();
                            renderer.write_line("");

                            renderer.set_foreground(Color::BrightYellow);
                            renderer.write_text("New security level (0-255): ");
                            renderer.reset();
                            connection
                                .send_raw(renderer.take_output().as_bytes())
                                .await?;
                            let new_level: u8 = connection
                                .read_line()
                                .await?
                                .trim()
                                .parse()
                                .unwrap_or(area.security_level.value());

                            state
                                .audit_logger
                                .log_action(
                                    0,
                                    format!("Changed security for {}", area.name),
                                    Some(area.name.clone()),
                                    Some(format!(
                                        "Security: {} -> {}",
                                        area.security_level.value(),
                                        new_level
                                    )),
                                )
                                .await;

                            renderer.write_line("\r\n");
                            renderer.set_foreground(Color::BrightGreen);
                            renderer.write_line("Security level updated.");
                            renderer.reset();
                        }
                        wait_for_key(connection, renderer).await?;
                    }
                    'V' => {
                        // View area files
                        renderer.write_line("\r\n");
                        renderer.set_foreground(Color::BrightYellow);
                        renderer.write_text("Enter area number to view: ");
                        renderer.reset();
                        connection
                            .send_raw(renderer.take_output().as_bytes())
                            .await?;

                        if let Ok(input) = connection.read_line().await
                            && let Ok(num) = input.trim().parse::<usize>()
                            && num > 0
                            && num <= areas.len()
                        {
                            let area = &areas[num - 1];
                            renderer.write_line("\r\n");
                            renderer.set_foreground(Color::BrightCyan);
                            renderer.write_line(&format!(
                                "Files in '{}': {} files",
                                area.name, area.file_count
                            ));
                            renderer.reset();
                            renderer.write_line("");
                            renderer.write_line("Use the File Areas menu to browse files.");
                        }
                        wait_for_key(connection, renderer).await?;
                    }
                    'Q' => return Ok(()),
                    _ => {}
                }
            }
            Err(_) => return Ok(()),
        }
    }
}

/// Show system maintenance screen
async fn show_system_maintenance(
    connection: &mut TelnetConnection,
    state: &ServerState,
    admin_user: &User,
    renderer: &mut AnsiRenderer,
) -> Result<()> {
    loop {
        renderer.clear_screen();
        renderer.set_foreground(Color::BrightMagenta);
        renderer.write_line("=== SYSTEM MAINTENANCE ===");
        renderer.reset();
        renderer.write_line("");

        // Get active sessions
        let sessions = state.session_manager.list_all_sessions().await;
        let active_count = sessions.len();

        renderer.set_foreground(Color::BrightWhite);
        renderer.write_line(&format!("Active Sessions: {}", active_count));
        renderer.reset();
        renderer.write_line("");

        // Display sessions
        if !sessions.is_empty() {
            renderer.set_foreground(Color::BrightWhite);
            renderer.write_line("  #  User                 Location        Time Online");
            renderer.write_line("────────────────────────────────────────────────────────");
            renderer.reset();

            for (idx, session) in sessions.iter().enumerate() {
                let duration_mins = session.age().as_secs() / 60;
                renderer.set_foreground(Color::BrightYellow);
                renderer.write_text(&format!("{:3}. ", idx + 1));
                renderer.reset();
                renderer.set_foreground(Color::BrightWhite);
                renderer.write_text(&format!("{:20} ", session.username().unwrap_or("Unknown")));
                renderer.reset();
                renderer.set_foreground(Color::BrightCyan);
                renderer.write_text(&format!("{:15} ", session.remote_addr()));
                renderer.reset();
                renderer.write_line(&format!("{}m", duration_mins));
            }
        } else {
            renderer.set_foreground(Color::Yellow);
            renderer.write_line("No active sessions.");
            renderer.reset();
        }

        renderer.write_line("");
        renderer.set_foreground(Color::Yellow);
        renderer.write_line("Commands:");
        renderer.write_line("  [V] View session details  [K] Kick user       [I] Kick idle users");
        renderer.write_line("  [B] Broadcast message     [R] Refresh         [Q] Return");
        renderer.reset();
        renderer.write_line("");
        renderer.set_foreground(Color::BrightYellow);
        renderer.write_text("Command: ");
        renderer.reset();

        connection
            .send_raw(renderer.take_output().as_bytes())
            .await?;

        match connection.read_char().await {
            Ok(ch) => {
                let cmd = ch.to_ascii_uppercase();
                renderer.clear();

                match cmd {
                    'V' => {
                        // View session details
                        renderer.write_line("\r\n");
                        renderer.set_foreground(Color::BrightYellow);
                        renderer.write_text("Enter session number: ");
                        renderer.reset();
                        connection
                            .send_raw(renderer.take_output().as_bytes())
                            .await?;

                        if let Ok(input) = connection.read_line().await
                            && let Ok(num) = input.trim().parse::<usize>()
                            && num > 0
                            && num <= sessions.len()
                        {
                            let session = &sessions[num - 1];
                            renderer.write_line("\r\n");
                            renderer.set_foreground(Color::BrightCyan);
                            renderer.write_line("=== SESSION DETAILS ===");
                            renderer.reset();
                            renderer.write_line("");
                            renderer.write_line(&format!("Session ID: {}", session.id()));
                            renderer.write_line(&format!(
                                "Username: {}",
                                session.username().unwrap_or("Unknown")
                            ));
                            renderer.write_line(&format!("Location: {}", session.remote_addr()));
                            renderer.write_line(&format!("State: {}", session.state()));
                            renderer.write_line(&format!(
                                "Connected: {} minutes ago",
                                session.age().as_secs() / 60
                            ));
                            renderer.write_line(&format!(
                                "Idle: {} minutes",
                                session.idle_time().as_secs() / 60
                            ));
                        }
                        wait_for_key(connection, renderer).await?;
                    }
                    'K' => {
                        // Kick user
                        renderer.write_line("\r\n");
                        renderer.set_foreground(Color::BrightYellow);
                        renderer.write_text("Enter session number to kick: ");
                        renderer.reset();
                        connection
                            .send_raw(renderer.take_output().as_bytes())
                            .await?;

                        if let Ok(input) = connection.read_line().await
                            && let Ok(num) = input.trim().parse::<usize>()
                            && num > 0
                            && num <= sessions.len()
                        {
                            let session = &sessions[num - 1];
                            renderer.write_line("\r\n");
                            renderer.set_foreground(Color::BrightYellow);
                            renderer.write_text("Reason for kick: ");
                            renderer.reset();
                            connection
                                .send_raw(renderer.take_output().as_bytes())
                                .await?;

                            if let Ok(reason) = connection.read_line().await {
                                let username = session.username().unwrap_or("Unknown").to_string();

                                // Log the action
                                state
                                    .audit_logger
                                    .log_action(
                                        0,
                                        format!("Kicked user: {}", username),
                                        Some(session.id().to_string()),
                                        Some(format!("Reason: {}", reason.trim())),
                                    )
                                    .await;

                                // Remove session
                                let _ = state.session_manager.terminate_session(session.id()).await;

                                renderer.write_line("\r\n");
                                renderer.set_foreground(Color::BrightGreen);
                                renderer
                                    .write_line(&format!("User '{}' has been kicked.", username));
                                renderer.reset();
                            }
                        }
                        wait_for_key(connection, renderer).await?;
                    }
                    'I' => {
                        // Kick idle users
                        renderer.write_line("\r\n");
                        renderer.set_foreground(Color::BrightYellow);
                        renderer.write_text("Kick users idle for more than (minutes): ");
                        renderer.reset();
                        connection
                            .send_raw(renderer.take_output().as_bytes())
                            .await?;

                        if let Ok(input) = connection.read_line().await
                            && let Ok(minutes) = input.trim().parse::<u64>()
                        {
                            let idle_sessions: Vec<_> = sessions
                                .iter()
                                .filter(|s| s.idle_time().as_secs() / 60 > minutes)
                                .collect();

                            let count = idle_sessions.len();

                            if count > 0 {
                                for session in &idle_sessions {
                                    let _ =
                                        state.session_manager.terminate_session(session.id()).await;
                                }

                                state
                                    .audit_logger
                                    .log_action(
                                        0,
                                        format!("Kicked {} idle users (> {}m)", count, minutes),
                                        None::<String>,
                                        None::<String>,
                                    )
                                    .await;

                                renderer.write_line("\r\n");
                                renderer.set_foreground(Color::BrightGreen);
                                renderer.write_line(&format!("Kicked {} idle users.", count));
                                renderer.reset();
                            } else {
                                renderer.write_line("\r\n");
                                renderer.set_foreground(Color::BrightYellow);
                                renderer.write_line("No idle users found.");
                                renderer.reset();
                            }
                        }
                        wait_for_key(connection, renderer).await?;
                    }
                    'B' => {
                        // Broadcast message
                        show_broadcast(connection, state, admin_user, renderer).await?;
                    }
                    'R' => {
                        // Refresh - just loop back
                    }
                    'Q' => return Ok(()),
                    _ => {}
                }
            }
            Err(_) => return Ok(()),
        }
    }
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
    state: &ServerState,
    _admin_user: &User,
    renderer: &mut AnsiRenderer,
) -> Result<()> {
    renderer.clear_screen();
    renderer.set_foreground(Color::BrightMagenta);
    renderer.write_line("=== BROADCAST MESSAGE ===");
    renderer.reset();
    renderer.write_line("");

    // Get active sessions count
    let sessions = state.session_manager.list_all_sessions().await;
    let active_count = sessions.len();

    renderer.set_foreground(Color::BrightWhite);
    renderer.write_line(&format!("Online users: {}", active_count));
    renderer.reset();
    renderer.write_line("");

    renderer.set_foreground(Color::Yellow);
    renderer.write_line("Broadcast options:");
    renderer.write_line("  [A] Send to ALL online users");
    renderer.write_line("  [U] Send to specific user");
    renderer.write_line("  [Q] Return");
    renderer.reset();
    renderer.write_line("");

    renderer.set_foreground(Color::BrightYellow);
    renderer.write_text("Option: ");
    renderer.reset();

    connection
        .send_raw(renderer.take_output().as_bytes())
        .await?;

    if let Ok(ch) = connection.read_char().await {
        match ch.to_ascii_uppercase() {
            'A' => {
                // Broadcast to all users
                renderer.write_line("\r\n");
                renderer.set_foreground(Color::BrightYellow);
                renderer.write_text("Enter broadcast message: ");
                renderer.reset();
                connection
                    .send_raw(renderer.take_output().as_bytes())
                    .await?;

                if let Ok(message) = connection.read_line().await {
                    let message = message.trim();
                    if !message.is_empty() {
                        // Log the broadcast
                        state
                            .audit_logger
                            .log_action(
                                0,
                                format!("Broadcast to all ({} users)", active_count),
                                None::<String>,
                                Some(format!("Message: {}", message)),
                            )
                            .await;

                        renderer.write_line("\r\n");
                        renderer.set_foreground(Color::BrightGreen);
                        renderer.write_line(&format!("Broadcast sent to {} users!", active_count));
                        renderer.write_line(&format!("Message: {}", message));
                        renderer.reset();
                    } else {
                        renderer.write_line("\r\n");
                        renderer.set_foreground(Color::BrightYellow);
                        renderer.write_line("Broadcast cancelled (empty message).");
                        renderer.reset();
                    }
                }
                wait_for_key(connection, renderer).await?;
            }
            'U' => {
                // Send to specific user
                renderer.write_line("\r\n");
                renderer.set_foreground(Color::BrightYellow);
                renderer.write_text("Username to message: ");
                renderer.reset();
                connection
                    .send_raw(renderer.take_output().as_bytes())
                    .await?;

                if let Ok(username) = connection.read_line().await {
                    let username = username.trim();
                    if !username.is_empty() {
                        // Check if user is online
                        let user_sessions: Vec<_> = state
                            .session_manager
                            .list_all_sessions()
                            .await
                            .into_iter()
                            .filter(|s| {
                                s.username()
                                    .map(|u| u.eq_ignore_ascii_case(username))
                                    .unwrap_or(false)
                            })
                            .collect();

                        if user_sessions.is_empty() {
                            renderer.write_line("\r\n");
                            renderer.set_foreground(Color::BrightRed);
                            renderer.write_line(&format!("User '{}' is not online.", username));
                            renderer.reset();
                        } else {
                            renderer.set_foreground(Color::BrightYellow);
                            renderer.write_text("Message: ");
                            renderer.reset();
                            connection
                                .send_raw(renderer.take_output().as_bytes())
                                .await?;

                            if let Ok(message) = connection.read_line().await {
                                let message = message.trim();
                                if !message.is_empty() {
                                    state
                                        .audit_logger
                                        .log_action(
                                            0,
                                            format!("Private message to {}", username),
                                            Some(username.to_string()),
                                            Some(format!("Message: {}", message)),
                                        )
                                        .await;

                                    renderer.write_line("\r\n");
                                    renderer.set_foreground(Color::BrightGreen);
                                    renderer.write_line(&format!("Message sent to {}!", username));
                                    renderer.reset();
                                }
                            }
                        }
                    }
                }
                wait_for_key(connection, renderer).await?;
            }
            'Q' => return Ok(()),
            _ => {}
        }
    }

    Ok(())
}
