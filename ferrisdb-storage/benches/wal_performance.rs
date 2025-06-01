use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ferrisdb_storage::wal::WALEntry;

fn bench_encode_small_entry(c: &mut Criterion) {
    let entry =
        WALEntry::new_put(b"key".to_vec(), b"value".to_vec(), 123).expect("Failed to create entry");

    c.bench_function("encode_small_entry", |b| {
        b.iter(|| black_box(entry.encode().expect("Failed to encode")))
    });
}

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

fn bench_decode_small_entry(c: &mut Criterion) {
    let entry =
        WALEntry::new_put(b"key".to_vec(), b"value".to_vec(), 123).expect("Failed to create entry");
    let encoded = entry.encode().expect("Failed to encode");

    c.bench_function("decode_small_entry", |b| {
        b.iter(|| black_box(WALEntry::decode(&encoded).expect("Failed to decode")))
    });
}

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
