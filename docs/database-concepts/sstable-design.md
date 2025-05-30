---
layout: default
title: "SSTable Format Design"
parent: Database Concepts
nav_order: 3
permalink: /database-concepts/sstable-design/
---

How databases organize and access data on disk for optimal performance
{: .fs-6 .fw-300 }

📙 **Difficulty:** Intermediate • ⏱️ **Reading time:** 30 minutes

## Table of contents

{: .no_toc .text-delta }

<!-- prettier-ignore-start -->

1. TOC
{:toc}
<!-- prettier-ignore-end -->

---

## The Problem & Why It Matters

Imagine you're building a web application that needs to store millions of user profiles, product listings, or transaction records. Your in-memory cache (MemTable) is getting full, and you need to save this data to disk. But here's the challenge:

**Real-world problems CRUD developers face:**

- **Slow queries**: Finding one record among millions takes forever
- **Huge files**: Loading entire datasets into memory crashes your server
- **Disk costs**: Storing uncompressed data gets expensive fast
- **Recovery time**: Restarting your app takes hours to reload data

Traditional approaches have painful trade-offs:

1. **CSV/JSON files**: Easy to debug but terribly slow to search
2. **Binary dumps**: Fast to write but impossible to query without loading everything
3. **Regular databases**: Add complexity and another point of failure

SSTable (Sorted String Table) solves these problems elegantly. Think of it as a phone book for your data - organized, indexed, and easy to search without reading every page.

## Conceptual Overview

### The Core Idea

SSTables are like a well-organized library:

**Without SSTables** (pile of books):

- To find a book, check every single one
- Moving books around is a nightmare
- No way to know if a book exists without searching

**With SSTables** (organized library):

- Books sorted alphabetically on shelves (data blocks)
- Card catalog tells you which shelf (index)
- Quick reference list of all books (bloom filter)
- Information desk with library stats (footer)

### Visual Architecture

```text
┌─────────────────┐
│   Data Block 1  │  ← "A-C" authors (sorted books)
├─────────────────┤
│   Data Block 2  │  ← "D-F" authors
├─────────────────┤
│   Data Block 3  │  ← "G-M" authors
├─────────────────┤
│   Data Block 4  │  ← "N-Z" authors
├─────────────────┤
│   Index Block   │  ← Card catalog (which shelf has what)
├─────────────────┤
│  Bloom Filter   │  ← Quick "do we have this?" check
├─────────────────┤
│     Footer      │  ← Information desk (where everything is)
└─────────────────┘
```

**Key principles:**

1. **Immutability**: Once written, never changed (like printed books)
2. **Block-based**: Read only what you need, not the whole file
3. **Binary format**: Compact and efficient, not human-readable

## FerrisDB Implementation Deep Dive

### Core Data Structures

Let's examine how FerrisDB structures SSTable data:

```rust
// ferrisdb-storage/src/sstable/mod.rs:180-204
pub struct SSTableEntry {
    /// The internal key (user_key + timestamp)
    pub key: InternalKey,
    /// The value associated with this key version
    pub value: Value,
    /// The operation type (Put/Delete) for this entry
    pub operation: Operation,
}

pub struct InternalKey {
    pub user_key: Key,
    pub timestamp: Timestamp,
}

// ferrisdb-storage/src/sstable/mod.rs:207-228
pub struct IndexEntry {
    /// File offset of the data block
    pub block_offset: u64,
    /// First key in the data block
    pub first_key: Key,
}
```

**Key design decisions:**

1. **Separate key components**: User key and timestamp stored separately for efficient comparison
2. **Operation tracking**: Distinguishes between insertions and deletions
3. **Block indexing**: Each index entry points to a 4KB data block

### Implementation Details

#### Binary Format Details

The actual bytes on disk follow a precise format:

