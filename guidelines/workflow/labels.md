# Label System Guidelines

This document describes FerrisDB's GitHub label system for consistent issue tracking and workflow management.

## Label Categories

Our label system uses prefix-based categorization for clarity and easy filtering.

### Type Labels (Required)

Every issue MUST have exactly one type label:

- `type:bug` - Something isn't working
- `type:feature` - New feature or request
- `type:chore` - Maintenance tasks (deps, CI, etc.)
- `type:refactor` - Code refactoring without functional changes
- `type:security` - Security-related issues

### Priority Labels (Required for bugs)

Bug issues MUST have a priority label:

- `priority:critical` - Must be fixed ASAP, blocking issues
- `priority:high` - Important issues that should be addressed soon
- `priority:medium` - Normal priority issues
- `priority:low` - Nice to have, can be addressed later

### Area Labels (At least one required)

Indicates which component(s) are affected:

- `area:storage` - Storage engine related
- `area:wal` - Write-ahead log specific
- `area:memtable` - MemTable component
- `area:sstable` - SSTable component
- `area:client` - Client library
- `area:server` - Server implementation
- `area:ci` - CI/CD pipeline
- `area:docs` - Documentation
- `area:tutorials` - Tutorial content

### Status Labels (Optional)

Track workflow state:

- `status:blocked` - Blocked by another issue
- `status:ready` - Ready to be worked on
- `status:in-progress` - Currently being worked on
- `status:needs-design` - Needs design discussion

### Special Labels (Optional)

Additional categorization:

- `good first issue` - Good for newcomers to FerrisDB
- `help wanted` - Community contributions welcome
- `educational` - Good for learning database internals
- `deep-dive` - Requires understanding of complex concepts
- `investigation` - Requires investigation or research
- `performance` - Performance related issues
- `testing` - Related to testing infrastructure
- `blog` - Blog post related
- `rust-by-example` - Rust by Example articles
- `needs-review` - PR needs review from maintainers

## Label Usage Examples

### Bug Report

```
type:bug, priority:high, area:wal, status:ready
```

### New Feature

```
type:feature, area:storage, area:memtable, status:needs-design
```

### Performance Investigation

```
type:bug, priority:medium, area:wal, performance, investigation
```

### Good First Issue

```
type:feature, area:docs, good first issue, status:ready
```

## Label Lifecycle

### Issue Creation

1. Issue templates automatically apply initial labels
2. Triager adds missing required labels
3. Priority assigned based on impact

### During Development

1. Change `status:ready` → `status:in-progress` when work begins
2. Add `status:blocked` if blocked
3. Update areas if scope changes

### Resolution

1. Labels remain for historical tracking
2. Closed issues keep their labels
3. Can be used for metrics and reporting

## CI Integration

Some labels trigger specific CI behaviors:

- `area:ci` - Runs extended CI validation
- `performance` - May trigger benchmark comparisons
- `type:security` - May trigger security scans

## Label Management

### Adding New Labels

1. Follow the prefix convention (type:, area:, etc.)
2. Use consistent color coding:
   - Red tones for urgent/critical
   - Blue tones for areas
   - Green tones for ready/good states
   - Yellow tones for caution/blocked

### Removing Labels

1. Don't remove labels from closed issues
2. Archive unused labels rather than deleting
3. Update automation when removing labels

## Automation

GitHub Actions can use labels for:

- Auto-assigning reviewers based on area
- Triggering specific test suites
- Generating release notes by type
- Stale issue management

## Best Practices

1. **Be Specific**: Use multiple area labels if needed
2. **Update Promptly**: Keep status labels current
3. **Don't Over-Label**: Use only relevant labels
4. **Follow Templates**: Let templates set initial labels
5. **Document Changes**: Update this guide when adding new labels

---
_Last updated: 2025-06-01_
