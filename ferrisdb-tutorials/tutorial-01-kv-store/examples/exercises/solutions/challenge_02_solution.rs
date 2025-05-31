//! Solution for Challenge 2: TTL (Time To Live) support

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Stores a value with an optional expiration time
struct ValueWithTtl {
    value: String,
    expires_at: Option<Instant>,
}

/// A key-value store with TTL support
pub struct TtlKeyValueStore {
    data: HashMap<String, ValueWithTtl>,
}

impl TtlKeyValueStore {
    pub fn new() -> Self {
        TtlKeyValueStore {
            data: HashMap::new(),
        }
    }

    /// Set a key-value pair that never expires
    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(
            key,
            ValueWithTtl {
                value,
                expires_at: None,
            },
        );
    }

    /// Set a key-value pair with a time-to-live
    pub fn set_with_ttl(&mut self, key: String, value: String, ttl: Duration) {
        let expires_at = Instant::now() + ttl;
        self.data.insert(
            key,
            ValueWithTtl {
                value,
                expires_at: Some(expires_at),
            },
        );
    }

    /// Get a value if it exists and hasn't expired
    pub fn get(&self, key: &str) -> Option<String> {
        self.data.get(key).and_then(|entry| match entry.expires_at {
            Some(expires_at) if Instant::now() > expires_at => None,
            _ => Some(entry.value.clone()),
        })
    }

    /// Remove all expired entries
    pub fn cleanup_expired(&mut self) {
        let now = Instant::now();
        self.data.retain(|_, entry| match entry.expires_at {
            Some(expires_at) => now <= expires_at,
            None => true,
        });
    }

    /// Get the number of non-expired entries
    pub fn len(&self) -> usize {
        let now = Instant::now();
        self.data
            .values()
            .filter(|entry| match entry.expires_at {
                Some(expires_at) => now <= expires_at,
                None => true,
            })
            .count()
    }
}

// The tests from the challenge file will validate this solution
#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn set_and_get_without_ttl_works_normally() {
        let mut store = TtlKeyValueStore::new();
        store.set("key1".to_string(), "value1".to_string());

        assert_eq!(store.get("key1"), Some("value1".to_string()));
    }

    #[test]
    fn set_with_ttl_returns_value_before_expiration() {
        let mut store = TtlKeyValueStore::new();
        store.set_with_ttl(
            "key1".to_string(),
            "value1".to_string(),
            Duration::from_secs(10),
        );

        assert_eq!(store.get("key1"), Some("value1".to_string()));
    }

    #[test]
    fn set_with_ttl_returns_none_after_expiration() {
        let mut store = TtlKeyValueStore::new();
        store.set_with_ttl(
            "key1".to_string(),
            "value1".to_string(),
            Duration::from_millis(50),
        );

        thread::sleep(Duration::from_millis(100));

        assert_eq!(store.get("key1"), None);
    }

    #[test]
    fn cleanup_expired_removes_only_expired_entries() {
        let mut store = TtlKeyValueStore::new();

        store.set_with_ttl(
            "expire_soon".to_string(),
            "value1".to_string(),
            Duration::from_millis(50),
        );
        store.set_with_ttl(
            "expire_later".to_string(),
            "value2".to_string(),
            Duration::from_secs(10),
        );
        store.set("never_expire".to_string(), "value3".to_string());

        assert_eq!(store.len(), 3);

        thread::sleep(Duration::from_millis(100));

        store.cleanup_expired();

        assert_eq!(store.len(), 2);
        assert_eq!(store.get("expire_soon"), None);
        assert_eq!(store.get("expire_later"), Some("value2".to_string()));
        assert_eq!(store.get("never_expire"), Some("value3".to_string()));
    }

    #[test]
    fn overwrite_replaces_ttl_with_no_expiration() {
        let mut store = TtlKeyValueStore::new();

        store.set_with_ttl(
            "key1".to_string(),
            "value1".to_string(),
            Duration::from_millis(50),
        );
        store.set("key1".to_string(), "value2".to_string());

        thread::sleep(Duration::from_millis(100));

        assert_eq!(store.get("key1"), Some("value2".to_string()));
    }
}
