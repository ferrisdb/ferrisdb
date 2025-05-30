# Common Commands

Quick reference for frequently used commands in FerrisDB development.

## Development Commands

### Building

```bash
# Build all crates
cargo build --all

# Build in release mode
cargo build --all --release

# Build specific crate
cargo build -p ferrisdb-storage
```

### Testing

```bash
# Run all tests
cargo test --all

# Run tests with output
cargo test --all -- --nocapture

# Run specific test
cargo test test_name

# Run tests for specific crate
cargo test -p ferrisdb-storage
```

### Code Quality

```bash
# Format code
cargo fmt --all

# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Run all checks (format, clippy, test)
cargo fmt --all && cargo clippy --all-targets --all-features -- -D warnings && cargo test --all
```

## Documentation Commands

### Markdown

```bash
# Format markdown files
prettier --write "**/*.md"

# Check markdown formatting
prettier --check "**/*.md"

# Lint markdown
markdownlint-cli2 "**/*.md"

# Fix markdown issues
markdownlint-cli2 "**/*.md" --fix
```

### Rust Docs

```bash
# Generate documentation
cargo doc --all --no-deps

# Generate and open documentation
cargo doc --all --no-deps --open

# Generate documentation with private items
cargo doc --all --no-deps --document-private-items
```

## Git Commands

### Branching

```bash
# Create feature branch
git checkout -b feature/your-feature

# Create from specific commit
git checkout -b feature/your-feature commit-hash
```

### Committing

```bash
# Stage all changes
git add -A

# Commit with message
git commit -m "type: Short description

Longer description if needed

🤖 Generated with Claude Code
Co-Authored-By: Claude <noreply@anthropic.com>"

# Amend last commit
git commit --amend
```

### Pull Requests

```bash
# Push branch and create PR
git push -u origin feature/your-feature
gh pr create

# Create PR with title and body
gh pr create --title "Title" --body "Description"

# List PRs
gh pr list

# Merge PR
gh pr merge PR_NUMBER --squash
```

## Jekyll Commands

### Local Development

```bash
# Install dependencies
cd docs && bundle install

# Serve locally
cd docs && bundle exec jekyll serve

# Build site
cd docs && bundle exec jekyll build

# Clean and rebuild
cd docs && bundle exec jekyll clean && bundle exec jekyll build
```

## Helpful Aliases

Add these to your shell configuration:

```bash
# Quick test and format
alias fdb-check='cargo fmt --all && cargo clippy --all-targets --all-features -- -D warnings && cargo test --all'

# Quick markdown check
alias fdb-md='prettier --check "**/*.md" && markdownlint-cli2 "**/*.md"'

# Serve docs locally
alias fdb-docs='cd docs && bundle exec jekyll serve'
```

## Before Committing Checklist

Run these commands before every commit:

1. `cargo fmt --all` - Format code
2. `cargo clippy --all-targets --all-features -- -D warnings` - Check for issues
3. `cargo test --all` - Run tests
4. `prettier --write "**/*.md"` - Format markdown
5. `markdownlint-cli2 "**/*.md"` - Lint markdown
