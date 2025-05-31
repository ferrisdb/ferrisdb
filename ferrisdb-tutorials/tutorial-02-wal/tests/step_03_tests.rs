//! Step 3: Checksum tests

use tutorial_02_wal::{Operation, WriteAheadLog, WalBuilder, WalError};
use tempfile::tempdir;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};

#[test]
fn test_checksum_detection() {
    let dir = tempdir().unwrap();
    let wal_path = dir.path().join("checksum.wal");
    
    // Write some data
    {
        let mut wal = WalBuilder::new(&wal_path).build().unwrap();
        wal.append(Operation::Set {
            key: "important".to_string(),
            value: "data".to_string(),
        }).unwrap();
    }
    
    // Corrupt the file
    {
        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .open(&wal_path)
            .unwrap();
        
        // Skip header (8 bytes) and length (4 bytes)
        file.seek(SeekFrom::Start(12)).unwrap();
        
        // Read a byte and flip some bits
        let mut byte = [0u8; 1];
        file.read_exact(&mut byte).unwrap();
        byte[0] ^= 0xFF; // Flip all bits
        
        // Write it back
        file.seek(SeekFrom::Start(12)).unwrap();
        file.write_all(&byte).unwrap();
    }
    
    // Try to recover - should detect corruption
    let wal = WalBuilder::new(&wal_path).build().unwrap();
    let entries = wal.recover_entries().unwrap();
    
    // Should have no entries due to checksum failure
    assert_eq!(entries.len(), 0);
}

#[test]
fn test_partial_write_recovery() {
    let dir = tempdir().unwrap();
    let wal_path = dir.path().join("partial.wal");
    
    // Write some complete entries
    {
        let mut wal = WalBuilder::new(&wal_path).build().unwrap();
        
        wal.append(Operation::Set {
            key: "complete1".to_string(),
            value: "data1".to_string(),
        }).unwrap();
        
        wal.append(Operation::Set {
            key: "complete2".to_string(),
            value: "data2".to_string(),
        }).unwrap();
    }
    
    // Simulate partial write by truncating file
    {
        let file = OpenOptions::new()
            .write(true)
            .open(&wal_path)
            .unwrap();
        
        let original_len = file.metadata().unwrap().len();
        // Truncate in the middle of the last entry (but not too much)
        if original_len > 20 {
            file.set_len(original_len - 5).unwrap();
        }
    }
    
    // Recovery should get the complete entries
    let wal = WalBuilder::new(&wal_path).build().unwrap();
    let entries = wal.recover_entries().unwrap();
    
    // Should have at least the first entry
    assert!(entries.len() >= 1);
    match &entries[0].operation {
        Operation::Set { key, value } => {
            assert_eq!(key, "complete1");
            assert_eq!(value, "data1");
        }
        _ => panic!("Expected Set operation"),
    }
}