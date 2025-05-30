# Storage Engine Guidelines

Guidelines for working on FerrisDB's LSM-tree storage engine.

## Storage Engine Components

### MemTable

The in-memory component using a concurrent skip list.

**Guidelines:**

- Keep the skip list implementation lock-free
- Use Arc for node management (no unsafe)
- Maintain sorted order for efficient scans
- Track memory usage for flush decisions

**Interface:**

```rust
pub trait MemTable: Send + Sync {
    fn put(&self, key: &[u8], value: &[u8]) -> Result<()>;
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>>;
    fn delete(&self, key: &[u8]) -> Result<()>;
    fn scan(&self, start: &[u8], end: &[u8]) -> Result<Vec<(Vec<u8>, Vec<u8>)>>;
}
```

### SSTable (Sorted String Table)

Immutable on-disk files with sorted key-value pairs.

**Format:**

```
┌─────────────┐
│   Header    │ - Magic number, version
├─────────────┤
│ Data Blocks │ - Key-value pairs
├─────────────┤
│ Index Block │ - Block offsets
├─────────────┤
│Bloom Filter │ - Optional
├─────────────┤
│   Footer    │ - Metadata, checksums
└─────────────┘
```

**Guidelines:**

- Use block-based format for efficient reads
- Include bloom filters for existence checks
- Compress blocks individually
- Calculate checksums for data integrity

### Write-Ahead Log (WAL)

Durability through sequential writes.

**Entry Format:**

```rust
pub struct LogEntry {
    pub sequence: u64,
    pub operation: Operation,
    pub key: Vec<u8>,
    pub value: Option<Vec<u8>>,
    pub checksum: u32,
}
```

**Guidelines:**

- Always sync after write (configurable)
- Use CRC32 for checksums
- Implement log rotation
- Support recovery from partial writes

### Compaction

Background process to merge SSTables.

**Strategies:**

1. **Size-tiered**: Compact similar-sized files
2. **Leveled**: Maintain level invariants
3. **Universal**: Balance space and write amplification

**Guidelines:**

- Start with size-tiered (simpler)
- Make strategy pluggable
- Track statistics
- Implement rate limiting

## Best Practices

### Error Handling

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Corruption detected: {0}")]
    Corruption(String),

    #[error("Key not found")]
    KeyNotFound,
}
```

### Testing Storage Components

```rust
#[cfg(test)]
mod tests {
    // Test single-threaded operations
    #[test]
    fn test_basic_operations() { }

    // Test concurrent operations
    #[test]
    fn test_concurrent_writes() { }

    // Test crash recovery
    #[test]
    fn test_recovery_after_crash() { }

    // Test compaction
    #[test]
    fn test_compaction_correctness() { }
}
```

### Performance Considerations

1. **Write Path**

   - Batch writes when possible
   - Use write buffer
   - Async WAL writes (with durability trade-off)

2. **Read Path**

   - Check MemTable first
   - Use bloom filters
   - Cache frequently accessed blocks
   - Implement read-ahead

3. **Memory Management**
   - Monitor MemTable size
   - Implement back-pressure
   - Use memory-mapped files carefully

### Debugging Tools

Build debugging aids:

```rust
impl StorageEngine {
    #[cfg(debug_assertions)]
    pub fn dump_stats(&self) -> Stats {
        // Return internal statistics
    }

    #[cfg(debug_assertions)]
    pub fn validate_invariants(&self) -> Result<()> {
        // Check all invariants hold
    }
}
```

## Common Pitfalls

1. **Not handling partial writes** - Always checksum
2. **Ignoring memory pressure** - Monitor and limit
3. **Poor compaction scheduling** - Balance with foreground work
4. **Missing fsync** - Durability requires explicit sync
5. **Concurrent modification** - Use proper synchronization

## References

- [LevelDB Implementation Notes](https://github.com/google/leveldb/blob/main/doc/impl.md)
- [RocksDB Wiki](https://github.com/facebook/rocksdb/wiki)
- [Building a Database](https://cstack.github.io/db_tutorial/)
- [The Log-Structured Merge-Tree](https://www.cs.umb.edu/~poneil/lsmtree.pdf)
