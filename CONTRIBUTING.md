# Contributing to FerrisDB

Thank you for your interest in contributing to FerrisDB! This document provides guidelines for contributing to our educational distributed database project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [How to Contribute](#how-to-contribute)
- [Development Workflow](#development-workflow)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Documentation](#documentation)
- [Community](#community)

## Code of Conduct

This project is an educational initiative focused on learning distributed systems. We welcome contributors of all skill levels and backgrounds. Please be respectful, patient, and constructive in all interactions.

## Getting Started

### Prerequisites

- **Rust**: Install via [rustup](https://rustup.rs/) (MSRV: 1.81.0)
- **Git**: For version control
- **Ruby** (optional): For Jekyll documentation site

### Initial Setup

1. **Fork and clone the repository**:

   ```bash
   git clone https://github.com/YOUR_USERNAME/ferrisdb.git
   cd ferrisdb
   ```

2. **Set up the development environment**:

   ```bash
   # Install Rust toolchain
   rustup toolchain install stable
   rustup default stable
   rustup component add rustfmt clippy

   # Build the project
   cargo build --all

   # Run tests to ensure everything works
   cargo test --all
   ```

3. **Read the development guide**: See [DEVELOPMENT.md](./DEVELOPMENT.md) for detailed setup instructions.

## How to Contribute

### Types of Contributions Welcome

- **Bug fixes**: Help identify and fix issues
- **Feature implementation**: Implement planned features (check issues/TODO.md)
- **Documentation**: Improve guides, comments, and examples
- **Testing**: Add tests and improve test coverage
- **Performance**: Optimize critical paths with benchmarks
- **Educational content**: Blog posts, tutorials, examples

### Finding Work

1. **Check existing issues**: Look for `good first issue` or `help wanted` labels
2. **Review TODO.md**: See planned features and improvements
3. **Ask questions**: Open a discussion if you're unsure where to start
4. **Propose ideas**: Open an issue to discuss new features or improvements

## Development Workflow

### Branch Strategy

- **main**: Stable branch, always buildable
- **feature/description**: New features
- **fix/description**: Bug fixes
- **docs/description**: Documentation changes

### Making Changes

1. **Create a feature branch**:

   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes following our [coding standards](#coding-standards)**

3. **Test your changes**:

   ```bash
   # Format code
   cargo fmt --all

   # Run linter
   cargo clippy --all-targets --all-features -- -D warnings

   # Run tests
   cargo test --all

   # For docs changes
   markdownlint-cli2 "**/*.md" "!target/**" "!**/target/**"
   ```

4. **Commit with conventional commits**:

   ```bash
   git commit -m "feat: add new storage engine feature"
   git commit -m "fix: resolve memory leak in compaction"
   git commit -m "docs: update architecture guide"
   ```

## Pull Request Process

### Before Submitting

- [ ] Code builds without warnings
- [ ] All tests pass
- [ ] Documentation is updated if needed
- [ ] Commit messages follow conventional format
- [ ] Branch is up to date with main

### PR Requirements

1. **Clear description**: Explain what changes you made and why
2. **Link issues**: Reference related issues with `Fixes #123` or `Closes #456`
3. **Test coverage**: Include tests for new functionality
4. **Documentation**: Update relevant docs (code comments, guides, etc.)
5. **Breaking changes**: Clearly mark and justify any breaking changes

### Review Process

- **Maintainers**: Can self-merge after CI passes (no review required)
- **External contributors**: Require review from a maintainer
- **All PRs**: Must pass CI checks before merging
- **Merge strategy**: Squash merge to keep history clean

### CI Optimization

Our CI uses path-based filtering:

- **Docs-only changes**: Run markdown lint, spell check, Jekyll build (~3 min)
- **Code changes**: Run full Rust test suite (~15-20 min)

Add `[docs only]` to your commit message for documentation-only changes.

## Coding Standards

### Rust Code Style

- **Formatting**: Use `cargo fmt` (enforced by CI)
- **Linting**: Address all `cargo clippy` warnings
- **Line length**: 100 characters maximum
- **Naming**: `snake_case` for functions/variables, `CamelCase` for types
- **Documentation**: All public APIs must have doc comments with examples

### Error Handling

- Use `thiserror` for error types
- Always propagate errors with context
- Never panic in production code paths
- Use descriptive error messages

### Memory Safety

- Use `unsafe` sparingly and document safety invariants
- Prefer safe abstractions over raw pointers
- Document lifetime requirements clearly

### Performance

- Profile before optimizing
- Use `criterion` for benchmarks
- Avoid unnecessary allocations
- Prefer zero-copy operations where possible

## Testing Guidelines

### Test Categories

1. **Unit tests**: Test individual functions and methods
2. **Integration tests**: Test component interactions
3. **Property tests**: Use `proptest` for complex invariants
4. **Benchmarks**: Use `criterion` for performance-critical code

### Test Requirements

- **Coverage target**: >80%
- **Edge cases**: Test boundary conditions and error paths
- **Documentation**: Include examples in doc comments that run as doctests
- **Deterministic**: Tests should not be flaky

### Running Tests

```bash
# All tests
cargo test --all

# Specific crate
cargo test -p ferrisdb-storage

# With output
cargo test --all -- --nocapture

# Benchmarks
cargo bench
```

## Documentation

### Types of Documentation

1. **Code documentation**: Doc comments with examples
2. **User guides**: Getting started, tutorials
3. **Architecture docs**: System design and decisions
4. **API documentation**: Generated with `cargo doc`
5. **Blog posts**: Development insights and learnings

### Documentation Standards

- **Always document public APIs** with usage examples
- **Use `//!` for module-level documentation**
- **Include examples that compile and run**
- **Update guides when adding features**
- **Lint markdown**: Run `markdownlint-cli2` before committing

### Building Documentation

```bash
# Generate API docs
cargo doc --all --no-deps --open

# Build Jekyll site
cd docs && bundle exec jekyll serve
```

## Community

### Communication Channels

- **Issues**: Bug reports, feature requests, questions
- **Discussions**: General questions, ideas, feedback
- **Pull Requests**: Code review and collaboration

### Getting Help

- **New to Rust?** Check out [The Rust Book](https://doc.rust-lang.org/book/)
- **New to distributed systems?** See our [Architecture Guide](docs/architecture.md)
- **Stuck on something?** Open a discussion or ask in issues

### Recognition

Contributors are recognized in:

- Commit co-authorship for significant contributions
- Release notes for feature contributions
- Special thanks in project documentation

### Show Your Support

If you find FerrisDB interesting or useful for learning:

- ⭐ **Star the repository** on GitHub to show your support
- 🍴 **Fork the project** to experiment with your own ideas
- 📢 **Share your learnings** by writing blog posts or tutorials
- 💬 **Join discussions** to help others in the community

## Educational Focus

Remember that FerrisDB is an **educational project**:

- **Learning over perfection**: We value clear, understandable code
- **Documentation matters**: Explain the "why" behind decisions
- **Share knowledge**: Blog about interesting problems you solve
- **Be patient**: Help others learn, regardless of their experience level

## License

By contributing to FerrisDB, you agree that your contributions will be licensed under the project's license.

---

Thank you for contributing to FerrisDB! Together, we're building not just a database, but a learning community around distributed systems and Rust. 🦀
