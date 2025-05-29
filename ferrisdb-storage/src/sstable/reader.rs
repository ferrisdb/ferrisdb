//! SSTable reader implementation

use crate::sstable::{Footer, IndexEntry, InternalKey, SSTableEntry, FOOTER_SIZE, SSTABLE_MAGIC};
use ferrisdb_core::{Error, Key, Operation, Result, Timestamp, Value};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;

/// Reader for querying SSTable files
///
/// The SSTableReader provides efficient point lookups and range scans over
/// immutable SSTable files. It uses the index to locate data blocks and
/// supports both exact key matches and range queries.
///
/// # Example
///
/// ```ignore
/// use ferrisdb_storage::sstable::reader::SSTableReader;
///
/// let mut reader = SSTableReader::open("path/to/sstable.sst")?;
///
/// // Get exact key-timestamp match
/// if let Some(value) = reader.get(&b"key1".to_vec(), 100)? {
///     println!("Found value: {:?}", value);
/// }
///
/// // Get latest version of a key
/// if let Some((value, timestamp, operation)) = reader.get_latest(&b"key1".to_vec(), 1000)? {
///     println!("Latest value: {:?} at timestamp {}", value, timestamp);
/// }
/// ```
pub struct SSTableReader {
    /// Buffered reader for the file
    reader: BufReader<File>,
    /// SSTable metadata from footer
    footer: Footer,
    /// Index entries for efficient block lookup
    index: Vec<IndexEntry>,
    /// Cached data blocks (block_offset -> entries)
    block_cache: BTreeMap<u64, Vec<SSTableEntry>>,
}

impl std::fmt::Debug for SSTableReader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SSTableReader")
            .field("footer", &self.footer)
            .field("index_count", &self.index.len())
            .field("cached_blocks", &self.block_cache.len())
            .finish()
    }
}

impl SSTableReader {
    /// Opens an SSTable file for reading
    ///
    /// This method:
    /// 1. Opens the file and reads the footer
    /// 2. Validates the magic number
    /// 3. Reads and parses the index block
    /// 4. Prepares the reader for queries
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the SSTable file
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be opened
    /// - The file format is invalid
    /// - The magic number doesn't match
    /// - Index data is corrupted
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        // Read and parse footer
        let footer = Self::read_footer(&mut reader)?;

        // Read and parse index
        let index = Self::read_index(&mut reader, &footer)?;

