# FerrisDB Development Guidelines - Quick Reference

Welcome! This is the quick reference for FerrisDB development. For detailed guidelines, see the [comprehensive documentation](guidelines/README.md).

⚠️ **Important**: This file is an INDEX for quick lookups. Do NOT add detailed content here - update the appropriate guideline file instead and link to it.

## 🚨 CRITICAL RULE: NEVER PUSH TO MAIN BRANCH

> **⚠️ ABSOLUTE RULE FOR EVERYONE - NO EXCEPTIONS ⚠️**
>
> **NEVER push directly to the `main` branch!**
>
> - ❌ **FORBIDDEN**: Direct commits/pushes to main
> - ✅ **REQUIRED**: Always use feature branches + Pull Requests
> - 👍 **NO EXCEPTIONS**: Not even for typos or "quick fixes"
>
> See [Git Workflow](guidelines/workflow/git-workflow.md#critical-never-push-to-main-branch) and [PR Process](guidelines/workflow/pr-process.md#critical-all-changes-must-use-pull-requests) for details.

## 📌 Guidelines as Source of Truth

**CRITICAL**: The [guidelines directory](guidelines/) contains the authoritative source of truth for ALL design decisions, technical approaches, and content standards.

### 📜 How We Maintain Guidelines

See [GOVERNANCE.md](guidelines/GOVERNANCE.md) - Our constitution for maintaining information architecture. Key principles:

1. **Absolute Accuracy** - No lies, no fiction, no speculation without labels
2. **Living Source of Truth** - Follow faithfully, evolve thoughtfully
3. **Information Architecture First** - Structure for humans and AI efficiency
4. **Maintain the Architecture** - Every update preserves the structure

When creating any content or implementing any feature:

- Consult guidelines first for established patterns
- Never contradict without updating the source
- Follow the update cascade protocol in GOVERNANCE.md

## 🚀 Quick Start

### Essential Reading

1. [Code Style](guidelines/development/code-style.md) → [Idiomatic Rust](guidelines/development/idiomatic-rust.md) - Write good Rust
2. [Git Workflow](guidelines/workflow/git-workflow.md) → [PR Process](guidelines/workflow/pr-process.md) - Submit changes
3. [Testing](guidelines/workflow/testing.md) → [Commands](guidelines/workflow/commands.md) - Verify your work

### For Different Roles

#### 👨‍💻 New Contributors

Start with: Essential Reading → [Development README](guidelines/development/) → [DEVELOPMENT.md](DEVELOPMENT.md)

#### ✍️ Content Writers

Start with: [Content README](guidelines/content/) → Choose your content type:

- [Blog Posts](guidelines/content/blogging.md) + [Claude's Voice](guidelines/content/claude-blog-voice.md)
- [Database Concepts](guidelines/content/database-concepts-articles.md)
- [Rust by Example](guidelines/content/rust-by-example.md)

#### 🏗️ Core Developers

Review: [Technical README](guidelines/technical/) → [Architecture](guidelines/technical/architecture.md) → [Invariants](guidelines/technical/invariants.md)

#### 🎯 Content Strategists

Start with: [Content Strategy](guidelines/content/content-strategy.md) → [Information Architecture](guidelines/content/information-architecture.md)

#### 🌐 Website Maintainers

Follow: [Website Design](guidelines/content/website-design-starlight.md) → [Website Maintenance](guidelines/workflow/website-maintenance-simple.md)

#### 🤖 Claude (me!)

I follow ALL guidelines, especially:

- [PR Review Process](guidelines/workflow/pr-process.md#claudes-pr-review-process)
- [Collaboration Commentary](guidelines/workflow/git-workflow.md#claudes-collaboration-commentary)
- [My Blog Voice](guidelines/content/claude-blog-voice.md)

## 📋 Quick Lookup: "When you ask me to..."

### Code & Development Tasks

- **"Write some code"** → [Code Style](guidelines/development/code-style.md) + [Idiomatic Rust](guidelines/development/idiomatic-rust.md)
- **"Add documentation"** → [Documentation Standards](guidelines/development/documentation.md) + [Visualization](guidelines/development/visualization.md)
- **"Fix formatting"** → [Markdown Standards](guidelines/development/markdown-standards.md)
- **"Run tests"** → [Testing Standards](guidelines/workflow/testing.md)
- **"Check performance"** → [Performance Guidelines](guidelines/technical/performance.md)
- **"Design a storage file"** → [File Format Standards](guidelines/technical/file-formats.md)
- **"Implement a new component"** → [File Format Standards](guidelines/technical/file-formats.md) + [Architecture](guidelines/technical/architecture.md)

### Content & Writing Tasks

- **"Review our content strategy"** → [Content Strategy](guidelines/content/content-strategy.md) **START HERE**
- **"Write a blog post"** → [Blogging Guidelines](guidelines/content/blogging.md)
- **"Write as Claude"** → [Claude's Blog Voice](guidelines/content/claude-blog-voice.md)
- **"Create a tutorial"** → [Tutorial Guidelines](guidelines/content/tutorials.md) + **DOGFOOD IT!**
- **"Update the website"** → [Website Maintenance - Simplified](guidelines/workflow/website-maintenance-simple.md)
- **"Debug Starlight issues"** → [Starlight Technical Reference](guidelines/workflow/starlight-technical-reference.md)
- **"Check what's actually built"** → Update Current Status page (be honest!)
- **"Document our journey"** → Blog post showing real progress
- **"Organize content"** → [Information Architecture](guidelines/content/information-architecture.md)

### Process & Workflow Tasks

- **"Review this PR"** → [PR Review Process](guidelines/workflow/pr-process.md#claudes-pr-review-process)
- **"Create a branch"** → [Git Workflow](guidelines/workflow/git-workflow.md)
- **"Submit changes"** → [PR Process](guidelines/workflow/pr-process.md)
- **"What commands to run?"** → [Common Commands](guidelines/workflow/commands.md)
- **"Manage issues"** → [Issue Management](guidelines/workflow/issue-management.md) + [Label System](guidelines/workflow/labels.md)
- **"Set up automation"** → [GitHub Automation](guidelines/workflow/github-automation.md)
- **"Optimize CI"** → [CI Optimization](guidelines/workflow/ci-optimization.md)

### Architecture & Design Tasks

- **"Design a feature"** → [Architecture Decisions](guidelines/technical/architecture.md)
- **"Work on storage"** → [Storage Engine Guidelines](guidelines/technical/storage-engine.md) + [File Format Standards](guidelines/technical/file-formats.md)
- **"Consider security"** → [Security Practices](guidelines/technical/security.md)
- **"Check invariants"** → [System Invariants](guidelines/technical/invariants.md)
- **"Organize website content"** → [Information Architecture](guidelines/content/information-architecture.md)
- **"Implement WAL"** → [File Format Standards](guidelines/technical/file-formats.md#wal-file-format-specification) + [Architecture](guidelines/technical/architecture.md#persistence-write-ahead-log)
- **"Add metrics"** → [Architecture](guidelines/technical/architecture.md#metrics-architecture)
- **"Test WAL code"** → [Testing Standards](guidelines/workflow/testing.md#wal-specific-testing-patterns)

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
# 🚨 CRITICAL: Verify you're NOT on main branch
git branch --show-current  # Should NOT show 'main'

# Before committing (ALL MANDATORY)
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
prettier --write "**/*.md" "**/*.mdx"  # REQUIRED after ANY markdown changes

# If you modified docs/ (MANDATORY)
cd docs && npm run build  # REQUIRED to verify Starlight builds

# Create PR (NEVER push to main!)
git checkout -b feature/your-feature  # ALWAYS create a branch
# ... make changes ...
git push -u origin feature/your-feature  # Push to FEATURE branch
gh pr create  # Create PR for review
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
- [Visualization](guidelines/development/visualization.md) - Diagrams and tables
- [Markdown Standards](guidelines/development/markdown-standards.md) - Formatting and tools

### [Content Creation](guidelines/content/)

- [Tutorial Guidelines](guidelines/content/tutorials.md) - Learn by Building series
- [Website Design (Starlight)](guidelines/content/website-design-starlight.md) - Visual standards
- [Blogging](guidelines/content/blogging.md) - Blog post guidelines
- [Claude's Voice](guidelines/content/claude-blog-voice.md) - AI perspective
- [Database Concepts](guidelines/content/database-concepts-articles.md) - Technical articles
- [Rust by Example](guidelines/content/rust-by-example.md) - Educational content
- **Tutorial Tracking Files:**
  - [RUST-CONCEPTS-TAUGHT.md](guidelines/content/RUST-CONCEPTS-TAUGHT.md) - Rust concept tracker
  - [DATABASE-CONCEPTS-TAUGHT.md](guidelines/content/DATABASE-CONCEPTS-TAUGHT.md) - Database concept tracker
  - [LEARNING-PROGRESS.md](guidelines/content/LEARNING-PROGRESS.md) - Tutorial progress dashboard

### [Development Workflow](guidelines/workflow/)

- [Testing](guidelines/workflow/testing.md) - Test requirements
- [Commands](guidelines/workflow/commands.md) - Common commands
- [Git Workflow](guidelines/workflow/git-workflow.md) - Version control
- [PR Process](guidelines/workflow/pr-process.md) - Pull requests
- [Website Maintenance](guidelines/workflow/website-maintenance-simple.md) - Website updates

### [Technical Architecture](guidelines/technical/)

- [Architecture](guidelines/technical/architecture.md) - Design decisions
- [Storage Engine](guidelines/technical/storage-engine.md) - Storage details
- [File Format Standards](guidelines/technical/file-formats.md) - Binary format design
- [Performance](guidelines/technical/performance.md) - Optimization
- [Security](guidelines/technical/security.md) - Security practices
- [Invariants](guidelines/technical/invariants.md) - System properties

## 🔄 Common Workflows

### Starting a New Feature

1. [Architecture Decision](guidelines/technical/architecture.md) - Design first
2. [Code Style](guidelines/development/code-style.md) - Write code
3. [Testing](guidelines/workflow/testing.md) - Add tests
4. [Git Workflow](guidelines/workflow/git-workflow.md) - Commit with commentary
5. [PR Process](guidelines/workflow/pr-process.md) - Submit for review

### Creating a Tutorial (High Bar!)

1. Write MDX tutorial following [Tutorial Guidelines](guidelines/content/tutorials.md)
2. Create `ferrisdb-tutorials/tutorial-XX-name/` with full implementation
3. **DOGFOOD**: Follow your own tutorial step-by-step
4. Write tests for EVERY step + concurrent tests if applicable
5. Include benchmarks to prove performance claims
6. Add exercises with solutions
7. Update tracking files (RUST-CONCEPTS-TAUGHT.md, etc.)
8. CI must pass all tutorial tests

### Writing Content

1. Choose type: [Blog](guidelines/content/blogging.md) / [Database Concept](guidelines/content/database-concepts-articles.md) / [Rust Example](guidelines/content/rust-by-example.md)
2. [Visualization](guidelines/development/visualization.md) - Add diagrams if needed
3. [Markdown Standards](guidelines/development/markdown-standards.md) - Format properly
4. [Website Maintenance](guidelines/workflow/website-maintenance-simple.md) - Update stats/progress

### Daily Maintenance

1. [Website Maintenance](guidelines/workflow/website-maintenance-simple.md#daily-maintenance-tasks) - Update stats
2. [Commands](guidelines/workflow/commands.md#website-maintenance-commands) - Use cached functions
3. Check ROADMAP.md - Ensure progress accuracy

## 🛠️ Troubleshooting

### "Markdown won't format correctly"

→ See [Markdown Standards](guidelines/development/markdown-standards.md#common-issues-and-solutions)

### "Jekyll build is failing"

→ Check [Markdown Standards](guidelines/development/markdown-standards.md#jekyllkramdown-compatibility)

### "Where do I put this content?"

→ See guideline READMEs: [Development](guidelines/development/), [Content](guidelines/content/), [Technical](guidelines/technical/), [Workflow](guidelines/workflow/)

### "Statistics don't match across pages"

→ Use [cached statistics](guidelines/workflow/commands.md#statistics-and-metrics) function

### "How do I maintain the website?"

→ Follow [Website Maintenance](guidelines/workflow/website-maintenance-simple.md) workflow

## 🤖 Claude's Maintenance Notes

### When Updating Guidelines

**MUST FOLLOW [GOVERNANCE.md](guidelines/GOVERNANCE.md)**

1. **Update the specific guideline file** in `guidelines/` (single source principle)
2. **Update this index ONLY** if adding new sections or changing structure
3. **Follow update cascade protocol** - Update all cross-references
4. **Test all links** to ensure they work
5. **NEVER add detailed content to CLAUDE.md** - it's an index, not a manual!
6. **Check for redundancy** - Don't duplicate existing content
7. **Maintain information architecture** - Right content in right place

### My Quick Reminders

- 🚨 **NEVER PUSH TO MAIN BRANCH** (create feature branch + PR)
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
