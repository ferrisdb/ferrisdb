//! Sorted String Table (SSTable) implementation
//!
//! SSTables are immutable on-disk files that store sorted key-value pairs.
//! They are the persistent storage format for FerrisDB and are organized
//! in levels for efficient reads.
//!
//! # Binary Format Specification
//!
//! ```text
//! ┌─────────────────┐
//! │   Data Block 0  │ ← Sorted (InternalKey, Value) pairs
//! ├─────────────────┤
//! │   Data Block 1  │
//! ├─────────────────┤
//! │       ...       │
//! ├─────────────────┤
//! │   Data Block N  │
//! ├─────────────────┤
//! │  Index Block    │ ← Block offsets and first keys
//! ├─────────────────┤
//! │  Bloom Filter   │ ← Probabilistic existence filter
//! ├─────────────────┤
//! │     Footer      │ ← Metadata and magic number
//! └─────────────────┘
//! ```
//!
//! ## Data Block Format (4KB default)
//!
//! ```text
//! ┌─────────────────┬─────────────────┬─────────────┐
//! │   Entry Count   │     Entries     │  Checksum   │
//! │    (4 bytes)    │   (variable)    │  (4 bytes)  │
//! └─────────────────┴─────────────────┴─────────────┘
//! ```
//!
//! ## Entry Format (within Data Block)
//!
//! ```text
//! ┌──────────┬─────────────┬───────────┬──────────────┬────────────┬──────────┐
//! │ Key Len  │ Value Len   │ Timestamp │  Operation   │    Key     │  Value   │
//! │(4 bytes) │ (4 bytes)   │ (8 bytes) │   (1 byte)   │ (var len)  │(var len) │
//! └──────────┴─────────────┴───────────┴──────────────┴────────────┴──────────┘
//! ```
//!
//! ## Index Block Format
//!
//! ```text
//! ┌─────────────────┬─────────────────┬─────────────┐
//! │   Entry Count   │     Entries     │  Checksum   │
//! │    (4 bytes)    │   (variable)    │  (4 bytes)  │
//! └─────────────────┴─────────────────┴─────────────┘
//! ```
//!
//! ## Index Entry Format
//!
//! ```text
//! ┌─────────────┬─────────────┬────────────┐
//! │ Block Offset│  Key Len    │    Key     │
//! │  (8 bytes)  │ (4 bytes)   │ (var len)  │
//! └─────────────┴─────────────┴────────────┘
//! ```
//!
//! ## Bloom Filter Format
//!
//! ```text
//! ┌─────────────────┬─────────────────┬─────────────┐
//! │   Bit Array     │   Hash Count    │  Checksum   │
//! │   (variable)    │    (4 bytes)    │  (4 bytes)  │
//! └─────────────────┴─────────────────┴─────────────┘
//! ```
//!
//! ## Footer Format (40 bytes)
//!
//! The SSTable footer contains metadata about the file's structure and is written
//! last during SSTable creation. This design enables single-pass sequential writes
//! during MemTable flush - we can build the index and bloom filter as we write
//! data blocks, then write the footer with their final positions. Reading an
//! SSTable requires only two I/O operations: seek to end minus 40 bytes, then
//! read the footer to locate all other components.
//!
//! ```text
//! ┌─────────────┬─────────────┬─────────────┬─────────────┬─────────────┐
//! │Index Offset │Index Length │Bloom Offset │Bloom Length │Magic Number │
//! │  (8 bytes)  │  (8 bytes)  │  (8 bytes)  │  (8 bytes)  │  (8 bytes)  │
//! └─────────────┴─────────────┴─────────────┴─────────────┴─────────────┘
//! ```
//!
//! The fixed-size footer (40 bytes) can be located with a simple calculation,
//! and the magic number validates file integrity - incomplete writes leave no
//! valid footer, making corruption detection straightforward.
//!
//! # Key Invariants
//!
//! 1. **Sorting**: Entries sorted by (user_key ASC, timestamp DESC)
//! 2. **Immutability**: SSTables are never modified after creation
//! 3. **Checksums**: All blocks include CRC32 checksums
//! 4. **Little Endian**: All multi-byte integers in little-endian format
//! 5. **Magic Number**: `0x46455252_49534442` ("FERRISDB" in ASCII)
//!
//! # Features
//!
//! - Block compression (LZ4, Snappy, None)
//! - Prefix compression for keys within blocks (future)
//! - Checksums for corruption detection
//! - Bloom filters for existence checks

