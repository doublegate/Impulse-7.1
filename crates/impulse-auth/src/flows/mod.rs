//! User authentication flows
//!
//! This module provides high-level authentication workflows that coordinate
//! multiple components (validation, rate limiting, lockout, session management)
//! to implement complete user-facing authentication operations.
//!
//! # Modules
//!
//! - [`login`] - Login flow with credentials verification
//! - [`register`] - New user registration workflow
//! - [`logout`] - Session termination flow

pub mod login;
pub mod logout;
pub mod register;
