//! Tests for Step 2: Adding HashMap storage

use tutorial_01_kv_store::KeyValueStore;

#[test]
fn step_02_struct_with_hashmap() {
    // After Step 2, we have a struct with HashMap field
    let store = KeyValueStore::new();
    
    // We can check it's empty
    assert!(store.is_empty());
    assert_eq!(store.len(), 0);
}