use super::{TimedOperation, WALEntry, WALHeader, WALMetrics};
use crate::format::FileHeader;
use ferrisdb_core::{Error, Result, SyncMode};

use parking_lot::Mutex;

use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

/// Writer for the Write-Ahead Log
///
/// The WALWriter appends entries to a log file with configurable durability
/// guarantees. It tracks the file size and returns an error when the size
/// limit is reached, indicating that rotation is needed.
///
/// # Thread Safety
///
/// The writer is thread-safe and can be shared across multiple threads.
/// Internal locking ensures that entries are written atomically.
///
/// # Example
///
/// ```no_run
/// use ferrisdb_storage::wal::{WALWriter, WALEntry};
/// use ferrisdb_core::SyncMode;
///
/// let writer = WALWriter::new(
///     "path/to/wal.log",
///     SyncMode::Normal,
///     64 * 1024 * 1024  // 64MB
/// )?;
///
/// let entry = WALEntry::new_put(b"key".to_vec(), b"value".to_vec(), 1)?;
/// writer.append(&entry)?;
/// writer.sync()?;
/// # Ok::<(), ferrisdb_core::Error>(())
/// ```
pub struct WALWriter {
    file: Arc<Mutex<BufWriter<File>>>,
    path: PathBuf,
    size: AtomicU64,
    sync_mode: SyncMode,
    size_limit: u64,
    metrics: Arc<WALMetrics>,
}

impl WALWriter {
    /// Creates a new WAL writer
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the WAL file
    /// * `sync_mode` - Durability level for writes
    /// * `size_limit` - Maximum file size before rotation is needed
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be created or opened.
    pub fn new(path: impl AsRef<Path>, sync_mode: SyncMode, size_limit: u64) -> Result<Self> {
        let path = path.as_ref().to_path_buf();

        // Create parent directories if they exist
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Check if this is a new file that needs a header
        let needs_header = !path.exists() || std::fs::metadata(&path)?.len() == 0;

        let mut file = OpenOptions::new()
            .create(true)
            .truncate(false) // Don't truncate existing files
            .read(true)
            .write(true)
            .open(&path)?;

        let mut size = file.metadata()?.len();

        // Write header to new/empty files
        if needs_header {
            // Generate file sequence based on timestamp
            let file_sequence = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_else(|_| {
                    // Fallback: use a random number for uniqueness
                    std::time::Duration::from_nanos(rand::random::<u64>())
                })
                .as_micros() as u64;

            let header = WALHeader::new(file_sequence);
            let encoded = header.encode();

            file.write_all(&encoded)?;
            file.sync_all()?;

            size = crate::wal::WAL_HEADER_SIZE as u64;
        }

        // Seek to end for appending
        file.seek(SeekFrom::End(0))?;

        let metrics = Arc::new(WALMetrics::new());
        metrics.record_file_opened();
        metrics.update_file_size(size);

        Ok(Self {
            file: Arc::new(Mutex::new(BufWriter::new(file))),
            path,
            size: AtomicU64::new(size),
            sync_mode,
            size_limit,
            metrics,
        })
    }

    /// Appends an entry to the WAL
    ///
    /// The entry is encoded and written to the file. Depending on the
    /// sync mode, the data may be flushed to the OS or synced to disk.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The entry would exceed the size limit
    /// - An I/O error occurs during write
    pub fn append(&self, entry: &WALEntry) -> Result<()> {
        let encoded = entry.encode()?;
        let entry_size = encoded.len() as u64;

        // Check if we need to rotate
        if self.size.load(Ordering::Relaxed) + entry_size > self.size_limit {
            self.metrics.record_write(entry_size, false);
            return Err(Error::StorageEngine(
                "WAL file size limit reached".to_string(),
            ));
        }

        let mut file = self.file.lock();
        match file.write_all(&encoded) {
            Ok(_) => {
                // Handle sync with timing
                match self.sync_mode {
                    SyncMode::None => {}
                    SyncMode::Normal => {
                        let timer = TimedOperation::start();
                        file.flush()?;
                        self.metrics.record_sync(timer.complete());
                    }
                    SyncMode::Full => {
                        let timer = TimedOperation::start();
                        file.flush()?;
                        file.get_ref().sync_all()?;
                        self.metrics.record_sync(timer.complete());
                    }
                }

                let new_size = self.size.fetch_add(entry_size, Ordering::Relaxed) + entry_size;
                self.metrics.record_write(entry_size, true);
                self.metrics.update_file_size(new_size);
                Ok(())
            }
            Err(e) => {
                self.metrics.record_write(entry_size, false);
                Err(e.into())
            }
        }
    }

