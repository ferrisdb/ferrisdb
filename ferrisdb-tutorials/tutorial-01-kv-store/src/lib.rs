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
    fn new_creates_empty_store() {
        let store = KeyValueStore::new();
        assert!(store.is_empty());
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn set_stores_value_and_get_retrieves_it() {
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
    fn multiple_entries_are_stored_independently() {
        let mut store = KeyValueStore::new();

        store.set("user:1".to_string(), "Alice".to_string());
        store.set("user:2".to_string(), "Bob".to_string());
        store.set("user:3".to_string(), "Charlie".to_string());

        assert_eq!(store.len(), 3);
        assert_eq!(store.get("user:1"), Some("Alice".to_string()));
        assert_eq!(store.get("user:2"), Some("Bob".to_string()));
        assert_eq!(store.get("user:3"), Some("Charlie".to_string()));
    }

    #[test]
    fn default_creates_empty_store() {
        let store = KeyValueStore::default();
        assert!(store.is_empty());
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn is_empty_returns_false_after_adding_entries() {
        let mut store = KeyValueStore::new();
        assert!(store.is_empty());

        store.set("key".to_string(), "value".to_string());
        assert!(!store.is_empty());
    }

    #[test]
    fn len_increases_with_new_entries() {
        let mut store = KeyValueStore::new();
        assert_eq!(store.len(), 0);

        store.set("key1".to_string(), "value1".to_string());
        assert_eq!(store.len(), 1);

        store.set("key2".to_string(), "value2".to_string());
        assert_eq!(store.len(), 2);
    }

    #[test]
    fn len_unchanged_when_overwriting_existing_key() {
        let mut store = KeyValueStore::new();

        store.set("key".to_string(), "value1".to_string());
        assert_eq!(store.len(), 1);

        store.set("key".to_string(), "value2".to_string());
        assert_eq!(store.len(), 1);
        assert_eq!(store.get("key"), Some("value2".to_string()));
    }

    #[test]
    fn handles_empty_string_keys_and_values() {
        let mut store = KeyValueStore::new();

        store.set("".to_string(), "".to_string());
        assert_eq!(store.get(""), Some("".to_string()));
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn handles_unicode_keys_and_values() {
        let mut store = KeyValueStore::new();

        store.set("🦀".to_string(), "🚀".to_string());
        store.set("测试".to_string(), "тест".to_string());

        assert_eq!(store.get("🦀"), Some("🚀".to_string()));
        assert_eq!(store.get("测试"), Some("тест".to_string()));
        assert_eq!(store.len(), 2);
    }

    #[test]
    fn get_returns_none_for_missing_keys() {
        let store = KeyValueStore::new();
        assert_eq!(store.get("nonexistent"), None);

        let mut store = KeyValueStore::new();
        store.set("existing".to_string(), "value".to_string());
        assert_eq!(store.get("different"), None);
    }
}
