//! Xmodem receiver implementation.
//!
//! This module provides the functionality for receiving files using the Xmodem protocol.

use super::block::XmodemBlock;
use super::error::{Result, XmodemError};
use super::variants::XmodemVariant;
use super::{ACK, CAN, CRC_MODE, EOT, MAX_RETRIES, NAK, TIMEOUT_MS};
use std::io::{Read, Write};
use std::time::{Duration, Instant};

/// Configuration for Xmodem receiver.
#[derive(Debug, Clone)]
pub struct ReceiverConfig {
    /// Preferred protocol variant.
    pub variant: XmodemVariant,
    /// Maximum retry attempts per block.
    pub max_retries: usize,
    /// Timeout duration for waiting for data.
    pub timeout: Duration,
}

impl Default for ReceiverConfig {
    fn default() -> Self {
        Self {
            variant: XmodemVariant::OneK,
            max_retries: MAX_RETRIES,
            timeout: Duration::from_millis(TIMEOUT_MS),
        }
    }
}

/// Statistics for a file reception.
#[derive(Debug, Clone, Default)]
pub struct ReceiveStats {
    /// Total bytes received.
    pub bytes_received: u64,
    /// Number of blocks received successfully.
    pub blocks_received: usize,
    /// Number of blocks that had to be re-requested.
    pub blocks_retried: usize,
    /// Total number of errors encountered.
    pub errors: usize,
    /// Transfer duration.
    pub duration: Duration,
}

/// Xmodem file receiver.
///
/// Handles receiving files using the Xmodem protocol with automatic error
/// detection and recovery.
pub struct XmodemReceiver<R, W>
where
    R: Read,
    W: Write,
{
    /// Transport to read from.
    reader: R,
    /// Data sink to write to.
    writer: W,
    /// Receiver configuration.
    config: ReceiverConfig,
    /// Negotiated variant (reserved for future use).
    #[allow(dead_code)]
    negotiated_variant: Option<XmodemVariant>,
}

