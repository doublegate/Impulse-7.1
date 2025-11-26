//! FidoNet address handling
//!
//! FidoNet addresses follow the format: zone:net/node.point
//! where zone, net, node, and point are all numeric.
//!
//! Examples:
//! - `1:234/567` - Zone 1, Net 234, Node 567 (no point)
//! - `1:234/567.89` - Zone 1, Net 234, Node 567, Point 89
//! - `2:5030/1997` - Zone 2, Net 5030, Node 1997

use std::fmt;
use std::str::FromStr;
use thiserror::Error;

/// FidoNet address parsing errors
#[derive(Debug, Error, PartialEq)]
pub enum FidoAddressError {
    /// Invalid address format
    #[error("Invalid FidoNet address format: {0}")]
    InvalidFormat(String),

    /// Invalid zone number
    #[error("Invalid zone number: {0}")]
    InvalidZone(String),

    /// Invalid net number
    #[error("Invalid net number: {0}")]
    InvalidNet(String),

    /// Invalid node number
    #[error("Invalid node number: {0}")]
    InvalidNode(String),

    /// Invalid point number
    #[error("Invalid point number: {0}")]
    InvalidPoint(String),
}

/// FidoNet address (zone:net/node.point)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FidoAddress {
    /// Zone number (1-6 for FidoNet)
    pub zone: u16,
    /// Net number
    pub net: u16,
    /// Node number
    pub node: u16,
    /// Point number (0 if not a point system)
    pub point: u16,
}

impl FidoAddress {
    /// Create a new FidoNet address
    pub fn new(zone: u16, net: u16, node: u16, point: u16) -> Self {
        Self {
            zone,
            net,
            node,
            point,
        }
    }

    /// Create a node address (point = 0)
    pub fn node(zone: u16, net: u16, node: u16) -> Self {
        Self::new(zone, net, node, 0)
    }

    /// Create a point address
    pub fn point(zone: u16, net: u16, node: u16, point: u16) -> Self {
        Self::new(zone, net, node, point)
    }

    /// Check if this is a point system
    pub fn is_point(&self) -> bool {
        self.point != 0
    }

    /// Check if this is a node (not a point)
    pub fn is_node(&self) -> bool {
        self.point == 0
    }

    /// Check if two addresses are in the same zone
    pub fn is_same_zone(&self, other: &FidoAddress) -> bool {
        self.zone == other.zone
    }

    /// Check if two addresses are in the same net
    pub fn is_same_net(&self, other: &FidoAddress) -> bool {
        self.zone == other.zone && self.net == other.net
    }

    /// Check if two addresses are the same node (ignoring point)
    pub fn is_same_node(&self, other: &FidoAddress) -> bool {
        self.zone == other.zone && self.net == other.net && self.node == other.node
    }

    /// Get the parent node address (if this is a point)
    pub fn parent_node(&self) -> Option<FidoAddress> {
        if self.is_point() {
            Some(FidoAddress::node(self.zone, self.net, self.node))
        } else {
            None
        }
    }

    /// Check if this address is local to another (same node and net)
    pub fn is_local(&self, other: &FidoAddress) -> bool {
        self.is_same_node(other)
    }
}

impl FromStr for FidoAddress {
    type Err = FidoAddressError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Parse format: zone:net/node.point or zone:net/node
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err(FidoAddressError::InvalidFormat(
                "expected format zone:net/node.point".to_string(),
            ));
        }

        let zone: u16 = parts[0]
            .parse()
            .map_err(|_| FidoAddressError::InvalidZone(parts[0].to_string()))?;

        let net_node_point = parts[1];
        let net_node: Vec<&str> = net_node_point.split('/').collect();
        if net_node.len() != 2 {
            return Err(FidoAddressError::InvalidFormat(
                "expected format zone:net/node.point".to_string(),
            ));
        }

        let net: u16 = net_node[0]
            .parse()
            .map_err(|_| FidoAddressError::InvalidNet(net_node[0].to_string()))?;

        // Check for point
        let node_point: Vec<&str> = net_node[1].split('.').collect();
        let node: u16 = node_point[0]
            .parse()
            .map_err(|_| FidoAddressError::InvalidNode(node_point[0].to_string()))?;

        let point: u16 = if node_point.len() > 1 {
            node_point[1]
                .parse()
                .map_err(|_| FidoAddressError::InvalidPoint(node_point[1].to_string()))?
        } else {
            0
        };

        Ok(FidoAddress::new(zone, net, node, point))
    }
}

