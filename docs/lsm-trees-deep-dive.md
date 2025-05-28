---
layout: page
title: "LSM-Trees: The Secret Behind Modern Database Performance"
subtitle: "Understanding Log-Structured Merge Trees and how they power high-performance storage engines"
permalink: /deep-dive/lsm-trees/
---

Have you ever wondered how databases like Cassandra, LevelDB, and RocksDB can handle millions of writes per second? The secret is **LSM-Trees** (Log-Structured Merge Trees) - a data structure that revolutionized database storage engines.

This deep-dive explores LSM-Trees through FerrisDB's implementation and explains why they're perfect for modern workloads.

## The Write Problem in Traditional Databases

Traditional B-tree based databases face a fundamental challenge with writes:

```
B-Tree writes are expensive because:
1. Random disk access to find the right page
2. Read entire page into memory
3. Modify the page
4. Write entire page back to disk
5. Update index pages (more random I/O)
```

For write-heavy workloads, this becomes a bottleneck. LSM-Trees solve this with a completely different approach.

## LSM-Tree Philosophy: Make Writes Fast

The core insight of LSM-Trees is:

> **"Instead of updating data in place, append all changes and merge them later"**

This transforms expensive random writes into fast sequential writes.

### FerrisDB's LSM-Tree Architecture

```text
┌─────────────┐    ┌──────────────┐    ┌─────────────┐
│   WAL       │    │  MemTable    │    │  SSTables   │
│ (Sequential │ -> │ (In-Memory   │ -> │ (Immutable  │
│  Write Log) │    │  Skip List)  │    │  Sorted     │
│             │    │              │    │  Files)     │
└─────────────┘    └──────────────┘    └─────────────┘
      ^                    ^                    ^
   Durability         Fast Writes         Fast Reads
```

Let's dive into each component:

## Component 1: MemTable - Fast In-Memory Writes

The MemTable is where all writes initially go:

```rust
use crossbeam::epoch::{self, Atomic, Guard, Owned, Shared};
use std::sync::atomic::{AtomicUsize, Ordering};

/// Concurrent skip list for the MemTable
pub struct SkipList {
    head: Atomic<Node>,
    max_level: AtomicUsize,
}

struct Node {
    key: Key,
    value: Value,
    timestamp: Timestamp,
    next: Vec<Atomic<Node>>,
    level: usize,
}
```

**Why a Skip List?**

- **O(log n) inserts/lookups** - Nearly as fast as a balanced tree
- **Lock-free implementation** - Multiple threads can write concurrently
- **Ordered iteration** - Essential for range queries and SSTable generation
- **Memory efficient** - No tree rebalancing overhead

### MemTable Operations

```rust
impl SkipList {
    pub fn put(&self, key: Key, value: Value, timestamp: Timestamp) -> Result<()> {
        let guard = &epoch::pin();

        // Find insertion point
        let (preds, succs) = self.find_position(&key, guard);

        // Create new node
        let level = self.random_level();
        let new_node = Owned::new(Node {
            key,
            value,
            timestamp,
            next: vec![Atomic::null(); level + 1],
            level,
        });

        // Link node into skip list (lock-free)
        self.link_node(new_node, preds, succs);

        Ok(())
    }

    pub fn get(&self, key: &Key, timestamp: Timestamp) -> Result<Option<Value>> {
        let guard = &epoch::pin();
        let mut current = self.head.load(Ordering::Acquire, guard);

        // Traverse skip list levels (top to bottom)
        for level in (0..=self.max_level.load(Ordering::Relaxed)).rev() {
            while let Some(node) = unsafe { current.as_ref() } {
                let next = node.next[level].load(Ordering::Acquire, guard);

                if let Some(next_node) = unsafe { next.as_ref() } {
                    match next_node.key.cmp(key) {
                        Ordering::Less => current = next,
                        Ordering::Equal => {
                            // Found key, check timestamp for MVCC
                            if next_node.timestamp <= timestamp {
                                return Ok(Some(next_node.value.clone()));
                            }
                        }
                        Ordering::Greater => break,
                    }
                } else {
                    break;
                }
            }
        }

        Ok(None)
    }
}
```

