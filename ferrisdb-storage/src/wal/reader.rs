use super::{WALEntry, WALHeader, WALMetrics};
use crate::format::FileHeader;
use crate::utils::BytesMutExt;
use crate::wal::log_entry::{DEFAULT_READER_BUFFER_SIZE, MAX_ENTRY_SIZE};
use ferrisdb_core::Result;

use bytes::BytesMut;

use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;
use std::sync::Arc;

/// Statistics for monitoring reader performance
#[derive(Debug, Default, Clone)]
pub struct ReaderStats {
    pub entries_read: usize,
    pub bytes_read: usize,
    pub peak_buffer_size: usize,
    pub buffer_resizes: usize,
}

/// Reader for the Write-Ahead Log
///
/// The WALReader reads entries from a WAL file sequentially. It uses BytesMut
/// for efficient buffer management with minimal allocations.
///
/// # Example
///
/// ```no_run
/// use ferrisdb_storage::wal::WALReader;
///
/// let mut reader = WALReader::new("path/to/wal.log")?;
///
/// // Read all entries
/// let entries = reader.read_all()?;
///
/// // Check performance stats
/// let stats = reader.stats();
/// println!("Peak buffer size: {} bytes", stats.peak_buffer_size);
///
/// // Or iterate through entries
/// for entry in reader {
///     match entry {
///         Ok(entry) => println!("Entry: {:?}", entry),
///         Err(e) => eprintln!("Error: {}", e),
///     }
/// }
/// # Ok::<(), ferrisdb_core::Error>(())
/// ```
pub struct WALReader {
    reader: BufReader<File>,
    header: WALHeader,
    buffer: BytesMut,
    stats: ReaderStats,
    metrics: Arc<WALMetrics>,
}

impl WALReader {
    /// Creates a new WAL reader with default buffer size
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be opened
    /// - The header is missing or invalid
    /// - The file is corrupted
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        Self::with_initial_capacity(path, DEFAULT_READER_BUFFER_SIZE)
    }

    /// Creates a new WAL reader with specified initial buffer capacity
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be opened
    /// - The header is missing or invalid
    /// - The file is corrupted
    pub fn with_initial_capacity(path: impl AsRef<Path>, capacity: usize) -> Result<Self> {
        let mut file = File::open(path)?;

        // Read and validate header
        let mut header_data = vec![0u8; crate::wal::WAL_HEADER_SIZE];
        file.read_exact(&mut header_data)?;

        let header = WALHeader::decode(&header_data)?;
        // validate() is already called in decode()

        // Seek to where entries begin
        file.seek(SeekFrom::Start(header.entry_start_offset as u64))?;

        let metrics = Arc::new(WALMetrics::new());
        metrics.record_file_opened();

        Ok(Self {
            reader: BufReader::new(file),
            header,
            buffer: BytesMut::with_capacity(capacity),
            stats: ReaderStats::default(),
            metrics,
        })
    }

    /// Get the WAL file header
    pub fn header(&self) -> &WALHeader {
        &self.header
    }

    /// Get reader statistics
    pub fn stats(&self) -> &ReaderStats {
        &self.stats
    }

    /// Get metrics for this reader
    pub fn metrics(&self) -> &WALMetrics {
        &self.metrics
    }

    /// Reads the next entry from the WAL
    ///
    /// Returns `Ok(None)` when the end of file is reached.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - An I/O error occurs
    /// - Corruption is detected (checksum mismatch)
    /// - The entry format is invalid
    pub fn read_entry(&mut self) -> Result<Option<WALEntry>> {
        // Clear buffer but retain capacity
        self.buffer.clear();

        // Read length field (4 bytes)
        if !self.read_exact_bytes(4)? {
            return Ok(None); // EOF
        }

        let length = u32::from_le_bytes(self.buffer[..4].try_into().map_err(|_| {
            ferrisdb_core::Error::Corruption("Failed to parse entry length".to_string())
        })?) as usize;

        // Validate length
        if length > MAX_ENTRY_SIZE - 4 {
            self.metrics.record_corruption();
            self.metrics.record_read(4, false);
            return Err(ferrisdb_core::Error::Corruption(format!(
                "Entry size {} exceeds maximum {}",
                length,
                MAX_ENTRY_SIZE - 4
            )));
        }

        // Read the rest of the entry
        if !self.read_exact_bytes(length)? {
            self.metrics.record_read(4, false);
            return Err(ferrisdb_core::Error::Corruption(
                "Unexpected EOF while reading entry".to_string(),
            ));
        }

        // Update statistics
        let total_size = (length + 4) as u64;
        self.stats.entries_read += 1;
        self.stats.bytes_read += total_size as usize;

        // Decode the complete entry
        match WALEntry::decode(&self.buffer) {
            Ok(entry) => {
                self.metrics.record_read(total_size, true);
                Ok(Some(entry))
            }
            Err(e) => {
                self.metrics.record_corruption();
                self.metrics.record_read(total_size, false);
                Err(e)
            }
        }
    }

    /// Helper to read exact number of bytes into buffer
    fn read_exact_bytes(&mut self, count: usize) -> Result<bool> {
        // Track buffer resizes before potentially growing
        let old_capacity = self.buffer.capacity();
        
        // Use our extension trait for efficient reading without zero-fill
        match self.buffer.read_exact_from(&mut self.reader, count) {
            Ok(()) => {
                // Track buffer growth
                let new_capacity = self.buffer.capacity();
                if new_capacity > self.stats.peak_buffer_size {
                    self.stats.peak_buffer_size = new_capacity;
                    if new_capacity > old_capacity {
                        self.stats.buffer_resizes += 1;
                    }
                }
                Ok(true)
            }
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                // EOF is not an error, just signals end of file
                Ok(false)
            }
            Err(e) => {
                // Other errors are propagated
                Err(e.into())
            }
        }
    }

    /// Reads all remaining entries from the WAL
    ///
    /// This is useful for recovery, where all entries need to be
    /// replayed to reconstruct the state.
    pub fn read_all(&mut self) -> Result<Vec<WALEntry>> {
        let mut entries = Vec::new();

        // Try to pre-allocate based on file size
        if let Ok(metadata) = self.reader.get_ref().metadata() {
            let remaining = metadata
                .len()
                .saturating_sub(self.header.entry_start_offset as u64);
            let estimated_entries = (remaining / 100) as usize; // Assume ~100 bytes average
            entries.reserve(estimated_entries);
        }

        while let Some(entry) = self.read_entry()? {
            entries.push(entry);
        }

        Ok(entries)
    }
}