```text
Entry Format (within Data Block):
┌──────────┬─────────────┬───────────┬──────────────┬────────────┬──────────┐
│ Key Len  │ Value Len   │ Timestamp │  Operation   │    Key     │  Value   │
│(4 bytes) │ (4 bytes)   │ (8 bytes) │   (1 byte)   │ (variable) │(variable)│
└──────────┴─────────────┴───────────┴──────────────┴────────────┴──────────┘

Footer Format (always 40 bytes at end of file):
┌─────────────┬─────────────┬─────────────┬─────────────┬─────────────┐
│Index Offset │Index Length │Bloom Offset │Bloom Length │Magic Number │
│  (8 bytes)  │  (8 bytes)  │  (8 bytes)  │  (8 bytes)  │  (8 bytes)  │
└─────────────┴─────────────┴─────────────┴─────────────┴─────────────┘
```

**How it works:**

1. **Fixed-size headers**: Can calculate positions without scanning
2. **Little-endian encoding**: Consistent byte order across platforms
3. **Magic number**: Validates file integrity (detects corruption)

**Why this matters:**

- **Efficient seeking**: Jump directly to any block without reading others
- **Corruption detection**: Magic number and checksums catch disk errors
- **Platform independence**: Same format works everywhere

#### SSTable Writer Implementation

Creating an SSTable involves careful data organization:

```rust
// ferrisdb-storage/src/sstable/writer.rs:34-65 (simplified)
pub struct SSTableWriter {
    file: File,
    current_block: Vec<SSTableEntry>,
    index_entries: Vec<IndexEntry>,
    block_size: usize,
    entries_written: usize,
}

impl SSTableWriter {
    pub fn add(&mut self, key: InternalKey, value: Value, operation: Operation) -> Result<()> {
        // Check ordering (entries must be sorted)
        if let Some(last) = self.current_block.last() {
            if key <= last.key {
                return Err(Error::OutOfOrder);
            }
        }

        let entry = SSTableEntry::new(key, value, operation);

        // Check if block is full
        if self.current_block_size() + entry.serialized_size() > self.block_size {
            self.write_block()?;
        }

        self.current_block.push(entry);
        self.entries_written += 1;

        Ok(())
    }

    fn write_block(&mut self) -> Result<()> {
        let offset = self.file.seek(SeekFrom::Current(0))?;
        let first_key = self.current_block[0].key.user_key.clone();

        // Write all entries in the block
        for entry in &self.current_block {
            self.write_entry(entry)?;
        }

        // Add to index
        self.index_entries.push(IndexEntry::new(offset, first_key));

        // Clear for next block
        self.current_block.clear();

        Ok(())
    }
}
```

**Performance characteristics:**

- **Write throughput**: Sequential I/O only (fast on all storage types)
- **Memory usage**: Only one block (4KB) in memory at a time
- **No random writes**: Append-only design ideal for SSDs

#### Binary Search Within Blocks

The real performance magic happens during reads:

```rust
// ferrisdb-storage/src/sstable/reader.rs:150-180 (conceptual)
pub fn find_in_block(&self, block: &DataBlock, key: &Key, timestamp: Timestamp) -> Option<Value> {
    // Binary search for the key
    let entries = &block.entries;

    let idx = entries.binary_search_by(|entry| {
        match entry.key.user_key.cmp(key) {
            Ordering::Equal => {
                // Keys match, compare timestamps (newer first)
                timestamp.cmp(&entry.key.timestamp)
            }
            other => other,
        }
    });

    match idx {
        Ok(i) => {
            // Found exact match
            if entries[i].operation == Operation::Put {
                Some(entries[i].value.clone())
            } else {
                None // Deleted entry
            }
        }
        Err(_) => None, // Not found
    }
}
```

**Why binary search works so well:**

- **O(log n) lookups**: 100 entries needs only 7 comparisons max
- **Cache-friendly**: Sequential memory access within blocks
- **Predictable performance**: No worst-case degradation

## Performance Analysis

### Mathematical Analysis

**Search Performance:**

- **Block location**: O(log B) where B = number of blocks (using index)
- **Within block**: O(log E) where E = entries per block (~100)
- **Total**: O(log B + log E) = O(log N) where N = total entries

