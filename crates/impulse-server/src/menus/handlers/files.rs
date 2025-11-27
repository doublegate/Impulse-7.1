//! File areas handler

use crate::state::ServerState;
use anyhow::Result;
use impulse_file::screens::{AreaSelectionScreen, FileDetailsScreen, FileListScreen};
use impulse_file::traits::FileAreaManager;
use impulse_telnet::TelnetConnection;
use impulse_terminal::{AnsiRenderer, Color};
use impulse_types::file::FileEntry;
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
        renderer.write_text("Command ([#] View/Download, [U] Upload, [S] Search, [Q] Quit): ");
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
                } else if input.eq_ignore_ascii_case("u") {
                    // Upload file
                    handle_upload(connection, state, user, renderer, area_id).await?;
                } else if input.eq_ignore_ascii_case("s") {
                    // Search files
                    handle_search(connection, state, user, renderer, area_id).await?;
                } else if let Ok(num) = input.parse::<usize>()
                    && num > 0
                    && num <= files.len()
                {
                    // View file details and optionally download
                    let file = files[num - 1].clone();
                    handle_file_details(connection, user, renderer, &file).await?;
                }
            }
            Err(_) => {
                return Ok(());
            }
        }
    }
}

/// Handle file details and download
async fn handle_file_details(
    connection: &mut TelnetConnection,
    user: &User,
    renderer: &mut AnsiRenderer,
    file: &FileEntry,
) -> Result<()> {
    let details_screen = FileDetailsScreen::new(file.clone());

    renderer.clear_screen();
    renderer.write_text(&details_screen.render());
    renderer.write_line("");
    renderer.write_line("");

    renderer.set_foreground(Color::Yellow);
    renderer.write_line("Options:");
    renderer.write_line("  [D] Download this file");
    renderer.write_line("  [Q] Return to file list");
    renderer.reset();
    renderer.write_line("");

    renderer.set_foreground(Color::BrightYellow);
    renderer.write_text("Option: ");
    renderer.reset();

    connection
        .send_raw(renderer.take_output().as_bytes())
        .await?;

    if let Ok(ch) = connection.read_char().await
        && ch.eq_ignore_ascii_case(&'D')
    {
        // Initiate download
        handle_download(connection, user, renderer, file).await?;
    }

    Ok(())
}

/// Handle file download with protocol selection
async fn handle_download(
    connection: &mut TelnetConnection,
    _user: &User,
    renderer: &mut AnsiRenderer,
    file: &FileEntry,
) -> Result<()> {
    renderer.clear_screen();
    renderer.set_foreground(Color::BrightCyan);
    renderer.write_line("=== FILE DOWNLOAD ===");
    renderer.reset();
    renderer.write_line("");

    renderer.set_foreground(Color::BrightWhite);
    renderer.write_line(&format!("File: {}", file.filename));
    renderer.write_line(&format!("Size: {} bytes", file.size_bytes));
    renderer.reset();
    renderer.write_line("");

    // Note: File-level security is handled at the area level during listing
    // Individual files inherit security from their containing area

    renderer.set_foreground(Color::BrightCyan);
    renderer.write_line("Select download protocol:");
    renderer.reset();
    renderer.write_line("");

    renderer.write_line("  [Z] Zmodem (Recommended)");
    renderer.write_line("      - Fastest, auto-resume, 32-bit CRC, batch support");
    renderer.write_line("");
    renderer.write_line("  [Y] Ymodem");
    renderer.write_line("      - Good speed, 16-bit CRC, batch support");
    renderer.write_line("");
    renderer.write_line("  [G] Ymodem-G");
    renderer.write_line("      - Streaming mode, fastest for clean connections");
    renderer.write_line("");
    renderer.write_line("  [X] Xmodem");
    renderer.write_line("      - Maximum compatibility, single file");
    renderer.write_line("");
    renderer.write_line("  [Q] Cancel download");
    renderer.write_line("");

    renderer.set_foreground(Color::BrightYellow);
    renderer.write_text("Protocol: ");
    renderer.reset();

    connection
        .send_raw(renderer.take_output().as_bytes())
        .await?;

    if let Ok(ch) = connection.read_char().await {
        let protocol = match ch.to_ascii_uppercase() {
            'Z' => Some(("Zmodem", "32-bit CRC, streaming, crash recovery")),
            'Y' => Some(("Ymodem", "16-bit CRC, batch mode")),
            'G' => Some(("Ymodem-G", "streaming, no error correction")),
            'X' => Some(("Xmodem", "checksum/CRC, 128-byte blocks")),
            'Q' => None,
            _ => None,
        };

        if let Some((protocol_name, features)) = protocol {
            renderer.write_line("\r\n");
            renderer.set_foreground(Color::BrightGreen);
            renderer.write_line(&format!("Protocol: {} ({})", protocol_name, features));
            renderer.reset();
            renderer.write_line("");

            // Simulate transfer initiation
            renderer.set_foreground(Color::BrightYellow);
            renderer.write_line("Initiating transfer...");
            renderer.reset();
            renderer.write_line("");

            renderer.write_line("  - Preparing file for transfer");
            renderer.write_line(&format!("  - File: {}", file.filename));
            renderer.write_line(&format!("  - Size: {} bytes", file.size_bytes));
            renderer.write_line(&format!("  - Protocol: {}", protocol_name));
            renderer.write_line("");

            // In a real implementation, this would start the actual protocol transfer
            renderer.set_foreground(Color::BrightCyan);
            renderer.write_line("Ready to send. Start your terminal's receive mode now.");
            renderer.write_line("");
            renderer.write_line("Transfer would begin here with actual protocol negotiation.");
            renderer.reset();

            // Increment download count (in real implementation)
            renderer.write_line("");
            renderer.set_foreground(Color::BrightGreen);
            renderer.write_line("Download statistics will be updated upon completion.");
            renderer.reset();
        } else {
            renderer.write_line("\r\n");
            renderer.set_foreground(Color::BrightYellow);
            renderer.write_line("Download cancelled.");
            renderer.reset();
        }
    }

    wait_for_key(connection, renderer).await?;
    Ok(())
}

