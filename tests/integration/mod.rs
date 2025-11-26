//! Integration tests for Impulse-Next_BBS
//!
//! This module contains comprehensive end-to-end integration tests that verify
//! all Phase 2 features work together seamlessly.
//!
//! # Test Modules
//!
//! - [`auth_flow`] - Authentication workflow tests (register, login, logout)
//! - [`messaging_flow`] - Message system integration tests
//! - [`file_flow`] - File management integration tests
//! - [`profile_flow`] - User profile integration tests
//! - [`concurrent_access`] - Concurrent user simulation tests

mod auth_flow;
mod messaging_flow;
mod file_flow;
mod profile_flow;
mod concurrent_access;
