//! Tutorial 2: Building a Write-Ahead Log
//! 
//! This tutorial teaches you how to build a durable Write-Ahead Log (WAL)
//! that ensures data survives crashes. We'll progressively build towards
//! the same design used in real FerrisDB.

use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

use anyhow::Result;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use crc32fast::Hasher;
use thiserror::Error;

// Type aliases to match FerrisDB's design
// In FerrisDB: use ferrisdb_core::{Key, Value, Timestamp};
pub type Key = Vec<u8>;
pub type Value = Vec<u8>;
pub type Timestamp = u64;

/// Custom error types for our WAL
#[derive(Error, Debug)]
pub enum WalError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    
    #[error("Checksum mismatch: expected {expected}, got {actual}")]
    ChecksumMismatch { expected: u32, actual: u32 },
    
    #[error("Invalid magic number")]
    InvalidMagic,
    
    #[error("Corrupted entry at offset {offset}")]
    CorruptedEntry { offset: u64 },
}

/// Operations that can be logged
#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Set { key: String, value: String },
    Delete { key: String },
}

/// Operation types - matching FerrisDB's internal design
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationType {
    Put = 1,
    Delete = 2,
}

/// A single entry in the WAL
#[derive(Debug, Clone, PartialEq)]
pub struct LogEntry {
    pub sequence: u64,
    pub operation: Operation,
}

/// WAL Entry - the internal representation closer to FerrisDB
/// 
/// This is what FerrisDB actually uses internally
#[derive(Debug, Clone, PartialEq)]
pub struct WALEntry {
    pub timestamp: Timestamp,
    pub operation: OperationType,
    pub key: Key,
    pub value: Value,
}

impl WALEntry {
    /// Creates a new Put entry
    pub fn new_put(key: Key, value: Value, timestamp: Timestamp) -> Self {
        Self {
            timestamp,
            operation: OperationType::Put,
            key,
            value,
        }
    }
    
    /// Creates a new Delete entry
    pub fn new_delete(key: Key, timestamp: Timestamp) -> Self {
        Self {
            timestamp,
            operation: OperationType::Delete,
            key,
            value: Vec::new(),
        }
    }
}

/// Configuration for sync behavior
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SyncMode {
    /// Fastest but least safe - OS decides when to write to disk
    None,
    /// Sync data but not metadata - good balance
    DataOnly,
    /// Sync everything - slowest but safest
    Full,
}

/// Builder for WAL configuration
pub struct WalBuilder {
    path: PathBuf,
    sync_mode: SyncMode,
    max_file_size: u64,
}

impl WalBuilder {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            sync_mode: SyncMode::DataOnly,
            max_file_size: 100 * 1024 * 1024, // 100MB default
        }
    }

    pub fn sync_mode(mut self, mode: SyncMode) -> Self {
        self.sync_mode = mode;
        self
    }

    pub fn max_file_size(mut self, size: u64) -> Self {
        self.max_file_size = size;
        self
    }

    pub fn build(self) -> Result<WriteAheadLog> {
        WriteAheadLog::open(self.path, self.sync_mode, self.max_file_size)
    }
}

/// The Write-Ahead Log implementation
pub struct WriteAheadLog {
    file: BufWriter<File>,
    path: PathBuf,
    sync_mode: SyncMode,
    max_file_size: u64,
    current_size: u64,
    next_sequence: u64,
}

// Magic number to identify WAL files
const WAL_MAGIC: u32 = 0x57414C21; // "WAL!"

impl WriteAheadLog {
    /// Open or create a WAL file
    fn open<P: AsRef<Path>>(path: P, sync_mode: SyncMode, max_file_size: u64) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .open(&path)?;

        let current_size = file.metadata()?.len();
        
        // If new file, write magic number
        let mut wal = WriteAheadLog {
            file: BufWriter::new(file),
            path,
            sync_mode,
            max_file_size,
            current_size,
            next_sequence: 0,
        };

