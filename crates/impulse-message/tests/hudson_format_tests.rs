//! Hudson format parsing tests

use impulse_message::formats::hudson::HudsonAttributes;

#[test]
fn test_hudson_attributes() {
    let attrs = HudsonAttributes::new(
        HudsonAttributes::PRIVATE | HudsonAttributes::LOCAL | HudsonAttributes::UNREAD,
    );

    assert!(attrs.is_private());
    assert!(attrs.is_local());
    assert!(!attrs.is_read());
    assert!(!attrs.is_deleted());
}

#[test]
fn test_hudson_attributes_deleted() {
    let attrs = HudsonAttributes::new(HudsonAttributes::DELETED);

    assert!(attrs.is_deleted());
    assert!(!attrs.is_private());
    assert!(!attrs.is_local());
}

#[test]
fn test_hudson_attributes_read() {
    // UNREAD flag NOT set means it IS read
    let attrs = HudsonAttributes::new(HudsonAttributes::PRIVATE);

    assert!(attrs.is_read());
    assert!(attrs.is_private());
}

#[test]
fn test_hudson_attributes_unread() {
    // UNREAD flag set means it is NOT read
    let attrs = HudsonAttributes::new(HudsonAttributes::UNREAD);

    assert!(!attrs.is_read());
}

#[test]
fn test_hudson_attributes_netmail() {
    let attrs = HudsonAttributes::new(HudsonAttributes::NETMAIL | HudsonAttributes::PRIVATE);

    assert!(attrs.has(HudsonAttributes::NETMAIL));
    assert!(attrs.is_private());
}
