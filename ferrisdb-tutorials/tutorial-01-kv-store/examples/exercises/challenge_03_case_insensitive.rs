//! Challenge 3: Make keys case-insensitive
//!
//! Modify the store to treat keys case-insensitively while preserving
//! the original key casing.
//!
//! Requirements:
//! - `get("KEY")` should return the value for `"key"`
//! - Store should remember the original casing
//! - `keys()` method should return original casings
//!
//! Hints:
//! - You'll need to store both the normalized key and original key
//! - Consider using `.to_lowercase()` for normalization
//! - Think about what happens when you set "Key" then "KEY"

// Allow warnings for educational exercise templates
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::collections::HashMap;

/// A case-insensitive key-value store
pub struct CaseInsensitiveStore {
    // TODO: Change this to handle case-insensitive lookups
    // while preserving original key casing
    data: HashMap<String, String>,
}

impl CaseInsensitiveStore {
    pub fn new() -> Self {
        CaseInsensitiveStore {
            data: HashMap::new(),
        }
    }

    /// Set a key-value pair (case-insensitive key)
    pub fn set(&mut self, key: String, value: String) {
        // TODO: Implement this
        // Remember: "KEY", "Key", and "key" should all map to the same entry
    }

    /// Get a value by key (case-insensitive)
    pub fn get(&self, key: &str) -> Option<String> {
        // TODO: Implement this
        // Hint: Normalize the key before looking it up
        None
    }

    /// Get all keys in their original casing
    pub fn keys(&self) -> Vec<String> {
        // TODO: Implement this
        // Should return the original casing of keys
        vec![]
    }

    /// Remove a key-value pair (case-insensitive)
    pub fn delete(&mut self, key: &str) -> Option<String> {
        // TODO: Implement this
        None
    }

    /// Get the number of entries
    pub fn len(&self) -> usize {
        // TODO: Implement this
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_and_get_work_case_insensitively() {
        let mut store = CaseInsensitiveStore::new();

        store.set("Hello".to_string(), "world".to_string());

        // All these should return the same value
        assert_eq!(store.get("Hello"), Some("world".to_string()));
        assert_eq!(store.get("hello"), Some("world".to_string()));
        assert_eq!(store.get("HELLO"), Some("world".to_string()));
        assert_eq!(store.get("hElLo"), Some("world".to_string()));
    }

    #[test]
    fn overwrite_preserves_most_recent_key_casing() {
        let mut store = CaseInsensitiveStore::new();

        store.set("hello".to_string(), "value1".to_string());
        store.set("HELLO".to_string(), "value2".to_string());
        store.set("Hello".to_string(), "value3".to_string());

        // Should only have one entry
        assert_eq!(store.len(), 1);
        assert_eq!(store.get("hello"), Some("value3".to_string()));

        // keys() should return the last casing used
        let keys = store.keys();
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0], "Hello");
    }

    #[test]
    fn keys_returns_current_original_casing() {
        let mut store = CaseInsensitiveStore::new();

        store.set("First".to_string(), "1".to_string());
        store.set("SECOND".to_string(), "2".to_string());
        store.set("tHiRd".to_string(), "3".to_string());

        let mut keys = store.keys();
        keys.sort(); // Sort for consistent testing

        assert_eq!(keys, vec!["First", "SECOND", "tHiRd"]);
    }

    #[test]
    fn delete_works_case_insensitively() {
        let mut store = CaseInsensitiveStore::new();

        store.set("Hello".to_string(), "world".to_string());

        // Delete with different casing
        assert_eq!(store.delete("HELLO"), Some("world".to_string()));
        assert_eq!(store.get("Hello"), None);
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn empty_store_and_missing_keys_return_defaults() {
        let mut store = CaseInsensitiveStore::new();

        assert_eq!(store.get("anything"), None);
        assert_eq!(store.delete("anything"), None);
        assert_eq!(store.keys(), Vec::<String>::new());
        assert_eq!(store.len(), 0);
    }
}
