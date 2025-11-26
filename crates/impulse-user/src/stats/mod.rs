//! User statistics tracking and display
//!
//! This module provides functionality for tracking and displaying user activity statistics.
//! It integrates with the `UserStats` type from `impulse-types` and provides high-level
//! operations for updating statistics through the `UserManager` trait.

pub mod display;
pub mod tracker;

pub use display::{format_ratio, format_stats_summary, format_time_online};
pub use tracker::StatsTracker;
