//! Step 2: Binary format tests

use tutorial_02_wal::{Operation, LogEntry, WriteAheadLog, WalBuilder, SyncMode};
use tempfile::tempdir;

#[test]
fn test_binary_encoding_decoding() {
    let dir = tempdir().unwrap();
    let wal_path = dir.path().join("binary.wal");
    
    // Test encoding various operations
    let mut wal = WalBuilder::new(&wal_path).build().unwrap();
    
    // Test simple strings
    let seq1 = wal.append(Operation::Set {
        key: "test".to_string(),
        value: "data".to_string(),
    }).unwrap();
    
    assert_eq!(seq1, 0);
    
    // Test Unicode strings
    let seq2 = wal.append(Operation::Set {
        key: "emoji".to_string(),
        value: "🚀🦀".to_string(),
    }).unwrap();
    
    assert_eq!(seq2, 1);
    
    // Test delete operation
    let seq3 = wal.append(Operation::Delete {
        key: "temp".to_string(),
    }).unwrap();
    
    assert_eq!(seq3, 2);
}

#[test]
fn test_binary_format_efficiency() {
    let dir = tempdir().unwrap();
    let wal_path = dir.path().join("efficiency.wal");
    
    let mut wal = WalBuilder::new(&wal_path).build().unwrap();
    
    // Add some data
    wal.append(Operation::Set {
        key: "user:123456789".to_string(),
        value: "Alice Johnson".to_string(),
    }).unwrap();
    
    // Check file size is reasonable
    let metadata = std::fs::metadata(&wal_path).unwrap();
    let file_size = metadata.len();
    
    // Should be less than 100 bytes for this small entry
    // (header + length + sequence + op type + key/value lengths + data + checksum)
    assert!(file_size < 100, "File too large: {} bytes", file_size);
}