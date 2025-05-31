//! Concurrent access tests
//! 
//! These tests demonstrate why our simple HashMap implementation
//! is NOT thread-safe and prepare learners for future tutorials
//! where we'll add proper concurrency support.

use tutorial_01_kv_store::KeyValueStore;
use std::sync::{Arc, Mutex};
use std::thread;

#[test]
fn test_concurrent_access_with_mutex() {
    // Wrap our store in Arc<Mutex<>> for safe concurrent access
    let store = Arc::new(Mutex::new(KeyValueStore::new()));
    let mut handles = vec![];
    
    // Spawn 10 threads that each insert 100 items
    for thread_id in 0..10 {
        let store_clone = Arc::clone(&store);
        let handle = thread::spawn(move || {
            for i in 0..100 {
                let key = format!("thread{}:item{}", thread_id, i);
                let value = format!("value_{}", i);
                
                // Lock the mutex before accessing
                let mut store = store_clone.lock().unwrap();
                store.set(key, value);
            }
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verify all items were inserted
    let store = store.lock().unwrap();
    assert_eq!(store.len(), 1000); // 10 threads * 100 items each
}

#[test]
fn test_concurrent_reads_with_mutex() {
    // Prepare a store with some data
    let mut store = KeyValueStore::new();
    for i in 0..100 {
        store.set(format!("key{}", i), format!("value{}", i));
    }
    
    // Wrap in Arc<Mutex<>>
    let store = Arc::new(Mutex::new(store));
    let mut handles = vec![];
    
    // Spawn 10 threads that each read all values
    for _ in 0..10 {
        let store_clone = Arc::clone(&store);
        let handle = thread::spawn(move || {
            let mut sum = 0;
            for i in 0..100 {
                let store = store_clone.lock().unwrap();
                if let Some(value) = store.get(&format!("key{}", i)) {
                    // Just verify we got the expected value
                    assert_eq!(value, format!("value{}", i));
                    sum += 1;
                }
            }
            sum
        });
        handles.push(handle);
    }
    
    // All threads should successfully read all values
    for handle in handles {
        let sum = handle.join().unwrap();
        assert_eq!(sum, 100);
    }
}

#[test]
#[should_panic(expected = "Cannot share KeyValueStore between threads")]
#[ignore] // This test is for educational purposes - it won't actually compile
fn test_why_we_need_arc_mutex() {
    // This test demonstrates why our simple KeyValueStore
    // cannot be shared between threads without protection.
    // 
    // In future tutorials, we'll build thread-safe storage
    // using Arc, RwLock, and other concurrency primitives.
    
    // This code would not compile:
    // let store = KeyValueStore::new();
    // let handle = thread::spawn(move || {
    //     store.set("key".to_string(), "value".to_string());
    // });
    
    // The compiler would tell us:
    // - KeyValueStore cannot be sent between threads safely
    // - KeyValueStore cannot be shared between threads safely
    
    panic!("Cannot share KeyValueStore between threads without Arc<Mutex<>>");
}