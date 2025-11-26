//! Telnet server implementation

use crate::connection::TelnetConnection;
use crate::error::Result;
use std::net::SocketAddr;
use tokio::net::TcpListener;

/// Telnet server that accepts incoming connections
pub struct TelnetServer {
    /// TCP listener
    listener: TcpListener,
    /// Local bind address
    local_addr: SocketAddr,
}

impl TelnetServer {
    /// Bind to a local address
    ///
    /// # Example
    ///
    /// ```no_run
    /// use impulse_telnet::{TelnetServer, Result};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let server = TelnetServer::bind("0.0.0.0:2323").await?;
    ///     println!("Listening on {}", server.local_addr());
    ///     Ok(())
    /// }
    /// ```
    pub async fn bind(addr: &str) -> Result<Self> {
        let listener = TcpListener::bind(addr).await?;
        let local_addr = listener.local_addr()?;

        Ok(Self {
            listener,
            local_addr,
        })
    }

    /// Get the local address the server is bound to
    pub fn local_addr(&self) -> SocketAddr {
        self.local_addr
    }

    /// Accept an incoming connection
    ///
    /// This will block until a new connection is received.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use impulse_telnet::{TelnetServer, Result};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let server = TelnetServer::bind("0.0.0.0:2323").await?;
    ///
    ///     loop {
    ///         let mut conn = server.accept().await?;
    ///         tokio::spawn(async move {
    ///             conn.initialize().await.ok();
    ///             conn.send_line("Welcome to Impulse BBS!").await.ok();
    ///         });
    ///     }
    /// }
    /// ```
    pub async fn accept(&self) -> Result<TelnetConnection> {
        let (stream, peer_addr) = self.listener.accept().await?;

        // Disable Nagle's algorithm for interactive sessions
        stream.set_nodelay(true)?;

        Ok(TelnetConnection::new(stream, peer_addr))
    }

    /// Accept incoming connections and pass them to a handler function
    ///
    /// This is a convenience method that runs a loop accepting connections
    /// and spawning tasks for each one.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use impulse_telnet::{TelnetServer, Result};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let server = TelnetServer::bind("0.0.0.0:2323").await?;
    ///
    ///     server.serve(|mut conn| async move {
    ///         conn.initialize().await?;
    ///         conn.send_line("Hello from BBS!").await?;
    ///         Ok(())
    ///     }).await
    /// }
    /// ```
    pub async fn serve<F, Fut>(self, handler: F) -> Result<()>
    where
        F: Fn(TelnetConnection) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<()>> + Send + 'static,
    {
        let handler = std::sync::Arc::new(handler);

        loop {
            let conn = self.accept().await?;
            let handler = handler.clone();

            tokio::spawn(async move {
                if let Err(e) = handler(conn).await {
                    eprintln!("Connection handler error: {}", e);
                }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bind_server() {
        // Bind to random port
        let server = TelnetServer::bind("127.0.0.1:0").await.unwrap();
        let addr = server.local_addr();

        // Should be bound to a port
        assert_ne!(addr.port(), 0);
    }

    #[tokio::test]
    async fn test_bind_invalid_address() {
        // Invalid address should fail
        let result = TelnetServer::bind("999.999.999.999:23").await;
        assert!(result.is_err());
    }
}
