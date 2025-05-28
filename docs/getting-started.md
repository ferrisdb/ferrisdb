---
layout: page
title: Getting Started
subtitle: How to build, run, and contribute to FerrisDB
permalink: /getting-started/
---

Welcome to FerrisDB! This guide will help you get up and running with the project, whether you're interested in following along with development or contributing to the codebase.

## ⚠️ Important Note

**FerrisDB is an educational project** designed for learning Rust and distributed systems concepts. It is **not intended for production use**. If you need a production-ready distributed database, consider [FoundationDB](https://apple.github.io/foundationdb/), [TiKV](https://tikv.org/), or [CockroachDB](https://www.cockroachlabs.com/).

## Prerequisites

### Requirements

- **Rust 1.81+** - Install from [rustup.rs](https://rustup.rs/)
- **Git** - For cloning the repository

### Verify Installation

```bash
# Check Rust installation
rustc --version
cargo --version

# Check Git installation
git --version
```

## Quick Start

### 1. Clone the Repository

```bash
git clone https://github.com/ferrisdb/ferrisdb.git
cd ferrisdb
```

### 2. Build the Project

```bash
# Build all workspace members
cargo build --all

# Or build with optimizations (slower build, faster runtime)
cargo build --release --all
```

### 3. Run Tests

```bash
# Run all tests
cargo test --all

# Run tests with output (helpful for debugging)
cargo test --all -- --nocapture

# Run tests for a specific crate
cargo test -p ferrisdb-storage
```

### 4. Check Code Quality

```bash
# Format code
cargo fmt --all

# Run linter
cargo clippy --all-targets --all-features -- -D warnings

# Generate documentation
cargo doc --all --no-deps --open
```

## Project Structure

FerrisDB uses a Rust workspace with multiple crates:

```
ferrisdb/
├── ferrisdb-core/       # Common types and traits
├── ferrisdb-storage/    # Storage engine (LSM-tree)
├── ferrisdb-client/     # Client library
├── ferrisdb-server/     # Server implementation
├── ferrisdb/            # Main binary crate
├── CLAUDE.md           # Development guidelines
└── docs/               # Website and documentation
```

## Development Workflow

### Building Specific Components

```bash
# Build only the storage engine
cargo build -p ferrisdb-storage

# Build and run tests for storage engine
cargo test -p ferrisdb-storage

# Build with debug logging enabled
RUST_LOG=debug cargo build -p ferrisdb-storage
```

### Running with Debug Output

```bash
# Run with trace-level logging
RUST_LOG=trace cargo test -p ferrisdb-storage

# Run specific test with logging
RUST_LOG=debug cargo test -p ferrisdb-storage wal_tests::test_write_and_read
```

### Working with the Storage Engine

The storage engine is the most developed component. Here's how to explore it:

```bash
# Look at the storage engine tests
ls ferrisdb-storage/src/

# Run WAL (Write-Ahead Log) tests
cargo test -p ferrisdb-storage wal

# Run MemTable tests
cargo test -p ferrisdb-storage memtable

# View test output
cargo test -p ferrisdb-storage -- --nocapture
```

## Understanding the Code

### Key Components (Current Status)

#### ✅ Write-Ahead Log (WAL)

- **Location**: `ferrisdb-storage/src/wal/`
- **Status**: Complete with tests
- **Features**: Binary encoding, CRC32 checksums, crash recovery

```bash
# Explore WAL implementation
ls ferrisdb-storage/src/wal/
# Files: log_entry.rs, writer.rs, reader.rs, mod.rs
```

#### ✅ MemTable with Skip List

- **Location**: `ferrisdb-storage/src/memtable/`
- **Status**: Complete with concurrent access
- **Features**: Lock-free skip list, MVCC support, timestamp ordering

```bash
# Explore MemTable implementation
ls ferrisdb-storage/src/memtable/
# Files: skiplist.rs, memtable.rs, mod.rs
```

#### 🚧 SSTable (In Progress)

- **Location**: `ferrisdb-storage/src/sstable/`
- **Status**: Basic structure, needs implementation
- **Next**: File format, compression, bloom filters

### Reading the Code

Start with these files to understand the architecture:

1. **`ferrisdb-core/src/types.rs`** - Core data types
2. **`ferrisdb-storage/src/wal/log_entry.rs`** - WAL entry format
3. **`ferrisdb-storage/src/memtable/skiplist.rs`** - Skip list implementation
4. **[Architecture Documentation]({{ '/architecture/' | relative_url }})** - Overall system design
5. **[Storage Engine Documentation]({{ '/storage-engine/' | relative_url }})** - Storage engine details

## Running Examples

### Basic WAL Usage

Currently, there are no high-level examples, but you can run the tests to see the components in action:

```bash
# See WAL in action
cargo test -p ferrisdb-storage wal_tests::test_write_and_read -- --nocapture

# See MemTable operations
cargo test -p ferrisdb-storage memtable_tests::test_basic_operations -- --nocapture
```

### Benchmark Performance

```bash
# Run benchmarks (when available)
cargo bench

# Profile with perf (Linux only)
cargo build --release
perf record --call-graph=dwarf target/release/ferrisdb-storage-example
```

## Development Setup

### Recommended VS Code Extensions

```json
{
  "recommendations": [
    "rust-lang.rust-analyzer",
    "vadimcn.vscode-lldb",
    "serayuzgur.crates",
    "tamasfe.even-better-toml"
  ]
}
```

### Git Workflow

```bash
# Create a feature branch
git checkout -b feature/my-feature

# Make changes and commit
git add .
git commit -m "feat: add new feature"

# Push and create PR
git push origin feature/my-feature
```

### Branch Protection Rules

The main branch has protection rules:

- Pull requests required
- Linear history enforced
- Dismiss stale reviews on new commits

## Contributing

### Development Guidelines

Follow the guidelines in `CLAUDE.md`:

- **Code Style**: Use `rustfmt` and `clippy`
- **Documentation**: Document all public APIs
- **Testing**: Write tests for new functionality
- **Error Handling**: Use proper `Result` types
- **Performance**: Profile before optimizing

### Areas for Contribution

Current priorities for contributors:

1. **SSTable Implementation** - File format, compression, bloom filters
2. **Compaction Strategy** - Background merging and optimization
3. **Integration Tests** - Multi-threaded scenarios
4. **Benchmarks** - Performance measurement
5. **Documentation** - API docs and examples

### Submitting Changes

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Submit a pull request

## Troubleshooting

### Common Issues

**Build Failures:**

```bash
# Clean and rebuild
cargo clean
cargo build --all
```

**Test Failures:**

```bash
# Run tests with full output
cargo test --all -- --nocapture

# Run a specific test
cargo test -p ferrisdb-storage test_name -- --nocapture
```

**Clippy Warnings:**

```bash
# Fix automatically where possible
cargo clippy --all-targets --all-features --fix

# Manual review required for some warnings
cargo clippy --all-targets --all-features
```

### Getting Help

- **GitHub Issues**: Report bugs or ask questions
- **Discussions**: General questions and ideas
- **Blog**: Follow daily development progress
- **Documentation**: [Architecture]({{ '/architecture/' | relative_url }}) and [Storage Engine]({{ '/storage-engine/' | relative_url }}) guides

## What's Next?

After getting familiar with the codebase:

1. **Follow the Blog** - Daily development updates
2. **Read the Design Docs** - Understand the architecture
3. **Try the Examples** - Run tests and explore the code
4. **Join Development** - Pick up an issue or suggest improvements

## Learning Resources

### Distributed Systems

- [FoundationDB Paper](https://apple.github.io/foundationdb/)
- [Google Spanner Paper](https://research.google/pubs/pub39966/)
- [Designing Data-Intensive Applications](https://dataintensive.net/)

### Rust Programming

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Async Rust Book](https://rust-lang.github.io/async-book/)

### Storage Engines

- [Database Internals](https://www.databass.dev/)
- [LSM-tree Paper](https://www.cs.umb.edu/~poneil/lsmtree.pdf)
- [RocksDB Wiki](https://github.com/facebook/rocksdb/wiki)

---

**Ready to start?** Clone the repository and run `cargo test --all` to see everything in action!

[🚀 Clone FerrisDB]({{ site.project.repo_url }}){: .btn .btn-primary}
[📖 Read Architecture]({{ '/architecture/' | relative_url }}){: .btn .btn-outline}
