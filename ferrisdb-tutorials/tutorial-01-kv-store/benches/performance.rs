//! Performance benchmarks for the key-value store
//! 
//! These benchmarks validate our performance claims and
//! demonstrate HashMap's O(1) average-case performance.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use tutorial_01_kv_store::KeyValueStore;

fn bench_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("insert");
    
    for size in [10, 100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut store = KeyValueStore::new();
                for i in 0..size {
                    store.set(
                        format!("key{}", i),
                        format!("value{}", i)
                    );
                }
            });
        });
    }
    
    group.finish();
}

fn bench_get_existing(c: &mut Criterion) {
    let mut group = c.benchmark_group("get_existing");
    
    for size in [10, 100, 1000, 10000].iter() {
        let mut store = KeyValueStore::new();
        for i in 0..*size {
            store.set(format!("key{}", i), format!("value{}", i));
        }
        
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            let key = format!("key{}", size / 2); // Get from middle
            b.iter(|| {
                black_box(store.get(&key));
            });
        });
    }
    
    group.finish();
}

fn bench_get_missing(c: &mut Criterion) {
    let mut group = c.benchmark_group("get_missing");
    
    for size in [10, 100, 1000, 10000].iter() {
        let mut store = KeyValueStore::new();
        for i in 0..*size {
            store.set(format!("key{}", i), format!("value{}", i));
        }
        
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                black_box(store.get("missing_key"));
            });
        });
    }
    
    group.finish();
}

fn bench_mixed_operations(c: &mut Criterion) {
    c.bench_function("mixed_operations", |b| {
        let mut store = KeyValueStore::new();
        
        // Pre-populate with some data
        for i in 0..1000 {
            store.set(format!("key{}", i), format!("value{}", i));
        }
        
        let mut counter = 0;
        b.iter(|| {
            // Mix of operations that might happen in real usage
            match counter % 4 {
                0 => {
                    // Insert new
                    store.set(
                        format!("new_key{}", counter),
                        format!("new_value{}", counter)
                    );
                }
                1 => {
                    // Update existing
                    store.set(
                        format!("key{}", counter % 1000),
                        format!("updated{}", counter)
                    );
                }
                2 => {
                    // Read existing
                    black_box(store.get(&format!("key{}", counter % 1000)));
                }
                _ => {
                    // Read missing
                    black_box(store.get(&format!("missing{}", counter)));
                }
            }
            counter += 1;
        });
    });
}

fn bench_memory_efficiency(c: &mut Criterion) {
    c.bench_function("create_1000_entries", |b| {
        b.iter(|| {
            let mut store = KeyValueStore::new();
            for i in 0..1000 {
                store.set(
                    format!("user:{}", i),
                    format!("User Name {}", i)
                );
            }
            black_box(store.len());
        });
    });
}

criterion_group!(
    benches,
    bench_insert,
    bench_get_existing,
    bench_get_missing,
    bench_mixed_operations,
    bench_memory_efficiency
);
criterion_main!(benches);