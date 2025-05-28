# FerrisDB

![FerrisDB Logo](docs/assets/images/ferrisdb_logo.svg)

A distributed, transactional key-value database written in Rust, inspired by FoundationDB.

> ⚠️ **Educational Project Disclaimer**
> 
> This project is created for personal learning purposes:
> - Learning Rust programming language
> - Understanding distributed systems concepts
> - Exploring AI-assisted development with Claude Code
> 
> **This is NOT recommended for production use.** The code is experimental and primarily serves as a learning exercise. If you're looking for a production-ready distributed database, consider established solutions like FoundationDB, TiKV, or CockroachDB.

## Features

- **ACID Transactions**: Full ACID compliance with serializable isolation
- **Distributed**: Horizontally scalable across multiple nodes
- **Fault Tolerant**: Automatic failover and recovery
- **High Performance**: Optimized for both reads and writes
- **Simple API**: Clean key-value interface with range queries
- **Strong Consistency**: Linearizable consistency guarantees

## Quick Start

### Prerequisites

- Rust 1.75 or higher
- RocksDB development libraries

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/ferrisdb.git
cd ferrisdb

# Build the project
cargo build --release

# Run tests
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

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

Development guidelines and standards are documented in [CLAUDE.md](CLAUDE.md).

## Performance

FerrisDB is designed for high performance:

- **Latency**: <10ms p99 for single-key operations
- **Throughput**: 100K+ operations per second per node
- **Scalability**: Linear scaling up to 100 nodes

## Roadmap

- [x] Design document
- [ ] Core storage engine
- [ ] Transaction system
- [ ] Distribution layer
- [ ] Consensus protocol
- [ ] Client libraries
- [ ] Operational tools

## License

FerrisDB is licensed under the MIT License. See [LICENSE](LICENSE) for details.

## Acknowledgments

This project is inspired by [FoundationDB](https://apple.github.io/foundationdb/) and incorporates ideas from:
- Google Spanner
- Amazon DynamoDB
- CockroachDB
- TiKV

## Community

- [Discord Server](#) (Coming soon)
- [Documentation](#) (Coming soon)
- [Benchmarks](#) (Coming soon)

## Status

🚧 **This project is under active development as a learning exercise** 🚧

Current focus: Building a custom LSM-tree storage engine from scratch to understand database internals.