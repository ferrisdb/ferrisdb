# FerrisDB 🦀

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

## Project Status & Roadmap

🚧 **This project is under active development as a learning exercise** 🚧

### Completed ✅
- [x] Design document
- [x] Write-Ahead Log (WAL)
- [x] MemTable with SkipList
- [x] SSTable implementation (Day 2)
- [x] Documentation site with blogs

### In Progress 🔨
- [ ] Compaction system
- [ ] Transaction system
- [ ] Distribution layer

### Future Plans 🚀
- [ ] Consensus protocol (Raft)
- [ ] Client libraries (multiple languages)
- [ ] Operational tools
- [ ] Performance benchmarks

## License - Plain English Version 📜

FerrisDB is licensed under the **Apache License 2.0** - here's what that means in human terms:

### ✅ You CAN:
- **Use it** - For any purpose, even commercially!
- **Modify it** - Make changes to suit your needs
- **Distribute it** - Share the original or modified versions
- **Patent it** - The license includes patent grants
- **Keep it private** - No obligation to share your modifications

### 📋 You MUST:
- **Include the license** - Keep the LICENSE file and notices
- **State changes** - Document what you modified (if you distribute)
- **Include NOTICE** - If we had one (we don't yet!)

### ❌ You CANNOT:
- **Blame us** - Software is "AS IS" with no warranty
- **Use our trademarks** - The FerrisDB name/logo aren't included

### 🤝 In Simple Terms:
"Do whatever you want with this code, just don't blame us if something breaks, and mention where you got it from!"

See [LICENSE](LICENSE) for the full legal text (warning: written by lawyers, not humans 😄).

## Acknowledgments

This project is inspired by [FoundationDB](https://apple.github.io/foundationdb/) and incorporates ideas from:

- Google Spanner - Distributed transactions
- Amazon DynamoDB - Scalability patterns  
- CockroachDB - SQL layer design
- TiKV - Rust implementation patterns

Special thanks to:
- The Rust community for excellent documentation and crates
- Claude (that's me! 🤖) for helping maintain this README and assisting with development
- You, for reading this far!

## Documentation

Visit our [documentation site](https://ferrisdb.org/) for comprehensive guides:

### For Users
- [Getting Started Guide](docs/getting-started.md) - Quick setup and basic usage
- [Architecture Overview](docs/architecture.md) - System design and components
- [FAQ](https://ferrisdb.org/faq/) - Common questions answered

### For Developers
- [CONTRIBUTING.md](CONTRIBUTING.md) - How to contribute to the project
- [DEVELOPMENT.md](DEVELOPMENT.md) - Development environment and workflows
- [CLAUDE.md](CLAUDE.md) - AI-assisted development guidelines

### Technical Deep Dives
- [LSM-Trees Explained](docs/deep-dive/lsm-trees.md) - Storage engine internals
- [WAL and Crash Recovery](docs/deep-dive/wal-crash-recovery.md) - Write-ahead logging implementation
- [Concurrent Skip Lists](docs/deep-dive/concurrent-skip-list.md) - Lock-free data structures
- [SSTable Design](docs/deep-dive/sstable-design.md) - On-disk storage format

### Learning Resources
- [Human Development Blog](https://ferrisdb.org/blog/) - Daily progress from a CRUD developer's perspective
- [Claude's Dev Blog](https://ferrisdb.org/claude-blog/) - AI insights on patterns and collaboration
- [Rust by Example: Database Edition](https://ferrisdb.org/rust-by-example/) - Learn Rust through real database code


## Show Your Support

If you find FerrisDB interesting or useful for learning distributed systems:

- ⭐ **Star this repository** to show your support
- 🍴 **Fork the project** to experiment with your own ideas
- 📖 **Read our blog posts** about the development journey
- 💬 **Join discussions** to share ideas and ask questions
- 🤖 **Learn from Claude** - See how AI assists in real development

## Questions?

- 📧 Open an issue for questions or bug reports
- 💬 Start a discussion for ideas and feedback
- 📚 Check the [FAQ](https://ferrisdb.org/faq/) for common questions

---

*Built with ❤️ and 🦀 by humans and AI working together*
