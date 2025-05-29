---
layout: default
title: "WAL and Crash Recovery"
parent: Technical Deep Dives
nav_order: 1
permalink: /deep-dive/wal-crash-recovery/
---

How Write-Ahead Logs ensure data durability and enable crash recovery in database systems
{: .fs-6 .fw-300 }

📗 **Difficulty:** Beginner • ⏱️ **Reading time:** 15 minutes

## Table of contents

{: .no_toc .text-delta }

1. TOC
{:toc}

---

## The Problem & Why It Matters

Imagine you're running an e-commerce website. A customer completes their order, sees the "Order Successful!" message, and then... your server crashes. When it restarts, is their order still there? Or did it vanish into the digital void?

This is the fundamental durability problem that every database must solve. Without proper crash recovery, you risk:

{: .warning }

> **Real-world nightmares for CRUD developers:**
>
> - **Lost transactions**: Customer paid but order disappeared
> - **Inconsistent data**: Half-completed updates that corrupt your database
> - **Angry users**: "I know I updated my profile, where did my changes go?"
> - **Business impact**: Lost sales, refunds, and damaged reputation

The problem is that computers use two types of storage:

1. **RAM (Memory)**: Super fast but disappears when power is lost
2. **Disk (Storage)**: Slower but survives power loss

If your database only writes to memory for speed, all data is lost on crash. If it writes to disk for every operation, it becomes painfully slow. Write-Ahead Logging (WAL) solves this dilemma elegantly.

## Conceptual Overview

### The Core Idea

Write-Ahead Logging follows a simple principle:

{: .highlight }

> **WAL Core Principle**
>
> "Write changes to a log file BEFORE updating the actual data"

**Think of it like a restaurant's order system:**

1. **Taking the order** (WAL write): Waiter writes order on paper first
2. **Kitchen preparation** (Memory update): Cook starts preparing the meal
3. **Order tracking** (Recovery): If cook forgets, the written order still exists

Even if the kitchen catches fire (system crash), the written orders survive, and a new cook can continue where the previous one left off.

### Visual Architecture

```text
User Request → WAL (Disk) → MemTable (RAM) → SSTable (Disk)
      ↓           ↓              ↓
   "Success"   Durability    Fast Access    Long-term Storage
```

{: .important }

> **Key WAL Principles**
>
> 1. **Write-ahead**: Log first, update data structures second
> 2. **Sequential writes**: Appending to log is fast (like writing in a journal)
> 3. **Recovery guarantee**: Can rebuild state from log after crash

## FerrisDB Implementation Deep Dive

### Core Data Structures

Let's examine how FerrisDB implements WAL:

```rust
// ferrisdb-storage/src/wal/log_entry.rs:15-29
pub struct WALEntry {
    /// Timestamp when this operation occurred
    pub timestamp: Timestamp,
    /// Type of operation (Put or Delete)
    pub operation: Operation,
    /// The key being operated on
    pub key: Key,
    /// The value (empty for Delete operations)
    pub value: Value,
}

impl WALEntry {
    pub fn new_put(key: Key, value: Value, timestamp: Timestamp) -> Self {
        Self {
            timestamp,
            operation: Operation::Put,
            key,
            value,
        }
    }
}
```

**Key design decisions:**

1. **Self-contained entries**: Each entry has all information needed to replay the operation
2. **Timestamp tracking**: Essential for maintaining operation order during recovery
3. **Operation types**: Distinguish between insertions and deletions

### Implementation Details

#### Binary Format for Efficiency

FerrisDB stores WAL entries in a compact binary format:

```text
+------------+------------+------------+-------+------------+
| Length(4B) | CRC32(4B)  | Time(8B)   | Op(1B)| Key Len(4B)|
+------------+------------+------------+-------+------------+
| Key(var)   | Val Len(4B)| Value(var) |
+------------+------------+------------+
```

**How it works:**

1. **Length prefix**: Allows skipping corrupted entries during recovery
2. **CRC32 checksum**: Detects corruption from partial writes or disk errors
3. **Fixed-size headers**: Enables efficient parsing without scanning entire entry
4. **Variable-length data**: Space-efficient for different key/value sizes

**Why this matters:**

- **Corruption detection**: CRC32 catches 99.99% of random bit flips
- **Partial write handling**: Length prefix lets us skip incomplete entries
- **Fast recovery**: Can quickly scan through log without parsing every byte

#### The Write Process

Here's how FerrisDB ensures durability:

