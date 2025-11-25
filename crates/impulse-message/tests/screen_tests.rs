//! Screen rendering tests

use impulse_message::screens::{MessageListConfig, MessageReadConfig, MessageReadScreen};

#[test]
fn test_message_list_config_default() {
    let config = MessageListConfig::default();

    assert_eq!(config.messages_per_page, 20);
    assert!(config.show_numbers);
    assert!(config.show_read_status);
    assert!(config.show_private_status);
}

#[test]
fn test_message_read_config_default() {
    let config = MessageReadConfig::default();

    assert_eq!(config.wrap_width, 79);
    assert!(!config.show_kludges);
    assert!(config.show_thread);
    assert!(config.highlight_quotes);
}

#[test]
fn test_message_read_screen_no_message() {
    let screen = MessageReadScreen::default_config();

    assert!(screen.message().is_none());
    assert!(screen.thread().is_none());

    let output = screen.render();
    assert!(output.contains("No message loaded"));
}
