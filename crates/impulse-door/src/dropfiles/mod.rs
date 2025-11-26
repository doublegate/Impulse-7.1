//! Dropfile generation for BBS door games.
//!
//! This module provides support for generating various dropfile formats
//! used by BBS door games to receive user and system information.

pub mod doorsys;
pub mod dorinfo;
pub mod generator;

pub use doorsys::DoorSysDropfile;
pub use dorinfo::DorinfoDropfile;
pub use generator::{DropfileGenerator, DropfileType};
