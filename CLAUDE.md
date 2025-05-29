# FerrisDB Development Guidelines

## Project Overview

FerrisDB is a distributed, transactional key-value database inspired by FoundationDB, implemented in Rust.

## Development Standards

### Code Style

- Follow Rust standard formatting with `rustfmt`
- Use `clippy` for linting
- Maximum line length: 100 characters
- Use descriptive variable names
- Prefer `snake_case` for functions and variables, `CamelCase` for types

### Idiomatic Rust Guidelines

- **Module Organization**: 
  - Use snake_case file names that match struct names (e.g., `MemTable` → `mem_table.rs`)
  - Keep public API types in separate files, not in `mod.rs`
  - Only re-export types that should be part of the public API
  - Keep implementation details private (use `super::module::Type` imports for internal types)
  - Avoid module inception (module name ≠ file name in same directory)
- **Error Handling**: Always use `Result<T>` for fallible operations, never panic in library code
- **Trait Bounds**: Use `Send + Sync` where appropriate for types used across threads
- **Ownership**: Prefer owned types in public APIs, use references only when necessary
- **Iterator Patterns**: Use `.iter().enumerate()` instead of manual indexing where possible
- **Memory Safety**: Document safety invariants clearly for any `unsafe` code
- **Type Aliases**: Use meaningful type aliases for complex generic types
- **Pattern Matching**: Use exhaustive pattern matching and avoid catch-all `_` patterns when possible
- **Encapsulation**: Internal implementation details should not be exposed via `pub` or `pub(crate)` unless necessary

### Documentation

- **Always** add comprehensive doc comments for all public APIs
- Include usage examples in doc comments
- Use `//!` for module-level documentation
- Add `#[doc(hidden)]` for internal implementation details
- Generate docs with `cargo doc --all --no-deps --open`
- Review generated documentation before submitting PRs
- **Run markdown linting** with `markdownlint-cli2 "**/*.md"` before committing
- Use `prettier --write "**/*.md"` to auto-fix formatting issues

### Blogging

- **Claude's Dev Blog**: Share AI perspective on collaboration, lessons learned, and tips for human-AI development
- **Regular Blog Posts**: Document significant milestones, architectural decisions, and learning experiences
- Blog posts located in `docs/_claude_blog/_posts/` (Claude) and `docs/_posts/` (team)
- Use descriptive titles and include practical insights
- Tag posts with relevant categories for easy discovery
- Write posts after major features, interesting debugging sessions, or collaboration insights

### Testing

- Write unit tests for all public APIs
- Integration tests for distributed scenarios
- Use `proptest` for property-based testing
- Test coverage target: >80%
- Run tests with `cargo test --all`
- Use `cargo test --all -- --nocapture` for debugging

### Commands

```bash
# Format code
cargo fmt --all

# Run linter
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
cargo test --all

# Run specific crate tests
cargo test -p ferrisdb-storage

# Generate documentation
cargo doc --all --no-deps --open

# Run benchmarks
cargo bench

# Build all workspace members
cargo build --all

# Run with logging
RUST_LOG=debug cargo run

# Lint markdown files
markdownlint-cli2 "**/*.md" "!target/**" "!**/target/**"

# Auto-fix markdown formatting
prettier --write "**/*.md"
```

### Git Workflow

- Main branch: `main`
- Feature branches: `feature/description`
- Bug fixes: `fix/description`
- Commit messages: Use conventional commits format
- Always run tests before pushing
- Create focused PRs (one feature/fix per PR)

### Pull Request Policy

- **Maintainers**: Can merge PRs after all CI checks pass (no review required)
- **External contributors**: Require review from a maintainer
- All PRs must pass CI checks before merging
- Use squash merge to keep history clean

### Architecture Decisions

- Use `tokio` for async runtime
- Custom LSM-tree storage engine implementation
- gRPC (Tonic) for network protocol
- Raft for consensus (evaluate existing libraries first)

### Storage Engine Guidelines

- Binary formats should include checksums for corruption detection
- Use little-endian byte order consistently
- Implement proper error handling for all I/O operations
- Use `#[allow(dead_code)]` temporarily for unused fields/methods
- Remember to remove `#[allow(dead_code)]` when implementing features
- Use epoch-based memory reclamation for lock-free data structures

### Error Handling

- Use `thiserror` for error types
- Always propagate errors with context
- Log errors at appropriate levels
- Never panic in production code paths
- Use custom `Result<T>` type alias for consistency

### Performance Guidelines

- Profile before optimizing
- Use `criterion` for benchmarks
- Avoid unnecessary allocations
- Prefer zero-copy operations where possible
- Use `bytes` crate for efficient buffer management
- Implement proper batching for I/O operations

### Memory Safety

- Use `unsafe` sparingly and document safety invariants
- Prefer safe abstractions over raw pointers
- Use `crossbeam` for lock-free data structures
- Document lifetime requirements clearly

### Security

- Never log sensitive data
- Use TLS for network communication
- Validate all inputs
- Follow principle of least privilege
- Use checksums for data integrity

## Project Structure

```text
ferrisdb/
├── ferrisdb-core/       # Common types and traits
├── ferrisdb-storage/    # Storage engine
│   ├── wal/            # Write-ahead log
│   ├── memtable/       # In-memory storage
│   ├── sstable/        # Sorted string tables
│   └── compaction/     # Compaction logic
├── ferrisdb-client/     # Client library
├── ferrisdb-server/     # Server implementation
├── tests/              # Integration tests
├── benches/            # Benchmarks
└── docs/               # Additional documentation
```

## Key Invariants

1. Transactions must be serializable
2. All writes must be durable before acknowledgment
3. Node failures must not cause data loss
4. Reads must see a consistent snapshot
5. WAL entries must be written before MemTable updates
6. Timestamps must be monotonically increasing

## Storage Engine Invariants

1. Keys in MemTable are sorted by (user_key, timestamp DESC)
2. Multiple versions of same key ordered by timestamp
3. Delete operations create tombstones (not immediate deletion)
4. Compaction removes obsolete versions and tombstones
5. All disk writes include checksums

## Dependencies

- Prefer well-maintained, popular crates
- Minimize dependency count
- Pin major versions in Cargo.toml
- Review security advisories regularly
- Document why each dependency is needed

## Debugging Tips

- Use `RUST_LOG=trace` for detailed logging
- Enable debug symbols in release builds for profiling
- Use `tokio-console` for async runtime debugging
- Capture traces for distributed debugging
- Use `RUST_BACKTRACE=1` for panic debugging
- Add debug assertions for invariants

## Code Review Checklist

- [ ] All public APIs have documentation
- [ ] Examples included in doc comments
- [ ] Unit tests cover edge cases
- [ ] No clippy warnings
- [ ] No unnecessary `#[allow(...)]` attributes
- [ ] Error messages are descriptive
- [ ] TODOs are tracked in TODO.md
- [ ] Performance implications considered
