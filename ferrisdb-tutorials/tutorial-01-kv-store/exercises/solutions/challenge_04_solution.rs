//! Solution for Challenge 4: Prefix scanning

use std::collections::HashMap;

/// A key-value store with prefix scanning
pub struct ScanableStore {
    data: HashMap<String, String>,
}

impl ScanableStore {
    pub fn new() -> Self {
        ScanableStore {
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.data.get(key).cloned()
    }

    /// Scan for all keys with the given prefix
    pub fn scan_prefix(&self, prefix: &str) -> Vec<String> {
        let mut keys: Vec<String> = self
            .data
            .keys()
            .filter(|key| key.starts_with(prefix))
            .cloned()
            .collect();

        keys.sort();
        keys
    }

    /// Get all keys (useful for debugging)
    pub fn keys(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

// Tests are included in the challenge file
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_prefix_returns_matching_keys_sorted() {
        let mut store = ScanableStore::new();

        store.set("user:1".to_string(), "Alice".to_string());
        store.set("user:2".to_string(), "Bob".to_string());
        store.set("user:10".to_string(), "Charlie".to_string());
        store.set("product:1".to_string(), "Laptop".to_string());
        store.set("product:2".to_string(), "Mouse".to_string());

        let user_keys = store.scan_prefix("user:");
        assert_eq!(user_keys, vec!["user:1", "user:10", "user:2"]);

        let product_keys = store.scan_prefix("product:");
        assert_eq!(product_keys, vec!["product:1", "product:2"]);
    }

    #[test]
    fn scan_prefix_with_empty_string_returns_all_keys_sorted() {
        let mut store = ScanableStore::new();

        store.set("a".to_string(), "1".to_string());
        store.set("b".to_string(), "2".to_string());
        store.set("c".to_string(), "3".to_string());

        let all_keys = store.scan_prefix("");
        assert_eq!(all_keys, vec!["a", "b", "c"]);
    }

    #[test]
    fn scan_prefix_returns_empty_vec_when_no_matches() {
        let mut store = ScanableStore::new();

        store.set("user:1".to_string(), "Alice".to_string());
        store.set("user:2".to_string(), "Bob".to_string());

        let admin_keys = store.scan_prefix("admin:");
        assert_eq!(admin_keys, Vec::<String>::new());
    }

    #[test]
    fn scan_prefix_returns_lexicographically_sorted_results() {
        let mut store = ScanableStore::new();

        store.set("key:10".to_string(), "10".to_string());
        store.set("key:2".to_string(), "2".to_string());
        store.set("key:1".to_string(), "1".to_string());
        store.set("key:20".to_string(), "20".to_string());

        let keys = store.scan_prefix("key:");
        assert_eq!(keys, vec!["key:1", "key:10", "key:2", "key:20"]);
    }

    #[test]
    fn scan_prefix_includes_exact_matches_and_longer_keys() {
        let mut store = ScanableStore::new();

        store.set("test".to_string(), "value".to_string());
        store.set("test123".to_string(), "value".to_string());
        store.set("testing".to_string(), "value".to_string());

        let keys = store.scan_prefix("test");
        assert_eq!(keys, vec!["test", "test123", "testing"]);

        let keys = store.scan_prefix("test1");
        assert_eq!(keys, vec!["test123"]);
    }

    #[test]
    fn keys_returns_all_keys_in_store() {
        let mut store = ScanableStore::new();

        store.set("zebra".to_string(), "1".to_string());
        store.set("apple".to_string(), "2".to_string());
        store.set("banana".to_string(), "3".to_string());

        let mut keys = store.keys();
        keys.sort();

        assert_eq!(keys, vec!["apple", "banana", "zebra"]);
    }

    #[test]
    fn len_returns_count_of_unique_keys() {
        let mut store = ScanableStore::new();

        assert_eq!(store.len(), 0);

        store.set("key1".to_string(), "value1".to_string());
        assert_eq!(store.len(), 1);

        store.set("key2".to_string(), "value2".to_string());
        assert_eq!(store.len(), 2);

        store.set("key1".to_string(), "new_value".to_string());
        assert_eq!(store.len(), 2);
    }

    #[test]
    fn get_returns_value_for_existing_key_and_none_for_missing() {
        let mut store = ScanableStore::new();

        store.set("key1".to_string(), "value1".to_string());
        store.set("key2".to_string(), "value2".to_string());

        assert_eq!(store.get("key1"), Some("value1".to_string()));
        assert_eq!(store.get("key2"), Some("value2".to_string()));
        assert_eq!(store.get("key3"), None);
    }
}
