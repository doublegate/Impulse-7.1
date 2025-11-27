//! Message areas handler

use crate::state::ServerState;
use anyhow::Result;
use impulse_message::screens::{MessageListConfig, MessageListScreen, MessageReadScreen};
use impulse_message::traits::MessageBase;
use impulse_message::{NewMessage, ReplyBuilder};
use impulse_telnet::TelnetConnection;
use impulse_terminal::{AnsiRenderer, Color};
use impulse_types::user::User;

/// Handle messages menu
pub async fn handle_messages(
    connection: &mut TelnetConnection,
    _user: &User,
    state: &ServerState,
    renderer: &mut AnsiRenderer,
) -> Result<()> {
    loop {
        // Create message list screen
        let mut list_screen = MessageListScreen::new(MessageListConfig::default());

        // Load messages from the message base
        let message_base = state.message_base.read().await;
        match list_screen.load_page(&*message_base, 0).await {
            Ok(_) => {
                // Display message list
                renderer.clear_screen();
                renderer.set_foreground(Color::BrightCyan);
                renderer.write_line(
                    "╔══════════════════════════════════════════════════════════════════════════╗",
                );
                renderer.write_line(
                    "║                          MESSAGE AREAS                                   ║",
                );
                renderer.write_line(
                    "╚══════════════════════════════════════════════════════════════════════════╝",
                );
                renderer.reset();
                renderer.write_line("");

                // Show message count
                let total = list_screen.total_count();
                let page = list_screen.current_page() + 1;
                let max_page = list_screen.max_page() + 1;

                renderer.write_line(&format!(
                    "Total Messages: {}  |  Page {}/{}",
                    total, page, max_page
                ));
                renderer.write_line("");

                // Show messages
                for msg in list_screen.messages() {
                    renderer.set_foreground(Color::BrightYellow);
                    renderer.write_text(&format!("#{} ", msg.msg_num));
                    renderer.reset();
                    renderer.set_foreground(Color::BrightWhite);
                    renderer.write_text(&format!("From: {} ", msg.from));
                    renderer.reset();
                    renderer.set_foreground(Color::BrightGreen);
                    renderer.write_line(&format!("Subject: {}", msg.subject));
                    renderer.reset();
                }

                renderer.write_line("");
                renderer.set_foreground(Color::Yellow);
                renderer.write_line("Commands:");
                renderer.write_line("  [R] Read message  [N] Next page  [P] Previous page");
                renderer.write_line("  [W] Write new message  [Q] Return to main menu");
                renderer.reset();
                renderer.write_line("");
                renderer.set_foreground(Color::BrightYellow);
                renderer.write_text("Command: ");
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
                            'R' => {
                                // Read message - prompt for message number
                                renderer.write_line("\r\n");
                                renderer.set_foreground(Color::BrightYellow);
                                renderer.write_text("Enter message number to read: ");
                                renderer.reset();
                                connection
                                    .send_raw(renderer.take_output().as_bytes())
                                    .await?;

                                // Read message number
                                if let Ok(input) = connection.read_line().await
                                    && let Ok(msg_num) = input.trim().parse::<u32>()
                                {
                                    // Load and display message
                                    drop(message_base);
                                    let message_base = state.message_base.read().await;

                                    let mut read_screen = MessageReadScreen::default_config();
                                    match read_screen.load_message(&*message_base, msg_num).await {
                                        Ok(_) => {
                                            renderer.clear_screen();
                                            renderer.write_text(&read_screen.render());
                                            renderer.write_line("");
                                            renderer.write_line("");
                                            renderer.set_foreground(Color::BrightYellow);
                                            renderer.write_line(
                                                "Commands: [P]revious  [N]ext  [R]eply  [Q]uit",
                                            );
                                            renderer.reset();
                                            renderer.write_line("");
                                            renderer.set_foreground(Color::BrightYellow);
                                            renderer.write_text("Command: ");
                                            renderer.reset();
                                            connection
                                                .send_raw(renderer.take_output().as_bytes())
                                                .await?;

                                            // Read command
                                            if let Ok(ch) = connection.read_char().await {
                                                let subcmd = ch.to_ascii_uppercase();
                                                if subcmd == 'R' {
                                                    // Reply to message
                                                    if let Some(msg) = read_screen.message() {
                                                        handle_reply(
                                                            connection,
                                                            state,
                                                            _user,
                                                            renderer,
                                                            msg.header.msg_num,
                                                        )
                                                        .await?;
                                                    }
                                                }
                                            }
                                        }
                                        Err(_) => {
                                            renderer.write_line("\r\n");
                                            renderer.set_foreground(Color::BrightRed);
                                            renderer.write_line("Message not found.");
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
                            }
                            'W' => {
                                // Write new message
                                drop(message_base);
                                handle_new_message(connection, state, _user, renderer).await?;
                            }
                            'N' => {
                                // Next page
                                drop(message_base);
                                let message_base = state.message_base.read().await;
                                list_screen.next_page(&*message_base).await.ok();
                            }
                            'P' => {
                                // Previous page
                                drop(message_base);
                                let message_base = state.message_base.read().await;
                                list_screen.prev_page(&*message_base).await.ok();
                            }
                            'Q' => {
                                // Return to main menu
                                return Ok(());
                            }
                            _ => {
                                // Unknown command
                                renderer.write_line("\r\n");
                                renderer.set_foreground(Color::BrightRed);
                                renderer.write_line("Unknown command.");
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
            Err(e) => {
                // Error loading messages - check if it's a file not found error
                let error_msg = e.to_string();
                let is_not_found = error_msg.contains("No such file or directory")
                    || error_msg.contains("cannot find the path")
                    || error_msg.contains("not found");

                renderer.clear_screen();
                renderer.set_foreground(Color::BrightCyan);
                renderer.write_line(
                    "╔══════════════════════════════════════════════════════════════════════════╗",
                );
                renderer.write_line(
                    "║                          MESSAGE AREAS                                   ║",
                );
                renderer.write_line(
                    "╚══════════════════════════════════════════════════════════════════════════╝",
                );
                renderer.reset();
                renderer.write_line("");

                if is_not_found {
                    // Graceful message for empty/new message base
                    renderer.set_foreground(Color::BrightYellow);
                    renderer.write_line("No messages in this area yet.");
                    renderer.write_line("");
                    renderer.reset();
                    renderer
                        .write_line("This is a new message base. Be the first to post a message!");
                } else {
                    // Show error for other issues
                    renderer.set_foreground(Color::BrightRed);
                    renderer.write_line("Unable to access message area:");
                    renderer.write_line(&format!("  {}", e));
                    renderer.reset();
                }

                renderer.write_line("");
                renderer.set_foreground(Color::Yellow);
                renderer.write_line("Commands:");
                renderer.write_line("  [W] Write new message  [Q] Return to main menu");
                renderer.reset();
                renderer.write_line("");
                renderer.set_foreground(Color::BrightYellow);
                renderer.write_text("Command: ");
                renderer.reset();

                connection
                    .send_raw(renderer.take_output().as_bytes())
                    .await?;

                // Read command
                match connection.read_char().await {
                    Ok(ch) => {
                        let cmd = ch.to_ascii_uppercase();
                        if cmd == 'W' {
                            // Write new message
                            drop(message_base);
                            handle_new_message(connection, state, _user, renderer).await?;
                        } else {
                            // Return to main menu
                            return Ok(());
                        }
                    }
                    Err(_) => {
                        return Ok(());
                    }
                }
            }
        }
    }
}

/// Handle posting a new message
async fn handle_new_message(
    connection: &mut TelnetConnection,
    state: &ServerState,
    user: &User,
    renderer: &mut AnsiRenderer,
) -> Result<()> {
    renderer.clear_screen();
    renderer.set_foreground(Color::BrightCyan);
    renderer
        .write_line("═══════════════════════════════════════════════════════════════════════════");
    renderer
        .write_line("                          POST NEW MESSAGE                                 ");
    renderer
        .write_line("═══════════════════════════════════════════════════════════════════════════");
    renderer.reset();
    renderer.write_line("");

    // Get recipient
    renderer.set_foreground(Color::BrightYellow);
    renderer.write_text("To: ");
    renderer.reset();
    connection
        .send_raw(renderer.take_output().as_bytes())
        .await?;
    let to = connection.read_line().await?.trim().to_string();

    // Get subject
    renderer.set_foreground(Color::BrightYellow);
    renderer.write_text("Subject: ");
    renderer.reset();
    connection
        .send_raw(renderer.take_output().as_bytes())
        .await?;
    let subject = connection.read_line().await?.trim().to_string();

    // Get message body
    renderer.write_line("");
    renderer.set_foreground(Color::BrightWhite);
    renderer.write_line("Enter message body (blank line to end):");
    renderer.reset();
    connection
        .send_raw(renderer.take_output().as_bytes())
        .await?;

    let mut body_lines = Vec::new();
    while let Ok(line) = connection.read_line().await {
        if line.trim().is_empty() {
            break;
        }
        body_lines.push(line);
    }
    let body = body_lines.join("\n");

    // Post the message
    let message = NewMessage::new(user.username(), &to, &subject).with_body(&body);

    let mut message_base = state.message_base.write().await;
    match message_base.post_message(message).await {
        Ok(msg_num) => {
            renderer.write_line("");
            renderer.set_foreground(Color::BrightGreen);
            renderer.write_line(&format!("Message #{} posted successfully!", msg_num));
            renderer.reset();
        }
        Err(e) => {
            renderer.write_line("");
            renderer.set_foreground(Color::BrightRed);
            renderer.write_line(&format!("Error posting message: {}", e));
            renderer.reset();
        }
    }

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

/// Handle replying to a message
async fn handle_reply(
    connection: &mut TelnetConnection,
    state: &ServerState,
    user: &User,
    renderer: &mut AnsiRenderer,
    original_msg_num: u32,
) -> Result<()> {
    renderer.clear_screen();
    renderer.set_foreground(Color::BrightCyan);
    renderer
        .write_line("═══════════════════════════════════════════════════════════════════════════");
    renderer
        .write_line("                          REPLY TO MESSAGE                                 ");
    renderer
        .write_line("═══════════════════════════════════════════════════════════════════════════");
    renderer.reset();
    renderer.write_line("");

    // Read original message
    let message_base = state.message_base.read().await;
    let original = match message_base.read_message(original_msg_num).await {
        Ok(msg) => msg,
        Err(e) => {
            renderer.set_foreground(Color::BrightRed);
            renderer.write_line(&format!("Error reading message: {}", e));
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
    };
    drop(message_base);

    // Display original message info
    renderer.set_foreground(Color::BrightWhite);
    renderer.write_line(&format!(
        "Replying to message from: {}",
        original.header.from
    ));
    renderer.write_line(&format!("Subject: {}", original.header.subject));
    renderer.reset();
    renderer.write_line("");

    // Get reply body
    renderer.set_foreground(Color::BrightWhite);
    renderer.write_line("Enter reply (blank line to end):");
    renderer.reset();
    connection
        .send_raw(renderer.take_output().as_bytes())
        .await?;

    let mut body_lines = Vec::new();
    while let Ok(line) = connection.read_line().await {
        if line.trim().is_empty() {
            break;
        }
        body_lines.push(line);
    }
    let reply_text = body_lines.join("\n");

    // Build reply with quoting
    let reply_message = ReplyBuilder::new(original)
        .quote_original(true)
        .build(user.username(), &reply_text);

    // Post the reply
    let mut message_base = state.message_base.write().await;
    match message_base.post_message(reply_message).await {
        Ok(msg_num) => {
            renderer.write_line("");
            renderer.set_foreground(Color::BrightGreen);
            renderer.write_line(&format!("Reply #{} posted successfully!", msg_num));
            renderer.reset();
        }
        Err(e) => {
            renderer.write_line("");
            renderer.set_foreground(Color::BrightRed);
            renderer.write_line(&format!("Error posting reply: {}", e));
            renderer.reset();
        }
    }

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
