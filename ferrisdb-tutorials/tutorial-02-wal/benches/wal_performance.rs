//! Performance benchmarks for the Write-Ahead Log
//! 
//! Run with: cargo bench

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use tutorial_02_wal::{Operation, WalBuilder, SyncMode};
use tempfile::tempdir;

fn benchmark_append_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("wal_append");
    
    // Test different sync modes
    for sync_mode in [SyncMode::None, SyncMode::DataOnly, SyncMode::Full] {
        group.bench_with_input(
            BenchmarkId::new("sync_mode", format!("{:?}", sync_mode)),
            &sync_mode,
            |b, &mode| {
                let dir = tempdir().unwrap();
                let wal_path = dir.path().join("bench.wal");
                let mut wal = WalBuilder::new(&wal_path)
                    .sync_mode(mode)
                    .build()
                    .unwrap();
                
                let key = "benchmark_key".to_string();
                let value = "x".repeat(100); // 100 byte value
                
                b.iter(|| {
                    wal.append(Operation::Set {
                        key: black_box(key.clone()),
                        value: black_box(value.clone()),
                    }).unwrap();
                });
            },
        );
    }
    
    group.finish();
}

fn benchmark_value_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("wal_value_size");
    
    // Test different value sizes
    for size in [10, 100, 1_000, 10_000] {
        group.bench_with_input(
            BenchmarkId::new("bytes", size),
            &size,
            |b, &size| {
                let dir = tempdir().unwrap();
                let wal_path = dir.path().join("bench.wal");
                let mut wal = WalBuilder::new(&wal_path)
                    .sync_mode(SyncMode::None) // Fastest mode to isolate serialization cost
                    .build()
                    .unwrap();
                
                let key = "benchmark_key".to_string();
                let value = "x".repeat(size);
                
                b.iter(|| {
                    wal.append(Operation::Set {
                        key: black_box(key.clone()),
                        value: black_box(value.clone()),
                    }).unwrap();
                });
            },
        );
    }
    
    group.finish();
}

fn benchmark_recovery(c: &mut Criterion) {
    let mut group = c.benchmark_group("wal_recovery");
    
    // Test recovery performance with different entry counts
    for num_entries in [100, 1_000, 10_000] {
        group.bench_with_input(
            BenchmarkId::new("entries", num_entries),
            &num_entries,
            |b, &num_entries| {
                // Setup: Create WAL with many entries
                let dir = tempdir().unwrap();
                let wal_path = dir.path().join("bench.wal");
                
                {
                    let mut wal = WalBuilder::new(&wal_path)
                        .sync_mode(SyncMode::None)
                        .build()
                        .unwrap();
                    
                    for i in 0..num_entries {
                        wal.append(Operation::Set {
                            key: format!("key_{}", i),
                            value: format!("value_{}", i),
                        }).unwrap();
                    }
                }
                
                // Benchmark recovery
                b.iter(|| {
                    let wal = WalBuilder::new(&wal_path).build().unwrap();
                    let entries = wal.recover_entries().unwrap();
                    black_box(entries);
                });
            },
        );
    }
    
    group.finish();
}

fn benchmark_checksum_overhead(c: &mut Criterion) {
    c.bench_function("checksum_1kb", |b| {
        use crc32fast::Hasher;
        let data = vec![0u8; 1024]; // 1KB of data
        
        b.iter(|| {
            let mut hasher = Hasher::new();
            hasher.update(black_box(&data));
            black_box(hasher.finalize());
        });
    });
}

criterion_group!(
    benches,
    benchmark_append_operations,
    benchmark_value_sizes,
    benchmark_recovery,
    benchmark_checksum_overhead
);
criterion_main!(benches);