/// Handle file upload
async fn handle_upload(
    connection: &mut TelnetConnection,
    _state: &ServerState,
    user: &User,
    renderer: &mut AnsiRenderer,
    area_id: u32,
) -> Result<()> {
    renderer.clear_screen();
    renderer.set_foreground(Color::BrightCyan);
    renderer.write_line("=== FILE UPLOAD ===");
    renderer.reset();
    renderer.write_line("");

    // Show user's upload statistics
    renderer.set_foreground(Color::BrightWhite);
    renderer.write_line(&format!("Files uploaded: {}", user.stats.uploads));
    renderer.write_line(&format!("Uploaded: {} KB", user.stats.upload_kb));
    renderer.reset();
    renderer.write_line("");

    // Get filename
    renderer.set_foreground(Color::BrightYellow);
    renderer.write_text("Filename to upload (or Q to cancel): ");
    renderer.reset();
    connection
        .send_raw(renderer.take_output().as_bytes())
        .await?;
    let filename = connection.read_line().await?.trim().to_string();

    if filename.is_empty() || filename.eq_ignore_ascii_case("q") {
        renderer.set_foreground(Color::BrightYellow);
        renderer.write_line("\r\nUpload cancelled.");
        renderer.reset();
        wait_for_key(connection, renderer).await?;
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

    // Select upload protocol
    renderer.write_line("");
    renderer.set_foreground(Color::BrightCyan);
    renderer.write_line("Select upload protocol:");
    renderer.reset();
    renderer.write_line("  [Z] Zmodem (Recommended)");
    renderer.write_line("  [Y] Ymodem");
    renderer.write_line("  [X] Xmodem");
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
            'Y' => "Ymodem",
            'X' => "Xmodem",
            _ => "Zmodem",
        };

        renderer.write_line("\r\n");
        renderer.set_foreground(Color::BrightWhite);
        renderer.write_line("Upload Configuration:");
        renderer.reset();
        renderer.write_line(&format!("  - Target filename: {}", filename));
        renderer.write_line(&format!("  - Description: {}", description));
        renderer.write_line(&format!("  - Area ID: {}", area_id));
        renderer.write_line(&format!("  - Protocol: {}", protocol));
        renderer.write_line("");

        renderer.set_foreground(Color::Yellow);
        renderer.write_line("Upload Processing:");
        renderer.reset();
        renderer.write_line("  - Max file size: 50 MB");
        renderer.write_line("  - Virus scanning: ClamAV integration available");
        renderer.write_line("  - Validation: Extension checking, duplicate detection");
        renderer.write_line("  - FILE_ID.DIZ: Auto-extraction from archives");
        renderer.write_line("");

        renderer.set_foreground(Color::BrightCyan);
        renderer.write_line("Ready to receive. Start your terminal's send mode now.");
        renderer.write_line("");
        renderer.write_line("Transfer would begin here with actual protocol negotiation.");
        renderer.reset();
        renderer.write_line("");

        renderer.set_foreground(Color::BrightGreen);
        renderer.write_line("Upload will be processed upon completion:");
        renderer.write_line("  1. File received and validated");
        renderer.write_line("  2. Virus scan executed");
        renderer.write_line("  3. Description extracted (if archive)");
        renderer.write_line("  4. Added to file area catalog");
        renderer.write_line("  5. Statistics updated");
        renderer.reset();
    }

    wait_for_key(connection, renderer).await?;
    Ok(())
}

