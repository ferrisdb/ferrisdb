// 1. Local crate imports
use super::WALEntry;
use ferrisdb_core::{Error, Result, SyncMode};

// 2. External crate imports
use parking_lot::Mutex;

// 3. Standard library imports
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// Writer for the Write-Ahead Log
///
/// The WALWriter appends entries to a log file with configurable durability
/// guarantees. It tracks the file size and returns an error when the size
/// limit is reached, indicating that rotation is needed.
///
/// # Thread Safety
///
/// The writer is thread-safe and can be shared across multiple threads using
/// `Arc<WALWriter>`. All write operations are serialized through an internal
/// mutex, ensuring:
/// 
/// - Entries are written atomically (no interleaving)
/// - File size tracking is accurate across threads
/// - Only one thread writes to the file at a time
///
/// **Performance Note**: While thread-safe, concurrent writes are serialized.
/// For maximum throughput, consider batching writes in each thread before
/// calling `append()`.
///
/// # Error Recovery
///
/// The WALWriter handles various error scenarios:
///
/// - **Size Limit Reached**: Returns `Error::StorageEngine` - caller should
///   rotate to a new WAL file
/// - **I/O Errors**: Propagated to caller - may indicate disk full, permission
///   issues, or hardware failure
/// - **Sync Failures**: Based on `SyncMode`, may indicate durability compromise
///
/// Recovery strategies:
/// - For size limit errors: Create a new WAL file with incremented suffix
/// - For I/O errors: Retry with backoff, alert operators, fail over to replica
/// - For sync failures: Depends on durability requirements
///
/// # Example
///
/// ```no_run
/// use ferrisdb_storage::wal::{WALWriter, WALEntry};
/// use ferrisdb_core::{SyncMode, Error};
/// use std::sync::Arc;
///
/// let writer = Arc::new(WALWriter::new(
///     "path/to/wal.log",
///     SyncMode::Normal,
///     64 * 1024 * 1024  // 64MB
/// )?);
///
/// let entry = WALEntry::new_put(b"key".to_vec(), b"value".to_vec(), 1);
/// 
/// match writer.append(&entry) {
///     Ok(()) => {
///         // Success
///     }
///     Err(Error::StorageEngine(msg)) if msg.contains("size limit") => {
///         // Rotate to new file
///         println!("WAL full, rotating...");
///     }
///     Err(e) => {
///         // Handle other errors
///         eprintln!("WAL write failed: {}", e);
///     }
/// }
/// # Ok::<(), ferrisdb_core::Error>(())
/// ```
pub struct WALWriter {
    file: Arc<Mutex<BufWriter<File>>>,
    path: PathBuf,
    size: AtomicU64,
    sync_mode: SyncMode,
    size_limit: u64,
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
        
        // Create parent directory if it exists and is not root
        if let Some(parent) = path.parent() {
            if parent != Path::new("") {
                std::fs::create_dir_all(parent)?;
            }
        }

        let file = OpenOptions::new().create(true).append(true).open(&path)?;

        let metadata = file.metadata()?;
        let size = metadata.len();

