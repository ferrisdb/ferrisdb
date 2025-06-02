# Issue Management Guidelines

This document describes how to effectively use GitHub issues in FerrisDB, including templates, triage, and lifecycle management.

## Issue Templates

We provide structured templates to ensure issues contain necessary information:

### Available Templates

1. **🐛 Bug Report** - For reporting bugs

   - Auto-labels: `type:bug`, `status:ready`
   - Use when: Something isn't working as expected

2. **🚀 Feature Request** - For suggesting new features

   - Auto-labels: `type:feature`, `status:needs-design`
   - Use when: Proposing new functionality

3. **⚡ Performance Issue** - For performance problems

   - Auto-labels: `type:bug`, `performance`, `status:ready`
   - Use when: Experiencing slowness or resource issues

4. **📖 Documentation** - For documentation improvements

   - Auto-labels: `area:docs`, `type:feature`
   - Use when: Docs are missing, unclear, or incorrect

5. **📚 Tutorial Request** - For tutorial content

   - Auto-labels: `area:tutorials`, `educational`, `type:feature`
   - Use when: Requesting new learning content

6. **❓ Question** - For asking questions

   - Auto-labels: `question`
   - Use when: Need help (but check docs/discussions first)

7. **🔒 Security Vulnerability** - GitHub's built-in security reporting
   - Use when: Discovering security issues (keeps them private)

## Issue Creation Guidelines

### Before Creating an Issue

1. **Search existing issues** - Avoid duplicates
2. **Check documentation** - Answer might already exist
3. **Try GitHub Discussions** - For open-ended questions
4. **Use the right template** - Don't use blank issues

### Writing Good Issues

#### Bug Reports

- **One bug per issue** - Don't combine multiple bugs
- **Reproducible steps** - Clear, minimal reproduction
- **Environment details** - OS, Rust version, FerrisDB version
- **Expected vs actual** - Be specific about the difference

#### Feature Requests

- **Problem first** - Explain the problem before the solution
- **Use cases** - Provide real-world scenarios
- **Alternatives** - What have you considered?
- **Scope** - Keep requests focused and achievable

#### Questions

- **Be specific** - Vague questions get vague answers
- **Show effort** - What have you already tried?
- **Context** - Why do you need this information?

## Issue Triage Process

### Initial Triage (Maintainers)

1. **Verify labels** - Add missing required labels
2. **Assess priority** - For bugs, add priority label
3. **Check completeness** - Request missing information
4. **Remove duplicates** - Link to original issue
5. **Route appropriately** - Some questions → Discussions

### Priority Guidelines

**Critical** (`priority:critical`)

- Data loss or corruption
- Security vulnerabilities
- Complete feature breakdown
- Blocks multiple users

**High** (`priority:high`)

- Significant functionality impaired
- Affects many users
- No reasonable workaround

**Medium** (`priority:medium`)

- Default for most issues
- Has workaround available
- Affects some users

**Low** (`priority:low`)

- Nice to have
- Cosmetic issues
- Affects few users

### Area Assignment

Assign area labels based on affected components:

- Multiple areas are fine
- Helps route to right reviewers
- Enables filtered views

## Issue Lifecycle

### 1. **Creation** → `status:ready`

- Issue created with template
- Initial labels applied
- Waits for triage

### 2. **Triage** → Priority/Area assigned

- Maintainer reviews
- Adds missing labels
- May request clarification

### 3. **Design** → `status:needs-design` (if needed)

- Complex features need design
- Discussion in issue comments
- May link to design docs

### 4. **Development** → `status:in-progress`

- Someone assigns themselves
- Updates status label
- Links PR when ready

### 5. **Blocked** → `status:blocked` (if needed)

- Waiting on other work
- Document blockers in comment
- Link to blocking issues

### 6. **Resolution**

- **Fixed** - PR merged, issue closed
- **Won't Fix** - Not aligned with project
- **Duplicate** - Closed with link
- **Invalid** - Not an actual issue

## Issue Etiquette

### For Reporters

- **Be patient** - Maintainers are volunteers
- **Stay on topic** - Don't hijack issues
- **Be respectful** - Follow Code of Conduct
- **Provide updates** - If situation changes

### For Maintainers

- **Acknowledge quickly** - Even if can't fix immediately
- **Set expectations** - When will you look at it?
- **Ask for help** - Tag others if needed
- **Close respectfully** - Explain why if closing

## Stale Issue Management

Issues may be marked stale if:

- No activity for 60 days
- Waiting for reporter response
- Can't reproduce

Stale issues are closed after 14 more days unless updated.

## Integration with Development

### Linking PRs

- Use keywords: "Fixes #123", "Closes #123"
- Auto-closes issue when PR merges
- Shows connection in both PR and issue

### Milestones

- Group related issues
- Track release progress
- Show project roadmap

### Projects

- Kanban boards for visual tracking
- Automated status updates
- Cross-repository views

## Best Practices

1. **One issue, one concern** - Don't combine unrelated items
2. **Search before creating** - Reduce duplicates
3. **Use reactions** - 👍 instead of "+1" comments
4. **Update status** - Keep labels current
5. **Link related issues** - Build context
6. **Close completed issues** - Don't leave them hanging

## Common Patterns

### Investigation Issues

```
type:bug, investigation, area:wal
"We need to understand why X happens..."
```

### Refactoring Proposals

```
type:refactor, status:needs-design, area:storage
"The current implementation of Y could be improved..."
```

### Documentation Gaps

```
area:docs, type:feature, good first issue
"The docs don't explain how to..."
```

### Performance Improvements

```
type:feature, performance, area:sstable
"Optimize the compaction algorithm to..."
```

## Metrics and Reporting

Track issue metrics for project health:

- Time to first response
- Time to resolution by priority
- Issues created vs closed
- Label distribution
- Stale issue count

Use these metrics to improve processes and identify bottlenecks.

---
_Last updated: 2025-06-01_
