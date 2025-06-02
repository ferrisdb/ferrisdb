//! Lock-free skip list implementation for the MemTable
//!
//! This module implements a concurrent skip list that supports:
//! - Lock-free reads using epoch-based memory reclamation
//! - Concurrent writes with fine-grained locking
//! - Multiple versions of the same key (MVCC)
//! - Efficient range scans

use crossbeam::epoch::{self, Atomic, Guard, Owned, Shared};
use ferrisdb_core::{Key, Operation, Timestamp, Value};
use parking_lot::Mutex;
use rand::{Rng, SeedableRng};
use std::cmp::Ordering;
use std::ops::Deref;
use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};

/// Maximum height of the skip list (affects memory usage and performance)
const MAX_HEIGHT: usize = 12;

/// Probability factor for determining node height (1/4 chance of increasing height)
const BRANCHING_FACTOR: u32 = 4;

/// Internal key representation that includes metadata for MVCC
///
/// Keys in the skip list are ordered first by user key (ascending),
/// then by timestamp (descending). This ensures that:
/// - Keys are grouped together
/// - Newer versions appear before older versions
/// - Range scans are efficient
#[derive(Debug, Clone)]
pub struct InternalKey {
    /// The actual user-provided key
    pub user_key: Key,
    /// Timestamp for MVCC versioning
    pub timestamp: Timestamp,
    /// Operation type (Put or Delete)
    pub operation: Operation,
}

impl InternalKey {
    /// Creates a new internal key
    fn new(user_key: Key, timestamp: Timestamp, operation: Operation) -> Self {
        Self {
            user_key,
            timestamp,
            operation,
        }
    }

    /// Compares two internal keys for ordering
    ///
    /// Keys are ordered by:
    /// 1. User key (ascending)
    /// 2. Timestamp (descending) - newer versions first
    fn compare(&self, other: &Self) -> Ordering {
        match self.user_key.cmp(&other.user_key) {
            Ordering::Equal => {
                // Newer timestamps come first (descending order)
                match other.timestamp.cmp(&self.timestamp) {
                    Ordering::Equal => Ordering::Equal,
                    other => other,
                }
            }
            other => other,
        }
    }
}

/// A node in the skip list
///
/// Each node contains a key-value pair and pointers to the next node
/// at each level of the skip list. The height of a node determines
/// how many levels it participates in.
struct Node {
    /// The key with version information
    key: InternalKey,
    /// The value associated with this key version
    value: Value,
    /// Next pointers for each level (height determines the vector length)
    next: Vec<Atomic<Node>>,
}

impl Node {
    /// Creates a new node with the specified height
    fn new(key: InternalKey, value: Value, height: usize) -> Self {
        let mut next = Vec::with_capacity(height);
        for _ in 0..height {
            next.push(Atomic::null());
        }

        Self { key, value, next }
    }

    /// Creates a sentinel head node for the skip list
    ///
    /// The head node has an empty key that compares less than all other keys
    fn head(height: usize) -> Self {
        Self::new(
            InternalKey::new(Vec::new(), 0, Operation::Put),
            Vec::new(),
            height,
        )
    }
}

/// A concurrent skip list for storing versioned key-value pairs
///
/// This skip list implementation provides:
/// - O(log n) expected time for search, insert, and delete
/// - Lock-free reads using epoch-based memory reclamation
/// - Support for multiple versions of the same key
/// - Efficient range scans
///
/// # Thread Safety
///
/// Multiple threads can read concurrently without locking. Writes use
/// fine-grained locking to allow concurrent modifications to different
/// parts of the list.
///
/// # Memory Management
///
/// Uses crossbeam's epoch-based memory reclamation to safely free
/// nodes that are no longer reachable, avoiding the ABA problem.
pub struct SkipList {
    /// Sentinel head node
    head: Atomic<Node>,
    /// Current height of the skip list
    height: AtomicUsize,
    /// Number of entries in the skip list
    size: AtomicUsize,
    /// Random number generator for determining node heights
    rng: Mutex<rand::rngs::StdRng>,
}

