//! Message addressing support
//!
//! This module provides addressing support for various BBS networking standards,
//! including FidoNet addressing (zone:net/node.point format).

pub mod fidonet;

pub use fidonet::{FidoAddress, FidoAddressError};