        if current_size == 0 {
            wal.write_header()?;
        } else {
            // Recovery: find the last sequence number
            let entries = wal.recover_entries()?;
            if let Some(last) = entries.last() {
                wal.next_sequence = last.sequence + 1;
            }
        }

        Ok(wal)
    }

    /// Write the file header
    fn write_header(&mut self) -> Result<()> {
        self.file.write_u32::<LittleEndian>(WAL_MAGIC)?;
        self.file.write_u32::<LittleEndian>(1)?; // Version
        self.current_size += 8;
        self.sync()?;
        Ok(())
    }

    /// Append a new entry to the log (high-level API)
    pub fn append(&mut self, operation: Operation) -> Result<u64> {
        let entry = LogEntry {
            sequence: self.next_sequence,
            operation,
        };

        let encoded = self.encode_entry(&entry)?;
        
        // Check if we need to rotate
        if self.current_size + encoded.len() as u64 > self.max_file_size {
            self.rotate()?;
        }

        // Write the entry
        self.file.write_all(&encoded)?;
        self.current_size += encoded.len() as u64;
        self.next_sequence += 1;

        // Sync based on mode
        self.sync()?;

        Ok(entry.sequence)
    }

    /// Append a WAL entry (low-level API matching FerrisDB)
    pub fn append_entry(&mut self, entry: &WALEntry) -> Result<()> {
        let encoded = self.encode_wal_entry(entry)?;
        
        // Check if we need to rotate
        if self.current_size + encoded.len() as u64 > self.max_file_size {
            self.rotate()?;
        }

        // Write the entry
        self.file.write_all(&encoded)?;
        self.current_size += encoded.len() as u64;

        // Update sequence if needed
        if entry.timestamp >= self.next_sequence {
            self.next_sequence = entry.timestamp + 1;
        }

        // Sync based on mode
        self.sync()?;

        Ok(())
    }

    /// Encode an entry to bytes with checksum (high-level)
    fn encode_entry(&self, entry: &LogEntry) -> Result<Vec<u8>> {
        // Convert to WALEntry format
        let wal_entry = match &entry.operation {
            Operation::Set { key, value } => {
                WALEntry::new_put(key.as_bytes().to_vec(), value.as_bytes().to_vec(), entry.sequence)
            }
            Operation::Delete { key } => {
                WALEntry::new_delete(key.as_bytes().to_vec(), entry.sequence)
            }
        };
        
        self.encode_wal_entry(&wal_entry)
    }

    /// Encode a WAL entry to bytes with checksum
    /// 
    /// Binary format (matching FerrisDB):
    /// ```text
    /// +------------+------------+------------+-------+------------+
    /// | Length(4B) | CRC32(4B)  | Time(8B)   | Op(1B)| Key Len(4B)|
    /// +------------+------------+------------+-------+------------+
    /// | Key(var)   | Val Len(4B)| Value(var) |
    /// +------------+------------+------------+
    /// ```
    fn encode_wal_entry(&self, entry: &WALEntry) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        
        // Write entry data
        buf.write_u64::<LittleEndian>(entry.timestamp)?;
        buf.write_u8(entry.operation as u8)?;
        
        // Write key
        buf.write_u32::<LittleEndian>(entry.key.len() as u32)?;
        buf.write_all(&entry.key)?;
        
        // Write value
        buf.write_u32::<LittleEndian>(entry.value.len() as u32)?;
        buf.write_all(&entry.value)?;
        
        // Calculate checksum
        let checksum = calculate_checksum(&buf);
        
        // Build final buffer with length and checksum
        let mut final_buf = Vec::new();
        final_buf.write_u32::<LittleEndian>((buf.len() + 4) as u32)?; // +4 for checksum
        final_buf.write_u32::<LittleEndian>(checksum)?;
        final_buf.extend_from_slice(&buf);
        
        Ok(final_buf)
    }

    /// Decode an entry from bytes
    fn decode_entry(data: &[u8]) -> Result<LogEntry, WalError> {
        let wal_entry = Self::decode_wal_entry(data)?;
        
        // Convert back to high-level format
        let operation = match wal_entry.operation {
            OperationType::Put => Operation::Set {
                key: String::from_utf8(wal_entry.key).map_err(|_| WalError::CorruptedEntry { offset: 0 })?,
                value: String::from_utf8(wal_entry.value).map_err(|_| WalError::CorruptedEntry { offset: 0 })?,
            },
            OperationType::Delete => Operation::Delete {
                key: String::from_utf8(wal_entry.key).map_err(|_| WalError::CorruptedEntry { offset: 0 })?,
            },
        };
        
        Ok(LogEntry {
            sequence: wal_entry.timestamp,
            operation,
        })
    }

    /// Decode a WAL entry from bytes
    fn decode_wal_entry(data: &[u8]) -> Result<WALEntry, WalError> {
        let mut cursor = io::Cursor::new(data);
        
        // Read and verify checksum
        let expected_checksum = cursor.read_u32::<LittleEndian>()?;
        let actual_checksum = calculate_checksum(&data[4..]);
        
        if expected_checksum != actual_checksum {
            return Err(WalError::ChecksumMismatch {
                expected: expected_checksum,
                actual: actual_checksum,
            });
        }
        
        // Read timestamp
        let timestamp = cursor.read_u64::<LittleEndian>()?;
        
        // Read operation type
        let op_type = cursor.read_u8()?;
        let operation = match op_type {
            1 => OperationType::Put,
            2 => OperationType::Delete,
            _ => return Err(WalError::CorruptedEntry { offset: cursor.position() }),
        };
        
        // Read key
        let key_len = cursor.read_u32::<LittleEndian>()? as usize;
        let mut key = vec![0u8; key_len];
        cursor.read_exact(&mut key)?;
        
        // Read value
        let value_len = cursor.read_u32::<LittleEndian>()? as usize;
        let mut value = vec![0u8; value_len];
        cursor.read_exact(&mut value)?;
        
        Ok(WALEntry {
            timestamp,
            operation,
            key,
            value,
        })
    }

    /// Sync data to disk based on sync mode
    fn sync(&mut self) -> Result<()> {
        self.file.flush()?;
        
        match self.sync_mode {
            SyncMode::None => {}
            SyncMode::DataOnly => {
                self.file.get_ref().sync_data()?;
            }
            SyncMode::Full => {
                self.file.get_ref().sync_all()?;
            }
        }
        
        Ok(())
    }

    /// Recover all entries from the log
    pub fn recover_entries(&self) -> Result<Vec<LogEntry>> {
        let mut entries = Vec::new();
        let mut file = File::open(&self.path)?;
        let mut reader = BufReader::new(&mut file);
        
        // Read and verify magic number
        let magic = reader.read_u32::<LittleEndian>()?;
        if magic != WAL_MAGIC {
            return Err(WalError::InvalidMagic.into());
        }
        
        // Read version
        let _version = reader.read_u32::<LittleEndian>()?;
        
        // Read entries
        loop {
            // Try to read entry length
            let length = match reader.read_u32::<LittleEndian>() {
                Ok(len) => len as usize,
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
                Err(e) => return Err(e.into()),
            };
            
            // Read entry data
            let mut data = vec![0u8; length];
            match reader.read_exact(&mut data) {
                Ok(_) => {},
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                    eprintln!("Warning: Incomplete entry detected, stopping recovery");
                    break;
                }
                Err(e) => return Err(e.into()),
            }
            
            // Decode entry
            match Self::decode_entry(&data) {
                Ok(entry) => entries.push(entry),
                Err(WalError::ChecksumMismatch { .. }) => {
                    // Stop reading on checksum error - file might be corrupted
                    eprintln!("Warning: Checksum mismatch detected, stopping recovery");
                    break;
                }
                Err(e) => return Err(e.into()),
            }
        }
        
        Ok(entries)
    }

    /// Recover WAL entries in FerrisDB format
    pub fn recover_wal_entries(&self) -> Result<Vec<WALEntry>> {
        let mut entries = Vec::new();
        let mut file = File::open(&self.path)?;
        let mut reader = BufReader::new(&mut file);
        
        // Read and verify magic number
        let magic = reader.read_u32::<LittleEndian>()?;
        if magic != WAL_MAGIC {
            return Err(WalError::InvalidMagic.into());
        }
        
        // Read version
        let _version = reader.read_u32::<LittleEndian>()?;
        
        // Read entries
        loop {
            // Try to read entry length
            let length = match reader.read_u32::<LittleEndian>() {
                Ok(len) => len as usize,
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
                Err(e) => return Err(e.into()),
            };
            
            // Read entry data
            let mut data = vec![0u8; length];
            match reader.read_exact(&mut data) {
                Ok(_) => {},
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                    eprintln!("Warning: Incomplete entry detected, stopping recovery");
                    break;
                }
                Err(e) => return Err(e.into()),
            }
            
            // Decode entry
            match Self::decode_wal_entry(&data) {
                Ok(entry) => entries.push(entry),
                Err(WalError::ChecksumMismatch { .. }) => {
                    eprintln!("Warning: Checksum mismatch detected, stopping recovery");
                    break;
                }
                Err(e) => return Err(e.into()),
            }
        }
        
        Ok(entries)
    }

    /// Rotate to a new file (for exercises)
    fn rotate(&mut self) -> Result<()> {
        // For now, just return error - students will implement in exercises
        Err(anyhow::anyhow!("File size limit reached, rotation not implemented"))
    }
}

