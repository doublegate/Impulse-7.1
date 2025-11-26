//! Message routing infrastructure
//!
//! Determines how messages should be routed based on source and destination addresses.

use crate::addressing::FidoAddress;

/// Message routing decision
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoutingDecision {
    /// Message is local (same node)
    Local,
    /// Route directly to destination
    DirectRoute {
        /// Destination address
        destination: FidoAddress,
    },
    /// Route via hub/host
    ViaHub {
        /// Hub address to route through
        hub: FidoAddress,
        /// Final destination
        destination: FidoAddress,
    },
    /// Route via gateway (different zone)
    ViaGate {
        /// Gateway address
        gate: FidoAddress,
        /// Final destination
        destination: FidoAddress,
    },
    /// Message cannot be routed
    Unroutable {
        /// Reason for routing failure
        reason: String,
    },
}

impl RoutingDecision {
    /// Check if this is a local message
    pub fn is_local(&self) -> bool {
        matches!(self, RoutingDecision::Local)
    }

    /// Check if routing requires a hub
    pub fn requires_hub(&self) -> bool {
        matches!(self, RoutingDecision::ViaHub { .. })
    }

    /// Check if routing requires a gateway
    pub fn requires_gateway(&self) -> bool {
        matches!(self, RoutingDecision::ViaGate { .. })
    }

    /// Check if message is routable
    pub fn is_routable(&self) -> bool {
        !matches!(self, RoutingDecision::Unroutable { .. })
    }

    /// Get the next hop address (if applicable)
    pub fn next_hop(&self) -> Option<FidoAddress> {
        match self {
            RoutingDecision::DirectRoute { destination } => Some(*destination),
            RoutingDecision::ViaHub { hub, .. } => Some(*hub),
            RoutingDecision::ViaGate { gate, .. } => Some(*gate),
            _ => None,
        }
    }

    /// Get the final destination address (if applicable)
    pub fn final_destination(&self) -> Option<FidoAddress> {
        match self {
            RoutingDecision::DirectRoute { destination }
            | RoutingDecision::ViaHub { destination, .. }
            | RoutingDecision::ViaGate { destination, .. } => Some(*destination),
            _ => None,
        }
    }
}

/// Message router configuration
#[derive(Debug, Clone)]
pub struct RouterConfig {
    /// Local node address
    pub local_address: FidoAddress,
    /// Hub address (for routing to other nets in same zone)
    pub hub_address: Option<FidoAddress>,
    /// Gateway address (for routing to other zones)
    pub gate_address: Option<FidoAddress>,
}

impl RouterConfig {
    /// Create a new router configuration
    pub fn new(local_address: FidoAddress) -> Self {
        Self {
            local_address,
            hub_address: None,
            gate_address: None,
        }
    }

    /// Set the hub address
    pub fn with_hub(mut self, hub: FidoAddress) -> Self {
        self.hub_address = Some(hub);
        self
    }

    /// Set the gateway address
    pub fn with_gate(mut self, gate: FidoAddress) -> Self {
        self.gate_address = Some(gate);
        self
    }
}

/// Message router
///
/// Determines how messages should be routed based on FidoNet addressing.
pub struct MessageRouter {
    config: RouterConfig,
}

impl MessageRouter {
    /// Create a new message router
    pub fn new(config: RouterConfig) -> Self {
        Self { config }
    }

    /// Determine routing for a message
    pub fn route(&self, destination: FidoAddress) -> RoutingDecision {
        // Check if local
        if self.config.local_address.is_local(&destination) {
            return RoutingDecision::Local;
        }

        // Check if same zone
        if self.config.local_address.is_same_zone(&destination) {
            // Same zone, check if same net
            if self.config.local_address.is_same_net(&destination) {
                // Same net, route directly
                RoutingDecision::DirectRoute { destination }
            } else {
                // Different net, route via hub if available
                if let Some(hub) = self.config.hub_address {
                    RoutingDecision::ViaHub { hub, destination }
                } else {
                    RoutingDecision::Unroutable {
                        reason: "No hub configured for inter-net routing".to_string(),
                    }
                }
            }
        } else {
            // Different zone, route via gateway
            if let Some(gate) = self.config.gate_address {
                RoutingDecision::ViaGate { gate, destination }
            } else {
                RoutingDecision::Unroutable {
                    reason: "No gateway configured for inter-zone routing".to_string(),
                }
            }
        }
    }

    /// Check if a destination is directly reachable
    pub fn is_directly_reachable(&self, destination: FidoAddress) -> bool {
        matches!(
            self.route(destination),
            RoutingDecision::Local | RoutingDecision::DirectRoute { .. }
        )
    }

