# System Invariants

Critical invariants that must be maintained throughout FerrisDB's implementation.

**Prerequisites**: Understanding of database ACID properties and distributed systems concepts  
**Related**: [Architecture](architecture.md), [Storage Engine](storage-engine.md), [Testing](../workflow/testing.md)

## Key System Invariants

1. **Transactions must be serializable** [FUTURE]

   - All transactions execute as if they ran in some serial order
   - No dirty reads, non-repeatable reads, or phantom reads
   - Isolation level guarantees must be maintained

2. **All writes must be durable before acknowledgment** [IMPLEMENTED]

   - Data written to WAL before responding to client
   - fsync() called before acknowledging writes
   - No data loss on process crash after acknowledgment

3. **Node failures must not cause data loss** [FUTURE]

   - Replicated data survives node failures
   - Quorum writes ensure durability
   - Recovery procedures restore full functionality

4. **Reads must see a consistent snapshot** [PARTIAL]

   - Point-in-time consistency for all reads
   - No partial updates visible
   - Snapshot isolation for read transactions
   
   **Current Status:** MVCC timestamps implemented, full snapshot isolation planned

5. **WAL entries must be written before MemTable updates** [ENFORCED]

   - Strict write ordering: WAL → MemTable → Response
   - Recovery relies on this ordering
   - No in-memory updates without persistent log

6. **Timestamps must be monotonically increasing** [ENFORCED]
   - No timestamp regression within a node
   - Lamport timestamps or similar for distributed ordering [FUTURE]
   - Critical for MVCC correctness

## Storage Engine Invariants

1. **Keys in MemTable are sorted by (user_key, timestamp DESC)** [ENFORCED]

   - Enables efficient range scans
   - Latest version appears first
   - Binary search possible on keys

2. **Multiple versions of same key ordered by timestamp** [ENFORCED]

   - MVCC requires version history
   - Newer versions shadow older ones
   - Timestamp ordering is strict

3. **Delete operations create tombstones (not immediate deletion)** [ENFORCED]

   - Deletes are special write operations
   - Tombstones removed during compaction [PLANNED]
   - Necessary for distributed consistency

4. **Compaction removes obsolete versions and tombstones** [PLANNED]

   - Old versions beyond retention removed
   - Tombstones removed after grace period
   - Storage reclamation happens here

5. **All disk writes include checksums** [ENFORCED]
   - Data integrity verification
   - Corruption detection on read
   - Checksum mismatch triggers recovery

## Concurrency Invariants

1. **Lock-free data structures maintain consistency** [PARTIAL]

   - Skip list operations are atomic [ENFORCED]
   - No ABA problems in updates [ENFORCED]
   - Memory reclamation is safe [Using Arc, no unsafe]

2. **Reference counting prevents use-after-free** [ENFORCED]

   - Arc/Rc for shared ownership
   - Epoch-based reclamation for lock-free structures [FUTURE]
   - No dangling pointers

3. **Atomic operations for critical counters** [ENFORCED]
   - Sequence numbers use atomics
   - Reference counts are atomic
   - Stats counters don't cause races

## Network Protocol Invariants [FUTURE]

1. **Request ordering preserved per connection**

   - FIFO processing within connection
   - Pipelining maintains order
   - Response order matches request order

2. **Idempotent operations are retry-safe**

   - Conditional writes check versions
   - Read operations always idempotent
   - Client can safely retry on timeout

3. **Connection state properly cleaned up**
   - Resources freed on disconnect
   - Transactions aborted on connection loss
   - No resource leaks

## Distributed System Invariants [FUTURE]

1. **Consensus decisions are permanent**

   - Once committed, never reverted
   - Majority agreement required
   - Split-brain prevention

2. **Replication maintains causal consistency**

   - Happens-before relationships preserved
   - Vector clocks or similar mechanism
   - No causal anomalies

3. **Membership changes are linearizable**
   - One configuration active at a time
   - Safe transitions between configurations
   - No data loss during reconfiguration

