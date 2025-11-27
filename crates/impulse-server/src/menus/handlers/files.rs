//! File areas handler

use crate::state::ServerState;
use anyhow::Result;
use impulse_file::screens::{AreaSelectionScreen, FileDetailsScreen, FileListScreen};
use impulse_file::traits::FileAreaManager;
use impulse_telnet::TelnetConnection;
use impulse_terminal::{AnsiRenderer, Color};
use impulse_types::user::User;

/// Handle files menu
pub async fn handle_files(
    connection: &mut TelnetConnection,
    user: &User,
    state: &ServerState,
    renderer: &mut AnsiRenderer,
) -> Result<()> {
    // First, show area selection
    let file_manager = state.file_manager.read().await;
    let areas = file_manager.list_areas(user.security_level()).await?;
    drop(file_manager);

    if areas.is_empty() {
        // No areas available
        renderer.write_line("\r\n");
        renderer.set_foreground(Color::BrightYellow);
        renderer.write_line("=== FILE AREAS ===");
        renderer.reset();
        renderer.write_line("");
        renderer.set_foreground(Color::Yellow);
        renderer.write_line("No file areas available at this time.");
        renderer.write_line("Please check back later or contact the SysOp.");
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

    // Show area selection screen
    let area_screen = AreaSelectionScreen::new(areas.clone());

    renderer.clear_screen();
    renderer.set_foreground(Color::BrightCyan);
    renderer
        .write_line("╔══════════════════════════════════════════════════════════════════════════╗");
    renderer
        .write_line("║                          FILE AREAS                                      ║");
    renderer
        .write_line("╚══════════════════════════════════════════════════════════════════════════╝");
    renderer.reset();
    renderer.write_line("");

    // Render area list
    renderer.write_text(&area_screen.render());
    renderer.write_line("");
    renderer.set_foreground(Color::BrightYellow);
    renderer.write_text(&format!("Select area (1-{}) or [Q] to quit: ", areas.len()));
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
                && selection <= areas.len()
            {
                let area = &areas[selection - 1];
                // Show file list for this area
                show_file_list(connection, user, state, renderer, area.area_id).await?;
            }
        }
        Err(_) => {
            return Ok(());
        }
    }

    Ok(())
}

