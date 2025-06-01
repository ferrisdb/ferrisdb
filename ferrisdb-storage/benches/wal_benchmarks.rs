use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use ferrisdb_core::SyncMode;
use ferrisdb_storage::wal::{WALEntry, WALWriter};
use std::sync::Arc;
use tempfile::TempDir;

fn bench_sync_modes(c: &mut Criterion) {
    let mut group = c.benchmark_group("wal_sync_modes");
    
    // Test different sync modes with same workload
    for sync_mode in [SyncMode::None, SyncMode::Normal, SyncMode::Full] {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{:?}", sync_mode)),
            &sync_mode,
            |b, &mode| {
                let temp_dir = TempDir::new().unwrap();
                let wal_path = temp_dir.path().join("bench.wal");
                let writer = WALWriter::new(&wal_path, mode, 100 * 1024 * 1024).unwrap();
                
                b.iter(|| {
                    let entry = WALEntry::new_put(
                        black_box(b"benchmark_key".to_vec()),
                        black_box(b"benchmark_value".to_vec()),
                        black_box(1),
                    );
                    writer.append(&entry).unwrap();
                });
            },
        );
    }
    
    group.finish();
}

fn bench_entry_size_impact(c: &mut Criterion) {
    let mut group = c.benchmark_group("wal_entry_size");
    
    // Test how entry size affects throughput
    for size_kb in [1, 4, 16, 64].iter() {
        let size = size_kb * 1024;
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}KB", size_kb)),
            size,
            |b, &size| {
                let temp_dir = TempDir::new().unwrap();
                let wal_path = temp_dir.path().join("bench.wal");
                let writer = WALWriter::new(&wal_path, SyncMode::None, 1024 * 1024 * 1024).unwrap();
                
                let value = vec![0u8; size];
                
                b.iter(|| {
                    let entry = WALEntry::new_put(
                        black_box(b"key".to_vec()),
                        black_box(value.clone()),
                        black_box(1),
                    );
                    writer.append(&entry).unwrap();
                });
            },
        );
    }
    
    group.finish();
}

fn bench_concurrent_writes(c: &mut Criterion) {
    let mut group = c.benchmark_group("wal_concurrent_writes");
    
    for num_threads in [1, 2, 4, 8].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}_threads", num_threads)),
            num_threads,
            |b, &num_threads| {
                let temp_dir = TempDir::new().unwrap();
                let wal_path = temp_dir.path().join("bench.wal");
                let writer = Arc::new(
                    WALWriter::new(&wal_path, SyncMode::None, 1024 * 1024 * 1024).unwrap()
                );
                
                b.iter(|| {
                    let handles: Vec<_> = (0..num_threads)
                        .map(|thread_id| {
                            let writer = Arc::clone(&writer);
                            std::thread::spawn(move || {
                                for i in 0..10 {
                                    let entry = WALEntry::new_put(
                                        format!("key_{}_{}", thread_id, i).into_bytes(),
                                        b"value".to_vec(),
                                        thread_id * 100 + i,
                                    );
                                    writer.append(&entry).unwrap();
                                }
                            })
                        })
                        .collect();
                    
                    for handle in handles {
                        handle.join().unwrap();
                    }
                });
            },
        );
    }
    
    group.finish();
}

fn bench_recovery_speed(c: &mut Criterion) {
    use ferrisdb_storage::wal::WALReader;
    
    let mut group = c.benchmark_group("wal_recovery");
    
    // Prepare WAL files with different entry counts
    for num_entries in [100, 1000, 10000].iter() {
        let temp_dir = TempDir::new().unwrap();
        let wal_path = temp_dir.path().join("bench.wal");
        
        // Write entries
        {
            let writer = WALWriter::new(&wal_path, SyncMode::None, 1024 * 1024 * 1024).unwrap();
            for i in 0..*num_entries {
                let entry = WALEntry::new_put(
                    format!("key_{}", i).into_bytes(),
                    format!("value_{}", i).into_bytes(),
                    i as u64,
                );
                writer.append(&entry).unwrap();
            }
            writer.sync().unwrap();
        }
        
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}_entries", num_entries)),
            &wal_path,
            |b, path| {
                b.iter(|| {
                    let mut reader = WALReader::new(path).unwrap();
                    let entries = reader.read_all().unwrap();
                    black_box(entries);
                });
            },
        );
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_sync_modes,
    bench_entry_size_impact,
    bench_concurrent_writes,
    bench_recovery_speed
);
criterion_main!(benches);