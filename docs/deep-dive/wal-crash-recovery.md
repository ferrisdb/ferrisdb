---
layout: page
title: "Understanding WAL and Crash Recovery"
subtitle: "How Write-Ahead Logs ensure data durability and enable crash recovery in database systems"
permalink: /deep-dive/wal-crash-recovery/
---

Write-Ahead Logs (WAL) are one of the most fundamental concepts in database systems, yet they're often mysterious to developers. This deep-dive explores how WAL enables crash recovery in FerrisDB and why it's essential for data durability.

## The Fundamental Problem

Imagine you're building a database and a user performs this operation:

```rust
db.put("user:123", "Alice")?;
// User receives success response
```

But then - **crash!** 💥 The power goes out. When the system restarts, is Alice's data still there?

Without proper crash recovery, the answer might be "maybe" - and that's unacceptable for a database system.

## The WAL Solution: Write-Ahead Logging

The core principle of WAL is deceptively simple:

> **"Write changes to the log BEFORE writing to data structures"**

This ensures that even if the system crashes, we have a complete record of what was supposed to happen.

### FerrisDB's WAL Implementation

Let's look at how FerrisDB implements this principle:

```rust
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
```

Each WAL entry is a complete, self-contained record of a database operation.

### Binary Format for Efficiency

FerrisDB stores WAL entries in a compact binary format:

```text
+------------+------------+------------+-------+------------+
| Length(4B) | CRC32(4B)  | Time(8B)   | Op(1B)| Key Len(4B)|
+------------+------------+------------+-------+------------+
| Key(var)   | Val Len(4B)| Value(var) |
+------------+------------+------------+
```

**Why this format?**

- **Length prefix**: Enables skipping corrupted entries
- **CRC32 checksum**: Detects data corruption
- **Timestamp**: Essential for MVCC and operation ordering
- **Variable-length fields**: Space-efficient for different key/value sizes

## The Write Process: Ensuring Durability

Here's what happens when you write data to FerrisDB:

```rust
pub fn put(&mut self, key: Key, value: Value) -> Result<()> {
    let timestamp = self.clock.now();

    // 1. Create WAL entry
    let entry = WALEntry {
        timestamp,
        operation: Operation::Put,
        key: key.clone(),
        value: value.clone(),
    };

    // 2. Write to WAL first (CRITICAL!)
    self.wal.append(&entry)?;
    self.wal.sync()?; // Force to disk

    // 3. Update MemTable in memory
    self.memtable.put(key, value, timestamp)?;

    // 4. Return success to client
    Ok(())
}
```

**The crucial ordering:**

1. **WAL write + sync** - Durably record the operation
2. **MemTable update** - Update in-memory state
3. **Return success** - Tell client operation succeeded

If a crash happens after step 2, we can replay from the WAL. If it happens before step 2, the operation never happened (which is consistent).

## Crash Recovery: Rebuilding State

When FerrisDB starts up after a crash, it performs recovery:

```rust
pub fn recover_from_wal(&mut self) -> Result<()> {
    let mut wal_reader = WALReader::new(&self.wal_path)?;

    // Read all valid entries from WAL
    let entries = wal_reader.read_all()?;

    println!("Recovering {} operations from WAL", entries.len());

    // Replay each operation into MemTable
    for entry in entries {
        match entry.operation {
            Operation::Put => {
                self.memtable.put(
                    entry.key,
                    entry.value,
                    entry.timestamp
                )?;
            }
            Operation::Delete => {
                self.memtable.delete(entry.key, entry.timestamp)?;
            }
        }
    }

    println!("Recovery complete!");
    Ok(())
}
```

### Handling Corruption and Partial Writes

Real-world systems must handle corruption gracefully:

```rust
pub fn read_entry(&mut self) -> Result<Option<WALEntry>> {
    // Read length prefix
    let mut length_buf = [0u8; 4];
    match self.reader.read_exact(&mut length_buf) {
        Err(_) => return Ok(None), // End of file
        Ok(_) => {}
    }

    let length = u32::from_le_bytes(length_buf);

    // Read entry data
    let mut entry_buf = vec![0u8; length as usize];
    self.reader.read_exact(&mut entry_buf)?;

    // Verify checksum
    let stored_checksum = u32::from_le_bytes([
        entry_buf[0], entry_buf[1],
        entry_buf[2], entry_buf[3]
    ]);

    let computed_checksum = crc32fast::hash(&entry_buf[4..]);

    if stored_checksum != computed_checksum {
        // Corruption detected - skip this entry
        eprintln!("WAL corruption detected, skipping entry");
        return Ok(None);
    }

    // Why skip instead of panic?
    // - Partial write during crash (normal)
    // - Disk corruption (unfortunate but happens)
    // - Better to lose one entry than crash on startup

    // Decode valid entry
    let entry = WALEntry::decode(&entry_buf[4..])?;
    Ok(Some(entry))
}
```

