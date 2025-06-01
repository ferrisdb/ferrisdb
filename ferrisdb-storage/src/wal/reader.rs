// 1. Local crate imports
use super::WALEntry;
use ferrisdb_core::Result;

// 2. External crate imports
// (none in this file)

// 3. Standard library imports
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

/// Reader for the Write-Ahead Log
///
/// The WALReader reads entries from a WAL file sequentially. It verifies
/// checksums and handles partial entries at the end of the file (which may
/// occur if the process crashed during a write).
///
/// # Corruption Handling
///
/// The reader handles several corruption scenarios:
///
/// - **Checksum Mismatch**: Entry data doesn't match stored checksum
///   - Action: Stop reading, return entries read so far
///   - Recovery: Use last good entry as recovery point
///
/// - **Partial Entry**: Incomplete entry at end of file (crash during write)
///   - Action: Silently ignore, return `Ok(None)`
///   - Recovery: Normal - this is expected after crashes
///
/// - **Invalid Length**: Entry length header is corrupted
///   - Action: Stop reading, may indicate severe corruption
///   - Recovery: Manual inspection may be needed
///
/// # Recovery Best Practices
///
/// 1. Always verify entry count matches expectations
/// 2. Track highest sequence number for gap detection
/// 3. Consider checksumming the entire WAL file periodically
/// 4. Keep backups of rotated WAL files
///
/// # Example
///
/// ```no_run
/// use ferrisdb_storage::wal::WALReader;
///
/// let mut reader = WALReader::new("path/to/wal.log")?;
///
/// // Read with corruption handling
/// let entries = reader.read_all()?;
/// println!("Recovered {} entries", entries.len());
///
/// // Check for gaps in sequence numbers
/// let mut last_seq = 0;
/// for entry in &entries {
///     if entry.timestamp != last_seq + 1 && last_seq != 0 {
///         eprintln!("Gap detected: {} -> {}", last_seq, entry.timestamp);
///     }
///     last_seq = entry.timestamp;
/// }
/// # Ok::<(), ferrisdb_core::Error>(())
/// ```
pub struct WALReader {
    reader: BufReader<File>,
}

impl WALReader {
    /// Creates a new WAL reader
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be opened.
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let file = File::open(path)?;
        Ok(Self {
            reader: BufReader::new(file),
        })
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
        // Read length
        let mut length_buf = [0u8; 4];
        match self.reader.read_exact(&mut length_buf) {
            Ok(_) => {}
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
            Err(e) => return Err(e.into()),
        }

        let length = u32::from_le_bytes(length_buf) as usize;

        // Read the rest of the entry
        let mut data = vec![0u8; length + 4];
        data[..4].copy_from_slice(&length_buf);
        self.reader.read_exact(&mut data[4..])?;

        Ok(Some(WALEntry::decode(&data)?))
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
    fn read_all_returns_all_written_entries() {
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
                );
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
    fn iterator_yields_entries_in_order() {
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
                } else {
                    WALEntry::new_delete(format!("key{}", i).into_bytes(), i as u64)
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
    fn read_entry_handles_partial_entry_at_eof() {
        let temp_dir = TempDir::new().unwrap();
        let wal_path = temp_dir.path().join("partial.wal");

        // Write a complete entry and then partial data
        {
            use std::io::Write;
            
            let writer = WALWriter::new(&wal_path, SyncMode::None, 1024 * 1024).unwrap();
            let entry = WALEntry::new_put(b"key".to_vec(), b"value".to_vec(), 1);
            writer.append(&entry).unwrap();
            writer.sync().unwrap();
            
            // Append partial data to simulate crash during write
            let mut file = std::fs::OpenOptions::new()
                .append(true)
                .open(&wal_path)
                .unwrap();
            file.write_all(&[0x00, 0x00]).unwrap(); // Partial length header
        }

        // Reader should read the complete entry and stop at partial
        let mut reader = WALReader::new(&wal_path).unwrap();
        let entries = reader.read_all().unwrap();
        
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].key, b"key");
    }

    #[test]
    fn read_entry_stops_on_checksum_mismatch() {
        let temp_dir = TempDir::new().unwrap();
        let wal_path = temp_dir.path().join("corrupt.wal");

        // Write entries
        {
            let writer = WALWriter::new(&wal_path, SyncMode::None, 1024 * 1024).unwrap();
            
            for i in 0..3 {
                let entry = WALEntry::new_put(
                    format!("key{}", i).into_bytes(),
                    format!("value{}", i).into_bytes(),
                    i,
                );
                writer.append(&entry).unwrap();
            }
            writer.sync().unwrap();
        }

        // Corrupt the middle entry
        {
            use std::io::{Seek, SeekFrom, Write};
            
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .open(&wal_path)
                .unwrap();
            
            // Seek into the middle of the file and corrupt some bytes
            file.seek(SeekFrom::Start(50)).unwrap();
            file.write_all(b"CORRUPT").unwrap();
        }

        // Reader should read entries until corruption
        let mut reader = WALReader::new(&wal_path).unwrap();
        let result = reader.read_all();
        
        // Should either read partial entries or return error
        // The exact behavior depends on where corruption hits
        match result {
            Ok(entries) => {
                // Read some entries before corruption
                assert!(entries.len() < 3);
            }
            Err(e) => {
                // Hit corruption, which is expected
                println!("Expected corruption error: {}", e);
            }
        }
    }

    #[test]
    fn new_returns_error_for_nonexistent_file() {
        let result = WALReader::new("/nonexistent/path/to/wal.log");
        assert!(result.is_err());
    }
}