        Ok(Self {
            reader,
            footer,
            index,
            block_cache: BTreeMap::new(),
        })
    }

    /// Looks up a specific key at a specific timestamp in the SSTable
    ///
    /// Returns the value associated with the exact key-timestamp combination,
    /// or None if not found. This searches for an exact match of both the
    /// user key and timestamp.
    ///
    /// # Performance
    ///
    /// Uses binary search based on InternalKey ordering (user_key ASC, timestamp DESC)
    /// to directly locate the exact key-timestamp combination in O(log n) time.
    /// This is significantly faster than linear search for blocks with many entries.
    ///
    /// # Arguments
    ///
    /// * `user_key` - The user key to search for
    /// * `timestamp` - The exact timestamp to match
    ///
    /// # Errors
    ///
    /// Returns an error if an I/O error occurs during lookup
    pub fn get(&mut self, user_key: &Key, timestamp: Timestamp) -> Result<Option<Value>> {
        // Find the block that might contain this key
        let block_offset = match self.find_block_for_key(user_key) {
            Some(offset) => offset,
            None => return Ok(None), // Key is outside the range of this SSTable
        };

        // Load the block (from cache or disk)
        let entries = self.load_block(block_offset)?;

        // Create target key for binary search (Operation doesn't affect ordering)
        let target_key = InternalKey::new(user_key.clone(), timestamp, ferrisdb_core::Operation::Put);
        
        // Use binary search to find exact key match
        match entries.binary_search_by(|entry| entry.key.cmp(&target_key)) {
            Ok(index) => {
                // Found exact match
                Ok(Some(entries[index].value.clone()))
            }
            Err(_) => {
                // No exact match found
                Ok(None)
            }
        }
    }

    /// Finds the latest version of a user key
    ///
    /// This method searches for the most recent version of a user key
    /// (highest timestamp) that is visible to the given timestamp.
    ///
    /// # Performance
    ///
    /// Uses binary search to locate the first entry for the user key, then linear
    /// search through versions (ordered by timestamp DESC) to find the latest
    /// version within the timestamp limit. This is optimal for MVCC workloads.
    ///
    /// # Arguments
    ///
    /// * `user_key` - The user key to search for
    /// * `max_timestamp` - Maximum timestamp to consider (for snapshot isolation)
    ///
    /// # Returns
    ///
    /// Returns (value, timestamp, operation) if found, None otherwise
    pub fn get_latest(
        &mut self,
        user_key: &Key,
        max_timestamp: Timestamp,
    ) -> Result<Option<(Value, Timestamp, Operation)>> {
        // Find the block that might contain this key
        let block_offset = match self.find_block_for_key(user_key) {
            Some(offset) => offset,
            None => return Ok(None),
        };

        // Load the block
        let entries = self.load_block(block_offset)?;

        // Use binary search to find the first entry with matching user_key
        let start_index = entries.partition_point(|entry| entry.key.user_key < *user_key);
        
        // Linear search through versions (timestamp DESC) for the latest valid version
        for i in start_index..entries.len() {
            let entry = &entries[i];
            
            // Stop if we've moved to a different user_key
            if entry.key.user_key != *user_key {
                break;
            }
            
            // Check if this version is within our timestamp limit
            if entry.key.timestamp <= max_timestamp {
                return Ok(Some((
                    entry.value.clone(),
                    entry.key.timestamp,
                    entry.key.operation,
                )));
            }
        }

        Ok(None)
    }

    /// Creates an iterator over all entries in the SSTable
    ///
    /// The iterator yields entries in sorted order (user_key ASC, timestamp DESC).
    pub fn iter(&mut self) -> Result<SSTableIterator> {
        SSTableIterator::new(self)
    }

    /// Creates an iterator over a range of keys
    ///
    /// # Arguments
    ///
    /// * `start_key` - Optional start key (inclusive)
    /// * `end_key` - Optional end key (exclusive)
    pub fn range_iter(
        &mut self,
        start_key: Option<&Key>,
        end_key: Option<&Key>,
    ) -> Result<SSTableIterator> {
        SSTableIterator::new_range(self, start_key, end_key)
    }

    /// Returns metadata about the SSTable
    pub fn info(&self) -> SSTableReaderInfo {
        SSTableReaderInfo {
            index_entries: self.index.len(),
            footer: self.footer.clone(),
        }
    }

    /// Reads the footer from the end of the file
    fn read_footer(reader: &mut BufReader<File>) -> Result<Footer> {
        // Seek to the start of the footer (file_size - FOOTER_SIZE)
        let file_size = reader.seek(SeekFrom::End(0))?;
        if file_size < FOOTER_SIZE as u64 {
            return Err(Error::InvalidFormat(
                "File too small to contain footer".to_string(),
            ));
        }

        reader.seek(SeekFrom::End(-(FOOTER_SIZE as i64)))?;

        // Read footer bytes
        let mut footer_bytes = [0u8; FOOTER_SIZE];
        reader.read_exact(&mut footer_bytes)?;

        // Parse footer
        Footer::from_bytes(&footer_bytes)
    }

    /// Reads and parses the index block
    fn read_index(reader: &mut BufReader<File>, footer: &Footer) -> Result<Vec<IndexEntry>> {
        // Seek to index block
        reader.seek(SeekFrom::Start(footer.index_offset))?;

        // Read entry count
        let mut count_bytes = [0u8; 4];
        reader.read_exact(&mut count_bytes)?;
        let entry_count = u32::from_le_bytes(count_bytes) as usize;

        let mut index_entries = Vec::with_capacity(entry_count);

        // Read each index entry
        for _ in 0..entry_count {
            // Read block offset
            let mut offset_bytes = [0u8; 8];
            reader.read_exact(&mut offset_bytes)?;
            let block_offset = u64::from_le_bytes(offset_bytes);

            // Read key length
            let mut key_len_bytes = [0u8; 4];
            reader.read_exact(&mut key_len_bytes)?;
            let key_len = u32::from_le_bytes(key_len_bytes) as usize;

            // Read key
            let mut key = vec![0u8; key_len];
            reader.read_exact(&mut key)?;

            index_entries.push(IndexEntry::new(block_offset, key));
        }

        // Read and verify checksum (placeholder for now)
        let mut checksum_bytes = [0u8; 4];
        reader.read_exact(&mut checksum_bytes)?;
        let _checksum = u32::from_le_bytes(checksum_bytes);
        // TODO: Verify checksum

        Ok(index_entries)
    }

    /// Finds the block offset that might contain the given user key
    fn find_block_for_key(&self, user_key: &Key) -> Option<u64> {
        if self.index.is_empty() {
            return None;
        }

        // Find the last block whose first key is <= user_key
        let mut result = None;
        for entry in &self.index {
            if entry.first_key <= *user_key {
                result = Some(entry.block_offset);
            } else {
                break;
            }
        }

        // If no block's first key is <= user_key, the key might be in the first block
        // (if it's smaller than the first key in the file)
        result.or(Some(self.index[0].block_offset))
    }

    /// Loads a data block, using cache if available
    fn load_block(&mut self, block_offset: u64) -> Result<&Vec<SSTableEntry>> {
        if !self.block_cache.contains_key(&block_offset) {
            let entries = self.read_block(block_offset)?;
            self.block_cache.insert(block_offset, entries);
        }
        Ok(self.block_cache.get(&block_offset).unwrap())
    }

    /// Reads a data block from disk
    fn read_block(&mut self, block_offset: u64) -> Result<Vec<SSTableEntry>> {
        // Seek to block
        self.reader.seek(SeekFrom::Start(block_offset))?;

        // Read entry count
        let mut count_bytes = [0u8; 4];
        self.reader.read_exact(&mut count_bytes)?;
        let entry_count = u32::from_le_bytes(count_bytes) as usize;

        let mut entries = Vec::with_capacity(entry_count);

        // Read each entry
        for _ in 0..entry_count {
            let entry = self.read_entry()?;
            entries.push(entry);
        }

        // Read and verify checksum (placeholder for now)
        let mut checksum_bytes = [0u8; 4];
        self.reader.read_exact(&mut checksum_bytes)?;
        let _checksum = u32::from_le_bytes(checksum_bytes);
        // TODO: Verify checksum

        Ok(entries)
    }

    /// Reads a single entry from the current position
    fn read_entry(&mut self) -> Result<SSTableEntry> {
        // Read key length
        let mut key_len_bytes = [0u8; 4];
        self.reader.read_exact(&mut key_len_bytes)?;
        let key_len = u32::from_le_bytes(key_len_bytes) as usize;

        // Read value length
        let mut value_len_bytes = [0u8; 4];
        self.reader.read_exact(&mut value_len_bytes)?;
        let value_len = u32::from_le_bytes(value_len_bytes) as usize;

        // Read timestamp
        let mut timestamp_bytes = [0u8; 8];
        self.reader.read_exact(&mut timestamp_bytes)?;
        let timestamp = u64::from_le_bytes(timestamp_bytes);

        // Read operation
        let mut op_byte = [0u8; 1];
        self.reader.read_exact(&mut op_byte)?;
        let operation = match op_byte[0] {
            0 => Operation::Put,
            1 => Operation::Delete,
            _ => {
                return Err(Error::InvalidFormat(format!(
                    "Invalid operation byte: {}",
                    op_byte[0]
                )))
            }
        };

        // Read key
        let mut user_key = vec![0u8; key_len];
        self.reader.read_exact(&mut user_key)?;

        // Read value
        let mut value = vec![0u8; value_len];
        self.reader.read_exact(&mut value)?;

        let internal_key = InternalKey::new(user_key, timestamp, operation);
        Ok(SSTableEntry::new(internal_key, value))
    }
}

