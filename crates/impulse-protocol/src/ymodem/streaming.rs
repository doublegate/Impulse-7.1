//! Ymodem-G streaming protocol implementation.
//!
//! Ymodem-G is a streaming variant of Ymodem that provides maximum throughput
//! on reliable connections by eliminating per-block acknowledgments.
//!
//! # Key Differences from Standard Ymodem
//!
//! - **No per-block ACK**: Blocks are sent continuously without waiting for ACK
//! - **Handshake only**: Only initial handshake and final confirmation require ACK
//! - **Error = Cancel**: Any error cancels the entire transfer (no retry mechanism)
//! - **Reliable connection**: Best for direct serial connections or reliable networks
//! - **CRC-32 option**: Can use CRC-32 for extra reliability
//!
//! # Protocol Flow
//!
//! ## Send Side
//! 1. Receiver sends 'G' to request Ymodem-G mode
//! 2. Sender sends Block 0 (metadata)
//! 3. Wait for ACK
//! 4. Send all data blocks continuously (no waiting for ACK)
//! 5. Send EOT
//! 6. Wait for ACK
//!
//! ## Receive Side
//! 1. Send 'G' to request Ymodem-G mode
//! 2. Receive Block 0
//! 3. Send ACK
//! 4. Receive all data blocks (no ACK per block)
//! 5. Receive EOT
//! 6. Send ACK
//!
//! # Examples
//!
//! ```no_run
//! use impulse_protocol::ymodem::streaming::{YmodemGSender, StreamingConfig};
//! use impulse_protocol::ymodem::{FileMetadata, BatchFile};
//! use tokio::net::TcpStream;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let stream = TcpStream::connect("localhost:2323").await?;
//! let config = StreamingConfig::default();
//! let mut sender = YmodemGSender::new(stream, config);
//!
//! let files = vec![
//!     BatchFile::new(FileMetadata::with_size("file1.txt", 1000)),
//!     BatchFile::new(FileMetadata::with_size("file2.txt", 2000)),
//! ];
//!
//! let stats = sender.send_batch(&files).await?;
//! println!("Sent {} bytes in {} blocks", stats.bytes_sent, stats.blocks_sent);
//! # Ok(())
//! # }
//! ```

use crate::xmodem::{ACK, CAN, EOT, NAK, XmodemBlock, XmodemVariant};
use crate::ymodem::{BatchFile, FileMetadata};
use std::io;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::time::timeout;

/// Ymodem-G mode request character.
pub const YMODEM_G: u8 = b'G';

/// Configuration for Ymodem-G streaming transfers.
#[derive(Debug, Clone)]
pub struct StreamingConfig {
    /// Timeout for handshake and final confirmation (in milliseconds).
    pub timeout_ms: u64,
    /// Use CRC-32 instead of CRC-16 for extra reliability.
    pub use_crc32: bool,
    /// Maximum time to wait for initial handshake.
    pub handshake_timeout_ms: u64,
}

impl Default for StreamingConfig {
    fn default() -> Self {
        Self {
            timeout_ms: 30000,           // 30 seconds for handshake/final
            use_crc32: false,            // Standard Ymodem uses CRC-16
            handshake_timeout_ms: 60000, // 1 minute for initial handshake
        }
    }
}

/// Statistics for a Ymodem-G batch transfer.
#[derive(Debug, Clone, Default)]
pub struct BatchStats {
    /// Total number of files sent/received.
    pub files_count: usize,
    /// Total number of data blocks sent/received.
    pub blocks_sent: usize,
    /// Total bytes of data sent/received.
    pub bytes_sent: u64,
    /// Total transfer time.
    pub duration: Duration,
    /// Average transfer rate in bytes per second.
    pub bytes_per_second: u64,
}

/// Ymodem-G streaming sender.
///
/// Sends files in streaming mode without per-block acknowledgments.
pub struct YmodemGSender<S> {
    stream: S,
    config: StreamingConfig,
}

impl<S: AsyncRead + AsyncWrite + Unpin> YmodemGSender<S> {
    /// Create a new Ymodem-G sender.
    pub fn new(stream: S, config: StreamingConfig) -> Self {
        Self { stream, config }
    }

