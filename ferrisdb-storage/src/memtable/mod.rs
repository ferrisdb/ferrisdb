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

mod memtable;
mod skiplist;

pub use memtable::MemTable;
