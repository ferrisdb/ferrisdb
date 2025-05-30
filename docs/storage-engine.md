---
layout: default
title: Storage Engine Design
nav_exclude: true
permalink: /storage-engine/
---

Detailed design and implementation of FerrisDB's custom LSM-tree storage engine
{: .fs-6 .fw-300 }

## Table of contents

{: .no_toc .text-delta }

1. TOC
   {:toc}

---

## Overview

FerrisDB implements a custom LSM-tree (Log-Structured Merge-tree) storage engine from scratch. This document details the design and implementation of each component.

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Write Path                           │
│  Write Request → WAL → MemTable → (Flush) → SSTable     │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│                     Read Path                           │
│  Read Request → MemTable → Immutable MemTables →        │
│                 L0 SSTables → L1 SSTables → ...         │
└─────────────────────────────────────────────────────────┘
```

## Components

### 1. Write-Ahead Log (WAL)

The WAL ensures durability by persisting all writes before they're applied to the MemTable.

**Design:**

- Append-only log file with sequential writes
- Each entry contains: [length][checksum][key_len][key][value_len][value][timestamp]
- Sync to disk after each write (configurable)
- Rotation when file reaches size limit (default: 64MB)

**Format:**

```rust
struct WALEntry {
    length: u32,          // Total entry length
    checksum: u32,        // CRC32 checksum
    timestamp: u64,       // Transaction timestamp
    operation: u8,        // Put = 1, Delete = 2
    key_len: u32,
    key: Vec<u8>,
    value_len: u32,       // 0 for deletes
    value: Vec<u8>,
}
```

### 2. MemTable

In-memory sorted structure for recent writes. We implement a concurrent skip list for O(log n) operations.

**Design:**

- Lock-free skip list implementation with configurable max height (default: 12)
- Thread-safe using epoch-based memory reclamation
- Size limit triggers flush to SSTable (default: 4MB)
- Supports concurrent reads during writes

**Skip List Structure:**

```rust
struct SkipListNode<K, V> {
    key: K,
    value: V,
    timestamp: u64,
    next: Vec<AtomicPtr<SkipListNode<K, V>>>,
}

struct SkipList<K, V> {
    head: Box<SkipListNode<K, V>>,
    max_height: usize,
    current_height: AtomicUsize,
    size: AtomicUsize,
}
```

### 3. SSTable (Sorted String Table)

Immutable on-disk files storing sorted key-value pairs.

**File Format:**

```
┌─────────────┐
│   Header    │ - Magic number, version, metadata
├─────────────┤
│ Data Blocks │ - Sorted KV pairs in 4KB blocks
├─────────────┤
│Index Blocks │ - Block offsets for binary search
├─────────────┤
│ Bloom Filter│ - Optional, for existence checks
├─────────────┤
│   Footer    │ - Index offset, bloom offset, checksum
└─────────────┘
```

**Data Block Format:**

- Prefix compression for keys
- Restart points every 16 keys
- Block compression (LZ4 or Snappy)

**Index Format:**

- One entry per data block
- Contains: last key in block + block offset
- Enables binary search across blocks

### 4. Compaction

Merges and organizes SSTables to maintain read performance.

**Leveled Compaction Strategy:**

- L0: Direct flushes from MemTable (overlapping)
- L1-L6: Non-overlapping, exponentially larger
- Size ratios: L0=10MB, L1=100MB, L2=1GB, etc.

**Compaction Process:**

1. Select candidate files based on size/age
2. Merge overlapping key ranges
3. Apply tombstone deletion
4. Write new SSTable(s)
5. Atomically update manifest
6. Delete old files

### 5. Manifest

Tracks the current version of the database and all live SSTable files.

**Format:**

```rust
struct Manifest {
    version: u64,
    levels: Vec<Level>,
    sequence_number: u64,
    log_number: u64,
}

struct Level {
    level_num: u32,
    files: Vec<FileMetadata>,
}

