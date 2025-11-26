//! I/O handler for door game processes.
//!
//! This module provides asynchronous I/O handling between the BBS and
//! door game processes, managing stdin/stdout communication.

use crate::error::Result;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::Child;
use tokio::sync::mpsc;
use tracing::{debug, trace, warn};

/// Door I/O handler for managing communication with a door game process.
///
/// This handler manages bidirectional communication between the BBS and
/// a door game process, providing async send/receive operations.
pub struct DoorIoHandler {
    /// Channel for sending input to the door process
    stdin_tx: mpsc::Sender<Vec<u8>>,
    /// Channel for receiving output from the door process
    stdout_rx: mpsc::Receiver<Vec<u8>>,
    /// Handle to the background stdin task
    _stdin_handle: tokio::task::JoinHandle<()>,
    /// Handle to the background stdout task
    _stdout_handle: tokio::task::JoinHandle<()>,
}

impl DoorIoHandler {
    /// Create a new I/O handler for a child process.
    ///
    /// # Arguments
    ///
    /// * `child` - The child process to manage I/O for
    ///
    /// # Returns
    ///
    /// A new I/O handler with background tasks for stdin/stdout
    pub async fn new(child: &mut Child) -> Result<Self> {
        let mut stdin = child
            .stdin
            .take()
            .ok_or_else(|| std::io::Error::other("Failed to get stdin"))?;

        let mut stdout = child
            .stdout
            .take()
            .ok_or_else(|| std::io::Error::other("Failed to get stdout"))?;

        // Create channels for stdin/stdout
        let (stdin_tx, mut stdin_rx) = mpsc::channel::<Vec<u8>>(32);
        let (stdout_tx, stdout_rx) = mpsc::channel::<Vec<u8>>(32);

        // Spawn stdin writer task
        let stdin_handle = tokio::spawn(async move {
            while let Some(data) = stdin_rx.recv().await {
                trace!("Sending {} bytes to door stdin", data.len());
                if let Err(e) = stdin.write_all(&data).await {
                    warn!("Failed to write to door stdin: {}", e);
                    break;
                }
                if let Err(e) = stdin.flush().await {
                    warn!("Failed to flush door stdin: {}", e);
                    break;
                }
            }
            debug!("Stdin writer task exiting");
        });

        // Spawn stdout reader task
        let stdout_handle = tokio::spawn(async move {
            let mut buffer = vec![0u8; 4096];
            loop {
                match stdout.read(&mut buffer).await {
                    Ok(0) => {
                        debug!("Door stdout closed");
                        break;
                    }
                    Ok(n) => {
                        trace!("Read {} bytes from door stdout", n);
                        let data = buffer[..n].to_vec();
                        if stdout_tx.send(data).await.is_err() {
                            warn!("Failed to send stdout data to channel");
                            break;
                        }
                    }
                    Err(e) => {
                        warn!("Failed to read from door stdout: {}", e);
                        break;
                    }
                }
            }
            debug!("Stdout reader task exiting");
        });

        Ok(Self {
            stdin_tx,
            stdout_rx,
            _stdin_handle: stdin_handle,
            _stdout_handle: stdout_handle,
        })
    }

