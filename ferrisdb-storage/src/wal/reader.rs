use super::{WALEntry, WALHeader};
use crate::format::FileHeader;
use ferrisdb_core::Result;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;

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
}

impl WALReader {
    /// Creates a new WAL reader
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be opened
    /// - The header is missing or invalid
    /// - The file is corrupted
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let mut file = File::open(path)?;

        // Read and validate header
        let mut header_data = vec![0u8; crate::wal::WAL_HEADER_SIZE];
        file.read_exact(&mut header_data)?;

        let header = WALHeader::decode(&header_data)?;
        // validate() is already called in decode()

        // Seek to where entries begin
        file.seek(SeekFrom::Start(header.entry_start_offset as u64))?;

        Ok(Self {
            reader: BufReader::new(file),
            header,
        })
    }

    /// Get the WAL file header
    pub fn header(&self) -> &WALHeader {
        &self.header
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
    fn test_wal_reader_writer_integration() {
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
    fn test_wal_reader_iterator() {
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
    fn test_reader_validates_header() {
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
    fn test_reader_rejects_file_without_header() {
        let temp_dir = TempDir::new().unwrap();
        let wal_path = temp_dir.path().join("bad.wal");

        // Create file that's too small
        std::fs::write(&wal_path, b"too small").unwrap();

        let result = WALReader::new(&wal_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_reader_rejects_wrong_magic() {
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