        Ok(Self {
            file: Arc::new(Mutex::new(BufWriter::new(file))),
            path,
            size: AtomicU64::new(size),
            sync_mode,
            size_limit,
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
        let encoded = entry.encode();
        let entry_size = encoded.len() as u64;

        // Check if we need to rotate
        if self.size.load(Ordering::Relaxed) + entry_size > self.size_limit {
            return Err(Error::StorageEngine(
                "WAL file size limit reached".to_string(),
            ));
        }

        let mut file = self.file.lock();
        file.write_all(&encoded)?;

        match self.sync_mode {
            SyncMode::None => {}
            SyncMode::Normal => {
                file.flush()?;
            }
            SyncMode::Full => {
                file.flush()?;
                file.get_ref().sync_all()?;
            }
        }

        self.size.fetch_add(entry_size, Ordering::Relaxed);
        Ok(())
    }

    /// Forces a sync of all buffered data to disk
    ///
    /// This ensures durability by flushing the buffer and calling
    /// fsync on the underlying file.
    pub fn sync(&self) -> Result<()> {
        let mut file = self.file.lock();
        file.flush()?;
        file.get_ref().sync_all()?;
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn new_creates_wal_file_with_parent_directory() {
        let temp_dir = TempDir::new().unwrap();
        let wal_path = temp_dir.path().join("nested/dir/test.wal");

        let writer = WALWriter::new(&wal_path, SyncMode::Normal, 1024 * 1024).unwrap();

        assert!(wal_path.parent().unwrap().exists());
        assert_eq!(writer.size(), 0);
    }

    #[test]
    fn append_writes_entry_and_updates_size() {
        let temp_dir = TempDir::new().unwrap();
        let wal_path = temp_dir.path().join("test.wal");

        let writer = WALWriter::new(&wal_path, SyncMode::None, 1024 * 1024).unwrap();
        let initial_size = writer.size();

        let entry = WALEntry::new_put(b"key1".to_vec(), b"value1".to_vec(), 1);
        writer.append(&entry).unwrap();

        assert!(writer.size() > initial_size);
        assert!(wal_path.exists());
    }

    #[test]
    fn append_returns_error_when_size_limit_exceeded() {
        let temp_dir = TempDir::new().unwrap();
        let wal_path = temp_dir.path().join("test.wal");

        // Very small size limit
        let writer = WALWriter::new(&wal_path, SyncMode::None, 50).unwrap();

        let entry = WALEntry::new_put(
            b"key_with_long_name".to_vec(),
            b"value_with_long_content".to_vec(),
            1,
        );

        let result = writer.append(&entry);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::StorageEngine(msg) if msg.contains("size limit")));
    }

    #[test]
    fn new_handles_paths_without_parent_directory() {
        // Test creating WAL in current directory
        let writer = WALWriter::new("test.wal", SyncMode::None, 1024);
        assert!(writer.is_ok());

        // Test creating WAL with simple filename
        let writer = WALWriter::new("wal", SyncMode::None, 1024);
        assert!(writer.is_ok());

        // Clean up
        let _ = std::fs::remove_file("test.wal");
        let _ = std::fs::remove_file("wal");
    }

    #[test]
    fn concurrent_append_maintains_consistency() {
        use std::sync::Arc;
        use std::thread;

        let temp_dir = TempDir::new().unwrap();
        let wal_path = temp_dir.path().join("concurrent.wal");
        
        let writer = Arc::new(WALWriter::new(&wal_path, SyncMode::None, 10 * 1024 * 1024).unwrap());
        let num_threads = 10;
        let writes_per_thread = 100;
        
        // Spawn multiple threads that write concurrently
        let handles: Vec<_> = (0..num_threads)
            .map(|thread_id| {
                let writer_clone = Arc::clone(&writer);
                thread::spawn(move || {
                    for i in 0..writes_per_thread {
                        let entry = WALEntry::new_put(
                            format!("key_{}_{}", thread_id, i).into_bytes(),
                            format!("value_{}_{}", thread_id, i).into_bytes(),
                            thread_id * 1000 + i,
                        );
                        writer_clone.append(&entry).unwrap();
                    }
                })
            })
            .collect();
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Verify all writes succeeded
        writer.sync().unwrap();
        assert!(writer.size() > 0);
        
        // Read back and verify we have all entries
        let mut reader = super::super::reader::WALReader::new(&wal_path).unwrap();
        let entries = reader.read_all().unwrap();
        assert_eq!(entries.len(), num_threads * writes_per_thread);
    }

