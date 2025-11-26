//! Xmodem sender implementation.
//!
//! This module provides the functionality for sending files using the Xmodem protocol.

use super::block::XmodemBlock;
use super::error::{Result, XmodemError};
use super::variants::XmodemVariant;
use super::{ACK, CAN, CRC_MODE, EOT, MAX_RETRIES, NAK, SUB, TIMEOUT_MS};
use std::io::{Read, Write};
use std::time::{Duration, Instant};

/// Configuration for Xmodem sender.
#[derive(Debug, Clone)]
pub struct SenderConfig {
    /// Preferred protocol variant.
    pub variant: XmodemVariant,
    /// Maximum retry attempts per block.
    pub max_retries: usize,
    /// Timeout duration for responses.
    pub timeout: Duration,
}

impl Default for SenderConfig {
    fn default() -> Self {
        Self {
            variant: XmodemVariant::OneK,
            max_retries: MAX_RETRIES,
            timeout: Duration::from_millis(TIMEOUT_MS),
        }
    }
}

/// Statistics for a file transfer.
#[derive(Debug, Clone, Default)]
pub struct SendStats {
    /// Total bytes sent.
    pub bytes_sent: u64,
    /// Number of blocks sent successfully.
    pub blocks_sent: usize,
    /// Number of blocks that had to be retransmitted.
    pub blocks_retried: usize,
    /// Total number of errors encountered.
    pub errors: usize,
    /// Transfer duration.
    pub duration: Duration,
}

/// Xmodem file sender.
///
/// Handles sending files using the Xmodem protocol with automatic retry and
/// error recovery.
pub struct XmodemSender<R, W>
where
    R: Read,
    W: Write,
{
    /// Data source to send from.
    reader: R,
    /// Transport to write to.
    writer: W,
    /// Sender configuration.
    config: SenderConfig,
    /// Negotiated variant (may differ from config after handshake).
    negotiated_variant: Option<XmodemVariant>,
}

