# Guidelines Governance

The constitution for maintaining FerrisDB's information architecture and documentation standards.

## Core Principles (In Priority Order)

### 1. Absolute Accuracy

**No lies, no fiction, no speculation without labels.**

- Every claim must be verifiable against code or commits
- Unimplemented features must be marked [PLANNED], [CONCEPTUAL], or [FUTURE]
- Performance claims require actual measurements
- Historical accuracy matters - document what really happened
- **NEVER claim production readiness** - This is an educational project for learning

### 2. Guidelines as Living Source of Truth

**Follow guidelines faithfully, but evolve them thoughtfully.**

- Guidelines are authoritative for all development decisions
- However, they're not scripture - challenge them with experience
- When better approaches emerge, update guidelines first
- Document why changes were made for future understanding

### 3. Information Architecture First

**Structure for both human understanding and AI efficiency.**

- Every piece of information has ONE authoritative location
- All other references link back to the source
- No redundant content - use links instead
- Design for AI agents to find the right information quickly
- Think "work smart not work hard" for context management

### 4. Maintain the Architecture

**Every update must preserve the information structure.**

- Update content in the correct location only
- Maintain all indexes when adding/removing content
- Update all cross-references when moving content
- Never break the navigation paths

## Operational Principles

### 5. Single Source Principle

Each concept, process, or decision should have exactly ONE authoritative location:

```
BAD:  FAQ.md explains statistics collection
      website-design.md also explains statistics collection
      Both drift apart over time

GOOD: commands.md has the statistics function
      website-maintenance.md links to it
      FAQ.md links to website-maintenance.md
```

### 6. Context-Aware Structure

Design for AI retrieval efficiency:

- **Clear Headers**: Enable targeted section access
- **Self-Contained Sections**: Minimize required context
- **Explicit Prerequisites**: State dependencies upfront
- **Purpose-First**: Explain why in the first paragraph

### 7. Update Cascade Protocol

When making fundamental changes:

1. Update the authoritative source first
2. Find all references using grep/search
3. Update every cross-reference
4. Update parent indexes (README files)
5. Update CLAUDE.md if structure changed
6. Verify no orphaned content remains

### 8. Living Documentation

- **Regular Audits**: Check accuracy quarterly
- **Sunset Obsolete Content**: Mark as deprecated, then remove
- **Track Transitions**: Update [PLANNED] → implemented
- **Date Major Changes**: Include "Last updated: YYYY-MM-DD" for volatile content

### 9. Minimal Context Loading

Optimize for AI agents (LLMs):

- **Semantic Boundaries**: Break at logical points
- **Clear Relationships**: "Supersedes X", "Implements Y"
- **Avoid Circular Dependencies**: A→B→C, not A→B→A
- **Progressive Disclosure**: Overview → Details → Deep Dive

### 10. Change Documentation

Major guideline changes must document:

- **What Changed**: Specific modifications
- **Why It Changed**: Reasoning and evidence
- **Migration Path**: How to adapt existing work
- **Change Date**: For historical context

## Practical Rules

### For Adding New Guidelines

1. Check if content already exists elsewhere
2. Determine the ONE correct location
3. Add to appropriate directory with clear filename
4. Update parent README index
5. Add to CLAUDE.md if commonly needed
6. Add "Related Guidelines" section
7. Cross-reference from related guidelines

### For Updating Guidelines

1. Edit ONLY the authoritative location
2. Search for all references to update
3. Maintain backward compatibility notes
4. Document the change in commit message
5. Update "Last updated" if significant

### For Removing Guidelines

1. Mark as deprecated with date
2. Point to replacement (if any)
3. Keep for 1-2 months
4. Update all references before removal
5. Remove from all indexes
6. Document removal in commit

## Information Hierarchy

```
CLAUDE.md
├── Quick lookup by task
├── Role-based navigation
├── Common workflows
└── Troubleshooting

guidelines/README.md
├── Category overviews
├── Complete file index
└── Principles

Category READMEs
├── File list with purposes
├── Quick reference
└── Related categories

Individual Guidelines
├── Purpose statement
├── Main content
├── Related guidelines
└── Prerequisites (if any)
```

## AI-Friendly Patterns

### Good Pattern: Clear Section Purpose

```markdown
## Testing Standards

This section defines the minimum testing requirements for all FerrisDB code.

Prerequisites: Understanding of Rust's testing framework
Related: [Code Style](code-style.md), [PR Process](pr-process.md)
```

### Bad Pattern: Assumed Context

```markdown
## Testing

As mentioned above, we need tests. Here's how...
(What's "above"? AI agent might not have that context)
```

### Good Pattern: Single Source

```markdown
For statistics commands, see [Commands](commands.md#statistics)
```

### Bad Pattern: Duplicated Content

```markdown
Here's how to calculate statistics:
[entire script duplicated from another file]
```

## Enforcement

1. **PR Reviews**: Check governance compliance
2. **Automated Checks**: Lint for broken links
3. **Quarterly Audits**: Full accuracy review
4. **AI Feedback**: When I notice issues, I'll flag them

## Meta-Governance

This document itself follows these principles:

- It's the single source for governance rules
- It's designed for AI navigation
- It will evolve with experience
- It maintains absolute accuracy

---

_Last updated: 2025-06-01_
_Next review: When guidelines structure changes significantly_
