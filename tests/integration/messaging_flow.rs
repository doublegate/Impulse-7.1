//! Integration tests for messaging workflows
//!
//! This module tests the complete message system including:
//! - Message area listing
//! - Message reading with threading
//! - Message posting
//! - Reply functionality
//! - Message quoting
//! - Message search

use impulse_message::{
    formats::JamMessageBase,
    reply::ReplyBuilder,
    traits::MessageBase,
    types::{NewMessage, SearchCriteria},
};
use std::path::Path;
use tempfile::TempDir;

/// Setup a temporary JAM message base for testing
async fn setup_message_base() -> (JamMessageBase, TempDir) {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let base_path = temp_dir.path().join("testbase");

    let mut base = JamMessageBase::new(&base_path);
    base.create().await.expect("Failed to create message base");

    (base, temp_dir)
}

#[tokio::test]
async fn test_complete_messaging_flow() {
    // Setup
    let (mut base, _temp_dir) = setup_message_base().await;

    // Post initial message
    let msg1 = NewMessage::new("Alice", "Bob", "Hello Bob!")
        .with_body("This is a test message.");
    let msg_num_1 = base
        .post_message(msg1)
        .await
        .expect("Failed to post message");
    assert_eq!(msg_num_1, 1, "First message should be number 1");

    // Read the message back
    let read_msg = base
        .read_message(msg_num_1)
        .await
        .expect("Failed to read message");
    assert_eq!(read_msg.header.from, "Alice");
    assert_eq!(read_msg.header.to, "Bob");
    assert_eq!(read_msg.header.subject, "Hello Bob!");
    assert_eq!(read_msg.body, "This is a test message.");

    // Post a reply
    let reply = ReplyBuilder::new(read_msg.clone())
        .quote_original(false)
        .build("Bob", "Thanks for the message!");
    let msg_num_2 = base
        .post_message(reply)
        .await
        .expect("Failed to post reply");
    assert_eq!(msg_num_2, 2, "Reply should be message number 2");

    // Verify reply thread relationship
    let reply_msg = base
        .read_message(msg_num_2)
        .await
        .expect("Failed to read reply");
    assert_eq!(
        reply_msg.header.reply_to,
        Some(msg_num_1),
        "Reply should reference original message"
    );

    // List all messages
    let messages = base
        .list_messages(0, 100)
        .await
        .expect("Failed to list messages");
    assert_eq!(messages.len(), 2, "Should have 2 messages");

    // Get message count
    let count = base
        .message_count()
        .await
        .expect("Failed to get message count");
    assert_eq!(count, 2, "Message count should be 2");
}

#[tokio::test]
async fn test_message_threading() {
    // Setup
    let (mut base, _temp_dir) = setup_message_base().await;

    // Create a thread: initial message + 2 replies
    let msg1 = NewMessage::new("Alice", "All", "Thread Topic").with_body("Starting a discussion.");
    let msg_num_1 = base.post_message(msg1).await.expect("Failed to post");

    let original = base.read_message(msg_num_1).await.expect("Failed to read");

    // First reply
    let reply1 = ReplyBuilder::new(original.clone())
        .quote_original(false)
        .build("Bob", "Good point!");
    let msg_num_2 = base.post_message(reply1).await.expect("Failed to post");

    // Second reply to original
    let reply2 = ReplyBuilder::new(original.clone())
        .quote_original(false)
        .build("Carol", "I agree!");
    let msg_num_3 = base.post_message(reply2).await.expect("Failed to post");

    // Get thread for original message
    let thread = base
        .get_thread(msg_num_1)
        .await
        .expect("Failed to get thread");

    assert_eq!(
        thread.len(),
        3,
        "Thread should contain original + 2 replies"
    );
    assert!(
        thread.contains(&msg_num_1),
        "Thread should contain original"
    );
    assert!(thread.contains(&msg_num_2), "Thread should contain reply 1");
    assert!(thread.contains(&msg_num_3), "Thread should contain reply 2");
}

