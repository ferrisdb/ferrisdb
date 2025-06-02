use ferrisdb_core::SyncMode;
use ferrisdb_storage::wal::{WALEntry, WALReader, WALWriter};

use criterion::{
    black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput,
};
use tempfile::TempDir;

use std::sync::Arc;
use std::thread;

fn create_test_wal(path: &std::path::Path, num_entries: usize) {
    let writer = WALWriter::new(path, SyncMode::Full, 100 * 1024 * 1024).unwrap();

    for i in 0..num_entries {
        let key = format!("key_{:06}", i).into_bytes();
        let value = vec![b'v'; 1000]; // 1KB value
        let entry = WALEntry::new_put(key, value, i as u64).unwrap();
        writer.append(&entry).unwrap();
    }

    writer.sync().unwrap();
}

/// Benchmarks bulk reading of all entries from a WAL file.
///
/// Measures:
/// - Time to read 1000 entries in a single operation
/// - Efficiency of bulk read operations
/// - Buffer allocation and reuse performance
fn bench_read_all(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    let wal_path = temp_dir.path().join("bench.wal");

    // Create WAL with 1000 entries
    create_test_wal(&wal_path, 1000);

    c.bench_function("wal_reader_read_all_1000", |b| {
        b.iter(|| {
            let mut reader = WALReader::new(&wal_path).unwrap();
            let entries = reader.read_all().unwrap();
            black_box(entries);
        });
    });
}

/// Benchmarks reading entries one at a time using read_entry().
///
/// Measures:
/// - Per-entry read overhead
/// - Iterator-style access performance
/// - Comparison with bulk read_all() approach
fn bench_read_individual(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    let wal_path = temp_dir.path().join("bench.wal");

    // Create WAL with 100 entries
    create_test_wal(&wal_path, 100);

    c.bench_function("wal_reader_individual_100", |b| {
        b.iter(|| {
            let mut reader = WALReader::new(&wal_path).unwrap();
            let mut count = 0;
            while let Some(entry) = reader.read_entry().unwrap() {
                black_box(entry);
                count += 1;
            }
            assert_eq!(count, 100);
        });
    });
}

/// Benchmarks the impact of initial buffer size on read performance.
///
/// Tests:
/// - Small (64B), medium (8KB), and large (64KB) initial buffers
/// - Buffer resize overhead with small initial sizes
/// - Optimal buffer size for mixed entry sizes
fn bench_buffer_sizes(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    let wal_path = temp_dir.path().join("bench.wal");

    // Create WAL with mixed size entries
    let writer = WALWriter::new(&wal_path, SyncMode::Full, 100 * 1024 * 1024).unwrap();
    for i in 0..100 {
        let key = format!("key_{:06}", i).into_bytes();
        // Vary value sizes: small (100B), medium (1KB), large (10KB)
        let value_size = match i % 3 {
            0 => 100,
            1 => 1024,
            _ => 10240,
        };
        let value = vec![b'x'; value_size];
        let entry = WALEntry::new_put(key, value, i as u64).unwrap();
        writer.append(&entry).unwrap();
    }
    writer.sync().unwrap();

    let mut group = c.benchmark_group("buffer_sizes");

    // Small initial buffer (will resize frequently)
    group.bench_function("small_buffer_64B", |b| {
        b.iter(|| {
            let mut reader = WALReader::with_initial_capacity(&wal_path, 64).unwrap();
            let entries = reader.read_all().unwrap();
            black_box(entries);
        });
    });

    // Medium initial buffer
    group.bench_function("medium_buffer_8KB", |b| {
        b.iter(|| {
            let mut reader = WALReader::with_initial_capacity(&wal_path, 8 * 1024).unwrap();
            let entries = reader.read_all().unwrap();
            black_box(entries);
        });
    });

    // Large initial buffer (minimal resizing)
    group.bench_function("large_buffer_64KB", |b| {
        b.iter(|| {
            let mut reader = WALReader::with_initial_capacity(&wal_path, 64 * 1024).unwrap();
            let entries = reader.read_all().unwrap();
            black_box(entries);
        });
    });

    group.finish();
}

/// Benchmarks write throughput for various entry sizes.
///
/// Measures:
/// - Throughput for 100B, 1KB, 10KB, and 100KB entries
/// - Scaling characteristics with entry size
/// - Identifies optimal entry size for throughput
fn bench_write_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("write_throughput");

    for entry_size in &[100, 1024, 10240, 102400] {
        group.throughput(Throughput::Bytes(*entry_size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}B", entry_size)),
            entry_size,
            |b, &size| {
                b.iter_batched(
                    || {
                        let temp_dir = TempDir::new().unwrap();
                        let path = temp_dir.path().join("bench.wal");
                        let writer =
                            WALWriter::new(&path, SyncMode::None, 100 * 1024 * 1024).unwrap();
                        let value = vec![b'v'; size];
                        (temp_dir, writer, value)
                    },
                    |(temp_dir, writer, value)| {
                        for i in 0..100 {
                            let entry = WALEntry::new_put(
                                format!("key_{}", i).into_bytes(),
                                value.clone(),
                                i as u64,
                            )
                            .unwrap();
                            writer.append(&entry).unwrap();
                        }
                        black_box(temp_dir);
                    },
                    BatchSize::SmallInput,
                );
            },
        );
    }
    group.finish();
}

