---
layout: post
title: "Building FerrisDB - Laying the Foundations"
subtitle: "Starting the journey: architecture design, storage engine planning, and implementing WAL + MemTable with concurrent skip lists"
date: 2025-05-27
day: 1
tags: [Architecture, Storage Engine, WAL, MemTable, Rust, Claude Code]
stats: ["📊 13 tests passing", "📄 8 technical PRs merged", "🏗️ WAL + MemTable implementation", "📖 Complete documentation site"]
---

Today marks the beginning of an exciting journey: building a distributed database from scratch to learn Rust and distributed systems concepts. I'm calling it **FerrisDB** (named after Ferris, the Rust mascot), and I'm building it in the open with the help of Claude Code to share the learning experience with the community.

## Why Build a Database?

Building a database is one of the best ways to learn about:

- **Rust systems programming** - Memory management, concurrency, performance
- **Distributed systems** - Consensus, replication, fault tolerance
- **Storage engines** - Data structures, persistence, performance optimization
- **AI-assisted development** - Using Claude Code as a coding partner

I decided to build my own storage engine from scratch rather than using RocksDB to really understand the internals.

## What We Accomplished Today

### 1. Project Architecture & Design

We started with a comprehensive design document outlining FerrisDB's architecture, heavily inspired by FoundationDB:

- **Transaction Coordinator (TC)** - Manages distributed ACID transactions
- **Storage Servers (SS)** - Handle data storage and retrieval
- **Cluster Controller (CC)** - Manages cluster membership and coordination
- **Client Library** - Simple key-value API with transaction support

The full design is documented in our [Architecture Documentation]({{ '/architecture/' | relative_url }}) with detailed explanations of the transaction flow, storage layer, and distributed consensus.

### 2. Rust Workspace Setup

Created a proper Rust workspace structure:

```
ferrisdb/
├── ferrisdb-core/       # Common types and traits
├── ferrisdb-storage/    # Storage engine (our custom LSM-tree)
├── ferrisdb-client/     # Client library
├── ferrisdb-server/     # Server implementation
└── ferrisdb/            # Main binary crate
```

### 3. Storage Engine Design

Instead of using an existing storage engine, we designed our own **LSM-tree (Log-Structured Merge-tree)** implementation:

```
Write Path: Write Request → WAL → MemTable → (Flush) → SSTable
Read Path:  Read Request → MemTable → SSTable (L0 → L1 → L2...)
```

Key components:

- **Write-Ahead Log (WAL)** - Durability and crash recovery
- **MemTable** - In-memory sorted structure (concurrent skip list)
- **SSTables** - Immutable sorted files on disk
- **Compaction** - Background process to merge and optimize SSTables

### 4. WAL Implementation

Built a complete Write-Ahead Log implementation with:

```rust
pub struct WALEntry {
    pub timestamp: u64,
    pub key: Vec<u8>,
    pub value: Option<Vec<u8>>, // None for deletes
    pub operation: Operation,
}
```

Key features:

- **Binary encoding** with little-endian consistency
- **CRC32 checksums** for corruption detection
- **Atomic writes** for crash safety
- **Efficient recovery** by replaying entries

### 5. MemTable with Concurrent Skip List

Implemented a **lock-free concurrent skip list** for the MemTable:

```rust
pub struct MemTable {
    skiplist: Arc<SkipList>,
    size: AtomicUsize,
    size_limit: usize,
}
```

Features:

- **MVCC support** - Multiple versions of the same key
- **Lock-free reads** using crossbeam's epoch-based memory reclamation
- **Timestamp ordering** - Keys sorted by (user_key, timestamp DESC)
- **Atomic operations** for thread safety

The skip list maintains sorted order while allowing concurrent access, which is crucial for database performance.

## Technical Challenges Solved

### 1. Endianness Consistency

Fixed WAL encoding to use little-endian consistently across all integer types for cross-platform compatibility.

### 2. MVCC in Skip List

Designed the key ordering to support multiple versions:

- Primary sort: user key (ascending)
- Secondary sort: timestamp (descending, so newest first)

### 3. Lock-Free Memory Management

Used crossbeam's epoch-based memory reclamation to safely share data between threads without locks.

## Code Quality & Documentation

Following Rust best practices:

- **Comprehensive documentation** for all public APIs
- **Unit tests** for all components (13 tests passing)
- **Error handling** with proper Result types
- **Clippy linting** with zero warnings
- **rustfmt formatting** for consistent style

Created comprehensive development guidelines covering code style, testing, git workflow, and architecture decisions.

## What's Next?

The storage engine foundation is now complete. Next priorities:

1. **SSTable Implementation** - Persistent sorted files with compression
2. **Compaction Strategy** - Background merging and optimization
3. **Bloom Filters** - Probabilistic data structure for faster lookups
4. **Integration Tests** - Multi-threaded concurrent scenarios
5. **Benchmarks** - Performance measurement and optimization

## Lessons Learned

### Working with Claude Code

Building with Claude Code as a pair programming partner has been fascinating:

- **Design-first approach** - Writing comprehensive docs before coding
- **Systematic implementation** - Breaking complex features into manageable pieces
- **Code review mindset** - Claude catches potential issues early
- **Documentation emphasis** - Every public API gets proper docs

### Rust Systems Programming

- **Ownership model** makes concurrent data structures safer but requires careful design
- **Type system** catches many bugs at compile time
- **Performance** is excellent but requires understanding of allocations and data layout
- **Ecosystem** has excellent crates (crossbeam, tokio, thiserror, etc.)

## Building in the Open

This project is intentionally educational and experimental. The goal is to learn and share knowledge, not to build a production database. If you're interested in following along:

- **Code**: [GitHub repository]({{ site.project.repo_url }})
- **Design docs**: Architecture and storage engine design included
- **Blog series**: Daily progress updates

The combination of Rust, distributed systems, and AI-assisted development makes for a unique learning experience. Looking forward to sharing more progress tomorrow!

---

_This is Day 1 of building FerrisDB. Follow along for the complete journey from design to implementation._

**Note**: This blog post was written by Claude Code as part of the AI-assisted development process. The code, decisions, and technical content reflect the collaborative work between human guidance and AI assistance.
