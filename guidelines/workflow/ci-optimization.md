# CI Optimization Guidelines

This document outlines our strategy for maintaining comprehensive test coverage while keeping CI times reasonable.

## Test Categories

### 🚀 **Fast Tests (Always Run)**

- **Unit tests**: Core functionality, runs on all platforms
- **Compilation checks**: Clippy, formatting, basic build
- **Documentation**: Doc tests and link checking
- **Time**: 3-5 minutes per platform

### ⚡ **Medium Tests (Conditional)**

- **Integration tests**: Full WAL workflows, Ubuntu only
- **MSRV checks**: Only when storage code changes
- **Tutorial tests**: Only when Rust code changes
- **Starlight validation**: Only when docs change
- **Time**: 5-10 minutes

### 🐌 **Slow Tests (Selective)**

- **Property tests**: Only on main branch or explicit request
- **Benchmarks**: Only when explicitly needed
- **Full cross-platform**: Only on main branch or with label
- **Security audit**: On dependency changes
- **Time**: 15-30 minutes

## Triggering Comprehensive Tests

### PR Labels for Extended Testing

Add these labels to PRs when needed:

- `test:property` - Run property tests with full test cases
- `test:benchmarks` - Run performance benchmarks
- `test:full` - Run complete test suite on all platforms

### Environment Variables

Tests automatically adapt to CI environment:

```bash
# CI environment reduces test scope
CI=true cargo test

# Local development uses full scope
cargo test
```

## PropTest Configuration

Property tests use different configurations:

- **CI**: 20 test cases, 50 shrink iterations, 30s timeout
- **Local**: 256 test cases, 1024 shrink iterations, 60s timeout

Configuration file (`proptest.toml`):

```toml
[proptest]
cases = 20               # CI default
max_shrink_iters = 50    # CI default
timeout = 30000          # 30 seconds

[proptest.local]
cases = 256              # Local development
max_shrink_iters = 1024  # More thorough shrinking
timeout = 60000          # 60 seconds
```

## Optimization Strategies Applied

### 1. **Intelligent Test Selection**

```yaml
# Path-based filtering
- uses: dorny/paths-filter@v2
  with:
    filters: |
      rust:
        - '**/*.rs'
        - '**/Cargo.toml'
      storage:
        - 'ferrisdb-storage/**'
      docs:
        - 'docs/**/*.md'
        - 'docs/**/*.mdx'
```

### 2. **Reduced Matrix Testing**

- Unit tests: 3 platforms × 1 Rust version = 3 jobs
- Full tests: 3 platforms × 2 Rust versions = 6 jobs (main branch only)

### 3. **Caching Strategy**

- Separate cache keys for different test types
- Workspace-specific caching for tutorials
- Build artifact reuse between test jobs

### 4. **Timeout Protection**

```yaml
timeout-minutes: 15 # Prevent runaway tests
```

### 5. **Parallel Execution**

- Unit tests run in parallel across platforms
- Integration tests run separately to avoid conflicts
- Property tests isolated to prevent resource contention
- Use `--test-threads=1` for concurrent tests to avoid flakiness

### 6. **Smart Security Auditing**

- Only runs when Cargo.toml or Cargo.lock changes
- Caches advisory database for faster runs
- Runs as separate job to not block tests

## Expected CI Times

### PR Workflow (Fast Path)

- **Quick checks**: 2-3 minutes
- **Unit tests**: 5-8 minutes (parallel)
- **Integration tests**: 8-12 minutes
- **Total**: ~15 minutes for typical PR

### Main Branch (Full Path)

- **All fast tests**: 15 minutes
- **Property tests**: +10 minutes
- **Full platform matrix**: +15 minutes
- **Total**: ~40 minutes for complete validation

### Benchmark Runs (On-Demand)

- **Performance validation**: +20 minutes
- **Only when explicitly requested**

## Maintenance Guidelines

### When to Add `test:` Labels

1. **`test:property`**:

   - Large data structure changes
   - New serialization formats
   - Critical path modifications

2. **`test:benchmarks`**:

   - Performance optimizations
   - Algorithm changes
   - New performance features

3. **`test:full`**:
   - Major refactoring
   - Cross-platform compatibility changes
   - Release preparation

### Monitoring CI Performance

Track these metrics:

- Average PR CI time (target: <20 minutes)
- Main branch CI time (target: <45 minutes)
- Test flakiness rate (target: <1%)
- Cache hit rates (target: >80%)

### Adjusting Test Scope

**If CI gets slower:**

1. Reduce PropTest cases further
2. Add more conditional triggers
3. Consider test parallelization improvements

**If test coverage drops:**

1. Ensure main branch gets full testing
2. Add nightly comprehensive test runs
3. Increase developer local testing requirements

## Developer Workflow

### Local Development

```bash
# Fast feedback loop
cargo test --lib

# Integration testing
cargo test --tests

# Full local validation (before pushing)
cargo test --all
cargo test --features=property-tests
```

### Pre-commit Recommendations

```bash
# Essential checks (fast)
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --lib --all

# Full validation (before important PRs)
cargo test --all --features=slow-tests
```

## Benefits of This Approach

1. **Faster feedback**: Most PRs get results in 15 minutes
2. **Comprehensive coverage**: Main branch gets full testing
3. **Resource efficiency**: Only run expensive tests when needed
4. **Developer productivity**: Local development remains fast
5. **Reliability**: Critical paths still get thorough testing

## Label-Based Test Triggers

### How Labels Control CI

Labels on PRs can trigger additional test suites:

```yaml
# In CI workflow
run_property_tests: |
  contains(github.event.pull_request.labels.*.name, 'test:property') ||
  github.ref == 'refs/heads/main'
```

### Available Test Labels

- `test:property` - Runs full property test suite
- `test:benchmarks` - Runs performance benchmarks
- `test:full` - Runs all tests on all platforms
- `area:ci` - Triggers extended CI validation

## Recent Optimizations (PR #108)

### 1. **Removed Duplicate Security Audit**

- Was running in both test and separate job
- Now only runs once when dependencies change

### 2. **Lightweight Docs Validation**

- Created `validate-docs.sh` script
- Skips full Astro build in CI
- Still validates markdown and structure

### 3. **PropTest Environment Awareness**

- Tests auto-detect CI environment
- Reduces data sizes in CI (1MB vs 10MB)
- Maintains thorough testing locally

## Measuring CI Performance

### Key Metrics

```bash
# Average CI time by workflow
gh run list --workflow=ci.yml --json durationMs,conclusion | jq ...

# Success rate
gh run list --limit 100 --json conclusion | jq ...
```

### Performance Targets

- PR CI (fast path): < 15 minutes
- PR CI (with integration): < 20 minutes
- Main branch CI: < 45 minutes
- Property tests: < 10 minutes additional

## Troubleshooting Slow CI

### Common Causes

1. **Cache Misses**

   - Check cache keys in workflow
   - Verify Cargo.lock changes

2. **Test Data Size**

   - Review PropTest strategies
   - Check for large test fixtures

3. **Flaky Tests**
   - Look for timing-dependent tests
   - Add retries for network operations

### Quick Fixes

```yaml
# Add timeout to prevent hanging
timeout-minutes: 30

# Cancel outdated runs
concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: true
```

## Related Documentation

- [Testing Standards](testing.md)
- [Performance Guidelines](../technical/performance.md)
- [GitHub Automation](github-automation.md)
- [Label System](labels.md)

---

_Last updated: 2025-06-01_