```rust
// ferrisdb-storage/src/wal/writer.rs:50-85 (conceptual)
pub fn write_entry(&mut self, entry: &WALEntry) -> Result<()> {
    // 1. Encode entry to binary format
    let encoded = entry.encode()?;
    let length = encoded.len() as u32;
    let crc = crc32::checksum(&encoded);

    // 2. Write to OS buffer
    self.file.write_all(&length.to_le_bytes())?;
    self.file.write_all(&crc.to_le_bytes())?;
    self.file.write_all(&encoded)?;

    // 3. Force to disk (this is the critical step!)
    self.file.sync_all()?;

    // 4. Update position for next write
    self.position += HEADER_SIZE + encoded.len();

    Ok(())
}
```

**Performance characteristics:**

- **Time complexity**: O(1) - just appending to end of file
- **I/O pattern**: Sequential writes (100x faster than random writes on HDD)
- **Sync overhead**: `sync_all()` ensures durability but adds latency

## Performance Analysis

### Mathematical Analysis

**Write performance comparison:**

- **Without WAL**: Random I/O to update data = O(log n) disk seeks
- **With WAL**: Sequential append = O(1) disk operation
- **Recovery time**: O(n) where n = number of operations since last checkpoint

**Durability vs Performance trade-off:**

- **Full sync mode**: Every operation calls `fsync()` - slowest but safest
- **Periodic sync**: Batch multiple operations - faster but small data loss window
- **No sync**: OS handles flushing - fastest but risky

### Trade-off Analysis

**Advantages:**

- ✅ **Guaranteed durability**: Data survives crashes once written to WAL
- ✅ **Fast writes**: Sequential I/O is much faster than random I/O
- ✅ **Simple recovery**: Just replay the log from last checkpoint
- ✅ **Corruption detection**: CRC checksums catch disk errors

**Disadvantages:**

- ⚠️ **Write amplification**: Data written twice (WAL + actual storage)
- ⚠️ **Recovery time**: Large logs take time to replay
- ⚠️ **Disk space**: Need space for both WAL and data files
- ⚠️ **Sync overhead**: `fsync()` calls can limit throughput

**When to use alternatives:**

- **In-memory only**: If data loss is acceptable (caches, sessions)
- **Batch processing**: Can reconstruct from source data
- **Read-only systems**: No writes means no need for WAL

## Advanced Topics

### WAL Truncation and Checkpointing

WAL files can't grow forever. FerrisDB implements truncation:

```rust
// ferrisdb-storage/src/wal/writer.rs (conceptual)
pub fn checkpoint(&mut self) -> Result<()> {
    // 1. Ensure all MemTable data is flushed to SSTables
    self.storage_engine.flush_all_memtables()?;

    // 2. Record checkpoint position
    let checkpoint_pos = self.current_position;

    // 3. Truncate WAL up to checkpoint
    self.truncate_before(checkpoint_pos)?;

    Ok(())
}
```

**Checkpoint strategies:**

- **Size-based**: Checkpoint when WAL reaches certain size
- **Time-based**: Checkpoint every N minutes
- **Operation-based**: Checkpoint every N operations

### Group Commit Optimization

For better performance, FerrisDB can batch multiple operations:

```rust
// Conceptual implementation
pub fn group_commit(&mut self, entries: Vec<WALEntry>) -> Result<()> {
    // Write all entries to OS buffer
    for entry in entries {
        self.write_to_buffer(entry)?;
    }

    // Single fsync for entire batch
    self.file.sync_all()?;

    Ok(())
}
```

This reduces the number of expensive `fsync()` calls while maintaining durability.

## Hands-On Exploration

### Try It Yourself

**Exercise 1**: Understanding fsync impact

```rust
// Compare performance with and without fsync
use std::fs::OpenOptions;
use std::io::Write;
use std::time::Instant;

fn benchmark_wal_writes() {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open("test.wal")
        .unwrap();

    // Test with fsync
    let start = Instant::now();
    for i in 0..100 {
        write!(file, "Entry {}\n", i).unwrap();
        file.sync_all().unwrap(); // Force to disk
    }
    println!("With fsync: {:?}", start.elapsed());

    // Test without fsync
    let start = Instant::now();
    for i in 0..100 {
        write!(file, "Entry {}\n", i).unwrap();
        // No sync - let OS handle it
    }
    println!("Without fsync: {:?}", start.elapsed());
}
```

**Exercise 2**: Crash recovery simulation

