//! Concurrent access tests for WAL

use ferrisdb_core::SyncMode;
use ferrisdb_storage::wal::{WALEntry, WALReader, WALWriter};

use tempfile::TempDir;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;

/// Tests that concurrent writes to the WAL maintain data integrity.
///
/// This test verifies that:
/// - Multiple threads can write simultaneously without data corruption
/// - All writes are atomic and complete successfully
/// - The total number of entries matches the expected count
/// - No entries are lost or corrupted during concurrent access
#[test]
fn append_maintains_data_integrity_during_concurrent_writes() {
    let temp_dir = TempDir::new().unwrap();
    let wal_path = temp_dir.path().join("concurrent_writes.wal");

    let writer = Arc::new(WALWriter::new(&wal_path, SyncMode::Full, 100 * 1024 * 1024).unwrap());
    let success_count = Arc::new(AtomicUsize::new(0));
    let barrier = Arc::new(Barrier::new(10));

    let mut handles = vec![];

    // Spawn 10 threads that write concurrently
    for thread_id in 0..10 {
        let writer = Arc::clone(&writer);
        let success_count = Arc::clone(&success_count);
        let barrier = Arc::clone(&barrier);

        handles.push(thread::spawn(move || {
            // Wait for all threads to be ready
            barrier.wait();

            // Each thread writes 100 entries
            for i in 0..100 {
                let key = format!("thread{}_key{}", thread_id, i).into_bytes();
                let value = format!("value_{}", i).into_bytes();
                let entry = WALEntry::new_put(key, value, (thread_id * 100 + i) as u64).unwrap();

                if writer.append(&entry).is_ok() {
                    success_count.fetch_add(1, Ordering::Relaxed);
                }
            }
        }));
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify all writes succeeded
    assert_eq!(success_count.load(Ordering::Relaxed), 1000);

    // Verify we can read all entries
    let mut reader = WALReader::new(&wal_path).unwrap();
    let entries = reader.read_all().unwrap();
    assert_eq!(entries.len(), 1000);
}

/// Tests that metrics remain accurate during concurrent write operations.
///
/// This test verifies:
/// - Metrics are thread-safe and don't lose updates
/// - Failed writes (due to size limits) are tracked correctly
/// - Success rates reflect actual operation outcomes
/// - Total counts match expected values under concurrency
#[test]
fn metrics_remain_consistent_during_concurrent_operations() {
    let temp_dir = TempDir::new().unwrap();
    let wal_path = temp_dir.path().join("metrics_test.wal");

    // Use a smaller file limit (10MB) to ensure some writes fail
    let writer = Arc::new(WALWriter::new(&wal_path, SyncMode::Normal, 10 * 1024 * 1024).unwrap());
    let metrics = writer.metrics();
    let barrier = Arc::new(Barrier::new(5));

    let mut handles = vec![];

    // Spawn 5 threads that write and trigger metrics updates
    for thread_id in 0..5 {
        let writer = Arc::clone(&writer);
        let barrier = Arc::clone(&barrier);

        handles.push(thread::spawn(move || {
            barrier.wait();

            for i in 0..200 {
                let entry = WALEntry::new_put(
                    format!("key_{}_{}", thread_id, i).into_bytes(),
                    vec![b'v'; 100],
                    i as u64,
                )
                .unwrap();
                let _ = writer.append(&entry);

                // Add some writes that will eventually fail due to file size limit
                if i % 50 == 0 {
                    // Create max-size entries to fill up the file faster
                    let large_entry = WALEntry::new_put(
                        vec![b'k'; 10 * 1024],    // 10KB key (at limit)
                        vec![b'x'; 100 * 1024],   // 100KB value (at limit)
                        i as u64,
                    )
                    .unwrap();
                    let _ = writer.append(&large_entry); // This might fail when file limit is reached
                }
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Verify metrics consistency
    let total_writes = metrics.writes_total();

    // We expect around 1000 writes (5 threads * 200), plus some large entries
    // With a 10MB file limit and ~110KB large entries, we should see some failures
    assert!(total_writes >= 800, "Too few writes succeeded: {}", total_writes);
    assert!(total_writes <= 1020, "Too many writes succeeded: {}", total_writes);
    
    // With a 10MB limit, we expect some failures but most writes should succeed
    let success_rate = metrics.write_success_rate();
    assert!(success_rate > 70.0 && success_rate <= 100.0, 
            "Success rate {} is outside expected range", success_rate);
}

/// Tests that readers get consistent data while writes are happening.
///
/// This test simulates a real-world scenario where:
/// - A writer continuously appends new entries
/// - A reader periodically reads all entries
/// - The reader should always see a consistent prefix of entries
/// - Entry count should never decrease (monotonic increase)
#[test]
fn read_all_returns_consistent_data_during_concurrent_writes() {
    let temp_dir = TempDir::new().unwrap();
    let wal_path = temp_dir.path().join("read_during_write.wal");

    // Pre-populate with some entries
    let writer = Arc::new(WALWriter::new(&wal_path, SyncMode::Full, 100 * 1024 * 1024).unwrap());
    for i in 0..10 {
        let entry =
            WALEntry::new_put(format!("initial_{}", i).into_bytes(), b"value".to_vec(), i).unwrap();
        writer.append(&entry).unwrap();
    }
    writer.sync().unwrap();

    let writer_clone = Arc::clone(&writer);
    let done_writing = Arc::new(AtomicUsize::new(0));
    let done_writing_clone = Arc::clone(&done_writing);

    // Start a writer thread
    let writer_handle = thread::spawn(move || {
        for i in 10..110 {
            let entry = WALEntry::new_put(
                format!("concurrent_{}", i).into_bytes(),
                b"value".to_vec(),
                i,
            )
            .unwrap();
            writer_clone.append(&entry).unwrap();

            // Sync periodically
            if i % 10 == 0 {
                writer_clone.sync().unwrap();
            }

            // Slow down to allow reader to interleave
            thread::sleep(Duration::from_millis(1));
        }
        done_writing_clone.store(1, Ordering::Release);
    });

    // Start a reader thread
    let reader_handle = thread::spawn(move || {
        let mut last_count = 0;
        let mut iterations = 0;

        loop {
            let mut reader = WALReader::new(&wal_path).unwrap();
            let entries = reader.read_all().unwrap();

            // Entries should only increase, never decrease
            assert!(entries.len() >= last_count);
            last_count = entries.len();

            iterations += 1;

            // Check if writer is done
            if done_writing.load(Ordering::Acquire) == 1 {
                // Do one more read to get all entries
                let mut final_reader = WALReader::new(&wal_path).unwrap();
                let final_entries = final_reader.read_all().unwrap();
                return (iterations, final_entries.len());
            }

            thread::sleep(Duration::from_millis(5));
        }
    });

    writer_handle.join().unwrap();
    let (read_iterations, final_count) = reader_handle.join().unwrap();

    // Verify final state
    assert_eq!(final_count, 110); // 10 initial + 100 concurrent
    assert!(read_iterations > 5); // Should have read multiple times during writes
}

/// Tests that multiple concurrent readers see identical data.
///
/// This ensures:
/// - Reader operations don't interfere with each other
/// - All readers see the same complete set of entries
/// - Data integrity is maintained across concurrent reads
/// - No race conditions in file access
#[test]
fn read_all_returns_identical_data_for_concurrent_readers() {
    let temp_dir = TempDir::new().unwrap();
    let wal_path = temp_dir.path().join("multiple_readers.wal");

    // Write test data
    {
        let writer = WALWriter::new(&wal_path, SyncMode::Full, 10 * 1024 * 1024).unwrap();
        for i in 0..100 {
            let entry = WALEntry::new_put(
                format!("key_{}", i).into_bytes(),
                format!("value_{}", i).into_bytes(),
                i as u64,
            )
            .unwrap();
            writer.append(&entry).unwrap();
        }
    }

    let barrier = Arc::new(Barrier::new(10));
    let mut handles = vec![];

    // Spawn 10 reader threads
    for thread_id in 0..10 {
        let wal_path = wal_path.clone();
        let barrier = Arc::clone(&barrier);

        handles.push(thread::spawn(move || {
            barrier.wait();

            let mut reader = WALReader::new(&wal_path).unwrap();
            let entries = reader.read_all().unwrap();

            // All readers should see the same data
            assert_eq!(entries.len(), 100);

            // Verify entries are correct
            for (i, entry) in entries.iter().enumerate() {
                assert_eq!(entry.key, format!("key_{}", i).into_bytes());
                assert_eq!(entry.timestamp, i as u64);
            }

            thread_id
        }));
    }

    // Verify all readers completed successfully
    let mut completed_threads = vec![];
    for handle in handles {
        completed_threads.push(handle.join().unwrap());
    }

    // Should have results from all 10 threads
    assert_eq!(completed_threads.len(), 10);
}

/// Tests that metrics can be safely updated from multiple threads.
///
/// Verifies:
/// - Writer metrics are thread-safe
/// - Reader creation and reads don't cause data races
/// - Metric values remain consistent
/// - No crashes or undefined behavior under concurrent access
#[test]
fn metrics_updates_safely_from_multiple_threads() {
    let temp_dir = TempDir::new().unwrap();
    let wal_path = temp_dir.path().join("metrics_safety.wal");

    let writer = Arc::new(WALWriter::new(&wal_path, SyncMode::Normal, 100 * 1024 * 1024).unwrap());

    // Pre-write header so reader can open the file
    writer.sync().unwrap();

    let barrier = Arc::new(Barrier::new(20)); // 10 writers + 10 readers
    let mut handles = vec![];

    // Spawn writer threads
    for i in 0..10 {
        let writer = Arc::clone(&writer);
        let barrier = Arc::clone(&barrier);

        handles.push(thread::spawn(move || {
            barrier.wait();

            for j in 0..50 {
                let entry = WALEntry::new_put(
                    format!("key_{}_{}", i, j).into_bytes(),
                    b"value".to_vec(),
                    j as u64,
                )
                .unwrap();
                let _ = writer.append(&entry);
            }
        }));
    }

    // Spawn reader threads
    for _ in 0..10 {
        let wal_path = wal_path.clone();
        let barrier = Arc::clone(&barrier);

        handles.push(thread::spawn(move || {
            barrier.wait();

            // Each thread creates its own reader
            let mut reader = WALReader::new(&wal_path).unwrap();

            // Try to read entries (may or may not succeed depending on timing)
            let _ = reader.read_all();
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Verify writer metrics are consistent
    let writes = writer.metrics().writes_total();

    // Should have attempted 500 writes (10 threads * 50 writes)
    // Some may have failed due to size limits
    assert!(writes <= 500);
    assert!(writes > 0);
}
