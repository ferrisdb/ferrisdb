# FerrisDB 🦀

<img src="docs/assets/images/ferrisdb_logo.svg" alt="FerrisDB Logo" width="120">

A distributed, transactional key-value database written in Rust, inspired by FoundationDB.

> 📚 **Educational Project**
>
> FerrisDB is an educational project where humans and AI collaborate to:
>
> - Learn distributed systems by building one
> - Implement a real database from scratch in Rust
> - Pioneer human-AI collaborative development
>
> **Not recommended for production use** - this is a learning journey!

## Vision

We're building a distributed database inspired by FoundationDB's architecture. Like any ambitious project, we're starting with the foundation (storage engine) and building up to the full distributed system.

## Current Progress

### ✅ What's Working Now

The storage engine foundation:
- **Write-Ahead Log (WAL)** - Durability and crash recovery
- **MemTable** - Lock-free concurrent skip list for in-memory operations
- **SSTable** - Persistent sorted string tables with binary search
- **MVCC Timestamps** - Multi-version concurrency control preparation

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

Read our [development blogs](https://ferrisdb.org/blog/) to see this collaboration in action!

## Documentation

- **[Getting Started](docs/getting-started.md)** - Build and run FerrisDB
- **[Architecture](docs/architecture.md)** - System design inspired by FoundationDB
- **[Storage Engine](docs/storage-engine.md)** - Current LSM-tree implementation

### For Contributors

- **[CONTRIBUTING.md](CONTRIBUTING.md)** - How to contribute (humans & AI welcome!)
- **[DEVELOPMENT.md](DEVELOPMENT.md)** - Technical development guide
- **[CLAUDE.md](CLAUDE.md)** - Guidelines for AI contributors

### Learning Resources

- **[Human Dev Blog](https://ferrisdb.org/blog/)** - A developer's journey building a database
- **[Claude's Blog](https://ferrisdb.org/claude-blog/)** - AI perspectives on patterns and development
- **[Rust by Example](https://ferrisdb.org/rust-by-example/)** - Learn Rust through real database code

## Roadmap

### Phase 1: Storage Engine ✅ (Mostly Complete)
- [x] Write-Ahead Log
- [x] MemTable with SkipList
- [x] SSTable implementation
- [ ] Compaction (in progress)
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

## Why Another Database?

Good question! We're not trying to compete with production databases. FerrisDB exists to:

1. **Learn by Doing** - The best way to understand databases is to build one
2. **Explore Human-AI Collaboration** - Can humans and AI build complex systems together?
3. **Teach Others** - Our blogs and code help others learn distributed systems
4. **Have Fun** - Building databases is surprisingly enjoyable!

## License

Apache License 2.0 - see [LICENSE](LICENSE) for details.

**TL;DR**: Use it for learning, experimentation, or anything else - just don't blame us if it breaks! 😄

## Acknowledgments

Standing on the shoulders of giants:
- [FoundationDB](https://apple.github.io/foundationdb/) - Architectural inspiration
- [RocksDB](http://rocksdb.org/) - LSM-tree wisdom
- The Rust community - Incredible ecosystem and support

Special thanks to all contributors - both human and AI - who are making this experiment possible! 🦀🤖

---

_Join us in building the future of collaborative software development!_