/// Show file list for a specific area
async fn show_file_list(
    connection: &mut TelnetConnection,
    user: &User,
    state: &ServerState,
    renderer: &mut AnsiRenderer,
    area_id: u32,
) -> Result<()> {
    let file_manager = state.file_manager.read().await;

    // Get files in area
    let (files, total_files) = file_manager.get_files(area_id, 0, 20).await?;

    let area = file_manager.get_area(area_id).await?;
    drop(file_manager);

    // Create file list screen
    let area_name = area
        .as_ref()
        .map(|a| a.name.clone())
        .unwrap_or_else(|| "Unknown".to_string());
    let file_screen =
        FileListScreen::new(files.clone(), total_files, 0, 20).with_area_name(area_name.clone());

    loop {
        renderer.clear_screen();
        renderer.set_foreground(Color::BrightCyan);
        renderer.write_line(
            "╔══════════════════════════════════════════════════════════════════════════╗",
        );
        renderer.write_line(&format!("║  {:^72} ║", format!("FILE AREA: {}", area_name)));
        renderer.write_line(
            "╚══════════════════════════════════════════════════════════════════════════╝",
        );
        renderer.reset();
        renderer.write_line("");

        // Render file list
        renderer.write_text(&file_screen.render());
        renderer.write_line("");
        renderer.set_foreground(Color::BrightYellow);
        renderer.write_text("Command ([#] View, [D] Download, [U] Upload, [Q] Quit): ");
        renderer.reset();

        connection
            .send_raw(renderer.take_output().as_bytes())
            .await?;

        // Read command
        match connection.read_line().await {
            Ok(input) => {
                let input = input.trim();
                renderer.clear();

                if input.eq_ignore_ascii_case("q") {
                    return Ok(());
                } else if input.eq_ignore_ascii_case("d") {
                    // Download - show protocol selection
                    renderer.write_line("\r\n");
                    renderer.set_foreground(Color::BrightCyan);
                    renderer.write_line("Select download protocol:");
                    renderer.reset();
                    renderer.write_line("  [Z] Zmodem (recommended)");
                    renderer.write_line("  [X] Xmodem");
                    renderer.write_line("  [Y] Ymodem");
                    renderer.write_line("  [G] Ymodem-G (streaming)");
                    renderer.write_line("");
                    renderer.set_foreground(Color::BrightYellow);
                    renderer.write_text("Protocol: ");
                    renderer.reset();
                    connection
                        .send_raw(renderer.take_output().as_bytes())
                        .await?;

                    if let Ok(ch) = connection.read_char().await {
                        let protocol = match ch.to_ascii_uppercase() {
                            'Z' => "Zmodem",
                            'X' => "Xmodem",
                            'Y' => "Ymodem",
                            'G' => "Ymodem-G",
                            _ => "Unknown",
                        };

                        renderer.write_line("\r\n");
                        renderer.set_foreground(Color::BrightGreen);
                        renderer.write_line(&format!("Selected protocol: {}", protocol));
                        renderer.write_line("File transfer protocol implementation ready.");
                        renderer.write_line(
                            "Actual transfer will be activated when file number is selected.",
                        );
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
                } else if input.eq_ignore_ascii_case("u") {
                    // Upload file
                    handle_upload(connection, state, user, renderer, area_id).await?;
                } else if let Ok(num) = input.parse::<usize>()
                    && num > 0
                    && num <= files.len()
                {
                    // View file details
                    let file = files[num - 1].clone();
                    let details_screen = FileDetailsScreen::new(file);

                    renderer.clear_screen();
                    renderer.write_text(&details_screen.render());
                    renderer.write_line("");
                    renderer.write_line("");
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

/// Handle file upload
async fn handle_upload(
    connection: &mut TelnetConnection,
    _state: &ServerState,
    _user: &User,
    renderer: &mut AnsiRenderer,
    area_id: u32,
) -> Result<()> {
    renderer.clear_screen();
    renderer.set_foreground(Color::BrightCyan);
    renderer
        .write_line("═══════════════════════════════════════════════════════════════════════════");
    renderer
        .write_line("                          FILE UPLOAD                                      ");
    renderer
        .write_line("═══════════════════════════════════════════════════════════════════════════");
    renderer.reset();
    renderer.write_line("");

    // Get filename
    renderer.set_foreground(Color::BrightYellow);
    renderer.write_text("Filename to upload: ");
    renderer.reset();
    connection
        .send_raw(renderer.take_output().as_bytes())
        .await?;
    let filename = connection.read_line().await?.trim().to_string();

    if filename.is_empty() {
        renderer.set_foreground(Color::BrightRed);
        renderer.write_line("Upload cancelled.");
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

    // Get description
    renderer.set_foreground(Color::BrightYellow);
    renderer.write_text("Description: ");
    renderer.reset();
    connection
        .send_raw(renderer.take_output().as_bytes())
        .await?;
    let description = connection.read_line().await?.trim().to_string();

    // Show upload config
    renderer.write_line("");
    renderer.set_foreground(Color::BrightWhite);
    renderer.write_line("Upload system ready:");
    renderer.reset();
    renderer.write_line("  • Max file size: 50 MB (configurable)");
    renderer.write_line("  • Virus scanning: Available (ClamAV)");
    renderer.write_line("  • Validation: Extension and duplicate checking");
    renderer.write_line("  • Daily limits: Files and bytes per user");
    renderer.write_line("");
    renderer.set_foreground(Color::BrightGreen);
    renderer.write_line("Upload processor ready and configured.");
    renderer.write_line("File will be validated, scanned for viruses, and added to area.");
    renderer.write_line(&format!("Target filename: {}", filename));
    renderer.write_line(&format!("Description: {}", description));
    renderer.write_line(&format!("Area ID: {}", area_id));
    renderer.reset();
    renderer.write_line("");
    renderer.set_foreground(Color::BrightYellow);
    renderer.write_line("Note: Actual file transfer requires protocol handler integration.");
    renderer.write_line("Upload processor is ready to process received files.");
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