use ferrisdb_core::{Key, Operation, Result, Timestamp, Value};
use std::fmt;

/// Magic number for SSTable files ("FERRISDB" in ASCII)
pub const SSTABLE_MAGIC: u64 = 0x46455252_49534442;

/// Default block size (4KB)
pub const DEFAULT_BLOCK_SIZE: usize = 4096;

/// Footer size in bytes
pub const FOOTER_SIZE: usize = 40;

/// Maximum key or value size (16MB)
pub const MAX_ENTRY_SIZE: usize = 16 * 1024 * 1024;

/// Internal key representation for SSTable entries
///
/// Combines user key with MVCC timestamp for versioning.
/// Keys are ordered by (user_key ASC, timestamp DESC).
/// Operation metadata is stored separately in SSTableEntry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InternalKey {
    pub user_key: Key,
    pub timestamp: Timestamp,
}

impl InternalKey {
    /// Creates a new internal key
    pub fn new(user_key: Key, timestamp: Timestamp) -> Self {
        Self {
            user_key,
            timestamp,
        }
    }

    /// Returns the total serialized size of this internal key
    pub fn serialized_size(&self) -> usize {
        4 + 8 + self.user_key.len() // key_len + timestamp + key
    }
}

impl PartialOrd for InternalKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for InternalKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;

        match self.user_key.cmp(&other.user_key) {
            Ordering::Equal => {
                // Newer timestamps come first (descending order)
                other.timestamp.cmp(&self.timestamp)
            }
            other => other,
        }
    }
}

impl fmt::Display for InternalKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}@{}",
            String::from_utf8_lossy(&self.user_key),
            self.timestamp
        )
    }
}

/// An entry in the SSTable containing key, value, and operation metadata
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SSTableEntry {
    /// The internal key (user_key + timestamp)
    pub key: InternalKey,
    /// The value associated with this key version
    pub value: Value,
    /// The operation type (Put/Delete) for this entry
    pub operation: Operation,
}

impl SSTableEntry {
    /// Creates a new SSTable entry
    pub fn new(key: InternalKey, value: Value, operation: Operation) -> Self {
        Self {
            key,
            value,
            operation,
        }
    }

    /// Returns the total serialized size of this entry
    pub fn serialized_size(&self) -> usize {
        self.key.serialized_size() + 4 + self.value.len() + 1 // key + value_len + value + operation
    }
}

/// Index entry pointing to a data block
#[derive(Debug, Clone)]
pub struct IndexEntry {
    /// File offset of the data block
    pub block_offset: u64,
    /// First key in the data block
    pub first_key: Key,
}

impl IndexEntry {
    /// Creates a new index entry
    pub fn new(block_offset: u64, first_key: Key) -> Self {
        Self {
            block_offset,
            first_key,
        }
    }

    /// Returns the serialized size of this index entry
    pub fn serialized_size(&self) -> usize {
        8 + 4 + self.first_key.len() // offset + key_len + key
    }
}

/// SSTable metadata stored in the footer
#[derive(Debug, Clone)]
pub struct Footer {
    /// Offset of the index block
    pub index_offset: u64,
    /// Length of the index block
    pub index_length: u64,
    /// Offset of the bloom filter
    pub bloom_offset: u64,
    /// Length of the bloom filter
    pub bloom_length: u64,
    /// Magic number for validation
    pub magic: u64,
}

impl Footer {
    /// Creates a new footer
    pub fn new(index_offset: u64, index_length: u64, bloom_offset: u64, bloom_length: u64) -> Self {
        Self {
            index_offset,
            index_length,
            bloom_offset,
            bloom_length,
            magic: SSTABLE_MAGIC,
        }
    }

    /// Serializes the footer to bytes
    pub fn to_bytes(&self) -> [u8; FOOTER_SIZE] {
        let mut bytes = [0u8; FOOTER_SIZE];

        bytes[0..8].copy_from_slice(&self.index_offset.to_le_bytes());
        bytes[8..16].copy_from_slice(&self.index_length.to_le_bytes());
        bytes[16..24].copy_from_slice(&self.bloom_offset.to_le_bytes());
        bytes[24..32].copy_from_slice(&self.bloom_length.to_le_bytes());
        bytes[32..40].copy_from_slice(&self.magic.to_le_bytes());

        bytes
    }