**MVCC Support**: The skip list stores multiple versions of the same key, ordered by timestamp. This enables:

- **Snapshot isolation**: Each transaction sees a consistent view
- **Non-blocking reads**: Readers don't block writers
- **Point-in-time queries**: "What was the value at timestamp X?"

## Component 2: SSTables - Immutable Sorted Files

When the MemTable gets full, it's flushed to an SSTable (Sorted String Table):

```rust
/// SSTable file format
pub struct SSTable {
    /// Metadata about the SSTable
    metadata: SSTableMetadata,
    /// Bloom filter for fast negative lookups
    bloom_filter: BloomFilter,
    /// Index blocks for binary search
    index_blocks: Vec<IndexBlock>,
    /// Data blocks containing key-value pairs
    data_blocks: Vec<DataBlock>,
}

#[derive(Debug)]
pub struct SSTableMetadata {
    /// Smallest key in this SSTable
    pub min_key: Key,
    /// Largest key in this SSTable
    pub max_key: Key,
    /// Number of key-value pairs
    pub num_entries: u64,
    /// File size in bytes
    pub file_size: u64,
    /// Creation timestamp
    pub created_at: Timestamp,
}
```

### SSTable File Layout

```text
┌─────────────────────────────────────────────────────┐
│                   SSTable File                      │
├─────────────────────────────────────────────────────┤
│ Data Block 0    │ Data Block 1    │ Data Block N    │
│ [key1,val1,ts1] │ [key4,val4,ts4] │ [keyN,valN,tsN] │
│ [key2,val2,ts2] │ [key5,val5,ts5] │                 │
│ [key3,val3,ts3] │ [key6,val6,ts6] │                 │
├─────────────────────────────────────────────────────┤
│ Index Block 0   │ Index Block 1   │ Index Block N   │
│ [key1 -> offset]│ [key4 -> offset]│ [keyN -> offset]│
├─────────────────────────────────────────────────────┤
│                 Bloom Filter                        │
│ [bit array for fast negative lookups]               │
├─────────────────────────────────────────────────────┤
│                   Metadata                          │
│ [min_key, max_key, num_entries, file_size]          │
└─────────────────────────────────────────────────────┘
```

### SSTable Writer Implementation

```rust
pub struct SSTableWriter {
    file: File,
    data_blocks: Vec<DataBlock>,
    index_blocks: Vec<IndexBlock>,
    bloom_filter: BloomFilter,
    current_block: DataBlock,
    block_size: usize,
}

impl SSTableWriter {
    pub fn add(&mut self, key: Key, value: Value, timestamp: Timestamp) -> Result<()> {
        // Add to bloom filter
        self.bloom_filter.insert(&key);

        // Check if current block is full
        if self.current_block.size() > self.block_size {
            self.finish_block()?;
        }

        // Add entry to current block
        self.current_block.add(key, value, timestamp);

        Ok(())
    }

    fn finish_block(&mut self) -> Result<()> {
        if self.current_block.is_empty() {
            return Ok(());
        }

        // Write data block to file
        let offset = self.file.seek(SeekFrom::Current(0))?;
        self.current_block.write_to(&mut self.file)?;

        // Create index entry pointing to this block
        let index_entry = IndexBlock {
            first_key: self.current_block.first_key().clone(),
            offset,
            size: self.current_block.size(),
        };
        self.index_blocks.push(index_entry);

        // Start new block
        self.current_block = DataBlock::new();

        Ok(())
    }

    pub fn finalize(mut self) -> Result<SSTable> {
        // Finish last block
        self.finish_block()?;

        // Write index blocks
        let index_offset = self.file.seek(SeekFrom::Current(0))?;
        for index_block in &self.index_blocks {
            index_block.write_to(&mut self.file)?;
        }

        // Write bloom filter
        let bloom_offset = self.file.seek(SeekFrom::Current(0))?;
        self.bloom_filter.write_to(&mut self.file)?;

        // Write metadata
        let metadata = SSTableMetadata {
            min_key: self.index_blocks.first().unwrap().first_key.clone(),
            max_key: self.index_blocks.last().unwrap().first_key.clone(),
            num_entries: self.count_entries(),
            file_size: self.file.seek(SeekFrom::Current(0))?,
            created_at: Timestamp::now(),
        };
        metadata.write_to(&mut self.file)?;

        self.file.sync_all()?;

        Ok(SSTable {
            metadata,
            bloom_filter: self.bloom_filter,
            index_blocks: self.index_blocks,
            data_blocks: self.data_blocks,
        })
    }
}
```

