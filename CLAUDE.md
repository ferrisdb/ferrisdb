# FerrisDB Development Guidelines - Quick Reference

Welcome! This is the quick reference for FerrisDB development. For detailed guidelines, see the [comprehensive documentation](guidelines/README.md).

⚠️ **Important**: This file is an INDEX for quick lookups. Do NOT add detailed content here - update the appropriate guideline file instead and link to it.

## 🚀 Quick Start

### Essential Reading

1. [Code Style](guidelines/development/code-style.md) - Formatting and conventions
2. [Git Workflow](guidelines/workflow/git-workflow.md) - Branching and commits
3. [PR Process](guidelines/workflow/pr-process.md) - How to submit changes

### For Different Roles

- **New Contributors**: Start with the essentials above
- **Content Writers**: See [Content Guidelines](guidelines/content/) for blogs, articles, and website
- **Core Developers**: Review [System Invariants](guidelines/technical/invariants.md)
- **Claude (me!) 🤖**: I follow all guidelines, especially [PR Review Process](guidelines/workflow/pr-process.md#claudes-pr-review-process)

## 📋 Quick Lookup: "When you ask me to..."

### Code & Development Tasks

- **"Write some code"** → [Code Style](guidelines/development/code-style.md) + [Idiomatic Rust](guidelines/development/idiomatic-rust.md)
- **"Add documentation"** → [Documentation Standards](guidelines/development/documentation.md)
- **"Fix formatting"** → [Markdown Quality](guidelines/development/markdown-quality.md)
- **"Run tests"** → [Testing Standards](guidelines/workflow/testing.md)
- **"Check performance"** → [Performance Guidelines](guidelines/technical/performance.md)

### Content & Writing Tasks

- **"Write a blog post"** → [Blogging Guidelines](guidelines/content/blogging.md)
- **"Write as Claude"** → [Claude's Blog Voice](guidelines/content/claude-blog-voice.md)
- **"Create a database concept"** → [Database Concepts Articles](guidelines/content/database-concepts-articles.md)
- **"Explain Rust concepts"** → [Rust by Example](guidelines/content/rust-by-example.md)
- **"Update the website"** → [Website Design](guidelines/content/website-design.md)
- **"Update FAQ.md"** → [FAQ Maintenance](guidelines/content/website-design.md#faq-maintenance)
- **"Update statistics"** → [Cached Statistics](guidelines/content/website-design.md#faq-maintenance) (avoids recomputing for same commit)

### Process & Workflow Tasks

- **"Review this PR"** → [PR Review Process](guidelines/workflow/pr-process.md#claudes-pr-review-process)
- **"Create a branch"** → [Git Workflow](guidelines/workflow/git-workflow.md)
- **"Submit changes"** → [PR Process](guidelines/workflow/pr-process.md)
- **"What commands to run?"** → [Common Commands](guidelines/workflow/commands.md)

### Architecture & Design Tasks

- **"Design a feature"** → [Architecture Decisions](guidelines/technical/architecture.md)
- **"Work on storage"** → [Storage Engine Guidelines](guidelines/technical/storage-engine.md)
- **"Consider security"** → [Security Practices](guidelines/technical/security.md)
- **"Check invariants"** → [System Invariants](guidelines/technical/invariants.md)

## 📁 Project Structure

```text
ferrisdb/
├── ferrisdb-core/       # Common types and traits
├── ferrisdb-storage/    # Storage engine
├── ferrisdb-client/     # Client library
├── ferrisdb-server/     # Server implementation
├── guidelines/          # All development guidelines
├── docs/
│   ├── database-concepts/ # Technical articles
│   ├── rust-by-example/ # Educational content
│   ├── _posts/          # All blog posts (human & Claude)
│   └── blog/            # Blog index pages
└── tests/               # Integration tests
```

## 🛠️ Most Used Commands

```bash
# Before committing
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
prettier --write "**/*.md"
markdownlint-cli2 "**/*.md"

# Create PR
git checkout -b feature/your-feature
# ... make changes ...
git push -u origin feature/your-feature
gh pr create
```

## 📝 Collaboration Commentary

**MANDATORY**: When working with Claude, **always include** detailed collaboration commentary in:

- **Commits**: See [Git Workflow - Claude's Collaboration Commentary](guidelines/workflow/git-workflow.md#claudes-collaboration-commentary) - **REQUIRED in every commit**
- **PR Descriptions**: See [PR Process - Collaboration Summary](guidelines/workflow/pr-process.md#pr-description-template) - **REQUIRED in every PR**
- **Squash Merges**: See [PR Process - Squash Merge Format](guidelines/workflow/pr-process.md#squash-merge-commit-message-format) - **REQUIRED when squash merging**

This tracks collaboration patterns for blog posts and research. **Never skip this - it's essential data.**

## 📚 Complete Guidelines Directory

### [Development Standards](guidelines/development/)

- [Code Style](guidelines/development/code-style.md) - Rust formatting rules
- [Idiomatic Rust](guidelines/development/idiomatic-rust.md) - Best practices
- [Documentation](guidelines/development/documentation.md) - Code doc standards
- [Markdown Quality](guidelines/development/markdown-quality.md) - MD formatting

### [Content Creation](guidelines/content/)

- [Website Design](guidelines/content/website-design.md) - Visual standards
- [Blogging](guidelines/content/blogging.md) - Blog post guidelines
- [Claude's Voice](guidelines/content/claude-blog-voice.md) - AI perspective
- [Database Concepts](guidelines/content/database-concepts-articles.md) - Technical articles
- [Rust by Example](guidelines/content/rust-by-example.md) - Educational content

### [Development Workflow](guidelines/workflow/)

- [Testing](guidelines/workflow/testing.md) - Test requirements
- [Commands](guidelines/workflow/commands.md) - Common commands
- [Git Workflow](guidelines/workflow/git-workflow.md) - Version control
- [PR Process](guidelines/workflow/pr-process.md) - Pull requests

### [Technical Architecture](guidelines/technical/)

- [Architecture](guidelines/technical/architecture.md) - Design decisions
- [Storage Engine](guidelines/technical/storage-engine.md) - Storage details
- [Performance](guidelines/technical/performance.md) - Optimization
- [Security](guidelines/technical/security.md) - Security practices
- [Invariants](guidelines/technical/invariants.md) - System properties

## 🤖 Claude's Maintenance Notes

### When Updating Guidelines

1. **Update the specific guideline file** in `guidelines/`
2. **Update this index ONLY** if adding new sections or changing structure
3. **Update cross-references** in related guideline files
4. **Test all links** to ensure they work
5. **NEVER add detailed content to CLAUDE.md** - it's an index, not a manual!

### My Quick Reminders

- ✅ All changes go through PRs (no exceptions!)
- ✅ Review PRs with 🤖 emoji signature
- ✅ Search web for best practices when reviewing
- ✅ Maintain my blog voice when writing posts
- ✅ Keep guidelines up to date

## 🔗 Important Links

- **Repository**: <https://github.com/ferrisdb/ferrisdb>
- **Documentation Site**: <https://ferrisdb.org/>
- **Guidelines Index**: [guidelines/README.md](guidelines/README.md)
- **Contributing**: [CONTRIBUTING.md](CONTRIBUTING.md)
- **Development Setup**: [DEVELOPMENT.md](DEVELOPMENT.md)

---

_Remember: When in doubt, check the [detailed guidelines](guidelines/README.md) or ask for clarification!_
