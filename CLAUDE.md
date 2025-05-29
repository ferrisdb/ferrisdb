# FerrisDB Development Guidelines - Quick Reference

Welcome! This is the quick reference for FerrisDB development. For detailed guidelines, see the [comprehensive documentation](docs/guidelines/README.md).

## 🚀 Quick Start

### Essential Reading

1. [Code Style](docs/guidelines/development/code-style.md) - Formatting and conventions
2. [Git Workflow](docs/guidelines/workflow/git-workflow.md) - Branching and commits
3. [PR Process](docs/guidelines/workflow/pr-process.md) - How to submit changes

### For Different Roles

- **New Contributors**: Start with the essentials above
- **Content Writers**: See [Content Guidelines](docs/guidelines/content/) for blogs, articles, and website
- **Core Developers**: Review [System Invariants](docs/guidelines/technical/invariants.md)
- **Claude (me!) 🤖**: I follow all guidelines, especially [PR Review Process](docs/guidelines/workflow/pr-process.md#claudes-pr-review-process)

## 📋 Quick Lookup: "When you ask me to..."

### Code & Development Tasks

- **"Write some code"** → [Code Style](docs/guidelines/development/code-style.md) + [Idiomatic Rust](docs/guidelines/development/idiomatic-rust.md)
- **"Add documentation"** → [Documentation Standards](docs/guidelines/development/documentation.md)
- **"Fix formatting"** → [Markdown Quality](docs/guidelines/development/markdown-quality.md)
- **"Run tests"** → [Testing Standards](docs/guidelines/workflow/testing.md)
- **"Check performance"** → [Performance Guidelines](docs/guidelines/technical/performance.md)

### Content & Writing Tasks

- **"Write a blog post"** → [Blogging Guidelines](docs/guidelines/content/blogging.md)
- **"Write as Claude"** → [Claude's Blog Voice](docs/guidelines/content/claude-blog-voice.md)
- **"Create a deep dive"** → [Deep Dive Articles](docs/guidelines/content/deep-dive-articles.md)
- **"Explain Rust concepts"** → [Rust by Example](docs/guidelines/content/rust-by-example.md)
- **"Update the website"** → [Website Design](docs/guidelines/content/website-design.md)

### Process & Workflow Tasks

- **"Review this PR"** → [PR Review Process](docs/guidelines/workflow/pr-process.md#claudes-pr-review-process)
- **"Create a branch"** → [Git Workflow](docs/guidelines/workflow/git-workflow.md)
- **"Submit changes"** → [PR Process](docs/guidelines/workflow/pr-process.md)
- **"What commands to run?"** → [Common Commands](docs/guidelines/workflow/commands.md)

### Architecture & Design Tasks

- **"Design a feature"** → [Architecture Decisions](docs/guidelines/technical/architecture.md)
- **"Work on storage"** → [Storage Engine Guidelines](docs/guidelines/technical/storage-engine.md)
- **"Consider security"** → [Security Practices](docs/guidelines/technical/security.md)
- **"Check invariants"** → [System Invariants](docs/guidelines/technical/invariants.md)

## 📁 Project Structure

```text
ferrisdb/
├── ferrisdb-core/       # Common types and traits
├── ferrisdb-storage/    # Storage engine
├── ferrisdb-client/     # Client library
├── ferrisdb-server/     # Server implementation
├── docs/
│   ├── guidelines/      # All development guidelines
│   ├── deep-dive/       # Technical articles
│   ├── _posts/          # Human blog posts
│   └── _claude_blog/    # Claude's blog posts
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

## 📚 Complete Guidelines Directory

### [Development Standards](docs/guidelines/development/)

- [Code Style](docs/guidelines/development/code-style.md) - Rust formatting rules
- [Idiomatic Rust](docs/guidelines/development/idiomatic-rust.md) - Best practices
- [Documentation](docs/guidelines/development/documentation.md) - Code doc standards
- [Markdown Quality](docs/guidelines/development/markdown-quality.md) - MD formatting

### [Content Creation](docs/guidelines/content/)

- [Website Design](docs/guidelines/content/website-design.md) - Visual standards
- [Blogging](docs/guidelines/content/blogging.md) - Blog post guidelines
- [Claude's Voice](docs/guidelines/content/claude-blog-voice.md) - AI perspective
- [Deep Dives](docs/guidelines/content/deep-dive-articles.md) - Technical articles
- [Rust by Example](docs/guidelines/content/rust-by-example.md) - Educational content

### [Development Workflow](docs/guidelines/workflow/)

- [Testing](docs/guidelines/workflow/testing.md) - Test requirements
- [Commands](docs/guidelines/workflow/commands.md) - Common commands
- [Git Workflow](docs/guidelines/workflow/git-workflow.md) - Version control
- [PR Process](docs/guidelines/workflow/pr-process.md) - Pull requests

### [Technical Architecture](docs/guidelines/technical/)

- [Architecture](docs/guidelines/technical/architecture.md) - Design decisions
- [Storage Engine](docs/guidelines/technical/storage-engine.md) - Storage details
- [Performance](docs/guidelines/technical/performance.md) - Optimization
- [Security](docs/guidelines/technical/security.md) - Security practices
- [Invariants](docs/guidelines/technical/invariants.md) - System properties

## 🤖 Claude's Maintenance Notes

### When Updating Guidelines

1. **Update the specific guideline file** in `docs/guidelines/`
2. **Update this index** if adding new sections or changing structure
3. **Update cross-references** in related guideline files
4. **Test all links** to ensure they work

### My Quick Reminders

- ✅ All changes go through PRs (no exceptions!)
- ✅ Review PRs with 🤖 emoji signature
- ✅ Search web for best practices when reviewing
- ✅ Maintain my blog voice when writing posts
- ✅ Keep guidelines up to date

## 🔗 Important Links

- **Repository**: <https://github.com/ferrisdb/ferrisdb>
- **Documentation Site**: <https://ferrisdb.org/>
- **Guidelines Index**: [docs/guidelines/README.md](docs/guidelines/README.md)
- **Contributing**: [CONTRIBUTING.md](CONTRIBUTING.md)
- **Development Setup**: [DEVELOPMENT.md](DEVELOPMENT.md)

---

_Remember: When in doubt, check the [detailed guidelines](docs/guidelines/README.md) or ask for clarification!_
