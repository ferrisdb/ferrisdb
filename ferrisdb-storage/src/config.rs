//! Configuration for the storage engine

use ferrisdb_core::{CompressionType, SyncMode};
use std::path::PathBuf;

/// Configuration options for the storage engine
///
/// This struct contains all tunable parameters for the LSM-tree storage engine,
/// including paths, size limits, and performance tuning options.
///
/// # Example
///
/// ```
/// use ferrisdb_storage::StorageConfig;
/// use ferrisdb_core::{CompressionType, SyncMode};
///
/// let config = StorageConfig {
///     data_dir: "./data".into(),
///     wal_dir: "./data/wal".into(),
///     wal_sync_mode: SyncMode::Normal,
///     memtable_size: 4 * 1024 * 1024, // 4MB
///     compression: CompressionType::Lz4,
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone)]
pub struct StorageConfig {
    /// Directory for storing SSTable files
    pub data_dir: PathBuf,

    /// Directory for Write-Ahead Log files
    pub wal_dir: PathBuf,

    /// Synchronization mode for WAL writes
    /// - `None`: No sync (fastest, least durable)
    /// - `Normal`: Sync to OS buffer
    /// - `Full`: Sync to disk
    pub wal_sync_mode: SyncMode,

    /// Maximum size of a single WAL file before rotation (in bytes)
    pub wal_size_limit: usize,

    /// Maximum size of active MemTable before flush (in bytes)
    pub memtable_size: usize,

    /// Maximum number of immutable MemTables to keep before blocking writes
    pub max_immutable_memtables: usize,

    /// Size of each data block in SSTable files (in bytes)
    pub block_size: usize,

    /// Compression algorithm for SSTable blocks
    pub compression: CompressionType,

    /// Number of L0 files that trigger compaction
    pub level0_file_num_compaction_trigger: i32,

    /// Target size for L1 (in bytes)
    pub max_bytes_for_level_base: u64,

    /// Size multiplier between levels (L2 = L1 * multiplier)
    pub max_bytes_for_level_multiplier: f64,

    /// Size of the block cache for SSTable reads (in bytes)
    pub block_cache_size: usize,

    /// Bits per key for bloom filters (10 = ~1% false positive rate)
    pub bloom_filter_bits_per_key: i32,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            data_dir: PathBuf::from("./data"),
            wal_dir: PathBuf::from("./data/wal"),
            wal_sync_mode: SyncMode::Normal,
            wal_size_limit: 64 * 1024 * 1024, // 64MB
            memtable_size: 4 * 1024 * 1024,   // 4MB
            max_immutable_memtables: 2,
            block_size: 4 * 1024, // 4KB
            compression: CompressionType::Lz4,
            level0_file_num_compaction_trigger: 4,
            max_bytes_for_level_base: 10 * 1024 * 1024, // 10MB
            max_bytes_for_level_multiplier: 10.0,
            block_cache_size: 128 * 1024 * 1024, // 128MB
            bloom_filter_bits_per_key: 10,
        }
    }
}