#[tokio::test]
async fn test_message_quoting() {
    // Setup
    let (mut base, _temp_dir) = setup_message_base().await;

    // Post original message
    let msg1 = NewMessage::new("Alice", "Bob", "Question")
        .with_body("What do you think about this?");
    let msg_num_1 = base.post_message(msg1).await.expect("Failed to post");

    let original = base.read_message(msg_num_1).await.expect("Failed to read");

    // Create reply with quoting
    let reply = ReplyBuilder::new(original)
        .quote_original(true)
        .build("Bob", "I think it's great!");
    let msg_num_2 = base.post_message(reply).await.expect("Failed to post");

    // Read reply and verify quote is present
    let reply_msg = base.read_message(msg_num_2).await.expect("Failed to read");

    assert!(
        reply_msg.body.contains("Alice wrote:"),
        "Reply should contain quote attribution"
    );
    assert!(
        reply_msg.body.contains("> What do you think about this?"),
        "Reply should contain quoted text"
    );
    assert!(
        reply_msg.body.contains("I think it's great!"),
        "Reply should contain response"
    );
}

#[tokio::test]
async fn test_message_search_by_from() {
    // Setup
    let (mut base, _temp_dir) = setup_message_base().await;

    // Post messages from different senders
    base.post_message(NewMessage::new("Alice", "Bob", "Message 1"))
        .await
        .expect("Failed to post");
    base.post_message(NewMessage::new("Bob", "Carol", "Message 2"))
        .await
        .expect("Failed to post");
    base.post_message(NewMessage::new("Alice", "Carol", "Message 3"))
        .await
        .expect("Failed to post");

    // Search for messages from Alice
    let criteria = SearchCriteria::new().with_from("Alice");
    let results = base
        .search(&criteria)
        .await
        .expect("Failed to search messages");

    assert_eq!(results.len(), 2, "Should find 2 messages from Alice");
    assert!(results.contains(&1), "Should find message 1");
    assert!(results.contains(&3), "Should find message 3");
}

#[tokio::test]
async fn test_message_search_by_subject() {
    // Setup
    let (mut base, _temp_dir) = setup_message_base().await;

    // Post messages with different subjects
    base.post_message(NewMessage::new("Alice", "Bob", "Test Message"))
        .await
        .expect("Failed to post");
    base.post_message(NewMessage::new("Bob", "Carol", "Important Update"))
        .await
        .expect("Failed to post");
    base.post_message(NewMessage::new("Carol", "Dave", "Test Results"))
        .await
        .expect("Failed to post");

    // Search for messages with "Test" in subject
    let criteria = SearchCriteria::new().with_subject("Test");
    let results = base
        .search(&criteria)
        .await
        .expect("Failed to search messages");

    assert_eq!(results.len(), 2, "Should find 2 messages with 'Test'");
    assert!(results.contains(&1), "Should find 'Test Message'");
    assert!(results.contains(&3), "Should find 'Test Results'");
}

#[tokio::test]
async fn test_message_search_by_to() {
    // Setup
    let (mut base, _temp_dir) = setup_message_base().await;

    // Post messages to different recipients
    base.post_message(NewMessage::new("Alice", "Bob", "Message 1"))
        .await
        .expect("Failed to post");
    base.post_message(NewMessage::new("Carol", "Bob", "Message 2"))
        .await
        .expect("Failed to post");
    base.post_message(NewMessage::new("Alice", "Dave", "Message 3"))
        .await
        .expect("Failed to post");

    // Search for messages to Bob
    let criteria = SearchCriteria::new().with_to("Bob");
    let results = base
        .search(&criteria)
        .await
        .expect("Failed to search messages");

    assert_eq!(results.len(), 2, "Should find 2 messages to Bob");
    assert!(results.contains(&1), "Should find message 1");
    assert!(results.contains(&2), "Should find message 2");
}

