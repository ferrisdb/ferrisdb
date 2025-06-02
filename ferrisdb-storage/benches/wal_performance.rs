use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ferrisdb_storage::wal::WALEntry;

/// Benchmarks encoding performance for small entries.
///
/// Measures:
/// - Serialization overhead for typical small entries
/// - Fixed overhead of encoding process
/// - Performance baseline for comparison
/// - Typical case: small keys and values
fn bench_encode_small_entry(c: &mut Criterion) {
    let entry =
        WALEntry::new_put(b"key".to_vec(), b"value".to_vec(), 123).expect("Failed to create entry");

    c.bench_function("encode_small_entry", |b| {
        b.iter(|| black_box(entry.encode().expect("Failed to encode")))
    });
}

/// Benchmarks encoding performance for large entries.
///
/// Measures:
/// - Encoding throughput for larger data
/// - Scaling characteristics with size
/// - Memory allocation impact
/// - Realistic case: 1KB key + 64KB value
fn bench_encode_large_entry(c: &mut Criterion) {
    let entry = WALEntry::new_put(
        vec![0u8; 1024],      // 1KB key
        vec![0u8; 64 * 1024], // 64KB value
        123,
    )
    .expect("Failed to create entry");

    c.bench_function("encode_large_entry", |b| {
        b.iter(|| black_box(entry.encode().expect("Failed to encode")))
    });
}

/// Benchmarks decoding performance for small entries.
///
/// Measures:
/// - Deserialization overhead
/// - Validation and checksum verification cost
/// - Typical read performance
/// - Baseline for optimization efforts
fn bench_decode_small_entry(c: &mut Criterion) {
    let entry =
        WALEntry::new_put(b"key".to_vec(), b"value".to_vec(), 123).expect("Failed to create entry");
    let encoded = entry.encode().expect("Failed to encode");

    c.bench_function("decode_small_entry", |b| {
        b.iter(|| black_box(WALEntry::decode(&encoded).expect("Failed to decode")))
    });
}

/// Benchmarks decoding performance for large entries.
///
/// Measures:
/// - Decoding throughput at scale
/// - Memory allocation during decode
/// - Checksum verification on large data
/// - Performance with realistic large entries
fn bench_decode_large_entry(c: &mut Criterion) {
    let entry = WALEntry::new_put(
        vec![0u8; 1024],      // 1KB key
        vec![0u8; 64 * 1024], // 64KB value
        123,
    )
    .expect("Failed to create entry");
    let encoded = entry.encode().expect("Failed to encode");

    c.bench_function("decode_large_entry", |b| {
        b.iter(|| black_box(WALEntry::decode(&encoded).expect("Failed to decode")))
    });
}

/// Benchmarks CRC32 checksum calculation performance.
///
/// Measures:
/// - Raw checksum computation speed
/// - Performance of crc32fast library
/// - Overhead added to encode/decode
/// - Helps identify if checksum is bottleneck
fn bench_checksum_calculation(c: &mut Criterion) {
    use crc32fast::Hasher;
    let data = vec![0u8; 1024]; // 1KB of data

    c.bench_function("checksum_calculation", |b| {
        b.iter(|| {
            let mut hasher = Hasher::new();
            hasher.update(&data);
            black_box(hasher.finalize())
        })
    });
}

criterion_group!(
    benches,
    bench_encode_small_entry,
    bench_encode_large_entry,
    bench_decode_small_entry,
    bench_decode_large_entry,
    bench_checksum_calculation
);
criterion_main!(benches);
