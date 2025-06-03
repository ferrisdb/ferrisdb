# FerrisDB Development Guide

Technical guide for developing FerrisDB. For contribution guidelines, see [CONTRIBUTING.md](CONTRIBUTING.md).

## Project Structure

```text
ferrisdb/
├── ferrisdb-core/       # Common types and traits
├── ferrisdb-storage/    # Storage engine (LSM-tree implementation)
├── ferrisdb-client/     # Client library (stub for now)
├── ferrisdb-server/     # Server implementation (planned)
├── guidelines/          # Development guidelines
├── docs/                # Documentation site
│   ├── _posts/          # Blog posts (both human and AI)
│   ├── database-concepts/ # Technical articles
│   ├── rust-by-example/ # Educational content
│   └── blog/            # Blog index pages
└── tests/               # Integration tests
```

## Development Setup

### Prerequisites

- Rust 1.81+ (via [rustup](https://rustup.rs/))
- Optional: `cargo-watch`, `cargo-nextest`

### Essential Tools

```bash
# Install all build/format tools
./scripts/setup-dev-tools.sh

# Manual installation
rustup toolchain install stable
rustup component add rustfmt clippy
cargo install cargo-watch cargo-nextest
npm install -g prettier
```

### Environment Variables

```bash
# Enable debug logging
export RUST_LOG=debug
export RUST_BACKTRACE=1

# For async debugging with tokio-console
export TOKIO_CONSOLE_BIND=127.0.0.1:6669
```

## Development Workflow

### Daily Development Loop

```bash
# 1. Start fresh
git checkout main && git pull

# 2. Create feature branch
git checkout -b feature/your-feature

# 3. Development with auto-reload
cargo watch -x "test --all"

# 4. Before committing
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
prettier --write "**/*.md"
```

### Building & Testing

```bash
# Build everything
cargo build --all

# Run all tests
cargo test --all

# Run specific test
cargo test test_name

# Run with output
cargo test --all -- --nocapture

# Use nextest (faster)
cargo nextest run --all

# Generate docs
cargo doc --all --no-deps --open
```

## Architecture Overview

### Current Implementation

We have a basic LSM-tree storage engine:

```
Write Path: Client → WAL → MemTable → (flush) → SSTable
Read Path:  Client → MemTable → SSTables (newest first)
```

**Components:**

- **WAL**: Write-ahead log for durability
- **MemTable**: In-memory skip list for recent writes
- **SSTable**: Immutable on-disk sorted files
- **Storage Engine**: Coordinates the above components

### Key Design Decisions

1. **Skip List for MemTable**: Lock-free concurrent access
2. **Binary Format for SSTable**: Space-efficient with 4KB blocks
3. **MVCC Timestamps**: Prepare for future transaction support
4. **Arc for Sharing**: Safe concurrent access to immutable data

## Code Organization

### ferrisdb-core

- `types.rs`: Common types (Key, Value, Timestamp)
- `error.rs`: Error types using `thiserror`

### ferrisdb-storage

- `wal/`: Write-ahead log implementation
- `memtable/`: Skip list based memory table
- `sstable/`: Sorted string table format
- `storage_engine.rs`: Main storage API

## Testing Strategy

### Test Categories

1. **Unit Tests**: In `src/` files next to code
2. **Integration Tests**: In `tests/` directory
3. **Doc Tests**: Examples in documentation
4. **Benchmarks**: In `benches/` (coming soon)

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        // Arrange
        let storage = TestStorage::new();

        // Act
        storage.put(b"key", b"value").unwrap();

        // Assert
        assert_eq!(storage.get(b"key").unwrap(), Some(b"value".to_vec()));
    }
}
```

## Performance Considerations

### Current Optimizations

- Binary search in SSTable blocks (O(log n))
- Lock-free skip list for concurrent access
- Memory-mapped files for read-only SSTables

### Profiling

```bash
# CPU profiling with flamegraph
cargo install flamegraph
cargo flamegraph --bin ferrisdb-server

# Memory profiling (Linux)
heaptrack target/debug/ferrisdb-server
```

## Debugging Tips

### Using tracing

```rust
use tracing::{debug, info};

#[tracing::instrument]
fn complex_operation(key: &[u8]) -> Result<()> {
    debug!("Starting operation");
    // ... code ...
    info!("Operation completed");
    Ok(())
}
```

### Common Issues

1. **Lifetime Errors**: Usually means you're trying to return a reference to local data
2. **Cannot Move**: Use `.clone()` or restructure to avoid moving
3. **Type Mismatch**: Check if you need `&` or `&mut`

## For AI Developers

See [CLAUDE.md](CLAUDE.md) for AI-specific guidelines. Key points:

- Use `TodoRead/TodoWrite` to track work
- Run tests frequently
- Follow existing patterns in codebase
- Ask clarifying questions

## Current Focus Areas

1. **Compaction**: Merging SSTables to reclaim space
2. **Bloom Filters**: Reduce unnecessary disk reads
3. **Block Cache**: Cache frequently accessed blocks
4. **Documentation**: Technical deep-dives and examples

## Resources

- [Architecture Docs](docs/architecture.md) - System design
- [Storage Engine](docs/storage-engine.md) - Implementation details
- [Development Blog](https://ferrisdb.org/blog/) - Daily progress
- [Rust Book](https://doc.rust-lang.org/book/) - Rust learning

---

_Questions? Open an issue or check existing discussions. Happy coding! 🦀_