    /// Get the local address
    pub fn local_address(&self) -> FidoAddress {
        self.config.local_address
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_routing() {
        let local = FidoAddress::node(1, 234, 567);
        let config = RouterConfig::new(local);
        let router = MessageRouter::new(config);

        // Same node should be local
        let decision = router.route(local);
        assert_eq!(decision, RoutingDecision::Local);
        assert!(decision.is_local());

        // Point on same node should be local
        let point = FidoAddress::point(1, 234, 567, 89);
        let decision = router.route(point);
        assert_eq!(decision, RoutingDecision::Local);
    }

    #[test]
    fn test_direct_routing() {
        let local = FidoAddress::node(1, 234, 567);
        let config = RouterConfig::new(local);
        let router = MessageRouter::new(config);

        // Same net, different node should route directly
        let destination = FidoAddress::node(1, 234, 999);
        let decision = router.route(destination);

        assert_eq!(decision, RoutingDecision::DirectRoute { destination });
        assert!(decision.is_routable());
        assert_eq!(decision.next_hop(), Some(destination));
    }

    #[test]
    fn test_hub_routing() {
        let local = FidoAddress::node(1, 234, 567);
        let hub = FidoAddress::node(1, 234, 0);
        let config = RouterConfig::new(local).with_hub(hub);
        let router = MessageRouter::new(config);

        // Different net, should route via hub
        let destination = FidoAddress::node(1, 999, 123);
        let decision = router.route(destination);

        assert_eq!(decision, RoutingDecision::ViaHub { hub, destination });
        assert!(decision.requires_hub());
        assert_eq!(decision.next_hop(), Some(hub));
        assert_eq!(decision.final_destination(), Some(destination));
    }

    #[test]
    fn test_gateway_routing() {
        let local = FidoAddress::node(1, 234, 567);
        let gate = FidoAddress::node(1, 234, 1);
        let config = RouterConfig::new(local).with_gate(gate);
        let router = MessageRouter::new(config);

        // Different zone, should route via gateway
        let destination = FidoAddress::node(2, 5030, 1997);
        let decision = router.route(destination);

        assert_eq!(decision, RoutingDecision::ViaGate { gate, destination });
        assert!(decision.requires_gateway());
        assert_eq!(decision.next_hop(), Some(gate));
        assert_eq!(decision.final_destination(), Some(destination));
    }

    #[test]
    fn test_unroutable_no_hub() {
        let local = FidoAddress::node(1, 234, 567);
        let config = RouterConfig::new(local); // No hub configured
        let router = MessageRouter::new(config);

        // Different net, no hub configured
        let destination = FidoAddress::node(1, 999, 123);
        let decision = router.route(destination);

        assert!(matches!(decision, RoutingDecision::Unroutable { .. }));
        assert!(!decision.is_routable());
    }

    #[test]
    fn test_unroutable_no_gateway() {
        let local = FidoAddress::node(1, 234, 567);
        let config = RouterConfig::new(local); // No gateway configured
        let router = MessageRouter::new(config);

        // Different zone, no gateway configured
        let destination = FidoAddress::node(2, 5030, 1997);
        let decision = router.route(destination);

        assert!(matches!(decision, RoutingDecision::Unroutable { .. }));
        assert!(!decision.is_routable());
    }

    #[test]
    fn test_is_directly_reachable() {
        let local = FidoAddress::node(1, 234, 567);
        let config = RouterConfig::new(local);
        let router = MessageRouter::new(config);

        // Local should be directly reachable
        assert!(router.is_directly_reachable(local));

        // Same net should be directly reachable
        let same_net = FidoAddress::node(1, 234, 999);
        assert!(router.is_directly_reachable(same_net));

        // Different net should not be directly reachable
        let diff_net = FidoAddress::node(1, 999, 123);
        assert!(!router.is_directly_reachable(diff_net));
    }

    #[test]
    fn test_routing_decision_methods() {
        let dest = FidoAddress::node(1, 234, 567);

        let local = RoutingDecision::Local;
        assert!(local.is_local());
        assert!(local.is_routable());
        assert_eq!(local.next_hop(), None);

        let direct = RoutingDecision::DirectRoute { destination: dest };
        assert!(!direct.is_local());
        assert!(direct.is_routable());
        assert_eq!(direct.next_hop(), Some(dest));

        let unroutable = RoutingDecision::Unroutable {
            reason: "test".to_string(),
        };
        assert!(!unroutable.is_routable());
    }

    #[test]
    fn test_router_local_address() {
        let local = FidoAddress::node(1, 234, 567);
        let config = RouterConfig::new(local);
        let router = MessageRouter::new(config);

        assert_eq!(router.local_address(), local);
    }
}
