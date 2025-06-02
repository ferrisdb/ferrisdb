# Storage Integration Tests

This directory contains integration tests for the FerrisDB storage engine components.
Unit tests are located within their respective source files in `src/`.

## Test Organization

### WAL (Write-Ahead Log) Tests

#### `wal_integration_tests.rs`

Tests complete WAL workflows through public API:

- Write/read cycles with data integrity verification
- Recovery from partial writes and crashes
- Large entry handling (up to 10MB values)
- Iterator API functionality
- Sync mode behavior and durability guarantees
- File size limit enforcement

**Coverage**: ✅ All major use cases covered

#### `wal_concurrent_tests.rs`

Thread safety and concurrent access tests:

- Concurrent writes maintain data integrity
- Metrics remain consistent during parallel operations
- Readers get consistent data during active writes
- Multiple concurrent readers see identical data
- Thread-safe metrics updates

**Coverage**: ✅ Comprehensive concurrency scenarios

#### `wal_format_tests.rs`

Binary format validation and corruption handling:

- Error conditions (oversized keys/values)
- Boundary conditions (empty data, maximum sizes)
- Corruption detection (checksums, truncation, invalid data)
- Version compatibility and future-proofing
- Header and entry format validation
- Truncation recovery scenarios

**Coverage**: ✅ All format edge cases tested

#### `wal_property_tests.rs`

Property-based testing with proptest:

- Roundtrip encoding/decoding with arbitrary data
- Invariant validation (size calculations, checksums)
- Stress testing with mixed valid/invalid entries
- Performance invariants (buffer reuse, allocation tracking)
- Concurrent operation properties

**Coverage**: ✅ Extensive property coverage

### Future Test Categories

As new components are added, their integration tests will follow this pattern:

- `memtable_integration_tests.rs` - MemTable public API tests
- `sstable_integration_tests.rs` - SSTable format and API tests
- `storage_engine_tests.rs` - Full storage engine integration

## Running Tests

```bash
# Run all storage tests
cargo test --package ferrisdb-storage

# Run specific test category
cargo test --test wal_concurrent_tests
cargo test --test wal_integration_tests
cargo test --test wal_format_tests
cargo test --test wal_property_tests

# Run with output for debugging
cargo test --test wal_integration_tests -- --nocapture

# Run single-threaded for deterministic debugging
cargo test --test wal_concurrent_tests -- --test-threads=1
```

## Test Naming Convention

All tests follow: `method_expected_behavior_condition`

Examples:

- `append_maintains_data_integrity_during_concurrent_writes`
- `read_all_recovers_complete_entries_before_partial_write`
- `decode_detects_corrupted_checksum`

## Test Coverage Status

| Component | Integration | Concurrent | Format     | Property   | Overall |
| --------- | ----------- | ---------- | ---------- | ---------- | ------- |
| WAL       | ✅ 100%     | ✅ 100%    | ✅ 100%    | ✅ 100%    | ✅ 100% |
| MemTable  | 🔄 Planned  | 🔄 Planned | N/A        | 🔄 Planned | 🔄 0%   |
| SSTable   | 🔄 Planned  | 🔄 Planned | 🔄 Planned | 🔄 Planned | 🔄 0%   |
