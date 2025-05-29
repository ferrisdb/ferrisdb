---
layout: page
title: "Lock-Free Skip Lists: Building a Concurrent MemTable"
subtitle: "How FerrisDB implements a thread-safe in-memory storage with lock-free reads"
permalink: /deep-dive/concurrent-skip-list/
---

Building a database that can handle thousands of concurrent operations requires careful attention to data structure design. This deep dive explores how FerrisDB uses a lock-free skip list for its MemTable, achieving high concurrency without sacrificing correctness.

## The Concurrency Challenge

Traditional approaches to concurrent data structures often use locks:

```rust
// Naive approach - global lock
struct NaiveMemTable {
    data: Mutex<BTreeMap<Key, Value>>,
}

impl NaiveMemTable {
    fn get(&self, key: &Key) -> Option<Value> {
        let guard = self.data.lock().unwrap();  // Blocks ALL operations!
        guard.get(key).cloned()
    }
}
```

Problems with this approach:

- **Reader blocks reader** - Even though reads don't modify data
- **Writer blocks everything** - Single writer can halt all progress
- **Lock contention** - Performance degrades with more threads

## Enter Skip Lists

Skip lists are probabilistic data structures that provide O(log n) operations like balanced trees, but with a simpler implementation that's more amenable to lock-free algorithms.

**What does this mean in plain English?**

- **Probabilistic**: Uses randomness to decide structure (like flipping a coin)
- **O(log n)**: Performance scales well - doubling data size adds only one more step
- **Lock-free friendly**: Multiple threads can work without waiting for each other

### Skip List Structure

```text
Level 3: HEAD ------------------> 30 -------------------------> NULL
Level 2: HEAD ------> 10 -------> 30 -------> 50 -------------> NULL
Level 1: HEAD -> 5 -> 10 -> 20 -> 30 -> 40 -> 50 -> 60 -------> NULL
Level 0: HEAD -> 5 -> 10 -> 20 -> 30 -> 40 -> 50 -> 60 -> 70 -> NULL
```

Each node has a random height, creating "express lanes" for faster traversal.

**Think of it like a subway system:**

- Level 0 is the local train that stops at every station
- Level 1 is the express that skips some stops
- Level 2 is the super-express that skips even more
- To find your stop, take the fastest train and switch down when needed

## FerrisDB's Implementation

### Core Node Structure

```rust
pub struct Node<K, V> {
    key: K,
    value: V,
    next: Vec<Atomic<Node<K, V>>>,  // One per level
}

pub struct SkipList<K, V> {
    head: Atomic<Node<K, V>>,
    max_height: usize,
    height: AtomicUsize,
}
```

### Lock-Free Search

The beauty of skip lists is that search requires no locks:

```rust
impl<K: Ord, V> SkipList<K, V> {
    pub fn get<'a>(&'a self, key: &K, guard: &'a Guard) -> Option<&'a V> {
        let mut current = self.head.load(Ordering::Acquire, guard);

        // Start from the highest level
        for level in (0..self.height()).rev() {
            // Move forward until we find a larger key
            loop {
                match current.as_ref() {
                    Some(node) => {
                        let next = node.next[level].load(Ordering::Acquire, guard);
                        match next.as_ref() {
                            Some(next_node) if next_node.key < *key => {
                                current = next;
                            }
                            _ => break,
                        }
                    }
                    None => break,
                }
            }
        }

        // Check if we found the key
        current.as_ref()
            .filter(|node| node.key == *key)
            .map(|node| &node.value)
    }
}
```

Key insights:

- **No locks needed** - Just atomic loads (reading a value that might be changing)
- **Consistent snapshots** - Guard ensures nodes aren't freed while reading
- **Wait-free** - Readers never block (they always make progress)

**What's an atomic load?**
An atomic load is a read operation that's guaranteed to see a complete value, not a partially-written one. It's like taking a photo - you get the whole picture at one instant, not a blurry mix of two states.

### Memory Management with Epochs

FerrisDB uses `crossbeam-epoch` for safe memory reclamation:

```rust
pub struct MemTable {
    skiplist: Arc<SkipList<InternalKey, Value>>,
    size: AtomicUsize,
}

impl MemTable {
    pub fn get(&self, user_key: &[u8], timestamp: Timestamp) -> Option<Value> {
        let guard = crossbeam_epoch::pin();  // Pin current epoch
        let internal_key = InternalKey::new(user_key.to_vec(), timestamp);

        self.skiplist.get(&internal_key, &guard)
            .map(|v| v.clone())
    }
}
```