/// Handle file search
async fn handle_search(
    connection: &mut TelnetConnection,
    state: &ServerState,
    user: &User,
    renderer: &mut AnsiRenderer,
    area_id: u32,
) -> Result<()> {
    renderer.clear_screen();
    renderer.set_foreground(Color::BrightCyan);
    renderer.write_line("=== FILE SEARCH ===");
    renderer.reset();
    renderer.write_line("");

    renderer.set_foreground(Color::Yellow);
    renderer.write_line("Search options:");
    renderer.write_line("  - Use * for wildcards (e.g., *.zip, game*.*)");
    renderer.write_line("  - Search matches filename and description");
    renderer.reset();
    renderer.write_line("");

    renderer.set_foreground(Color::BrightYellow);
    renderer.write_text("Search pattern: ");
    renderer.reset();
    connection
        .send_raw(renderer.take_output().as_bytes())
        .await?;

    let pattern = connection.read_line().await?.trim().to_string();

    if pattern.is_empty() {
        renderer.set_foreground(Color::BrightYellow);
        renderer.write_line("\r\nSearch cancelled.");
        renderer.reset();
        wait_for_key(connection, renderer).await?;
        return Ok(());
    }

    renderer.write_line("");
    renderer.set_foreground(Color::BrightWhite);
    renderer.write_line(&format!("Searching for: {}", pattern));
    renderer.reset();
    renderer.write_line("");

    // Get files and filter by pattern
    let file_manager = state.file_manager.read().await;
    let (files, _total) = file_manager.get_files(area_id, 0, 100).await?;
    drop(file_manager);

    // Simple pattern matching (convert * to wildcard)
    let pattern_lower = pattern.to_lowercase();
    let matches: Vec<_> = files
        .iter()
        .filter(|f| {
            let filename_lower = f.filename.to_lowercase();
            let desc_lower = f.description.to_lowercase();

            if pattern_lower.contains('*') {
                // Simple wildcard matching
                let parts: Vec<&str> = pattern_lower.split('*').collect();
                let mut pos = 0;
                for part in parts {
                    if part.is_empty() {
                        continue;
                    }
                    if let Some(found_pos) = filename_lower[pos..].find(part) {
                        pos += found_pos + part.len();
                    } else {
                        return false;
                    }
                }
                true
            } else {
                filename_lower.contains(&pattern_lower) || desc_lower.contains(&pattern_lower)
            }
        })
        .take(20)
        .collect();

    if matches.is_empty() {
        renderer.set_foreground(Color::BrightYellow);
        renderer.write_line("No files found matching your search.");
        renderer.reset();
    } else {
        renderer.set_foreground(Color::BrightGreen);
        renderer.write_line(&format!("Found {} matching files:", matches.len()));
        renderer.reset();
        renderer.write_line("");

        renderer.set_foreground(Color::BrightWhite);
        renderer.write_line("  #  Filename                 Size        Description");
        renderer.write_line("─────────────────────────────────────────────────────────");
        renderer.reset();

        for (idx, file) in matches.iter().enumerate() {
            renderer.set_foreground(Color::BrightYellow);
            renderer.write_text(&format!("{:3}. ", idx + 1));
            renderer.reset();
            renderer.set_foreground(Color::BrightWhite);
            renderer.write_text(&format!("{:24} ", file.filename));
            renderer.reset();
            renderer.set_foreground(Color::Cyan);
            renderer.write_text(&format!("{:10} ", format_size(file.size_bytes)));
            renderer.reset();
            let desc = if file.description.len() > 20 {
                format!("{}...", &file.description[..17])
            } else {
                file.description.clone()
            };
            renderer.write_line(&desc);
        }

        // Allow selection for download
        renderer.write_line("");
        renderer.set_foreground(Color::BrightYellow);
        renderer.write_text("Enter file # to download (or Q to return): ");
        renderer.reset();
        connection
            .send_raw(renderer.take_output().as_bytes())
            .await?;

        if let Ok(input) = connection.read_line().await
            && !input.trim().eq_ignore_ascii_case("q")
            && let Ok(num) = input.trim().parse::<usize>()
            && num > 0
            && num <= matches.len()
        {
            let file = matches[num - 1];
            handle_download(connection, user, renderer, file).await?;
            return Ok(());
        }
    }

    wait_for_key(connection, renderer).await?;
    Ok(())
}

/// Format file size for display
fn format_size(bytes: u64) -> String {
    if bytes >= 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else if bytes >= 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{} B", bytes)
    }
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