## MVCC and Timestamps

FerrisDB's WAL includes timestamps, which enables Multi-Version Concurrency Control (MVCC):

```rust
// Example: Multiple versions of the same key
WAL entries:
[Put(key="user:1", value="Alice", timestamp=100)]
[Put(key="user:1", value="Alice Smith", timestamp=150)]
[Delete(key="user:1", timestamp=200)]

// After recovery, MemTable contains all versions:
MemTable: {
    "user:1" => [
        (value="Alice", ts=100),
        (value="Alice Smith", ts=150),
        (DELETE, ts=200)  // Tombstone
    ]
}
```

This enables:

- **Point-in-time queries**: "What was user:1 at timestamp 125?"
- **Transaction isolation**: Each transaction sees a consistent snapshot
- **Conflict detection**: Detecting concurrent modifications

## Real-World Example

Let's trace through a complete example:

**Initial state:**

```
MemTable: {}
WAL: (empty)
```

**Operations:**

```rust
db.put("user:1", "Alice")?;     // timestamp=100
db.put("user:2", "Bob")?;       // timestamp=101
db.delete("user:1")?;           // timestamp=102
```

**After operations:**

```
MemTable: {
    "user:2" => "Bob"@101,
    "user:1" => DELETE@102
}
WAL: [
    Put(user:1, Alice, 100),
    Put(user:2, Bob, 101),
    Delete(user:1, 102)
]
```

**💥 CRASH! System restarts...**

**Recovery process:**

```
1. Read WAL entries: 100, 101, 102
2. Replay operations in order:
   - Put("user:1", "Alice", 100)
   - Put("user:2", "Bob", 101)
   - Delete("user:1", 102)
3. MemTable restored: {
     "user:2" => "Bob"@101,
     "user:1" => DELETE@102
   }
```

**Result**: System is back to exactly the same state! ✅

## Performance Considerations

### Why WAL is Fast

WAL transforms expensive random writes into fast sequential writes:

- **Random writes**: ~100-200 IOPS on typical SSDs
- **Sequential writes**: ~50,000+ IOPS on the same SSD

### Batching for Better Performance

```rust
pub fn batch_write(&mut self, operations: &[Operation]) -> Result<()> {
    // Write all operations to WAL in one sync
    for op in operations {
        let entry = WALEntry::from_operation(op);
        self.wal.append(&entry)?;
    }

    // Single fsync for entire batch
    self.wal.sync()?;

    // Update MemTable
    for op in operations {
        self.memtable.apply(op)?;
    }

    Ok(())
}
```

**Batching benefits:**

- Amortize sync costs across multiple operations
- Better throughput for write-heavy workloads
- Reduced write amplification

### Durability vs Performance Trade-offs

```rust
pub enum SyncMode {
    /// fsync after every write (maximum durability)
    Always,
    /// fsync every N milliseconds (good balance)
    Periodic(Duration),
    /// Never fsync (fastest, but data loss possible)
    Never,
}
```

Different applications need different guarantees:

- **Financial systems**: Always sync (durability critical)
- **Analytics**: Periodic sync (some data loss acceptable)
- **Caching**: Never sync (data is replaceable)

## WAL Maintenance and Rotation

WAL files grow over time and need maintenance:

```rust
pub fn checkpoint_and_rotate(&mut self) -> Result<()> {
    // 1. Flush MemTable to SSTable
    self.flush_memtable_to_sstable()?;

    // 2. All data is now durable in SSTables
    // 3. WAL is no longer needed for recovery
    self.wal.rotate()?;

    // 4. Start fresh WAL file
    Ok(())
}
```

**Why rotation matters:**

- Keeps WAL files manageable size
- Reduces recovery time after crashes
- Enables efficient storage management

## Conclusion: The Foundation of Reliability

Write-Ahead Logging is the bedrock of database reliability. It enables:

- **Durability**: Committed data survives crashes
- **Consistency**: System state is always recoverable
- **Performance**: Sequential writes are much faster than random writes
- **Simplicity**: Recovery is straightforward - just replay the log

Understanding WAL helps you appreciate why databases are reliable and gives insight into the trade-offs between performance and durability.

---

## Related Deep Dives

- [LSM-Trees and Storage Engine Design]({{ '/deep-dive/lsm-trees/' | relative_url }})
- [MVCC and Transaction Isolation]({{ '/deep-dive/mvcc/' | relative_url }}) _(coming soon)_
- [Distributed Consensus with Raft]({{ '/deep-dive/raft/' | relative_url }}) _(coming soon)_

## Further Reading

- [Architecture Overview]({{ '/architecture/' | relative_url }})
- [Storage Engine Design]({{ '/storage-engine/' | relative_url }})
- [Future Architecture Explorations]({{ '/future-architecture/' | relative_url }})
