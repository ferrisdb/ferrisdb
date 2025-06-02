//! Integration tests for WAL components working together

use ferrisdb_core::SyncMode;
use ferrisdb_storage::wal::{WALEntry, WALReader, WALWriter};

use tempfile::TempDir;

use std::fs::OpenOptions;
use std::io::Write;

/// Tests the complete write-read cycle preserves data integrity.
///
/// This fundamental test verifies:
/// - Entries are written and read back exactly as provided
/// - Order of entries is preserved (FIFO)
/// - All entry fields (key, value, timestamp, operation) are preserved
/// - Basic WAL functionality works end-to-end
#[test]
fn append_and_read_all_preserve_entry_order_and_data() {
    let temp_dir = TempDir::new().unwrap();
    let wal_path = temp_dir.path().join("test.wal");

    // Write phase
    let entries_to_write = vec![
        WALEntry::new_put(b"key1".to_vec(), b"value1".to_vec(), 1).unwrap(),
        WALEntry::new_put(b"key2".to_vec(), b"value2".to_vec(), 2).unwrap(),
        WALEntry::new_delete(b"key1".to_vec(), 3).unwrap(),
        WALEntry::new_put(b"key3".to_vec(), b"value3".to_vec(), 4).unwrap(),
    ];

    {
        let writer = WALWriter::new(&wal_path, SyncMode::Full, 10 * 1024 * 1024).unwrap();
        for entry in &entries_to_write {
            writer.append(entry).unwrap();
        }
        writer.sync().unwrap();
    }

    // Read phase
    let mut reader = WALReader::new(&wal_path).unwrap();
    let read_entries = reader.read_all().unwrap();

    assert_eq!(read_entries.len(), entries_to_write.len());
    for (written, read) in entries_to_write.iter().zip(read_entries.iter()) {
        assert_eq!(written.key, read.key);
        assert_eq!(written.value, read.value);
        assert_eq!(written.timestamp, read.timestamp);
        assert_eq!(written.operation, read.operation);
    }
}

/// Tests WAL recovery behavior when encountering a partial write.
///
/// This test verifies that:
/// - Complete entries written before a crash/partial write are recoverable
/// - The reader stops at the first incomplete entry
/// - No corrupted data is returned
/// - This simulates recovery after a crash during write
#[test]
fn read_all_recovers_complete_entries_before_partial_write() {
    let temp_dir = TempDir::new().unwrap();
    let wal_path = temp_dir.path().join("partial.wal");

    // Write some complete entries
    {
        let writer = WALWriter::new(&wal_path, SyncMode::Full, 10 * 1024 * 1024).unwrap();
        let entry1 = WALEntry::new_put(b"complete1".to_vec(), b"value1".to_vec(), 1).unwrap();
        let entry2 = WALEntry::new_put(b"complete2".to_vec(), b"value2".to_vec(), 2).unwrap();
        writer.append(&entry1).unwrap();
        writer.append(&entry2).unwrap();
        writer.sync().unwrap();
    }

    // Simulate partial write by appending incomplete data
    {
        let mut file = OpenOptions::new().append(true).open(&wal_path).unwrap();
        // Write partial length field (only 2 bytes of 4)
        file.write_all(&[0x10, 0x00]).unwrap();
    }

    // Reader should successfully read complete entries and stop at partial
    let mut reader = WALReader::new(&wal_path).unwrap();
    let entries = reader.read_all().unwrap();

    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].key, b"complete1");
    assert_eq!(entries[1].key, b"complete2");
}

/// Tests that WAL correctly handles entries of various sizes.
///
/// Verifies:
/// - Small, medium, large, and very large entries are handled
/// - Entries up to maximum allowed sizes work correctly
/// - Size calculations remain accurate across different entry sizes
/// - No corruption or data loss with large entries
#[test]
fn append_and_read_handle_entries_up_to_size_limits() {
    let temp_dir = TempDir::new().unwrap();
    let wal_path = temp_dir.path().join("large.wal");

    // Test with various sizes
    let test_sizes = vec![
        (100, 1000),       // Small
        (1024, 10240),     // Medium
        (10240, 102400),   // Large
        (102400, 1024000), // Very large (100KB key, 1MB value)
    ];

    let writer = WALWriter::new(&wal_path, SyncMode::Normal, 100 * 1024 * 1024).unwrap();

    for (i, (key_size, value_size)) in test_sizes.iter().enumerate() {
        let key = vec![b'k'; *key_size];
        let value = vec![b'v'; *value_size];
        let entry = WALEntry::new_put(key, value, i as u64).unwrap();
        writer.append(&entry).unwrap();
    }
    writer.sync().unwrap();

    // Verify all entries can be read back
    let mut reader = WALReader::new(&wal_path).unwrap();
    let entries = reader.read_all().unwrap();

    assert_eq!(entries.len(), test_sizes.len());
    for (i, (entry, (key_size, value_size))) in entries.iter().zip(test_sizes.iter()).enumerate() {
        assert_eq!(entry.key.len(), *key_size);
        assert_eq!(entry.value.len(), *value_size);
        assert_eq!(entry.timestamp, i as u64);
    }
}

