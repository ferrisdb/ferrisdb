//! Challenge 3: Add concurrent read support
//! 
//! Goal: Allow multiple readers while writing continues.
//! 
//! Requirements:
//! 1. Wrap WAL in Arc<RwLock<T>> for shared access
//! 2. Readers take read locks, writers take write locks
//! 3. Implement a snapshot reader that doesn't block writers
//! 4. Ensure consistency when reading while writing
//! 
//! Hints:
//! - Use std::sync::RwLock or parking_lot::RwLock
//! - Consider: what if a reader is in the middle of a file when writer appends?
//! - Snapshot could copy current file position at start
//! - Think about sequence number visibility

// TODO: Your implementation here!

use std::sync::{Arc, RwLock};

pub struct ConcurrentWal {
    // Your fields here
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    
    #[test]
    #[ignore] // Remove this when you implement the solution
    fn test_concurrent_access() {
        // Test multiple readers don't block each other
        // Test writer doesn't block on readers
        // Test consistency of reads during writes
    }
}