## Component 3: Compaction - Managing Multiple SSTables

Over time, you accumulate many SSTables. Compaction merges them to:

- **Remove deleted data** (tombstones)
- **Merge duplicate keys** (keeping latest version)
- **Improve read performance** (fewer files to check)

### Compaction Strategies

**1. Size-Tiered Compaction**

```rust
pub fn size_tiered_compaction(&mut self) -> Result<()> {
    // Group SSTables by similar size
    let mut size_tiers: BTreeMap<u64, Vec<SSTable>> = BTreeMap::new();

    for sstable in &self.sstables {
        let size_tier = size_tier_for_size(sstable.metadata.file_size);
        size_tiers.entry(size_tier).or_default().push(sstable.clone());
    }

    // Compact tiers with too many SSTables
    for (tier_size, sstables) in size_tiers {
        if sstables.len() >= self.config.max_sstables_per_tier {
            self.compact_sstables(sstables)?;
        }
    }

    Ok(())
}
```

**2. Leveled Compaction (more sophisticated)**

```rust
pub fn leveled_compaction(&mut self) -> Result<()> {
    // Level 0: Direct MemTable flushes (may overlap)
    // Level 1: 10MB total, non-overlapping
    // Level 2: 100MB total, non-overlapping
    // Level 3: 1GB total, non-overlapping

    for level in 0..self.config.max_levels {
        let level_size = self.calculate_level_size(level);
        let max_level_size = self.config.max_size_for_level(level);

        if level_size > max_level_size {
            self.compact_level(level)?;
        }
    }

    Ok(())
}
```

### Compaction Implementation

```rust
pub fn compact_sstables(&mut self, sstables: Vec<SSTable>) -> Result<SSTable> {
    // Create iterators for each SSTable
    let mut iterators: Vec<SSTableIterator> = sstables
        .into_iter()
        .map(|sst| sst.iterator())
        .collect();

    // Merge iterators using a priority queue (min-heap)
    let mut merge_iter = MergeIterator::new(iterators);

    // Create new SSTable writer
    let mut writer = SSTableWriter::new(&self.next_sstable_path())?;

    let mut last_key: Option<Key> = None;

    while let Some((key, value, timestamp)) = merge_iter.next() {
        // Skip duplicate keys (keep latest timestamp)
        if let Some(ref last) = last_key {
            if *last == key {
                continue; // Skip older version
            }
        }

        // Skip tombstones if they're old enough
        if value.is_delete() && self.can_drop_tombstone(&key, timestamp) {
            continue;
        }

        // Add to new SSTable
        writer.add(key.clone(), value, timestamp)?;
        last_key = Some(key);
    }

    // Finalize new SSTable
    let new_sstable = writer.finalize()?;

    // Remove old SSTables
    for old_sstable in &old_sstables {
        self.delete_sstable(old_sstable)?;
    }

    // Add new SSTable
    self.sstables.push(new_sstable.clone());

    Ok(new_sstable)
}
```

## Read Path: Finding Data Efficiently

Reading from an LSM-Tree requires checking multiple places:

```rust
pub fn get(&self, key: &Key, timestamp: Timestamp) -> Result<Option<Value>> {
    // 1. Check MemTable first (most recent data)
    if let Some(value) = self.memtable.get(key, timestamp)? {
        return Ok(Some(value));
    }

    // 2. Check SSTables from newest to oldest
    for sstable in self.sstables.iter().rev() {
        // Quick range check
        if key < &sstable.metadata.min_key || key > &sstable.metadata.max_key {
            continue;
        }

        // Bloom filter check (fast negative lookup)
        if !sstable.bloom_filter.might_contain(key) {
            continue; // Definitely not in this SSTable
        }

        // Binary search in index blocks
        if let Some(block_idx) = sstable.find_block_for_key(key) {
            let data_block = sstable.read_data_block(block_idx)?;

            if let Some(value) = data_block.get(key, timestamp)? {
                return Ok(Some(value));
            }
        }
    }

    Ok(None) // Key not found
}
```

