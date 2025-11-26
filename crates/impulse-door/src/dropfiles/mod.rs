//! Dropfile generation for BBS door games.
//!
//! This module provides support for generating various dropfile formats
//! used by BBS door games to receive user and system information.

pub mod dorinfo;
pub mod doorsys;
pub mod generator;

pub use dorinfo::DorinfoDropfile;
pub use doorsys::DoorSysDropfile;
pub use generator::{DropfileGenerator, DropfileType};
