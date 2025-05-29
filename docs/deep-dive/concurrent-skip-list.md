---
layout: page
title: "Lock-Free Skip Lists: Building a Concurrent MemTable"
subtitle: "How FerrisDB implements a thread-safe in-memory storage with lock-free reads"
permalink: /deep-dive/concurrent-skip-list/
tags: [database, concurrency, skip-list, lock-free, memtable]
difficulty: intermediate
estimated_reading: "20 minutes"
ferrisdb_components: [memtable, skiplist]
prerequisites: [/deep-dive/lsm-trees/]
---

## The Problem & Why It Matters

Imagine your web application suddenly goes viral. Thousands of users are simultaneously creating accounts, updating profiles, and posting content. Your database needs to handle all these writes at the same time without making users wait in line.

This is the concurrency challenge every modern database faces. In traditional systems, when multiple users try to write data:

**Real-world problems CRUD developers face:**

- **Lock contention**: Users wait in a virtual queue, like a single bathroom at a party
- **Deadlocks**: Two users waiting for each other, like two cars at a narrow bridge
- **Slow response times**: Your API endpoints timeout during traffic spikes
- **Wasted server resources**: CPUs sit idle while threads wait for locks

The traditional solution uses locks (mutexes) to ensure only one thread modifies data at a time:

```rust
// Naive approach - everyone waits in line
struct SlowDatabase {
    data: Mutex<HashMap<String, String>>,
}

// Every operation locks EVERYTHING
db.data.lock().unwrap().insert(key, value); // All other threads wait!
```

This is like having a single cashier at a busy store - no matter how many customers arrive, they all wait in one line. FerrisDB's lock-free skip list solves this by allowing multiple "cashiers" to work simultaneously.

## Conceptual Overview

### The Core Idea

Skip lists are like a subway system for your data:

**Regular linked list** (local train):

```
Station1 → Station2 → Station3 → Station4 → Station5
```

**Skip list** (express system):

```
Express:    Station1 -----------→ Station3 -----------→ Station5
Local:      Station1 → Station2 → Station3 → Station4 → Station5
```

To find Station4:

1. Take express to Station3 (skip Station2)
2. Switch to local for one stop

This makes finding data much faster - O(log n) instead of O(n).

### Visual Architecture

```text
Level 3: HEAD ------------------> 30 -------------------------> NULL
Level 2: HEAD ------> 10 -------> 30 -------> 50 -------------> NULL
Level 1: HEAD -> 5 -> 10 -> 20 -> 30 -> 40 -> 50 -> 60 -------> NULL
Level 0: HEAD -> 5 -> 10 -> 20 -> 30 -> 40 -> 50 -> 60 -> 70 -> NULL
           ↑                          ↑                      ↑
        Start here              Found quickly!          Without skipping
```

**Key principles:**

1. **Multiple levels**: Express lanes for faster traversal
2. **Probabilistic structure**: Randomly decide how many levels each node gets
3. **Lock-free reads**: Multiple threads can search simultaneously

## FerrisDB Implementation Deep Dive

### Core Data Structures

Let's examine FerrisDB's concurrent skip list implementation:

```rust
// ferrisdb-storage/src/memtable/skip_list.rs:124-147
pub struct SkipList {
    /// Sentinel head node
    head: Atomic<Node>,
    /// Current height of the skip list
    height: AtomicUsize,
    /// Number of entries in the skip list
    size: AtomicUsize,
    /// Random number generator for determining node heights
    rng: Mutex<rand::rngs::StdRng>,
}

struct Node {
    key: InternalKey,
    value: Value,
    timestamp: Timestamp,
    next: Vec<Atomic<Node>>,  // One pointer per level
    level: usize,
}
```

**Key design decisions:**

1. **Atomic pointers**: Enable lock-free operations using atomic CPU instructions
2. **Epoch-based reclamation**: Safely free memory without locks
3. **Height randomization**: Maintains balance probabilistically (no rebalancing needed)

### Implementation Details

#### Lock-Free Search Operation

The beauty of skip lists is that searches never need locks:

```rust
// ferrisdb-storage/src/memtable/skip_list.rs:326-354
pub fn get(&self, user_key: &[u8], timestamp: Timestamp) -> Option<(Value, Operation)> {
    let guard = &epoch::pin();  // Memory safety guard

    // Start from highest level and work down
    let mut current = self.head.load(Ordering::Acquire, guard);

    // Search each level from top to bottom
    for level in (0..self.height.load(Ordering::Acquire)).rev() {
        // Move forward on current level while key is smaller
        while let Some(node) = unsafe { current.as_ref() } {
            let next = node.next[level].load(Ordering::Acquire, guard);

            if let Some(next_node) = unsafe { next.as_ref() } {
                if next_node.key.user_key < user_key {
                    current = next;  // Keep going on this level
                } else {
                    break;  // Drop down a level
                }
            }
        }
    }

    // Now we're at the right position - check for exact match
    // ... (version checking logic)
}
```

**How it works:**