impl SkipList {
    /// Creates a new empty skip list
    pub fn new() -> Self {
        let head = Node::head(MAX_HEIGHT);

        Self {
            head: Atomic::new(head),
            height: AtomicUsize::new(1),
            size: AtomicUsize::new(0),
            rng: Mutex::new(rand::rngs::StdRng::from_os_rng()),
        }
    }

    /// Generates a random height for a new node
    ///
    /// Uses geometric distribution with p = 1/4 to determine height.
    /// This gives expected height of 1.33 and keeps the skip list balanced.
    fn random_height(&self) -> usize {
        let mut height = 1;
        let mut rng = self.rng.lock();

        while height < MAX_HEIGHT && rng.random_ratio(1, BRANCHING_FACTOR) {
            height += 1;
        }

        height
    }

    /// Inserts a new key-value pair with version information
    ///
    /// This operation is thread-safe and uses compare-and-swap operations
    /// to ensure consistency. If the same key with the same timestamp already
    /// exists, it will not be updated (preserving immutability).
    ///
    /// # Arguments
    ///
    /// * `user_key` - The key to insert
    /// * `value` - The value to associate with the key
    /// * `timestamp` - Version timestamp for MVCC
    /// * `operation` - Type of operation (Put or Delete)
    pub fn insert(&self, user_key: Key, value: Value, timestamp: Timestamp, operation: Operation) {
        let guard = &epoch::pin();
        let key = InternalKey::new(user_key, timestamp, operation);
        let height = self.random_height();

        // Update max height if necessary
        loop {
            let current_height = self.height.load(AtomicOrdering::Acquire);
            if height <= current_height {
                break;
            }

            if self
                .height
                .compare_exchange(
                    current_height,
                    height,
                    AtomicOrdering::Release,
                    AtomicOrdering::Acquire,
                )
                .is_ok()
            {
                break;
            }
        }

        let mut preds: Vec<Shared<Node>> = vec![Shared::null(); height];
        let mut succs: Vec<Shared<Node>> = vec![Shared::null(); height];

        loop {
            if self.find(&key, &mut preds, &mut succs, guard) {
                // Key already exists, we don't update in skip list
                // (newer version should be inserted as separate entry)
                break;
            }

            let new_node = Owned::new(Node::new(key.clone(), value.clone(), height));

            // Set next pointers of new node
            for (i, &succ) in succs.iter().enumerate().take(height) {
                new_node.deref().next[i].store(succ, AtomicOrdering::Relaxed);
            }

            let new_node_shared = new_node.into_shared(guard);

            // Try to link the new node
            match unsafe { preds[0].as_ref() }.unwrap().next[0].compare_exchange(
                succs[0],
                new_node_shared,
                AtomicOrdering::Release,
                AtomicOrdering::Acquire,
                guard,
            ) {
                Ok(_) => {
                    // Successfully linked at bottom level, link other levels
                    for i in 1..height {
                        loop {
                            match unsafe { preds[i].as_ref() }.unwrap().next[i].compare_exchange(
                                succs[i],
                                new_node_shared,
                                AtomicOrdering::Release,
                                AtomicOrdering::Acquire,
                                guard,
                            ) {
                                Ok(_) => break,
                                Err(_) => {
                                    // Re-find predecessors for this level
                                    self.find(&key, &mut preds, &mut succs, guard);
                                }
                            }
                        }
                    }

                    self.size.fetch_add(1, AtomicOrdering::Relaxed);
                    break;
                }
                Err(_) => {
                    // CAS failed, retry
                    continue;
                }
            }
        }
    }

