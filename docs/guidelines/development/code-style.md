# Code Style Guidelines

This document outlines the code style standards for FerrisDB. Consistent style makes the codebase easier to read, understand, and maintain.

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

---

_Next: [Idiomatic Rust Guidelines](idiomatic-rust.md)_