struct FileMetadata {
    number: u64,
    size: u64,
    smallest_key: Vec<u8>,
    largest_key: Vec<u8>,
    smallest_timestamp: u64,
    largest_timestamp: u64,
}
```

### 6. Block Cache

LRU cache for frequently accessed SSTable blocks.

**Design:**

- Sharded by key hash to reduce contention
- Configurable size (default: 128MB)
- Tracks access patterns for adaptive caching

### 7. Bloom Filters

Probabilistic data structure to avoid unnecessary disk reads.

**Implementation:**

- Bits per key: 10 (1% false positive rate)
- Hash functions: MurmurHash3
- Stored in SSTable footer

## Operations

### Write Operation

```
1. Append to WAL
2. Insert into MemTable
3. If MemTable full:
   a. Make immutable
   b. Create new MemTable
   c. Schedule background flush
4. Return success
```

### Read Operation

```
1. Check MemTable
2. Check immutable MemTables
3. For each level (L0 to L6):
   a. Check bloom filter
   b. Binary search index
   c. Read data block
   d. Search within block
4. Return value or not found
```

### Delete Operation

- Inserts tombstone marker
- Actual deletion happens during compaction
- Tombstones have timestamps for MVCC

## Optimizations

### 1. Write Optimizations

- Group commit for WAL
- Parallel MemTable and WAL writes
- Write batching API

### 2. Read Optimizations

- Bloom filters to skip files
- Block cache for hot data
- Parallel searches across levels
- Read-ahead for sequential scans

### 3. Memory Management

- Memory-mapped files for read-only SSTables
- Direct I/O to bypass OS cache
- Custom allocators for MemTable

## Configuration

```rust
struct StorageConfig {
    // WAL
    wal_dir: PathBuf,
    wal_sync_mode: SyncMode,
    wal_size_limit: usize,

    // MemTable
    memtable_size: usize,
    max_immutable_memtables: usize,

    // SSTable
    block_size: usize,
    compression: CompressionType,

    // Compaction
    level0_file_num_compaction_trigger: i32,
    max_bytes_for_level_base: u64,
    max_bytes_for_level_multiplier: f64,

    // Cache
    block_cache_size: usize,
    bloom_filter_bits_per_key: i32,
}
```

## Testing Strategy

### Correctness Tests

- Single-threaded operations
- Concurrent operations
- Crash recovery
- Compaction correctness

### Performance Tests

- Write throughput
- Read latency
- Mixed workloads
- Large value handling

### Stress Tests

- Random operations with verification
- Crash injection
- Disk full scenarios
- Memory pressure

## Implementation Progress

### ✅ Phase 1: Basic Functionality (COMPLETED)

- [x] Simple WAL with binary encoding
- [x] Concurrent skip list MemTable
- [x] Basic SSTable writer/reader (skeleton)
- [x] MVCC support with timestamps

### 🚧 Phase 2: Performance (IN PROGRESS)

- [ ] Complete SSTable implementation
- [ ] Bloom filters
- [ ] Block cache
- [ ] Automatic compaction

### ⏳ Phase 3: Advanced Features (PLANNED)

- [ ] Compression
- [ ] Column families
- [ ] Backup/restore
- [ ] Statistics collection

## Current Implementation Status

As of Day 1, we have successfully implemented:

**✅ Write-Ahead Log**

- Binary format with CRC32 checksums
- Little-endian encoding for cross-platform compatibility
- Atomic writes with proper error handling
- Recovery by replaying log entries

**✅ MemTable with Concurrent Skip List**

- Lock-free implementation using crossbeam
- MVCC support with timestamp-based versioning
- Proper key ordering: (user_key ASC, timestamp DESC)
- Epoch-based memory reclamation for thread safety

**🚧 Next Steps**

- Complete SSTable format implementation
- Add bloom filters for read optimization
- Implement compaction strategy
- Add comprehensive integration tests

## References

- LevelDB Implementation Notes
- RocksDB Wiki
- WiredTiger Architecture Guide
- "The Log-Structured Merge-Tree" paper

---

**Related Documentation:**

- [Overall Architecture]({{ '/architecture/' | relative_url }})
- [GitHub Repository]({{ site.project.repo_url }})
- [Development Blog]({{ '/blog/' | relative_url }})
