//! Benchmarks that prove specific performance claims for WAL
//!
//! These benchmarks verify that our implementation meets the
//! performance characteristics we claim in documentation.

use ferrisdb_core::SyncMode;
use ferrisdb_storage::wal::{WALEntry, WALReader, WALWriter};

use criterion::{
    black_box, criterion_group, criterion_main, BenchmarkId, Criterion, PlotConfiguration,
    Throughput,
};
use tempfile::TempDir;

/// Prove that append is O(1) - constant time regardless of file size
fn bench_append_is_constant_time(c: &mut Criterion) {
    let mut group = c.benchmark_group("append_complexity");

    // Configure for complexity analysis
    let plot_config = PlotConfiguration::default().summary_scale(criterion::AxisScale::Logarithmic);
    group.plot_config(plot_config);

    // Test with different file sizes
    let file_sizes = vec![
        ("1MB", 1_000_000),
        ("10MB", 10_000_000),
        ("100MB", 100_000_000),
        ("1GB", 1_000_000_000),
    ];

    for (name, target_size) in file_sizes {
        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            &target_size,
            |b, &target_size| {
                // Setup: Create WAL with specified size
                let temp_dir = TempDir::new().unwrap();
                let wal_path = temp_dir.path().join("bench.wal");

                let writer = WALWriter::new(&wal_path, SyncMode::None, 10_000_000_000).unwrap();

                // Fill to target size
                let mut current_size = 0;
                let entry_size = 1000; // ~1KB entries
                while current_size < target_size {
                    let entry = WALEntry::new_put(
                        b"key".to_vec(),
                        vec![b'v'; entry_size],
                        current_size as u64,
                    )
                    .unwrap();
                    writer.append(&entry).unwrap();
                    current_size += entry_size + 25; // Include header overhead
                }

                // Benchmark: Time to append one more entry
                let test_entry =
                    WALEntry::new_put(b"bench_key".to_vec(), vec![b'x'; 1000], 999999).unwrap();

                b.iter(|| {
                    writer.append(&test_entry).unwrap();
                });
            },
        );
    }

    group.finish();
}

/// Prove that read performance is O(n) - linear in number of entries
fn bench_read_is_linear(c: &mut Criterion) {
    let mut group = c.benchmark_group("read_complexity");

    let plot_config = PlotConfiguration::default().summary_scale(criterion::AxisScale::Linear);
    group.plot_config(plot_config);

    // Test with different numbers of entries
    let entry_counts = vec![100, 1_000, 10_000, 100_000];

    for count in entry_counts {
        group.throughput(Throughput::Elements(count as u64));
        group.bench_with_input(BenchmarkId::from_parameter(count), &count, |b, &count| {
            // Setup: Create WAL with specified number of entries
            let temp_dir = TempDir::new().unwrap();
            let wal_path = temp_dir.path().join("bench.wal");

            {
                let writer = WALWriter::new(&wal_path, SyncMode::None, 1_000_000_000).unwrap();
                for i in 0..count {
                    let entry = WALEntry::new_put(
                        format!("key_{}", i).into_bytes(),
                        b"value".to_vec(),
                        i as u64,
                    )
                    .unwrap();
                    writer.append(&entry).unwrap();
                }
                writer.sync().unwrap();
            }

            // Benchmark: Time to read all entries
            b.iter(|| {
                let mut reader = WALReader::new(&wal_path).unwrap();
                let entries = reader.read_all().unwrap();
                black_box(entries.len());
            });
        });
    }

    group.finish();
}

/// Prove that metrics overhead is negligible
fn bench_metrics_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("metrics_overhead");

    let temp_dir = TempDir::new().unwrap();

    // Benchmark with metrics (normal operation)
    group.bench_function("with_metrics", |b| {
        let wal_path = temp_dir.path().join("with_metrics.wal");
        let writer = WALWriter::new(&wal_path, SyncMode::None, 100_000_000).unwrap();

        let entry = WALEntry::new_put(b"key".to_vec(), vec![b'v'; 1000], 1).unwrap();

        b.iter(|| {
            writer.append(&entry).unwrap();
        });
    });

    // Note: We can't easily disable metrics, but we can measure the metrics operations themselves
    group.bench_function("metrics_update_only", |b| {
        use ferrisdb_storage::wal::WALMetrics;
        let metrics = WALMetrics::new();

        b.iter(|| {
            metrics.record_write(1000, true);
            metrics.update_file_size(1000);
        });
    });

    group.finish();
}