impl Iterator for WALReader {
    type Item = Result<WALEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.read_entry() {
            Ok(Some(entry)) => Some(Ok(entry)),
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wal::WALWriter;
    use ferrisdb_core::SyncMode;
    use tempfile::TempDir;

    /// Tests that reader correctly reads all entries written by writer.
    ///
    /// Verifies:
    /// - Complete write/read cycle maintains data integrity
    /// - All entries are recovered in order
    /// - Entry data (key, value, timestamp) is preserved exactly
    #[test]
    fn read_all_returns_entries_written_by_writer() {
        let temp_dir = TempDir::new().unwrap();
        let wal_path = temp_dir.path().join("test.wal");

        // Write entries
        {
            let writer = WALWriter::new(&wal_path, SyncMode::Full, 1024 * 1024).unwrap();

            for i in 0..10 {
                let entry = WALEntry::new_put(
                    format!("key{}", i).into_bytes(),
                    format!("value{}", i).into_bytes(),
                    i as u64,
                )
                .unwrap();
                writer.append(&entry).unwrap();
            }

            writer.sync().unwrap();
        }

        // Read entries
        let mut reader = WALReader::new(&wal_path).unwrap();
        let entries = reader.read_all().unwrap();

        assert_eq!(entries.len(), 10);
        for (i, entry) in entries.iter().enumerate() {
            assert_eq!(entry.key, format!("key{}", i).into_bytes());
            assert_eq!(entry.value, format!("value{}", i).into_bytes());
            assert_eq!(entry.timestamp, i as u64);
        }
    }

    /// Tests that the iterator interface returns entries in correct order.
    ///
    /// This ensures:
    /// - Iterator API works as expected
    /// - Mixed Put/Delete operations are preserved
    /// - Order matches write sequence
    #[test]
    fn iterator_yields_entries_in_write_order() {
        let temp_dir = TempDir::new().unwrap();
        let wal_path = temp_dir.path().join("test.wal");

        // Write some entries
        {
            let writer = WALWriter::new(&wal_path, SyncMode::Full, 1024 * 1024).unwrap();

            for i in 0..5 {
                let entry = if i % 2 == 0 {
                    WALEntry::new_put(
                        format!("key{}", i).into_bytes(),
                        format!("value{}", i).into_bytes(),
                        i as u64,
                    )
                    .unwrap()
                } else {
                    WALEntry::new_delete(format!("key{}", i).into_bytes(), i as u64).unwrap()
                };
                writer.append(&entry).unwrap();
            }
        }

        // Read using iterator
        let reader = WALReader::new(&wal_path).unwrap();
        let entries: Result<Vec<_>> = reader.collect();
        let entries = entries.unwrap();

        assert_eq!(entries.len(), 5);
        assert_eq!(entries[1].operation, ferrisdb_core::Operation::Delete);
        assert_eq!(entries[1].value, Vec::<u8>::new());
    }

    /// Tests that reader validates WAL header on opening.
    ///
    /// Verifies:
    /// - Header checksum is validated
    /// - Version compatibility is checked
    /// - Magic bytes are verified
    /// - File sequence numbers are read correctly
    #[test]
    fn new_validates_header_checksum_and_version() {
        let temp_dir = TempDir::new().unwrap();
        let wal_path = temp_dir.path().join("test.wal");

        // Create WAL with header
        {
            let writer = WALWriter::new(&wal_path, SyncMode::Full, 1024 * 1024).unwrap();
            let entry = WALEntry::new_put(b"test".to_vec(), b"data".to_vec(), 1).unwrap();
            writer.append(&entry).unwrap();
        }

        // Reader should read header
        let reader = WALReader::new(&wal_path).unwrap();
        let header = reader.header();

        assert_eq!(header.version, crate::wal::WAL_CURRENT_VERSION);
        assert_eq!(&header.magic, crate::wal::WAL_MAGIC);
        assert!(header.created_at > 0);
        assert!(header.file_sequence > 0);
    }

    /// Tests that reader rejects files too small to contain a header.
    ///
    /// This ensures:
    /// - Truncated files are detected early
    /// - Clear error for incomplete WAL files
    /// - No buffer overruns on small files
    #[test]
    fn new_returns_error_for_file_smaller_than_header() {
        let temp_dir = TempDir::new().unwrap();
        let wal_path = temp_dir.path().join("bad.wal");

        // Create file that's too small
        std::fs::write(&wal_path, b"too small").unwrap();

        let result = WALReader::new(&wal_path);
        assert!(result.is_err());
    }

    /// Tests that reader rejects files with incorrect magic bytes.
    ///
    /// Verifies:
    /// - Non-WAL files are rejected
    /// - Clear error message for wrong file type
    /// - Prevents reading arbitrary files as WAL
    #[test]
    fn new_returns_error_for_invalid_magic_bytes() {
        use crate::format::FileHeader;

        let temp_dir = TempDir::new().unwrap();
        let wal_path = temp_dir.path().join("wrong.wal");

        // Create header with wrong magic
        let mut header = WALHeader::new(12345);
        header.magic = *b"WRONGMAG";
        let data = header.encode();
        std::fs::write(&wal_path, data).unwrap();

        let result = WALReader::new(&wal_path);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("Invalid WAL magic"));
    }

