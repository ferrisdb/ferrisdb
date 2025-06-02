# FerrisDB Development Guidelines

Welcome to the FerrisDB development guidelines! These documents contain all the standards, processes, and best practices for contributing to FerrisDB.

## 📜 Governance

**IMPORTANT**: See [GOVERNANCE.md](GOVERNANCE.md) for how we maintain these guidelines. It serves as our constitution for information architecture and documentation standards.

## 🚨 Before You Commit

**MANDATORY for ALL contributors**:

### 1. Format ALL markdown/MDX changes:

```bash
prettier --write "**/*.md" "**/*.mdx"
```

### 2. Build Starlight if you touched docs/:

```bash
cd docs && npm run build
```

**Both are as critical as running `cargo fmt` for Rust code. CI will fail if skipped!**

## Quick Navigation

### 📝 Development Standards

- [Code Style](development/code-style.md) - Rust formatting and style conventions
- [Idiomatic Rust](development/idiomatic-rust.md) - Rust best practices and patterns
- [Documentation](development/documentation.md) - Code documentation standards
- [Visualization](development/visualization.md) - ASCII diagrams and table standards
- [Markdown Standards](development/markdown-standards.md) - Markdown formatting, linting, and tools
- [Utility Modules](development/utility-modules.md) - Creating shared utilities

### 🎨 Content Creation

- [Content Strategy](content/content-strategy.md) - **START HERE** - Our content philosophy and principles
- [Tutorial Guidelines](content/tutorials.md) - "Learn by Building" series structure
- [Blogging Guidelines](content/blogging.md) - Development journey documentation
- [Claude's Blog Voice](content/claude-blog-voice.md) - AI perspective and personality
- [Website Design - Starlight](content/website-design-starlight.md) - Astro Starlight design standards
- [Rust by Example](content/rust-by-example.md) - Educational article format
- [Database Concepts Articles](content/database-concepts-articles.md) - _Being phased out - merged into tutorials_

### 🔄 Development Workflow

- [Workflow README](workflow/) - Index of all workflow guidelines
- [Testing Standards](workflow/testing.md) - Test requirements and coverage
- [Common Commands](workflow/commands.md) - Frequently used commands
- [Git Workflow](workflow/git-workflow.md) - Branching and commit standards
- [PR Process](workflow/pr-process.md) - Pull request and review policies
- [Website Maintenance - Simplified](workflow/website-maintenance-simple.md) - ✅ **PRIMARY** - Daily content updates
- [Starlight Technical Reference](workflow/starlight-technical-reference.md) - 📖 Technical reference (not daily use)

### 🏗️ Technical Architecture

- [Architecture Decisions](technical/architecture.md) - Key design choices
- [Storage Engine](technical/storage-engine.md) - Storage implementation guidelines
- [Performance](technical/performance.md) - Optimization and benchmarking
- [Security](technical/security.md) - Security best practices
- [System Invariants](technical/invariants.md) - Critical system properties
- [File Format Standards](technical/file-formats.md) - Binary file format patterns and standards

## Getting Started

1. **New Contributors**: Start with [Code Style](development/code-style.md) and [Git Workflow](workflow/git-workflow.md)
2. **Content Writers**: Review [Blogging Guidelines](content/blogging.md) or relevant article formats
3. **Core Developers**: Familiarize yourself with [Architecture Decisions](technical/architecture.md) and [System Invariants](technical/invariants.md)

## Our Human-AI Workflow

FerrisDB demonstrates a specific collaboration pattern:

1. **Human assigns task**: Clear direction with context
2. **Claude implements**: Complete solution with tests
3. **Human reviews**: Questions, optimization ideas, edge cases
4. **Claude iterates**: Improvements based on feedback
5. **Together refine**: Until both are satisfied
6. **PR & merge**: Clear commits and documentation

This workflow is documented honestly in our blog posts, showing real conversations and actual development progress.

## Project Overview

FerrisDB is an educational project where a CRUD developer and an AI (Claude) are building a distributed database from scratch. The goal is not to create another production database, but to document the journey of learning database internals and Rust through building one.

**Important**: This is an educational project focused on learning. While we strive for correctness and good practices, the code is not intended for production use.

**What makes us unique**: We're the only project transparently documenting every step of human-AI collaboration in building complex systems software.

## Key Principles

1. **Journey Over Destination**: The process of building matters more than the final product
2. **Educational First**: Every decision should make learning easier
3. **Radical Transparency**: Document real progress, real mistakes, real learning
4. **Factual Content**: Clearly distinguish what's built vs. what's planned
5. **Story-Driven**: Technical content needs narrative context
6. **Quality Code**: Follow Rust best practices and idioms
7. **Real Collaboration**: Show actual human-AI workflow, not idealized version
8. **No Premature Documentation**: Don't document features that don't exist

## Guidelines as Source of Truth

**IMPORTANT**: These guidelines serve as the authoritative source of truth for all FerrisDB development:

- **Design Decisions**: All architectural choices, algorithms, and technical approaches are documented in the [Technical Architecture](technical/) section
- **Content Creation**: Any blog posts, articles, or documentation must align with the standards and decisions documented here
- **Implementation**: Code must follow the patterns and principles established in these guidelines
- **Evolution**: Guidelines can and should be updated when we discover better approaches, but changes must be deliberate and documented

When writing any content or code:

1. Always reference these guidelines first
2. Ensure consistency with documented decisions
3. Update guidelines if a better approach is discovered
4. Never contradict established patterns without updating the guidelines

This ensures consistency across the entire project and provides a single, reliable reference for all contributors.

## Quick Links

- [Main Repository](https://github.com/ferrisdb/ferrisdb)
- [Documentation Site](https://ferrisdb.org/)
- [Contributing Guide](../CONTRIBUTING.md)
- [Development Setup](../DEVELOPMENT.md)

---

_These guidelines are maintained collaboratively by humans and Claude 🤖_
