# Documentation Guidelines

Guidelines for documenting FerrisDB code and technical specifications.

**Purpose**: Ensure comprehensive and consistent documentation across all FerrisDB code and specifications.  
**Prerequisites**: Understanding of Rust doc comments and markdown

## Code Documentation Standards

### Public API Documentation

- **Always** add comprehensive doc comments for all public APIs
- Include usage examples in doc comments
- Use `//!` for module-level documentation
- Add `#[doc(hidden)]` for internal implementation details
- Document safety requirements for any `unsafe` code
- Explain complex algorithms with inline comments

### Documentation Comments Format

````rust
/// Brief one-line description.
///
/// More detailed explanation of the functionality, including:
/// - Purpose and use cases
/// - Important behavior details
/// - Performance characteristics
///
/// # Arguments
///
/// * `arg1` - Description of first argument
/// * `arg2` - Description of second argument
///
/// # Returns
///
/// Description of return value and possible states
///
/// # Errors
///
/// Explanation of error conditions and types
///
/// # Examples
///
/// ```rust
/// use ferrisdb_storage::MemTable;
///
/// let memtable = MemTable::new();
/// memtable.insert(b"key", b"value", 1)?;
/// ```
///
/// # Panics
///
/// Conditions under which this function will panic (if any)
pub fn example_function(arg1: Type1, arg2: Type2) -> Result<ReturnType> {
    // Implementation
}
````

### Module Documentation

````rust
//! # Module Name
//!
//! Brief description of what this module provides.
//!
//! ## Overview
//!
//! More detailed explanation of the module's purpose and architecture.
//!
//! ## Examples
//!
//! ```rust
//! // Example usage of the module
//! ```
//!
//! ## Implementation Notes
//!
//! Important details about the implementation that users should know.
````

### Documentation Generation

```bash
# Generate documentation for all crates
cargo doc --all --no-deps --open

# Generate documentation with private items (for development)
cargo doc --all --no-deps --document-private-items

# Check documentation coverage
cargo doc --all --no-deps 2>&1 | grep "warning:"
```

## Technical Specification Documentation

### Visualization Guidelines

**IMPORTANT**: We prioritize clear visualizations to aid understanding. Use these approaches:

1. **ASCII Diagrams** - Primary visualization method

   - Architecture diagrams
   - Data flow representations
   - Component relationships
   - State transitions

2. **Tables** - For structured comparisons

   - Performance characteristics
   - Feature comparisons
   - Configuration options
   - API parameters

3. **Standard Diagram Types**

   - **Flow Charts** - Process flows and decision trees
   - **State Diagrams** - State machines and transitions
   - **Sequence Diagrams** - Interaction between components
   - **Activity Diagrams** - Workflow and parallel processes
   - **Block Diagrams** - System architecture

4. **Best Practices**

   - Keep diagrams simple and focused
   - Use consistent notation and symbols
   - Add legends when using special symbols
   - Align boxes and lines for readability
   - Update diagrams when implementation changes

5. **Implementation Status**
   - **Clearly mark speculative content** - Use "[PLANNED]" or "[CONCEPTUAL]"
   - **Update when implemented** - Remove speculative markers
   - **Track in TODOs** - Add TODO comments to revisit

Example ASCII diagram:

```
┌─────────────┐     ┌─────────────┐
│   Client    │────▶│   Server    │
└─────────────┘     └──────┬──────┘
                           │
                    ┌──────▼──────┐
                    │   Storage   │
                    └─────────────┘
```

Example state diagram:

```
    ┌─────────┐
    │  Init   │
    └────┬────┘
         │ start()
    ┌────▼────┐
    │ Running │◀─┐
    └────┬────┘  │ recover()
         │       │
         │ error()
    ┌────▼────┐  │
    │  Error  │──┘
    └─────────┘
```

### Architecture Documents

- Located in `/docs/architecture.md` and related files
- Must include clear ASCII diagrams
- Explain design decisions and trade-offs
- Link to relevant code implementations
- Keep updated as architecture evolves
- Mark speculative sections clearly

### API Reference

