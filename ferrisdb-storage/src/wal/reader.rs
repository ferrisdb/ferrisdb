use super::{WALEntry, WALHeader, WALMetrics};
use crate::format::FileHeader;
use crate::utils::BytesMutExt;
use bytes::BytesMut;
use ferrisdb_core::Result;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;
use std::sync::Arc;

/// Statistics for the WAL reader buffer management
#[derive(Debug, Clone)]
pub struct ReaderStats {
    /// Peak buffer size reached during reading
    pub peak_buffer_size: usize,
    /// Number of buffer resizes that occurred
    pub buffer_resizes: usize,
    /// Initial buffer capacity
    pub initial_capacity: usize,
}

/// Reader for the Write-Ahead Log
///
/// The WALReader reads entries from a WAL file sequentially. It first validates
/// the file header, then reads entries following the header. It verifies
/// checksums and handles partial entries at the end of the file (which may
/// occur if the process crashed during a write).
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
    metrics: Arc<WALMetrics>,
    stats: ReaderStats,
}

impl WALReader {
    /// Default initial buffer capacity for reading entries
    const DEFAULT_BUFFER_CAPACITY: usize = 8 * 1024; // 8KB

    /// Creates a new WAL reader with default buffer capacity
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be opened
    /// - The header is missing or invalid
    /// - The file is corrupted
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        Self::with_initial_capacity(path, Self::DEFAULT_BUFFER_CAPACITY)
    }

    /// Creates a new WAL reader with specified initial buffer capacity
    ///
    /// This allows tuning memory usage for different workloads:
    /// - Small capacity for memory-constrained environments
    /// - Large capacity for high-throughput reading
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be opened
    /// - The header is missing or invalid
    /// - The file is corrupted
    pub fn with_initial_capacity(path: impl AsRef<Path>, initial_capacity: usize) -> Result<Self> {
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
            buffer: BytesMut::with_capacity(initial_capacity),
            metrics,
            stats: ReaderStats {
                peak_buffer_size: 0,
                buffer_resizes: 0,
                initial_capacity,
            },
        })
    }

    /// Get the WAL file header
    pub fn header(&self) -> &WALHeader {
        &self.header
    }

    /// Get reader statistics for buffer management
    pub fn stats(&self) -> ReaderStats {
        self.stats.clone()
    }

    /// Get metrics for WAL operations
    pub fn metrics(&self) -> Arc<WALMetrics> {
        Arc::clone(&self.metrics)
    }

    /// Reads the next entry from the WAL using efficient buffer management
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
        // Read length
        let mut length_buf = [0u8; 4];
        match self.reader.read_exact(&mut length_buf) {
            Ok(_) => {}
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
            Err(e) => {
                self.metrics.record_read(0, false);
                return Err(e.into());
            }
        }

        let length = u32::from_le_bytes(length_buf) as usize;
        let total_size = length + 4; // Include the length field

        // Track buffer capacity before potential resize
        let capacity_before = self.buffer.capacity();

        // Clear buffer and read entire entry using BytesMutExt
        self.buffer.clear();
        self.buffer.extend_from_slice(&length_buf);

        // Read remaining data efficiently
        match self.buffer.read_exact_from(&mut self.reader, length) {
            Ok(_) => {
                // Update statistics
                let capacity_after = self.buffer.capacity();
                if capacity_after > capacity_before {
                    self.stats.buffer_resizes += 1;
                }
                if capacity_after > self.stats.peak_buffer_size {
                    self.stats.peak_buffer_size = capacity_after;
                }

                // Record successful read
                self.metrics.record_read(total_size as u64, true);

                // Decode the entry
                let entry = WALEntry::decode(&self.buffer)?;
                Ok(Some(entry))
            }
            Err(e) => {
                self.metrics.record_read(total_size as u64, false);
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

    #[test]
    fn reader_and_writer_maintain_data_integrity() {
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

    #[test]
    fn new_validates_wal_header_format() {
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

    #[test]
    fn new_rejects_files_with_insufficient_header_data() {
        let temp_dir = TempDir::new().unwrap();
        let wal_path = temp_dir.path().join("bad.wal");

        // Create file that's too small
        std::fs::write(&wal_path, b"too small").unwrap();

        let result = WALReader::new(&wal_path);
        assert!(result.is_err());
    }

    #[test]
    fn new_rejects_files_with_invalid_magic_number() {
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
}
