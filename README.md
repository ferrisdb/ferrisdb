# FerrisDB

<img src="docs/assets/images/ferrisdb_logo.svg" alt="FerrisDB Logo" width="120">

A distributed, transactional key-value database written in Rust, inspired by FoundationDB.

> ⚠️ **Educational Project Disclaimer**
>
> This project is created for personal learning purposes:
>
> - Learning Rust programming language
> - Understanding distributed systems concepts
> - Exploring AI-assisted development with Claude Code
>
> **This is NOT recommended for production use.** The code is experimental and primarily serves as a learning exercise.
> If you're looking for a production-ready distributed database, consider established solutions like FoundationDB, TiKV, or CockroachDB.

## Features

- **ACID Transactions**: Full ACID compliance with serializable isolation
- **Distributed**: Horizontally scalable across multiple nodes
- **Fault Tolerant**: Automatic failover and recovery
- **High Performance**: Optimized for both reads and writes
- **Simple API**: Clean key-value interface with range queries
- **Strong Consistency**: Linearizable consistency guarantees

## Quick Start

### Prerequisites

- Rust 1.81 or higher
- Git

### Installation

```bash
# Clone the repository
git clone https://github.com/ferrisdb/ferrisdb.git
cd ferrisdb

# Build the project
cargo build --release

# Run tests (currently 55+ tests passing)
cargo test --all
```

### Basic Usage

```rust
use ferrisdb_client::FerrisDB;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to cluster
    let db = FerrisDB::connect("ferrisdb://localhost:8080").await?;

    // Simple key-value operations
    db.set(b"hello", b"world").await?;
    let value = db.get(b"hello").await?;
    println!("Value: {:?}", value);

    // Transactional operations
    db.run_transaction(|txn| async move {
        let count = txn.get(b"counter").await?.unwrap_or_default();
        let new_count = u64::from_le_bytes(count.try_into().unwrap()) + 1;
        txn.set(b"counter", &new_count.to_le_bytes()).await?;
        Ok(())
    }).await?;

    Ok(())
}
```

## Architecture

FerrisDB uses a distributed architecture with the following components:

- **Transaction Coordinator**: Manages distributed transactions and ensures ACID properties
- **Storage Servers**: Store data using an LSM-tree based storage engine
- **Cluster Controller**: Handles cluster membership and failure detection
- **Client Library**: Provides a simple API for applications

See the [Architecture Documentation](docs/architecture.md) for detailed design documentation.

## Development

### Building from Source

```bash
# Development build
cargo build --all

# Release build with optimizations
cargo build --release --all

# Run with debug logging
RUST_LOG=debug cargo run --bin ferrisdb-server
```

### Running Tests

```bash
# Unit tests
cargo test --all

# Integration tests
cargo test --all --test '*' -- --test-threads=1

# Benchmarks
cargo bench
```

### Contributing

We welcome contributions! Please see our documentation:

- **[CONTRIBUTING.md](CONTRIBUTING.md)** - Contribution guidelines and process
- **[DEVELOPMENT.md](DEVELOPMENT.md)** - Development setup and workflows
- **[CLAUDE.md](CLAUDE.md)** - AI assistant development guidelines

## Performance

FerrisDB targets high performance through careful design:

- **Storage**: Binary search in SSTables for O(log n) lookups
- **Concurrency**: Lock-free skip list for MemTable operations
- **I/O Efficiency**: 4KB block-based storage with checksums
- **Architecture**: Clean separation of concerns for maintainability

## Roadmap

- [x] Design document
- [x] Write-Ahead Log (WAL)
- [x] MemTable with SkipList
- [x] SSTable implementation (Day 2)
- [ ] Compaction system
- [ ] Transaction system
- [ ] Distribution layer
- [ ] Consensus protocol
- [ ] Client libraries
- [ ] Operational tools

## License

FerrisDB is licensed under the Apache License 2.0. See [LICENSE](LICENSE) for details.

## Acknowledgments

This project is inspired by [FoundationDB](https://apple.github.io/foundationdb/) and incorporates ideas from:

- Google Spanner
- Amazon DynamoDB
- CockroachDB
- TiKV

## Documentation

### For Users

- [Getting Started Guide](docs/getting-started.md) - Quick setup and basic usage
- [Architecture Overview](docs/architecture.md) - System design and components

### For Developers

- [CONTRIBUTING.md](CONTRIBUTING.md) - How to contribute to the project
- [DEVELOPMENT.md](DEVELOPMENT.md) - Development environment and workflows
- [CLAUDE.md](CLAUDE.md) - AI-assisted development guidelines

### Technical Deep Dives

- [WAL and Crash Recovery](docs/wal-crash-recovery.md) - Write-ahead logging implementation
- [LSM-Trees Deep Dive](docs/lsm-trees-deep-dive.md) - Storage engine internals

## Repository Setup

For maintainers setting up branch protection:

1. Go to **Settings → Branches → Add rule**
2. Apply settings from `.github/branch-protection.json`
3. Key settings:
   - Require status checks (all CI must pass)
   - Require linear history (squash merge only)
   - Allow maintainers to merge without review
   - Require review for external contributors

See our [contribution documentation](CONTRIBUTING.md) for detailed guidelines.

## Show Your Support

If you find FerrisDB interesting or useful for learning distributed systems:

- ⭐ **Star this repository** to show your support
- 🍴 **Fork the project** to experiment with your own ideas
- 📖 **Read our blog posts** about the development journey
- 💬 **Join discussions** to share ideas and ask questions

## Status

🚧 **This project is under active development as a learning exercise** 🚧

Current focus: Building a custom LSM-tree storage engine from scratch to understand database internals.
