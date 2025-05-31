//! Tests for Step 3: Adding the set() method

use tutorial_01_kv_store::KeyValueStore;

#[test]
fn step_03_can_store_values() {
    let mut store = KeyValueStore::new();

    // After Step 3, we can store values
    store.set("user:1".to_string(), "Alice".to_string());
    store.set("user:2".to_string(), "Bob".to_string());

    // We can verify the count
    assert_eq!(store.len(), 2);
    assert!(!store.is_empty());
}

#[test]
fn step_03_can_overwrite_values() {
    let mut store = KeyValueStore::new();

    // Store initial value
    store.set("key".to_string(), "value1".to_string());
    assert_eq!(store.len(), 1);

    // Overwrite with new value
    store.set("key".to_string(), "value2".to_string());

    // Count should still be 1
    assert_eq!(store.len(), 1);
}