The epoch-based reclamation ensures:

- Nodes are only freed when no thread can access them
- No ABA problems
- No use-after-free bugs

**What's the ABA Problem?**

The ABA problem is a classic concurrency bug. Imagine this scenario:

1. Thread 1 reads a pointer to memory location A
2. Thread 2 frees A and allocates new memory B at the same address
3. Thread 1's pointer still looks valid (same address) but now points to completely different data!

It's called "ABA" because the value changed from A to B and back to A (same address), but it's actually different data. Epoch-based reclamation prevents this by ensuring memory isn't reused while any thread might still have a reference to it.

### Insertion with Compare-and-Swap

Insertions use lock-free algorithms with CAS (Compare-and-Swap):

```rust
impl<K: Ord, V> SkipList<K, V> {
    pub fn insert(&self, key: K, value: V, guard: &Guard) -> Option<V> {
        let new_height = self.random_height();
        let new_node = Owned::new(Node {
            key,
            value,
            next: vec![Atomic::null(); new_height],
        });

        loop {
            let (found, preds, succs) = self.find_position(&key, guard);

            if found {
                // Key exists - update value atomically
                // ... CAS logic here ...
            } else {
                // Insert new node
                let new_node_ref = new_node.clone().into_shared(guard);

                // Update all levels atomically
                for level in 0..new_height {
                    new_node_ref.next[level].store(succs[level], Ordering::Release);
                }

                // CAS to insert
                match preds[0].compare_exchange(
                    succs[0],
                    new_node_ref,
                    Ordering::Release,
                    guard,
                ) {
                    Ok(_) => {
                        // Success! Update higher levels
                        for level in 1..new_height {
                            // ... update other levels ...
                        }
                        return None;
                    }
                    Err(_) => continue,  // Retry
                }
            }
        }
    }
}
```

## MVCC Support

FerrisDB's MemTable supports Multi-Version Concurrency Control (MVCC) through careful key ordering:

```rust
#[derive(Clone, PartialEq, Eq)]
pub struct InternalKey {
    pub user_key: Vec<u8>,
    pub timestamp: Timestamp,
}

impl Ord for InternalKey {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.user_key.cmp(&other.user_key) {
            Ordering::Equal => {
                // Reverse timestamp order (newest first)
                other.timestamp.cmp(&self.timestamp)
            }
            other => other,
        }
    }
}
```

This ordering enables:

- **Multiple versions** of the same key
- **Point-in-time queries** by timestamp
- **Efficient garbage collection** of old versions

## Performance Characteristics

### Benchmark Code (To Be Implemented)

The following benchmark code will measure concurrent performance:

```rust
#[bench]
fn bench_concurrent_reads(b: &mut Bencher) {
    let memtable = Arc::new(MemTable::new(1 << 20));

    // Insert 10,000 entries
    for i in 0..10_000 {
        memtable.put(format!("key{}", i).as_bytes(), b"value", i);
    }

    // Benchmark concurrent reads with 8 threads
    b.iter(|| {
        crossbeam::scope(|scope| {
            for _ in 0..8 {
                scope.spawn(|_| {
                    for i in 0..1000 {
                        memtable.get(format!("key{}", i).as_bytes(), i);
                    }
                });
            }
        }).unwrap();
    });
}
```

_Note: Actual benchmarks are pending implementation. The following shows expected performance characteristics based on lock-free data structure theory:_

Expected results on 8-core machine:

- **Single-threaded**: Baseline performance
- **8 threads**: Near-linear scaling for reads
- **Write contention**: Moderate scaling due to CAS retries

### Expected Performance Characteristics

| Approach            | Read Scaling | Write Scaling | Implementation Complexity |
| ------------------- | ------------ | ------------- | ------------------------- |
| Mutex<BTreeMap>     | Poor         | Poor          | Simple                    |
| RwLock<BTreeMap>    | Good         | Poor          | Simple                    |
| Lock-free Skip List | Excellent    | Good          | Complex                   |

The actual performance will depend on:

- Key distribution and size
- Value size
- Read/write ratio
- CPU architecture and cache sizes

## Implementation Challenges

### 1. ABA Problem

The classic concurrent programming issue where a value changes from A to B and back to A:

```rust
// Thread 1 reads pointer P pointing to node A
let ptr = atomic_ptr.load();

// Thread 2: frees A, allocates new node B at same address
// Thread 1: CAS succeeds but ptr now points to different data!
```