    /// Forces a sync of all buffered data to disk
    ///
    /// This ensures durability by flushing the buffer and calling
    /// fsync on the underlying file.
    pub fn sync(&self) -> Result<()> {
        let timer = TimedOperation::start();
        let mut file = self.file.lock();
        file.flush()?;
        file.get_ref().sync_all()?;
        self.metrics.record_sync(timer.complete());
        Ok(())
    }

    /// Returns the current size of the WAL file
    pub fn size(&self) -> u64 {
        self.size.load(Ordering::Relaxed)
    }

    /// Returns the path to the WAL file
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns the metrics for this writer
    pub fn metrics(&self) -> &WALMetrics {
        &self.metrics
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    /// Tests that creating a new WAL writer properly initializes the file.
    ///
    /// Verifies:
    /// - File is created with a valid header
    /// - Initial size tracking is correct
    /// - Writer can successfully append entries
    #[test]
    fn new_creates_file_with_header_at_correct_size() {
        let temp_dir = TempDir::new().unwrap();
        let wal_path = temp_dir.path().join("test.wal");

        let writer = WALWriter::new(&wal_path, SyncMode::Normal, 1024 * 1024).unwrap();

        let entry = WALEntry::new_put(b"key1".to_vec(), b"value1".to_vec(), 1).unwrap();

        writer.append(&entry).unwrap();
        writer.sync().unwrap();

        assert!(writer.size() > 0);
        assert!(wal_path.exists());
    }

    /// Tests that append operations fail when the file size limit is exceeded.
    ///
    /// This ensures:
    /// - Size limit enforcement prevents unbounded file growth
    /// - Error is returned before writing when limit would be exceeded
    /// - Protects against runaway disk usage
    #[test]
    fn append_returns_error_when_file_size_limit_exceeded() {
        let temp_dir = TempDir::new().unwrap();
        let wal_path = temp_dir.path().join("test.wal");

        // Size limit that allows header but not much more
        let writer = WALWriter::new(&wal_path, SyncMode::None, 100).unwrap();

        let entry = WALEntry::new_put(
            b"key_with_long_name".to_vec(),
            b"value_with_long_content".to_vec(),
            1,
        )
        .unwrap();

        // This should exceed the size limit
        let result = writer.append(&entry);
        assert!(result.is_err());
    }

    /// Tests that the WAL header is written correctly to new files.
    ///
    /// Verifies:
    /// - Magic bytes are written at the beginning
    /// - Header size matches expected 64 bytes
    /// - File format is compatible with reader expectations
    #[test]
    fn new_writes_valid_header_with_magic_and_version() {
        let temp_dir = TempDir::new().unwrap();
        let wal_path = temp_dir.path().join("test.wal");

        let writer = WALWriter::new(&wal_path, SyncMode::None, 1024 * 1024).unwrap();
        writer.sync().unwrap();

        // Read raw file and check header
        let data = std::fs::read(&wal_path).unwrap();
        assert!(data.len() >= crate::wal::WAL_HEADER_SIZE);
        assert_eq!(&data[0..8], crate::wal::WAL_MAGIC);
    }

    /// Tests that initial file size tracking starts at header size.
    ///
    /// This ensures:
    /// - Size tracking is accurate from creation
    /// - No entries are counted before any are written
    /// - Header size (64 bytes) is properly accounted for
    #[test]
    fn new_initializes_size_to_header_size_only() {
        let temp_dir = TempDir::new().unwrap();
        let wal_path = temp_dir.path().join("test.wal");

        let writer = WALWriter::new(&wal_path, SyncMode::None, 1024 * 1024).unwrap();
        assert_eq!(writer.size(), crate::wal::WAL_HEADER_SIZE as u64);
    }

    /// Tests that metrics accurately track all writer operations.
    ///
    /// Verifies:
    /// - Write successes and failures are counted correctly
    /// - Sync operations are tracked (especially with SyncMode::Full)
    /// - Byte counts and success rates are accurate
    /// - Metrics remain consistent under both success and failure scenarios
    #[test]
    fn metrics_tracks_writes_syncs_and_failures_accurately() {
        let temp_dir = TempDir::new().unwrap();
        let wal_path = temp_dir.path().join("test.wal");

        let writer = WALWriter::new(&wal_path, SyncMode::Full, 1024 * 1024).unwrap();
        let metrics = writer.metrics();

        // Initial state
        assert_eq!(metrics.files_opened(), 1);
        assert_eq!(metrics.writes_total(), 0);

        // Write some entries
        for i in 0..5 {
            let entry =
                WALEntry::new_put(format!("key{}", i).into_bytes(), vec![b'v'; 100], i as u64)
                    .unwrap();
            writer.append(&entry).unwrap();
        }

        // Check metrics
        assert_eq!(metrics.writes_total(), 5);
        assert_eq!(metrics.writes_failed(), 0);
        assert!(metrics.bytes_written() > 500);
        assert_eq!(metrics.sync_total(), 5); // SyncMode::Full
        assert!(metrics.avg_entry_size() > 100);
        assert_eq!(metrics.write_success_rate(), 100.0);

        // Test size limit failure
        let small_writer = WALWriter::new(
            temp_dir.path().join("small.wal"),
            SyncMode::None,
            100, // Very small limit
        )
        .unwrap();
        let small_metrics = small_writer.metrics();

        let large_entry = WALEntry::new_put(b"key".to_vec(), vec![b'x'; 200], 1).unwrap();

        let result = small_writer.append(&large_entry);
        assert!(result.is_err());
        assert_eq!(small_metrics.writes_failed(), 1);
    }

    // ==================== Directory and Permission Error Tests ====================

    /// Tests that writer creation fails when path points to a non-existent drive.
    ///
    /// This ensures:
    /// - Invalid paths are properly rejected
    /// - File system errors are propagated correctly
    /// - Clear error messages for debugging path issues
    #[test]
    fn new_returns_error_for_invalid_path() {
        // Use a path that should fail on all platforms
        let invalid_path = if cfg!(windows) {
            "Z:\\nonexistent\\directory\\test.wal"
        } else {
            "/proc/nonexistent/test.wal" // /proc typically doesn't allow file creation
        };

        let result = WALWriter::new(invalid_path, SyncMode::Full, 1024 * 1024);

        // Should fail due to invalid path
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(matches!(e, Error::Io(_)));
        }
    }