1. **No locks needed**: Just reading pointers atomically
2. **Top-down search**: Like taking express trains first
3. **Multiple readers**: Thousands of threads can search simultaneously

**Performance characteristics:**

- **Time complexity**: O(log n) average case for all operations
- **Space complexity**: O(n) with small constant factor (~1.33)
- **Concurrency**: Lock-free reads, fine-grained locking for writes

#### Concurrent Insert Operation

Insertions are more complex but still allow high concurrency:

```rust
// ferrisdb-storage/src/memtable/skip_list.rs:175-257 (simplified)
pub fn insert(&self, user_key: Key, value: Value, timestamp: Timestamp, operation: Operation) {
    let guard = &epoch::pin();
    let key = InternalKey::new(user_key, timestamp, operation);

    // 1. Randomly determine height (like flipping coins)
    let height = self.random_height();

    // 2. Find insertion position at each level
    let mut preds: Vec<Shared<Node>> = vec![Shared::null(); height];
    let mut succs: Vec<Shared<Node>> = vec![Shared::null(); height];

    self.find(&key, &mut preds, &mut succs, guard);

    // 3. Create new node
    let new_node = Owned::new(Node::new(key, value, height));

    // 4. Link the node (bottom-up for correctness)
    // First link at level 0, then work up
    let new_node_shared = new_node.into_shared(guard);

    // Compare-and-swap at level 0
    match preds[0].next[0].compare_exchange(
        succs[0],
        new_node_shared,
        Ordering::Release,
        guard
    ) {
        Ok(_) => {
            // Success! Link higher levels
            for i in 1..height {
                // Link at each level (may retry if concurrent modification)
            }
        }
        Err(_) => {
            // Someone else inserted here, retry
        }
    }
}
```

**Why this works well:**

1. **Optimistic concurrency**: Try to insert, retry only on conflict
2. **Bottom-up linking**: Ensures consistency even with concurrent operations
3. **Compare-and-swap**: Atomic operation prevents race conditions

## Performance Analysis

### Mathematical Analysis

**Skip List Properties:**

- **Average height**: 1/(1-p) where p = probability of going up (typically 0.25)
- **Expected search time**: O(log n) with high probability
- **Space overhead**: Average 1.33 pointers per node (vs 2 for balanced trees)

**Concurrency Benefits:**

- **Read parallelism**: Unlimited concurrent readers
- **Write parallelism**: Conflicts only for same key insertions
- **No global locks**: Fine-grained synchronization

### Trade-off Analysis

**Advantages:**

- ✅ **Lock-free reads**: Perfect scaling for read-heavy workloads
- ✅ **Simple implementation**: Easier than lock-free B-trees
- ✅ **No rebalancing**: Probabilistic balance means no complex rotations
- ✅ **Cache-friendly**: Sequential memory access patterns

**Disadvantages:**

- ⚠️ **Probabilistic guarantees**: Worst case O(n) is possible (but extremely rare)
- ⚠️ **Memory overhead**: Extra pointers for skip list levels
- ⚠️ **Complex memory management**: Epoch-based reclamation adds complexity
- ⚠️ **Non-deterministic**: Random heights mean unpredictable structure

**When to use alternatives:**

- **Deterministic performance needed**: Use B-trees or AVL trees
- **Memory constrained**: Simple sorted arrays might be better
- **Single-threaded**: No need for concurrency complexity

## Advanced Topics

### Epoch-Based Memory Reclamation

The trickiest part of lock-free programming is safely freeing memory:

```rust
// Using crossbeam's epoch-based reclamation
let guard = &epoch::pin();  // Pin thread to current epoch

// Safe to access any node through guard
let node = ptr.load(Ordering::Acquire, guard);

// Mark for deletion (happens in next epoch)
unsafe { guard.defer_destroy(old_node); }

// Memory freed when all threads move past this epoch
```

**How epochs work:**

1. **Global epoch counter**: Advances periodically
2. **Thread pinning**: Each thread declares which epoch it's in
3. **Safe deletion**: Memory freed only when all threads advance

### Memory Ordering and Atomics

FerrisDB uses specific memory ordering for correctness:

```rust
// Acquire: See all writes before the release
let next = node.next[level].load(Ordering::Acquire, guard);

// Release: Make all previous writes visible
node.next[level].store(new_node, Ordering::Release);

// Compare-and-swap: Full synchronization
next[0].compare_exchange(old, new, Ordering::Release, Ordering::Acquire, guard)
```

**Why ordering matters:**

- **Prevents reordering**: CPU can't optimize away safety
- **Establishes happens-before**: Ensures correct visibility across threads
- **Minimal overhead**: Only as strong as necessary

## Hands-On Exploration

### Try It Yourself

**Exercise 1**: Benchmark concurrent performance

