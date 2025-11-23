//! Shared data types for Impulse 7.1 BBS

#![warn(missing_docs)]
#![warn(clippy::all)]

/// BBS configuration types
pub mod config;

/// User data types
pub mod user;

/// Message data types
pub mod message;

/// File data types
pub mod file;

/// Session data types
pub mod session;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