/// Iterator over SSTable entries
pub struct SSTableIterator<'a> {
    reader: &'a mut SSTableReader,
    current_block_idx: usize,
    current_entry_idx: usize,
    start_key: Option<Key>,
    end_key: Option<Key>,
    current_block_entries: Option<Vec<SSTableEntry>>,
}

impl<'a> SSTableIterator<'a> {
    /// Creates a new iterator over all entries
    fn new(reader: &'a mut SSTableReader) -> Result<Self> {
        Ok(Self {
            reader,
            current_block_idx: 0,
            current_entry_idx: 0,
            start_key: None,
            end_key: None,
            current_block_entries: None,
        })
    }

    /// Creates a new iterator over a key range
    fn new_range(
        reader: &'a mut SSTableReader,
        start_key: Option<&Key>,
        end_key: Option<&Key>,
    ) -> Result<Self> {
        let mut iter = Self::new(reader)?;
        iter.start_key = start_key.cloned();
        iter.end_key = end_key.cloned();

        // Find the starting block if we have a start key
        if let Some(start) = start_key {
            if let Some(block_offset) = iter.reader.find_block_for_key(start) {
                // Find the index of this block
                for (idx, entry) in iter.reader.index.iter().enumerate() {
                    if entry.block_offset == block_offset {
                        iter.current_block_idx = idx;
                        break;
                    }
                }
            }
        }

        Ok(iter)
    }

