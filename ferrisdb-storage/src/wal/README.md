# Write-Ahead Log (WAL) Implementation

This module implements the Write-Ahead Log for FerrisDB, providing durability guarantees for all write operations.

## Current Implementation Status

The WAL currently provides:
- ✅ Basic write and read operations
- ✅ CRC32 checksums for corruption detection
- ✅ Configurable sync modes (None, Normal, Full)
- ✅ Thread-safe concurrent writes
- ✅ Size-based rotation triggers
- ✅ Partial entry handling during recovery

## WAL Format Specification v0 (Current)

### Entry Format
```
+------------+------------+------------+-------+------------+
| Length(4B) | CRC32(4B)  | Time(8B)   | Op(1B)| Key Len(4B)|
+------------+------------+------------+-------+------------+
| Key(var)   | Val Len(4B)| Value(var) |
+------------+------------+------------+
```

**Note**: Current format has no file header or version information.

## Improvement Roadmap

### Phase 1: File Header and Versioning ⏳

- [ ] Add magic number (0x57414C21 / "WAL!")
- [ ] Add version field for format evolution
- [ ] Add file header structure:
  ```rust
  struct WALHeader {
      magic: u32,         // 0x57414C21 ("WAL!")
      version: u32,       // Format version (start with 1)
      flags: u32,         // Feature flags (compression, encryption)
      page_size: u32,     // For future page-based I/O
      creation_time: u64, // When WAL was created
      checksum: u32,      // Header checksum
  }
  ```
- [ ] Implement header validation on open
- [ ] Support reading v0 (headerless) files for migration
- [ ] Add version compatibility checks
- [ ] Update documentation with format specification

### Phase 2: Log Sequence Numbers (LSN) 📋

- [ ] Replace timestamp-only sequencing with proper LSN
- [ ] Add LSN to entry structure:
  ```rust
  pub struct WALEntry {
      pub lsn: u64,        // Monotonic byte offset
      pub prev_lsn: u64,   // Previous entry's LSN (for consistency)
      pub timestamp: u64,  // Wall clock time (keep for debugging)
      pub operation: Operation,
      pub key: Key,
      pub value: Value,
  }
  ```
- [ ] Implement LSN tracking in writer
- [ ] Add gap detection in reader
- [ ] Support partial replay from specific LSN
- [ ] Add LSN-based consistency checks

### Phase 3: Automatic Rotation 🔄

- [ ] Implement segment-based file naming (wal.00000001, wal.00000002)
- [ ] Add automatic rotation when size limit reached
- [ ] Create `WALManager` to coordinate multiple files:
  ```rust
  pub struct WALManager {
      base_path: PathBuf,
      current_segment: u64,
      current_writer: WALWriter,
      segment_size: u64,
      retention_policy: RetentionPolicy,
  }
  ```
- [ ] Add retention policies (keep last N files, time-based)
- [ ] Implement cleanup of old segments
- [ ] Add segment discovery during recovery
- [ ] Support reading across multiple segments

### Phase 4: Page-Based I/O 📄

- [ ] Implement fixed-size page structure (8KB default)
- [ ] Add page headers with metadata:
  ```rust
  struct WALPage {
      lsn: u64,           // First LSN in page
      prev_page_lsn: u64, // For consistency
      checksum: u32,      // Page checksum
      flags: u16,         // Page flags
      remaining: u16,     // Bytes remaining in page
  }
  ```
- [ ] Support entries spanning multiple pages
- [ ] Implement aligned I/O for performance
- [ ] Add page-level corruption recovery
- [ ] Enable direct I/O support (bypass OS cache)

### Phase 5: Performance Optimizations 🚀

- [ ] Implement group commit / write batching:
  ```rust
  pub struct BatchedWALWriter {
      pending: Vec<WALEntry>,
      last_flush: Instant,
      max_batch_size: usize,
      max_delay: Duration,
  }
  ```
- [ ] Add write combining for small entries
- [ ] Implement parallel recovery (partition by key)
- [ ] Add compression support (LZ4, Snappy)
- [ ] Optimize memory allocation patterns
- [ ] Add zero-copy paths where possible

### Phase 6: Advanced Features 🔧

- [ ] Add encryption support (at-rest encryption)
- [ ] Implement WAL archiving for backup
- [ ] Add replication support (streaming WAL)
- [ ] Support point-in-time recovery
- [ ] Add WAL inspection tools
- [ ] Implement WAL compression
- [ ] Add metrics and monitoring hooks

## Migration Strategy

### From v0 to v1 (Adding Headers)

1. **Detection**: Check first 4 bytes for magic number
2. **Migration**: If no magic, assume v0 format
3. **Compatibility**: Support reading both formats
4. **Upgrade**: Rewrite with header on next rotation

### Future Version Upgrades

1. **Version Check**: Refuse to open incompatible versions
2. **Migration Tool**: Provide offline conversion utility
3. **Online Upgrade**: Support live format upgrades
4. **Rollback**: Keep old format until confirmed stable

## Testing Requirements

### Current Tests ✅
- Unit tests for encode/decode
- Integration tests for write/read
- Concurrent write tests
- Corruption detection tests
- Edge case tests (disk full, etc.)

### Additional Tests Needed 📝
- [ ] Version migration tests
- [ ] Multi-segment recovery tests
- [ ] Page boundary tests
- [ ] Performance regression tests
- [ ] Crash recovery scenarios
- [ ] Replication tests

## Performance Targets

### Current Performance
- Single-threaded writes: ~50K ops/sec (SyncMode::None)
- Concurrent writes: Serialized through mutex
- Recovery speed: ~100K entries/sec

### Target Performance
- [ ] Group commit: 200K+ ops/sec
- [ ] Parallel recovery: 500K+ entries/sec
- [ ] Page-based I/O: 2x throughput improvement
- [ ] Direct I/O: Predictable latency

## API Stability

### Stable APIs (v1.0)
- `WALWriter::new()`
- `WALWriter::append()`
- `WALReader::new()`
- `WALReader::read_all()`
- `WALEntry` structure

### Unstable APIs (subject to change)
- Internal encoding format
- File naming scheme
- Recovery strategies
- Performance optimizations

## References

- [PostgreSQL WAL Design](https://www.postgresql.org/docs/current/wal-internals.html)
- [RocksDB WAL Format](https://github.com/facebook/rocksdb/wiki/Write-Ahead-Log-File-Format)
- [SQLite WAL Mode](https://www.sqlite.org/wal.html)
- [MySQL InnoDB Redo Log](https://dev.mysql.com/doc/refman/8.0/en/innodb-redo-log.html)

## Contributing

When implementing improvements:

1. Follow the phases in order (dependencies exist)
2. Maintain backward compatibility where possible
3. Add comprehensive tests for new features
4. Update this README with progress
5. Document format changes clearly
6. Benchmark before/after performance

## Questions?

For design discussions or questions about the WAL implementation, please open an issue or reach out to the storage team.