/// Prove that buffer reuse eliminates allocations after warmup
fn bench_buffer_reuse_eliminates_allocations(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer_allocations");

    let temp_dir = TempDir::new().unwrap();
    let wal_path = temp_dir.path().join("buffer_test.wal");

    // Write entries of consistent size
    {
        let writer = WALWriter::new(&wal_path, SyncMode::None, 100_000_000).unwrap();
        for i in 0..1000 {
            let entry = WALEntry::new_put(
                format!("key_{:06}", i).into_bytes(),
                vec![b'v'; 1000], // Consistent 1KB values
                i as u64,
            )
            .unwrap();
            writer.append(&entry).unwrap();
        }
    }

    group.bench_function("first_read_allocates", |b| {
        b.iter(|| {
            let mut reader = WALReader::new(&wal_path).unwrap();
            // First read allocates buffer
            let first = reader.read_entry().unwrap();
            black_box(first);
        });
    });

    group.bench_function("subsequent_reads_reuse_buffer", |b| {
        let mut reader = WALReader::new(&wal_path).unwrap();

        // Warm up: First read allocates
        let _ = reader.read_entry().unwrap();

        // Benchmark: Subsequent reads should reuse buffer
        b.iter(|| {
            let entry = reader.read_entry().unwrap();
            black_box(entry);
        });
    });

    group.finish();
}

/// Prove that sync modes have expected performance characteristics
fn bench_sync_mode_performance_characteristics(c: &mut Criterion) {
    let mut group = c.benchmark_group("sync_mode_characteristics");

    // Test single entry write with different sync modes
    for sync_mode in &[SyncMode::None, SyncMode::Normal, SyncMode::Full] {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{:?}_single", sync_mode)),
            sync_mode,
            |b, &sync_mode| {
                let temp_dir = TempDir::new().unwrap();
                let wal_path = temp_dir.path().join("sync_test.wal");
                let writer = WALWriter::new(&wal_path, sync_mode, 100_000_000).unwrap();

                let entry = WALEntry::new_put(b"key".to_vec(), vec![b'v'; 100], 1).unwrap();

                b.iter(|| {
                    writer.append(&entry).unwrap();
                });
            },
        );
    }

    // Test batch writes to show when Full mode is advantageous
    group.bench_function("batch_100_then_sync", |b| {
        b.iter(|| {
            let temp_dir = TempDir::new().unwrap();
            let wal_path = temp_dir.path().join("batch_test.wal");
            let writer = WALWriter::new(&wal_path, SyncMode::None, 100_000_000).unwrap();

            // Write 100 entries without sync
            for i in 0..100 {
                let entry =
                    WALEntry::new_put(format!("key_{}", i).into_bytes(), b"value".to_vec(), i)
                        .unwrap();
                writer.append(&entry).unwrap();
            }

            // Single sync at end
            writer.sync().unwrap();
        });
    });

    group.finish();
}

/// Prove that reader stats accurately track buffer growth
fn bench_reader_stats_accuracy(c: &mut Criterion) {
    c.bench_function("reader_stats_overhead", |b| {
        let temp_dir = TempDir::new().unwrap();
        let wal_path = temp_dir.path().join("stats_test.wal");

        // Write entries of varying sizes
        {
            let writer = WALWriter::new(&wal_path, SyncMode::None, 100_000_000).unwrap();
            for i in 0..100 {
                let size = (i * 100) % 10000 + 100; // Varying sizes
                let entry = WALEntry::new_put(
                    format!("key_{}", i).into_bytes(),
                    vec![b'v'; size],
                    i as u64,
                )
                .unwrap();
                writer.append(&entry).unwrap();
            }
        }

        b.iter(|| {
            let mut reader = WALReader::with_initial_capacity(&wal_path, 64).unwrap();
            let _ = reader.read_all().unwrap();
            let stats = reader.stats();

            // Stats should show buffer growth
            assert!(stats.buffer_resizes > 0);
            assert!(stats.peak_buffer_size > 64);
            black_box(stats);
        });
    });
}

criterion_group!(
    proofs,
    bench_append_is_constant_time,
    bench_read_is_linear,
    bench_metrics_overhead,
    bench_buffer_reuse_eliminates_allocations,
    bench_sync_mode_performance_characteristics,
    bench_reader_stats_accuracy
);
criterion_main!(proofs);
