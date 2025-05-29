---
layout: default
title: Architecture Design
nav_order: 8
permalink: /architecture/
---

Comprehensive design document for FerrisDB's distributed database architecture
{: .fs-6 .fw-300 }

## Table of contents

{: .no_toc .text-delta }

1. TOC
{:toc}

---

## Overview

FerrisDB is a distributed, transactional key-value database inspired by FoundationDB. It aims to provide:

- ACID transactions with serializable isolation
- Horizontal scalability
- Fault tolerance with automatic recovery
- Simple key-value API with range queries
- Strong consistency guarantees

## Architecture Overview

### Core Components

1. **Transaction Coordinator (TC)**

   - Manages distributed transactions
   - Assigns transaction timestamps
   - Handles conflict resolution
   - Implements MVCC (Multi-Version Concurrency Control)

2. **Storage Servers (SS)**

   - Store actual key-value data
   - Handle reads and writes
   - Manage local LSM-tree storage engine
   - Participate in distributed transactions

3. **Cluster Controller (CC)**

   - Manages cluster membership
   - Handles failure detection
   - Coordinates data rebalancing
   - Maintains system metadata

4. **Proxy Servers**
   - Route client requests
   - Cache transaction state
   - Provide load balancing

## Data Model

### Key-Value Store

- Keys: Variable-length byte arrays (max 10KB)
- Values: Variable-length byte arrays (max 100KB)
- Operations: Get, Set, Clear, GetRange
- Atomic operations: Add, BitAnd, BitOr, BitXor, Max, Min

### Namespace Design

```
/system/...              # System metadata
/user/...                # User data
/internal/coord/...      # Coordination data
/internal/txn/...        # Transaction metadata
```

## Transaction System

### MVCC Implementation

- Each key-value pair has multiple versions
- Versions identified by transaction commit timestamp
- Read timestamp determines visible version
- Garbage collection removes old versions

### Transaction Flow

1. **Begin**: Client contacts TC for start timestamp
2. **Read**: Client reads from SS at start timestamp
3. **Write**: Client buffers writes locally
4. **Commit**:
   - Client sends write set to TC
   - TC checks conflicts
   - TC assigns commit timestamp
   - TC coordinates 2PC with involved SS

### Conflict Detection

- Write-write conflicts: Two transactions modify same key
- Optimistic concurrency control
- First-writer-wins with retry logic

## Storage Layer

### LSM-Tree Architecture

- Write-ahead log (WAL) for durability
- MemTable for recent writes
- SSTables for persistent storage
- Compaction strategy: Leveled compaction

### Data Distribution

- Consistent hashing for key distribution
- Virtual nodes for load balancing
- Replication factor: 3 (configurable)
- Quorum reads/writes: W+R > N

### Storage Format

```rust
// Key format
struct Key {
    user_key: Vec<u8>,
    timestamp: u64,
    type: KeyType, // Put, Delete
}

// Value format
struct Value {
    data: Vec<u8>,
    checksum: u32,
}
```

## Distributed Coordination

### Consensus Protocol

- Raft for cluster coordination
- Leader election for CC role
- Configuration changes through joint consensus

### Failure Detection

- Heartbeat mechanism
- Configurable timeout (default: 5s)
- Phi Accrual Failure Detector

### Recovery

- Automatic failover for failed nodes
- Re-replication to maintain replication factor
- Transaction recovery from WAL

## Network Protocol

### Transport

- TCP with custom framing
- Protocol Buffers for serialization
- Connection pooling
- TLS support for encryption

### Message Types

```proto
message Request {
    oneof request_type {
        BeginTransaction begin;
        ReadRequest read;
        CommitRequest commit;
        HeartbeatRequest heartbeat;
    }
}
```

## Client API

### Rust Client Library

```rust
// Basic operations
async fn example() -> Result<()> {
    let db = FerrisDB::connect("cluster://localhost:8080").await?;

    // Simple KV operations
    db.set(b"key", b"value").await?;
    let value = db.get(b"key").await?;

    // Transactions
    db.run_transaction(|txn| async move {
        let val = txn.get(b"counter").await?;
        let new_val = increment(val);
        txn.set(b"counter", new_val);
        Ok(())
    }).await?;

    Ok(())
}
```

## Performance Targets

- Latency: <10ms p99 for single-key operations
- Throughput: 100K+ ops/sec per node
- Scalability: Linear scaling up to 100 nodes
- Recovery: <30s for single node failure

## Implementation Phases

### Phase 1: Core Storage

- [x] Basic key-value storage engine
- [x] LSM-tree implementation
- [x] WAL for durability

### Phase 2: Transactions

- [ ] MVCC implementation
- [ ] Transaction coordinator
- [ ] Conflict detection

### Phase 3: Distribution

- [ ] Data partitioning
- [ ] Replication
- [ ] Failure detection

### Phase 4: Consensus

- [ ] Raft implementation
- [ ] Cluster management
- [ ] Configuration changes

### Phase 5: Client Libraries

- [ ] Rust client
- [ ] Transaction retry logic
- [ ] Connection pooling

## Testing Strategy

### Unit Tests

- Storage engine correctness
- Transaction isolation
- Network protocol

### Integration Tests

- Multi-node scenarios
- Failure injection
- Performance benchmarks

### Chaos Testing

- Network partitions
- Node failures
- Clock skew

## Open Questions

1. Should we implement our own Raft or use an existing library?
2. What compression algorithm for SSTables?
3. How to handle hot keys?
4. Should we support SQL layer in future?

## References

- FoundationDB Architecture
- Google Spanner Paper
- Amazon DynamoDB Paper
- CockroachDB Design Docs

---

**Related Documentation:**

- [Storage Engine Design]({{ '/storage-engine/' | relative_url }})
- [Future Architecture Explorations]({{ '/future-architecture/' | relative_url }})

**Technical Deep Dives:**

- [Understanding WAL and Crash Recovery]({{ '/deep-dive/wal-crash-recovery/' | relative_url }})
- [LSM-Trees and Storage Engine Design]({{ '/deep-dive/lsm-trees/' | relative_url }})

**Project Links:**

- [GitHub Repository]({{ site.project.repo_url }})
- [Development Blog]({{ '/blog/' | relative_url }})
