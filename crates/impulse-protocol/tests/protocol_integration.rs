//! Integration tests for protocol implementations.
//!
//! These tests verify that all protocol implementations work correctly together
//! and integrate properly with the selection and detection systems.

use impulse_protocol::{
    detection::{DetectedProtocol, ProtocolDetector},
    selection::FileProtocol,
    xmodem::{XmodemBlock, XmodemVariant},
    ymodem::{FileMetadata, YmodemBatch},
};

/// Test protocol detection from bytes.
#[test]
fn test_detect_zmodem_from_bytes() {
    let bytes = b"**\x18B0100000023be50\r\x8a\x11";
    let protocol = ProtocolDetector::detect_from_bytes(bytes);
    assert_eq!(protocol, DetectedProtocol::Zmodem);
    assert!(protocol.is_known());
}

#[test]
fn test_detect_ymodem_g_from_bytes() {
    let bytes = b"G";
    let protocol = ProtocolDetector::detect_from_bytes(bytes);
    assert_eq!(protocol, DetectedProtocol::YmodemG);
    assert!(protocol.is_known());
}

#[test]
fn test_detect_ymodem_from_bytes() {
    let bytes = b"C";
    let protocol = ProtocolDetector::detect_from_bytes(bytes);
    assert_eq!(protocol, DetectedProtocol::Ymodem);
    assert!(protocol.is_known());
}

#[test]
fn test_detect_xmodem_from_bytes() {
    let bytes = b"\x01\x01\xFE...";
    let protocol = ProtocolDetector::detect_from_bytes(bytes);
    assert_eq!(protocol, DetectedProtocol::Xmodem);
    assert!(protocol.is_known());
}

#[test]
fn test_detect_unknown() {
    let bytes = b"INVALID";
    let protocol = ProtocolDetector::detect_from_bytes(bytes);
    assert_eq!(protocol, DetectedProtocol::Unknown);
    assert!(!protocol.is_known());
}

/// Test protocol selection features.
#[test]
fn test_protocol_capabilities() {
    assert!(FileProtocol::Zmodem.supports_crash_recovery());
    assert!(FileProtocol::Zmodem.is_streaming());
    assert!(!FileProtocol::Zmodem.supports_batch());

    assert!(FileProtocol::Ymodem.supports_batch());
    assert!(!FileProtocol::Ymodem.is_streaming());
    assert!(!FileProtocol::Ymodem.supports_crash_recovery());

    assert!(FileProtocol::YmodemG.supports_batch());
    assert!(FileProtocol::YmodemG.is_streaming());
    assert!(!FileProtocol::YmodemG.supports_crash_recovery());
}

#[test]
fn test_protocol_block_sizes() {
    assert_eq!(FileProtocol::Zmodem.block_size(), 1024);
    assert_eq!(FileProtocol::Ymodem.block_size(), 1024);
    assert_eq!(FileProtocol::YmodemG.block_size(), 1024);
    assert_eq!(FileProtocol::Xmodem1K.block_size(), 1024);
    assert_eq!(FileProtocol::XmodemCrc.block_size(), 128);
    assert_eq!(FileProtocol::Xmodem.block_size(), 128);
}

#[test]
fn test_protocol_xmodem_variants() {
    assert_eq!(
        FileProtocol::Xmodem.xmodem_variant(),
        Some(XmodemVariant::Checksum)
    );
    assert_eq!(
        FileProtocol::XmodemCrc.xmodem_variant(),
        Some(XmodemVariant::Crc)
    );
    assert_eq!(
        FileProtocol::Xmodem1K.xmodem_variant(),
        Some(XmodemVariant::OneK)
    );
    assert_eq!(FileProtocol::Zmodem.xmodem_variant(), None);
    assert_eq!(FileProtocol::Ymodem.xmodem_variant(), None);
}

/// Test Xmodem block operations.
#[test]
fn test_xmodem_block_creation() {
    for variant in [
        XmodemVariant::Checksum,
        XmodemVariant::Crc,
        XmodemVariant::OneK,
    ] {
        let data = vec![0x42; variant.block_size()];
        let block = XmodemBlock::new(1, data.clone(), variant).unwrap();

        assert_eq!(block.block_num, 1);
        assert_eq!(block.data.len(), variant.block_size());
        assert_eq!(block.variant, variant);
    }
}

#[test]
fn test_xmodem_serialization_round_trip() {
    for variant in [
        XmodemVariant::Checksum,
        XmodemVariant::Crc,
        XmodemVariant::OneK,
    ] {
        let data = vec![0xAA; variant.block_size()];
        let block = XmodemBlock::new(42, data.clone(), variant).unwrap();

        let serialized = block.serialize();
        let deserialized = XmodemBlock::deserialize(&serialized, variant.uses_crc()).unwrap();

        assert_eq!(deserialized.block_num, 42);
        assert_eq!(deserialized.data, data);
        assert_eq!(deserialized.variant, variant);
    }
}

/// Test Ymodem metadata operations.
#[test]
fn test_ymodem_metadata_creation() {
    let metadata = FileMetadata::new("test.txt");
    assert_eq!(metadata.name, "test.txt");
    assert_eq!(metadata.size, None);
    assert_eq!(metadata.mod_time, None);
}

#[test]
fn test_ymodem_metadata_with_size() {
    let metadata = FileMetadata::with_size("test.txt", 12345);
    assert_eq!(metadata.name, "test.txt");
    assert_eq!(metadata.size, Some(12345));
}

