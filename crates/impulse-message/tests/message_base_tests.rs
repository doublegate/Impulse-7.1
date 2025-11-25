//! Integration tests for message base functionality

use chrono::Utc;
use impulse_message::types::{MessageHeader, SearchCriteria};

#[test]
fn test_message_header_creation() {
    let header = MessageHeader {
        msg_num: 1,
        from: "Alice".to_string(),
        to: "Bob".to_string(),
        subject: "Test Subject".to_string(),
        date: Utc::now(),
        is_read: false,
        is_private: false,
        reply_to: None,
        reply_count: 0,
    };

    assert_eq!(header.msg_num, 1);
    assert_eq!(header.from, "Alice");
    assert_eq!(header.to, "Bob");
    assert!(!header.is_public());
    assert!(!header.is_reply());
    assert!(!header.has_replies());
}

#[test]
fn test_message_header_public() {
    let mut header = MessageHeader {
        msg_num: 1,
        from: "Alice".to_string(),
        to: "All".to_string(),
        subject: "Public Message".to_string(),
        date: Utc::now(),
        is_read: false,
        is_private: false,
        reply_to: None,
        reply_count: 0,
    };

    assert!(header.is_public());

    header.to = "all".to_string();
    assert!(header.is_public());

    header.to = "Bob".to_string();
    assert!(!header.is_public());
}

#[test]
fn test_message_header_reply() {
    let header = MessageHeader {
        msg_num: 2,
        from: "Bob".to_string(),
        to: "Alice".to_string(),
        subject: "Re: Test".to_string(),
        date: Utc::now(),
        is_read: false,
        is_private: false,
        reply_to: Some(1),
        reply_count: 0,
    };

    assert!(header.is_reply());
    assert!(!header.has_replies());
}

#[test]
fn test_message_header_with_replies() {
    let header = MessageHeader {
        msg_num: 1,
        from: "Alice".to_string(),
        to: "All".to_string(),
        subject: "Original".to_string(),
        date: Utc::now(),
        is_read: true,
        is_private: false,
        reply_to: None,
        reply_count: 3,
    };

    assert!(!header.is_reply());
    assert!(header.has_replies());
}

#[test]
fn test_search_criteria_builder() {
    let criteria = SearchCriteria::new()
        .with_subject("test")
        .with_from("Alice")
        .unread_only();

    assert_eq!(criteria.subject, Some("test".to_string()));
    assert_eq!(criteria.from, Some("Alice".to_string()));
    assert!(criteria.unread_only);
    assert!(!criteria.private_only);
}

#[test]
fn test_search_criteria_default() {
    let criteria = SearchCriteria::default();

    assert!(criteria.subject.is_none());
    assert!(criteria.from.is_none());
    assert!(criteria.to.is_none());
    assert!(criteria.body.is_none());
    assert!(!criteria.unread_only);
    assert!(!criteria.private_only);
}