impl<R, W> XmodemSender<R, W>
where
    R: Read,
    W: Write,
{
    /// Create a new Xmodem sender.
    ///
    /// # Arguments
    ///
    /// * `reader` - Source to read file data from
    /// * `writer` - Transport to write protocol data to
    /// * `config` - Sender configuration
    pub fn new(reader: R, writer: W, config: SenderConfig) -> Self {
        Self {
            reader,
            writer,
            config,
            negotiated_variant: None,
        }
    }

    /// Wait for the receiver to initiate the transfer.
    ///
    /// The receiver sends either NAK (for checksum mode) or 'C' (for CRC mode)
    /// to start the transfer.
    ///
    /// # Returns
    ///
    /// * `Ok(variant)` - The negotiated protocol variant
    /// * `Err(XmodemError)` - If timeout or error occurs
    pub fn wait_for_start(&mut self) -> Result<XmodemVariant> {
        let start_time = Instant::now();
        let mut buffer = [0u8; 1];

        loop {
            // Check for timeout
            if start_time.elapsed() > self.config.timeout {
                return Err(XmodemError::Timeout);
            }

            // Try to read one byte (non-blocking would be better in real implementation)
            match self.try_read_byte(&mut buffer) {
                Ok(Some(byte)) => {
                    let variant = match byte {
                        NAK => {
                            // Receiver wants checksum mode
                            if self.config.variant == XmodemVariant::Checksum {
                                XmodemVariant::Checksum
                            } else {
                                // We prefer CRC but receiver sent NAK, use checksum
                                XmodemVariant::Checksum
                            }
                        }
                        CRC_MODE => {
                            // Receiver wants CRC mode, use our preferred variant
                            if self.config.variant == XmodemVariant::OneK {
                                XmodemVariant::OneK
                            } else {
                                XmodemVariant::Crc
                            }
                        }
                        CAN => {
                            return Err(XmodemError::Cancelled);
                        }
                        _ => continue, // Ignore other bytes
                    };

                    self.negotiated_variant = Some(variant);
                    return Ok(variant);
                }
                Ok(None) => continue, // No data yet, keep waiting
                Err(e) => return Err(e),
            }
        }
    }

    /// Try to read a single byte without blocking indefinitely.
    fn try_read_byte(&mut self, buffer: &mut [u8; 1]) -> Result<Option<u8>> {
        match self.reader.read(buffer) {
            Ok(1) => Ok(Some(buffer[0])),
            Ok(0) => Ok(None),
            Ok(_) => unreachable!(),
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => Ok(None),
            Err(e) => Err(XmodemError::Io(e)),
        }
    }

    /// Send a single block with retry logic.
    ///
    /// # Arguments
    ///
    /// * `block` - The block to send
    ///
    /// # Returns
    ///
    /// * `Ok(retries)` - Number of retries needed (0 if sent first time)
    /// * `Err(XmodemError)` - If max retries exceeded or transfer cancelled
    pub fn send_block(&mut self, block: &XmodemBlock) -> Result<usize> {
        let packet = block.serialize();
        let mut retries = 0;

        loop {
            // Send the block
            self.writer.write_all(&packet)?;
            self.writer.flush()?;

            // Wait for response
            match self.wait_for_ack()? {
                ACK => return Ok(retries),
                NAK => {
                    retries += 1;
                    if retries >= self.config.max_retries {
                        return Err(XmodemError::MaxRetriesExceeded { attempts: retries });
                    }
                    // Retry sending this block
                    continue;
                }
                CAN => return Err(XmodemError::Cancelled),
                _ => {
                    // Unexpected response, count as error and retry
                    retries += 1;
                    if retries >= self.config.max_retries {
                        return Err(XmodemError::MaxRetriesExceeded { attempts: retries });
                    }
                }
            }
        }
    }

    /// Wait for ACK, NAK, or CAN response.
    fn wait_for_ack(&mut self) -> Result<u8> {
        let start_time = Instant::now();
        let mut buffer = [0u8; 1];

        loop {
            if start_time.elapsed() > self.config.timeout {
                return Err(XmodemError::Timeout);
            }

            match self.try_read_byte(&mut buffer)? {
                Some(byte @ (ACK | NAK | CAN)) => return Ok(byte),
                Some(_) => continue, // Ignore other bytes
                None => continue,    // Keep waiting
            }
        }
    }

    /// Send a complete file.
    ///
    /// # Arguments
    ///
    /// * `_file_size` - Optional file size (for progress tracking, currently unused)
    ///
    /// # Returns
    ///
    /// Transfer statistics
    pub fn send_file(&mut self, _file_size: Option<u64>) -> Result<SendStats> {
        let start_time = Instant::now();
        let mut stats = SendStats::default();

        // Wait for receiver to start
        let variant = self.wait_for_start()?;

        // Read and send blocks
        let mut block_num: u8 = 1;
        let block_size = variant.block_size();
        let mut buffer = vec![0u8; block_size];

        loop {
            // Read block data
            let bytes_read = self.read_block_data(&mut buffer)?;

            if bytes_read == 0 {
                // End of file
                break;
            }

            // Pad partial block with SUB
            if bytes_read < block_size {
                buffer[bytes_read..].fill(SUB);
            }

            // Create and send block
            let block = XmodemBlock::new(block_num, buffer.clone(), variant)?;
            let retries = self.send_block(&block)?;

            // Update statistics
            stats.blocks_sent += 1;
            stats.blocks_retried += retries;
            stats.bytes_sent += bytes_read as u64;

            // Increment block number (wraps at 256)
            block_num = block_num.wrapping_add(1);
        }

        // Send EOT
        self.send_eot()?;

        stats.duration = start_time.elapsed();
        Ok(stats)
    }

    /// Read a full block of data from the reader.
    fn read_block_data(&mut self, buffer: &mut [u8]) -> Result<usize> {
        let mut total_read = 0;
        while total_read < buffer.len() {
            match self.reader.read(&mut buffer[total_read..])? {
                0 => break, // EOF
                n => total_read += n,
            }
        }
        Ok(total_read)
    }

    /// Send End-Of-Transmission marker.
    fn send_eot(&mut self) -> Result<()> {
        let mut retries = 0;

        loop {
            // Send EOT
            self.writer.write_all(&[EOT])?;
            self.writer.flush()?;

            // Wait for ACK
            match self.wait_for_ack()? {
                ACK => return Ok(()),
                NAK => {
                    retries += 1;
                    if retries >= self.config.max_retries {
                        return Err(XmodemError::MaxRetriesExceeded { attempts: retries });
                    }
                    // Retry sending EOT
                    continue;
                }
                CAN => return Err(XmodemError::Cancelled),
                _ => {
                    retries += 1;
                    if retries >= self.config.max_retries {
                        return Err(XmodemError::MaxRetriesExceeded { attempts: retries });
                    }
                }
            }
        }
    }

    /// Cancel the current transfer.
    ///
    /// Sends multiple CAN bytes to abort the transfer.
    pub fn cancel(&mut self) -> Result<()> {
        // Send multiple CAN bytes (standard practice)
        self.writer.write_all(&[CAN, CAN, CAN, CAN, CAN])?;
        self.writer.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_sender_config_default() {
        let config = SenderConfig::default();
        assert_eq!(config.variant, XmodemVariant::OneK);
        assert_eq!(config.max_retries, MAX_RETRIES);
        assert_eq!(config.timeout, Duration::from_millis(TIMEOUT_MS));
    }

    #[test]
    fn test_sender_creation() {
        let data = b"Hello, World!";
        let reader = Cursor::new(data);
        let writer = Vec::new();
        let config = SenderConfig::default();

        let sender = XmodemSender::new(reader, writer, config);
        assert!(sender.negotiated_variant.is_none());
    }

    #[test]
    fn test_send_stats_default() {
        let stats = SendStats::default();
        assert_eq!(stats.bytes_sent, 0);
        assert_eq!(stats.blocks_sent, 0);
        assert_eq!(stats.blocks_retried, 0);
        assert_eq!(stats.errors, 0);
    }

    #[test]
    fn test_block_creation_for_sending() {
        let data = vec![0x42; 128];
        let block = XmodemBlock::new(1, data, XmodemVariant::Checksum).unwrap();
        let packet = block.serialize();
        assert_eq!(packet.len(), 132); // SOH + block# + ~block# + 128 + checksum
    }

    #[test]
    fn test_read_block_data() {
        let data = b"Hello, World!";
        let reader = Cursor::new(data);
        let writer = Vec::new();
        let config = SenderConfig::default();

        let mut sender = XmodemSender::new(reader, writer, config);
        let mut buffer = vec![0u8; 128];

        let bytes_read = sender.read_block_data(&mut buffer).unwrap();
        assert_eq!(bytes_read, data.len());
        assert_eq!(&buffer[..data.len()], data);
    }

    #[test]
    fn test_read_block_data_exact_size() {
        let data = vec![0x42; 128];
        let reader = Cursor::new(data.clone());
        let writer = Vec::new();
        let config = SenderConfig::default();

        let mut sender = XmodemSender::new(reader, writer, config);
        let mut buffer = vec![0u8; 128];

        let bytes_read = sender.read_block_data(&mut buffer).unwrap();
        assert_eq!(bytes_read, 128);
        assert_eq!(buffer, data);
    }

    #[test]
    fn test_read_block_data_larger_than_block() {
        let data = vec![0x42; 256];
        let reader = Cursor::new(data.clone());
        let writer = Vec::new();
        let config = SenderConfig::default();

        let mut sender = XmodemSender::new(reader, writer, config);
        let mut buffer = vec![0x0; 128];

        // First block
        let bytes_read = sender.read_block_data(&mut buffer).unwrap();
        assert_eq!(bytes_read, 128);
        assert_eq!(buffer, vec![0x42; 128]);

        // Second block
        let bytes_read = sender.read_block_data(&mut buffer).unwrap();
        assert_eq!(bytes_read, 128);
        assert_eq!(buffer, vec![0x42; 128]);

        // EOF
        let bytes_read = sender.read_block_data(&mut buffer).unwrap();
        assert_eq!(bytes_read, 0);
    }

    #[test]
    fn test_sender_config_custom() {
        let config = SenderConfig {
            variant: XmodemVariant::Checksum,
            max_retries: 5,
            timeout: Duration::from_secs(5),
        };

        assert_eq!(config.variant, XmodemVariant::Checksum);
        assert_eq!(config.max_retries, 5);
        assert_eq!(config.timeout, Duration::from_secs(5));
    }
}
