//! JAM kludge line parsing
//!
//! Kludge lines are control information embedded in message text,
//! typically at the beginning or end of the message.

use crate::types::KludgeLine;

/// Parse kludge lines from message text
///
/// Kludge lines start with ASCII 1 (SOH) or ^A and contain control information
/// like MSGID, REPLY, SEEN-BY, PATH, etc.
pub fn parse_kludges(text: &str) -> (Vec<KludgeLine>, String) {
    let mut kludges = Vec::new();
    let mut body_lines = Vec::new();
    let mut in_body = false;

    for line in text.lines() {
        if !in_body && line.starts_with('\x01') {
            // This is a kludge line
            let content = &line[1..]; // Remove SOH
            if let Some(pos) = content.find(':') {
                let kludge_type = content[..pos].trim().to_string();
                let value = content[pos + 1..].trim().to_string();
                kludges.push(KludgeLine { kludge_type, value });
            } else if let Some(pos) = content.find(' ') {
                let kludge_type = content[..pos].trim().to_string();
                let value = content[pos + 1..].trim().to_string();
                kludges.push(KludgeLine { kludge_type, value });
            } else {
                let kludge_type = content.trim().to_string();
                kludges.push(KludgeLine {
                    kludge_type,
                    value: String::new(),
                });
            }
        } else {
            in_body = true;
            body_lines.push(line);
        }
    }

    // Check for trailing kludges (after body)
    let body = body_lines.join("\n");
    let mut final_body = body.clone();

    // Common trailing kludge patterns
    if let Some(pos) = body.rfind("--- ") {
        // Tearline
        let rest = &body[pos..];
        if rest.lines().count() <= 3 {
            // Likely tearline and origin
            final_body = body[..pos].trim_end().to_string();
            for line in rest.lines() {
                if line.starts_with("---") || line.starts_with(" * Origin:") {
                    kludges.push(KludgeLine {
                        kludge_type: "TEARLINE".to_string(),
                        value: line.to_string(),
                    });
                }
            }
        }
    }

    (kludges, final_body)
}

/// Parse MSGID kludge
///
/// Format: "MSGID: <zone>:<net>/<node>.<point> <serial>"
/// Example: "MSGID: 1:234/567 12345678"
pub fn parse_msgid(value: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = value.split_whitespace().collect();
    if parts.len() >= 2 {
        Some((parts[0].to_string(), parts[1].to_string()))
    } else {
        None
    }
}

/// Parse REPLY kludge (same format as MSGID)
pub fn parse_reply(value: &str) -> Option<(String, String)> {
    parse_msgid(value)
}

/// Parse SEEN-BY kludge
///
/// Format: "SEEN-BY: <net>/<node> <net>/<node> ..."
/// Example: "SEEN-BY: 234/567 234/890"
pub fn parse_seen_by(value: &str) -> Vec<String> {
    value.split_whitespace().map(|s| s.to_string()).collect()
}

/// Parse PATH kludge (same format as SEEN-BY)
pub fn parse_path(value: &str) -> Vec<String> {
    parse_seen_by(value)
}

/// Parse INTL kludge
///
/// Format: "INTL <dest_zone>:<dest_net>/<dest_node> <orig_zone>:<orig_net>/<orig_node>"
/// Example: "INTL 1:234/567 1:234/890"
pub fn parse_intl(value: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = value.split_whitespace().collect();
    if parts.len() >= 2 {
        Some((parts[0].to_string(), parts[1].to_string()))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_kludges() {
        let text = "\x01MSGID: 1:234/567 12345678\n\
                    \x01REPLY: 1:234/567 12345677\n\
                    \x01PID: Impulse 7.1\n\
                    This is the message body.\n\
                    With multiple lines.\n\
                    --- Impulse 7.1\n\
                     * Origin: Test BBS (1:234/567)";

        let (kludges, body) = parse_kludges(text);

        // We get 4 kludges: MSGID, REPLY, PID, and 2 TEARLINE entries
        assert!(kludges.len() >= 3); // At least MSGID, REPLY, PID
        assert!(kludges.iter().any(|k| k.kludge_type == "MSGID"));
        assert!(kludges.iter().any(|k| k.kludge_type == "REPLY"));
        assert!(kludges.iter().any(|k| k.kludge_type == "PID"));
        assert!(body.contains("This is the message body"));
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
    fn test_parse_seen_by() {
        let result = parse_seen_by("234/567 234/890 234/999");
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "234/567");
        assert_eq!(result[1], "234/890");
        assert_eq!(result[2], "234/999");
    }

    #[test]
    fn test_parse_intl() {
        let result = parse_intl("1:234/567 1:234/890");
        assert_eq!(
            result,
            Some(("1:234/567".to_string(), "1:234/890".to_string()))
        );
    }
}
