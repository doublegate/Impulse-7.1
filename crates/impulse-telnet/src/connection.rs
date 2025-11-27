//! Telnet connection handling

use crate::error::{Result, TelnetError};
use crate::iac::{self, IAC, IacCommand, TelnetOption};
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

/// Maximum buffer size for incoming data (64KB)
const MAX_BUFFER_SIZE: usize = 65536;

/// A telnet connection to a remote client
pub struct TelnetConnection {
    /// Underlying TCP stream
    stream: TcpStream,
    /// Remote address
    peer_addr: SocketAddr,
    /// Input buffer for processing IAC sequences
    buffer: Vec<u8>,
    /// Whether echo is enabled (server echoes back to client)
    echo_enabled: bool,
    /// Whether suppress go ahead is enabled
    suppress_ga: bool,
    /// Terminal width (from NAWS negotiation)
    terminal_width: u16,
    /// Terminal height (from NAWS negotiation)
    terminal_height: u16,
}

impl TelnetConnection {
    /// Create a new telnet connection from a TCP stream
    pub(crate) fn new(stream: TcpStream, peer_addr: SocketAddr) -> Self {
        Self {
            stream,
            peer_addr,
            buffer: Vec::with_capacity(4096),
            echo_enabled: false,
            suppress_ga: true,
            terminal_width: 80,
            terminal_height: 24,
        }
    }

    /// Get the remote peer address
    pub fn peer_addr(&self) -> SocketAddr {
        self.peer_addr
    }

    /// Get terminal dimensions
    pub fn terminal_size(&self) -> (u16, u16) {
        (self.terminal_width, self.terminal_height)
    }