impl fmt::Display for FidoAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_point() {
            write!(f, "{}:{}/{}.{}", self.zone, self.net, self.node, self.point)
        } else {
            write!(f, "{}:{}/{}", self.zone, self.net, self.node)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_node_address() {
        let addr: FidoAddress = "1:234/567".parse().unwrap();
        assert_eq!(addr.zone, 1);
        assert_eq!(addr.net, 234);
        assert_eq!(addr.node, 567);
        assert_eq!(addr.point, 0);
        assert!(addr.is_node());
        assert!(!addr.is_point());
    }

    #[test]
    fn test_parse_point_address() {
        let addr: FidoAddress = "1:234/567.89".parse().unwrap();
        assert_eq!(addr.zone, 1);
        assert_eq!(addr.net, 234);
        assert_eq!(addr.node, 567);
        assert_eq!(addr.point, 89);
        assert!(addr.is_point());
        assert!(!addr.is_node());
    }

    #[test]
    fn test_display_node() {
        let addr = FidoAddress::node(1, 234, 567);
        assert_eq!(addr.to_string(), "1:234/567");
    }

    #[test]
    fn test_display_point() {
        let addr = FidoAddress::point(1, 234, 567, 89);
        assert_eq!(addr.to_string(), "1:234/567.89");
    }

    #[test]
    fn test_roundtrip() {
        let original = "2:5030/1997.123";
        let addr: FidoAddress = original.parse().unwrap();
        assert_eq!(addr.to_string(), original);
    }

    #[test]
    fn test_invalid_format_no_colon() {
        let result: Result<FidoAddress, _> = "1234/567".parse();
        assert!(matches!(result, Err(FidoAddressError::InvalidFormat(_))));
    }

    #[test]
    fn test_invalid_format_no_slash() {
        let result: Result<FidoAddress, _> = "1:234567".parse();
        assert!(matches!(result, Err(FidoAddressError::InvalidFormat(_))));
    }

    #[test]
    fn test_invalid_zone() {
        let result: Result<FidoAddress, _> = "abc:234/567".parse();
        assert!(matches!(result, Err(FidoAddressError::InvalidZone(_))));
    }

    #[test]
    fn test_invalid_net() {
        let result: Result<FidoAddress, _> = "1:abc/567".parse();
        assert!(matches!(result, Err(FidoAddressError::InvalidNet(_))));
    }

    #[test]
    fn test_invalid_node() {
        let result: Result<FidoAddress, _> = "1:234/abc".parse();
        assert!(matches!(result, Err(FidoAddressError::InvalidNode(_))));
    }

    #[test]
    fn test_invalid_point() {
        let result: Result<FidoAddress, _> = "1:234/567.abc".parse();
        assert!(matches!(result, Err(FidoAddressError::InvalidPoint(_))));
    }

    #[test]
    fn test_same_zone() {
        let addr1 = FidoAddress::node(1, 234, 567);
        let addr2 = FidoAddress::node(1, 999, 123);
        let addr3 = FidoAddress::node(2, 234, 567);

        assert!(addr1.is_same_zone(&addr2));
        assert!(!addr1.is_same_zone(&addr3));
    }

    #[test]
    fn test_same_net() {
        let addr1 = FidoAddress::node(1, 234, 567);
        let addr2 = FidoAddress::node(1, 234, 999);
        let addr3 = FidoAddress::node(1, 999, 567);

        assert!(addr1.is_same_net(&addr2));
        assert!(!addr1.is_same_net(&addr3));
    }

    #[test]
    fn test_same_node() {
        let addr1 = FidoAddress::node(1, 234, 567);
        let addr2 = FidoAddress::point(1, 234, 567, 89);
        let addr3 = FidoAddress::node(1, 234, 999);

        assert!(addr1.is_same_node(&addr2));
        assert!(!addr1.is_same_node(&addr3));
    }

    #[test]
    fn test_parent_node() {
        let point = FidoAddress::point(1, 234, 567, 89);
        let parent = point.parent_node().unwrap();
        assert_eq!(parent, FidoAddress::node(1, 234, 567));

        let node = FidoAddress::node(1, 234, 567);
        assert_eq!(node.parent_node(), None);
    }

    #[test]
    fn test_is_local() {
        let addr1 = FidoAddress::node(1, 234, 567);
        let addr2 = FidoAddress::point(1, 234, 567, 89);
        let addr3 = FidoAddress::node(1, 234, 999);

        assert!(addr1.is_local(&addr2));
        assert!(!addr1.is_local(&addr3));
    }

    #[test]
    fn test_equality() {
        let addr1 = FidoAddress::node(1, 234, 567);
        let addr2 = FidoAddress::node(1, 234, 567);
        let addr3 = FidoAddress::point(1, 234, 567, 89);

        assert_eq!(addr1, addr2);
        assert_ne!(addr1, addr3);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(FidoAddress::node(1, 234, 567));
        set.insert(FidoAddress::node(1, 234, 567)); // Duplicate
        set.insert(FidoAddress::point(1, 234, 567, 89));

        assert_eq!(set.len(), 2);
    }
}