## Recovery Invariants

1. **WAL replay restores exact state** [PARTIAL]

   - Deterministic replay process [IMPLEMENTED]
   - Same final state as before crash [IMPLEMENTED]
   - Idempotent log application [PLANNED]

2. **Checkpoints are consistent snapshots** [PLANNED]

   - Atomic checkpoint creation
   - All data structures consistent
   - Can restore from checkpoint + WAL

3. **Recovery completes in bounded time** [PLANNED]
   - Linear in WAL size
   - Checkpoints limit replay work
   - Progress monitoring possible

## Performance Invariants

1. **Read latency independent of data size** [PARTIAL]

   - O(log n) lookup in skip list [ENFORCED]
   - Index structures maintain efficiency [PARTIAL]
   - No full scans for point queries [ENFORCED]

2. **Write latency bounded by WAL sync** [ENFORCED]

   - Bottleneck is disk fsync
   - Batching amortizes sync cost [PLANNED]
   - Predictable latency profile

3. **Memory usage proportional to working set** [PARTIAL]
   - Cold data evicted to disk [PARTIAL]
   - Compaction controls growth [PLANNED]
   - No unbounded memory growth [ENFORCED via MemTable limits]

## Safety Invariants

1. **No undefined behavior in safe code** [ENFORCED]

   - All unsafe blocks documented [No unsafe code currently]
   - Safety requirements explicit
   - Fuzzing catches violations [Via PropTest]

2. **Resource limits enforced** [PARTIAL]

   - Maximum transaction size [PLANNED]
   - Connection count limits [FUTURE]
   - Memory usage bounds [ENFORCED for MemTable]

3. **Error handling never panics** [ENFORCED]
   - All errors propagated as Result
   - Panics only in true bugs
   - Graceful degradation

## Maintaining Invariants

### During Development

1. **Add assertions for invariants**

   ```rust
   debug_assert!(self.keys_are_sorted());
   debug_assert!(timestamp > self.last_timestamp);
   ```

2. **Write invariant-checking tests**

   ```rust
   #[test]
   fn test_memtable_maintains_sort_order() {
       // Test that invariant holds after operations
   }
   ```

3. **Document invariants in code**

   ```rust
   /// Invariant: Keys are always sorted by (user_key, timestamp DESC)
   /// This is required for correct MVCC operation
   struct MemTable { ... }
   ```

### During Review

- Check that changes preserve invariants
- Verify new features don't break assumptions
- Ensure error paths maintain consistency
- Look for race conditions that violate invariants

### During Testing

- Property-based tests check invariants
- Chaos testing verifies distributed invariants
- Stress tests ensure concurrency invariants
- Recovery tests validate durability invariants

## Invariant Violations

### Detection

1. **Runtime checks in debug builds**
2. **Continuous invariant monitoring**
3. **Post-mortem analysis tools**
4. **Integration test coverage**

### Response

1. **Log detailed diagnostic information**
2. **Fail fast to prevent corruption**
3. **Trigger recovery procedures**
4. **Alert operators for investigation**

## Documentation Requirements

When adding new features:

1. **List new invariants introduced**
2. **Explain why invariant is necessary**
3. **Document how invariant is maintained**
4. **Add tests that verify invariant**
5. **Update this document**

## Examples of Invariant Documentation

```rust
/// Skip list node structure
///
/// # Invariants
///
/// 1. Node heights are immutable after creation
/// 2. Forward pointers at level i point to nodes at level >= i
/// 3. Nodes are ordered by key at every level
/// 4. Tower heights follow geometric distribution
///
/// These invariants ensure O(log n) operations and prevent
/// corruption during concurrent access.
struct Node<K, V> {
    // ...
}
```

## Conclusion

These invariants form the foundation of FerrisDB's correctness. Every code change must preserve these properties. When in doubt, err on the side of safety and add more checks rather than fewer.

---
_Last updated: 2025-06-01_
