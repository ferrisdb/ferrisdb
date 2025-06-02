//! Benchmarks for BytesMutExt trait
//!
//! This benchmark compares the performance of BytesMutExt::read_exact_from
//! versus the standard approach of pre-allocating and zeroing a buffer.

use bytes::BytesMut;
use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use ferrisdb_storage::utils::BytesMutExt;
use std::io::{Cursor, Read};

// Benchmark data sizes
const SMALL_SIZE: usize = 64;
const MEDIUM_SIZE: usize = 4 * 1024; // 4KB
const LARGE_SIZE: usize = 1024 * 1024; // 1MB
const HUGE_SIZE: usize = 16 * 1024 * 1024; // 16MB

/// Standard approach: pre-allocate and zero-fill buffer
fn read_with_standard_approach(data: &[u8], size: usize) -> BytesMut {
    let mut buf = BytesMut::new();
    buf.resize(size, 0); // This zeros the memory

    let mut reader = Cursor::new(data);
    reader.read_exact(&mut buf[..]).unwrap();

    buf
}

/// Optimized approach: use BytesMutExt
fn read_with_bytes_mut_ext(data: &[u8], size: usize) -> BytesMut {
    let mut buf = BytesMut::new();
    let mut reader = Cursor::new(data);
    buf.read_exact_from(&mut reader, size).unwrap();
    buf
}

fn benchmark_small_reads(c: &mut Criterion) {
    let mut group = c.benchmark_group("bytes_ext_small_reads");
    let data = vec![42u8; SMALL_SIZE];

    group.bench_function("standard_approach", |b| {
        b.iter(|| black_box(read_with_standard_approach(&data, SMALL_SIZE)));
    });

    group.bench_function("bytes_mut_ext", |b| {
        b.iter(|| black_box(read_with_bytes_mut_ext(&data, SMALL_SIZE)));
    });

    group.finish();
}

fn benchmark_medium_reads(c: &mut Criterion) {
    let mut group = c.benchmark_group("bytes_ext_medium_reads");
    let data = vec![42u8; MEDIUM_SIZE];

    group.bench_function("standard_approach", |b| {
        b.iter(|| black_box(read_with_standard_approach(&data, MEDIUM_SIZE)));
    });

    group.bench_function("bytes_mut_ext", |b| {
        b.iter(|| black_box(read_with_bytes_mut_ext(&data, MEDIUM_SIZE)));
    });

    group.finish();
}

fn benchmark_large_reads(c: &mut Criterion) {
    let mut group = c.benchmark_group("bytes_ext_large_reads");
    let data = vec![42u8; LARGE_SIZE];

    group.bench_function("standard_approach", |b| {
        b.iter(|| black_box(read_with_standard_approach(&data, LARGE_SIZE)));
    });

    group.bench_function("bytes_mut_ext", |b| {
        b.iter(|| black_box(read_with_bytes_mut_ext(&data, LARGE_SIZE)));
    });

    group.finish();
}

fn benchmark_huge_reads(c: &mut Criterion) {
    let mut group = c.benchmark_group("bytes_ext_huge_reads");
    group.sample_size(20); // Reduce sample size for huge buffers

    let data = vec![42u8; HUGE_SIZE];

    group.bench_function("standard_approach", |b| {
        b.iter(|| black_box(read_with_standard_approach(&data, HUGE_SIZE)));
    });

    group.bench_function("bytes_mut_ext", |b| {
        b.iter(|| black_box(read_with_bytes_mut_ext(&data, HUGE_SIZE)));
    });

    group.finish();
}

fn benchmark_sequential_reads(c: &mut Criterion) {
    let mut group = c.benchmark_group("bytes_ext_sequential_reads");

    // Simulate reading multiple chunks sequentially (like WAL entries)
    let chunk_size = 1024; // 1KB chunks
    let num_chunks = 100;
    let data: Vec<u8> = (0..chunk_size * num_chunks)
        .map(|i| (i % 256) as u8)
        .collect();

    group.bench_function("standard_approach", |b| {
        b.iter_batched(
            || BytesMut::new(),
            |mut buf| {
                let mut reader = Cursor::new(&data);
                for _ in 0..num_chunks {
                    let start = buf.len();
                    buf.resize(start + chunk_size, 0);
                    reader.read_exact(&mut buf[start..]).unwrap();
                }
                black_box(buf)
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("bytes_mut_ext", |b| {
        b.iter_batched(
            || BytesMut::new(),
            |mut buf| {
                let mut reader = Cursor::new(&data);
                for _ in 0..num_chunks {
                    buf.read_exact_from(&mut reader, chunk_size).unwrap();
                }
                black_box(buf)
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

fn benchmark_memory_reuse(c: &mut Criterion) {
    let mut group = c.benchmark_group("bytes_ext_memory_reuse");

    // Test performance when reusing the same buffer
    let data = vec![42u8; MEDIUM_SIZE];

    group.bench_function("standard_approach", |b| {
        let mut buf = BytesMut::with_capacity(MEDIUM_SIZE);
        b.iter(|| {
            buf.clear();
            buf.resize(MEDIUM_SIZE, 0);
            let mut reader = Cursor::new(&data);
            reader.read_exact(&mut buf[..]).unwrap();
            black_box(&buf);
        });
    });

    group.bench_function("bytes_mut_ext", |b| {
        let mut buf = BytesMut::with_capacity(MEDIUM_SIZE);
        b.iter(|| {
            buf.clear();
            let mut reader = Cursor::new(&data);
            buf.read_exact_from(&mut reader, MEDIUM_SIZE).unwrap();
            black_box(&buf);
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_small_reads,
    benchmark_medium_reads,
    benchmark_large_reads,
    benchmark_huge_reads,
    benchmark_sequential_reads,
    benchmark_memory_reuse
);
criterion_main!(benches);