    /// Deserializes footer from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != FOOTER_SIZE {
            return Err(ferrisdb_core::Error::InvalidFormat(
                "Invalid footer size".to_string(),
            ));
        }

        let index_offset = u64::from_le_bytes(bytes[0..8].try_into().unwrap());
        let index_length = u64::from_le_bytes(bytes[8..16].try_into().unwrap());
        let bloom_offset = u64::from_le_bytes(bytes[16..24].try_into().unwrap());
        let bloom_length = u64::from_le_bytes(bytes[24..32].try_into().unwrap());
        let magic = u64::from_le_bytes(bytes[32..40].try_into().unwrap());

        if magic != SSTABLE_MAGIC {
            return Err(ferrisdb_core::Error::InvalidFormat(format!(
                "Invalid magic number: expected {}, got {}",
                SSTABLE_MAGIC, magic
            )));
        }

        Ok(Self {
            index_offset,
            index_length,
            bloom_offset,
            bloom_length,
            magic,
        })
    }
}

pub mod reader;
pub mod writer;

pub use reader::{SSTableIterator, SSTableReader, SSTableReaderInfo};
pub use writer::{SSTableInfo, SSTableWriter};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_internal_key_ordering() {
        let key1 = InternalKey::new(b"key1".to_vec(), 100);
        let key2 = InternalKey::new(b"key1".to_vec(), 200);
        let key3 = InternalKey::new(b"key2".to_vec(), 100);

        // Same user key: newer timestamp comes first
        assert!(key2 < key1);

        // Different user keys: lexicographic order
        assert!(key1 < key3);
        assert!(key2 < key3);
    }

    #[test]
    fn test_internal_key_serialized_size() {
        let key = InternalKey::new(b"test_key".to_vec(), 12345);
        let expected_size = 4 + 8 + 8; // key_len + timestamp + key
        assert_eq!(key.serialized_size(), expected_size);
    }

    #[test]
    fn test_footer_serialization() {
        let footer = Footer::new(1000, 200, 1200, 100);

        let bytes = footer.to_bytes();
        assert_eq!(bytes.len(), FOOTER_SIZE);

        let deserialized = Footer::from_bytes(&bytes).unwrap();
        assert_eq!(deserialized.index_offset, 1000);
        assert_eq!(deserialized.index_length, 200);
        assert_eq!(deserialized.bloom_offset, 1200);
        assert_eq!(deserialized.bloom_length, 100);
        assert_eq!(deserialized.magic, SSTABLE_MAGIC);
    }

    #[test]
    fn test_footer_invalid_magic() {
        let mut bytes = [0u8; FOOTER_SIZE];
        // Set invalid magic number
        bytes[32..40].copy_from_slice(&0x12345678u64.to_le_bytes());

        let result = Footer::from_bytes(&bytes);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invalid magic number"));
    }

    #[test]
    fn test_footer_invalid_size() {
        let bytes = [0u8; 10]; // Too small
        let result = Footer::from_bytes(&bytes);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invalid footer size"));
    }

    #[test]
    fn test_index_entry_serialized_size() {
        let entry = IndexEntry::new(1000, b"first_key".to_vec());
        let expected_size = 8 + 4 + 9; // offset + key_len + key
        assert_eq!(entry.serialized_size(), expected_size);
    }

    #[test]
    fn test_internal_key_display() {
        let key = InternalKey::new(b"test_key".to_vec(), 12345);
        let display = format!("{}", key);
        assert_eq!(display, "test_key@12345");

        let delete_key = InternalKey::new(b"del_key".to_vec(), 99999);
        let display = format!("{}", delete_key);
        assert_eq!(display, "del_key@99999");
    }

    #[test]
    fn test_internal_key_equality() {
        let key1 = InternalKey::new(b"key".to_vec(), 100);
        let key2 = InternalKey::new(b"key".to_vec(), 100);
        let key3 = InternalKey::new(b"key".to_vec(), 101);

        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
    }

    #[test]
    fn test_magic_number_ascii() {
        // Verify our magic number spells "FERRISDB" in ASCII
        let bytes = SSTABLE_MAGIC.to_be_bytes();
        let ascii = std::str::from_utf8(&bytes).unwrap();
        assert_eq!(ascii, "FERRISDB");
    }

    #[test]
    fn test_sstable_entry() {
        let key = InternalKey::new(b"test_key".to_vec(), 12345);
        let value = b"test_value".to_vec();
        let entry = SSTableEntry::new(key.clone(), value.clone(), Operation::Put);

        assert_eq!(entry.key, key);
        assert_eq!(entry.value, value);
        assert_eq!(entry.operation, Operation::Put);
    }

    #[test]
    fn test_sstable_entry_serialized_size() {
        let key = InternalKey::new(b"test_key".to_vec(), 12345);
        let value = b"test_value".to_vec();
        let entry = SSTableEntry::new(key, value, Operation::Put);

        // key_serialized_size + value_len(4) + value + operation(1)
        let expected_size = (4 + 8 + 8) + 4 + 10 + 1;
        assert_eq!(entry.serialized_size(), expected_size);
    }

    #[test]
    fn test_sstable_writer_reader_integration() {
        use crate::sstable::{SSTableReader, SSTableWriter};
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("integration.sst");

        // Create test data
        let test_entries = vec![
            (
                InternalKey::new(b"apple".to_vec(), 100),
                b"red fruit".to_vec(),
                Operation::Put,
            ),
            (
                InternalKey::new(b"banana".to_vec(), 200),
                b"yellow fruit".to_vec(),
                Operation::Put,
            ),
            (
                InternalKey::new(b"banana".to_vec(), 150),
                b"old yellow".to_vec(),
                Operation::Put,
            ),
            (
                InternalKey::new(b"cherry".to_vec(), 300),
                Vec::new(),
                Operation::Delete,
            ),
            (
                InternalKey::new(b"date".to_vec(), 250),
                b"sweet fruit".to_vec(),
                Operation::Put,
            ),
        ];

        // Write the SSTable
        {
            let mut writer = SSTableWriter::new(&path).unwrap();
            for (key, value, operation) in &test_entries {
                writer.add(key.clone(), value.clone(), *operation).unwrap();
            }
            let info = writer.finish().unwrap();
            assert_eq!(info.entry_count, test_entries.len());
        }

        // Read and verify
        {
            let mut reader = SSTableReader::open(&path).unwrap();

            // Test exact key lookups
            for (key, expected_value, _operation) in &test_entries {
                let result = reader.get(&key.user_key, key.timestamp).unwrap();
                assert_eq!(result.as_ref(), Some(expected_value));
            }

            // Test get_latest functionality
            let latest_banana = reader.get_latest(&b"banana".to_vec(), 1000).unwrap();
            assert!(latest_banana.is_some());
            let (value, timestamp, _) = latest_banana.unwrap();
            assert_eq!(value, b"yellow fruit".to_vec());
            assert_eq!(timestamp, 200);

            // Test get_latest with timestamp constraint
            let old_banana = reader.get_latest(&b"banana".to_vec(), 175).unwrap();
            assert!(old_banana.is_some());
            let (value, timestamp, _) = old_banana.unwrap();
            assert_eq!(value, b"old yellow".to_vec());
            assert_eq!(timestamp, 150);

            // Test missing key
            let missing = reader.get(&b"missing".to_vec(), 100).unwrap();
            assert_eq!(missing, None);

            // Test iterator
            let mut iter = reader.iter().unwrap();
            let mut count = 0;
            let mut last_key: Option<InternalKey> = None;

            while let Some(entry_result) = iter.next() {
                let entry = entry_result.unwrap();

                // Verify ordering
                if let Some(ref last) = last_key {
                    assert!(entry.key >= *last, "Entries not in sorted order");
                }
                last_key = Some(entry.key.clone());
                count += 1;
            }
            assert_eq!(count, test_entries.len());

            // Test range iterator
            let start_key = b"banana".to_vec();
            let end_key = b"date".to_vec();
            let mut range_iter = reader.range_iter(Some(&start_key), Some(&end_key)).unwrap();

            let mut range_entries = Vec::new();
            while let Some(entry_result) = range_iter.next() {
                let entry = entry_result.unwrap();
                assert!(entry.key.user_key >= start_key);
                assert!(entry.key.user_key < end_key);
                range_entries.push(entry);
            }

            // Should include banana versions and cherry
            assert!(range_entries.len() >= 3);
        }
    }
}
