# FerrisDB Development Guidelines

Welcome to the FerrisDB development guidelines! These documents contain all the standards, processes, and best practices for contributing to FerrisDB.

## 📜 Governance

**IMPORTANT**: See [GOVERNANCE.md](GOVERNANCE.md) for how we maintain these guidelines. It serves as our constitution for information architecture and documentation standards.

## Quick Navigation

### 📝 Development Standards

- [Code Style](development/code-style.md) - Rust formatting and style conventions
- [Idiomatic Rust](development/idiomatic-rust.md) - Rust best practices and patterns
- [Documentation](development/documentation.md) - Code documentation standards
- [Visualization](development/visualization.md) - ASCII diagrams and table standards
- [Markdown Standards](development/markdown-standards.md) - Markdown formatting, linting, and tools

### 🎨 Content Creation

- [Website Design](content/website-design.md) - Visual design and Jekyll configuration
- [Website Design - Starlight](content/website-design-starlight.md) - **NEW** Astro Starlight design standards
- [Blogging Guidelines](content/blogging.md) - General blog post standards
- [Claude's Blog Voice](content/claude-blog-voice.md) - AI perspective and personality
- [Database Concepts Articles](content/database-concepts-articles.md) - Technical article structure
- [Rust by Example](content/rust-by-example.md) - Educational article format

### 🔄 Development Workflow

- [Testing Standards](workflow/testing.md) - Test requirements and coverage
- [Common Commands](workflow/commands.md) - Frequently used commands
- [Git Workflow](workflow/git-workflow.md) - Branching and commit standards
- [PR Process](workflow/pr-process.md) - Pull request and review policies
- [Website Maintenance](workflow/website-maintenance.md) - Maintaining Jekyll docs site
- [Website Maintenance - Starlight](workflow/website-maintenance-starlight.md) - **NEW** Maintaining Astro Starlight site

### 🏗️ Technical Architecture

- [Architecture Decisions](technical/architecture.md) - Key design choices
- [Storage Engine](technical/storage-engine.md) - Storage implementation guidelines
- [Performance](technical/performance.md) - Optimization and benchmarking
- [Security](technical/security.md) - Security best practices
- [System Invariants](technical/invariants.md) - Critical system properties

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

FerrisDB is a distributed, transactional key-value database inspired by FoundationDB, implemented in Rust. This project serves as both a learning exercise and a demonstration of human-AI collaboration in software development.

## Key Principles

1. **Educational First**: Every decision should make learning easier
2. **Accuracy Always**: Verify technical details against codebase and git history
3. **Transparency**: Document real progress, real mistakes, real learning
4. **Quality Code**: Follow Rust best practices and idioms
5. **Real Collaboration**: Show actual human-AI workflow, not idealized version
6. **Documentation**: Keep everything well-documented and accessible
7. **Visual Clarity**: Use ASCII diagrams and tables to enhance understanding

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