- Auto-generated from code documentation
- Supplement with usage guides where needed
- Include common patterns and best practices
- Document breaking changes clearly

## Documentation Honesty

### Implementation Status

- **Be transparent about implementation status** - Clearly indicate what's implemented vs planned
- **Don't claim features that don't exist** - Use "will" or "planned" for future features
- **Acknowledge limitations** - Be upfront about what the system can't do yet
- **Mark hypothetical examples** - Label code examples that show expected behavior vs actual
- **Update docs when features land** - Keep documentation in sync with actual capabilities

### Performance Claims

- **Never claim benchmark results without running them** - Be transparent about theoretical vs actual performance
- **Clearly label expected performance** - Use phrases like "expected", "theoretical", or "should achieve"
- **Document benchmark methodology** - When you do run benchmarks, document the setup and conditions
- **Avoid misleading claims** - Don't present example numbers as if they were measured results
- **Update when benchmarks are run** - Replace theoretical numbers with actual measurements once available

## Documentation Quality Standards

### Accuracy

- All code examples must compile and run
- Technical details must be correct
- Keep synchronized with code changes
- Review documentation in PRs

### Clarity

- Write for your target audience (developers using the API)
- Define technical terms on first use
- Use consistent terminology throughout
- Provide context for complex concepts

### Completeness

- Document all public APIs
- Include error conditions
- Explain performance characteristics
- Provide usage examples

### Maintenance

- Update documentation with code changes
- Remove outdated information
- Fix broken links and examples
- Regular documentation reviews

## Code Review Checklist

Documentation aspects to check during code review:

- [ ] All new public APIs have documentation
- [ ] Documentation includes examples
- [ ] Examples compile and run correctly
- [ ] Complex algorithms have explanatory comments
- [ ] Module documentation explains purpose
- [ ] Safety requirements documented for `unsafe` code
- [ ] Breaking changes noted in documentation
- [ ] Performance implications documented

## Tools and Commands

### Linting Documentation

```bash
# Check for common documentation issues
cargo clippy --all -- -W clippy::missing_docs_in_private_items

# Find undocumented public items
cargo rustdoc --all -- -D missing_docs
```

### Documentation Testing

```bash
# Run documentation tests
cargo test --doc --all

# Run specific documentation test
cargo test --doc --package ferrisdb-storage
```

## Best Practices

### Keep Documentation Close to Code

- Document functions where they're defined
- Update docs in the same commit as code changes
- Use inline comments for complex logic

### Write for Maintainers

- Explain "why" not just "what"
- Document design decisions
- Include links to relevant issues/discussions

### Make Examples Realistic

- Use real-world scenarios
- Show error handling
- Include complete, runnable code

### Version Documentation

- Note when features were added
- Document deprecations clearly
- Maintain compatibility notes

## Binary Format Documentation

### Overview

When documenting binary formats (e.g., SSTable format, WAL format, wire protocols), follow these standards to ensure clarity and maintainability.

### Documentation Location

- **Document in code** - Keep binary format documentation close to the implementation
- Use module-level documentation (`//!`) for overall format description
- Use struct/enum documentation for specific components
- Place detailed format tables near the relevant struct definitions
- **Never** create separate documentation files for binary formats

### Format Tables

Use structured tables with consistent columns:

```rust
//! ## SSTable Block Format
//!
//! | Offset | Size | Field        | Description                    |
//! |--------|------|--------------|--------------------------------|
//! | 0      | 4    | magic_number | 0x5354424C ('STBL' in ASCII)  |
//! | 4      | 4    | version      | Format version (little-endian) |
//! | 8      | 8    | block_size   | Size of data block in bytes    |
//! | 16     | 8    | num_entries  | Number of key-value entries    |
//! | 24     | var  | entries      | Sequential key-value pairs     |
//! | ...    | 4    | checksum     | CRC32 of all preceding bytes   |
```

### Visual ASCII Diagrams

Include ASCII diagrams to visualize the binary layout:

