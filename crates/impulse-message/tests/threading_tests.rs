//! Message threading tests

use impulse_message::types::MessageThread;

#[test]
fn test_message_thread_creation() {
    let thread = MessageThread::new(1);

    assert_eq!(thread.root_msg, 1);
    assert!(thread.is_root());
    assert_eq!(thread.depth, 0);
    assert_eq!(thread.reply_count, 0);
    assert_eq!(thread.children.len(), 0);
    assert_eq!(thread.path, vec![1]);
}

#[test]
fn test_message_thread_add_child() {
    let mut thread = MessageThread::new(1);

    thread.add_child(2);
    assert_eq!(thread.children.len(), 1);
    assert_eq!(thread.children[0], 2);
    assert_eq!(thread.reply_count, 1);

    thread.add_child(3);
    assert_eq!(thread.children.len(), 2);
    assert_eq!(thread.reply_count, 2);
}

#[test]
fn test_message_thread_add_duplicate_child() {
    let mut thread = MessageThread::new(1);

    thread.add_child(2);
    thread.add_child(2); // Duplicate

    assert_eq!(thread.children.len(), 1);
    assert_eq!(thread.reply_count, 1);
}

#[test]
fn test_message_thread_with_parent() {
    let mut thread = MessageThread::new(2);
    thread.parent_id = Some(1);
    thread.depth = 1;
    thread.path = vec![1, 2];

    assert!(!thread.is_root());
    assert_eq!(thread.parent_id, Some(1));
    assert_eq!(thread.depth, 1);
    assert_eq!(thread.path, vec![1, 2]);
}

#[test]
fn test_message_thread_deep_nesting() {
    let mut thread = MessageThread::new(5);
    thread.parent_id = Some(4);
    thread.depth = 4;
    thread.path = vec![1, 2, 3, 4, 5];

    assert!(!thread.is_root());
    assert_eq!(thread.depth, 4);
    assert_eq!(thread.path.len(), 5);
}
