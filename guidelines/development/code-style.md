# Code Style Guidelines

This document outlines the code style standards for FerrisDB. Consistent style makes the codebase easier to read, understand, and maintain.

**Purpose**: Define formatting and style conventions for all Rust code in FerrisDB.  
**Prerequisites**: Basic Rust knowledge and familiarity with cargo tools

## Rust Formatting

### Basic Rules

- **Always** use `rustfmt` for formatting
- Run `cargo fmt --all` before committing
- Maximum line length: 100 characters
- Use descriptive variable names (no single letters except in iterators)
- Prefer `snake_case` for functions and variables
- Use `CamelCase` for types, traits, and enums

### Naming Conventions

```rust
// Good examples
struct MemTable { ... }
fn calculate_checksum(data: &[u8]) -> u32 { ... }
let block_size = 4096;
const MAX_KEY_SIZE: usize = 1024;

// Bad examples
struct mem_table { ... }  // Should be CamelCase
fn CalcCRC(d: &[u8]) -> u32 { ... }  // Should be snake_case, descriptive
let bs = 4096;  // Too abbreviated
```

### Import Organization

Organize imports in logical groups with blank lines between:

```rust
// 1. Local crate imports
use crate::storage::MemTable;
use crate::wal::{LogEntry, Writer};
use ferrisdb_core::{Error, Result};

// 2. External crate imports
use bytes::{Buf, BufMut};
use tokio::fs::File;

// 3. Standard library imports
use std::collections::HashMap;
use std::sync::Arc;

// 4. Test-only imports (at the bottom)
#[cfg(test)]
use proptest::prelude::*;
```

### Comments and Documentation

- Use `///` for public API documentation
- Use `//` for implementation comments
- Write comments that explain "why", not "what"
- Keep comments up-to-date with code changes

```rust
/// Flushes the current memtable to disk as an SSTable.
///
/// This operation is atomic - either the entire flush succeeds
/// or the memtable remains unchanged. The flush process involves:
/// 1. Creating a new SSTable writer
/// 2. Iterating through all entries in sorted order
/// 3. Writing the SSTable with proper checksums
/// 4. Updating the manifest file
///
/// # Errors
///
/// Returns an error if:
/// - Disk I/O fails
/// - The manifest update fails
pub async fn flush_memtable(&self) -> Result<()> {
    // We take a read lock here to allow concurrent reads
    // during the flush process
    let memtable = self.active.read().await;
    ...
}
```

## Linting

### Clippy Configuration

- Run `cargo clippy --all-targets --all-features -- -D warnings`
- Fix all clippy warnings before committing
- Use `#[allow(...)]` sparingly and document why

### Common Clippy Fixes

```rust
// Instead of: if x == true
if x { ... }

// Instead of: if let Some(val) = opt { val } else { default }
opt.unwrap_or(default)

// Instead of: vec.iter().filter(|x| ...).collect::<Vec<_>>()
vec.into_iter().filter(|x| ...).collect()
```

## Code Organization

### File Structure

- Keep files focused on a single responsibility
- Prefer smaller files (< 500 lines) over large ones
- Group related functionality into modules

### Module Guidelines

```rust
// Good: Each type in its own file
ferrisdb-storage/
├── src/
│   ├── lib.rs
│   ├── memtable/
│   │   ├── mod.rs      // Re-exports and module structure
│   │   └── skip_list.rs // SkipList implementation
│   └── sstable/
│       ├── mod.rs
│       ├── reader.rs   // SSTableReader
│       └── writer.rs   // SSTableWriter
```

## Error Handling

- Always use `Result<T>` for fallible operations
- Never use `.unwrap()` in library code (tests are OK)
- Provide context with error messages
- Use `?` for error propagation

```rust
// Good
let file = File::open(&path)
    .map_err(|e| Error::FileOpen { path: path.clone(), source: e })?;

// Bad
let file = File::open(&path).unwrap();  // Will panic!
```

## Constants and Limits

### Definition Guidelines

Constants should be defined explicitly to improve code clarity and prevent security issues:

- Define operation constants explicitly
- Define size limits to prevent DoS attacks
- Group related constants together
- Document units in constant names or comments

### Examples