Solution: Epoch-based reclamation ensures nodes aren't reused while any thread might access them.

### 2. Memory Ordering

**What's Memory Ordering?**

Modern CPUs can reorder instructions for performance. Memory ordering tells the CPU what reorderings are allowed. Think of it like traffic rules:

- **Relaxed**: "Go whenever you want" - fastest but can see weird states
- **Acquire/Release**: "Wait for the green light" - ensures proper sequencing
- **SeqCst**: "Stop at every intersection" - safest but slowest

Choosing the right memory ordering is crucial:

```rust
// Too weak - might see inconsistent state
node.next.load(Ordering::Relaxed)
// Problem: Might read new pointer but old data!

// Just right - ensures happens-before relationship
node.next.load(Ordering::Acquire)
// Guarantees: If we see a new pointer, we also see all its data

// Overkill for reads - unnecessary synchronization
node.next.load(Ordering::SeqCst)
// Downside: Forces all CPUs to synchronize (slow!)
```

### 3. Height Distribution

Skip list performance depends on proper height distribution:

```rust
fn random_height(&self) -> usize {
    let mut height = 1;
    while height < self.max_height && rand::random::<bool>() {
        height += 1;
    }
    height
}
```

This gives each node:

- Height 1: 50% probability
- Height 2: 25% probability
- Height 3: 12.5% probability
- And so on...

## Lessons Learned

### 1. Lock-Free != Simple

Lock-free algorithms are complex. The skip list implementation is ~500 lines vs ~50 for a locked BTree wrapper. The performance gain must justify the complexity.

**When is the complexity worth it?**

- High-contention scenarios (many threads accessing data)
- Read-heavy workloads (readers don't block each other)
- Low-latency requirements (no unpredictable lock waits)
- Systems where one slow thread shouldn't block others

### 2. Memory Reclamation is Hard

Manual memory management in concurrent code is error-prone. Using battle-tested libraries like `crossbeam-epoch` is essential.

**Why is concurrent memory management hard?**

In single-threaded code, you know when you're done with memory. In concurrent code:

- Thread A might free memory while Thread B is still reading it
- You can't use reference counting (too much contention)
- You need to know when ALL threads are done with a piece of memory

`crossbeam-epoch` solves this by dividing time into "epochs" and only freeing memory from old epochs that no thread can be accessing.

### 3. Benchmarking is Critical

Lock-free doesn't automatically mean faster. Always benchmark with realistic workloads:

- **Read/write ratio** - 90% reads vs 50/50 mix behaves very differently
- **Key distribution** - Sequential keys vs random affects cache performance
- **Contention level** - Are threads accessing the same keys?
- **Thread count** - Diminishing returns beyond CPU core count

### 4. Document Invariants

Lock-free code has subtle invariants:

```rust
// INVARIANT: Nodes in a level are always sorted
// INVARIANT: A node's key never changes after insertion
// INVARIANT: Higher levels are always subsets of lower levels
```

## Using FerrisDB's MemTable

```rust
use ferrisdb_storage::memtable::MemTable;

// Create with 1MB capacity
let memtable = MemTable::new(1 << 20);

// Insert data
memtable.put(b"user:123", b"Alice", 1);
memtable.put(b"user:456", b"Bob", 2);

// Read latest version
let value = memtable.get(b"user:123", Timestamp::MAX);
assert_eq!(value, Some(b"Alice".to_vec()));

// Read at specific timestamp
let old_value = memtable.get(b"user:123", 0);
assert_eq!(old_value, None);  // Didn't exist at time 0
```

## What's Next?

The lock-free skip list is just one component. Future enhancements:

1. **Range queries** - Implement efficient iteration
2. **Memory accounting** - Track actual memory usage
3. **Compaction support** - Export to SSTable format
4. **Write batching** - Amortize insertion costs

## Conclusion

Building a lock-free skip list for FerrisDB's MemTable showcases the power and complexity of concurrent data structures. While the implementation is intricate, the result is a MemTable that scales linearly with CPU cores - essential for a modern database.

The key takeaway? Lock-free programming is a powerful tool, but use it judiciously. The complexity is only worth it when you need the absolute best concurrent performance.

---

_Want to learn more? Check out the [source code](https://github.com/ferrisdb/ferrisdb/tree/main/ferrisdb-storage/src/memtable) and run the benchmarks yourself!_
