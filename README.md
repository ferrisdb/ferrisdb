# FerrisDB 🦀

[![CI](https://github.com/ferrisdb/ferrisdb/actions/workflows/ci.yml/badge.svg)](https://github.com/ferrisdb/ferrisdb/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Documentation](https://img.shields.io/badge/docs-ferrisdb.org-green.svg)](https://ferrisdb.org)

<img src="docs/src/assets/ferrisdb_logo.svg" alt="FerrisDB Logo" width="120">

A distributed, transactional key-value database written in Rust, inspired by FoundationDB.

> 📚 **Educational Project** | 🌐 **[Documentation](https://ferrisdb.org)**
>
> FerrisDB is an educational project where humans and AI collaborate to:
>
> - Learn distributed systems by building one
> - Implement a real database from scratch in Rust
> - Pioneer human-AI collaborative development
>
> ⚠️ **NOT FOR PRODUCTION USE**
>
> This is a learning project. Components are built for education, not production reliability.
> We prioritize clarity and teaching value over performance and robustness.

## Vision

We're building a distributed database inspired by FoundationDB's architecture. Like any ambitious project, we're starting with the foundation (storage engine) and building up to the full distributed system.

## Current Progress

### ✅ What's Working Now

The storage engine foundation:

- **Write-Ahead Log (WAL)** - Durability with file headers, CRC32 checksums, and metrics
- **MemTable** - Lock-free concurrent skip list with MVCC timestamp support
- **SSTable** - Reader and writer with efficient binary search and block structure
- **BytesMutExt** - Zero-copy optimizations with 23-33% performance improvement

### 🚧 What We're Building

Active development on:

- **Compaction** - Background merging of SSTables
- **Transaction Layer** - ACID transaction support
- **Distribution Layer** - Data partitioning and replication
- **Consensus Protocol** - Likely Raft for coordination

### 🎯 The End Goal

A fully functional distributed database with:

- **ACID Transactions** - True serializable isolation
- **Horizontal Scalability** - Add nodes to scale out
- **Fault Tolerance** - Automatic failover and recovery
- **Strong Consistency** - Linearizable operations
- **Simple API** - Clean key-value interface

## Quick Start

```bash
# Clone and build
git clone https://github.com/ferrisdb/ferrisdb.git
cd ferrisdb
cargo build --all

# Run tests
cargo test --all

# Explore the code
cargo doc --all --open
```

## Architecture

FerrisDB follows FoundationDB's layered architecture:

```
┌─────────────────────────────────────┐
│         Client Library              │
├─────────────────────────────────────┤
│     Transaction Coordinator         │  ← In Development
├─────────────────────────────────────┤
│      Storage Servers                │  ← Working on this!
├─────────────────────────────────────┤
│    Cluster Controller & Consensus   │  ← Planned
└─────────────────────────────────────┘
```

Currently implementing the Storage Server layer with an LSM-tree engine.

## The Human-AI Collaboration Experiment

FerrisDB is unique: it's being built through genuine collaboration between human developers and AI. This isn't about AI generating code - it's about two different types of intelligence working together, each bringing their strengths:

- **Human**: Architecture vision, design decisions, "this feels wrong" intuition
- **AI**: Implementation details, edge case handling, systematic analysis

Read our [development blog](https://ferrisdb.org/blog/) to see this collaboration in action!

## Documentation

- **[Website](https://ferrisdb.org)** - Full documentation, tutorials, and blog
- **[Current Status](https://ferrisdb.org/status/)** - What's actually built today
- **[Architecture Overview](https://ferrisdb.org/reference/future-architecture/)** - System design

### Learning Resources

- **[Development Blog](https://ferrisdb.org/blog/)** - Human and AI perspectives on building FerrisDB
- **[Tutorials](https://ferrisdb.org/tutorials/)** - Learn by building database components
- **[Database Concepts](https://ferrisdb.org/concepts/)** - Deep dives into database internals

## Learn by Building

We offer hands-on tutorials where you build database components from scratch:

- **[Tutorial 01: Key-Value Store](https://ferrisdb.org/tutorials/01-key-value-store/)** - Build a simple in-memory store with HashMap (✅ Published)
- More tutorials coming soon! Check our [tutorial roadmap](https://ferrisdb.org/tutorials/)

## Contributing

We welcome contributions from both humans and AI!

- 📖 Read our [Contributing Guide](CONTRIBUTING.md)
- 🏗️ Check the [Development Setup](DEVELOPMENT.md)
- 🤖 AI contributors: See [CLAUDE.md](CLAUDE.md)
- 🏷️ Browse [open issues](https://github.com/ferrisdb/ferrisdb/issues)
- 💬 Join discussions in [pull requests](https://github.com/ferrisdb/ferrisdb/pulls)

## Roadmap

### Phase 1: Storage Engine ✅ (Core Complete)

- [x] Write-Ahead Log (with proper format)
- [x] MemTable with SkipList
- [x] SSTable writer & reader
- [x] MVCC timestamps
- [ ] Storage engine integration (in progress)
- [ ] Compaction (next up)
- [ ] Bloom filters

### Phase 2: Transaction System 🚧 (Starting Soon)

- [ ] MVCC implementation
- [ ] Transaction coordinator
- [ ] Snapshot isolation
- [ ] Serializable transactions

### Phase 3: Distribution Layer 📋 (Planned)

- [ ] Data partitioning
- [ ] Replication protocol
- [ ] Failure detection
- [ ] Automatic recovery

### Phase 4: Consensus & Coordination 🔮 (Future)

- [ ] Raft consensus
- [ ] Cluster controller
- [ ] Configuration management
- [ ] Client routing

## Why FerrisDB?

FerrisDB is **not** trying to be the next production database. It's:

1. **📚 A Learning Platform** - Watch a database being built from scratch
2. **🤝 A Collaboration Experiment** - Pioneering human-AI development
3. **🦀 A Rust Teaching Tool** - Learn Rust through real systems programming
4. **📖 Open Documentation** - Every decision explained, every mistake shared

**If you need a production database**, use PostgreSQL, SQLite, or FoundationDB.  
**If you want to learn how databases work**, you're in the right place!

## Project Statistics

- **Lines of Code**: ~2,400 (implementation) + ~3,500 (tests)
- **Test Coverage**: 85%+ for core components
- **Blog Posts**: 8 (4 human, 4 Claude)
- **Tutorials**: 1 published, 9 planned
- **Contributors**: Growing community of humans and AI

## Recent Highlights

- ✅ Implemented WAL with comprehensive testing
- ✅ Published Tutorial 01: Building a Key-Value Store
- ✅ Established governance and contribution guidelines
- ✅ 8 blog posts documenting our journey
- ✅ Growing community with organized issue tracking

## License

Apache License 2.0 - see [LICENSE](LICENSE) for details.

**TL;DR**: Use it for learning, experimentation, or anything else - just don't blame us if it breaks! 😄

## Acknowledgments

Standing on the shoulders of giants:

- [FoundationDB](https://apple.github.io/foundationdb/) - Architectural inspiration
- [RocksDB](https://rocksdb.org/) - LSM-tree wisdom
- The Rust community - Incredible ecosystem and support

Special thanks to all contributors - both human and AI - who are making this experiment possible! 🦀🤖

---

_Join us in building the future of collaborative software development!_
