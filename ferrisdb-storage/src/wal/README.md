# Write-Ahead Log (WAL) Module

The WAL provides durability guarantees by persisting all write operations to disk before applying them to in-memory structures.

## Components

### Core Components

#### `mod.rs`

Module documentation and public API exports. Provides comprehensive overview of:

- File format specification (header and entry structure)
- Performance characteristics
- Usage examples
- Design rationale

#### `log_entry.rs`

WAL entry encoding/decoding with binary format:

- **WALEntry**: Represents Put/Delete operations with key, value, and timestamp
- Binary format with CRC32 checksums for corruption detection
- Size limits: 1MB keys, 10MB values
- Zero-copy optimizations using BytesMut

**Test Coverage**: ✅ 95%+ (all error paths, boundaries, concurrent access)

#### `header.rs`

WAL file header (64 bytes, cache-line aligned):

- **WALHeader**: File identification, versioning, and integrity
- Magic bytes, version compatibility, file sequence numbers
- CRC32 checksum validation
- Future-proof with reserved fields

**Test Coverage**: ✅ 90%+ (validation, corruption, version handling)

#### `writer.rs`

Thread-safe WAL writer with configurable durability:

- **WALWriter**: Append-only writes with size tracking
- Sync modes: None (fast), Normal (OS buffer), Full (fsync)
- Automatic parent directory creation
- File size limit enforcement for rotation

**Test Coverage**: ✅ 90%+ (comprehensive error paths, concurrency)

#### `reader.rs`

Efficient WAL reader with zero-copy buffer management:

- **WALReader**: Sequential reading with corruption detection
- Dynamic buffer growth with reuse
- Iterator interface support
- Performance statistics tracking

**Test Coverage**: ✅ 85%+ (corruption handling, buffer management)

#### `metrics.rs`

Performance metrics collection:

- **WALMetrics**: Thread-safe counters and statistics
- Read/write success rates
- Sync duration tracking
- File size monitoring
- Zero overhead when not used

**Test Coverage**: ✅ 100% (all metric updates verified)

## Architecture

```
┌─────────────┐     ┌─────────────┐
│  WALWriter  │     │  WALReader  │
├─────────────┤     ├─────────────┤
│ Append-only │     │ Sequential  │
│ Thread-safe │     │ Buffer mgmt │
│ Size limits │     │ Corruption  │
└──────┬──────┘     └──────┬──────┘
       │                   │
       └────────┬──────────┘
                │
         ┌──────┴──────┐
         │  WAL File   │
         ├─────────────┤
         │   Header    │ 64 bytes
         ├─────────────┤
         │   Entry 1   │ Variable
         ├─────────────┤
         │   Entry 2   │ Variable
         ├─────────────┤
         │     ...     │
         └─────────────┘
```

## Performance Characteristics

- **Append**: O(1) - Constant time regardless of file size
- **Read**: O(n) - Linear in number of entries
- **Memory**: Zero-allocation reads after buffer warmup
- **Metrics**: Negligible overhead (<1%)

All performance claims are validated by benchmarks in `benches/`.

## Usage Patterns

### Writing

```rust
let writer = WALWriter::new(path, SyncMode::Normal, 64 * 1024 * 1024)?;
let entry = WALEntry::new_put(key, value, timestamp)?;
writer.append(&entry)?;
```

### Reading

```rust
let mut reader = WALReader::new(path)?;
for entry in reader {
    process_entry(entry?);
}
```

## Testing Strategy

### Unit Tests (in each file)

- Private function testing
- Error path coverage
- Edge cases and boundaries

### Integration Tests (in tests/)

- Public API testing
- Multi-component workflows
- Concurrent access patterns
- Format validation

## Future Enhancements

- [ ] Compression support (LZ4/Snappy)
- [ ] Streaming API for large values
- [ ] Parallel reader support
- [ ] Built-in file rotation