fn bench_read_zero_allocation(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    let wal_path = temp_dir.path().join("zero_alloc.wal");

    // Create WAL with consistent-sized entries
    create_test_wal(&wal_path, 1000);

    c.bench_function("read_zero_allocation", |b| {
        let mut reader = WALReader::new(&wal_path).unwrap();

        b.iter(|| {
            // Reset reader position
            reader = WALReader::new(&wal_path).unwrap();

            // First read allocates buffer
            let first = reader.read_entry().unwrap();
            black_box(first);

            // Subsequent reads should reuse buffer
            for _ in 0..99 {
                let entry = reader.read_entry().unwrap();
                black_box(entry);
            }
        });
    });
}

fn bench_bytesmut_vs_vec(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    let wal_path = temp_dir.path().join("comparison.wal");

    // Create test data with mixed sizes
    let writer = WALWriter::new(&wal_path, SyncMode::Full, 100 * 1024 * 1024).unwrap();
    for i in 0..100 {
        let size = match i % 3 {
            0 => 100,
            1 => 1024,
            _ => 10240,
        };
        let entry = WALEntry::new_put(
            format!("key_{}", i).into_bytes(),
            vec![b'x'; size],
            i as u64,
        )
        .unwrap();
        writer.append(&entry).unwrap();
    }

    let mut group = c.benchmark_group("buffer_implementation");

    // Current implementation (BytesMut)
    group.bench_function("bytesmut", |b| {
        b.iter(|| {
            let mut reader = WALReader::new(&wal_path).unwrap();
            let entries = reader.read_all().unwrap();
            black_box(entries);
        });
    });

    // Note: To properly compare with Vec<u8>, we'd need an alternative implementation
    // This is a placeholder showing the benchmark structure

    group.finish();
}

/// Benchmarks performance impact of different sync modes.
///
/// Compares:
/// - SyncMode::None (no sync)
/// - SyncMode::Normal (OS buffer flush)
/// - SyncMode::Full (fsync to disk)
/// - Helps users choose appropriate durability/performance tradeoff
fn bench_sync_modes(c: &mut Criterion) {
    let mut group = c.benchmark_group("sync_modes");
    group.throughput(Throughput::Elements(100)); // 100 entries

    for sync_mode in &[SyncMode::None, SyncMode::Normal, SyncMode::Full] {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{:?}", sync_mode)),
            sync_mode,
            |b, &sync_mode| {
                b.iter_batched(
                    || {
                        let temp_dir = TempDir::new().unwrap();
                        let path = temp_dir.path().join("sync_bench.wal");
                        let writer = WALWriter::new(&path, sync_mode, 100 * 1024 * 1024).unwrap();
                        (temp_dir, writer)
                    },
                    |(temp_dir, writer)| {
                        for i in 0..100 {
                            let entry = WALEntry::new_put(
                                format!("key_{}", i).into_bytes(),
                                vec![b'v'; 1024],
                                i as u64,
                            )
                            .unwrap();
                            writer.append(&entry).unwrap();
                        }
                        writer.sync().unwrap();
                        black_box(temp_dir);
                    },
                    BatchSize::SmallInput,
                );
            },
        );
    }
    group.finish();
}

fn bench_concurrent_reads(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    let wal_path = temp_dir.path().join("concurrent.wal");

    // Create WAL with many entries
    create_test_wal(&wal_path, 10000);

    let mut group = c.benchmark_group("concurrent_reads");

    for num_readers in &[1, 2, 4, 8] {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}_readers", num_readers)),
            num_readers,
            |b, &num_readers| {
                b.iter(|| {
                    let mut handles = vec![];
                    let path = Arc::new(wal_path.clone());

                    for _ in 0..num_readers {
                        let path = Arc::clone(&path);
                        handles.push(thread::spawn(move || {
                            let mut reader = WALReader::new(&*path).unwrap();
                            let entries = reader.read_all().unwrap();
                            black_box(entries.len());
                        }));
                    }

                    for handle in handles {
                        handle.join().unwrap();
                    }
                });
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_read_all,
    bench_read_individual,
    bench_buffer_sizes,
    bench_write_throughput,
    bench_read_zero_allocation,
    bench_bytesmut_vs_vec,
    bench_sync_modes,
    bench_concurrent_reads
);
criterion_main!(benches);
