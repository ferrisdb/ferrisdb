# Utility Module Guidelines

Shared utilities require special attention as they're used across the codebase. This guide covers when and how to create utility modules.

**Purpose**: Define standards for creating and maintaining shared utility modules  
**Prerequisites**: Understanding of Rust modules and trait system

## When to Create a Utility

Create a utility when:

- Functionality is needed by multiple modules
- The implementation involves performance optimizations
- You're abstracting complex but reusable patterns
- Standard library or external crates don't provide the exact functionality needed

## Module Structure

```
src/
└── utils/
    ├── mod.rs          # Public exports and module documentation
    ├── README.md       # Comprehensive documentation
    └── feature.rs      # Implementation with inline tests
```

### Example Structure

```
ferrisdb-storage/src/
└── utils/
    ├── mod.rs          # Exports BytesMutExt
    ├── README.md       # Documents the utils module
    └── bytes_ext.rs    # BytesMutExt implementation
```

## Requirements

### 1. Documentation

Every utility module must include:

- Module-level documentation explaining purpose
- README.md with usage examples and design rationale
- Comprehensive inline documentation for all public APIs
- Performance characteristics if relevant

### 2. Testing

Minimum test requirements:

- **16+ tests** covering all scenarios
- Unit tests for basic functionality
- Error condition tests
- Boundary and edge case tests
- Thread safety tests if applicable
- Property-based tests for complex invariants
- See [Testing Utility Modules](../workflow/testing.md#testing-utility-modules)

### 3. Benchmarks

Required if claiming performance benefits:

- Use criterion for reliable measurements
- Compare against standard approaches
- Test multiple scenarios (small, medium, large)
- Document actual performance improvements

### 4. Safety

If using unsafe code:

- Minimize unsafe blocks
- Document all safety invariants with SAFETY comments
- Include specific tests verifying memory safety
- Ensure error paths don't expose uninitialized memory

## Implementation Guidelines

### Extension Traits

When extending external types:

```rust
// Good: Clear naming with Ext suffix
pub trait BytesMutExt {
    /// Comprehensive documentation
    fn read_exact_from<R: Read>(&mut self, reader: &mut R, count: usize) -> io::Result<()>;
}

impl BytesMutExt for BytesMut {
    fn read_exact_from<R: Read>(&mut self, reader: &mut R, count: usize) -> io::Result<()> {
        // Implementation
    }
}
```

### Error Handling

Utilities should:

- Use appropriate error types
- Propagate errors correctly
- Never panic in normal operation
- Document error conditions

### Performance

When optimizing for performance:

- Measure first, optimize second
- Document performance characteristics
- Provide benchmarks as proof
- Consider trade-offs (e.g., unsafe code for speed)

## Example: BytesMutExt

`ferrisdb-storage/src/utils/bytes_ext.rs` demonstrates:

1. **Clear Purpose**: Avoids zero-initialization overhead
2. **Comprehensive Tests**: 16 tests including safety verification
3. **Benchmarks**: Proves 23-33% performance improvement
4. **Safe Abstraction**: Unsafe code wrapped in safe API
5. **Documentation**: README explains design and usage

## Maintenance

Utility modules require extra care:

- Keep backward compatibility when possible
- Update all callers when changing APIs
- Maintain comprehensive test coverage
- Document breaking changes clearly

## Related Guidelines

- **Testing**: [Testing Standards](../workflow/testing.md) - General testing requirements
- **Code Style**: [Extension Traits](code-style.md#extension-traits) - Extension trait patterns
- **Safety**: [Unsafe Code](idiomatic-rust.md#unsafe-code) - Working with unsafe
- **Performance**: [Performance Guidelines](../technical/performance.md) - Optimization standards

---

_Last updated: 2025-06-01_
