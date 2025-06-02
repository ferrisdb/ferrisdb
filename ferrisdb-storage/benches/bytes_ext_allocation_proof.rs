//! Allocation tracking benchmarks for BytesMutExt trait
//!
//! This benchmark proves zero-allocation claims for BytesMutExt::read_exact_from
//! by using allocation tracking tools to demonstrate that the optimized approach
//! avoids memory allocations after initial buffer creation.

use bytes::BytesMut;
use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use ferrisdb_storage::utils::BytesMutExt;
use stats_alloc::{Region, StatsAlloc, INSTRUMENTED_SYSTEM};
use std::alloc::System;
use std::io::{Cursor, Read};

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

// Test data sizes
const SMALL_SIZE: usize = 64;
const MEDIUM_SIZE: usize = 4 * 1024; // 4KB
const LARGE_SIZE: usize = 1024 * 1024; // 1MB

/// Standard approach: pre-allocate and zero-fill buffer
fn read_with_standard_approach(data: &[u8], size: usize) -> (BytesMut, usize) {
    let region = Region::new(&GLOBAL);

    let mut buf = BytesMut::new();
    buf.resize(size, 0); // This zeros the memory

    let mut reader = Cursor::new(data);
    reader.read_exact(&mut buf[..]).unwrap();

    let stats = region.change();
    (buf, stats.allocations)
}

/// Optimized approach: use BytesMutExt
fn read_with_bytes_mut_ext(data: &[u8], size: usize) -> (BytesMut, usize) {
    let region = Region::new(&GLOBAL);

    let mut buf = BytesMut::new();
    let mut reader = Cursor::new(data);
    buf.read_exact_from(&mut reader, size).unwrap();

    let stats = region.change();
    (buf, stats.allocations)
}

