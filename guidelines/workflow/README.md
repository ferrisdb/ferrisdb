# Workflow Guidelines

Guidelines for development processes, collaboration patterns, and maintaining the FerrisDB project.

**Purpose**: Index of all workflow guidelines for consistent development practices.

## Workflow Categories

### [Testing Standards](testing.md)

Comprehensive testing requirements for all FerrisDB code, including unit tests, integration tests, and documentation tests. Minimum 80% coverage with focus on edge cases.

### [Common Commands](commands.md)

Frequently used commands for development, testing, and maintenance. Includes cached statistics functions and development shortcuts.

### [Git Workflow](git-workflow.md)

Version control standards including branch naming, commit messages, and **mandatory** collaboration commentary for human-AI development tracking.

### [PR Process](pr-process.md)

Pull request creation, review, and merge procedures. Includes Claude's specific PR review process with 🤖 emoji signatures.

### [Website Maintenance - Simplified](website-maintenance-simple.md) ✅ **[PRIMARY]**

Focused approach to maintaining documentation that reflects actual progress. Use this for daily updates: Current Status, blog posts, real progress tracking.

### [Starlight Technical Reference](starlight-technical-reference.md) 📖 **[REFERENCE]**

Technical reference for Astro Starlight framework. Use this when you need MDX component details, build commands, or troubleshooting help. Not for daily maintenance.

## Key Principles

### Development First, Documentation Second

- Build features before documenting them
- Update docs to reflect reality, not plans
- Remove speculative documentation

### Transparent Collaboration

- Every commit includes collaboration commentary
- PR descriptions detail human-AI interaction
- Blog posts show real development process

### Quality Through Testing

- Comprehensive test coverage required
- Tests document expected behavior
- Broken tests block merges

## Quick Reference

### Daily Tasks

1. Run tests before committing
2. Update Current Status when features complete
3. Write collaboration commentary in commits

### Per Feature

1. Write tests first (TDD encouraged)
2. Implement with documentation
3. Update relevant docs (if any exist)
4. Submit PR with full context

### Weekly

1. Review and update statistics
2. Check for outdated documentation
3. Write blog posts about progress

## Related Sections

- [Development Standards](../development/) - Code quality guidelines
- [Content Creation](../content/) - Documentation and blog guidelines
- [Technical Architecture](../technical/) - System design guidelines

---

_Last updated: 2025-05-31_
