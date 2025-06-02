# GitHub Automation Guidelines

This document describes FerrisDB's GitHub automation setup, including branch protection, CI/CD workflows, and automated processes.

## Branch Protection Rules

### Main Branch Protection

The `main` branch has the following protections:

```yaml
protection_rules:
  main:
    required_reviews: 1
    dismiss_stale_reviews: true
    require_code_owner_reviews: true
    required_status_checks:
      - "CI / Format Check"
      - "CI / Rust Lint"
      - "CI / Test (ubuntu-latest)"
      - "CI / Test (macos-latest)"
      - "CI / Test (windows-latest)"
      - "CI / Security Audit"
    enforce_admins: false
    restrict_push_access: false
    allow_force_pushes: false
    allow_deletions: false
```

### Feature Branch Guidelines

- Create from `main`
- Name format: `feature/description`, `fix/description`, `chore/description`
- Delete after merge
- No direct commits to `main`

## CI/CD Workflows

### Primary CI Workflow (`.github/workflows/ci.yml`)

Runs on every PR and push to main:

#### 1. Format Check

```yaml
- Rust: cargo fmt --all --check
- Markdown: prettier --check "**/*.md" "**/*.mdx"
```

#### 2. Lint Check

```yaml
- Rust: cargo clippy --all-targets --all-features -- -D warnings
- Starlight docs validation (when docs change)
```

#### 3. Test Matrix

```yaml
os: [ubuntu-latest, macos-latest, windows-latest]
- cargo test --all
- cargo test --all --release (specific conditions)
```

#### 4. Security Audit

```yaml
- cargo audit
- Runs on dependency changes
```

### Specialized Workflows

#### Documentation CI

- Triggers on: `docs/**` changes
- Validates: Markdown format, broken links
- Builds: Starlight site
- Lightweight: Uses validation script

#### Benchmark CI

- Triggers on: Performance-related changes
- Runs: Criterion benchmarks
- Compares: Against base branch
- Reports: Performance regressions

## Automated Processes

### Dependabot Configuration

```yaml
version: 2
updates:
  - package-ecosystem: "cargo"
    directory: "/"
    schedule:
      interval: "weekly"
    reviewers:
      - "ferrisdb/maintainers"
    labels:
      - "type:chore"
      - "area:deps"
```

### Stale Issue Management

```yaml
days_until_stale: 60
days_until_close: 14
stale_label: "stale"
exempt_labels:
  - "pinned"
  - "security"
  - "priority:critical"
```

### Release Automation

1. **Version Tagging**

   - Format: `v0.1.0`
   - Triggers: Release workflow
   - Creates: GitHub release

2. **Release Notes**
   - Generated from: PR titles
   - Grouped by: Labels
   - Includes: Contributors

## GitHub Actions Best Practices

### Action Security

1. **Pin Actions to SHA**

   ```yaml
   uses: actions/checkout@v4  # Bad
   uses: actions/checkout@8e5e7e5ab8b370d6c329ec480221332ada57f0ab  # Good
   ```

2. **Minimal Permissions**

   ```yaml
   permissions:
     contents: read
     pull-requests: write
   ```

3. **No Secrets in Logs**
   - Use `::add-mask::`
   - Avoid echo with secrets

### Performance Optimization

1. **Cache Dependencies**

   ```yaml
   - uses: Swatinem/rust-cache@v2
     with:
       cache-on-failure: true
   ```

2. **Path Filtering**

   ```yaml
   - uses: dorny/paths-filter@v2
     id: changes
     with:
       filters: |
         rust:
           - '**/*.rs'
           - '**/Cargo.toml'
   ```

3. **Conditional Jobs**
   ```yaml
   if: needs.changes.outputs.rust == 'true'
   ```

### Workflow Organization

1. **Reusable Workflows**

   ```yaml
   uses: ./.github/workflows/reusable-rust-checks.yml
   ```

2. **Matrix Strategy**
   ```yaml
   strategy:
     fail-fast: false
     matrix:
       os: [ubuntu-latest, macos-latest]
   ```

## PR Automation

### Auto-merge Conditions

PRs can auto-merge when:

1. All required checks pass
2. Approved by maintainer
3. No changes requested
4. Not a draft
5. Has label: `ready-to-merge`

### PR Labels Automation

Automatic labeling based on:

- File paths changed
- PR title keywords
- PR size

Example configuration:

```yaml
area:docs:
  - docs/**
  - "*.md"

size/small:
  max_lines: 100

size/large:
  min_lines: 500
```

## Security Scanning

### Code Scanning

- CodeQL analysis on push
- Sarif results uploaded
- Security alerts created

### Dependency Scanning

- Dependabot security updates
- Cargo audit in CI
- License compliance checks

## Deployment Automation

### Documentation Site

- Trigger: Push to `main`
- Build: Starlight static site
- Deploy: GitHub Pages
- URL: https://ferrisdb.org

### Benchmark Dashboard

- Trigger: Push to `main`
- Run: Criterion benchmarks
- Store: GitHub Pages
- Track: Performance over time

## Monitoring and Alerts

### CI Failure Notifications

- Slack: `#ferrisdb-ci`
- Email: Committer
- GitHub: Check status

### Metrics Tracked

- CI duration trends
- Failure rates by job
- Flaky test detection
- Resource usage

## Cost Optimization

### GitHub Actions Minutes

- Use path filtering
- Cancel outdated runs
- Optimize test parallelism
- Cache aggressively

### Large Runners

- Only for release builds
- Benchmark comparisons
- Full integration tests

## Maintenance

### Regular Reviews

- Monthly: Workflow performance
- Quarterly: Security updates
- Yearly: Complete audit

### Documentation

- Keep workflows commented
- Document custom actions
- Explain complex conditions

## Troubleshooting

### Common Issues

1. **Cache Misses**

   - Check cache keys
   - Verify restore paths

2. **Flaky Tests**

   - Add retry logic
   - Increase timeouts
   - Isolate test state

3. **Permission Errors**
   - Check GITHUB_TOKEN
   - Verify write permissions

### Debug Techniques

1. **Enable Debug Logging**

   ```yaml
   env:
     ACTIONS_STEP_DEBUG: true
   ```

2. **SSH Debug Session**
   ```yaml
   - uses: mxschmitt/action-tmate@v3
     if: ${{ failure() }}
   ```

## Future Automation Plans

1. **Automated Benchmarking**

   - PR performance comparison
   - Regression detection
   - Historical tracking

2. **Documentation Preview**

   - PR preview deployments
   - Automatic screenshots
   - Link checking

3. **Release Automation**
   - Changelog generation
   - Version bumping
   - Crate publishing

---
_Last updated: 2025-06-01_