```rust
use std::sync::Arc;
use std::thread;
use std::time::Instant;

fn benchmark_concurrent_writes(num_threads: usize) {
    let skiplist = Arc::new(SkipList::new());
    let start = Instant::now();

    let handles: Vec<_> = (0..num_threads)
        .map(|thread_id| {
            let skiplist = skiplist.clone();
            thread::spawn(move || {
                for i in 0..1000 {
                    let key = format!("thread_{}_key_{}", thread_id, i);
                    skiplist.insert(
                        key.into_bytes(),
                        b"value".to_vec(),
                        i as u64,
                        Operation::Put
                    );
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Time with {} threads: {:?}", num_threads, start.elapsed());
    println!("Total entries: {}", skiplist.size());
}

// Compare 1, 2, 4, 8 threads
```

**Exercise 2**: Observe memory ordering effects

```rust
// Try changing Ordering::Acquire to Ordering::Relaxed
// What happens? Why?
```

### Debugging & Observability

**Key metrics to watch:**

- **Skip list height**: Average and maximum heights
- **Operation latency**: p50, p95, p99 percentiles
- **Conflict rate**: How often compare-and-swap fails

**Debugging techniques:**

- **Linearizability testing**: Verify operations appear atomic
- **Thread sanitizer**: Detect data races
- **Performance profiling**: Find contention hotspots

## Real-World Context

### Industry Comparison

**How other databases handle concurrent data structures:**

- **Redis**: Single-threaded, no concurrency issues
- **PostgreSQL**: MVCC with row-level locking
- **CockroachDB**: Lock-free B-trees with epoch reclamation
- **MemSQL**: Lock-free skip lists similar to FerrisDB

### Historical Evolution

**Timeline:**

- **1990**: Skip lists invented by William Pugh
- **2001**: First lock-free skip list algorithms
- **2011**: Java's ConcurrentSkipListMap mainstream adoption
- **Today**: Standard technique for in-memory databases

## Common Pitfalls & Best Practices

### Implementation Pitfalls

1. **ABA problem**:

   - **Problem**: Pointer changes from A→B→A, appears unchanged
   - **Solution**: Epoch-based reclamation prevents reuse

2. **Memory ordering bugs**:

   - **Problem**: Using Relaxed ordering incorrectly
   - **Solution**: Start with SeqCst, optimize carefully

3. **Epoch advancement**:
   - **Problem**: Threads not advancing epochs, memory never freed
   - **Solution**: Periodic epoch advancement, bounded memory

### Production Considerations

**Operational concerns:**

- **Memory usage monitoring**: Track skip list height distribution
- **Performance variance**: Monitor p99 latencies for outliers
- **Memory reclamation**: Ensure epochs advance regularly
- **CPU architecture**: Performance varies by CPU memory model

## Summary & Key Takeaways

### Core Concepts Learned

1. **Skip lists provide O(log n) operations probabilistically**: Random structure avoids rebalancing
2. **Lock-free reads enable massive concurrency**: Thousands of threads can read simultaneously
3. **Epoch-based reclamation solves memory safety**: Free memory without locks or use-after-free

### When to Apply This Knowledge

- **Use skip lists when**: High concurrency with many readers, in-memory storage
- **Consider alternatives when**: Need deterministic performance or minimal memory
- **Implementation complexity**: High - requires understanding of atomics and memory ordering

## Further Reading & References

### Related FerrisDB Articles

- [LSM-Trees: The Secret Behind Modern Database Performance](/deep-dive/lsm-trees/): Where skip lists fit in the architecture
- [Ownership & Sharing: MemTable Lifecycle](/rust-by-example/ownership-memtable-sharing/): Rust's memory model in practice

### Academic Papers

- "Skip Lists: A Probabilistic Alternative to Balanced Trees" (Pugh, 1990): Original skip list paper
- "A Pragmatic Implementation of Non-Blocking Linked-Lists" (Harris, 2001): Lock-free techniques

### Industry Resources

- [Java ConcurrentSkipListMap](https://docs.oracle.com/javase/8/docs/api/java/util/concurrent/ConcurrentSkipListMap.html): Production implementation
- [Crossbeam Epoch](https://docs.rs/crossbeam-epoch/): Rust's epoch-based reclamation

### FerrisDB Code Exploration

- **Skip list implementation**: `ferrisdb-storage/src/memtable/skip_list.rs` - Complete implementation
- **MemTable wrapper**: `ferrisdb-storage/src/memtable/mod.rs` - How skip list is used
- **Tests**: `ferrisdb-storage/src/memtable/skip_list.rs#[cfg(test)]` - Usage examples

---

## About This Series

This article is part of FerrisDB's technical deep dive series. Each article provides comprehensive coverage of database internals through practical implementation:

- ✅ **Real implementation details** from FerrisDB source code
- ✅ **Mathematical analysis** with concrete complexity bounds
- ✅ **Practical exercises** for hands-on learning
- ✅ **Industry context** and alternative approaches

**Target audience**: CRUD developers who want to understand database systems deeply.

[Browse all deep dives](/deep-dive/) | [Architecture overview](/architecture/) | [Contribute on GitHub]({{ site.project.repo_url }})

---

_Last updated: May 29, 2025_
_Estimated reading time: 20 minutes_
_Difficulty: Intermediate_
