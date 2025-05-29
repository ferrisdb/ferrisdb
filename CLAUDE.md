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
- **Import Organization**: 
  - Organize imports in logical groups with blank lines between:
    1. Local crate imports (`crate::`, `ferrisdb_*::`)
    2. External crate imports (third-party dependencies)
    3. Standard library imports (`std::`)
    4. Test-only imports with `#[cfg(test)]` at the bottom
  - Use conditional imports `#[cfg(test)]` for symbols only used in tests to avoid unused import warnings
  - Prefer direct imports over fully qualified paths (e.g., `use ferrisdb_core::Operation;` then `Operation` instead of `ferrisdb_core::Operation`)

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
- Blog posts located in `docs/_claude_blog/` (Claude) and `docs/_posts/` (team)
- Use descriptive titles and include practical insights
- Tag posts with relevant categories for easy discovery
- Write posts after major features, interesting debugging sessions, or collaboration insights

**Blog Post Format (for main blog):**
```yaml
---
layout: post
title: "Your Title Here"
subtitle: "Brief description of what was accomplished"
date: YYYY-MM-DD
day: N  # Day number of development
tags: [tag1, tag2, tag3]
stats: ["📊 X tests passing", "📄 Y PRs merged", "⏱️ Key achievement"]
---
```

**When to Write Blog Posts:**
- End of each development day (summarizing progress)
- After major architectural decisions
- When solving interesting technical challenges
- After significant refactoring or optimization work

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

### Day-to-Day Development Tips

**Quick Iteration:**
- Use `cargo check` for fast compilation checks without building
- Use `cargo check -p <crate-name>` to check specific crates
- Use `cargo test -p <crate-name>` to test specific crates
- Use `cargo test -p <crate-name> --lib` to run only unit tests (skip doctests)

**Debugging Compilation Errors:**
- Start with `cargo check` to see all errors quickly
- Fix errors from top to bottom (earlier errors often cause later ones)
- Use `cargo check --tests` to include test compilation

**Running Tests Efficiently:**
- `cargo test --lib` - Run only library tests (faster)
- `cargo test <test_name>` - Run specific test by name
- `cargo test --release` - Test with optimizations (for performance tests)
- `cargo test -- --nocapture` - See println! output during tests
- `cargo test -- --test-threads=1` - Run tests sequentially for debugging

**Working with Multiple Crates:**
- Always specify `-p <crate-name>` to avoid building everything
- Use `--all` only when you need to verify workspace-wide changes

### Git Workflow

- Main branch: `main`
- Feature branches: `feature/description`
- Bug fixes: `fix/description`
- Documentation: `docs/description`
- Commit messages: Use conventional commits format
- Always run tests before pushing
- Create focused PRs (one feature/fix per PR)

### Development Process (REQUIRED FOR ALL CHANGES)

**Every change, no matter how small, must follow this process:**

1. **Create feature branch**: `git checkout -b <branch-type>/<description>`
2. **Make changes**: Edit files, add tests, update documentation
3. **Commit changes**: Use conventional commit messages
4. **Push branch**: `git push -u origin <branch-name>`
5. **Open PR**: `gh pr create` with descriptive title and body
6. **Iterate if needed**: Push more commits to the feature branch
7. **Merge when ready**: Only after all CI checks pass

**Example workflow:**
```bash
# Step 1: Create feature branch
git checkout -b docs/update-readme

# Step 2-3: Make changes and commit
git add README.md
git commit -m "docs: Update installation instructions"

# Step 4: Push branch
git push -u origin docs/update-readme

# Step 5: Create PR
gh pr create --title "docs: Update installation instructions" --body "..."

# Step 6: If changes requested, add more commits
git add .
git commit -m "docs: Address review feedback"
git push

# Step 7: Merge (only after CI passes)
gh pr merge <PR-number> --squash
```

### Pull Request Policy

- **All changes must go through PRs** - This includes:
  - Code changes (features, bug fixes, refactoring)
  - Documentation updates (README, guides, comments)
  - Configuration changes (Cargo.toml, CI files)
  - Any file in the repository
- **NO EXCEPTIONS**: Even single-line typo fixes must use PRs
- **CRITICAL**: Never push directly to main branch - always use PRs
- **Maintainers**: Can merge PRs after all CI checks pass (no review required)
- **External contributors**: Require review from a maintainer
- All PRs must pass CI checks before merging
- Use squash merge to keep history clean
- **No direct pushes to main** - Admin privileges are for emergencies only
- **If you accidentally push to main**: Leave it as is, but be more careful in the future

### PR Description Guidelines

**Every PR should include:**

1. **Summary** - Brief overview of changes (2-3 sentences)
2. **Changes Made** - Bullet points of specific modifications
3. **Why This Matters** - Context and motivation
4. **Testing** - What tests were added/modified
5. **Breaking Changes** - Note any API changes (if applicable)

**PR Description Template:**
```markdown
## Summary
Brief description of what this PR accomplishes and why.

## Changes Made
- Change 1: Description
- Change 2: Description
- Change 3: Description

## Why This Matters
Explain the motivation and benefits of these changes.

## Testing
- Added unit tests for X
- Updated integration tests for Y
- All existing tests pass

## Breaking Changes
None / List any breaking changes here
```

**Good PR Practices:**
- Keep PRs focused on a single feature/fix
- Include relevant issue numbers (Fixes #123)
- Add reviewers if specific expertise needed
- Update documentation in the same PR as code changes
- Include before/after examples for API changes

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
