# Architecture Decisions

Key architectural decisions and design principles for FerrisDB.

## System Architecture

### Overview

FerrisDB is a distributed, transactional key-value database with the following architecture:

```
┌─────────────────┐     ┌─────────────────┐
│ Client Library  │     │ Client Library  │
└────────┬────────┘     └────────┬────────┘
         │                       │
         └───────────┬───────────┘
                     │
            ┌────────▼────────┐
            │  Server (gRPC)  │
            └────────┬────────┘
                     │
            ┌────────▼────────┐
            │ Storage Engine  │
            │   (LSM-tree)    │
            └────────┬────────┘
                     │
          ┌──────────┼──────────┐
          │          │          │
     ┌────▼────┐ ┌───▼────┐ ┌───▼────┐
     │MemTable │ │SSTables│ │  WAL   │
     └─────────┘ └────────┘ └────────┘
```

### Component Boundaries

1. **ferrisdb-core**: Common types and traits
2. **ferrisdb-storage**: Storage engine implementation
3. **ferrisdb-server**: Network server and API
4. **ferrisdb-client**: Client library

## Design Principles

### 1. Educational First

- Code should be clear and understandable
- Prefer explicit over implicit
- Document why, not just what
- Show database concepts in action

### 2. Safety Over Performance

During the learning phase:

- Avoid `unsafe` code unless absolutely necessary
- Use safe abstractions even if slower
- Make performance optimizations explicit and documented

### 3. Modular Design

- Clear separation of concerns
- Well-defined interfaces between components
- Each crate should have a single responsibility
- Dependencies flow in one direction

## Key Decisions

### Storage Engine: LSM-Tree

**Why LSM-tree over B-tree:**

- Better write performance
- Natural fit for append-only operations
- Demonstrates compaction concepts
- Used by RocksDB, LevelDB, Cassandra

**Trade-offs:**

- More complex read path
- Requires background compaction
- Space amplification

### Concurrency: Lock-Free Skip List

**Why skip list for MemTable:**

- Lock-free concurrent operations
- Good cache locality
- Simpler than lock-free B-trees
- Educational value

**Implementation:**

- No `unsafe` code (learning focus)
- Arc-based node management
- Eventually optimize if needed

### Persistence: Write-Ahead Log

**Design choices:**

- Simple append-only format
- CRC32 checksums for integrity
- Synchronous writes by default
- Binary format for efficiency

### API: gRPC

**Why gRPC:**

- Language-agnostic clients
- Built-in code generation
- Streaming support
- Industry standard

## Future Considerations

### Distribution

When we add distribution:

- Raft for consensus
- Range-based sharding
- Learner replicas
- Multi-version concurrency control

### Transactions

Transaction design:

- Optimistic concurrency control
- Snapshot isolation
- Multi-key transactions
- Deterministic transaction ordering

## Non-Goals

Things we explicitly don't optimize for:

1. **SQL compatibility** - We're a key-value store
2. **Embedded use** - We're building a server
3. **Maximum performance** - We prioritize learning
4. **Production readiness** - This is educational

## Invariants

See [System Invariants](invariants.md) for critical properties that must be maintained.

## References

- [Google's LevelDB Design](https://github.com/google/leveldb/blob/main/doc/impl.md)
- [RocksDB Architecture](https://github.com/facebook/rocksdb/wiki/RocksDB-Overview)
- [FoundationDB Design](https://apple.github.io/foundationdb/)
- [Apache Cassandra Architecture](https://cassandra.apache.org/doc/latest/architecture/)
