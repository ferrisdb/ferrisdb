# CI Optimization Plan - Eliminating Redundancies

## Executive Summary

Our CI currently has several redundancies causing unnecessary compute usage and longer wait times. This plan eliminates duplications while maintaining comprehensive coverage.

## Identified Redundancies

### 1. **Duplicate Security Audits**

- **Current**: Security audit runs in BOTH `ci.yml` and `security.yml`
- **Impact**: 2x compute time for same check
- **Solution**: Remove from `ci.yml`, keep in `security.yml` with daily schedule

### 2. **Duplicate Documentation Builds**

- **Current**: Starlight docs built in BOTH `ci.yml` and `deploy-docs.yml`
- **Impact**: Redundant builds of same content (3-5 min each)
- **Solution**:
  - ✅ `ci.yml` now uses lightweight validation script
  - ✅ `deploy-docs.yml` handles full build and deployment
  - **Savings**: ~3-4 minutes per PR

### 3. **Redundant Workflow File**

- **Current**: Both `ci.yml` and `ci-optimized.yml` exist
- **Impact**: Confusion and maintenance burden
- **Solution**: Merge optimizations into `ci.yml`, delete `ci-optimized.yml`

### 4. **Overlapping Test Runs**

- **Current**: All tests run on every PR regardless of changes
- **Impact**: 40-60 minute CI times
- **Solution**:
  - ✅ Intelligent test selection based on changed files
  - ✅ Separate filters for `starlight` (docs site) vs `markdown` (all MD files)
  - ✅ Starlight only runs when docs/ actually changes, not for README updates

## Recommended Changes

### Step 1: Update `ci.yml`

```yaml
# Remove these duplicate jobs from ci.yml:
- security audit job (lines 289-303)
- full starlight build (make it validation only)

# Add intelligent test selection:
- Skip storage tests if only docs changed
- Skip tutorial tests if only core lib changed
- Reduce property test cases in CI
```

### Step 2: Update `security.yml`

```yaml
# This becomes the single source for security checks
# Already has daily schedule for comprehensive scanning
# Runs on PRs only when security-relevant files change
```

### Step 3: Optimize Test Execution

#### Fast Tests (Always Run) - 5 min

- Formatting checks
- Clippy linting
- Basic compilation
- Unit tests (lib only)

#### Conditional Tests - 10-15 min

- Integration tests (when storage changes)
- Tutorial tests (when tutorial code changes)
- Full cross-platform (only on main branch)

#### Slow Tests (Selective) - 20+ min

- Property tests (reduced cases in CI)
- Benchmarks (only on request)
- Security scans (in separate workflow)

### Step 4: Delete Redundant Files

```bash
# Remove the duplicate optimized workflow
rm .github/workflows/ci-optimized.yml
```

## Intelligent Path Filters

### New Filter Structure

```yaml
rust: # Any Rust code changes
starlight: # ONLY Starlight docs site changes
markdown: # ALL markdown files (for prettier check)
```

### Examples of Smart Filtering

| File Changed                      | `rust` | `starlight` | `markdown` | What Runs                   |
| --------------------------------- | ------ | ----------- | ---------- | --------------------------- |
| `README.md`                       | ❌     | ❌          | ✅         | Only markdown format check  |
| `guidelines/workflow/testing.md`  | ❌     | ❌          | ✅         | Only markdown format check  |
| `docs/src/content/docs/index.mdx` | ❌     | ✅          | ✅         | Starlight + markdown checks |
| `src/lib.rs`                      | ✅     | ❌          | ❌         | Rust tests only             |
| `docs/astro.config.mjs`           | ❌     | ✅          | ❌         | Starlight validation only   |

### Time Savings Examples

**PR changing only README.md:**

- Before: ~40 minutes (all tests)
- After: ~1 minute (markdown check only)

**PR changing guidelines docs:**

- Before: ~40 minutes (all tests + Starlight build)
- After: ~1 minute (markdown check only)

**PR changing actual docs site:**

- Before: ~40 minutes (everything)
- After: ~20 minutes (skip unrelated Rust tests if no Rust changes)

## Implementation Plan

### Phase 1: Remove Duplicates (Immediate)

1. Remove security job from `ci.yml`
2. Simplify starlight check in `ci.yml`
3. Delete `ci-optimized.yml`

### Phase 2: Add Intelligence (This PR)

1. Implement path-based job filtering
2. Add PR label triggers for extended tests
3. Configure PropTest for CI environment

### Phase 3: Monitor & Adjust (Post-merge)

1. Track average CI times
2. Adjust test selection thresholds
3. Add caching improvements

## Expected Results

### Before

- PR CI Time: 40-60 minutes
- Duplicate security scans
- Redundant doc builds
- All tests always run

### After

- PR CI Time: 15-20 minutes (typical)
- Single security scan source
- Efficient doc validation
- Smart test selection

## Configuration Examples

### PropTest CI Configuration

```toml
# ferrisdb-storage/proptest.toml
[proptest]
cases = 20           # Reduced from 256
max_shrink_iters = 50  # Reduced from 1024
```

### Environment-Aware Tests

```rust
fn max_test_size() -> usize {
    if std::env::var("CI").is_ok() {
        1024 * 1024     // 1MB in CI
    } else {
        10 * 1024 * 1024  // 10MB locally
    }
}
```

### Label-Based Extended Testing

Add PR labels:

- `test:full` - Run all tests on all platforms
- `test:benchmarks` - Include benchmark suite
- `test:property` - Full property test cases

## Maintenance Guidelines

1. **Monthly Review**: Check CI performance metrics
2. **Adjust Thresholds**: Based on actual run times
3. **Update Filters**: When project structure changes
4. **Document Changes**: Keep README.md current

## Benefits

1. **Faster Feedback**: 15-20 min typical PR time
2. **Resource Efficiency**: ~60% reduction in compute usage
3. **Developer Experience**: Quicker iteration cycles
4. **Cost Savings**: Reduced GitHub Actions minutes
5. **Maintainability**: Single source of truth for each check

## Next Steps

1. Review this plan
2. Implement Phase 1 changes
3. Test on a sample PR
4. Roll out to all PRs
5. Monitor and adjust

This optimization maintains our high quality standards while dramatically improving CI efficiency.