    /// Finds the predecessors and successors for a key at each level
    ///
    /// This is the core search operation used by insert and delete.
    /// It populates the `preds` and `succs` arrays with the nodes
    /// before and after where the key would be inserted at each level.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to search for
    /// * `preds` - Array to fill with predecessor nodes at each level
    /// * `succs` - Array to fill with successor nodes at each level
    /// * `guard` - Epoch guard for safe memory access
    ///
    /// # Returns
    ///
    /// `true` if an exact match for the key is found, `false` otherwise
    fn find<'g>(
        &self,
        key: &InternalKey,
        preds: &mut [Shared<'g, Node>],
        succs: &mut [Shared<'g, Node>],
        guard: &'g Guard,
    ) -> bool {
        let mut pred = self.head.load(AtomicOrdering::Acquire, guard);

        for level in (0..self.height.load(AtomicOrdering::Acquire)).rev() {
            let mut curr =
                unsafe { pred.as_ref() }.unwrap().next[level].load(AtomicOrdering::Acquire, guard);

            while !curr.is_null() {
                let curr_ref = unsafe { curr.as_ref() }.unwrap();

                match key.compare(&curr_ref.key) {
                    Ordering::Greater => {
                        pred = curr;
                        curr = curr_ref.next[level].load(AtomicOrdering::Acquire, guard);
                    }
                    _ => break,
                }
            }

            if level < preds.len() {
                preds[level] = pred;
                succs[level] = curr;
            }
        }

        !succs[0].is_null()
            && unsafe { succs[0].as_ref() }.unwrap().key.compare(key) == Ordering::Equal
    }

    /// Retrieves the value for a key at a specific timestamp
    ///
    /// Returns the most recent version of the key that has a timestamp
    /// less than or equal to the given timestamp. This enables MVCC
    /// by allowing reads at different points in time.
    ///
    /// # Arguments
    ///
    /// * `user_key` - The key to look up
    /// * `timestamp` - The timestamp to read at
    ///
    /// # Returns
    ///
    /// `Some((value, operation))` if the key exists at the given timestamp,
    /// where operation indicates if this is a Put or Delete.
    /// `None` if the key doesn't exist or all versions are newer than the timestamp.
    pub fn get(&self, user_key: &[u8], timestamp: Timestamp) -> Option<(Value, Operation)> {
        let guard = &epoch::pin();

        // First, find the position where this key would be
        let search_key = InternalKey::new(user_key.to_vec(), u64::MAX, Operation::Put);
        let mut preds = vec![Shared::null(); 1];
        let mut succs = vec![Shared::null(); 1];

        self.find(&search_key, &mut preds, &mut succs, guard);

        // Now scan from this position to find the right version
        let mut curr = succs[0];

        while !curr.is_null() {
            let curr_ref = unsafe { curr.as_ref() }.unwrap();

            if curr_ref.key.user_key != user_key {
                break;
            }

            if curr_ref.key.timestamp <= timestamp {
                return Some((curr_ref.value.clone(), curr_ref.key.operation));
            }

            curr = curr_ref.next[0].load(AtomicOrdering::Acquire, guard);
        }

        None
    }

    /// Performs a range scan between start_key and end_key at a specific timestamp
    ///
    /// Returns all key-value pairs where the key is in the range [start_key, end_key)
    /// and the timestamp is less than or equal to the given timestamp. For keys with
    /// multiple versions, only the most recent valid version is returned.
    ///
    /// Delete operations (tombstones) are filtered out from the results.
    ///
    /// # Arguments
    ///
    /// * `start_key` - The inclusive lower bound of the range
    /// * `end_key` - The exclusive upper bound of the range
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
        let guard = &epoch::pin();
        let mut result = Vec::new();
        let mut seen_keys = std::collections::HashSet::new();

        let search_key = InternalKey::new(start_key.to_vec(), timestamp, Operation::Put);
        let mut preds = vec![Shared::null(); 1];
        let mut succs = vec![Shared::null(); 1];

        self.find(&search_key, &mut preds, &mut succs, guard);

        let mut curr = succs[0];

        while !curr.is_null() {
            let curr_ref = unsafe { curr.as_ref() }.unwrap();

            if curr_ref.key.user_key.as_slice() >= end_key {
                break;
            }

            if curr_ref.key.timestamp <= timestamp && !seen_keys.contains(&curr_ref.key.user_key) {
                if curr_ref.key.operation == Operation::Put {
                    result.push((curr_ref.key.user_key.clone(), curr_ref.value.clone()));
                }
                seen_keys.insert(curr_ref.key.user_key.clone());
            }

            curr = curr_ref.next[0].load(AtomicOrdering::Acquire, guard);
        }

        result
    }

