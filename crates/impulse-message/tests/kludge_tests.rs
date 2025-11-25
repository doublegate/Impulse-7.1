//! Kludge line parsing tests

use impulse_message::formats::jam::{
    parse_intl, parse_kludges, parse_msgid, parse_path, parse_reply, parse_seen_by,
};

#[test]
fn test_parse_kludges_simple() {
    let text = "\x01MSGID: 1:234/567 12345678\n\
                \x01REPLY: 1:234/567 12345677\n\
                This is the body.";

    let (kludges, body) = parse_kludges(text);

    assert_eq!(kludges.len(), 2);
    assert_eq!(kludges[0].kludge_type, "MSGID");
    assert_eq!(kludges[0].value, "1:234/567 12345678");
    assert_eq!(kludges[1].kludge_type, "REPLY");
    assert_eq!(kludges[1].value, "1:234/567 12345677");
    assert!(body.contains("This is the body"));
}

#[test]
fn test_parse_kludges_with_tearline() {
    let text = "\x01MSGID: 1:234/567 12345678\n\
                Message body here.\n\
                --- Impulse 7.1\n\
                 * Origin: Test BBS (1:234/567)";

    let (kludges, body) = parse_kludges(text);

    assert!(kludges.iter().any(|k| k.kludge_type == "MSGID"));
    assert!(kludges.iter().any(|k| k.kludge_type == "TEARLINE"));
    assert!(body.contains("Message body here"));
}

#[test]
fn test_parse_kludges_no_kludges() {
    let text = "This is just a plain message\nwith no kludges.";

    let (kludges, body) = parse_kludges(text);

    assert_eq!(kludges.len(), 0);
    assert_eq!(body, text);
}

#[test]
fn test_parse_msgid() {
    let result = parse_msgid("1:234/567 12345678");
    assert_eq!(
        result,
        Some(("1:234/567".to_string(), "12345678".to_string()))
    );
}

#[test]
fn test_parse_msgid_invalid() {
    let result = parse_msgid("invalid");
    assert_eq!(result, None);
}

#[test]
fn test_parse_reply() {
    let result = parse_reply("1:234/567.89 abcdef01");
    assert_eq!(
        result,
        Some(("1:234/567.89".to_string(), "abcdef01".to_string()))
    );
}

#[test]
fn test_parse_seen_by() {
    let result = parse_seen_by("234/567 234/890 234/999");
    assert_eq!(result.len(), 3);
    assert_eq!(result[0], "234/567");
    assert_eq!(result[1], "234/890");
    assert_eq!(result[2], "234/999");
}

#[test]
fn test_parse_seen_by_single() {
    let result = parse_seen_by("234/567");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], "234/567");
}

#[test]
fn test_parse_path() {
    let result = parse_path("234/1 234/2 234/3");
    assert_eq!(result.len(), 3);
}

#[test]
fn test_parse_intl() {
    let result = parse_intl("1:234/567 1:234/890");
    assert_eq!(
        result,
        Some(("1:234/567".to_string(), "1:234/890".to_string()))
    );
}

#[test]
fn test_parse_intl_invalid() {
    let result = parse_intl("invalid");
    assert_eq!(result, None);
}
