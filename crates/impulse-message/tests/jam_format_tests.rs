//! JAM format parsing tests

use impulse_message::formats::jam::{MessageAttributes, SubfieldType};

#[test]
fn test_message_attributes() {
    let attrs = MessageAttributes::new(
        MessageAttributes::PRIVATE | MessageAttributes::READ | MessageAttributes::LOCAL,
    );

    assert!(attrs.is_private());
    assert!(attrs.is_read());
    assert!(attrs.is_local());
    assert!(!attrs.has(MessageAttributes::SENT));
    assert!(!attrs.has(MessageAttributes::IN_TRANSIT));
}

#[test]
fn test_message_attributes_none() {
    let attrs = MessageAttributes::new(0);

    assert!(!attrs.is_private());
    assert!(!attrs.is_read());
    assert!(!attrs.is_local());
}

#[test]
fn test_message_attributes_all() {
    let attrs = MessageAttributes::new(0xFFFF);

    assert!(attrs.is_private());
    assert!(attrs.is_read());
    assert!(attrs.is_local());
    assert!(attrs.has(MessageAttributes::SENT));
    assert!(attrs.has(MessageAttributes::IN_TRANSIT));
}

#[test]
fn test_subfield_type_conversion() {
    assert_eq!(SubfieldType::from(0), SubfieldType::SendName);
    assert_eq!(SubfieldType::from(1), SubfieldType::RecvName);
    assert_eq!(SubfieldType::from(2), SubfieldType::SendAddr);
    assert_eq!(SubfieldType::from(3), SubfieldType::RecvAddr);
    assert_eq!(SubfieldType::from(4), SubfieldType::MsgId);
    assert_eq!(SubfieldType::from(5), SubfieldType::ReplyId);
    assert_eq!(SubfieldType::from(6), SubfieldType::Subject);
    assert_eq!(SubfieldType::from(7), SubfieldType::Path);
    assert_eq!(SubfieldType::from(8), SubfieldType::SeenBy);
    assert_eq!(SubfieldType::from(99), SubfieldType::Unknown);
    assert_eq!(SubfieldType::from(1000), SubfieldType::Unknown);
}
