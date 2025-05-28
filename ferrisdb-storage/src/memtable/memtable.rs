use super::skiplist::SkipList;
use ferrisdb_core::{Key, Operation, Result, Timestamp, Value};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// In-memory table for recent writes
///
/// The MemTable stores recent write operations in memory using a skip list
/// data structure. It provides fast lookups and supports multiple versions
/// of the same key for MVCC (Multi-Version Concurrency Control).
///
/// When the MemTable reaches its size limit, it becomes immutable and a
/// new MemTable is created for new writes. The immutable MemTable is then
/// flushed to disk as an SSTable.
///
/// # Thread Safety
///
/// The MemTable is thread-safe and supports concurrent reads and writes.
/// The underlying skip list uses lock-free algorithms for high performance.
///
/// # Example
///
/// ```
/// use ferrisdb_storage::memtable::MemTable;
/// use ferrisdb_core::Operation;
///
/// let memtable = MemTable::new(4 * 1024 * 1024); // 4MB
///
/// // Insert a key-value pair
/// memtable.put(b"key1".to_vec(), b"value1".to_vec(), 100)?;
///
/// // Delete a key
/// memtable.delete(b"key2".to_vec(), 101)?;
///
/// // Read a value
/// if let Some((value, op)) = memtable.get(b"key1", 150) {
///     match op {
///         Operation::Put => println!("Value: {:?}", value),
///         Operation::Delete => println!("Key was deleted"),
///     }
/// }
/// # Ok::<(), ferrisdb_core::Error>(())
/// ```
pub struct MemTable {
    skiplist: Arc<SkipList>,
    size: AtomicUsize,
    size_limit: usize,
}

impl MemTable {
    /// Creates a new MemTable with the specified size limit
    ///
    /// # Arguments
    ///
    /// * `size_limit` - Maximum size in bytes before the MemTable is considered full
    pub fn new(size_limit: usize) -> Self {
        Self {
            skiplist: Arc::new(SkipList::new()),
            size: AtomicUsize::new(0),
            size_limit,
        }
    }

    /// Inserts a key-value pair with the given timestamp
    ///
    /// # Errors
    ///
    /// Returns an error if the entry would exceed the size limit.
    pub fn put(&self, key: Key, value: Value, timestamp: Timestamp) -> Result<()> {
        let entry_size = key.len() + value.len() + 16; // Approximate size

        if self.would_exceed_limit(entry_size) {
            return Err(ferrisdb_core::Error::StorageEngine(
                "MemTable size limit would be exceeded".to_string(),
            ));
        }

        self.skiplist.insert(key, value, timestamp, Operation::Put);
        self.size.fetch_add(entry_size, Ordering::Relaxed);
        Ok(())
    }

    /// Marks a key as deleted with the given timestamp
    ///
    /// Delete operations create tombstone entries that are cleaned up
    /// during compaction.
    ///
    /// # Errors
    ///
    /// Returns an error if the entry would exceed the size limit.
    pub fn delete(&self, key: Key, timestamp: Timestamp) -> Result<()> {
        let entry_size = key.len() + 16; // Approximate size

        if self.would_exceed_limit(entry_size) {
            return Err(ferrisdb_core::Error::StorageEngine(
                "MemTable size limit would be exceeded".to_string(),
            ));
        }

        self.skiplist
            .insert(key, Vec::new(), timestamp, Operation::Delete);
        self.size.fetch_add(entry_size, Ordering::Relaxed);
        Ok(())
    }

    /// Retrieves the value for a key at the given timestamp
    ///
    /// Returns the most recent version of the key that is <= the given timestamp.
    /// If a delete operation is found, it returns the operation type so the
    /// caller can handle tombstones appropriately.
    pub fn get(&self, key: &[u8], timestamp: Timestamp) -> Option<(Value, Operation)> {
        self.skiplist.get(key, timestamp)
    }