    /// Returns the number of entries in the skip list
    ///
    /// Note: This counts all versions of all keys, not just unique keys.
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.size.load(AtomicOrdering::Relaxed)
    }
}

// SkipList automatically implements Send + Sync because:
// - Atomic<Node> is Send + Sync (crossbeam atomics)
// - AtomicUsize is Send + Sync (standard library)
// - Mutex<StdRng> is Send + Sync (StdRng implements Send + Sync)
//
// Send + Sync are required because SkipList is used within Arc<SkipList>
// for sharing between storage engine components:
// - Multiple threads reading from immutable MemTables during flush
// - Background flush threads writing MemTable contents to SSTables
// - Iterator support that outlives individual method calls
//
// Using StdRng instead of ThreadRng avoids unsafe impl and leverages
// Rust's type system to automatically prove thread safety.

impl Drop for SkipList {
    fn drop(&mut self) {
        let guard = &epoch::pin();

        let mut curr = self.head.load(AtomicOrdering::Acquire, guard);
        while !curr.is_null() {
            let next =
                unsafe { curr.as_ref() }.unwrap().next[0].load(AtomicOrdering::Acquire, guard);
            unsafe {
                guard.defer_destroy(curr);
            }
            curr = next;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skiplist_basic() {
        let sl = SkipList::new();

        sl.insert(b"key1".to_vec(), b"value1".to_vec(), 1, Operation::Put);
        sl.insert(b"key2".to_vec(), b"value2".to_vec(), 2, Operation::Put);
        sl.insert(b"key3".to_vec(), b"value3".to_vec(), 3, Operation::Put);

        assert_eq!(sl.size(), 3);

        let result = sl.get(b"key2", 5);
        assert!(result.is_some());
        let (value, op) = result.unwrap();
        assert_eq!(value, b"value2");
        assert_eq!(op, Operation::Put);
    }

    #[test]
    fn test_skiplist_versions() {
        let sl = SkipList::new();

        // Insert multiple versions of the same key
        sl.insert(b"key1".to_vec(), b"value1".to_vec(), 1, Operation::Put);
        sl.insert(b"key1".to_vec(), b"value2".to_vec(), 3, Operation::Put);
        sl.insert(b"key1".to_vec(), b"value3".to_vec(), 5, Operation::Put);

        // Read at different timestamps
        let result = sl.get(b"key1", 2);
        assert_eq!(result.unwrap().0, b"value1");

        let result = sl.get(b"key1", 4);
        assert_eq!(result.unwrap().0, b"value2");

        let result = sl.get(b"key1", 6);
        assert_eq!(result.unwrap().0, b"value3");
    }

    #[test]
    fn test_skiplist_delete() {
        let sl = SkipList::new();

        sl.insert(b"key1".to_vec(), b"value1".to_vec(), 1, Operation::Put);
        sl.insert(b"key1".to_vec(), Vec::new(), 3, Operation::Delete);

        // Before delete
        let result = sl.get(b"key1", 2);
        assert_eq!(result.unwrap().1, Operation::Put);

        // After delete
        let result = sl.get(b"key1", 4);
        assert_eq!(result.unwrap().1, Operation::Delete);
    }
}