**Space Efficiency:**

- **Overhead per entry**: ~17 bytes (lengths, timestamp, operation)
- **Index overhead**: ~1% of data size (one entry per 4KB block)
- **Compression potential**: 50-80% reduction for text data

### Trade-off Analysis

**Advantages:**

- ✅ **Fast lookups**: Binary search in sorted data
- ✅ **Efficient disk usage**: Only read necessary blocks
- ✅ **Compression-friendly**: Sequential data compresses well
- ✅ **Crash recovery**: Immutable files with checksums

**Disadvantages:**

- ⚠️ **Write-once**: Can't update in place (need compaction)
- ⚠️ **Space amplification**: Multiple versions until compaction
- ⚠️ **Not human-readable**: Binary format requires tools
- ⚠️ **Fixed block size**: May waste space for small datasets

**When to use alternatives:**

- **Frequently updated data**: Use B-trees or hash tables
- **Small datasets**: Simple formats like JSON might suffice
- **Debugging needs**: Text formats easier to inspect

## Advanced Topics

### Bloom Filters for Existence Checks

Before reading a block, check if the key might exist:

```rust
// Bloom filter basics
pub struct BloomFilter {
    bits: Vec<bool>,
    hash_count: usize,
}

impl BloomFilter {
    pub fn might_contain(&self, key: &[u8]) -> bool {
        for i in 0..self.hash_count {
            let hash = self.hash_key(key, i);
            let bit_pos = hash % self.bits.len();

            if !self.bits[bit_pos] {
                return false; // Definitely not present
            }
        }
        true // Might be present (or false positive)
    }
}
```

**Bloom filter properties:**

- **Space**: ~10 bits per key for 1% false positive rate
- **Speed**: O(k) where k = number of hash functions
- **No false negatives**: If it says "no", the key definitely doesn't exist

### Compression Strategies

FerrisDB can compress blocks before writing:

```rust
// Block compression options
pub enum CompressionType {
    None,
    Snappy,  // Fast compression, moderate ratio
    LZ4,     // Very fast, good for real-time
    Zstd,    // Best ratio, slower
}

// Compression happens at block level
fn compress_block(data: &[u8], compression: CompressionType) -> Vec<u8> {
    match compression {
        CompressionType::LZ4 => lz4::compress(data),
        CompressionType::Snappy => snappy::compress(data),
        // ... etc
    }
}
```

**Compression trade-offs:**

- **LZ4**: 2-4x compression, minimal CPU overhead
- **Snappy**: Google's choice, balanced performance
- **Zstd**: 3-5x compression, higher CPU cost

## Hands-On Exploration

### Try It Yourself

**Exercise 1**: Analyze SSTable structure

```bash
# Create a sample SSTable
cargo run --example create_sstable -- --entries 1000

# Examine the binary structure
hexdump -C sample.sst | head -50

# Look for the magic number at the end
tail -c 40 sample.sst | hexdump -C
```

**Exercise 2**: Benchmark lookup performance

```rust
use std::time::Instant;

fn benchmark_sstable_lookups() {
    let reader = SSTableReader::open("sample.sst").unwrap();

    // Random lookups
    let start = Instant::now();
    for _ in 0..1000 {
        let key = format!("user:{}", rand::random::<u32>() % 1000);
        reader.get(key.as_bytes(), u64::MAX);
    }
    println!("Random lookups: {:?}/op", start.elapsed() / 1000);

    // Sequential scan
    let start = Instant::now();
    let mut count = 0;
    for entry in reader.iter() {
        count += 1;
    }
    println!("Sequential scan: {} entries in {:?}", count, start.elapsed());
}
```

### Debugging & Observability

**Key metrics to watch:**

- **Block cache hit rate**: How often we avoid disk reads
- **Average block fill**: Efficiency of space usage
- **Bloom filter effectiveness**: False positive rate

**Debugging techniques:**

