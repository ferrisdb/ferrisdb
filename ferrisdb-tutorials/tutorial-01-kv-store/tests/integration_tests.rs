//! Integration tests for the complete key-value store

use tutorial_01_kv_store::KeyValueStore;

#[test]
fn test_complete_functionality() {
    let mut store = KeyValueStore::new();
    
    // Verify empty state
    assert!(store.is_empty());
    assert_eq!(store.len(), 0);
    assert_eq!(store.get("any_key"), None);
    
    // Add some data
    store.set("user:1".to_string(), "Alice".to_string());
    store.set("user:2".to_string(), "Bob".to_string());
    store.set("product:1".to_string(), "Laptop".to_string());
    
    // Verify storage
    assert_eq!(store.len(), 3);
    assert!(!store.is_empty());
    
    // Verify retrieval
    assert_eq!(store.get("user:1"), Some("Alice".to_string()));
    assert_eq!(store.get("user:2"), Some("Bob".to_string()));
    assert_eq!(store.get("product:1"), Some("Laptop".to_string()));
    
    // Verify missing keys
    assert_eq!(store.get("user:3"), None);
    assert_eq!(store.get(""), None);
    
    // Test updates
    store.set("user:1".to_string(), "Alice Smith".to_string());
    assert_eq!(store.get("user:1"), Some("Alice Smith".to_string()));
    assert_eq!(store.len(), 3); // Count unchanged
}

#[test]
fn test_empty_keys_and_values() {
    let mut store = KeyValueStore::new();
    
    // Empty key and value should work
    store.set("".to_string(), "empty_key".to_string());
    store.set("empty_value".to_string(), "".to_string());
    
    assert_eq!(store.get(""), Some("empty_key".to_string()));
    assert_eq!(store.get("empty_value"), Some("".to_string()));
}

#[test]
fn test_unicode_support() {
    let mut store = KeyValueStore::new();
    
    // Test with various Unicode characters
    store.set("greeting:jp".to_string(), "こんにちは".to_string());
    store.set("greeting:es".to_string(), "¡Hola!".to_string());
    store.set("emoji:happy".to_string(), "😊".to_string());
    
    assert_eq!(store.get("greeting:jp"), Some("こんにちは".to_string()));
    assert_eq!(store.get("greeting:es"), Some("¡Hola!".to_string()));
    assert_eq!(store.get("emoji:happy"), Some("😊".to_string()));
}

#[test]
fn test_case_sensitivity() {
    let mut store = KeyValueStore::new();
    
    // Keys are case-sensitive
    store.set("Key".to_string(), "uppercase".to_string());
    store.set("key".to_string(), "lowercase".to_string());
    store.set("KEY".to_string(), "allcaps".to_string());
    
    assert_eq!(store.get("Key"), Some("uppercase".to_string()));
    assert_eq!(store.get("key"), Some("lowercase".to_string()));
    assert_eq!(store.get("KEY"), Some("allcaps".to_string()));
    assert_eq!(store.len(), 3);
}