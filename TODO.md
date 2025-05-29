# FerrisDB TODO List

## Code Quality

- [ ] Remove `#[allow(dead_code)]` annotations when methods are implemented and used
  - `MemTableIterator.skiplist` field
  - `StorageEngine.config` field
  - `SkipList.size()` method

## Documentation

- [x] Add module-level documentation comments
- [x] Document public APIs with examples
- [x] Add comprehensive examples in doc comments
- [x] Generate and review cargo doc output
- [ ] Add architecture diagrams in documentation
- [ ] Create rustdoc book with mdBook integration

## Storage Engine

- [x] Implement Write-Ahead Log (WAL)
- [x] Implement MemTable with Skip List
- [x] Design SSTable binary format specification
- [x] Implement SSTable writer
- [x] Implement SSTable reader and index
- [x] Refactor Operation from InternalKey to SSTableEntry (Day 2)
- [x] Optimize SSTable reader with binary search (Day 2)
- [x] Improve SSTable API for better usability (Day 2)
- [ ] Implement compaction strategy
- [ ] Implement bloom filters for optimization
- [ ] Add block cache for frequently accessed data
- [ ] Implement proper MemTable iterator

## Testing

- [ ] Add integration tests for concurrent operations
- [ ] Add stress tests with random operations
- [ ] Add crash recovery tests
- [ ] Add performance benchmarks
- [ ] Test with large datasets

## Future Enhancements

- [ ] Add compression support for SSTables
- [ ] Implement column families
- [ ] Add backup/restore functionality
- [ ] Implement statistics collection
- [ ] Add monitoring and metrics
