# Idiomatic Rust Guidelines

This document outlines Rust-specific best practices and patterns for FerrisDB. Following these guidelines ensures our code is maintainable, performant, and leverages Rust's strengths.

## Module Organization

### File Naming

- Use snake_case file names that match struct names
- Keep public API types in separate files, not in `mod.rs`
- Only re-export types that should be part of the public API

```rust
// Good structure
memtable/
├── mod.rs          // Only re-exports and module docs
├── skip_list.rs    // Contains SkipList struct
└── node.rs         // Contains Node struct (internal)

// In mod.rs
pub use skip_list::SkipList;  // Public API
// Note: Node is NOT re-exported (internal detail)
```

### Module Inception

Avoid module inception (module name ≠ file name in same directory):

```rust
// Bad: src/storage/storage.rs in storage/ directory
// Good: src/storage.rs or src/storage/engine.rs
```

## Error Handling

### Result Types

- Always use `Result<T>` for fallible operations
- Never panic in library code
- Create custom error types with `thiserror`

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("corrupted block at offset {offset}: {reason}")]
    CorruptedBlock { offset: u64, reason: String },

    #[error("I/O error")]
    Io(#[from] std::io::Error),

    #[error("key too large: {size} bytes (max: {max})")]
    KeyTooLarge { size: usize, max: usize },
}
```

### Error Context

Provide meaningful context when propagating errors:

```rust
// Good: Add context
let data = fs::read(&path)
    .map_err(|e| StorageError::ReadFailed {
        path: path.clone(),
        source: e
    })?;

// Bad: Just propagate
let data = fs::read(&path)?;
```

## Ownership and Borrowing

### API Design

- Prefer owned types in public APIs for flexibility
- Use references for internal functions
- Consider `Cow<'_, T>` for APIs that might clone

```rust
// Good: Public API takes ownership
pub fn insert(mut self, key: Vec<u8>, value: Vec<u8>) -> Self {
    // Implementation
}

// Good: Internal function borrows
fn find_position(&self, key: &[u8]) -> Option<usize> {
    // Implementation
}
```

### Smart Pointers

Use appropriate smart pointers for your use case:

```rust
// Arc for shared ownership across threads
type SharedMemTable = Arc<RwLock<MemTable>>;

// Box for heap allocation without sharing
struct LargeNode {
    data: Box<[u8; 1024 * 1024]>,  // 1MB on heap
}

// Rc for single-threaded shared ownership (rare in FerrisDB)
```

## Concurrency

### Thread Safety

- Use `Send + Sync` bounds where appropriate
- Document thread safety guarantees
- Prefer message passing over shared state

```rust
// Ensure types used across threads are thread-safe
pub struct StorageEngine {
    memtable: Arc<RwLock<MemTable>>,  // Multiple readers, one writer
    wal: Arc<Mutex<WriteAheadLog>>,   // Exclusive access
}

// Document safety
/// This type is thread-safe and can be shared across threads.
/// Multiple threads can read concurrently, but writes are serialized.
impl StorageEngine { ... }
```

### Atomic Operations

Use atomics for simple counters and flags:

```rust
use std::sync::atomic::{AtomicU64, Ordering};

struct Metrics {
    writes: AtomicU64,
    reads: AtomicU64,
}

impl Metrics {
    fn record_write(&self) {
        self.writes.fetch_add(1, Ordering::Relaxed);
    }
}
```

## Type System

### Type Aliases

Use meaningful type aliases for complex types:

```rust
// Good: Clear what this represents
type BlockCache = HashMap<BlockId, Arc<Block>>;
type KeyValuePair = (Vec<u8>, Vec<u8>);

// Bad: Unclear generic names
type Cache = HashMap<u64, Arc<Vec<u8>>>;
```

### Trait Bounds

- Place bounds on implementations, not type definitions
- Use where clauses for complex bounds

```rust
// Good: Bounds on impl
struct Container<T> {
    items: Vec<T>,
}

impl<T> Container<T>
where
    T: Clone + Send + Sync,
{
    fn clone_items(&self) -> Vec<T> {
        self.items.clone()
    }
}
```

## Pattern Matching

### Exhaustive Matching

Prefer exhaustive matches over catch-all patterns:

```rust
// Good: Handle all cases explicitly
match operation {
    Operation::Get { key } => handle_get(key),
    Operation::Set { key, value } => handle_set(key, value),
    Operation::Delete { key } => handle_delete(key),
}

// Bad: Catch-all hides new variants
match operation {
    Operation::Get { key } => handle_get(key),
    _ => return Err(Error::UnsupportedOperation),
}
```

### If Let vs Match

Use `if let` for single pattern matching:

```rust
// Good: Single pattern
if let Some(value) = cache.get(&key) {
    return Ok(value.clone());
}

// Overkill: Full match for single pattern
match cache.get(&key) {
    Some(value) => return Ok(value.clone()),
    None => {},
}
```

## Iterator Patterns

### Functional Style

Prefer iterator methods over manual loops:

```rust
// Good: Functional style
let sum: u64 = values
    .iter()
    .filter(|v| v.is_valid())
    .map(|v| v.size())
    .sum();

// Less idiomatic: Manual loop
let mut sum = 0u64;
for value in values {
    if value.is_valid() {
        sum += value.size();
    }
}
```

### Enumerate Instead of Index

```rust
// Good: Use enumerate
for (i, item) in items.iter().enumerate() {
    println!("Item {}: {:?}", i, item);
}

// Bad: Manual indexing
for i in 0..items.len() {
    println!("Item {}: {:?}", i, items[i]);
}
```

## Memory Safety

### Unsafe Code

- Use `unsafe` sparingly
- Document safety invariants
- Encapsulate unsafe code in safe abstractions

```rust
/// Converts a slice to a fixed-size array reference.
///
/// # Safety
///
/// The caller must ensure that `slice.len() >= N`.
unsafe fn slice_to_array<const N: usize>(slice: &[u8]) -> &[u8; N] {
    debug_assert!(slice.len() >= N);
    &*(slice.as_ptr() as *const [u8; N])
}

// Safe wrapper
fn read_header(data: &[u8]) -> Result<&[u8; 16]> {
    if data.len() < 16 {
        return Err(Error::InvalidHeader);
    }
    // SAFETY: We just checked the length
    Ok(unsafe { slice_to_array(data) })
}
```

## Performance

### Zero-Copy Operations

Prefer borrowing over cloning when possible:

```rust
// Good: Borrow for read operations
fn find_key<'a>(&'a self, key: &[u8]) -> Option<&'a [u8]> {
    self.data.get(key).map(|v| v.as_slice())
}

// Bad: Unnecessary clone
fn find_key(&self, key: &[u8]) -> Option<Vec<u8>> {
    self.data.get(key).cloned()
}
```

### Allocation Strategies

Minimize allocations in hot paths:

```rust
// Good: Reuse buffer
let mut buffer = Vec::with_capacity(1024);
for item in items {
    buffer.clear();
    item.serialize_into(&mut buffer)?;
    writer.write_all(&buffer)?;
}

// Bad: Allocate per iteration
for item in items {
    let buffer = item.serialize()?;  // New allocation
    writer.write_all(&buffer)?;
}
```

---

_Next: [Documentation Standards](documentation.md)_
