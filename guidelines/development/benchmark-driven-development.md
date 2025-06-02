# Benchmark-Driven Development

This document describes FerrisDB's approach to performance validation through benchmarking.

## Core Principle

**No performance claims without proof.** Every optimization must be validated with benchmarks.

## When Benchmarks Are Required

You MUST provide benchmarks when:

1. **Claiming performance improvements** - "This is 23% faster"
2. **Optimizing algorithms** - Changing implementation for speed
3. **Adding zero-copy abstractions** - Like BytesMutExt
4. **Modifying hot paths** - WAL writes, MemTable operations
5. **Comparing approaches** - "X is better than Y"

## Benchmark Structure

### Small/Medium/Large Pattern

```rust
#[bench]
fn bench_operation_small(b: &mut Bencher) {
    let data = create_small_dataset(); // ~1KB
    b.iter(|| perform_operation(&data));
}

#[bench]
fn bench_operation_medium(b: &mut Bencher) {
    let data = create_medium_dataset(); // ~1MB
    b.iter(|| perform_operation(&data));
}

#[bench]
fn bench_operation_large(b: &mut Bencher) {
    let data = create_large_dataset(); // ~10MB
    b.iter(|| perform_operation(&data));
}
```

### Comparison Benchmarks

Always benchmark both old and new approaches:

```rust
#[bench]
fn bench_standard_approach(b: &mut Bencher) {
    b.iter(|| {
        let mut buf = vec![0u8; SIZE];
        fill_buffer(&mut buf);
    });
}

#[bench]
fn bench_optimized_approach(b: &mut Bencher) {
    b.iter(|| {
        let mut buf = BytesMut::with_capacity(SIZE);
        buf.put_bytes(0, SIZE);
        fill_buffer(&mut buf);
    });
}
```

## Using Criterion

For detailed benchmarks, use Criterion.rs:

```toml
[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "wal_benchmarks"
harness = false
```

```rust
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_wal_writes(c: &mut Criterion) {
    let mut group = c.benchmark_group("wal_writes");

    group.bench_function("sync_mode", |b| {
        let wal = WAL::new(SyncMode::Sync);
        b.iter(|| wal.append(b"key", b"value"));
    });

    group.bench_function("no_sync_mode", |b| {
        let wal = WAL::new(SyncMode::NoSync);
        b.iter(|| wal.append(b"key", b"value"));
    });

    group.finish();
}

criterion_group!(benches, benchmark_wal_writes);
criterion_main!(benches);
```

## Benchmark Documentation

### In Code Comments

```rust
/// Zero-copy buffer extension that avoids initialization overhead.
///
/// # Performance
///
/// Benchmarks show 23-33% improvement over standard Vec allocation:
/// - Small (1KB): 23% faster
/// - Medium (64KB): 31% faster
/// - Large (1MB): 33% faster
///
/// See benches/bytes_ext_benchmarks.rs for details.
pub trait BytesMutExt {
    // ...
}
```

### In Benchmark Files

Create README.md in bench directories:

````markdown
# WAL Benchmarks

## Results Summary

| Operation  | Sync Mode | NoSync Mode | Improvement |
| ---------- | --------- | ----------- | ----------- |
| Write 1KB  | 45µs      | 2µs         | 22.5x       |
| Write 64KB | 120µs     | 15µs        | 8x          |
| Write 1MB  | 950µs     | 180µs       | 5.3x        |

## Running Benchmarks

```bash
cargo bench --bench wal_benchmarks
```
````

## Methodology

- Each benchmark runs 100 iterations
- Results show median with confidence intervals
- Tests run on isolated cores when possible

````

## CI Integration

### PR Benchmarks

For performance-critical PRs:

```yaml
- name: Run benchmarks
  if: contains(github.event.pull_request.labels.*.name, 'performance')
  run: |
    cargo bench --bench main_benchmarks -- --save-baseline pr
    git checkout main
    cargo bench --bench main_benchmarks -- --baseline pr
````

### Regression Detection

Fail CI if performance regresses:

```rust
#[test]
fn performance_regression_test() {
    let mut wal = WAL::new(test_path());

    let start = Instant::now();
    for i in 0..1000 {
        wal.append(&format!("key{}", i), b"value").unwrap();
    }
    let elapsed = start.elapsed();

    // Fail if slower than baseline + 10%
    assert!(elapsed.as_millis() < 110); // 100ms baseline
}
```

## Best Practices

### 1. Isolate What You're Testing

```rust
// Bad: Tests both allocation and initialization
#[bench]
fn bench_combined(b: &mut Bencher) {
    b.iter(|| {
        let mut vec = Vec::with_capacity(SIZE);
        vec.resize(SIZE, 0);
        process_data(&mut vec);
    });
}

// Good: Tests only the optimization
#[bench]
fn bench_initialization(b: &mut Bencher) {
    let mut vec = Vec::with_capacity(SIZE);
    b.iter(|| {
        vec.clear();
        vec.resize(SIZE, 0);
    });
}
```

### 2. Use Realistic Data

```rust
// Bad: Unrealistic perfect data
let data = vec![42u8; 1000];

// Good: Realistic mixed data
let data = generate_realistic_workload();
```

### 3. Document Assumptions

```rust
/// Benchmarks assume:
/// - Single-threaded access (no contention)
/// - Pre-warmed file system cache
/// - SSD storage with >500MB/s write speed
```

### 4. Consider Memory

Track allocations when relevant:

```rust
#[global_allocator]
static ALLOC: CountingAllocator = CountingAllocator;

#[bench]
fn bench_with_allocation_tracking(b: &mut Bencher) {
    b.iter(|| {
        let before = ALLOC.allocated();
        perform_operation();
        let after = ALLOC.allocated();
        assert_eq!(before, after, "Operation should not allocate");
    });
}
```

## Example: BytesMutExt Validation

See `ferrisdb-storage/benches/bytes_ext_benchmarks.rs`:

1. **Hypothesis**: Avoiding zero-initialization improves performance
2. **Test Design**: Compare Vec vs BytesMut for various sizes
3. **Results**: 23-33% improvement across all sizes
4. **Conclusion**: Optimization validated, worth the complexity

## Benchmark Checklist

- [ ] Benchmark both old and new approaches
- [ ] Test small, medium, and large inputs
- [ ] Document results in code comments
- [ ] Add README to benchmark directory
- [ ] Consider CI integration for critical paths
- [ ] Track memory allocations if relevant
- [ ] Use realistic test data
- [ ] Run multiple times to ensure stability

## References

- [Criterion.rs Documentation](https://bheisler.github.io/criterion.rs/book/)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [BytesMutExt Benchmarks](../../ferrisdb-storage/benches/)