impl<R, W> XmodemReceiver<R, W>
where
    R: Read,
    W: Write,
{
    /// Create a new Xmodem receiver.
    ///
    /// # Arguments
    ///
    /// * `reader` - Transport to read protocol data from
    /// * `writer` - Sink to write received file data to
    /// * `config` - Receiver configuration
    pub fn new(reader: R, writer: W, config: ReceiverConfig) -> Self {
        Self {
            reader,
            writer,
            config,
            negotiated_variant: None,
        }
    }

    /// Send start signal to initiate the transfer.
    ///
    /// Sends either NAK (for checksum mode) or 'C' (for CRC mode) to tell
    /// the sender to start transmitting.
    pub fn send_start(&mut self) -> Result<()> {
        let start_byte = if self.config.variant.uses_crc() {
            CRC_MODE
        } else {
            NAK
        };

        self.writer.write_all(&[start_byte])?;
        self.writer.flush()?;
        Ok(())
    }

    /// Receive a single block.
    ///
    /// # Arguments
    ///
    /// * `expected_block_num` - The expected block number
    ///
    /// # Returns
    ///
    /// * `Ok(Some(block))` - Successfully received block
    /// * `Ok(None)` - Received EOT (end of transmission)
    /// * `Err(XmodemError)` - Error occurred
    pub fn receive_block(&mut self, expected_block_num: u8) -> Result<Option<XmodemBlock>> {
        let start_time = Instant::now();
        let mut header = [0u8; 1];

        // Read header byte
        loop {
            if start_time.elapsed() > self.config.timeout {
                return Err(XmodemError::Timeout);
            }

            match self.try_read_byte(&mut header)? {
                Some(byte) => {
                    match byte {
                        super::SOH | super::STX => {
                            // Got block header, read the rest
                            return self.read_block_packet(byte, expected_block_num);
                        }
                        EOT => {
                            // End of transmission
                            return Ok(None);
                        }
                        CAN => {
                            return Err(XmodemError::Cancelled);
                        }
                        _ => {
                            // Ignore unexpected bytes
                            continue;
                        }
                    }
                }
                None => continue, // Keep waiting
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

    /// Read the complete block packet after receiving the header.
    fn read_block_packet(
        &mut self,
        header: u8,
        expected_block_num: u8,
    ) -> Result<Option<XmodemBlock>> {
        // Determine packet size from header
        let use_crc = self.config.variant.uses_crc();
        let variant = XmodemVariant::from_header_byte(header, use_crc)?;
        let packet_size = variant.packet_size();

        // Read complete packet (we already have the header)
        let mut packet = vec![header];
        let mut remaining = vec![0u8; packet_size - 1];
        self.reader.read_exact(&mut remaining)?;
        packet.extend_from_slice(&remaining);

        // Deserialize and validate
        let block = XmodemBlock::deserialize(&packet, use_crc)?;

        // Check block number
        if !block.is_expected_block(expected_block_num) {
            return Err(XmodemError::InvalidBlockNumber {
                expected: expected_block_num,
                actual: block.block_num,
            });
        }

        Ok(Some(block))
    }

    /// Send ACK to acknowledge successful block receipt.
    fn send_ack(&mut self) -> Result<()> {
        self.writer.write_all(&[ACK])?;
        self.writer.flush()?;
        Ok(())
    }

    /// Send NAK to request block retransmission.
    fn send_nak(&mut self) -> Result<()> {
        self.writer.write_all(&[NAK])?;
        self.writer.flush()?;
        Ok(())
    }

    /// Receive a complete file.
    ///
    /// # Returns
    ///
    /// Transfer statistics
    pub fn receive_file(&mut self) -> Result<ReceiveStats> {
        let start_time = Instant::now();
        let mut stats = ReceiveStats::default();

        // Send start signal
        self.send_start()?;

        // Receive blocks
        let mut expected_block_num: u8 = 1;
        let mut consecutive_errors = 0;

        loop {
            match self.receive_block(expected_block_num) {
                Ok(Some(block)) => {
                    // Successfully received block
                    consecutive_errors = 0;

                    // Write data (trim padding for last block is caller's responsibility)
                    self.writer.write_all(&block.data)?;

                    // Send ACK
                    self.send_ack()?;

                    // Update statistics
                    stats.blocks_received += 1;
                    stats.bytes_received += block.data.len() as u64;

                    // Increment expected block number (wraps at 256)
                    expected_block_num = expected_block_num.wrapping_add(1);
                }
                Ok(None) => {
                    // Received EOT, send ACK and finish
                    self.send_ack()?;
                    break;
                }
                Err(e) => {
                    // Error receiving block
                    consecutive_errors += 1;
                    stats.errors += 1;
                    stats.blocks_retried += 1;

                    if consecutive_errors >= self.config.max_retries {
                        return Err(XmodemError::MaxRetriesExceeded {
                            attempts: consecutive_errors,
                        });
                    }

                    // Send NAK to request retransmission
                    self.send_nak()?;

                    // For timeout or other errors, don't increment expected block number
                    // The sender will retry the same block
                    match e {
                        XmodemError::Timeout | XmodemError::Cancelled => {
                            return Err(e);
                        }
                        _ => {
                            // Other errors (checksum, etc.) - continue with retry
                            continue;
                        }
                    }
                }
            }
        }

        stats.duration = start_time.elapsed();
        Ok(stats)
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
    fn test_receiver_config_default() {
        let config = ReceiverConfig::default();
        assert_eq!(config.variant, XmodemVariant::OneK);
        assert_eq!(config.max_retries, MAX_RETRIES);
        assert_eq!(config.timeout, Duration::from_millis(TIMEOUT_MS));
    }

    #[test]
    fn test_receiver_creation() {
        let reader = Cursor::new(Vec::new());
        let writer = Vec::new();
        let config = ReceiverConfig::default();

        let receiver = XmodemReceiver::new(reader, writer, config);
        assert!(receiver.negotiated_variant.is_none());
    }

    #[test]
    fn test_receive_stats_default() {
        let stats = ReceiveStats::default();
        assert_eq!(stats.bytes_received, 0);
        assert_eq!(stats.blocks_received, 0);
        assert_eq!(stats.blocks_retried, 0);
        assert_eq!(stats.errors, 0);
    }

    #[test]
    fn test_receiver_config_custom() {
        let config = ReceiverConfig {
            variant: XmodemVariant::Checksum,
            max_retries: 5,
            timeout: Duration::from_secs(5),
        };

        assert_eq!(config.variant, XmodemVariant::Checksum);
        assert_eq!(config.max_retries, 5);
        assert_eq!(config.timeout, Duration::from_secs(5));
    }

    #[test]
    fn test_send_start_crc_mode() {
        let reader = Cursor::new(Vec::new());
        let writer = Vec::new();
        let config = ReceiverConfig {
            variant: XmodemVariant::Crc,
            ..Default::default()
        };

        let mut receiver = XmodemReceiver::new(reader, writer, config);
        receiver.send_start().unwrap();

        // Check that 'C' was written
        assert_eq!(receiver.writer, vec![CRC_MODE]);
    }

    #[test]
    fn test_send_start_checksum_mode() {
        let reader = Cursor::new(Vec::new());
        let writer = Vec::new();
        let config = ReceiverConfig {
            variant: XmodemVariant::Checksum,
            ..Default::default()
        };

        let mut receiver = XmodemReceiver::new(reader, writer, config);
        receiver.send_start().unwrap();

        // Check that NAK was written
        assert_eq!(receiver.writer, vec![NAK]);
    }

    #[test]
    fn test_send_ack() {
        let reader = Cursor::new(Vec::new());
        let writer = Vec::new();
        let config = ReceiverConfig::default();

        let mut receiver = XmodemReceiver::new(reader, writer, config);
        receiver.send_ack().unwrap();

        assert_eq!(receiver.writer, vec![ACK]);
    }

    #[test]
    fn test_send_nak() {
        let reader = Cursor::new(Vec::new());
        let writer = Vec::new();
        let config = ReceiverConfig::default();

        let mut receiver = XmodemReceiver::new(reader, writer, config);
        receiver.send_nak().unwrap();

        assert_eq!(receiver.writer, vec![NAK]);
    }

    #[test]
    fn test_cancel() {
        let reader = Cursor::new(Vec::new());
        let writer = Vec::new();
        let config = ReceiverConfig::default();

        let mut receiver = XmodemReceiver::new(reader, writer, config);
        receiver.cancel().unwrap();

        assert_eq!(receiver.writer, vec![CAN, CAN, CAN, CAN, CAN]);
    }
}
