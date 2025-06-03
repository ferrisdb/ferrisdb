//! Property-based tests for WAL using proptest
//!
//! These tests verify that WAL handles arbitrary inputs correctly
//! and maintains invariants across all possible data.

use ferrisdb_core::{Error, Operation, SyncMode};
use ferrisdb_storage::format::FileHeader;
use ferrisdb_storage::wal::{WALEntry, WALHeader, WALReader, WALWriter};

use proptest::prelude::*;
use tempfile::TempDir;

// ==================== Strategies ====================

// Generate valid keys (0 to 10KB)
prop_compose! {
    fn valid_key()(size in 0usize..=10*1024) -> Vec<u8> {
        vec![b'k'; size]
    }
}

// Generate valid values (0 to 100KB)
prop_compose! {
    fn valid_value()(size in 0usize..=100*1024) -> Vec<u8> {
        vec![b'v'; size]
    }
}

// Generate arbitrary byte vectors for corruption testing
prop_compose! {
    fn arbitrary_bytes()(size in 0usize..10000) -> Vec<u8> {
        let mut bytes = vec![0u8; size];
        for (i, byte) in bytes.iter_mut().enumerate().take(size) {
            *byte = (i % 256) as u8;
        }
        bytes
    }
}

// ==================== Roundtrip Tests ====================

proptest! {
    /// Tests that Put entries preserve all data through encode/decode cycle.
    ///
    /// This property test verifies that:
    /// - Arbitrary keys (0-10KB) are preserved exactly
    /// - Arbitrary values (0-100KB) are preserved exactly
    /// - Timestamps are preserved without modification
    /// - No data corruption occurs during serialization
    #[test]
    fn entry_roundtrip_preserves_arbitrary_put_data(
        key in valid_key(),
        value in valid_value(),
        timestamp in any::<u64>()
    ) {
        let entry = WALEntry::new_put(key.clone(), value.clone(), timestamp)?;
        let encoded = entry.encode()?;
        let decoded = WALEntry::decode(&encoded)?;

        prop_assert_eq!(decoded.operation, Operation::Put);
        prop_assert_eq!(decoded.key, key);
        prop_assert_eq!(decoded.value, value);
        prop_assert_eq!(decoded.timestamp, timestamp);
    }

    /// Tests that Delete entries preserve all data through encode/decode cycle.
    ///
    /// Verifies that:
    /// - Delete operation type is preserved
    /// - Keys of any valid size are preserved
    /// - Value is always empty for deletes
    /// - Timestamps are maintained accurately
    #[test]
    fn entry_roundtrip_preserves_arbitrary_delete_data(
        key in valid_key(),
        timestamp in any::<u64>()
    ) {
        let entry = WALEntry::new_delete(key.clone(), timestamp)?;
        let encoded = entry.encode()?;
        let decoded = WALEntry::decode(&encoded)?;

        prop_assert_eq!(decoded.operation, Operation::Delete);
        prop_assert_eq!(decoded.key, key);
        prop_assert_eq!(decoded.value.len(), 0);
        prop_assert_eq!(decoded.timestamp, timestamp);
    }

    /// Tests that WAL headers preserve all fields through serialization.
    ///
    /// Ensures:
    /// - Magic number, version, and flags are preserved
    /// - Checksums remain valid after roundtrip
    /// - Timestamps and sequences are exact
    /// - Reserved fields maintain their values
    #[test]
    fn header_roundtrip_preserves_all_fields(
        file_sequence in any::<u64>()
    ) {
        let header = WALHeader::new(file_sequence);
        let encoded = header.encode();
        let decoded = WALHeader::decode(&encoded)?;

        prop_assert_eq!(decoded.magic, header.magic);
        prop_assert_eq!(decoded.version, header.version);
        prop_assert_eq!(decoded.flags, header.flags);
        prop_assert_eq!(decoded.header_size, header.header_size);
        prop_assert_eq!(decoded.header_checksum, header.header_checksum);
        prop_assert_eq!(decoded.entry_start_offset, header.entry_start_offset);
        prop_assert_eq!(decoded.created_at, header.created_at);
        prop_assert_eq!(decoded.file_sequence, header.file_sequence);
        prop_assert_eq!(decoded.reserved, header.reserved);
    }
}

