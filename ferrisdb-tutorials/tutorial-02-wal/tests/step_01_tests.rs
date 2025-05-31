//! Step 1: Basic text-based WAL tests

use tutorial_02_wal::Operation;
use tempfile::tempdir;
use std::fs;

#[test]
fn test_problem_without_wal() {
    // This test demonstrates the problem we're solving
    use std::collections::HashMap;
    
    struct VolatileStore {
        data: HashMap<String, String>,
    }
    
    impl VolatileStore {
        fn new() -> Self {
            Self { data: HashMap::new() }
        }
        
        fn set(&mut self, key: String, value: String) {
            self.data.insert(key, value);
        }
    }
    
    // Store some data
    let mut store = VolatileStore::new();
    store.set("balance".to_string(), "1000".to_string());
    
    // Simulate crash
    drop(store);
    
    // Try to recover...
    let new_store = VolatileStore::new();
    
    // Data is gone!
    assert!(new_store.data.is_empty());
}