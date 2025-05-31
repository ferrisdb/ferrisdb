# Testing Standards

This document outlines the testing requirements and standards for FerrisDB.

**Purpose**: Ensure code quality and prevent regressions through comprehensive testing practices.  
**Prerequisites**: Basic understanding of Rust's testing framework  
**Related**: [Code Style](../development/code-style.md), [PR Process](pr-process.md), [Commands](commands.md#testing)

## Test Requirements

### Mandatory Test Coverage

**Every component MUST have:**

1. **Unit Tests**: All public methods with comprehensive scenarios
2. **Integration Tests**: Component interactions and API contracts
3. **Concurrent Tests**: Required when using Arc, Mutex, channels, or atomics
4. **Benchmarks**: Required when claiming performance benefits
5. **Property Tests**: For algorithms with complex invariants

### When Concurrent Tests Are Required

You MUST add concurrent tests if your code:

- Uses `Arc`, `Mutex`, `RwLock`, or other sync primitives
- Implements `Send` or `Sync` traits
- Uses channels or message passing
- Modifies shared state
- Claims thread-safety

Example concurrent test:

```rust
#[test]
fn concurrent_writes_maintain_consistency() {
    let store = Arc::new(Mutex::new(KeyValueStore::new()));
    let mut handles = vec![];

    for i in 0..10 {
        let store = Arc::clone(&store);
        handles.push(thread::spawn(move || {
            store.lock().unwrap().set(format!("key{}", i), format!("value{}", i));
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(store.lock().unwrap().len(), 10);
}
```

### Benchmark Requirements

**MANDATORY**: If you claim performance characteristics, you MUST prove them with benchmarks.

```rust
// If you claim "O(1) lookups", prove it:
#[bench]
fn bench_get_performance_scales_constant(b: &mut Bencher) {
    let mut store = KeyValueStore::new();
    // Add 10,000 entries
    for i in 0..10_000 {
        store.set(format!("key{}", i), format!("value{}", i));
    }

    b.iter(|| {
        store.get("key5000")
    });
}
```

Performance claims without benchmarks will be rejected in PR review.

### Test Organization

```
component_name/
├── src/
│   └── lib.rs
├── tests/
│   ├── unit_tests.rs
│   ├── integration_tests.rs
│   ├── concurrent_tests.rs    # When applicable
│   └── property_tests.rs      # When applicable
├── benches/
│   └── performance.rs         # When making performance claims
└── exercises/                 # For educational components
    ├── challenge_01.rs
    └── solutions/
        └── challenge_01_solution.rs
```

## Running Tests

```bash
# Run all tests
cargo test --all

# Run with output
cargo test --all -- --nocapture

# Run specific test
cargo test test_name

# Run tests in release mode
cargo test --release
```

## Test Categories

### Unit Tests

- Test individual functions and methods
- Keep tests close to the code they test
- Use descriptive test names

### Integration Tests

- Test interactions between components
- Place in `tests/` directory
- Test public API behavior

### Property-Based Tests

Consider using `proptest` for testing with arbitrary inputs:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_serialization_roundtrip(key: String, value: Vec<u8>) {
        // Test that serialization and deserialization are inverse operations
    }
}
```

## Best Practices

1. **Test names MUST describe behavior, not method names**

   ```rust
   // ✅ GOOD: Describes behavior and conditions
   #[test]
   fn get_returns_none_for_missing_key() { }

   #[test]
   fn set_overwrites_existing_value() { }

   #[test]
   fn scan_prefix_returns_lexicographically_sorted_results() { }

   #[test]
   fn concurrent_writes_maintain_consistency() { }

   // ❌ BAD: Generic names that don't describe behavior
   #[test]
   fn test_get() { }

   #[test]
   fn test_basic() { }

   #[test]
   fn test_edge_cases() { }
   ```

   **Naming Format**: `method_name_expected_behavior_under_condition`

   - Start with the method/operation being tested
   - Describe the expected outcome
   - Include relevant conditions or context
   - Avoid redundant `test_` prefix

2. **Use helper functions to reduce duplication**

   ```rust
   fn create_test_memtable() -> MemTable {
       // Common setup code
   }
   ```

3. **Test one thing at a time**

   - Each test should verify a single behavior
   - Multiple assertions are fine if testing related aspects

4. **Clean up resources**
   - Use RAII patterns for test resources
   - Clean up temporary files

## Test Coverage Requirements

### Coverage Standards

**Target: 100% Code Coverage**

FerrisDB targets 100% test coverage because:

- Databases require exceptional reliability
- Users trust us with their data
- Every line of code should be justified and tested
- High coverage catches more bugs before production

**Required coverage:**

- **Unit Test Coverage**: 100% of all code paths
- **Public API Coverage**: 100% - Every public method must have tests
- **Error Path Coverage**: 100% - All `Result::Err` cases must be tested
- **Edge Case Coverage**: Explicit tests for boundary conditions

### Measuring Coverage

```bash
# Install cargo-tarpaulin
cargo install cargo-tarpaulin