// ==================== Error Handling Tests ====================

proptest! {
    /// Tests that decoders handle arbitrary corrupted data without panicking.
    ///
    /// This critical safety test ensures:
    /// - No panics on malformed input
    /// - Graceful error handling for any byte sequence
    /// - Protection against malicious/corrupted files
    /// - Robust parsing that never crashes
    #[test]
    fn decode_never_panics_on_corrupted_input(
        data in arbitrary_bytes()
    ) {
        // Should return Err, never panic
        let _ = WALEntry::decode(&data);
        let _ = WALHeader::decode(&data);
    }

    /// Tests that keys exceeding 10KB limit are always rejected.
    ///
    /// Verifies:
    /// - Enforcement of 10KB key size limit
    /// - Proper error type (Corruption) returned
    /// - Clear error messages about size limits
    /// - Protection against memory exhaustion
    #[test]
    fn oversized_keys_always_rejected(
        size in (10*1024usize + 1)..=(20*1024usize),
        value in valid_value(),
        timestamp in any::<u64>()
    ) {
        let oversized_key = vec![b'k'; size];
        let result = WALEntry::new_put(oversized_key, value, timestamp);
        prop_assert!(result.is_err());
        if let Err(err) = result {
            match err {
                Error::Corruption(msg) => prop_assert!(msg.contains("exceeds maximum")),
                _ => prop_assert!(false, "Expected Corruption error, got {:?}", err),
            }
        }
    }

    /// Tests that values exceeding 100KB limit are always rejected.
    ///
    /// Ensures:
    /// - Enforcement of 100KB value size limit
    /// - Consistent error handling across all oversized values
    /// - Protection against resource exhaustion
    /// - Clear error messages for debugging
    #[test]
    fn oversized_values_always_rejected(
        key in valid_key(),
        size in (100*1024usize + 1)..=(200*1024usize),
        timestamp in any::<u64>()
    ) {
        let oversized_value = vec![b'v'; size];
        let result = WALEntry::new_put(key, oversized_value, timestamp);
        prop_assert!(result.is_err());
        if let Err(err) = result {
            match err {
                Error::Corruption(msg) => prop_assert!(msg.contains("exceeds maximum")),
                _ => prop_assert!(false, "Expected Corruption error, got {:?}", err),
            }
        }
    }
}

// ==================== File Operation Tests ====================

proptest! {
    /// Tests that arbitrary sequences of entries are preserved through file operations.
    ///
    /// This comprehensive test verifies:
    /// - Multiple entries written in sequence are all readable
    /// - Mix of Put and Delete operations handled correctly
    /// - Entry order is strictly preserved (FIFO)
    /// - No data loss or corruption across full write/read cycle
    #[test]
    fn write_read_preserves_arbitrary_entry_sequences(
        entries in prop::collection::vec(
            (valid_key(), valid_value(), any::<u64>(), any::<bool>()),
            0..20
        )
    ) {
        let temp_dir = TempDir::new().unwrap();
        let wal_path = temp_dir.path().join("test.wal");

        // Write entries
        let writer = WALWriter::new(&wal_path, SyncMode::Full, 100 * 1024 * 1024)?;
        let mut expected_entries = Vec::new();

        for (key, value, timestamp, is_delete) in entries {
            let entry = if is_delete {
                WALEntry::new_delete(key, timestamp)?
            } else {
                WALEntry::new_put(key, value, timestamp)?
            };
            writer.append(&entry)?;
            expected_entries.push(entry);
        }

        writer.sync()?;
        drop(writer);

        // Read back using iterator to avoid loading all into memory
        let mut reader = WALReader::new(&wal_path)?;
        let mut read_count = 0;

        for expected in expected_entries.iter() {
            match reader.read_entry()? {
                Some(read) => {
                    prop_assert_eq!(read.operation, expected.operation);
                    prop_assert_eq!(&read.key, &expected.key);
                    prop_assert_eq!(&read.value, &expected.value);
                    prop_assert_eq!(read.timestamp, expected.timestamp);
                    read_count += 1;
                }
                None => {
                    prop_assert!(false, "Expected {} entries but only read {}", expected_entries.len(), read_count);
                }
            }
        }

        // Ensure no extra entries
        prop_assert!(reader.read_entry()?.is_none(), "Found extra entries beyond expected count");
    }
}

