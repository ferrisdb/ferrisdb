//! Challenge 2: Add TTL (Time To Live) support
//!
//! Implement basic expiration for entries. Keys should automatically expire
//! after a specified duration.
//!
//! Requirements:
//! - Store expiration time with each entry
//! - Implement `set_with_ttl(key, value, duration)`
//! - Expired entries should not be returned by `get()`
//! - Add `cleanup_expired()` method
//!
//! Hints:
//! - You'll need to change the internal data structure
//! - Consider using std::time::Instant for tracking time
//! - Think about when to check for expiration

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// A key-value store with TTL support
pub struct TtlKeyValueStore {
    // TODO: Change this to store both value and expiration time
    data: HashMap<String, String>,
}

impl TtlKeyValueStore {
    pub fn new() -> Self {
        TtlKeyValueStore {
            data: HashMap::new(),
        }
    }

    /// Set a key-value pair that never expires
    pub fn set(&mut self, key: String, value: String) {
        // TODO: Implement this
        // Hint: You might want to store a special value for "never expires"
    }

    /// Set a key-value pair with a time-to-live
    pub fn set_with_ttl(&mut self, key: String, value: String, ttl: Duration) {
        // TODO: Implement this
        // Hint: Calculate when this entry should expire
    }

    /// Get a value if it exists and hasn't expired
    pub fn get(&self, key: &str) -> Option<String> {
        // TODO: Implement this
        // Hint: Check if the entry has expired before returning it
        None
    }

    /// Remove all expired entries
    pub fn cleanup_expired(&mut self) {
        // TODO: Implement this
        // Hint: Iterate through all entries and remove expired ones
    }

    /// Get the number of non-expired entries
    pub fn len(&self) -> usize {
        // TODO: Implement this
        // Hint: Don't count expired entries
        0
    }
}

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

        // Should still be available immediately
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

        // Wait for expiration
        thread::sleep(Duration::from_millis(100));

        // Should no longer be available
        assert_eq!(store.get("key1"), None);
    }

    #[test]
    fn cleanup_expired_removes_only_expired_entries() {
        let mut store = TtlKeyValueStore::new();

        // Add some entries with different TTLs
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

        // Wait for first entry to expire
        thread::sleep(Duration::from_millis(100));

        // Before cleanup, len might still return 3 (implementation dependent)
        store.cleanup_expired();

        // After cleanup, should only have 2 entries
        assert_eq!(store.len(), 2);
        assert_eq!(store.get("expire_soon"), None);
        assert_eq!(store.get("expire_later"), Some("value2".to_string()));
        assert_eq!(store.get("never_expire"), Some("value3".to_string()));
    }

    #[test]
    fn overwrite_replaces_ttl_with_no_expiration() {
        let mut store = TtlKeyValueStore::new();

        // Set with short TTL
        store.set_with_ttl(
            "key1".to_string(),
            "value1".to_string(),
            Duration::from_millis(50),
        );

        // Overwrite with no TTL
        store.set("key1".to_string(), "value2".to_string());

        // Wait longer than original TTL
        thread::sleep(Duration::from_millis(100));

        // Should still be available
        assert_eq!(store.get("key1"), Some("value2".to_string()));
    }
}
