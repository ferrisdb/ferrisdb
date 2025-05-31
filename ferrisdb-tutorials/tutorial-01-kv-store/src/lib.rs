//! # Tutorial 01: Key-Value Store
//!
//! This is the final implementation from Tutorial 01.
//! It demonstrates a simple in-memory key-value store using HashMap.
//!
//! ## Key Concepts Demonstrated
//!
//! - Struct definition and methods
//! - Ownership with `&self` and `&mut self`
//! - Using standard collections (HashMap)
//! - Option<T> for nullable returns
//! - Basic testing patterns

use std::collections::HashMap;

/// A simple key-value store backed by a HashMap
#[derive(Default)]
pub struct KeyValueStore {
    /// Internal storage using HashMap
    data: HashMap<String, String>,
}

impl KeyValueStore {
    /// Creates a new, empty key-value store
    ///
    /// # Examples
    ///
    /// ```
    /// use tutorial_01_kv_store::KeyValueStore;
    ///
    /// let store = KeyValueStore::new();
    /// ```
    pub fn new() -> Self {
        KeyValueStore {
            data: HashMap::new(),
        }
    }

    /// Stores a key-value pair in the store
    ///
    /// If the key already exists, the value is updated.
    ///
    /// # Examples
    ///
    /// ```
    /// use tutorial_01_kv_store::KeyValueStore;
    ///
    /// let mut store = KeyValueStore::new();
    /// store.set("user:1".to_string(), "Alice".to_string());
    /// ```
    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    /// Retrieves a value by key
    ///
    /// Returns `Some(value)` if the key exists, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use tutorial_01_kv_store::KeyValueStore;
    ///
    /// let mut store = KeyValueStore::new();
    /// store.set("user:1".to_string(), "Alice".to_string());
    ///
    /// assert_eq!(store.get("user:1"), Some("Alice".to_string()));
    /// assert_eq!(store.get("user:2"), None);
    /// ```
    pub fn get(&self, key: &str) -> Option<String> {
        self.data.get(key).cloned()
    }

    /// Returns the number of key-value pairs in the store
    ///
    /// # Examples
    ///
    /// ```
    /// use tutorial_01_kv_store::KeyValueStore;
    ///
    /// let mut store = KeyValueStore::new();
    /// assert_eq!(store.len(), 0);
    ///
    /// store.set("key".to_string(), "value".to_string());
    /// assert_eq!(store.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns true if the store contains no key-value pairs
    ///
    /// # Examples
    ///
    /// ```
    /// use tutorial_01_kv_store::KeyValueStore;
    ///
    /// let store = KeyValueStore::new();
    /// assert!(store.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_store() {
        let store = KeyValueStore::new();
        assert!(store.is_empty());
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn test_set_and_get() {
        let mut store = KeyValueStore::new();

        // Test basic set and get
        store.set("user:1".to_string(), "Alice".to_string());
        assert_eq!(store.get("user:1"), Some("Alice".to_string()));

        // Test missing key
        assert_eq!(store.get("user:2"), None);

        // Test overwrite
        store.set("user:1".to_string(), "Alice Smith".to_string());
        assert_eq!(store.get("user:1"), Some("Alice Smith".to_string()));
    }

    #[test]
    fn test_multiple_entries() {
        let mut store = KeyValueStore::new();

        store.set("user:1".to_string(), "Alice".to_string());
        store.set("user:2".to_string(), "Bob".to_string());
        store.set("user:3".to_string(), "Charlie".to_string());

        assert_eq!(store.len(), 3);
        assert_eq!(store.get("user:1"), Some("Alice".to_string()));
        assert_eq!(store.get("user:2"), Some("Bob".to_string()));
        assert_eq!(store.get("user:3"), Some("Charlie".to_string()));
    }
}
