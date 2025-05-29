# FerrisDB Development Guide

This guide covers the technical aspects of developing FerrisDB, including environment setup, project structure, and development workflows.

## Table of Contents

- [Environment Setup](#environment-setup)
- [Project Structure](#project-structure)
- [Development Workflow](#development-workflow)
- [Commands Reference](#commands-reference)
- [Debugging](#debugging)
- [Performance Profiling](#performance-profiling)
- [Architecture Overview](#architecture-overview)
- [Storage Engine Details](#storage-engine-details)
- [Testing Strategy](#testing-strategy)

## Environment Setup

### Required Tools

```bash
# Rust toolchain (MSRV: 1.81.0)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup component add rustfmt clippy

# Development tools
cargo install cargo-watch cargo-nextest tokio-console

# Documentation tools (optional)
npm install -g markdownlint-cli2 prettier
```

### IDE Configuration

#### VS Code (Recommended)

Install these extensions:

- `rust-analyzer`: Rust language server
- `CodeLLDB`: Debugging support
- `Better TOML`: Cargo.toml syntax highlighting
- `markdownlint`: Markdown linting

#### Settings

```json
{
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.checkOnSave.command": "clippy",
  "editor.formatOnSave": true,
  "rust-analyzer.inlayHints.enable": true
}
```

### Environment Variables

```bash
# Development
export RUST_LOG=debug
export RUST_BACKTRACE=1

# Performance profiling
export CARGO_PROFILE_RELEASE_DEBUG=true

# Tokio console (debugging async)
export TOKIO_CONSOLE_BIND=127.0.0.1:6669
```

## Project Structure

```text
ferrisdb/
├── ferrisdb-core/          # Common types and traits
│   ├── src/
│   │   ├── lib.rs          # Core exports
│   │   ├── error.rs        # Error types
│   │   ├── types.rs        # Common types
│   │   └── traits.rs       # Core traits
│   └── Cargo.toml
├── ferrisdb-storage/       # Storage engine implementation
│   ├── src/
│   │   ├── lib.rs          # Storage engine exports
│   │   ├── wal/            # Write-ahead log
│   │   ├── memtable/       # In-memory storage
│   │   ├── sstable/        # Sorted string tables
│   │   ├── compaction/     # Background compaction
│   │   └── engine.rs       # Main storage engine
│   └── Cargo.toml
├── ferrisdb-client/        # Client library
├── ferrisdb-server/        # Server implementation
├── tests/                  # Integration tests
├── benches/                # Performance benchmarks
├── docs/                   # Documentation site
├── .github/                # CI/CD workflows
├── CLAUDE.md               # AI assistant guidelines
├── CONTRIBUTING.md         # Contribution guidelines
└── DEVELOPMENT.md          # This file
```

### Key Files

- **Cargo.toml**: Workspace configuration
- **deny.toml**: Dependency and license checking
- **TODO.md**: Planned features and improvements
- **.gitignore**: Git ignore patterns
- **LICENSE**: Project license

## Development Workflow

### Daily Development

1. **Start with fresh code**:

   ```bash
   git checkout main
   git pull origin main
   cargo build --all
   ```

2. **Create feature branch**:

   ```bash
   git checkout -b feature/your-feature
   ```

3. **Development loop**:

   ```bash
   # Watch for changes and run tests
   cargo watch -x "test --all"

   # Or manually run checks
   cargo clippy --all-targets --all-features
   cargo test --all
   cargo fmt --all
   ```

4. **Before committing**:

   ```bash
   # Full check
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test --all
   cargo fmt --all --check
   markdownlint-cli2 "**/*.md" "!target/**" "!**/target/**"
   ```

### Hot Reload Development

```bash
# Watch and rebuild on changes
cargo watch -x "build --all"

# Watch and run specific tests
cargo watch -x "test -p ferrisdb-storage"

# Watch and run with logging
RUST_LOG=debug cargo watch -x "run"
```

## Commands Reference

### Building

```bash
# Build all crates
cargo build --all

# Build with optimizations
cargo build --all --release

# Build specific crate
cargo build -p ferrisdb-storage

# Check compilation without building
cargo check --all
```

### Testing

```bash
# Run all tests (currently 55+ tests passing)
cargo test --all

# Run tests with output
cargo test --all -- --nocapture

# Run specific test
cargo test test_name

# Run tests for specific crate
cargo test -p ferrisdb-storage

# Integration tests only
cargo test --test integration_tests

# Run with nextest (faster)
cargo nextest run --all
```

### Linting and Formatting

```bash
# Format code
cargo fmt --all

# Check formatting
cargo fmt --all --check

# Run clippy
cargo clippy --all-targets --all-features

# Clippy with denial of warnings
cargo clippy --all-targets --all-features -- -D warnings

# Fix clippy suggestions
cargo clippy --all-targets --all-features --fix
```

### Documentation

```bash
# Generate API documentation
cargo doc --all --no-deps

# Generate and open documentation
cargo doc --all --no-deps --open

# Build with private items
cargo doc --all --no-deps --document-private-items

# Check doc links
cargo doc --all --no-deps --document-private-items
```

### Benchmarking

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench storage_benchmark

# Generate benchmark report
cargo bench -- --output-format html
```

### Security and Dependencies

```bash
# Security audit
cargo audit

# Check licenses and dependencies
cargo deny check

# Update dependencies
cargo update

# Show dependency tree
cargo tree
```

## Debugging

### Rust Debugging

#### Using `dbg!` Macro

```rust
let result = some_function();
dbg!(&result);  // Prints file:line: result = ...
```

#### Using `tracing` (Preferred)

```rust
use tracing::{debug, info, warn, error};

#[tracing::instrument]
fn complex_function(input: &str) -> Result<String> {
    debug!("Processing input: {}", input);
    // ... function body
    info!("Successfully processed");
    Ok(result)
}
```

#### VS Code Debugging

1. Set breakpoints in editor
2. Press F5 or use "Run and Debug"
3. Use integrated terminal for LLDB commands

### Async Debugging with Tokio Console

```bash
# Terminal 1: Run with tokio-console
RUSTFLAGS="--cfg tokio_unstable" cargo run --features tokio-console

# Terminal 2: Connect console
tokio-console
```

### Memory Debugging

```bash
# Run with Valgrind (Linux)
cargo build
valgrind --tool=memcheck target/debug/ferrisdb-server

# Memory profiling with heaptrack (Linux)
heaptrack target/debug/ferrisdb-server
```

## Performance Profiling

### CPU Profiling

#### Using `perf` (Linux)

```bash
# Build with debug symbols in release mode
CARGO_PROFILE_RELEASE_DEBUG=true cargo build --release

# Profile with perf
perf record -g target/release/ferrisdb-server
perf report
```

#### Using `cargo flamegraph`

```bash
# Install flamegraph
cargo install flamegraph

# Generate flamegraph
cargo flamegraph --bin ferrisdb-server
```

### Benchmarking Best Practices

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_storage_write(c: &mut Criterion) {
    let mut group = c.benchmark_group("storage");

    // Set measurement time
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("write_1kb", |b| {
        b.iter(|| {
            // Use black_box to prevent optimization
            black_box(storage.write(black_box(key), black_box(value)))
        });
    });

    group.finish();
}
```

## Architecture Overview

### Core Components

1. **Storage Engine**: LSM-tree based persistent storage
2. **WAL (Write-Ahead Log)**: Durability and crash recovery
3. **MemTable**: In-memory write buffer
4. **SSTable**: Immutable sorted files on disk
5. **Compaction**: Background cleanup and optimization

### Data Flow

```text
Write → WAL → MemTable → (Flush) → SSTable → (Compaction) → Merged SSTable
Read  → MemTable → SSTable (newest to oldest)
```

### Key Invariants

1. **WAL before MemTable**: WAL entries must be written before MemTable updates
2. **Timestamp ordering**: Keys sorted by (user_key, timestamp DESC)
3. **Tombstone handling**: Deletes create tombstones, not immediate deletion
4. **Checksum verification**: All disk writes include checksums
5. **MVCC consistency**: Reads see consistent snapshots

## Storage Engine Details

### File Formats

#### WAL Format

```text
Entry: | Length (4) | Checksum (4) | Timestamp (8) | Key Len (4) | Value Len (4) | Key | Value |
```

#### SSTable Format

```text
Data Block: | Key-Value pairs with delta encoding |
Index Block: | Sparse index pointing to data blocks |
Footer: | Index offset | Meta offset | Magic number |
```

### Memory Management

- Use `bytes::Bytes` for zero-copy buffer management
- Implement reference counting for shared data
- Use epoch-based reclamation for lock-free structures

### Concurrency Model

- **Reads**: Lock-free using immutable data structures
- **Writes**: Single-writer via async channel
- **Background tasks**: Compaction runs on separate async tasks
- **Resource limits**: Bounded channels and memory pools

## Testing Strategy

### Test Categories

1. **Unit Tests**: Individual function testing
2. **Integration Tests**: Component interaction testing
3. **Property Tests**: Invariant checking with `proptest`
4. **Stress Tests**: High-load scenario testing
5. **Chaos Tests**: Failure injection and recovery

### Test Organization

```text
tests/
├── integration/            # Integration tests
│   ├── storage_tests.rs    # Storage engine integration
│   ├── recovery_tests.rs   # Crash recovery tests
│   └── concurrent_tests.rs # Concurrency tests
├── stress/                 # Stress tests
└── chaos/                  # Chaos engineering tests
```

### Property Testing Example

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn storage_roundtrip(
        key in "\\PC*",
        value in "\\PC*"
    ) {
        let mut storage = TestStorage::new();
        storage.put(&key, &value)?;
        let retrieved = storage.get(&key)?;
        prop_assert_eq!(retrieved.as_deref(), Some(value.as_str()));
    }
}
```

### Continuous Integration

Our CI runs different test suites based on file changes:

- **Code changes**: Full Rust test suite
- **Docs changes**: Markdown lint, spell check, Jekyll build
- **All changes**: Security audit, license check

## Tips and Best Practices

### Performance Tips

1. **Profile first**: Don't optimize without measurements
2. **Async boundaries**: Minimize async/await overhead in hot paths
3. **Allocations**: Use object pools for frequently allocated objects
4. **I/O batching**: Group multiple operations together
5. **Lock contention**: Use lock-free algorithms where possible

### Code Organization

1. **Module structure**: Keep modules focused and cohesive
2. **Error handling**: Use `Result` consistently, avoid panics
3. **Documentation**: Every public item should have doc comments
4. **Testing**: Write tests before or alongside implementation
5. **Dependencies**: Minimize external dependencies

### Git Workflow

1. **Commit messages**: Use conventional commits format
2. **Branch naming**: Use descriptive feature/fix/docs prefixes
3. **PR size**: Keep pull requests focused and reviewable
4. **Rebase**: Keep history clean with interactive rebase
5. **Testing**: Always test locally before pushing

---

For more information, see:

- [CONTRIBUTING.md](./CONTRIBUTING.md) - Contribution guidelines
- [CLAUDE.md](./CLAUDE.md) - AI assistant development guidelines
- [Architecture Documentation](./docs/architecture.md) - System design details
- [Storage Engine Guide](./docs/storage-engine.md) - Storage implementation details
