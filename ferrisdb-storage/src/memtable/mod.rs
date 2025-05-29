//! In-memory storage using skip list data structure
//!
//! The MemTable is an in-memory write buffer that stores recent writes
//! before they are flushed to disk as SSTables. It uses a concurrent
//! skip list implementation that provides:
//!
//! - O(log n) insert, delete, and lookup operations
//! - Lock-free reads with epoch-based memory reclamation
//! - Support for multiple versions of the same key (MVCC)
//! - Efficient range scans
//!
//! # Example
//!
//! ```
//! use ferrisdb_storage::memtable::MemTable;
//!
//! let memtable = MemTable::new(4 * 1024 * 1024); // 4MB capacity
//!
//! // Insert a key-value pair
//! memtable.put(b"key".to_vec(), b"value".to_vec(), 1)?;
//!
//! // Read the value
//! if let Some((value, op)) = memtable.get(b"key", 10) {
//!     println!("Found: {:?}", value);
//! }
//! # Ok::<(), ferrisdb_core::Error>(())
//! ```

use self::skip_list::SkipList;
use ferrisdb_core::{Error, Key, Operation, Result, Timestamp, Value};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// In-memory write buffer using a concurrent skip list
///
/// MemTable stores recent writes in memory before they are flushed to disk
/// as SSTables. It supports:
/// - Concurrent reads and writes
/// - Multiple versions of the same key (MVCC)
/// - Efficient range scans
/// - Memory usage tracking
///
/// # Thread Safety
///
/// MemTable is designed to be shared across multiple threads safely.
/// It uses a lock-free skip list internally for optimal performance.
///
/// # Memory Management & LSM-Tree Integration
///
/// When the MemTable reaches its size limit, it becomes immutable and a new
/// MemTable is created for writes. The immutable MemTable is shared between:
/// - The storage engine (for ongoing reads)
/// - Background flush threads (to write SSTable to disk)
/// - Iterators (that may outlive the original method calls)
///
/// This sharing pattern requires `Arc<SkipList>` for zero-copy access across
/// multiple components without expensive cloning of the entire data structure.
pub struct MemTable {
    /// The underlying skip list data structure
    ///
    /// Uses Arc for shared ownership in LSM-tree scenarios:
    /// - Storage engine keeps immutable MemTables for reads during flush
    /// - Background threads flush MemTable to SSTable
    /// - Iterators need concurrent access without blocking writes
    skiplist: Arc<SkipList>,
    /// Current memory usage in bytes (approximate)
    memory_usage: AtomicUsize,
    /// Maximum memory capacity before flush is needed
    max_size: usize,
}

impl MemTable {
    /// Creates a new MemTable with the specified capacity
    ///
    /// # Arguments
    ///
    /// * `max_size` - Maximum memory usage in bytes before flush is required
    ///
    /// # Example
    ///
    /// ```
    /// use ferrisdb_storage::memtable::MemTable;
    ///
    /// let memtable = MemTable::new(4 * 1024 * 1024); // 4MB
    /// ```
    pub fn new(max_size: usize) -> Self {
        Self {
            skiplist: Arc::new(SkipList::new()),
            memory_usage: AtomicUsize::new(0),
            max_size,
        }
    }

    /// Inserts a key-value pair into the MemTable
    ///
    /// This operation is atomic and thread-safe. The timestamp is used
    /// for MVCC - multiple versions of the same key can coexist.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to insert
    /// * `value` - The value to associate with the key
    /// * `timestamp` - MVCC timestamp for this version
    ///
    /// # Errors
    ///
    /// Returns an error if the MemTable is over capacity after the insert.
    /// Callers should flush the MemTable to disk when this occurs.
    pub fn put(&self, key: Key, value: Value, timestamp: Timestamp) -> Result<()> {
        let size_estimate = key.len() + value.len() + 64; // 64 bytes overhead estimate

        self.skiplist.insert(key, value, timestamp, Operation::Put);

        let new_usage = self
            .memory_usage
            .fetch_add(size_estimate, Ordering::Relaxed);

        if new_usage + size_estimate > self.max_size {
            return Err(Error::MemTableFull);
        }

        Ok(())
    }

    /// Marks a key as deleted (tombstone)
    ///
    /// Instead of immediately removing the key, this creates a tombstone
    /// entry that will be cleaned up during compaction.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to delete
    /// * `timestamp` - MVCC timestamp for this delete operation
    pub fn delete(&self, key: Key, timestamp: Timestamp) -> Result<()> {
        let size_estimate = key.len() + 64; // 64 bytes overhead estimate

        self.skiplist
            .insert(key, Vec::new(), timestamp, Operation::Delete);

        let new_usage = self
            .memory_usage
            .fetch_add(size_estimate, Ordering::Relaxed);

        if new_usage + size_estimate > self.max_size {
            return Err(Error::MemTableFull);
        }

        Ok(())
    }