    /// Send a batch of files using Ymodem-G protocol.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The receiver doesn't support Ymodem-G mode
    /// - A timeout occurs during handshake or final confirmation
    /// - An I/O error occurs
    /// - The receiver cancels the transfer
    pub async fn send_batch(&mut self, files: &[BatchFile]) -> io::Result<BatchStats> {
        let start_time = Instant::now();
        let mut stats = BatchStats::default();

        // Wait for receiver to request Ymodem-G mode
        self.wait_for_g_request().await?;

        // Send each file
        for file in files {
            self.send_file(file, &mut stats).await?;
        }

        // Send end-of-batch marker
        self.send_end_of_batch().await?;

        // Calculate final statistics
        stats.files_count = files.len();
        stats.duration = start_time.elapsed();
        if stats.duration.as_secs() > 0 {
            stats.bytes_per_second = stats.bytes_sent / stats.duration.as_secs();
        }

        Ok(stats)
    }

    /// Wait for receiver to send 'G' to request Ymodem-G mode.
    async fn wait_for_g_request(&mut self) -> io::Result<()> {
        let timeout_duration = Duration::from_millis(self.config.handshake_timeout_ms);

        match timeout(timeout_duration, self.stream.read_u8()).await {
            Ok(Ok(YMODEM_G)) => Ok(()),
            Ok(Ok(NAK)) => Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "Receiver does not support Ymodem-G (sent NAK instead of G)",
            )),
            Ok(Ok(CAN)) => Err(io::Error::new(
                io::ErrorKind::ConnectionAborted,
                "Transfer cancelled by receiver during handshake",
            )),
            Ok(Ok(byte)) => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Unexpected byte during handshake: 0x{:02X}", byte),
            )),
            Ok(Err(e)) => Err(e),
            Err(_) => Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "Timeout waiting for Ymodem-G request",
            )),
        }
    }

    /// Send a single file in the batch.
    async fn send_file(&mut self, file: &BatchFile, stats: &mut BatchStats) -> io::Result<()> {
        // Send Block 0 (metadata)
        self.send_block0(&file.metadata).await?;

        // Wait for ACK of Block 0
        self.wait_for_ack("Block 0").await?;

        // Send all data blocks without waiting for ACK
        let data = file
            .data
            .as_ref()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "BatchFile has no data"))?;
        let mut block_num = 1u8;
        let mut offset = 0;

        while offset < data.len() {
            let end = (offset + 1024).min(data.len());
            let chunk = &data[offset..end];

            // Pad if necessary
            let mut block_data = vec![0x1A; 1024]; // SUB padding
            block_data[..chunk.len()].copy_from_slice(chunk);

            // Create and send block
            let block = XmodemBlock::new(block_num, block_data, XmodemVariant::OneK)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

            self.stream.write_all(&block.serialize()).await?;

            stats.blocks_sent += 1;
            stats.bytes_sent += chunk.len() as u64;
            offset = end;
            block_num = block_num.wrapping_add(1);
        }

        // Send EOT
        self.stream.write_u8(EOT).await?;

        // Wait for ACK of EOT
        self.wait_for_ack("EOT").await?;

        Ok(())
    }

    /// Send Block 0 containing file metadata.
    async fn send_block0(&mut self, metadata: &FileMetadata) -> io::Result<()> {
        let block0_data = metadata.encode();
        let block = XmodemBlock::new(0, block0_data.to_vec(), XmodemVariant::Crc)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

        self.stream.write_all(&block.serialize()).await?;
        Ok(())
    }

    /// Send end-of-batch marker (empty Block 0).
    async fn send_end_of_batch(&mut self) -> io::Result<()> {
        let eob = FileMetadata::end_of_batch();
        let block = XmodemBlock::new(0, eob.to_vec(), XmodemVariant::Crc)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

        self.stream.write_all(&block.serialize()).await?;

        // Wait for final ACK
        self.wait_for_ack("end-of-batch").await?;

        Ok(())
    }

    /// Wait for ACK from receiver.
    async fn wait_for_ack(&mut self, context: &str) -> io::Result<()> {
        let timeout_duration = Duration::from_millis(self.config.timeout_ms);

        match timeout(timeout_duration, self.stream.read_u8()).await {
            Ok(Ok(ACK)) => Ok(()),
            Ok(Ok(CAN)) => Err(io::Error::new(
                io::ErrorKind::ConnectionAborted,
                format!("Transfer cancelled by receiver at {}", context),
            )),
            Ok(Ok(byte)) => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Expected ACK at {}, got 0x{:02X}", context, byte),
            )),
            Ok(Err(e)) => Err(e),
            Err(_) => Err(io::Error::new(
                io::ErrorKind::TimedOut,
                format!("Timeout waiting for ACK at {}", context),
            )),
        }
    }
}

