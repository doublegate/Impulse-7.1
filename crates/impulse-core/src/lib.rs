//! Core BBS logic for Impulse 7.1

#![warn(missing_docs)]
#![warn(clippy::all)]

use async_trait::async_trait;
use impulse_types::session::Session;
use std::error::Error;

/// BBS core functionality trait
#[async_trait]
pub trait BbsCore: Send + Sync {
    /// Start the BBS server
    async fn start(&self) -> Result<(), Box<dyn Error>>;

    /// Stop the BBS server
    async fn stop(&self) -> Result<(), Box<dyn Error>>;

    /// Handle a new session
    async fn handle_session(&self, session: Session) -> Result<(), Box<dyn Error>>;
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_placeholder() {
        assert!(true);
    }
}