    /// Loads the current block if needed
    fn ensure_current_block(&mut self) -> Result<bool> {
        if self.current_block_idx >= self.reader.index.len() {
            return Ok(false); // No more blocks
        }

        if self.current_block_entries.is_none() {
            let block_offset = self.reader.index[self.current_block_idx].block_offset;
            let entries = self.reader.read_block(block_offset)?;
            self.current_block_entries = Some(entries);
            self.current_entry_idx = 0;
        }

        Ok(true)
    }

    /// Advances to the next block
    fn advance_to_next_block(&mut self) {
        self.current_block_idx += 1;
        self.current_entry_idx = 0;
        self.current_block_entries = None;
    }
}

impl<'a> Iterator for SSTableIterator<'a> {
    type Item = Result<SSTableEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Ensure we have a current block loaded
            match self.ensure_current_block() {
                Ok(false) => return None, // No more blocks
                Err(e) => return Some(Err(e)),
                Ok(true) => {}
            }

            let entries = self.current_block_entries.as_ref().unwrap();

            // Check if we've reached the end of current block
            if self.current_entry_idx >= entries.len() {
                self.advance_to_next_block();
                continue;
            }

            let entry = &entries[self.current_entry_idx];
            self.current_entry_idx += 1;

            // Check range constraints
            if let Some(ref start) = self.start_key {
                if entry.key.user_key < *start {
                    continue;
                }
            }

            if let Some(ref end) = self.end_key {
                if entry.key.user_key >= *end {
                    return None; // Reached end of range
                }
            }

            return Some(Ok(entry.clone()));
        }
    }
}

/// Metadata about an SSTable from reader perspective
#[derive(Debug, Clone)]
pub struct SSTableReaderInfo {
    /// Number of index entries (approximately number of blocks)
    pub index_entries: usize,
    /// Footer metadata
    pub footer: Footer,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sstable::writer::SSTableWriter;
    use tempfile::TempDir;

    fn create_test_sstable() -> (TempDir, std::path::PathBuf, Vec<(InternalKey, Value)>) {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("test.sst");

        let mut writer = SSTableWriter::new(&path).unwrap();

        let test_data = vec![
            (
                InternalKey::new(b"key1".to_vec(), 100, Operation::Put),
                b"value1".to_vec(),
            ),
            (
                InternalKey::new(b"key1".to_vec(), 50, Operation::Put),
                b"old_value1".to_vec(),
            ),
            (
                InternalKey::new(b"key2".to_vec(), 200, Operation::Delete),
                Vec::new(),
            ),
            (
                InternalKey::new(b"key3".to_vec(), 150, Operation::Put),
                b"value3".to_vec(),
            ),
        ];

        for (key, value) in &test_data {
            writer.add(key.clone(), value.clone()).unwrap();
        }

        writer.finish().unwrap();

        (temp_dir, path, test_data)
    }