    /// Tests that reader statistics accurately track buffer growth.
    ///
    /// This ensures:
    /// - Buffer resize events are counted
    /// - Peak buffer size is tracked correctly
    /// - Stats help identify buffer tuning opportunities
    /// - Performance monitoring is accurate
    #[test]
    fn stats_tracks_buffer_resizes_and_peak_usage() {
        let temp_dir = TempDir::new().unwrap();
        let wal_path = temp_dir.path().join("test.wal");

        // Write some entries of varying sizes
        {
            let writer = WALWriter::new(&wal_path, SyncMode::Full, 1024 * 1024).unwrap();

            // Small entry
            let entry = WALEntry::new_put(b"k".to_vec(), b"v".to_vec(), 1).unwrap();
            writer.append(&entry).unwrap();

            // Medium entry
            let entry = WALEntry::new_put(vec![b'k'; 100], vec![b'v'; 500], 2).unwrap();
            writer.append(&entry).unwrap();

            // Large entry
            let entry = WALEntry::new_put(vec![b'k'; 1000], vec![b'v'; 5000], 3).unwrap();
            writer.append(&entry).unwrap();

            writer.sync().unwrap();
        }

        // Read with small initial buffer to trigger resizes
        let mut reader = WALReader::with_initial_capacity(&wal_path, 64).unwrap();
        let entries = reader.read_all().unwrap();

        assert_eq!(entries.len(), 3);

        let stats = reader.stats();
        assert_eq!(stats.entries_read, 3);
        assert!(stats.bytes_read > 0);
        assert!(stats.peak_buffer_size >= 6000); // Should grow to accommodate largest entry
        assert!(stats.buffer_resizes > 0); // Should have resized at least once
    }