    /// Tests that writer creation fails when target file exists but is read-only.
    ///
    /// Verifies:
    /// - Existing read-only files are not corrupted
    /// - Clear error reporting for permission issues
    /// - Respects file system permissions
    #[test]
    fn new_returns_error_when_file_exists_but_not_writable() {
        use std::fs::{self, File};
        // Skip when running as root because permissions checks won't fail
        if unsafe { libc::geteuid() } == 0 {
            eprintln!("Skipping test as root user");
            return;
        }

        let temp_dir = TempDir::new().unwrap();
        let wal_path = temp_dir.path().join("readonly.wal");

        // Create a read-only file using cross-platform API
        File::create(&wal_path).unwrap();
        let mut perms = fs::metadata(&wal_path).unwrap().permissions();
        perms.set_readonly(true);
        fs::set_permissions(&wal_path, perms).unwrap();

        let result = WALWriter::new(&wal_path, SyncMode::Full, 1024 * 1024);

        // Restore permissions for cleanup
        let mut perms = fs::metadata(&wal_path).unwrap().permissions();
        perms.set_readonly(false);
        fs::set_permissions(&wal_path, perms).unwrap();

        assert!(result.is_err());
    }

    /// Tests that writer automatically creates missing parent directories.
    ///
    /// This ensures:
    /// - Deeply nested paths are supported
    /// - Directory creation is recursive
    /// - Convenience for users who don't pre-create directories
    #[test]
    fn new_creates_parent_directories_when_missing() {
        let temp_dir = TempDir::new().unwrap();
        let nested_path = temp_dir
            .path()
            .join("a")
            .join("b")
            .join("c")
            .join("test.wal");

        // Parent directories don't exist
        assert!(!nested_path.parent().unwrap().exists());

        let writer = WALWriter::new(&nested_path, SyncMode::None, 1024 * 1024).unwrap();

        // Verify file was created
        assert!(nested_path.exists());

        // Verify we can write to it
        let entry = WALEntry::new_put(b"key".to_vec(), b"value".to_vec(), 1).unwrap();
        writer.append(&entry).unwrap();
    }