    /// Send input to the door process.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to send to the door's stdin
    ///
    /// # Returns
    ///
    /// Ok if the data was sent successfully
    pub async fn send_input(&self, data: &[u8]) -> Result<()> {
        self.stdin_tx.send(data.to_vec()).await.map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::BrokenPipe, "Stdin channel closed")
        })?;
        Ok(())
    }

    /// Receive output from the door process.
    ///
    /// # Returns
    ///
    /// The next chunk of output data, or None if the stream is closed
    pub async fn receive_output(&mut self) -> Option<Vec<u8>> {
        self.stdout_rx.recv().await
    }

    /// Check if the I/O channels are still open.
    ///
    /// # Returns
    ///
    /// `true` if both stdin and stdout channels are open
    pub fn is_running(&self) -> bool {
        !self.stdin_tx.is_closed()
    }

    /// Send a line of text to the door (with newline).
    ///
    /// # Arguments
    ///
    /// * `line` - The text to send (newline will be appended)
    pub async fn send_line(&self, line: &str) -> Result<()> {
        let mut data = line.as_bytes().to_vec();
        data.push(b'\n');
        self.send_input(&data).await
    }

    /// Send a single byte to the door.
    ///
    /// # Arguments
    ///
    /// * `byte` - The byte to send
    pub async fn send_byte(&self, byte: u8) -> Result<()> {
        self.send_input(&[byte]).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::process::Command;

    #[tokio::test]
    async fn test_door_io_handler_creation() {
        // Use 'cat' command which echoes stdin to stdout
        let mut child = Command::new("cat")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .unwrap();

        let handler = DoorIoHandler::new(&mut child).await;
        assert!(handler.is_ok());

        // Clean up
        let _ = child.kill().await;
    }

    #[tokio::test]
    async fn test_send_input() {
        let mut child = Command::new("cat")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .unwrap();

        let handler = DoorIoHandler::new(&mut child).await.unwrap();

        let result = handler.send_input(b"Hello").await;
        assert!(result.is_ok());

        // Clean up
        let _ = child.kill().await;
    }

    #[tokio::test]
    async fn test_send_line() {
        let mut child = Command::new("cat")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .unwrap();

        let handler = DoorIoHandler::new(&mut child).await.unwrap();

        let result = handler.send_line("Hello World").await;
        assert!(result.is_ok());

        // Clean up
        let _ = child.kill().await;
    }

    #[tokio::test]
    async fn test_send_byte() {
        let mut child = Command::new("cat")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .unwrap();

        let handler = DoorIoHandler::new(&mut child).await.unwrap();

        let result = handler.send_byte(b'A').await;
        assert!(result.is_ok());

        // Clean up
        let _ = child.kill().await;
    }

    #[tokio::test]
    async fn test_receive_output() {
        let mut child = Command::new("echo")
            .arg("test output")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .unwrap();

        let mut handler = DoorIoHandler::new(&mut child).await.unwrap();

        // Give echo time to produce output
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let output = handler.receive_output().await;
        assert!(output.is_some());

        let output_data = output.unwrap();
        let output_str = String::from_utf8_lossy(&output_data);
        assert!(output_str.contains("test output"));

        // Clean up
        let _ = child.wait().await;
    }

    #[tokio::test]
    async fn test_is_running() {
        let mut child = Command::new("cat")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .unwrap();

        let handler = DoorIoHandler::new(&mut child).await.unwrap();
        assert!(handler.is_running());

        // Clean up
        let _ = child.kill().await;
    }

    #[tokio::test]
    async fn test_echo_roundtrip() {
        let mut child = Command::new("cat")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .unwrap();

        let mut handler = DoorIoHandler::new(&mut child).await.unwrap();

        // Send data
        handler.send_input(b"Hello\n").await.unwrap();

        // Give cat time to echo
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        // Receive echoed data
        let output = handler.receive_output().await;
        assert!(output.is_some());

        let output_data = output.unwrap();
        let output_str = String::from_utf8_lossy(&output_data);
        assert_eq!(output_str.trim(), "Hello");

        // Clean up
        let _ = child.kill().await;
    }

    #[tokio::test]
    async fn test_multiple_sends() {
        let mut child = Command::new("cat")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .unwrap();

        let handler = DoorIoHandler::new(&mut child).await.unwrap();

        // Send multiple messages
        for i in 0..5 {
            let msg = format!("Message {}\n", i);
            let result = handler.send_input(msg.as_bytes()).await;
            assert!(result.is_ok());
        }

        // Clean up
        let _ = child.kill().await;
    }

    #[tokio::test]
    async fn test_large_data_transfer() {
        let mut child = Command::new("cat")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .unwrap();

        let handler = DoorIoHandler::new(&mut child).await.unwrap();

        // Send large chunk of data
        let large_data = vec![b'A'; 8192];
        let result = handler.send_input(&large_data).await;
        assert!(result.is_ok());

        // Clean up
        let _ = child.kill().await;
    }

    #[tokio::test]
    async fn test_handler_with_no_stdin() {
        let mut child = Command::new("echo")
            .arg("test")
            .stdout(std::process::Stdio::piped())
            .spawn()
            .unwrap();

        let result = DoorIoHandler::new(&mut child).await;
        assert!(result.is_err());

        // Clean up
        let _ = child.wait().await;
    }

    #[tokio::test]
    async fn test_handler_with_no_stdout() {
        let mut child = Command::new("cat")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .unwrap();

        let result = DoorIoHandler::new(&mut child).await;
        assert!(result.is_err());

        // Clean up
        let _ = child.kill().await;
    }
}