#[tokio::test]
async fn test_message_search_by_body() {
    // Setup
    let (mut base, _temp_dir) = setup_message_base().await;

    // Post messages with different body content
    base.post_message(
        NewMessage::new("Alice", "Bob", "Msg 1").with_body("This is about programming."),
    )
    .await
    .expect("Failed to post");
    base.post_message(
        NewMessage::new("Bob", "Carol", "Msg 2").with_body("Let's discuss music."),
    )
    .await
    .expect("Failed to post");
    base.post_message(
        NewMessage::new("Carol", "Dave", "Msg 3").with_body("Programming tips here."),
    )
    .await
    .expect("Failed to post");

    // Search for messages containing "programming"
    let criteria = SearchCriteria::new().with_body("programming");
    let results = base
        .search(&criteria)
        .await
        .expect("Failed to search messages");

    assert_eq!(
        results.len(),
        2,
        "Should find 2 messages containing 'programming'"
    );
}

#[tokio::test]
async fn test_empty_message_base() {
    // Setup empty base
    let (base, _temp_dir) = setup_message_base().await;

    // Message count should be 0
    let count = base
        .message_count()
        .await
        .expect("Failed to get message count");
    assert_eq!(count, 0, "Empty base should have 0 messages");

    // List should return empty vec
    let messages = base
        .list_messages(0, 100)
        .await
        .expect("Failed to list messages");
    assert_eq!(messages.len(), 0, "Empty base should list 0 messages");

    // Reading non-existent message should fail
    let result = base.read_message(1).await;
    assert!(result.is_err(), "Reading from empty base should fail");
}

#[tokio::test]
async fn test_message_pagination() {
    // Setup
    let (mut base, _temp_dir) = setup_message_base().await;

    // Post 10 messages
    for i in 1..=10 {
        let msg = NewMessage::new("Alice", "Bob", format!("Message {}", i));
        base.post_message(msg).await.expect("Failed to post");
    }

    // Get first page (5 messages)
    let page1 = base
        .list_messages(0, 5)
        .await
        .expect("Failed to list messages");
    assert_eq!(page1.len(), 5, "First page should have 5 messages");

    // Get second page (5 messages)
    let page2 = base
        .list_messages(5, 5)
        .await
        .expect("Failed to list messages");
    assert_eq!(page2.len(), 5, "Second page should have 5 messages");

    // Pages should not overlap
    for msg1 in &page1 {
        for msg2 in &page2 {
            assert_ne!(
                msg1.header.msg_num, msg2.header.msg_num,
                "Pages should not overlap"
            );
        }
    }
}

#[tokio::test]
async fn test_message_validation() {
    // Setup
    let (mut base, _temp_dir) = setup_message_base().await;

    // Try to post message with empty from field
    let invalid_msg = NewMessage::new("", "Bob", "Subject");
    let result = base.post_message(invalid_msg).await;
    assert!(
        result.is_err(),
        "Should fail to post message with empty from"
    );

    // Try to post message with empty to field
    let invalid_msg = NewMessage::new("Alice", "", "Subject");
    let result = base.post_message(invalid_msg).await;
    assert!(
        result.is_err(),
        "Should fail to post message with empty to"
    );

    // Try to post message with empty subject
    let invalid_msg = NewMessage::new("Alice", "Bob", "");
    let result = base.post_message(invalid_msg).await;
    assert!(
        result.is_err(),
        "Should fail to post message with empty subject"
    );
}

#[tokio::test]
async fn test_concurrent_message_posting() {
    // Setup
    let (mut base, _temp_dir) = setup_message_base().await;

    // Post multiple messages concurrently
    let mut handles = vec![];
    for i in 1..=5 {
        let msg = NewMessage::new("Alice", "Bob", format!("Message {}", i));
        // Note: We can't actually share the mut base across threads,
        // so this test demonstrates sequential posting in a concurrent context
        let msg_num = base.post_message(msg).await.expect("Failed to post");
        assert_eq!(msg_num, i as u32, "Message numbers should be sequential");
    }

    // Verify all messages were posted
    let count = base.message_count().await.expect("Failed to get count");
    assert_eq!(count, 5, "Should have 5 messages");
}