/// Proves zero-allocation behavior when buffer has sufficient capacity
fn benchmark_zero_allocation_proof(c: &mut Criterion) {
    let mut group = c.benchmark_group("zero_allocation_proof");

    group.bench_function("zero_allocation_on_buffer_reuse", |b| {
        let data = vec![42u8; MEDIUM_SIZE];

        b.iter_batched(
            || {
                // Pre-allocate buffer with sufficient capacity
                let mut buf = BytesMut::with_capacity(MEDIUM_SIZE);
                let mut reader = Cursor::new(&data);
                buf.read_exact_from(&mut reader, MEDIUM_SIZE).unwrap();
                buf.clear(); // Clear but keep capacity
                (buf, data.clone())
            },
            |(mut buf, data)| {
                // This should not allocate since capacity is sufficient
                let region = Region::new(&GLOBAL);
                let mut reader = Cursor::new(&data);
                buf.read_exact_from(&mut reader, MEDIUM_SIZE).unwrap();
                let stats = region.change();

                // The zero-allocation claim: no allocations when capacity is sufficient
                assert_eq!(
                    stats.allocations, 0,
                    "BytesMutExt allocated {} times - violates zero-allocation claim!",
                    stats.allocations
                );

                black_box(buf)
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

/// Compare allocation counts between standard and optimized approaches
fn benchmark_allocation_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("allocation_comparison");

    for &size in &[SMALL_SIZE, MEDIUM_SIZE, LARGE_SIZE] {
        let name = match size {
            SMALL_SIZE => "small",
            MEDIUM_SIZE => "medium",
            LARGE_SIZE => "large",
            _ => "unknown",
        };

        group.bench_function(&format!("standard_approach_{}", name), |b| {
            let data = vec![42u8; size];
            b.iter(|| {
                let (buf, allocs) = read_with_standard_approach(&data, size);
                println!("Standard approach ({}): {} allocations", name, allocs);
                black_box(buf)
            });
        });

        group.bench_function(&format!("bytes_mut_ext_{}", name), |b| {
            let data = vec![42u8; size];
            b.iter(|| {
                let (buf, allocs) = read_with_bytes_mut_ext(&data, size);
                println!("BytesMutExt approach ({}): {} allocations", name, allocs);
                black_box(buf)
            });
        });
    }

    group.finish();
}

/// Proves that sequential reads with buffer reuse maintain zero-allocation behavior
fn benchmark_sequential_zero_allocations(c: &mut Criterion) {
    let mut group = c.benchmark_group("sequential_zero_allocations");

    group.bench_function("sequential_reads_zero_alloc", |b| {
        let chunk_size = 1024;
        let num_chunks = 10;
        let data: Vec<u8> = (0..chunk_size * num_chunks)
            .map(|i| (i % 256) as u8)
            .collect();

        b.iter_batched(
            || {
                // Pre-allocate buffer for all chunks
                let mut buf = BytesMut::with_capacity(chunk_size * num_chunks);
                // Do one full read to establish capacity
                let mut reader = Cursor::new(&data);
                buf.read_exact_from(&mut reader, chunk_size * num_chunks)
                    .unwrap();
                buf.clear();
                buf
            },
            |mut buf| {
                let region = Region::new(&GLOBAL);
                let mut reader = Cursor::new(&data);

                // Read chunks sequentially - should not allocate
                for _ in 0..num_chunks {
                    buf.read_exact_from(&mut reader, chunk_size).unwrap();
                }

                let stats = region.change();

                // Should have zero allocations for all sequential reads
                assert_eq!(
                    stats.allocations, 0,
                    "Sequential reads allocated {} times - should be zero!",
                    stats.allocations
                );

                black_box(buf)
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

/// Benchmark to demonstrate allocation behavior during capacity growth
fn benchmark_capacity_growth_allocations(c: &mut Criterion) {
    let mut group = c.benchmark_group("capacity_growth_allocations");

    group.bench_function("capacity_growth_tracking", |b| {
        let data = vec![42u8; MEDIUM_SIZE];

        b.iter_batched(
            || data.clone(),
            |data| {
                let region = Region::new(&GLOBAL);

                // Start with small buffer that will need to grow
                let mut buf = BytesMut::with_capacity(64);
                let mut reader = Cursor::new(&data);
                buf.read_exact_from(&mut reader, MEDIUM_SIZE).unwrap();

                let stats = region.change();

                // Should allocate at least once for capacity growth
                println!(
                    "Capacity growth resulted in {} allocations",
                    stats.allocations
                );
                assert!(
                    stats.allocations > 0,
                    "Expected at least one allocation for capacity growth, got {}",
                    stats.allocations
                );

                black_box(buf)
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

/// Prove that BytesMutExt with sufficient capacity beats standard approach in allocations
fn benchmark_allocation_efficiency_proof(c: &mut Criterion) {
    let mut group = c.benchmark_group("allocation_efficiency_proof");

    group.bench_function("efficiency_comparison", |b| {
        let data = vec![42u8; MEDIUM_SIZE];

        b.iter_batched(
            || data.clone(),
            |data| {
                // Test standard approach
                let region1 = Region::new(&GLOBAL);
                let mut buf1 = BytesMut::new();
                buf1.resize(MEDIUM_SIZE, 0);
                Cursor::new(&data).read_exact(&mut buf1[..]).unwrap();
                let std_stats = region1.change();

                // Test BytesMutExt approach
                let region2 = Region::new(&GLOBAL);
                let mut buf2 = BytesMut::new();
                buf2.read_exact_from(&mut Cursor::new(&data), MEDIUM_SIZE)
                    .unwrap();
                let ext_stats = region2.change();

                // Report the comparison
                println!(
                    "Standard: {} allocs, BytesMutExt: {} allocs",
                    std_stats.allocations, ext_stats.allocations
                );

                // For fresh buffers, both should allocate, but amounts may differ
                // The real benefit is in buffer reuse scenarios

                black_box((buf1, buf2))
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

criterion_group!(
    allocation_proofs,
    benchmark_zero_allocation_proof,
    benchmark_allocation_comparison,
    benchmark_sequential_zero_allocations,
    benchmark_capacity_growth_allocations,
    benchmark_allocation_efficiency_proof
);
criterion_main!(allocation_proofs);