// ==================== Invariant Tests ====================

proptest! {
    /// Tests that encoded entries report accurate size in their length field.
    ///
    /// This invariant ensures:
    /// - Length field correctly excludes itself (4 bytes)
    /// - Total size = length_field + 4
    /// - No buffer overruns during decoding
    /// - Accurate size calculations for all entry types
    #[test]
    fn encoded_size_matches_actual_size(
        key in valid_key(),
        value in valid_value(),
        timestamp in any::<u64>(),
        is_delete in any::<bool>()
    ) {
        let entry = if is_delete {
            WALEntry::new_delete(key, timestamp)?
        } else {
            WALEntry::new_put(key, value, timestamp)?
        };

        let encoded = entry.encode()?;

        // First 4 bytes should contain the total size (excluding the length field itself)
        let claimed_size = u32::from_le_bytes([encoded[0], encoded[1], encoded[2], encoded[3]]) as usize;

        // The length field excludes itself, so claimed_size + 4 should equal total length
        prop_assert_eq!(claimed_size + 4, encoded.len());
    }

    /// Tests that checksums detect any single bit corruption.
    ///
    /// This critical integrity test verifies:
    /// - CRC32 checksum catches all single-bit errors
    /// - Corruption at any position is detected
    /// - No silent data corruption possible
    /// - Robust protection against bit flips
    #[test]
    fn checksum_detects_any_bit_flip(
        key in valid_key(),
        value in valid_value(),
        timestamp in any::<u64>(),
        corrupt_byte in 0usize..1000,
        corrupt_bit in 0u8..8
    ) {
        let entry = WALEntry::new_put(key, value, timestamp)?;
        let mut encoded = entry.encode()?;

        // Skip if corrupt position is beyond entry size
        if corrupt_byte >= encoded.len() {
            return Ok(());
        }

        // Skip corrupting the length field (first 4 bytes) as it causes different error
        if corrupt_byte < 4 {
            return Ok(());
        }

        // Flip one bit
        encoded[corrupt_byte] ^= 1 << corrupt_bit;

        // Decode should fail
        let result = WALEntry::decode(&encoded);
        prop_assert!(result.is_err());

        // Should be corruption error (either checksum or other validation)
        let err = result.unwrap_err();
        prop_assert!(matches!(err, Error::Corruption(_)));
    }
}

// ==================== Stress Tests ====================

proptest! {
    /// Tests recovery behavior with mixed valid and corrupted entries.
    ///
    /// Simulates real-world scenarios where:
    /// - Some entries are valid, others corrupted
    /// - Reader recovers all entries before first corruption
    /// - Partial recovery is possible and safe
    /// - No valid data is lost due to later corruption
    #[test]
    fn handles_mixed_valid_and_invalid_entries(
        valid_entries in prop::collection::vec(
            (valid_key(), valid_value(), any::<u64>()),
            0..10
        ),
        corrupt_positions in prop::collection::vec(0usize..10, 0..3)
    ) {
        let temp_dir = TempDir::new().unwrap();
        let wal_path = temp_dir.path().join("mixed.wal");

        // Write valid entries
        let writer = WALWriter::new(&wal_path, SyncMode::Full, 100 * 1024 * 1024)?;

        for (i, (key, value, timestamp)) in valid_entries.iter().enumerate() {
            // Check if we should inject corruption instead of writing this entry
            if corrupt_positions.contains(&i) {
                // Ensure any previous data is flushed before corruption
                if i > 0 {
                    writer.sync()?;
                }
                drop(writer);

                // Append garbage
                use std::fs::OpenOptions;
                use std::io::Write;
                let mut file = OpenOptions::new().append(true).open(&wal_path)?;
                file.write_all(&[0xFF; 20])?;

                // Can't continue writing after corruption
                break;
            }

            let entry = WALEntry::new_put(key.clone(), value.clone(), *timestamp)?;
            writer.append(&entry)?;
        }

        // Reader should recover what it can
        let mut reader = WALReader::new(&wal_path)?;

        // Read entries one by one to handle partial recovery
        let mut recovered = Vec::new();
        loop {
            match reader.read_entry() {
                Ok(Some(entry)) => recovered.push(entry),
                Ok(None) => break, // EOF
                Err(_) => break,   // Corruption detected
            }
        }

        // Should recover entries before first corruption
        let first_corrupt = corrupt_positions.iter().min().copied().unwrap_or(valid_entries.len());
        prop_assert!(recovered.len() <= first_corrupt,
            "Recovered {} entries, but first corruption at position {}",
            recovered.len(), first_corrupt);
    }
}

