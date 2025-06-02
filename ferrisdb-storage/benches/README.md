# WAL Benchmarks

Performance benchmarks for the Write-Ahead Log (WAL) module.

## Benchmark Files

### `wal_performance_proofs.rs`

Proves specific performance characteristics claimed in documentation:

- **O(1) Append Complexity**: Verifies append time remains constant regardless of file size (1MB to 1GB)
- **O(n) Read Complexity**: Confirms read time scales linearly with entry count (100 to 100K entries)
- **Buffer Reuse**: Demonstrates zero-allocation reads after initial buffer allocation
- **Metrics Overhead**: Validates negligible performance impact of metrics collection
- **Sync Mode Characteristics**: Compares performance of None, Normal, and Full sync modes

### `wal_benchmarks.rs`

General performance benchmarks for common operations:

- **Read Performance**: Bulk reads, individual reads, buffer size impact
- **Write Throughput**: Tests various entry sizes (100B to 100KB)
- **Concurrent Operations**: Scalability with 1, 2, 4, and 8 concurrent readers
- **Zero-Allocation Reads**: Verifies buffer reuse efficiency
- **Sync Mode Comparison**: Performance impact of different durability levels

### `wal_performance.rs`

Micro-benchmarks for low-level operations:

- **Entry Encoding**: Small and large entry serialization
- **Entry Decoding**: Small and large entry deserialization
- **Checksum Calculation**: CRC32 overhead measurement

## Running Benchmarks

```bash
# Run all WAL benchmarks
cargo bench --package ferrisdb-storage

# Run specific benchmark suite
cargo bench --bench wal_performance_proofs

# Run with detailed output
cargo bench --bench wal_benchmarks -- --verbose

# Run specific test
cargo bench --bench wal_performance_proofs -- append_is_constant_time
```

## Performance Characteristics

All performance claims are validated through benchmarks:

- ✅ O(1) append complexity proven
- ✅ O(n) read complexity proven
- ✅ Efficient buffer reuse demonstrated
- ✅ Low metrics overhead confirmed
- ✅ Sync mode tradeoffs quantified

## Benchmark Framework

Uses [Criterion.rs](https://github.com/bheisler/criterion.rs) for:

- Statistical analysis
- Performance regression detection
- HTML report generation
- Consistent measurement methodology