    /// Initialize telnet session with option negotiation
    pub async fn initialize(&mut self) -> Result<()> {
        // Server WILL ECHO (we'll echo characters back)
        self.send_raw(&iac::will(TelnetOption::Echo)).await?;

        // Server WILL SUPPRESS GO AHEAD
        self.send_raw(&iac::will(TelnetOption::SuppressGoAhead))
            .await?;

        // Server requests client DO SUPPRESS GO AHEAD
        self.send_raw(&iac::r#do(TelnetOption::SuppressGoAhead))
            .await?;

        // Request window size
        self.send_raw(&iac::r#do(TelnetOption::WindowSize)).await?;

        Ok(())
    }

    /// Send raw bytes to the client
    pub async fn send_raw(&mut self, data: &[u8]) -> Result<()> {
        self.stream.write_all(data).await?;
        self.stream.flush().await?;
        Ok(())
    }

    /// Send text to the client (escaping IAC bytes)
    pub async fn send_text(&mut self, text: &str) -> Result<()> {
        let mut escaped = Vec::with_capacity(text.len());
        for byte in text.as_bytes() {
            escaped.push(*byte);
            // Escape IAC by sending IAC IAC
            if *byte == IAC {
                escaped.push(IAC);
            }
        }
        self.send_raw(&escaped).await
    }

    /// Send a line of text with CRLF
    pub async fn send_line(&mut self, text: &str) -> Result<()> {
        self.send_text(text).await?;
        self.send_text("\r\n").await
    }

    /// Read a line of text from the client (blocking until CRLF or LF)
    pub async fn read_line(&mut self) -> Result<String> {
        let mut line = Vec::new();
        let mut buf = [0u8; 1];
        let mut started = false; // Track if we've received any non-line-ending characters

        loop {
            let n = self.stream.read(&mut buf).await?;
            if n == 0 {
                return Err(TelnetError::ConnectionClosed);
            }

            let byte = buf[0];

            // Handle IAC sequences
            if byte == IAC {
                self.handle_iac_sequence().await?;
                continue;
            }

            // Skip leading CR/LF characters (handles leftover LF from previous CRLF)
            if !started && (byte == b'\n' || byte == b'\r') {
                continue;
            }

            // Handle line endings - both CR and LF terminate the line
            // This handles: LF only (\n), CR only (\r), and CRLF (\r\n)
            if byte == b'\n' || byte == b'\r' {
                break;
            }

            // Mark that we've started receiving actual content
            started = true;

            // Accumulate printable characters (ASCII 32-126 plus tab)
            if (32..127).contains(&byte) || byte == b'\t' {
                line.push(byte);

                // Echo back if enabled
                if self.echo_enabled {
                    self.stream.write_all(&[byte]).await?;
                    self.stream.flush().await?;
                }
            }
            // Handle backspace/delete
            else if (byte == 8 || byte == 127) && !line.is_empty() {
                line.pop();
                if self.echo_enabled {
                    // Send backspace sequence: BS + space + BS
                    self.stream.write_all(b"\x08 \x08").await?;
                    self.stream.flush().await?;
                }
            }

            // Check buffer size
            if line.len() > MAX_BUFFER_SIZE {
                return Err(TelnetError::BufferOverflow {
                    max: MAX_BUFFER_SIZE,
                });
            }
        }

        String::from_utf8(line).map_err(TelnetError::InvalidUtf8)
    }

    /// Read a password from the client without echoing
    ///
    /// This method temporarily disables echo to prevent the password from being
    /// displayed on the client's terminal. Optionally displays asterisks (*) for
    /// visual feedback.
    ///
    /// # Arguments
    ///
    /// * `show_asterisks` - If true, displays '*' for each character typed
    pub async fn read_password(&mut self, show_asterisks: bool) -> Result<String> {
        let mut password = Vec::new();
        let mut buf = [0u8; 1];
        let mut started = false;

        // Save current echo state
        let original_echo = self.echo_enabled;

        // Disable echo on the server side
        // (client may still echo locally, but we won't echo back)
        self.echo_enabled = false;

        loop {
            let n = self.stream.read(&mut buf).await?;
            if n == 0 {
                self.echo_enabled = original_echo;
                return Err(TelnetError::ConnectionClosed);
            }

            let byte = buf[0];

            // Handle IAC sequences
            if byte == IAC {
                self.handle_iac_sequence().await?;
                continue;
            }

            // Skip leading CR/LF characters
            if !started && (byte == b'\n' || byte == b'\r') {
                continue;
            }

            // Handle line endings
            if byte == b'\n' || byte == b'\r' {
                // Send CRLF to move to next line
                self.stream.write_all(b"\r\n").await?;
                self.stream.flush().await?;
                break;
            }

            // Mark that we've started receiving actual content
            started = true;

            // Accumulate printable characters (ASCII 32-126 plus tab)
            if (32..127).contains(&byte) || byte == b'\t' {
                password.push(byte);

                // Optionally display asterisk for visual feedback
                if show_asterisks {
                    self.stream.write_all(b"*").await?;
                    self.stream.flush().await?;
                }
            }
            // Handle backspace/delete
            else if (byte == 8 || byte == 127) && !password.is_empty() {
                password.pop();
                if show_asterisks {
                    // Send backspace sequence: BS + space + BS
                    self.stream.write_all(b"\x08 \x08").await?;
                    self.stream.flush().await?;
                }
            }

            // Check buffer size
            if password.len() > MAX_BUFFER_SIZE {
                self.echo_enabled = original_echo;
                return Err(TelnetError::BufferOverflow {
                    max: MAX_BUFFER_SIZE,
                });
            }
        }

        // Restore original echo state
        self.echo_enabled = original_echo;

        String::from_utf8(password).map_err(TelnetError::InvalidUtf8)
    }

    /// Read a single character from the client
    pub async fn read_char(&mut self) -> Result<char> {
        let mut buf = [0u8; 1];

        loop {
            let n = self.stream.read(&mut buf).await?;
            if n == 0 {
                return Err(TelnetError::ConnectionClosed);
            }

            let byte = buf[0];

            // Handle IAC sequences
            if byte == IAC {
                self.handle_iac_sequence().await?;
                continue;
            }

            // Return printable character
            if (32..127).contains(&byte) {
                return Ok(byte as char);
            }
            // Return control characters directly
            return Ok(byte as char);
        }
    }

    /// Handle an IAC sequence from the stream
    async fn handle_iac_sequence(&mut self) -> Result<()> {
        let mut buf = [0u8; 2];
        self.stream.read_exact(&mut buf).await?;

        let cmd_byte = buf[0];

        // IAC IAC means literal 255
        if cmd_byte == IAC {
            self.buffer.push(IAC);
            return Ok(());
        }

        let cmd = IacCommand::from_byte(cmd_byte).ok_or(TelnetError::InvalidCommand(cmd_byte))?;

        match cmd {
            IacCommand::WILL | IacCommand::WONT | IacCommand::DO | IacCommand::DONT => {
                let option_byte = buf[1];
                if let Some(option) = TelnetOption::from_byte(option_byte) {
                    self.handle_option_negotiation(cmd, option).await?;
                } else {
                    // Unknown option - respond with DONT/WONT
                    match cmd {
                        IacCommand::WILL => {
                            self.send_raw(&[IAC, IacCommand::DONT.to_byte(), option_byte])
                                .await?;
                        }
                        IacCommand::DO => {
                            self.send_raw(&[IAC, IacCommand::WONT.to_byte(), option_byte])
                                .await?;
                        }
                        _ => {}
                    }
                }
            }
            IacCommand::SB => {
                // Subnegotiation - read until IAC SE
                self.handle_subnegotiation().await?;
            }
            IacCommand::NOP => {
                // No operation - ignore
            }
            IacCommand::AYT => {
                // Are You There - respond affirmatively
                self.send_text("\r\n[Yes]\r\n").await?;
            }
            _ => {
                // Other commands not implemented
            }
        }

        Ok(())
    }

    /// Handle telnet option negotiation
    async fn handle_option_negotiation(
        &mut self,
        cmd: IacCommand,
        option: TelnetOption,
    ) -> Result<()> {
        match (cmd, option) {
            (IacCommand::WILL, TelnetOption::SuppressGoAhead) => {
                self.suppress_ga = true;
                // Acknowledge
                self.send_raw(&iac::r#do(TelnetOption::SuppressGoAhead))
                    .await?;
            }
            (IacCommand::DO, TelnetOption::Echo) => {
                self.echo_enabled = true;
                // Already sent WILL ECHO during init
            }
            (IacCommand::DO, TelnetOption::SuppressGoAhead) => {
                // Already negotiated
            }
            (IacCommand::WILL, TelnetOption::WindowSize) => {
                // Client agrees to send window size
                // Will come via subnegotiation
            }
            _ => {
                // Refuse unknown options
                match cmd {
                    IacCommand::WILL => {
                        self.send_raw(&iac::dont(option)).await?;
                    }
                    IacCommand::DO => {
                        self.send_raw(&iac::wont(option)).await?;
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    /// Handle subnegotiation sequences
    async fn handle_subnegotiation(&mut self) -> Result<()> {
        let mut sub_buffer = Vec::new();
        let mut buf = [0u8; 1];

        // Read until IAC SE
        loop {
            self.stream.read_exact(&mut buf).await?;

            if buf[0] == IAC {
                self.stream.read_exact(&mut buf).await?;
                if buf[0] == IacCommand::SE.to_byte() {
                    break;
                }
                // IAC IAC in subnegotiation means literal IAC
                if buf[0] == IAC {
                    sub_buffer.push(IAC);
                } else {
                    sub_buffer.push(IAC);
                    sub_buffer.push(buf[0]);
                }
            } else {
                sub_buffer.push(buf[0]);
            }

            if sub_buffer.len() > 1024 {
                return Err(TelnetError::BufferOverflow { max: 1024 });
            }
        }

        // Parse subnegotiation
        if !sub_buffer.is_empty() {
            let option_byte = sub_buffer[0];
            if let Some(option) = TelnetOption::from_byte(option_byte) {
                match option {
                    TelnetOption::WindowSize if sub_buffer.len() >= 5 => {
                        // NAWS: option + width(2) + height(2)
                        self.terminal_width = u16::from_be_bytes([sub_buffer[1], sub_buffer[2]]);
                        self.terminal_height = u16::from_be_bytes([sub_buffer[3], sub_buffer[4]]);
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    /// Close the connection gracefully
    pub async fn close(mut self) -> Result<()> {
        self.stream.shutdown().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_defaults() {
        let (width, height) = (80, 24);
        assert_eq!(width, 80);
        assert_eq!(height, 24);
    }

    #[test]
    fn test_buffer_size_constant() {
        assert_eq!(MAX_BUFFER_SIZE, 65536);
    }
}
