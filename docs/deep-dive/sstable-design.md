---
layout: page
title: "SSTable Format Design: Building Efficient Persistent Storage"
subtitle: "How FerrisDB implements sorted string tables with binary search optimization"
permalink: /deep-dive/sstable-design/
---

On Day 2 of FerrisDB development, we designed and implemented SSTables (Sorted String Tables) from scratch. This deep dive explores the design decisions, binary format, and optimizations that make SSTables the backbone of LSM-tree storage engines.

## What Problem Do SSTables Solve?

MemTables are fast but have limitations:

- **Limited by RAM** - Can't store more data than available memory
- **Volatile** - Data lost on crash (even with WAL, recovery is slow)
- **No compression** - In-memory data structures can't be compressed

SSTables solve these by providing:

- **Persistent storage** on disk
- **Immutability** - Once written, never modified (enables caching)
- **Compression-friendly** - Sequential data compresses well
- **Fast lookups** - Sorted structure enables binary search

## FerrisDB's SSTable Format

### High-Level Structure

```text
┌─────────────────┐
│   Data Block 1  │  ← 4KB blocks containing sorted K-V pairs
├─────────────────┤
│   Data Block 2  │
├─────────────────┤
│       ...       │
├─────────────────┤
│   Data Block N  │
├─────────────────┤
│   Index Block   │  ← Maps block number to first key
├─────────────────┤
│     Footer      │  ← Metadata and magic number
└─────────────────┘
```

### Binary Format Details

#### Data Block Structure

Each data block contains entries in this format:

```rust
// For each entry in the block:
┌────────────┬────────────┬───────────┬──────────┬───────────┬─────┬───────┬───────────┐
│ key_len    │ value_len  │ operation │ timestamp│ user_key  │ ... │ value │ (next)    │
│ (4 bytes)  │ (4 bytes)  │ (1 byte)  │ (8 bytes)│ (variable)│     │ (var) │           │
└────────────┴────────────┴───────────┴──────────┴───────────┴─────┴───────┴───────────┘
```

Our implementation:

```rust
pub struct SSTableEntry {
    pub key: InternalKey,    // user_key + timestamp
    pub value: Value,
    pub operation: Operation, // Put or Delete
}

impl SSTableEntry {
    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut buf = Vec::new();

        // Encode key length and data
        buf.write_u32::<LittleEndian>(self.key.user_key.len() as u32)?;
        buf.write_u32::<LittleEndian>(self.value.len() as u32)?;
        buf.write_u8(self.operation as u8)?;
        buf.write_u64::<LittleEndian>(self.key.timestamp)?;
        buf.extend_from_slice(&self.key.user_key);
        buf.extend_from_slice(&self.value);

        Ok(buf)
    }
}
```

#### Index Block Format

The index helps locate data blocks quickly:

```rust
pub struct IndexEntry {
    pub first_key: InternalKey,  // First key in the block
    pub offset: u64,             // Byte offset in file
    pub size: u32,               // Block size in bytes
}
```

#### Footer Format

The footer contains essential metadata:

```rust
pub struct Footer {
    pub index_offset: u64,    // Where index block starts
    pub index_size: u32,      // Size of index block
    pub version: u32,         // Format version
    pub magic: u64,           // 0x0123456789ABCDEF
}
```

### Key Design Decisions

#### 1. Why 4KB Blocks?

```rust
const TARGET_BLOCK_SIZE: usize = 4096; // 4KB
```

- **OS Page Size**: Most systems use 4KB pages, making this I/O efficient
- **Cache Friendly**: Fits well in CPU caches
- **Balance**: Large enough to amortize seek costs, small enough for fast scans

#### 2. Why Include Operation in SSTable?

Initially, we had `Operation` as part of `InternalKey`:

```rust
// Original design - problematic
pub struct InternalKey {
    pub user_key: Key,
    pub timestamp: Timestamp,
    pub operation: Operation,  // ❌ Mixed concerns
}
```

We refactored to separate concerns:

```rust
// Current design - clean separation
pub struct InternalKey {
    pub user_key: Key,
    pub timestamp: Timestamp,  // ✅ Pure key identity
}

pub struct SSTableEntry {
    pub key: InternalKey,
    pub value: Value,
    pub operation: Operation,  // ✅ Storage metadata
}
```

This makes the API cleaner:

```rust
// Before: Awkward API
reader.get(&InternalKey::new(key, ts, Operation::Put))?

// After: Natural API
reader.get(&key, timestamp)?
```

#### 3. Checksum Strategy

Each block includes a CRC32 checksum:

```rust
pub struct BlockHeader {
    pub block_size: u32,
    pub entry_count: u32,
    pub checksum: u32,  // CRC32 of block data
}
```

