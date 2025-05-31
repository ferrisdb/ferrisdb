//! Challenge 1: Implement a delete() method
//!
//! Your task: Add a method to remove entries from the key-value store.
//!
//! Requirements:
//! - Method signature: pub fn delete(&mut self, key: &str) -> Option<String>
//! - Return the old value if the key existed
//! - Return None if the key didn't exist

// Allow warnings for educational exercise templates
#![allow(unused_variables)]
#![allow(dead_code)]

use std::collections::HashMap;

pub struct KeyValueStore {
    data: HashMap<String, String>,
}

impl KeyValueStore {
    pub fn new() -> Self {
        KeyValueStore {
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.data.get(key).cloned()
    }

    // TODO: Implement this method!
    pub fn delete(&mut self, key: &str) -> Option<String> {
        todo!("Implement the delete method")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delete_existing_key_returns_old_value() {
        let mut store = KeyValueStore::new();
        store.set("key1".to_string(), "value1".to_string());

        // Delete should return the old value
        assert_eq!(store.delete("key1"), Some("value1".to_string()));

        // Key should no longer exist
        assert_eq!(store.get("key1"), None);
    }

    #[test]
    fn delete_missing_key_returns_none() {
        let mut store = KeyValueStore::new();

        // Deleting non-existent key should return None
        assert_eq!(store.delete("missing"), None);
    }

    #[test]
    fn delete_same_key_twice_returns_none_second_time() {
        let mut store = KeyValueStore::new();
        store.set("key1".to_string(), "value1".to_string());

        // First delete returns the value
        assert_eq!(store.delete("key1"), Some("value1".to_string()));

        // Second delete returns None
        assert_eq!(store.delete("key1"), None);
    }
}