- **SSTable inspection tool**: `cargo run --bin sstable-dump`
- **Block analysis**: Check compression ratios and fill rates
- **Performance profiling**: Identify slow lookups

## Real-World Context

### Industry Comparison

**How other databases implement persistent storage:**

- **LevelDB/RocksDB**: Similar SSTable format with levels
- **Cassandra**: SSTables with specialized compaction
- **HBase**: StoreFiles (SSTable variant) with HDFS
- **PostgreSQL**: Heap files with B-tree indexes (different approach)

### Historical Evolution

**Timeline:**

- **2006**: Google Bigtable paper introduces SSTable concept
- **2011**: LevelDB open-sources SSTable implementation
- **2012**: RocksDB adds optimizations for SSDs
- **Today**: Cloud-native adaptations for object storage

## Common Pitfalls & Best Practices

### Implementation Pitfalls

1. **Forgetting to sort entries**:

   - **Problem**: Binary search fails on unsorted data
   - **Solution**: Enforce ordering in writer

2. **Block size selection**:

   - **Problem**: Too small = many seeks, too large = wasted reads
   - **Solution**: 4-16KB sweet spot for most workloads

3. **Missing checksums**:
   - **Problem**: Silent data corruption
   - **Solution**: CRC32 per block minimum

### Production Considerations

**Operational concerns:**

- **File handle limits**: Many SSTables = many open files
- **Compaction scheduling**: Balance read performance vs I/O load
- **Backup strategies**: Immutable files make incremental backups easy
- **Monitoring**: Track SSTable count, sizes, and age distribution

## Summary & Key Takeaways

### Core Concepts Learned

1. **Immutable files enable efficient reads**: No locking, perfect for caching
2. **Binary format with blocks**: Read only what you need
3. **Sorted data enables binary search**: O(log n) lookups in large files

### When to Apply This Knowledge

- **Use SSTables when**: Write-once, read-many workloads (logs, time-series)
- **Consider alternatives when**: Need in-place updates or real-time queries
- **Implementation complexity**: Moderate - careful binary format handling required

## Further Reading & References

### Related FerrisDB Articles

- [LSM-Trees: The Secret Behind Modern Database Performance](/database-concepts/lsm-trees/): How SSTables fit in the architecture
- [Understanding WAL and Crash Recovery](/database-concepts/wal-crash-recovery/): What happens before SSTable

### Academic Papers

- "Bigtable: A Distributed Storage System" (Chang et al., 2006): Original SSTable design
- "Log-Structured Merge-Tree" (O'Neil et al., 1996): Theoretical foundation

### Industry Resources

- [RocksDB SSTable Format](https://github.com/facebook/rocksdb/wiki/Rocksdb-BlockBasedTable-Format): Production implementation
- [LevelDB Implementation Notes](https://github.com/google/leveldb/blob/master/doc/impl.md): Google's design doc

### FerrisDB Code Exploration

- **SSTable format**: `ferrisdb-storage/src/sstable/mod.rs` - Data structures
- **Writer implementation**: `ferrisdb-storage/src/sstable/writer.rs` - Creating SSTables
- **Reader implementation**: `ferrisdb-storage/src/sstable/reader.rs` - Querying SSTables
- **Integration tests**: `ferrisdb-storage/src/sstable/tests.rs` - Usage examples

---

## About This Series

This article is part of FerrisDB's technical deep dive series. Each article provides comprehensive coverage of database internals through practical implementation:

- ✅ **Real implementation details** from FerrisDB source code
- ✅ **Mathematical analysis** with concrete complexity bounds
- ✅ **Practical exercises** for hands-on learning
- ✅ **Industry context** and alternative approaches

**Target audience**: CRUD developers who want to understand database systems deeply.

[Browse all database concepts](/database-concepts/) | [Architecture overview](/architecture/) | [Contribute on GitHub]({{ site.project.repo_url }})

---

_Last updated: May 29, 2025_
_Estimated reading time: 20 minutes_
_Difficulty: Intermediate_