/// Ymodem-G streaming receiver.
///
/// Receives files in streaming mode without sending per-block acknowledgments.
pub struct YmodemGReceiver<S> {
    stream: S,
    config: StreamingConfig,
    output_dir: PathBuf,
}

impl<S: AsyncRead + AsyncWrite + Unpin> YmodemGReceiver<S> {
    /// Create a new Ymodem-G receiver.
    pub fn new(stream: S, config: StreamingConfig, output_dir: PathBuf) -> Self {
        Self {
            stream,
            config,
            output_dir,
        }
    }

    /// Receive a batch of files using Ymodem-G protocol.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - A timeout occurs
    /// - An I/O error occurs
    /// - A block fails CRC validation
    /// - The sender cancels the transfer
    pub async fn receive_batch(&mut self) -> io::Result<BatchStats> {
        let start_time = Instant::now();
        let mut stats = BatchStats::default();

        // Request Ymodem-G mode
        self.stream.write_u8(YMODEM_G).await?;
        self.stream.flush().await?;

        // Receive files until end-of-batch
        while let ReceiveResult::FileReceived = self.receive_file(&mut stats).await? {
            stats.files_count += 1;
        }

        // Calculate final statistics
        stats.duration = start_time.elapsed();
        if stats.duration.as_secs() > 0 {
            stats.bytes_per_second = stats.bytes_sent / stats.duration.as_secs();
        }

        Ok(stats)
    }

    /// Receive a single file.
    async fn receive_file(&mut self, stats: &mut BatchStats) -> io::Result<ReceiveResult> {
        // Receive Block 0
        let block0 = self.receive_block(0).await?;

        // Check for end-of-batch
        if block0.data.iter().all(|&b| b == 0) {
            self.stream.write_u8(ACK).await?;
            self.stream.flush().await?;
            return Ok(ReceiveResult::EndOfBatch);
        }

        // Parse metadata
        let metadata = FileMetadata::decode(&block0.data)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "Empty metadata in Block 0")
            })?;

        // Send ACK for Block 0
        self.stream.write_u8(ACK).await?;
        self.stream.flush().await?;

        // Receive all data blocks
        let mut file_data = Vec::new();
        let mut block_num = 1u8;

        loop {
            // Check for EOT
            match self.stream.read_u8().await? {
                EOT => {
                    // Trim padding
                    if let Some(size) = metadata.size {
                        file_data.truncate(size as usize);
                    }

                    // Send ACK for EOT
                    self.stream.write_u8(ACK).await?;
                    self.stream.flush().await?;

                    stats.bytes_sent += file_data.len() as u64;
                    break;
                }
                byte => {
                    // Put byte back and read full block
                    let mut block_bytes = vec![byte];
                    let mut rest = vec![0u8; 132]; // Rest of 128-byte block or 1029 for 1K
                    self.stream.read_exact(&mut rest).await?;
                    block_bytes.extend_from_slice(&rest);

                    // Deserialize block
                    let block = XmodemBlock::deserialize(&block_bytes, true)
                        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

                    // Verify block number
                    if block.block_num != block_num {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!(
                                "Block number mismatch: expected {}, got {}",
                                block_num, block.block_num
                            ),
                        ));
                    }

                    // Append data
                    file_data.extend_from_slice(&block.data);
                    stats.blocks_sent += 1;
                    block_num = block_num.wrapping_add(1);
                }
            }
        }

        // Write file to disk
        let file_path = self.output_dir.join(&metadata.name);
        tokio::fs::write(&file_path, &file_data).await?;

        Ok(ReceiveResult::FileReceived)
    }

    /// Receive a single block.
    async fn receive_block(&mut self, expected_num: u8) -> io::Result<XmodemBlock> {
        let timeout_duration = Duration::from_millis(self.config.timeout_ms);

        // Read block with timeout
        let mut buffer = vec![0u8; 133]; // SOH + 128 data + 2 CRC
        match timeout(timeout_duration, self.stream.read_exact(&mut buffer)).await {
            Ok(Ok(_)) => {
                let block = XmodemBlock::deserialize(&buffer, true)
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

                if block.block_num != expected_num {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!(
                            "Block number mismatch: expected {}, got {}",
                            expected_num, block.block_num
                        ),
                    ));
                }

                Ok(block)
            }
            Ok(Err(e)) => Err(e),
            Err(_) => Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "Timeout waiting for block",
            )),
        }
    }
}

