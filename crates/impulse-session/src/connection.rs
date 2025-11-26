//! Connection abstraction for different transport types

use async_trait::async_trait;
use std::fmt;

/// Connection type identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionType {
    /// Telnet connection (TCP)
    Telnet,
    /// WebSocket connection (HTTP/HTTPS)
    WebSocket,
    /// SSH connection
    Ssh,
}

impl fmt::Display for ConnectionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Telnet => write!(f, "Telnet"),
            Self::WebSocket => write!(f, "WebSocket"),
            Self::Ssh => write!(f, "SSH"),
        }
    }
}

/// Trait for abstracting different connection types
///
/// This trait provides a unified interface for sending and receiving data
/// across different transport protocols (Telnet, WebSocket, SSH).
#[async_trait]
pub trait Connection: Send + Sync {
    /// Get the connection type
    fn connection_type(&self) -> ConnectionType;

    /// Get the remote address
    fn remote_addr(&self) -> String;

    /// Send text data to the client
    async fn send_text(&mut self, data: &str) -> Result<(), ConnectionError>;

    /// Send binary data to the client
    async fn send_bytes(&mut self, data: &[u8]) -> Result<(), ConnectionError>;

    /// Receive data from the client
    ///
    /// Returns None if the connection is closed
    async fn recv(&mut self) -> Result<Option<Vec<u8>>, ConnectionError>;

    /// Close the connection gracefully
    async fn close(&mut self) -> Result<(), ConnectionError>;

    /// Check if the connection is still alive
    fn is_connected(&self) -> bool;
}

/// Errors that can occur with connections
#[derive(Debug, thiserror::Error)]
pub enum ConnectionError {
    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Connection closed
    #[error("Connection closed")]
    Closed,

    /// Protocol error
    #[error("Protocol error: {0}")]
    Protocol(String),

    /// Encoding error
    #[error("Encoding error: {0}")]
    Encoding(String),

    /// WebSocket-specific error
    #[cfg(feature = "websocket")]
    #[error("WebSocket error: {0}")]
    WebSocket(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_type_display() {
        assert_eq!(ConnectionType::Telnet.to_string(), "Telnet");
        assert_eq!(ConnectionType::WebSocket.to_string(), "WebSocket");
        assert_eq!(ConnectionType::Ssh.to_string(), "SSH");
    }

    #[test]
    fn test_connection_type_equality() {
        assert_eq!(ConnectionType::Telnet, ConnectionType::Telnet);
        assert_ne!(ConnectionType::Telnet, ConnectionType::WebSocket);
    }
}