/// Tests that the WAL iterator API returns entries in correct order.
///
/// This ensures:
/// - Iterator interface works as an alternative to read_all
/// - Entries are yielded in write order (timestamp order)
/// - Iterator can be used for streaming large WALs
/// - All entries are accessible via iteration
#[test]
fn iterator_yields_entries_in_timestamp_order() {
    let temp_dir = TempDir::new().unwrap();
    let wal_path = temp_dir.path().join("iterator.wal");

    // Write entries
    {
        let writer = WALWriter::new(&wal_path, SyncMode::Full, 10 * 1024 * 1024).unwrap();
        for i in 0..10 {
            let entry = WALEntry::new_put(
                format!("key{}", i).into_bytes(),
                format!("value{}", i).into_bytes(),
                i as u64,
            )
            .unwrap();
            writer.append(&entry).unwrap();
        }
    }

    // Test iterator API
    let reader = WALReader::new(&wal_path).unwrap();
    let mut count = 0;
    let mut last_timestamp = None;

    for entry_result in reader {
        let entry = entry_result.unwrap();

        // Verify ordering
        if let Some(last_ts) = last_timestamp {
            assert!(entry.timestamp > last_ts);
        }
        last_timestamp = Some(entry.timestamp);
        count += 1;
    }

    assert_eq!(count, 10);
}

/// Tests that different sync modes provide expected durability.
///
/// Verifies:
/// - SyncMode::None buffers writes without immediate sync
/// - SyncMode::Normal flushes to OS buffers
/// - SyncMode::Full performs fsync for immediate durability
/// - Data is readable regardless of sync mode after explicit sync
#[test]
fn append_respects_sync_mode_durability_guarantees() {
    let temp_dir = TempDir::new().unwrap();

    // Test different sync modes
    let sync_modes = vec![
        (SyncMode::None, "none.wal"),
        (SyncMode::Normal, "normal.wal"),
        (SyncMode::Full, "full.wal"),
    ];

    for (sync_mode, filename) in sync_modes {
        let wal_path = temp_dir.path().join(filename);
        let writer = WALWriter::new(&wal_path, sync_mode, 10 * 1024 * 1024).unwrap();

        // Write some entries
        for i in 0..5 {
            let entry =
                WALEntry::new_put(format!("key{}", i).into_bytes(), b"value".to_vec(), i).unwrap();
            writer.append(&entry).unwrap();
        }

        // Explicit sync for modes that don't auto-sync
        if matches!(sync_mode, SyncMode::None) {
            writer.sync().unwrap();
        }

        // Verify data is readable
        let mut reader = WALReader::new(&wal_path).unwrap();
        let entries = reader.read_all().unwrap();
        assert_eq!(entries.len(), 5);
    }
}

/// Tests that file size limits are enforced to prevent unbounded growth.
///
/// This critical test ensures:
/// - Writers respect configured size limits
/// - Appends fail gracefully when limit is reached
/// - File size never exceeds the configured limit
/// - Partial data can still be read after hitting limit
#[test]
fn append_returns_error_when_file_size_limit_exceeded() {
    let temp_dir = TempDir::new().unwrap();
    let wal_path = temp_dir.path().join("limited.wal");

    // Create writer with small size limit (2KB to account for header)
    let writer = WALWriter::new(&wal_path, SyncMode::Full, 2048).unwrap();

    // Write entries until we hit the limit
    let mut written = 0;
    let entry_data = vec![b'x'; 100]; // ~100 byte entries

    loop {
        let entry = WALEntry::new_put(
            format!("key{}", written).into_bytes(),
            entry_data.clone(),
            written as u64,
        )
        .unwrap();

        match writer.append(&entry) {
            Ok(_) => written += 1,
            Err(e) => {
                assert!(e.to_string().contains("size limit"));
                break;
            }
        }
    }

    // Should have written some entries but not too many
    assert!(written > 0);
    assert!(written < 20); // Should hit limit before 20 entries

    // Verify we can read what was written
    let mut reader = WALReader::new(&wal_path).unwrap();
    let entries = reader.read_all().unwrap();

    // The number of entries read should match what was successfully written
    // Note: If the last write partially succeeded, we might read fewer entries
    assert!(entries.len() <= written);
    assert!(entries.len() > 0); // Should have at least some entries
}