```rust
// Good: Grouped operation constants
pub const OP_PUT: u8 = 1;
pub const OP_GET: u8 = 2;
pub const OP_DELETE: u8 = 3;
pub const OP_SCAN: u8 = 4;

// Good: Size limits with clear units
pub const MAX_KEY_SIZE: usize = 1024;        // bytes
pub const MAX_VALUE_SIZE: usize = 64 * 1024; // 64 KiB
pub const MAX_BATCH_SIZE: usize = 1000;      // number of operations
pub const MAX_WAL_SIZE: u64 = 100 * 1024 * 1024; // 100 MiB

// Good: Related constants grouped
pub mod block {
    pub const BLOCK_SIZE: usize = 4096;      // 4 KiB
    pub const BLOCK_HEADER_SIZE: usize = 16; // bytes
    pub const BLOCK_FOOTER_SIZE: usize = 8;  // bytes
    pub const MAX_BLOCK_SIZE: usize = 64 * 1024; // 64 KiB
}

// Good: Time constants with units
pub const COMPACTION_INTERVAL_SECS: u64 = 300;     // 5 minutes
pub const FLUSH_TIMEOUT_MILLIS: u64 = 5000;        // 5 seconds
pub const DEFAULT_TTL_SECS: u64 = 24 * 60 * 60;    // 24 hours

// Bad: Magic numbers without context
const LIMIT: usize = 1000;  // What kind of limit?
const SIZE: usize = 4096;   // Size of what? In what units?
const OP: u8 = 1;          // What operation?
```

### Usage in Code

```rust
use crate::constants::{MAX_KEY_SIZE, MAX_VALUE_SIZE, OP_PUT};

pub fn validate_key_value(key: &[u8], value: &[u8]) -> Result<()> {
    if key.len() > MAX_KEY_SIZE {
        return Err(Error::KeyTooLarge {
            size: key.len(),
            max: MAX_KEY_SIZE,
        });
    }

    if value.len() > MAX_VALUE_SIZE {
        return Err(Error::ValueTooLarge {
            size: value.len(),
            max: MAX_VALUE_SIZE,
        });
    }

    Ok(())
}

pub fn encode_operation(op: Operation) -> u8 {
    match op {
        Operation::Put(_) => OP_PUT,
        Operation::Get(_) => OP_GET,
        Operation::Delete(_) => OP_DELETE,
        Operation::Scan(_) => OP_SCAN,
    }
}
```

## Extension Traits

When extending external types with new functionality:

```rust
// Good: Extension trait with clear naming
pub trait BytesMutExt {
    fn read_exact_from<R: Read>(&mut self, reader: &mut R, count: usize) -> io::Result<()>;
}

impl BytesMutExt for BytesMut {
    fn read_exact_from<R: Read>(&mut self, reader: &mut R, count: usize) -> io::Result<()> {
        // Implementation
    }
}

// Usage
use crate::utils::BytesMutExt;
let mut buf = BytesMut::new();
buf.read_exact_from(&mut reader, 1024)?;
```

### Extension Trait Guidelines

- Name extension traits with `Ext` suffix
- Document why the extension is needed
- Keep extensions focused on a single responsibility
- Place in a `utils` module if used across multiple modules
- Test thoroughly as they affect external types

## Performance Considerations

- Document performance implications in comments
- Prefer zero-copy operations where possible
- Use `&str` over `String` for function parameters when not taking ownership
- Consider using `Cow<'_, str>` for APIs that might need to clone

## Testing

- Write unit tests for all public APIs
- Use descriptive test names that explain what is being tested
- Group related tests in modules

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checksum_detects_single_bit_corruption() {
        // Test implementation
    }

    #[test]
    fn checksum_detects_multi_bit_corruption() {
        // Test implementation
    }
}
```

## Commit Standards

- Run `cargo fmt --all` before committing
- Run `cargo clippy --all-targets --all-features -- -D warnings`
- Ensure all tests pass with `cargo test --all`
- Use conventional commit messages (e.g., `feat:`, `fix:`, `docs:`)

## Related Guidelines

- **Next**: [Idiomatic Rust](idiomatic-rust.md) - Rust best practices and patterns
- **Then**: [Documentation](documentation.md) - How to document your code
- **Also See**: [Testing](../workflow/testing.md) - Writing tests for your code
- **Before Committing**: [Git Workflow](../workflow/git-workflow.md) - Collaboration standards

---

_Last updated: 2025-06-01_