````rust
//! ## WAL Entry Format
//!
//! ```text
//! ┌─────────────┬─────────────┬─────────────┬─────────────┐
//! │ Magic (4B)  │ Seq No (8B) │ Type (1B)   │ Reserved(3B)│
//! ├─────────────┴─────────────┴─────────────┴─────────────┤
//! │ Timestamp (8B)                                         │
//! ├────────────────────────────────────────────────────────┤
//! │ Key Length (4B)           │ Value Length (4B)          │
//! ├────────────────────────────────────────────────────────┤
//! │ Key Data (variable length)                             │
//! ├────────────────────────────────────────────────────────┤
//! │ Value Data (variable length)                           │
//! ├────────────────────────────────────────────────────────┤
//! │ CRC32 Checksum (4B)                                    │
//! └────────────────────────────────────────────────────────┘
//! ```
````

### Example: Complete Binary Format Documentation

````rust
//! # SSTable File Format
//!
//! The SSTable (Sorted String Table) file format stores key-value pairs
//! in sorted order with efficient lookup capabilities.
//!
//! ## File Layout
//!
//! ```text
//! ┌──────────────────┐
//! │   File Header    │ - Magic number, version, metadata
//! ├──────────────────┤
//! │   Data Blocks    │ - Compressed key-value entries
//! ├──────────────────┤
//! │  Filter Block    │ - Bloom filter for fast lookups
//! ├──────────────────┤
//! │   Index Block    │ - Block offsets for binary search
//! ├──────────────────┤
//! │     Footer       │ - Pointers to meta blocks
//! └──────────────────┘
//! ```
//!
//! ## File Header Format
//!
//! | Offset | Size | Field         | Description                          |
//! |--------|------|---------------|--------------------------------------|
//! | 0      | 8    | magic         | 0x7373746162666462 ('sstabfdb')     |
//! | 8      | 4    | version       | Format version (currently 1)         |
//! | 12     | 4    | block_size    | Target uncompressed block size       |
//! | 16     | 8    | creation_time | Unix timestamp (microseconds)        |
//! | 24     | 8    | num_entries   | Total number of key-value pairs      |
//! | 32     | 8    | data_size     | Total size of uncompressed data      |
//! | 40     | 1    | compression   | Compression type (0=none, 1=snappy)  |
//! | 41     | 7    | reserved      | Reserved for future use (must be 0)  |

/// Represents an SSTable file header
#[repr(C)]
pub struct SSTableHeader {
    /// Magic number identifying this as an SSTable file
    pub magic: [u8; 8],
    /// Format version for backward compatibility
    pub version: u32,
    /// Target size for uncompressed data blocks
    pub block_size: u32,
    /// File creation timestamp in microseconds since epoch
    pub creation_time: u64,
    /// Total number of key-value entries in this SSTable
    pub num_entries: u64,
    /// Total size of uncompressed data in bytes
    pub data_size: u64,
    /// Compression algorithm used for data blocks
    pub compression: CompressionType,
    /// Reserved bytes for future extensions
    pub reserved: [u8; 7],
}
````

### Best Practices

1. **Use fixed-width fonts** in tables for alignment
2. **Include byte offsets** for every field
3. **Specify endianness** for multi-byte values
4. **Document magic numbers** with both hex and ASCII representations
5. **Show variable-length fields** clearly in diagrams
6. **Include checksums** and their calculation method
7. **Version your formats** for future compatibility

### Common Patterns

- **Magic Numbers**: Always at offset 0, include ASCII representation
- **Version Fields**: Follow magic number, use for compatibility
- **Length Prefixes**: Document whether they include themselves
- **Padding/Alignment**: Explicitly show reserved/padding bytes
- **Checksums**: Specify algorithm and coverage

### Version Documentation

- Note when features were added
- Document deprecations clearly
- Maintain compatibility notes

## Related Guidelines

- [Code Style](code-style.md) - Code formatting standards
- [Visualization](visualization.md) - Creating diagrams for documentation
- [Markdown Standards](markdown-standards.md) - Markdown formatting
- [Website Design](../content/website-design-starlight.md) - Public documentation

---
_Last updated: 2025-06-01_