# Run coverage report
cargo tarpaulin --out Html --output-dir coverage

# Check coverage meets standards (100% target)
cargo tarpaulin --print-summary --fail-under 100
```

### What Must Be Tested

1. **Every Public Method**: No exceptions

   ```rust
   // If it's pub, it needs tests
   pub fn new() -> Self { }          // ✓ Test creation
   pub fn get(&self) -> Option<T> { } // ✓ Test Some and None cases
   pub fn set(&mut self) { }         // ✓ Test normal and edge cases
   ```

2. **All Error Conditions**:

   ```rust
   #[test]
   fn operation_fails_with_invalid_input() {
       let result = parse_config("invalid");
       assert!(matches!(result, Err(Error::InvalidConfig(_))));
   }
   ```

3. **Boundary Conditions**:

   ```rust
   #[test]
   fn handles_empty_input() { }

   #[test]
   fn handles_maximum_size() { }

   #[test]
   fn handles_unicode_correctly() { }
   ```

### Coverage Exemptions

**Default expectation: 100% coverage**. Exemptions are rare and must be explicitly justified.

#### Allowed Exemptions

Code that may be excluded from 100% coverage requirement:

1. **Platform-specific code**: Only testable on specific platforms in CI

   ```rust
   #[cfg(not(tarpaulin_include))]  // Platform-specific: Windows-only error handling
   #[cfg(windows)]
   fn handle_windows_error() -> Result<()> { ... }
   ```

2. **Unreachable panic handlers**: Truly unreachable safety assertions

   ```rust
   #[cfg(not(tarpaulin_include))]  // Unreachable: protected by type system
   unreachable!("This branch is impossible due to enum exhaustiveness")
   ```

3. **Complex macro-generated code**: If the macro itself is tested

   ```rust
   // If macro logic is tested separately, generated code may be exempt
   #[cfg(not(tarpaulin_include))]  // Generated by tested macro
   some_complex_generated_function!();
   ```

4. **Debug-only development code**: Code only used in development
   ```rust
   #[cfg(not(tarpaulin_include))]  // Debug-only: not shipped in release
   #[cfg(debug_assertions)]
   fn debug_only_function() { ... }
   ```

#### Exemption Requirements

1. **Explicit annotation**: Use `#[cfg(not(tarpaulin_include))]`
2. **Comment justification**: Explain why it can't be tested
3. **PR documentation**: Describe exemptions in PR description
4. **Reviewer approval**: Exemptions require explicit reviewer acknowledgment

#### Examples of REJECTED Exemptions

```rust
// ❌ REJECTED: Can be tested with proper setup
fn difficult_to_test() { ... }

// ❌ REJECTED: Should use Result instead
fn might_panic() { panic!("sometimes fails"); }

// ❌ REJECTED: Lazy testing, not a real exemption
fn i_dont_want_to_test_this() { ... }
```

**Remember**: If you think code needs an exemption, first ask if the code can be refactored to be more testable.

## Continuous Integration

All tests run automatically on:

- Every push to a PR
- Before merging to main
- Nightly for extended test suites

Tests must pass before merging any PR.
