# Website Maintenance - Simplified

**Purpose**: Maintain accurate content that reflects our actual progress and journey.

## Core Principle

We maintain content about **what exists**, not what we plan to build.

## Regular Updates

### 1. Current Status Page (Weekly)

Update `/status/` with:

- What components are complete
- What we're actively working on
- What's planned next
- Honest limitations

```markdown
## Storage Engine Status

✅ **Complete**

- WAL with binary encoding
- MemTable (concurrent skip list)
- SSTable writer

🚧 **In Progress**

- SSTable reader
- Basic compaction

📋 **Planned**

- Block cache
- Bloom filters
```

### 2. Blog Posts (As We Build)

Document actual progress:

- Human posts: What I learned/reviewed today
- AI posts: Patterns I observed in our collaboration
- Both perspectives on same work

### 3. Tutorial Updates (When Code Changes)

If implementation changes:

1. Update tutorial code
2. Update ferrisdb-tutorials/
3. Run all tests
4. Update explanations

## What NOT to Maintain

We don't maintain documentation for:

- ❌ Installation guides (nothing to install)
- ❌ Configuration (nothing to configure)
- ❌ API references (no API yet)
- ❌ Performance benchmarks (incomplete system)
- ❌ Operational guides (not deployable)

## Quick Checklist

Before any update:

- [ ] Is this documenting something that exists?
- [ ] Is the status (built/building/planned) clear?
- [ ] Does it connect to our journey?
- [ ] Would a CRUD dev understand it?

## Statistics Updates

Keep these current:

- Lines of actual code (not stubs)
- Working tests
- Blog posts published
- Tutorials completed

```bash
# Get real stats
find . -name "*.rs" -not -path "./target/*" | xargs wc -l
cargo test --all 2>&1 | grep "test result"
ls docs/_posts/*.md | wc -l
```

## When to Update

- **Immediately**: Current Status when features complete
- **Daily**: Blog posts during active development
- **Weekly**: Statistics and progress summaries
- **As Needed**: Tutorials when implementation changes

## Related Guidelines

- [Content Strategy](../content/content-strategy.md) - What content we create
- [Information Architecture](../content/information-architecture.md) - How we organize it
- [Starlight Technical Reference](starlight-technical-reference.md) - Framework technical details

---

_Last updated: 2025-05-31_
