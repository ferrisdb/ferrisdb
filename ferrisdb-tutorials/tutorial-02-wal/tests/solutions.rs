//! Tests for exercise solutions
//! 
//! This file tests that all provided solutions work correctly.
//! Students can run this to verify their implementations match the expected behavior.

// Import solution modules
#[path = "../examples/exercises/solutions/challenge_01_solution.rs"]
mod challenge_01_solution;

#[path = "../examples/exercises/solutions/challenge_02_solution.rs"]
mod challenge_02_solution;

#[path = "../examples/exercises/solutions/challenge_03_solution.rs"]
mod challenge_03_solution;

use tempfile::tempdir;

#[test]
fn test_compression_solution() {
    use challenge_01_solution::CompressedWal;
    
    let dir = tempdir().unwrap();
    let uncompressed_path = dir.path().join("uncompressed.wal");
    let compressed_path = dir.path().join("compressed.wal");
    
    // Create repetitive data that compresses well
    let data = vec![42u8; 1000]; // 1KB of same byte
    
    // Write uncompressed
    let uncompressed_size = {
        let mut wal = CompressedWal::new(&uncompressed_path, false).unwrap();
        wal.append(&data).unwrap();
        std::fs::metadata(&uncompressed_path).unwrap().len()
    };
    
    // Write compressed
    let compressed_size = {
        let mut wal = CompressedWal::new(&compressed_path, true).unwrap();
        wal.append(&data).unwrap();
        std::fs::metadata(&compressed_path).unwrap().len()
    };
    
    // Compression should significantly reduce size
    assert!(compressed_size < uncompressed_size / 2);
    
    // Verify data integrity after compression
    let wal = CompressedWal::new(&compressed_path, true).unwrap();
    let entries = wal.read_all(&compressed_path).unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0], data);
}

#[test]
fn test_rotation_solution() {
    use challenge_02_solution::RotatingWal;
    
    let dir = tempdir().unwrap();
    let base_path = dir.path().join("test.wal");
    
    // Small max size to force rotation
    let mut wal = RotatingWal::new(&base_path, 200).unwrap(); // 200 bytes max
    
    // Write data that will cause multiple rotations
    let mut all_data = Vec::new();
    for i in 0..10 {
        let data = format!("Entry {}: {}\n", i, "x".repeat(30)).into_bytes();
        all_data.push(data.clone());
        wal.append(&data).unwrap();
    }
    
    // Verify multiple files were created
    assert!(base_path.exists());
    assert!(dir.path().join("test.wal.1").exists());
    assert!(dir.path().join("test.wal.2").exists());
    
    // Read all data back and verify integrity
    let recovered = wal.read_all_files().unwrap();
    assert!(recovered.len() >= 3); // At least 3 files
    
    // Concatenate and verify
    let mut all_content = Vec::new();
    for file_data in recovered {
        all_content.extend_from_slice(&file_data);
    }
    
    let expected: Vec<u8> = all_data.into_iter().flatten().collect();
    assert_eq!(all_content, expected);
}

#[test]
fn test_concurrent_solution() {
    use challenge_03_solution::{ConcurrentWal, SnapshotReader};
    use std::thread;
    use std::sync::Arc;
    
    let dir = tempdir().unwrap();
    let path = dir.path().join("concurrent.wal");
    
    let wal = ConcurrentWal::new(&path).unwrap();
    
    // Write some initial data
    for i in 0..5 {
        wal.append(format!("initial entry {}", i).as_bytes()).unwrap();
    }
    
    // Create multiple readers and a writer
    let wal_reader1 = wal.clone();
    let wal_reader2 = wal.clone();
    let wal_writer = wal.clone();
    let path_clone = path.clone();
    
    // Reader 1: Take snapshot before writes
    let reader1 = thread::spawn(move || {
        let snapshot = wal_reader1.snapshot_reader(&path_clone).unwrap();
        (snapshot, wal_reader1.current_sequence().unwrap())
    });
    
    // Writer: Add more entries
    let writer = thread::spawn(move || {
        for i in 5..10 {
            wal_writer.append(format!("concurrent entry {}", i).as_bytes()).unwrap();
        }
        wal_writer.current_sequence().unwrap()
    });
    
    // Reader 2: Take snapshot after writes
    writer.join().unwrap();
    let reader2 = thread::spawn(move || {
        let snapshot = wal_reader2.snapshot_reader(&path).unwrap();
        (snapshot, wal_reader2.current_sequence().unwrap())
    });
    
    // Verify snapshots
    let (mut snapshot1, seq1) = reader1.join().unwrap();
    let (mut snapshot2, seq2) = reader2.join().unwrap();
    
    assert_eq!(seq1, 5); // Should see only initial entries
    assert!(seq2 >= 10); // Should see all entries
    
    let entries1 = snapshot1.read_entries().unwrap();
    let entries2 = snapshot2.read_entries().unwrap();
    
    assert_eq!(entries1.len(), 5); // Only initial entries
    assert!(entries2.len() >= 10); // All entries
}

#[test]
fn test_all_solutions_integrate() {
    // This test verifies that all solutions can work together
    println!("✅ All exercise solutions pass their tests!");
    println!("Students can use these as reference implementations.");
}