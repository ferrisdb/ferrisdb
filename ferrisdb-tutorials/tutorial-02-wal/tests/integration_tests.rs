//! Integration tests for the complete WAL implementation

use tutorial_02_wal::{Operation, WriteAheadLog, WalBuilder, SyncMode};
use tempfile::tempdir;
use std::collections::HashMap;
use std::thread;
use std::sync::{Arc, Mutex};

#[test]
fn test_full_workflow() {
    let dir = tempdir().unwrap();
    let wal_path = dir.path().join("workflow.wal");
    
    // Simulate a complete workflow
    let mut expected_state = HashMap::new();
    
    // Phase 1: Initial data load
    {
        let mut wal = WalBuilder::new(&wal_path)
            .sync_mode(SyncMode::Full)
            .build()
            .unwrap();
        
        // Add some users
        for i in 1..=10 {
            let key = format!("user:{}", i);
            let value = format!("User {}", i);
            
            wal.append(Operation::Set {
                key: key.clone(),
                value: value.clone(),
            }).unwrap();
            
            expected_state.insert(key, value);
        }
    }
    
    // Phase 2: Updates and deletes
    {
        let mut wal = WalBuilder::new(&wal_path).build().unwrap();
        
        // Update some users
        wal.append(Operation::Set {
            key: "user:1".to_string(),
            value: "Updated User 1".to_string(),
        }).unwrap();
        expected_state.insert("user:1".to_string(), "Updated User 1".to_string());
        
        // Delete some users
        wal.append(Operation::Delete {
            key: "user:5".to_string(),
        }).unwrap();
        expected_state.remove("user:5");
        
        wal.append(Operation::Delete {
            key: "user:7".to_string(),
        }).unwrap();
        expected_state.remove("user:7");
    }
    
    // Phase 3: Final recovery and verification
    {
        let wal = WalBuilder::new(&wal_path).build().unwrap();
        let entries = wal.recover_entries().unwrap();
        
        // Rebuild state from WAL
        let mut recovered_state = HashMap::new();
        for entry in entries {
            match entry.operation {
                Operation::Set { key, value } => {
                    recovered_state.insert(key, value);
                }
                Operation::Delete { key } => {
                    recovered_state.remove(&key);
                }
            }
        }
        
        // Verify states match
        assert_eq!(recovered_state, expected_state);
        assert_eq!(recovered_state.len(), 8); // 10 - 2 deleted
        assert_eq!(recovered_state.get("user:1"), Some(&"Updated User 1".to_string()));
        assert_eq!(recovered_state.get("user:5"), None);
        assert_eq!(recovered_state.get("user:7"), None);
    }
}

#[test]
fn test_concurrent_reads_during_write() {
    let dir = tempdir().unwrap();
    let wal_path = dir.path().join("concurrent.wal");
    
    // Create initial WAL with some data
    {
        let mut wal = WalBuilder::new(&wal_path).build().unwrap();
        
        for i in 0..5 {
            wal.append(Operation::Set {
                key: format!("initial:{}", i),
                value: format!("value:{}", i),
            }).unwrap();
        }
    }
    
    // Clone path for threads
    let read_path = wal_path.clone();
    let write_path = wal_path.clone();
    
    // Start a reader thread
    let reader = thread::spawn(move || {
        let wal = WalBuilder::new(&read_path).build().unwrap();
        let entries = wal.recover_entries().unwrap();
        entries.len()
    });
    
    // Start a writer thread
    let writer = thread::spawn(move || {
        let mut wal = WalBuilder::new(&write_path).build().unwrap();
        
        for i in 5..10 {
            wal.append(Operation::Set {
                key: format!("concurrent:{}", i),
                value: format!("value:{}", i),
            }).unwrap();
        }
    });
    
    // Wait for both to complete
    let initial_count = reader.join().unwrap();
    writer.join().unwrap();
    
    // Verify final state
    {
        let wal = WalBuilder::new(&wal_path).build().unwrap();
        let entries = wal.recover_entries().unwrap();
        
        assert!(entries.len() >= initial_count);
        assert!(entries.len() >= 5); // At least the initial entries
    }
}

#[test]
fn test_large_values() {
    let dir = tempdir().unwrap();
    let wal_path = dir.path().join("large.wal");
    
    // Create large value (1MB)
    let large_value = "x".repeat(1024 * 1024);
    
    // Write it
    {
        let mut wal = WalBuilder::new(&wal_path).build().unwrap();
        
        wal.append(Operation::Set {
            key: "large".to_string(),
            value: large_value.clone(),
        }).unwrap();
    }
    
    // Recover and verify
    {
        let wal = WalBuilder::new(&wal_path).build().unwrap();
        let entries = wal.recover_entries().unwrap();
        
        assert_eq!(entries.len(), 1);
        match &entries[0].operation {
            Operation::Set { key, value } => {
                assert_eq!(key, "large");
                assert_eq!(value.len(), 1024 * 1024);
                assert_eq!(value, &large_value);
            }
            _ => panic!("Expected Set operation"),
        }
    }
}

#[test]
fn test_special_characters() {
    let dir = tempdir().unwrap();
    let wal_path = dir.path().join("special.wal");
    
    let test_cases = vec![
        ("emoji", "🚀🦀💾"),
        ("unicode", "Hello 世界"),
        ("newlines", "line1\nline2\nline3"),
        ("tabs", "col1\tcol2\tcol3"),
        ("quotes", r#"She said "Hello!""#),
        ("null_bytes", "before\0after"),
    ];
    
    // Write all test cases
    {
        let mut wal = WalBuilder::new(&wal_path).build().unwrap();
        
        for (key, value) in &test_cases {
            wal.append(Operation::Set {
                key: key.to_string(),
                value: value.to_string(),
            }).unwrap();
        }
    }
    
    // Recover and verify
    {
        let wal = WalBuilder::new(&wal_path).build().unwrap();
        let entries = wal.recover_entries().unwrap();
        
        assert_eq!(entries.len(), test_cases.len());
        
        for (i, entry) in entries.iter().enumerate() {
            match &entry.operation {
                Operation::Set { key, value } => {
                    assert_eq!(key, &test_cases[i].0);
                    assert_eq!(value, &test_cases[i].1);
                }
                _ => panic!("Expected Set operation"),
            }
        }
    }
}