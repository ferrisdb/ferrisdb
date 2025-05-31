//! Step 5: Sync mode tests

use tutorial_02_wal::{Operation, WriteAheadLog, WalBuilder, SyncMode};
use tempfile::tempdir;
use std::time::Instant;

#[test]
fn test_sync_modes() {
    let dir = tempdir().unwrap();
    
    // Test that all sync modes work
    for mode in [SyncMode::None, SyncMode::DataOnly, SyncMode::Full] {
        let wal_path = dir.path().join(format!("{:?}.wal", mode));
        
        let mut wal = WalBuilder::new(&wal_path)
            .sync_mode(mode)
            .build()
            .unwrap();
        
        wal.append(Operation::Set {
            key: "test".to_string(),
            value: "data".to_string(),
        }).unwrap();
        
        // Verify file exists and has data
        assert!(wal_path.exists());
        let metadata = std::fs::metadata(&wal_path).unwrap();
        assert!(metadata.len() > 8); // At least header + some data
    }
}

#[test]
#[ignore] // This test is slow, run with: cargo test -- --ignored
fn test_sync_mode_performance() {
    let dir = tempdir().unwrap();
    let iterations = 100;
    
    for mode in [SyncMode::None, SyncMode::DataOnly, SyncMode::Full] {
        let wal_path = dir.path().join(format!("perf_{:?}.wal", mode));
        
        let mut wal = WalBuilder::new(&wal_path)
            .sync_mode(mode)
            .build()
            .unwrap();
        
        let start = Instant::now();
        
        for i in 0..iterations {
            wal.append(Operation::Set {
                key: format!("key:{}", i),
                value: format!("value:{}", i),
            }).unwrap();
        }
        
        let elapsed = start.elapsed();
        let ops_per_sec = iterations as f64 / elapsed.as_secs_f64();
        
        println!("SyncMode::{:?}: {:.0} ops/sec", mode, ops_per_sec);
        
        // Basic sanity check - None should be fastest
        match mode {
            SyncMode::None => assert!(ops_per_sec > 1000.0),
            SyncMode::DataOnly => assert!(ops_per_sec > 100.0),
            SyncMode::Full => assert!(ops_per_sec > 10.0),
        }
    }
}

#[test]
fn test_builder_pattern() {
    let dir = tempdir().unwrap();
    let wal_path = dir.path().join("builder.wal");
    
    // Test builder with all options
    let mut wal = WalBuilder::new(&wal_path)
        .sync_mode(SyncMode::DataOnly)
        .max_file_size(1024 * 1024) // 1MB
        .build()
        .unwrap();
    
    // Should work normally
    let seq = wal.append(Operation::Set {
        key: "test".to_string(),
        value: "builder".to_string(),
    }).unwrap();
    
    assert_eq!(seq, 0);
    
    // Verify it recovers correctly
    drop(wal);
    
    let wal = WalBuilder::new(&wal_path).build().unwrap();
    let entries = wal.recover_entries().unwrap();
    
    assert_eq!(entries.len(), 1);
    match &entries[0].operation {
        Operation::Set { key, value } => {
            assert_eq!(key, "test");
            assert_eq!(value, "builder");
        }
        _ => panic!("Expected Set operation"),
    }
}