## Performance Characteristics

### Write Performance: Excellent ✅

```
Sequential writes to MemTable: ~500,000 ops/sec
WAL append (sequential): ~50,000 IOPS on SSD
Batched commits: Even higher throughput
```

LSM-Trees excel at writes because:

- **MemTable writes are pure in-memory operations**
- **WAL writes are sequential** (much faster than random)
- **No expensive B-tree rebalancing**

### Read Performance: Good with Optimizations ⚡

```
Hot data (in MemTable): ~200,000 ops/sec
Cold data (in SSTables): ~10,000 ops/sec (with bloom filters)
Range scans: Excellent (sorted data)
```

Read optimizations:

- **Bloom filters** reduce unnecessary disk I/O
- **Block-level compression** improves I/O efficiency
- **Caching** keeps hot data in memory
- **Compaction** reduces the number of files to check

## Real-World Trade-offs

### When LSM-Trees Excel 🚀

- **Write-heavy workloads**: Logs, time-series data, analytics
- **Large datasets**: Multi-TB databases with infrequent updates
- **Append-mostly data**: Event streams, audit logs
- **Cloud storage**: Where sequential I/O is much cheaper

### When LSM-Trees Struggle ⚠️

- **Read-heavy workloads** with random access patterns
- **Small datasets** that fit entirely in memory
- **Heavy update workloads** (lots of overwrites)
- **Strong consistency requirements** across multiple keys

## Optimizations and Variants

### Bloom Filters for Faster Negative Lookups

```rust
impl BloomFilter {
    pub fn might_contain(&self, key: &Key) -> bool {
        let hash1 = self.hash1(key);
        let hash2 = self.hash2(key);

        for i in 0..self.num_hash_functions {
            let bit_pos = (hash1 + i * hash2) % self.bit_array.len();
            if !self.bit_array[bit_pos] {
                return false; // Definitely not present
            }
        }

        true // Might be present (could be false positive)
    }
}
```

### Block Compression

```rust
impl DataBlock {
    pub fn compress(&mut self) -> Result<()> {
        let compressed = lz4::compress(&self.data)?;
        if compressed.len() < self.data.len() {
            self.data = compressed;
            self.compression = CompressionType::LZ4;
        }
        Ok(())
    }
}
```

### Partitioned Indexes

```rust
// Split large index blocks for better cache performance
pub struct PartitionedIndex {
    partitions: Vec<IndexPartition>,
    partition_index: Vec<Key>, // First key of each partition
}
```

## Conclusion: Why LSM-Trees Matter

LSM-Trees represent a fundamental shift in database design:

**Traditional approach**: Update data in place (good for reads, bad for writes)
**LSM approach**: Append changes and merge later (excellent for writes, optimized reads)

This makes LSM-Trees perfect for:

- **Modern applications** with write-heavy workloads
- **Big data systems** that need to ingest massive amounts of data
- **Cloud-native databases** where sequential I/O is preferred
- **Time-series databases** where data is mostly appended

Understanding LSM-Trees helps you:

- **Choose the right database** for your workload
- **Optimize performance** by understanding the underlying data structures
- **Design better systems** by applying these principles

---

## Related Deep Dives

- [Understanding WAL and Crash Recovery]({{ '/deep-dive/wal-crash-recovery/' | relative_url }})
- [MVCC and Transaction Isolation]({{ '/deep-dive/mvcc/' | relative_url }}) _(coming soon)_
- [Distributed Consensus with Raft]({{ '/deep-dive/raft/' | relative_url }}) _(coming soon)_

## Further Reading

- [Architecture Overview]({{ '/architecture/' | relative_url }})
- [Storage Engine Design]({{ '/storage-engine/' | relative_url }})
- [Future Architecture Explorations]({{ '/future-architecture/' | relative_url }})