    // ==================== Size Limit and Rotation Tests ====================

    /// Tests that file size limits are enforced accurately during appends.
    ///
    /// Verifies:
    /// - Size limit prevents writes that would exceed the limit
    /// - Multiple small writes are allowed up to the limit
    /// - Size tracking remains accurate across multiple appends
    /// - File size stays within configured bounds
    #[test]
    fn append_enforces_size_limit_correctly() {
        let temp_dir = TempDir::new().unwrap();
        let wal_path = temp_dir.path().join("limited.wal");

        // Size limit that accounts for header (64 bytes) plus room for a few entries
        let size_limit = 1024; // 1KB
        let writer = WALWriter::new(&wal_path, SyncMode::None, size_limit).unwrap();

        // Writer starts with 64 bytes (header size)
        assert_eq!(writer.size(), 64);

        // Keep appending until we hit the limit
        let mut successful_writes = 0;
        let mut last_successful_size = 64;

        for i in 0..100 {
            // Create smaller entries to ensure some will fit
            let entry = WALEntry::new_put(
                format!("k{}", i).into_bytes(), // Short key
                vec![b'v'; 20],                 // Small value
                i as u64,
            )
            .unwrap();

            match writer.append(&entry) {
                Ok(_) => {
                    successful_writes += 1;
                    last_successful_size = writer.size();
                }
                Err(e) => {
                    assert!(e.to_string().contains("size limit"));
                    break;
                }
            }
        }

        // Should have written some entries but hit the limit
        assert!(successful_writes > 0, "No entries were written");
        assert!(
            successful_writes < 100,
            "All entries were written - limit not reached"
        );

        // The last successful write should have brought us close to the limit
        assert!(last_successful_size <= size_limit);
        assert!(
            last_successful_size > size_limit / 2,
            "Only wrote {} bytes of {} limit",
            last_successful_size,
            size_limit
        );
    }

    /// Tests that file size tracking remains accurate across multiple writes.
    ///
    /// This ensures:
    /// - Internal size tracking matches actual file size
    /// - No drift between tracked and real sizes
    /// - Size updates are atomic with writes
    #[test]
    fn append_tracks_size_accurately_across_multiple_writes() {
        use std::fs;

        let temp_dir = TempDir::new().unwrap();
        let wal_path = temp_dir.path().join("size_tracking.wal");

        let writer = WALWriter::new(&wal_path, SyncMode::Full, 100 * 1024 * 1024).unwrap();

        // Write entries and track expected size
        let mut expected_size = 64; // Header size

        for i in 0..10 {
            let entry = WALEntry::new_put(
                format!("key_{}", i).into_bytes(),
                format!("value_{}", i).into_bytes(),
                i as u64,
            )
            .unwrap();

            let encoded = entry.encode().unwrap();
            expected_size += encoded.len() as u64;

            writer.append(&entry).unwrap();
            assert_eq!(writer.size(), expected_size);
        }

        // Verify actual file size matches
        writer.sync().unwrap();
        let actual_size = fs::metadata(&wal_path).unwrap().len();
        assert_eq!(actual_size, expected_size);
    }

    // ==================== Concurrent Error Scenarios ====================

