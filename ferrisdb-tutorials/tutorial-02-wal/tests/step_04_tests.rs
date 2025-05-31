//! Step 4: Recovery tests

use tutorial_02_wal::{Operation, WriteAheadLog, WalBuilder, SyncMode};
use tempfile::tempdir;
use std::collections::HashMap;

#[test]
fn test_basic_recovery() {
    let dir = tempdir().unwrap();
    let wal_path = dir.path().join("recovery.wal");
    
    // Phase 1: Write operations
    {
        let mut wal = WalBuilder::new(&wal_path)
            .sync_mode(SyncMode::Full)
            .build()
            .unwrap();
        
        wal.append(Operation::Set {
            key: "user:1".to_string(),
            value: "Alice".to_string(),
        }).unwrap();
        
        wal.append(Operation::Set {
            key: "user:2".to_string(),
            value: "Bob".to_string(),
        }).unwrap();
        
        wal.append(Operation::Delete {
            key: "user:1".to_string(),
        }).unwrap();
        
        wal.append(Operation::Set {
            key: "user:3".to_string(),
            value: "Charlie".to_string(),
        }).unwrap();
    }
    
    // Phase 2: Recover and verify
    {
        let wal = WalBuilder::new(&wal_path).build().unwrap();
        let entries = wal.recover_entries().unwrap();
        
        assert_eq!(entries.len(), 4);
        
        // Verify sequence numbers
        for (i, entry) in entries.iter().enumerate() {
            assert_eq!(entry.sequence, i as u64);
        }
        
        // Rebuild state
        let mut state = HashMap::new();
        for entry in entries {
            match entry.operation {
                Operation::Set { key, value } => {
                    state.insert(key, value);
                }
                Operation::Delete { key } => {
                    state.remove(&key);
                }
            }
        }
        
        // Verify final state
        assert_eq!(state.get("user:1"), None); // Deleted
        assert_eq!(state.get("user:2"), Some(&"Bob".to_string()));
        assert_eq!(state.get("user:3"), Some(&"Charlie".to_string()));
    }
}

#[test]
fn test_recovery_continues_sequence() {
    let dir = tempdir().unwrap();
    let wal_path = dir.path().join("sequence.wal");
    
    // Write some entries
    {
        let mut wal = WalBuilder::new(&wal_path).build().unwrap();
        
        for i in 0..5 {
            wal.append(Operation::Set {
                key: format!("key:{}", i),
                value: format!("value:{}", i),
            }).unwrap();
        }
    }
    
    // Recover and add more
    {
        let mut wal = WalBuilder::new(&wal_path).build().unwrap();
        
        // Should continue from sequence 5
        let seq = wal.append(Operation::Set {
            key: "new".to_string(),
            value: "entry".to_string(),
        }).unwrap();
        
        assert_eq!(seq, 5);
    }
    
    // Final recovery check
    {
        let wal = WalBuilder::new(&wal_path).build().unwrap();
        let entries = wal.recover_entries().unwrap();
        
        assert_eq!(entries.len(), 6);
        assert_eq!(entries.last().unwrap().sequence, 5);
    }
}

#[test]
fn test_empty_wal_recovery() {
    let dir = tempdir().unwrap();
    let wal_path = dir.path().join("empty.wal");
    
    // Create and immediately close
    {
        let _wal = WalBuilder::new(&wal_path).build().unwrap();
    }
    
    // Recover from empty WAL
    {
        let wal = WalBuilder::new(&wal_path).build().unwrap();
        let entries = wal.recover_entries().unwrap();
        
        assert_eq!(entries.len(), 0);
    }
}