    /// Tests that custom initial buffer capacity is respected.
    ///
    /// Verifies:
    /// - Initial capacity parameter works correctly
    /// - Buffer starts at specified size
    /// - Allows performance tuning for known workloads
    #[test]
    fn with_initial_capacity_allocates_specified_buffer_size() {
        let temp_dir = TempDir::new().unwrap();
        let wal_path = temp_dir.path().join("test.wal");

        // Create a WAL file
        {
            let writer = WALWriter::new(&wal_path, SyncMode::Full, 1024 * 1024).unwrap();
            let entry = WALEntry::new_put(b"key".to_vec(), b"value".to_vec(), 1).unwrap();
            writer.append(&entry).unwrap();
        }

        // Test custom capacity
        let reader = WALReader::with_initial_capacity(&wal_path, 16 * 1024).unwrap();
        let stats = reader.stats();
        assert_eq!(stats.peak_buffer_size, 0); // Not read yet

        // Read an entry
        let mut reader = WALReader::with_initial_capacity(&wal_path, 16 * 1024).unwrap();
        let entry = reader.read_entry().unwrap();
        assert!(entry.is_some());

        let stats = reader.stats();
        assert!(stats.peak_buffer_size >= 16 * 1024); // Should have initial capacity
    }

    /// Tests that metrics accurately track read operations and outcomes.
    ///
    /// This comprehensive test ensures:
    /// - Successful reads are counted
    /// - Failed reads are tracked separately
    /// - Byte counts are accurate
    /// - Corruption events are recorded
    /// - Success rate calculation is correct
    #[test]
    fn metrics_tracks_reads_and_corruptions_accurately() {
        let temp_dir = TempDir::new().unwrap();
        let wal_path = temp_dir.path().join("test.wal");

        // Write some entries
        {
            let writer = WALWriter::new(&wal_path, SyncMode::Full, 1024 * 1024).unwrap();

            for i in 0..5 {
                let entry =
                    WALEntry::new_put(format!("key{}", i).into_bytes(), vec![b'v'; 100], i as u64)
                        .unwrap();
                writer.append(&entry).unwrap();
            }

            // Add a delete entry
            let delete = WALEntry::new_delete(b"key_del".to_vec(), 10).unwrap();
            writer.append(&delete).unwrap();
        }

        // Read entries and check metrics
        let mut reader = WALReader::new(&wal_path).unwrap();

        // Check initial metrics
        {
            let metrics = reader.metrics();
            assert_eq!(metrics.files_opened(), 1);
            assert_eq!(metrics.reads_total(), 0);
        }

        // Read all entries
        let entries = reader.read_all().unwrap();
        assert_eq!(entries.len(), 6);

        // Check final metrics
        let metrics = reader.metrics();
        assert_eq!(metrics.reads_total(), 6);
        assert_eq!(metrics.reads_failed(), 0);
        assert!(metrics.bytes_read() > 600);
        assert_eq!(metrics.corrupted_entries(), 0);
        assert_eq!(metrics.read_success_rate(), 100.0);
    }

    /// Tests that corruption metrics are updated when invalid data is encountered.
    ///
    /// Verifies:
    /// - Corruption counter increments on bad data
    /// - Failed read counter also increments
    /// - Metrics remain consistent after corruption
    /// - Helps monitor WAL health in production
    #[test]
    fn metrics_increments_corruption_count_on_invalid_data() {
        let temp_dir = TempDir::new().unwrap();
        let wal_path = temp_dir.path().join("test.wal");

        // Write a valid entry first
        {
            let writer = WALWriter::new(&wal_path, SyncMode::Full, 1024 * 1024).unwrap();
            let entry = WALEntry::new_put(b"key".to_vec(), b"value".to_vec(), 1).unwrap();
            writer.append(&entry).unwrap();
        }

        // Corrupt the file by appending invalid data
        {
            use std::fs::OpenOptions;
            use std::io::Write;

            let mut file = OpenOptions::new().append(true).open(&wal_path).unwrap();

            // Write invalid length field
            file.write_all(&[0xFF, 0xFF, 0xFF, 0xFF]).unwrap();
        }

        let mut reader = WALReader::new(&wal_path).unwrap();

        // Read first entry (should succeed)
        let entry1 = reader.read_entry().unwrap();
        assert!(entry1.is_some());

        // Check metrics after successful read
        {
            let metrics = reader.metrics();
            assert_eq!(metrics.reads_total(), 1);
        }

        // Try to read corrupted entry (should fail)
        let entry2 = reader.read_entry();
        assert!(entry2.is_err());

        // Check final metrics
        let metrics = reader.metrics();
        assert_eq!(metrics.reads_failed(), 1);
        assert_eq!(metrics.corrupted_entries(), 1);
    }
}
