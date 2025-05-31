//! Solution for Challenge 3: Case-insensitive keys

use std::collections::HashMap;

/// Stores both the value and the original key casing
struct ValueWithOriginalKey {
    value: String,
    original_key: String,
}

/// A case-insensitive key-value store
pub struct CaseInsensitiveStore {
    // Use lowercase keys for lookups, but store original casing
    data: HashMap<String, ValueWithOriginalKey>,
}

impl CaseInsensitiveStore {
    pub fn new() -> Self {
        CaseInsensitiveStore {
            data: HashMap::new(),
        }
    }

    /// Set a key-value pair (case-insensitive key)
    pub fn set(&mut self, key: String, value: String) {
        let normalized_key = key.to_lowercase();
        self.data.insert(
            normalized_key,
            ValueWithOriginalKey {
                value,
                original_key: key,
            },
        );
    }

    /// Get a value by key (case-insensitive)
    pub fn get(&self, key: &str) -> Option<String> {
        let normalized_key = key.to_lowercase();
        self.data
            .get(&normalized_key)
            .map(|entry| entry.value.clone())
    }

    /// Get all keys in their original casing
    pub fn keys(&self) -> Vec<String> {
        self.data
            .values()
            .map(|entry| entry.original_key.clone())
            .collect()
    }

    /// Remove a key-value pair (case-insensitive)
    pub fn delete(&mut self, key: &str) -> Option<String> {
        let normalized_key = key.to_lowercase();
        self.data.remove(&normalized_key).map(|entry| entry.value)
    }

    /// Get the number of entries
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

// Tests are included in the challenge file
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_and_get_work_case_insensitively() {
        let mut store = CaseInsensitiveStore::new();

        store.set("Hello".to_string(), "world".to_string());

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

        assert_eq!(store.len(), 1);
        assert_eq!(store.get("hello"), Some("value3".to_string()));

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
        keys.sort();

        assert_eq!(keys, vec!["First", "SECOND", "tHiRd"]);
    }

    #[test]
    fn delete_works_case_insensitively() {
        let mut store = CaseInsensitiveStore::new();

        store.set("Hello".to_string(), "world".to_string());

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
