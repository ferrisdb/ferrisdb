# FerrisDB Storage Engine

Core storage components for FerrisDB, providing durability, persistence, and performance.

## Components

### Write-Ahead Log (WAL)

Location: `src/wal/`

Provides durability through append-only logging of all write operations.

**Features**:

- O(1) append performance
- CRC32 corruption detection
- Configurable sync modes
- Thread-safe operations
- Zero-copy buffer management

**Status**: ✅ Production-ready with comprehensive testing

### MemTable (In Development)

Location: `src/memtable/`

In-memory ordered key-value store using skip lists.

**Planned Features**:

- Lock-free concurrent reads
- Fast ordered iteration
- Memory usage tracking
- Automatic flushing

**Status**: 🚧 Under development

### SSTable (Planned)

Location: `src/sstable/`

Sorted String Table for persistent, immutable storage.

**Planned Features**:

- Block-based format
- Bloom filters
- Compression support
- Index blocks

**Status**: 📋 Planned

### Storage Engine (Planned)

Location: `src/storage_engine.rs`

Coordinates all storage components into a cohesive engine.

**Planned Features**:

- LSM-tree architecture
- Compaction strategies
- Read/write path coordination
- Snapshot support

**Status**: 📋 Planned

## Testing

### Unit Tests

Located within each source file, testing private implementations.

```bash
# Run all unit tests
cargo test --lib

# Run specific module tests
cargo test --lib wal
```

### Integration Tests

Located in `tests/`, testing public APIs and component interactions.

```bash
# Run all integration tests
cargo test --tests

# Run specific test suite
cargo test --test wal_integration_tests
```

### Benchmarks

Located in `benches/`, validating performance characteristics.

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench wal_performance_proofs
```

## Test Coverage

| Component | Unit Tests | Integration | Benchmarks  | Overall |
| --------- | ---------- | ----------- | ----------- | ------- |
| WAL       | ✅ 90%+    | ✅ Complete | ✅ Complete | ✅ 95%+ |
| MemTable  | 🚧 Partial | 🔄 Planned  | 🔄 Planned  | 🚧 20%  |
| SSTable   | 🔄 Planned | 🔄 Planned  | 🔄 Planned  | 🔄 0%   |
| Engine    | 🔄 Planned | 🔄 Planned  | 🔄 Planned  | 🔄 0%   |

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
ferrisdb-storage = { path = "../ferrisdb-storage" }
```

Basic usage:

```rust
use ferrisdb_storage::wal::{WALWriter, WALEntry};
use ferrisdb_core::SyncMode;

// Create a WAL writer
let writer = WALWriter::new("data/wal.log", SyncMode::Normal, 64 * 1024 * 1024)?;

// Write an entry
let entry = WALEntry::new_put(b"key".to_vec(), b"value".to_vec(), timestamp)?;
writer.append(&entry)?;
```

## Architecture

```
┌─────────────────────────────────────────┐
│           Storage Engine API            │
├─────────────┬───────────┬───────────────┤
│   MemTable  │    WAL    │    SSTable    │
│  (In-Memory)│ (Logging) │ (Persistent)  │
└─────────────┴───────────┴───────────────┘
```

## Contributing

See the main [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines.

Key principles:

- Comprehensive testing (unit + integration + benchmarks)
- Performance validation through benchmarks
- Clear documentation with examples
- Thread safety where applicable