    #[test]
    fn test_sstable_reader_basic() {
        let (_temp_dir, path, test_data) = create_test_sstable();

        let mut reader = SSTableReader::open(&path).unwrap();

        // Test exact key lookups
        let result = reader.get(&test_data[0].0.user_key, test_data[0].0.timestamp).unwrap();
        assert_eq!(result, Some(test_data[0].1.clone()));

        let result = reader.get(&test_data[2].0.user_key, test_data[2].0.timestamp).unwrap();
        assert_eq!(result, Some(test_data[2].1.clone()));

        // Test key that doesn't exist
        let result = reader.get(&b"missing".to_vec(), 100).unwrap();
        assert_eq!(result, None);

        // Test existing key with wrong timestamp
        let result = reader.get(&test_data[0].0.user_key, 999).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn test_sstable_reader_get_latest() {
        let (_temp_dir, path, _test_data) = create_test_sstable();

        let mut reader = SSTableReader::open(&path).unwrap();

        // Test getting latest version of key1
        let result = reader.get_latest(&b"key1".to_vec(), 1000).unwrap();
        assert!(result.is_some());
        let (value, timestamp, operation) = result.unwrap();
        assert_eq!(value, b"value1".to_vec());
        assert_eq!(timestamp, 100);
        assert_eq!(operation, Operation::Put);

        // Test with timestamp constraint
        let result = reader.get_latest(&b"key1".to_vec(), 75).unwrap();
        assert!(result.is_some());
        let (value, timestamp, _) = result.unwrap();
        assert_eq!(value, b"old_value1".to_vec());
        assert_eq!(timestamp, 50);

        // Test with timestamp too low
        let result = reader.get_latest(&b"key1".to_vec(), 25).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn test_sstable_reader_iterator() {
        let (_temp_dir, path, test_data) = create_test_sstable();

        let mut reader = SSTableReader::open(&path).unwrap();
        let mut iter = reader.iter().unwrap();

        // Collect all entries
        let mut entries = Vec::new();
        while let Some(entry_result) = iter.next() {
            entries.push(entry_result.unwrap());
        }

        // Should have all test data entries
        assert_eq!(entries.len(), test_data.len());

        // Verify they're in sorted order
        for i in 1..entries.len() {
            assert!(entries[i - 1].key <= entries[i].key);
        }
    }

    #[test]
    fn test_sstable_reader_range_iterator() {
        let (_temp_dir, path, _test_data) = create_test_sstable();

        let mut reader = SSTableReader::open(&path).unwrap();

        // Test range from key1 to key3 (exclusive)
        let start_key = b"key1".to_vec();
        let end_key = b"key3".to_vec();
        let mut iter = reader.range_iter(Some(&start_key), Some(&end_key)).unwrap();

        let mut entries = Vec::new();
        while let Some(entry_result) = iter.next() {
            entries.push(entry_result.unwrap());
        }

        // Should include key1 versions and key2, but not key3
        assert!(entries.len() >= 2);
        for entry in &entries {
            assert!(entry.key.user_key >= start_key);
            assert!(entry.key.user_key < end_key);
        }
    }

    #[test]
    fn test_sstable_reader_info() {
        let (_temp_dir, path, _test_data) = create_test_sstable();

        let reader = SSTableReader::open(&path).unwrap();
        let info = reader.info();

        assert!(info.index_entries > 0);
        assert_eq!(info.footer.magic, SSTABLE_MAGIC);
    }

    #[test]
    fn test_sstable_reader_invalid_file() {
        let temp_dir = TempDir::new().unwrap();
        let invalid_path = temp_dir.path().join("invalid.sst");

        // Create a file that's too small
        std::fs::write(&invalid_path, b"too small").unwrap();

        let result = SSTableReader::open(&invalid_path);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("File too small to contain footer"));
    }

    #[test]
    fn test_sstable_reader_corrupted_magic() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("corrupted.sst");

        // Create a file with invalid magic number
        let mut invalid_footer = [0u8; FOOTER_SIZE];
        invalid_footer[32..40].copy_from_slice(&0x12345678u64.to_le_bytes());
        std::fs::write(&path, invalid_footer).unwrap();

        let result = SSTableReader::open(&path);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invalid magic number"));
    }

    #[test]
    fn test_sstable_reader_binary_search_performance() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("large_block.sst");

        // Create SSTable with many entries to force large blocks
        let mut writer = SSTableWriter::with_block_size(&path, 8192).unwrap(); // 8KB blocks

        // Add many entries to create larger blocks for testing binary search efficiency
        for i in 0..200 {
            let key = InternalKey::new(
                format!("key_{:06}", i).into_bytes(),
                i as u64,
                Operation::Put,
            );
            let value = format!("value_{}", i).into_bytes();
            writer.add(key, value).unwrap();
        }

        writer.finish().unwrap();

        // Test reading from the SSTable
        let mut reader = SSTableReader::open(&path).unwrap();

        // Test lookups throughout the range to ensure binary search works correctly
        for i in [0, 50, 100, 150, 199] {
            let key = format!("key_{:06}", i).into_bytes();
            let expected_value = format!("value_{}", i).into_bytes();
            
            let result = reader.get(&key, i as u64).unwrap();
            assert_eq!(result, Some(expected_value));
        }

        // Test get_latest functionality on the large block
        let result = reader.get_latest(&b"key_000100".to_vec(), 1000).unwrap();
        assert!(result.is_some());
        let (value, timestamp, _) = result.unwrap();
        assert_eq!(value, b"value_100".to_vec());
        assert_eq!(timestamp, 100);

        // Test non-existent key
        let result = reader.get(&b"key_999999".to_vec(), 100).unwrap();
        assert_eq!(result, None);
    }
}