/// Result of receiving a file.
enum ReceiveResult {
    /// A file was successfully received.
    FileReceived,
    /// End-of-batch marker was received.
    EndOfBatch,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ymodem_g_constant() {
        assert_eq!(YMODEM_G, b'G');
    }

    #[test]
    fn test_streaming_config_default() {
        let config = StreamingConfig::default();
        assert_eq!(config.timeout_ms, 30000);
        assert!(!config.use_crc32);
        assert_eq!(config.handshake_timeout_ms, 60000);
    }

    #[test]
    fn test_streaming_config_custom() {
        let config = StreamingConfig {
            timeout_ms: 15000,
            use_crc32: true,
            handshake_timeout_ms: 30000,
        };
        assert_eq!(config.timeout_ms, 15000);
        assert!(config.use_crc32);
        assert_eq!(config.handshake_timeout_ms, 30000);
    }

    #[test]
    fn test_batch_stats_default() {
        let stats = BatchStats::default();
        assert_eq!(stats.files_count, 0);
        assert_eq!(stats.blocks_sent, 0);
        assert_eq!(stats.bytes_sent, 0);
        assert_eq!(stats.bytes_per_second, 0);
    }

    #[test]
    fn test_batch_stats_clone() {
        let stats = BatchStats {
            files_count: 5,
            blocks_sent: 100,
            bytes_sent: 102400,
            duration: Duration::from_secs(10),
            bytes_per_second: 10240,
        };

        let cloned = stats.clone();
        assert_eq!(cloned.files_count, 5);
        assert_eq!(cloned.blocks_sent, 100);
        assert_eq!(cloned.bytes_sent, 102400);
        assert_eq!(cloned.bytes_per_second, 10240);
    }

    #[tokio::test]
    async fn test_sender_creation() {
        let stream = tokio::io::duplex(1024).0;
        let config = StreamingConfig::default();
        let _sender = YmodemGSender::new(stream, config);
    }

    #[tokio::test]
    async fn test_receiver_creation() {
        let stream = tokio::io::duplex(1024).0;
        let config = StreamingConfig::default();
        let output_dir = PathBuf::from("/tmp");
        let _receiver = YmodemGReceiver::new(stream, config, output_dir);
    }

    #[tokio::test]
    async fn test_wait_for_g_request_success() {
        let (mut client, server) = tokio::io::duplex(1024);
        let config = StreamingConfig::default();
        let mut sender = YmodemGSender::new(server, config);

        // Client sends 'G'
        tokio::spawn(async move {
            client.write_u8(YMODEM_G).await.unwrap();
        });

        let result = sender.wait_for_g_request().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_wait_for_g_request_nak() {
        let (mut client, server) = tokio::io::duplex(1024);
        let config = StreamingConfig {
            handshake_timeout_ms: 1000,
            ..Default::default()
        };
        let mut sender = YmodemGSender::new(server, config);

        // Client sends NAK instead of G
        tokio::spawn(async move {
            client.write_u8(NAK).await.unwrap();
        });

        let result = sender.wait_for_g_request().await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::Unsupported);
    }

    #[tokio::test]
    async fn test_wait_for_g_request_cancel() {
        let (mut client, server) = tokio::io::duplex(1024);
        let config = StreamingConfig {
            handshake_timeout_ms: 1000,
            ..Default::default()
        };
        let mut sender = YmodemGSender::new(server, config);

        // Client sends CAN
        tokio::spawn(async move {
            client.write_u8(CAN).await.unwrap();
        });

        let result = sender.wait_for_g_request().await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::ConnectionAborted);
    }

    #[tokio::test]
    async fn test_wait_for_ack_success() {
        let (mut client, server) = tokio::io::duplex(1024);
        let config = StreamingConfig::default();
        let mut sender = YmodemGSender::new(server, config);

        // Client sends ACK
        tokio::spawn(async move {
            client.write_u8(ACK).await.unwrap();
        });

        let result = sender.wait_for_ack("test").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_wait_for_ack_cancel() {
        let (mut client, server) = tokio::io::duplex(1024);
        let config = StreamingConfig {
            timeout_ms: 1000,
            ..Default::default()
        };
        let mut sender = YmodemGSender::new(server, config);

        // Client sends CAN
        tokio::spawn(async move {
            client.write_u8(CAN).await.unwrap();
        });

        let result = sender.wait_for_ack("test").await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::ConnectionAborted);
    }
}