```bash
# Start a write workload
cargo run --example wal_writer -- --entries 1000 &

# Simulate crash (kill process)
sleep 2 && kill -9 $!

# Run recovery
cargo run --example wal_recovery -- --recover-from test.wal
```

### Debugging & Observability

**Key metrics to watch:**

- **WAL size**: Monitor growth rate and truncation frequency
- **Sync latency**: Time spent in `fsync()` calls
- **Recovery duration**: Time to replay WAL after crash

**Debugging techniques:**

- **WAL inspection**: `cargo run --bin wal-dump` to examine entries
- **Corruption detection**: Look for CRC mismatches in logs
- **Performance profiling**: Measure time spent in WAL operations

## Real-World Context

### Industry Comparison

**How other databases handle WAL:**

- **PostgreSQL**: Uses WAL with configurable sync levels
- **MySQL (InnoDB)**: Redo log with group commit optimization
- **SQLite**: Rollback journal or WAL mode options
- **Redis**: Optional AOF (Append Only File) similar to WAL

### Historical Evolution

**Timeline:**

- **1992**: ARIES paper establishes WAL principles
- **2004**: SQLite adds WAL mode for better concurrency
- **2010**: NoSQL databases adopt WAL for durability
- **Today**: NVMe and persistent memory changing WAL design

## Common Pitfalls & Best Practices

### Implementation Pitfalls

1. **Forgetting to sync**:

   - **Problem**: Data in OS buffer not on disk
   - **Solution**: Always call `fsync()` for durability

2. **Corrupted WAL handling**:

   - **Problem**: Single bit flip makes entry unreadable
   - **Solution**: CRC checksums and length prefixes

3. **Unbounded growth**:
   - **Problem**: WAL fills up disk
   - **Solution**: Regular checkpointing and truncation

### Production Considerations

**Operational concerns:**

- **Disk monitoring**: WAL can fill disk quickly under high load
- **Sync tuning**: Balance durability vs performance for your use case
- **Backup strategy**: Include WAL in backups for point-in-time recovery
- **Separate disks**: Put WAL on different disk than data for better I/O

## Summary & Key Takeaways

### Core Concepts Learned

1. **Write-ahead principle ensures durability**: Log before modifying data structures
2. **Sequential writes are fast**: Appending to log much faster than random updates
3. **Recovery is straightforward**: Just replay the log from last checkpoint

### When to Apply This Knowledge

- **Use WAL when**: Data durability is critical (financial transactions, user data)
- **Consider alternatives when**: Data can be regenerated or loss is acceptable
- **Implementation complexity**: Moderate - requires careful handling of I/O and recovery

## Further Reading & References

### Related FerrisDB Articles

- [LSM-Trees: The Secret Behind Modern Database Performance](/deep-dive/lsm-trees/): How WAL fits into the larger storage architecture
- [SSTable Format Design](/deep-dive/sstable-design/): Where flushed WAL data eventually lands

### Academic Papers

- "ARIES: A Transaction Recovery Method" (Mohan et al., 1992): Foundational WAL concepts
- "aLSM: Redesigning LSMs for Nonvolatile Memory" (Eisenman et al., 2018): Modern WAL adaptations

### Industry Resources

- [PostgreSQL WAL Documentation](https://www.postgresql.org/docs/current/wal-intro.html): Production WAL implementation
- [etcd WAL Package](https://github.com/etcd-io/etcd/tree/main/server/wal): Go implementation example

### FerrisDB Code Exploration

- **WAL Writer**: `ferrisdb-storage/src/wal/writer.rs` - Core write logic
- **WAL Reader**: `ferrisdb-storage/src/wal/reader.rs` - Recovery implementation
- **Binary Format**: `ferrisdb-storage/src/wal/log_entry.rs` - Entry encoding/decoding
- **Tests**: `ferrisdb-storage/src/wal/` - Test cases showing usage

---

## About This Series

This article is part of FerrisDB's technical deep dive series. Each article provides comprehensive coverage of database internals through practical implementation:

- ✅ **Real implementation details** from FerrisDB source code
- ✅ **Mathematical analysis** with concrete complexity bounds
- ✅ **Practical exercises** for hands-on learning
- ✅ **Industry context** and alternative approaches

**Target audience**: CRUD developers who want to understand database systems deeply.

[Browse all deep dives](/deep-dive/) | [Architecture overview](/architecture/) | [Contribute on GitHub]({{ site.project.repo_url }})

---

_Last updated: May 29, 2025_
_Estimated reading time: 15 minutes_
_Difficulty: Beginner_