    /// Scans a range of keys at the given timestamp
    ///
    /// Returns all key-value pairs where:
    /// - The key is >= start_key and < end_key
    /// - The timestamp is <= the given timestamp
    /// - Delete operations are filtered out
    pub fn scan(
        &self,
        start_key: &[u8],
        end_key: &[u8],
        timestamp: Timestamp,
    ) -> Vec<(Key, Value)> {
        self.skiplist.scan(start_key, end_key, timestamp)
    }

    /// Returns the approximate memory usage of the MemTable
    pub fn approximate_size(&self) -> usize {
        self.size.load(Ordering::Relaxed)
    }

    /// Checks if the MemTable has reached its size limit
    pub fn is_full(&self) -> bool {
        self.approximate_size() >= self.size_limit
    }

    fn would_exceed_limit(&self, additional_size: usize) -> bool {
        self.size.load(Ordering::Relaxed) + additional_size > self.size_limit
    }

    /// Creates an iterator over the MemTable
    ///
    /// The iterator provides ordered access to all entries in the MemTable.
    pub fn iter(&self) -> MemTableIterator {
        MemTableIterator::new(self.skiplist.clone())
    }
}

/// Iterator over MemTable entries
///
/// Provides ordered iteration through all key-value pairs in the MemTable.
/// The iterator maintains a consistent snapshot view even if the MemTable
/// is modified during iteration.
pub struct MemTableIterator {
    #[allow(dead_code)] // TODO: Remove when implementing iterator
    skiplist: Arc<SkipList>,
    current_key: Option<Key>,
}

impl MemTableIterator {
    fn new(skiplist: Arc<SkipList>) -> Self {
        Self {
            skiplist,
            current_key: None,
        }
    }

    /// Seeks to the first key >= the given key
    pub fn seek(&mut self, key: &[u8]) {
        self.current_key = Some(key.to_vec());
    }

    /// Seeks to the first key in the MemTable
    pub fn seek_to_first(&mut self) {
        self.current_key = Some(Vec::new());
    }

    /// Advances to the next entry
    ///
    /// Returns the next key-value pair with its timestamp and operation type.
    /// Returns `None` when the end is reached.
    pub fn next(&mut self) -> Option<(Key, Value, Timestamp, Operation)> {
        // TODO: Implement proper iteration through skip list
        // This requires maintaining position in the skip list and
        // advancing through the linked nodes
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memtable_basic() {
        let memtable = MemTable::new(1024 * 1024); // 1MB

        memtable
            .put(b"key1".to_vec(), b"value1".to_vec(), 1)
            .unwrap();
        memtable
            .put(b"key2".to_vec(), b"value2".to_vec(), 2)
            .unwrap();
        memtable.delete(b"key3".to_vec(), 3).unwrap();

        let result = memtable.get(b"key1", 10);
        assert!(result.is_some());
        let (value, op) = result.unwrap();
        assert_eq!(value, b"value1");
        assert_eq!(op, Operation::Put);

        let result = memtable.get(b"key3", 10);
        assert!(result.is_some());
        let (_, op) = result.unwrap();
        assert_eq!(op, Operation::Delete);
    }

    #[test]
    fn test_memtable_size_limit() {
        let memtable = MemTable::new(100); // Very small limit

        // This should succeed
        memtable
            .put(b"key1".to_vec(), b"small".to_vec(), 1)
            .unwrap();

        // This should fail due to size limit
        let result = memtable.put(
            b"key_with_very_long_name".to_vec(),
            b"value_with_very_long_content_that_exceeds_limit".to_vec(),
            2,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_memtable_scan() {
        let memtable = MemTable::new(1024 * 1024);

        memtable.put(b"a".to_vec(), b"value_a".to_vec(), 1).unwrap();
        memtable.put(b"b".to_vec(), b"value_b".to_vec(), 1).unwrap();
        memtable.put(b"c".to_vec(), b"value_c".to_vec(), 1).unwrap();
        memtable.put(b"d".to_vec(), b"value_d".to_vec(), 1).unwrap();

        let results = memtable.scan(b"b", b"d", 10);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].0, b"b");
        assert_eq!(results[1].0, b"c");
    }
}