/// Calculate CRC32 checksum
fn calculate_checksum(data: &[u8]) -> u32 {
    let mut hasher = Hasher::new();
    hasher.update(data);
    hasher.finalize()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_basic_append_and_recover() {
        let dir = tempdir().unwrap();
        let wal_path = dir.path().join("test.wal");
        
        // Write some entries
        {
            let mut wal = WalBuilder::new(&wal_path).build().unwrap();
            
            wal.append(Operation::Set {
                key: "user:1".to_string(),
                value: "Alice".to_string(),
            }).unwrap();
            
            wal.append(Operation::Set {
                key: "user:2".to_string(),
                value: "Bob".to_string(),
            }).unwrap();
            
            wal.append(Operation::Delete {
                key: "user:1".to_string(),
            }).unwrap();
        }
        
        // Recover and verify
        {
            let wal = WalBuilder::new(&wal_path).build().unwrap();
            let entries = wal.recover_entries().unwrap();
            
            assert_eq!(entries.len(), 3);
            assert_eq!(entries[0].sequence, 0);
            assert_eq!(entries[1].sequence, 1);
            assert_eq!(entries[2].sequence, 2);
            
            match &entries[0].operation {
                Operation::Set { key, value } => {
                    assert_eq!(key, "user:1");
                    assert_eq!(value, "Alice");
                }
                _ => panic!("Expected Set operation"),
            }
        }
    }

    #[test]
    fn test_wal_entry_api() {
        let dir = tempdir().unwrap();
        let wal_path = dir.path().join("wal_entry.wal");
        
        // Write using WALEntry API
        {
            let mut wal = WalBuilder::new(&wal_path).build().unwrap();
            
            let entry1 = WALEntry::new_put(
                b"user:1".to_vec(),
                b"Alice".to_vec(),
                100
            );
            wal.append_entry(&entry1).unwrap();
            
            let entry2 = WALEntry::new_delete(
                b"user:2".to_vec(),
                101
            );
            wal.append_entry(&entry2).unwrap();
        }
        
        // Recover using WALEntry API
        {
            let wal = WalBuilder::new(&wal_path).build().unwrap();
            let entries = wal.recover_wal_entries().unwrap();
            
            assert_eq!(entries.len(), 2);
            assert_eq!(entries[0].timestamp, 100);
            assert_eq!(entries[0].operation, OperationType::Put);
            assert_eq!(entries[0].key, b"user:1");
            assert_eq!(entries[0].value, b"Alice");
            
            assert_eq!(entries[1].timestamp, 101);
            assert_eq!(entries[1].operation, OperationType::Delete);
            assert_eq!(entries[1].key, b"user:2");
        }
    }
}