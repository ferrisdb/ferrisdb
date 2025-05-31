//! Tests for Step 1: Creating the basic struct

use tutorial_01_kv_store::KeyValueStore;

#[test]
fn step_01_create_empty_struct() {
    // This test proves that Step 1 compiles and runs
    let store = KeyValueStore::new();

    // At this point, we just verify it exists
    // In the tutorial, this is all we have after Step 1
    let _ = store; // Use it to avoid warning
}