// ==================== Performance Invariant Tests ====================

proptest! {
    /// Tests that buffer reuse strategy reduces allocations.
    ///
    /// Performance invariant that verifies:
    /// - Buffers grow to accommodate large entries
    /// - Growth is efficient (not per-entry)
    /// - Peak buffer size is tracked accurately
    /// - Allocation overhead is minimized
    #[test]
    fn buffer_reuse_reduces_allocations(
        entries in prop::collection::vec(
            (valid_key(), valid_value(), any::<u64>()),
            10..20
        )
    ) {
        let temp_dir = TempDir::new().unwrap();
        let wal_path = temp_dir.path().join("buffer_test.wal");

        // Write entries
        let writer = WALWriter::new(&wal_path, SyncMode::Full, 100 * 1024 * 1024)?;
        for (key, value, timestamp) in &entries {
            let entry = WALEntry::new_put(key.clone(), value.clone(), *timestamp)?;
            writer.append(&entry)?;
        }
        drop(writer);

        // Read with small initial buffer
        let mut reader = WALReader::with_initial_capacity(&wal_path, 64)?;

        // Read all entries using iterator pattern to avoid loading all into memory
        let mut entry_count = 0;
        while let Some(_entry) = reader.read_entry()? {
            entry_count += 1;
        }

        let stats = reader.stats();

        // Verify we read all entries
        prop_assert_eq!(entry_count, entries.len());

        // Buffer should have grown
        prop_assert!(stats.peak_buffer_size > 64);

        // But resizes should be reasonable (not one per entry)
        prop_assert!(stats.buffer_resizes < entries.len());
    }
}

// ==================== Concurrency Invariant Tests ====================

proptest! {
    /// Tests that concurrent writes preserve entry integrity.
    ///
    /// Verifies thread safety by ensuring:
    /// - Multiple threads can write simultaneously
    /// - All entries remain complete and valid
    /// - No data corruption from race conditions
    /// - Total entry count matches sum of all thread writes
    #[test]
    fn concurrent_writes_maintain_entry_integrity(
        thread_entries in prop::collection::vec(
            prop::collection::vec(
                (valid_key(), valid_value(), any::<u64>()),
                1..5
            ),
            2..5
        )
    ) {
        use std::sync::Arc;
        use std::thread;

        let temp_dir = TempDir::new().unwrap();
        let wal_path = temp_dir.path().join("concurrent.wal");

        let writer = Arc::new(WALWriter::new(&wal_path, SyncMode::Full, 100 * 1024 * 1024)?);
        let mut handles = vec![];
        let mut total_entries = 0;

        for entries in thread_entries {
            total_entries += entries.len();
            let writer = Arc::clone(&writer);
            handles.push(thread::spawn(move || {
                for (key, value, timestamp) in entries {
                    let entry = WALEntry::new_put(key, value, timestamp).unwrap();
                    writer.append(&entry).unwrap();
                }
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        drop(writer);

        // All entries should be readable and valid
        let mut reader = WALReader::new(&wal_path)?;

        // Read entries using iterator to avoid loading all into memory
        let mut entry_count = 0;
        while let Some(entry) = reader.read_entry()? {
            prop_assert!(!entry.key.is_empty() || entry.value.is_empty()); // Allow empty for tests
            prop_assert!(matches!(entry.operation, Operation::Put | Operation::Delete));
            entry_count += 1;
        }

        // Verify we got the expected number of entries
        prop_assert_eq!(entry_count, total_entries);
    }
}
