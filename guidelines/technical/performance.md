# Performance Guidelines

Performance considerations and optimization strategies for FerrisDB.

## Performance Philosophy

1. **Correctness first** - Never sacrifice correctness for speed
2. **Measure before optimizing** - Use benchmarks and profiling
3. **Document trade-offs** - Explain why optimizations were made
4. **Keep it understandable** - Complex optimizations need clear docs

## Benchmarking

### Writing Benchmarks

Use Criterion.rs for benchmarking:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_memtable_insert(c: &mut Criterion) {
    c.bench_function("memtable_insert", |b| {
        let memtable = MemTable::new();
        let mut i = 0u64;
        b.iter(|| {
            let key = i.to_be_bytes();
            memtable.put(&key, b"value").unwrap();
            i += 1;
        });
    });
}

criterion_group!(benches, bench_memtable_insert);
criterion_main!(benches);
```

### Key Metrics to Track

1. **Throughput**
   - Operations per second
   - Bytes per second
   - Transactions per second

2. **Latency**
   - p50, p95, p99, p999
   - Mean and standard deviation
   - Worst case

3. **Resource Usage**
   - Memory consumption
   - CPU utilization
   - Disk I/O
   - Network bandwidth

### Benchmark Categories

Create benchmarks for:
- Single-threaded operations
- Concurrent operations
- Large value handling
- Scan performance
- Recovery time
- Compaction impact

## Optimization Strategies

### Memory Optimizations

1. **Arena Allocation**
   ```rust
   // Consider arena allocation for MemTable
   pub struct Arena {
       chunks: Vec<Vec<u8>>,
       current: usize,
   }
   ```

2. **Object Pooling**
   ```rust
   // Reuse expensive objects
   use crossbeam::queue::ArrayQueue;
   
   struct BufferPool {
       pool: ArrayQueue<Vec<u8>>,
   }
   ```

3. **Zero-Copy Operations**
   ```rust
   // Use bytes::Bytes for zero-copy
   use bytes::Bytes;
   
   pub struct Value {
       data: Bytes,
   }
   ```

### I/O Optimizations

1. **Vectored I/O**
   ```rust
   use std::io::IoSlice;
   
   // Write multiple buffers in one syscall
   file.write_vectored(&[
       IoSlice::new(&header),
       IoSlice::new(&data),
       IoSlice::new(&footer),
   ])?;
   ```

2. **Direct I/O**
   ```rust
   // Bypass page cache for large sequential writes
   #[cfg(target_os = "linux")]
   use std::os::unix::fs::OpenOptionsExt;
   
   OpenOptions::new()
       .custom_flags(libc::O_DIRECT)
       .open(path)?;
   ```

3. **Asynchronous I/O**
   ```rust
   // Use tokio for async I/O
   use tokio::fs::File;
   
   async fn write_async(file: &mut File, data: &[u8]) {
       file.write_all(data).await?;
   }
   ```

### Concurrency Optimizations

1. **Lock-Free Data Structures**
   - Already using for skip list
   - Consider for other hot paths
   - Document memory ordering carefully

2. **Read-Write Locks**
   ```rust
   use parking_lot::RwLock;
   
   // Prefer RwLock when reads dominate
   struct Cache {
       data: RwLock<HashMap<Key, Value>>,
   }
   ```

3. **Sharding**
   ```rust
   // Reduce contention by sharding
   struct ShardedMap {
       shards: Vec<Mutex<HashMap<Key, Value>>>,
   }
   ```

## Profiling Tools

### CPU Profiling

```bash
# Using perf on Linux
perf record --call-graph=dwarf target/release/ferrisdb-server
perf report

# Using flamegraph
cargo install flamegraph
cargo flamegraph --bench bench_name
```

### Memory Profiling

```bash
# Using valgrind
valgrind --tool=massif target/release/ferrisdb-server
ms_print massif.out.*

# Using heaptrack
heaptrack target/release/ferrisdb-server
heaptrack_gui heaptrack.ferrisdb-server.*
```

### Built-in Metrics

Add performance counters:

```rust
use prometheus::{Counter, Histogram, register_counter, register_histogram};

lazy_static! {
    static ref WRITES_TOTAL: Counter = register_counter!(
        "ferrisdb_writes_total",
        "Total number of write operations"
    ).unwrap();
    
    static ref WRITE_LATENCY: Histogram = register_histogram!(
        "ferrisdb_write_latency_seconds",
        "Write operation latency"
    ).unwrap();
}
```

## Performance Anti-Patterns

Avoid these common mistakes:

1. **Premature optimization** - Measure first
2. **Ignoring cache effects** - Consider data locality
3. **Excessive allocation** - Reuse buffers
4. **Lock contention** - Use fine-grained locking
5. **Synchronous I/O in hot path** - Use async where appropriate

## Performance Checklist

Before merging performance changes:

- [ ] Benchmarks show improvement
- [ ] No regression in other operations
- [ ] Memory usage is reasonable
- [ ] Code is still understandable
- [ ] Trade-offs are documented
- [ ] Edge cases are handled

## References

- [The Flame Graph](https://www.brendangregg.com/flamegraphs.html)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Systems Performance](https://www.brendangregg.com/systems-performance-2nd-edition-book.html)
- [Database Internals](https://www.databass.dev/)