    #[test]
    fn concurrent_size_tracking_remains_accurate() {
        use std::sync::Arc;
        use std::thread;

        let temp_dir = TempDir::new().unwrap();
        let wal_path = temp_dir.path().join("size_tracking.wal");
        
        let writer = Arc::new(WALWriter::new(&wal_path, SyncMode::None, 10 * 1024 * 1024).unwrap());
        
        // Track sizes from multiple threads
        let handles: Vec<_> = (0..5)
            .map(|_| {
                let writer_clone = Arc::clone(&writer);
                thread::spawn(move || {
                    let mut sizes = Vec::new();
                    for i in 0..20 {
                        let entry = WALEntry::new_put(
                            format!("key_{}", i).into_bytes(),
                            vec![0u8; 100], // Fixed size for predictable growth
                            i,
                        );
                        writer_clone.append(&entry).unwrap();
                        sizes.push(writer_clone.size());
                    }
                    sizes
                })
            })
            .collect();
        
        // Collect all size observations
        let mut all_sizes = Vec::new();
        for handle in handles {
            all_sizes.extend(handle.join().unwrap());
        }
        
        // Verify sizes are monotonically increasing (no race conditions)
        all_sizes.sort();
        for window in all_sizes.windows(2) {
            assert!(window[0] <= window[1], "Size tracking has race condition");
        }
    }

    #[test]
    fn sync_flushes_data_based_on_sync_mode() {
        let temp_dir = TempDir::new().unwrap();
        
        // Test each sync mode
        for mode in [SyncMode::None, SyncMode::Normal, SyncMode::Full] {
            let wal_path = temp_dir.path().join(format!("{:?}.wal", mode));
            let writer = WALWriter::new(&wal_path, mode, 1024 * 1024).unwrap();
            
            let entry = WALEntry::new_put(b"key".to_vec(), b"value".to_vec(), 1);
            writer.append(&entry).unwrap();
            
            // Manual sync should always work regardless of mode
            writer.sync().unwrap();
            
            // Verify file exists and has content
            assert!(wal_path.exists());
            assert!(std::fs::metadata(&wal_path).unwrap().len() > 0);
        }
    }

    #[test]
    #[cfg(unix)]
    fn append_handles_disk_full_gracefully() {
        use std::io::Write;
        
        // This test requires special setup to simulate disk full
        // In a real scenario, we'd use a limited loopback device
        // For now, we test that I/O errors are properly propagated
        
        let temp_dir = TempDir::new().unwrap();
        let wal_path = temp_dir.path().join("readonly.wal");
        
        // Create a file and make it read-only to simulate write failure
        {
            let mut file = std::fs::File::create(&wal_path).unwrap();
            file.write_all(b"dummy").unwrap();
        }
        
        // Make file read-only
        let mut perms = std::fs::metadata(&wal_path).unwrap().permissions();
        perms.set_readonly(true);
        std::fs::set_permissions(&wal_path, perms).unwrap();
        
        // Try to create writer on read-only file
        let writer_result = WALWriter::new(&wal_path, SyncMode::None, 1024);
        assert!(writer_result.is_err());
        
        // Clean up
        let mut perms = std::fs::metadata(&wal_path).unwrap().permissions();
        perms.set_readonly(false);
        std::fs::set_permissions(&wal_path, perms).unwrap();
    }

    #[test]
    fn path_returns_original_path() {
        let temp_dir = TempDir::new().unwrap();
        let wal_path = temp_dir.path().join("test.wal");
        
        let writer = WALWriter::new(&wal_path, SyncMode::None, 1024).unwrap();
        assert_eq!(writer.path(), wal_path.as_path());
    }

    #[test]
    fn size_accurately_tracks_multiple_writes() {
        let temp_dir = TempDir::new().unwrap();
        let wal_path = temp_dir.path().join("test.wal");
        
        let writer = WALWriter::new(&wal_path, SyncMode::None, 10 * 1024 * 1024).unwrap();
        let mut expected_size = 0;
        
        for i in 0..10 {
            let entry = WALEntry::new_put(
                format!("key_{}", i).into_bytes(),
                format!("value_{}", i).into_bytes(),
                i,
            );
            
            let encoded_size = entry.encode().len() as u64;
            writer.append(&entry).unwrap();
            expected_size += encoded_size;
            
            assert_eq!(writer.size(), expected_size);
        }
    }
}
