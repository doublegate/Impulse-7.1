//! Impulse 7.1 BBS Server
//!
//! Modern BBS server implementation in Rust

use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    info!("Impulse 7.1 BBS Server v0.1.0");
    info!("Initializing...");

    // TODO: Implement server initialization

    Ok(())
}