    /// Retrieves the value for a key at a specific timestamp
    ///
    /// Returns the most recent version of the key that is visible
    /// at the given timestamp.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look up
    /// * `timestamp` - The timestamp to read at
    ///
    /// # Returns
    ///
    /// - `Some((value, Operation::Put))` if the key exists and is not deleted
    /// - `Some((_, Operation::Delete))` if the key has been deleted
    /// - `None` if the key doesn't exist or all versions are newer
    pub fn get(&self, key: &[u8], timestamp: Timestamp) -> Option<(Value, Operation)> {
        self.skiplist.get(key, timestamp)
    }

    /// Performs a range scan over keys at a specific timestamp
    ///
    /// Returns all key-value pairs where the key is in the range [start_key, end_key)
    /// and the timestamp is less than or equal to the given timestamp.
    ///
    /// Deleted keys (tombstones) are filtered out from the results.
    ///
    /// # Arguments
    ///
    /// * `start_key` - Inclusive lower bound
    /// * `end_key` - Exclusive upper bound  
    /// * `timestamp` - The timestamp to read at
    ///
    /// # Returns
    ///
    /// A vector of (key, value) pairs in ascending key order
    pub fn scan(
        &self,
        start_key: &[u8],
        end_key: &[u8],
        timestamp: Timestamp,
    ) -> Vec<(Key, Value)> {
        self.skiplist.scan(start_key, end_key, timestamp)
    }

    /// Returns the approximate memory usage in bytes
    ///
    /// This is used to determine when the MemTable should be flushed
    /// to disk to free up memory.
    pub fn memory_usage(&self) -> usize {
        self.memory_usage.load(Ordering::Relaxed)
    }

    /// Returns true if the MemTable is at or over capacity
    ///
    /// When this returns true, the MemTable should be marked as immutable
    /// and flushed to disk as an SSTable.
    pub fn is_full(&self) -> bool {
        self.memory_usage() >= self.max_size
    }

    /// Returns the number of entries in the MemTable
    ///
    /// Note: This counts all versions of all keys, including tombstones.
    pub fn entry_count(&self) -> usize {
        self.skiplist.size()
    }
}

mod skip_list;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memtable_basic() {
        let memtable = MemTable::new(1024);

        memtable
            .put(b"key1".to_vec(), b"value1".to_vec(), 1)
            .unwrap();
        memtable
            .put(b"key2".to_vec(), b"value2".to_vec(), 2)
            .unwrap();

        let result = memtable.get(b"key1", 10);
        assert!(result.is_some());
        let (value, op) = result.unwrap();
        assert_eq!(value, b"value1");
        assert_eq!(op, Operation::Put);

        let result = memtable.get(b"key2", 10);
        assert!(result.is_some());
        let (value, op) = result.unwrap();
        assert_eq!(value, b"value2");
        assert_eq!(op, Operation::Put);
    }

    #[test]
    fn test_memtable_scan() {
        let memtable = MemTable::new(1024);

        memtable
            .put(b"key1".to_vec(), b"value1".to_vec(), 1)
            .unwrap();
        memtable
            .put(b"key2".to_vec(), b"value2".to_vec(), 2)
            .unwrap();
        memtable
            .put(b"key3".to_vec(), b"value3".to_vec(), 3)
            .unwrap();

        let results = memtable.scan(b"key1", b"key3", 10);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0], (b"key1".to_vec(), b"value1".to_vec()));
        assert_eq!(results[1], (b"key2".to_vec(), b"value2".to_vec()));
    }

    #[test]
    fn test_memtable_size_limit() {
        let memtable = MemTable::new(100); // Very small limit

        // First insert should succeed
        memtable
            .put(b"key1".to_vec(), b"value1".to_vec(), 1)
            .unwrap();

        // Eventually we should hit the limit
        let mut insert_count = 1;
        loop {
            let key = format!("key{}", insert_count).into_bytes();
            let value = format!("value{}", insert_count).into_bytes();

            match memtable.put(key, value, insert_count as u64) {
                Ok(_) => insert_count += 1,
                Err(Error::MemTableFull) => break,
                Err(e) => panic!("Unexpected error: {:?}", e),
            }

            if insert_count > 1000 {
                panic!("Never hit size limit");
            }
        }

        assert!(memtable.is_full());
    }
}