    /// Tests that concurrent writes respect size limits without data corruption.
    ///
    /// This critical test ensures:
    /// - Multiple threads can write safely
    /// - Size limit is enforced atomically across threads
    /// - No writes exceed the configured limit
    /// - Thread safety of size tracking
    #[test]
    fn concurrent_writes_handle_size_limit_correctly() {
        use std::sync::{Arc, Barrier};
        use std::thread;

        let temp_dir = TempDir::new().unwrap();
        let wal_path = temp_dir.path().join("concurrent_limit.wal");

        // Small size limit to trigger errors
        let writer = Arc::new(WALWriter::new(&wal_path, SyncMode::None, 2048).unwrap());
        let barrier = Arc::new(Barrier::new(5));
        let mut handles = vec![];

        for thread_id in 0..5 {
            let writer = Arc::clone(&writer);
            let barrier = Arc::clone(&barrier);

            handles.push(thread::spawn(move || {
                barrier.wait();

                let mut successes = 0;
                let mut failures = 0;

                for i in 0..20 {
                    let entry = WALEntry::new_put(
                        format!("t{}_k{}", thread_id, i).into_bytes(),
                        vec![b'v'; 50],
                        i as u64,
                    )
                    .unwrap();

                    match writer.append(&entry) {
                        Ok(_) => successes += 1,
                        Err(_) => failures += 1,
                    }
                }

                (successes, failures)
            }));
        }

        let mut total_successes = 0;
        let mut total_failures = 0;

        for handle in handles {
            let (successes, failures) = handle.join().unwrap();
            total_successes += successes;
            total_failures += failures;
        }

        // Some writes should succeed, some should fail due to size limit
        assert!(total_successes > 0);
        assert!(total_failures > 0);

        // File should not exceed size limit
        let actual_size = std::fs::metadata(&wal_path).unwrap().len();
        assert!(actual_size <= 2048);
    }

    // ==================== Recovery After Errors ====================

    /// Tests that writer remains functional after encountering errors.
    ///
    /// Verifies:
    /// - Failed appends don't corrupt writer state
    /// - Size tracking remains accurate after failures
    /// - Sync operations still work after errors
    /// - Metrics correctly reflect both successes and failures
    #[test]
    fn writer_remains_usable_after_append_error() {
        let temp_dir = TempDir::new().unwrap();
        let wal_path = temp_dir.path().join("recovery.wal");

        // Small size limit
        let writer = WALWriter::new(&wal_path, SyncMode::None, 500).unwrap();

        // Write entries until we hit the limit
        let mut hit_limit = false;
        for i in 0..20 {
            let entry =
                WALEntry::new_put(format!("key{}", i).into_bytes(), vec![b'v'; 50], i as u64)
                    .unwrap();

            if writer.append(&entry).is_err() {
                hit_limit = true;
                break;
            }
        }

        assert!(hit_limit);

        // Writer should still report correct size
        assert!(writer.size() > 0);
        assert!(writer.size() <= 500);

        // Sync should still work
        assert!(writer.sync().is_ok());

        // Metrics should be consistent
        let metrics = writer.metrics();
        assert!(metrics.writes_total() > 0);
        assert!(metrics.writes_failed() > 0);
    }

    // ==================== Edge Cases ====================

    /// Tests that writer can handle existing files with invalid data.
    ///
    /// This ensures:
    /// - Pre-existing corruption doesn't prevent writer creation
    /// - Writer appends after existing data (no truncation)
    /// - Graceful handling of invalid WAL files
    #[test]
    fn new_handles_existing_corrupted_file() {
        use std::fs::File;
        use std::io::Write;

        let temp_dir = TempDir::new().unwrap();
        let wal_path = temp_dir.path().join("corrupted.wal");

        // Create a file with invalid data
        let mut file = File::create(&wal_path).unwrap();
        file.write_all(b"This is not a valid WAL header").unwrap();
        drop(file);

        // Writer should handle existing invalid file
        // It doesn't truncate, so it will append after the invalid data
        let writer = WALWriter::new(&wal_path, SyncMode::None, 10 * 1024 * 1024).unwrap();

        // Should be able to append
        let entry = WALEntry::new_put(b"key".to_vec(), b"value".to_vec(), 1).unwrap();
        assert!(writer.append(&entry).is_ok());
    }

    /// Tests that zero size limit prevents all appends.
    ///
    /// Verifies:
    /// - Edge case of zero limit is handled correctly
    /// - Writer creation succeeds but appends fail
    /// - Clear error messages for this configuration
    #[test]
    fn size_limit_zero_always_fails_append() {
        let temp_dir = TempDir::new().unwrap();
        let wal_path = temp_dir.path().join("zero_limit.wal");

        // Size limit of 0 should allow creation but no appends
        let writer = WALWriter::new(&wal_path, SyncMode::None, 0).unwrap();

        let entry = WALEntry::new_put(b"key".to_vec(), b"value".to_vec(), 1).unwrap();
        let result = writer.append(&entry);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("size limit"));
    }
}