This enables:

- **Corruption detection** during reads
- **Selective retry** of corrupted blocks
- **Background verification** without impacting reads

## Binary Search Optimization

### The Problem

Our initial implementation used linear search:

```rust
// O(n) - Slow for large blocks!
for entry in block.entries {
    if entry.key.user_key == target_key
       && entry.key.timestamp <= target_timestamp {
        return Some(entry.value);
    }
}
```

### The Solution

We implemented binary search leveraging our sorted structure:

```rust
impl DataBlock {
    pub fn get(&self, key: &[u8], timestamp: Timestamp) -> Result<Option<Value>> {
        let target_key = InternalKey::new(key.to_vec(), timestamp);

        // Binary search by key
        match self.entries.binary_search_by(|entry| entry.key.cmp(&target_key)) {
            Ok(idx) => Ok(Some(self.entries[idx].value.clone())),
            Err(_) => Ok(None),
        }
    }
}
```

### Performance Impact

| Block Size   | Linear Search         | Binary Search        | Improvement |
| ------------ | --------------------- | -------------------- | ----------- |
| 100 entries  | 50 comparisons (avg)  | 7 comparisons (max)  | ~7x         |
| 1000 entries | 500 comparisons (avg) | 10 comparisons (max) | ~50x        |

## Two-Level Lookup

FerrisDB uses a two-level lookup strategy:

```rust
pub fn get(&self, key: &[u8], timestamp: Timestamp) -> Result<Option<Value>> {
    // Level 1: Binary search in index to find block
    let block_idx = self.find_block_for_key(key)?;

    // Level 2: Load block and binary search within it
    let block = self.read_block(block_idx)?;
    block.get(key, timestamp)
}
```

Total complexity: **O(log B + log E)** where:

- B = number of blocks
- E = entries per block

## Implementation Insights

### Writer Pattern

The writer uses a streaming approach:

```rust
impl SSTableWriter {
    pub fn add(&mut self, key: InternalKey, value: Value, op: Operation) -> Result<()> {
        self.current_block.push(SSTableEntry { key, value, operation: op });

        // Flush when block is full
        if self.current_block_size() >= TARGET_BLOCK_SIZE {
            self.flush_block()?;
        }
        Ok(())
    }

    fn flush_block(&mut self) -> Result<()> {
        // Write block with header and checksum
        let encoded = self.encode_current_block()?;
        self.file.write_all(&encoded)?;

        // Update index
        let first_key = self.current_block[0].key.clone();
        self.index.push(IndexEntry {
            first_key,
            offset: self.current_offset,
            size: encoded.len() as u32,
        });

        self.current_block.clear();
        Ok(())
    }
}
```

### Reader Pattern

The reader minimizes I/O through selective loading:

```rust
impl SSTableReader {
    pub fn new(path: PathBuf) -> Result<Self> {
        let mut file = File::open(&path)?;

        // Read footer (last 24 bytes)
        file.seek(SeekFrom::End(-24))?;
        let footer = Footer::decode(&mut file)?;

        // Read and cache index
        file.seek(SeekFrom::Start(footer.index_offset))?;
        let index = Self::read_index(&mut file, footer.index_size)?;

        Ok(Self { file, index, footer })
    }
}
```

## Lessons Learned

### 1. Separation of Concerns Matters

Moving `Operation` out of `InternalKey` made the code cleaner and more maintainable. Always question whether data belongs together.

### 2. Binary Search is Essential

For sorted data, binary search provides dramatic performance improvements. The implementation effort pays off quickly.

### 3. Block Size is a Key Tuning Parameter

4KB works well for our use case, but this should be configurable based on:

- Workload characteristics (key/value sizes)
- Storage medium (SSD vs HDD)
- Available memory for caching

### 4. Checksums are Non-Negotiable

Data corruption happens. Checksums let us detect it early and fail gracefully rather than returning bad data.

## What's Next?

With SSTables implemented, the next challenges are:

1. **Compaction**: Merging multiple SSTables to remove duplicates
2. **Bloom Filters**: Avoiding unnecessary disk reads
3. **Compression**: Reducing storage footprint
4. **Block Cache**: Keeping hot blocks in memory

## Try It Yourself

Check out the [full implementation](https://github.com/ferrisdb/ferrisdb/tree/main/ferrisdb-storage/src/sstable) and run the tests:

```bash
# Run SSTable tests
cargo test -p ferrisdb-storage sstable

# Benchmark read performance
cargo bench sstable_read
```

---

_This article is based on Day 2 of FerrisDB development. Follow our [blog](/blog/) for daily updates on building a database from scratch!_
