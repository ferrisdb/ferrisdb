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

## Related Guidelines

- [Code Style](code-style.md) - Code formatting standards
- [Visualization](visualization.md) - Creating diagrams for documentation
- [Markdown Standards](markdown-standards.md) - Markdown formatting
- [Website Design](../content/website-design.md) - Public documentation
