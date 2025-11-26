//! Message routing infrastructure
//!
//! This module provides routing capabilities for networked message systems,
//! particularly FidoNet-style addressing and routing decisions.
//!
//! # Examples
//!
//! ```
//! use impulse_message::routing::{MessageRouter, RouterConfig, RoutingDecision};
//! use impulse_message::addressing::FidoAddress;
//!
//! let local = FidoAddress::node(1, 234, 567);
//! let hub = FidoAddress::node(1, 234, 0);
//! let gate = FidoAddress::node(1, 234, 1);
//!
//! let config = RouterConfig::new(local)
//!     .with_hub(hub)
//!     .with_gate(gate);
//!
//! let router = MessageRouter::new(config);
//!
//! // Route a message to different net
//! let destination = FidoAddress::node(1, 999, 123);
//! let decision = router.route(destination);
//!
//! match decision {
//!     RoutingDecision::Local => println!("Local delivery"),
//!     RoutingDecision::DirectRoute { destination } => {
//!         println!("Route directly to {}", destination);
//!     }
//!     RoutingDecision::ViaHub { hub, destination } => {
//!         println!("Route via hub {} to {}", hub, destination);
//!     }
//!     RoutingDecision::ViaGate { gate, destination } => {
//!         println!("Route via gateway {} to {}", gate, destination);
//!     }
//!     RoutingDecision::Unroutable { reason } => {
//!         println!("Cannot route: {}", reason);
//!     }
//! }
//! ```

pub mod router;

pub use router::{MessageRouter, RouterConfig, RoutingDecision};