#[test]
fn test_ymodem_metadata_encoding() {
    let metadata = FileMetadata::with_size("test.txt", 12345);
    let encoded = metadata.encode();

    assert_eq!(encoded.len(), 128); // BLOCK0_SIZE
    assert!(encoded.starts_with(b"test.txt\0"));
}

#[test]
fn test_ymodem_end_of_batch() {
    let eob = FileMetadata::end_of_batch();
    assert_eq!(eob.len(), 128);
    assert!(eob.iter().all(|&b| b == 0));
}

/// Test Ymodem batch operations.
#[test]
fn test_ymodem_batch_creation() {
    let batch = YmodemBatch::new();
    assert!(batch.is_empty());
    assert_eq!(batch.len(), 0);
}

#[test]
fn test_ymodem_batch_add_files() {
    let mut batch = YmodemBatch::new();

    batch.add_metadata(FileMetadata::with_size("file1.txt", 1000));
    batch.add_metadata(FileMetadata::with_size("file2.txt", 2000));

    assert_eq!(batch.len(), 2);
    assert!(!batch.is_empty());
}

/// Test protocol detection with async streams.
#[tokio::test]
async fn test_async_protocol_detection_zmodem() {
    let data = b"**\x18B0100000023be50";
    let mut stream = &data[..];
    let detector = ProtocolDetector::default();

    let protocol = detector.detect(&mut stream).await;
    assert_eq!(protocol, DetectedProtocol::Zmodem);
}

#[tokio::test]
async fn test_async_protocol_detection_ymodem_g() {
    let data = b"G";
    let mut stream = &data[..];
    let detector = ProtocolDetector::default();

    let protocol = detector.detect(&mut stream).await;
    assert_eq!(protocol, DetectedProtocol::YmodemG);
}

#[tokio::test]
async fn test_async_protocol_detection_timeout() {
    let data = b"";
    let mut stream = &data[..];
    let detector = ProtocolDetector::new(100); // 100ms timeout

    let protocol = detector.detect(&mut stream).await;
    assert_eq!(protocol, DetectedProtocol::Unknown);
}

/// Test protocol preferences integration.
#[test]
fn test_protocol_selection_all() {
    let all = FileProtocol::all();
    assert_eq!(all.len(), 6);
    assert!(all.contains(&FileProtocol::Zmodem));
    assert!(all.contains(&FileProtocol::Ymodem));
    assert!(all.contains(&FileProtocol::YmodemG));
    assert!(all.contains(&FileProtocol::Xmodem1K));
    assert!(all.contains(&FileProtocol::XmodemCrc));
    assert!(all.contains(&FileProtocol::Xmodem));
}

#[test]
fn test_protocol_default_is_zmodem() {
    assert_eq!(FileProtocol::default(), FileProtocol::Zmodem);
}

#[test]
fn test_protocol_names() {
    assert_eq!(FileProtocol::Zmodem.name(), "Zmodem");
    assert_eq!(FileProtocol::Ymodem.name(), "Ymodem");
    assert_eq!(FileProtocol::YmodemG.name(), "Ymodem-G");
    assert_eq!(FileProtocol::Xmodem1K.name(), "Xmodem-1K");
    assert_eq!(FileProtocol::XmodemCrc.name(), "Xmodem-CRC");
    assert_eq!(FileProtocol::Xmodem.name(), "Xmodem");
}

/// Test error handling across protocols.
#[test]
fn test_xmodem_invalid_block_size() {
    let invalid_data = vec![0x42; 64]; // Invalid size
    let result = XmodemBlock::new(1, invalid_data, XmodemVariant::Checksum);
    assert!(result.is_err());
}

#[test]
fn test_xmodem_checksum_validation() {
    // Create a valid block
    let data = vec![0x42; 128];
    let block = XmodemBlock::new(1, data, XmodemVariant::Checksum).unwrap();
    let mut packet = block.serialize();

    // Corrupt the checksum
    let last_idx = packet.len() - 1;
    packet[last_idx] ^= 0xFF;

    // Deserialization should fail
    let result = XmodemBlock::deserialize(&packet, false);
    assert!(result.is_err());
}

#[test]
fn test_xmodem_crc_validation() {
    // Create a valid block
    let data = vec![0x42; 128];
    let block = XmodemBlock::new(1, data, XmodemVariant::Crc).unwrap();
    let mut packet = block.serialize();

    // Corrupt the CRC
    let last_idx = packet.len() - 1;
    packet[last_idx] ^= 0xFF;

    // Deserialization should fail
    let result = XmodemBlock::deserialize(&packet, true);
    assert!(result.is_err());
}

/// Integration test: Complete workflow simulation.
#[test]
fn test_complete_protocol_workflow() {
    // 1. User selects preferred protocol
    let preferred = FileProtocol::YmodemG;
    assert!(preferred.supports_batch());
    assert!(preferred.is_streaming());

    // 2. Create file metadata for batch
    let mut batch = YmodemBatch::new();
    batch.add_metadata(FileMetadata::with_size("file1.dat", 1024));
    batch.add_metadata(FileMetadata::with_size("file2.dat", 2048));
    assert_eq!(batch.len(), 2);

    // 3. Verify block size matches protocol
    assert_eq!(preferred.block_size(), 1024);

    // 4. Encode metadata
    let metadata = FileMetadata::with_size("file1.dat", 1024);
    let encoded = metadata.encode();
    assert_eq!(encoded.len(), 128);

    // 5. Verify protocol capabilities
    assert!(preferred.is_streaming());
    assert!(preferred.supports_batch());
    assert!(!preferred.supports_crash_recovery());
}
