//! Storage engine for FerrisDB
//!
//! This crate implements a LSM-tree (Log-Structured Merge-tree) based storage engine
//! with the following components:
//!
//! - **Write-Ahead Log (WAL)**: Ensures durability of writes
//! - **MemTable**: In-memory write buffer using a skip list
//! - **SSTable**: Sorted String Table for persistent storage
//! - **Compaction**: Background process to merge and optimize SSTables
//!
//! # Architecture
//!
//! ```text
//! Write Path:
//! Client Write → WAL → MemTable → (when full) → SSTable
//!
//! Read Path:
//! Client Read → MemTable → Immutable MemTables → SSTables (L0 → L6)
//! ```
//!
//! # Example
//!
//! ```no_run
//! use ferrisdb_storage::{StorageEngine, StorageConfig};
//!
//! let config = StorageConfig::default();
//! let engine = StorageEngine::new(config);
//! ```

pub mod config;
pub mod memtable;
pub mod sstable;
pub mod storage_engine;
pub mod wal;

pub use config::StorageConfig;
pub use storage_engine::StorageEngine;
