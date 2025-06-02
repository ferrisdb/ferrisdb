# Common Commands

Quick reference for frequently used commands in FerrisDB development.

**Purpose**: Provide a single reference for all commonly used development and maintenance commands.  
**Prerequisites**: Basic familiarity with command line, git, and cargo

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

## Starlight/Astro Commands

### Local Development

```bash
# Install dependencies
cd docs && npm install

# Serve locally (development mode)
cd docs && npm run dev

# Build site (production)
cd docs && npm run build

# Preview production build
cd docs && npm run preview

# Type check
cd docs && npm run astro check
```

## Helpful Aliases

Add these to your shell configuration:

```bash
# Quick test and format
alias fdb-check='cargo fmt --all && cargo clippy --all-targets --all-features -- -D warnings && cargo test --all'

# Quick markdown check
alias fdb-md='prettier --check "**/*.md"'

# Serve docs locally
alias fdb-docs='cd docs && npm run dev'
```

## Website Maintenance Commands

### Statistics and Metrics

```bash
# Cached statistics function (avoids recomputing)
get_cached_stats() {
    local current_commit=$(git rev-parse HEAD)
    local cache_file="/tmp/ferrisdb_stats_cache.txt"

    if [[ -f "$cache_file" ]]; then
        local cached_commit=$(head -n1 "$cache_file" 2>/dev/null)
        if [[ "$cached_commit" == "$current_commit" ]]; then
            tail -n+2 "$cache_file"
            return 0
        fi
    fi

    echo "Computing fresh statistics for commit $current_commit..."
    local days=$(git log --format="%ad" --date=short -- "*.rs" "Cargo.toml" | sort | uniq | wc -l)
    local lines=$(find . -path ./target -prune -o -name "*.rs" -type f -print | xargs wc -l | tail -1 | awk '{print $1}')
    local tests=$(grep -r "#\[test\]" --include="*.rs" . | wc -l)
    local posts=$(find docs/_posts -name "*.md" | grep -v template | wc -l)

    {
        echo "$current_commit"
        echo "DAYS=$days"
        echo "LINES=$lines"
        echo "TESTS=$tests"
        echo "POSTS=$posts"
        echo "SUMMARY=\"Day $days of development with $lines lines of Rust code, $tests passing tests, and $posts blog posts\""
    } > "$cache_file"

    tail -n+2 "$cache_file"
}

# Usage
eval "$(get_cached_stats)"
echo "Days: $DAYS, Lines: $LINES, Tests: $TESTS, Posts: $POSTS"
```

### Content Validation

```bash
# Find all progress sections
grep -n "Progress\|What's Working\|Currently Building\|Coming" docs/*.md

# Check for outdated status tags
grep -r "\[PLANNED\]\|\[CONCEPTUAL\]\|\[FUTURE\]" docs/ --include="*.md"

# Verify companion blog posts exist
for post in docs/_posts/*human*.md; do
    day=$(grep "day:" "$post" | cut -d' ' -f2)
    if ! find docs/_posts -name "*claude*day-$day-*.md" | grep -q .; then
        echo "Missing Claude post for day $day"
    fi
done
```

## Before Committing Checklist

Run these commands before every commit:

1. `cargo fmt --all` - Format code
2. `cargo clippy --all-targets --all-features -- -D warnings` - Check for issues
3. `cargo test --all` - Run tests
4. `prettier --write "**/*.md"` - Format markdown

## Related Guidelines

- [Git Workflow](git-workflow.md) - Using git commands effectively
- [Website Maintenance](website-maintenance-simple.md) - Maintenance workflows
- [Testing](testing.md) - Test command details
- [Markdown Standards](../development/markdown-standards.md) - Markdown tools

---
_Last updated: 2025-06-01_
