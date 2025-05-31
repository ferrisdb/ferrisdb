# Content Templates

This directory contains all content creation templates for FerrisDB documentation, now migrated to Starlight MDX format.

## Available Templates

### Tutorial Template 🆕

- **[tutorial.mdx](tutorial.mdx)** - For the "Learn by Building" series where CRUD developers build FerrisDB components step-by-step
  - One component per tutorial
  - One Rust concept per step
  - Integrated testing throughout
  - Language comparisons (JS/Python/Java/Go)
  - Tracks concepts systematically

### Blog Post Templates

- **[blog-post-human.mdx](blog-post-human.mdx)** - For human perspective blog posts documenting the learning journey
- **[blog-post-claude.mdx](blog-post-claude.mdx)** - For AI perspective blog posts analyzing collaboration patterns

### Article Templates

- **[database-concept.mdx](database-concept.mdx)** - For comprehensive database concept deep dives
- **[rust-by-example.mdx](rust-by-example.mdx)** - For Rust learning articles with language comparisons

## Design Philosophy Alignment

All templates follow our developer-focused design principles:

- **Simplicity and Correctness First** - Clean implementations before optimization
- **Developer-Skeptical Design** - Immediate code access and verification paths
- **Learning in Public** - Honest limitations and real learning documentation
- **Show Don't Tell** - Working examples before theory

## Migration from Jekyll

These templates have been migrated from Jekyll to Starlight format:

- Jekyll frontmatter → Starlight blog/MDX frontmatter
- Liquid syntax → MDX component imports
- Custom styling → Starlight components (Card, Tabs, Aside, etc.)

## Usage

1. Copy the appropriate template for your content type
2. Replace all placeholder text with your content
3. Follow the structure and component usage patterns
4. Ensure all code examples are tested and working
5. Use proper file references for verification

## Component Reference

Common Starlight components used in templates:

```mdx
import { Card, CardGrid, Tabs, TabItem, Aside, Steps, Badge } from "@astrojs/starlight/components";

;
```

- **Card/CardGrid**: Feature boxes and comparison grids
- **Tabs/TabItem**: Multi-language code examples or perspectives
- **Aside**: Notes, warnings, and callouts
- **Steps**: Sequential instructions
- **Badge**: Status indicators (PLANNED, IN PROGRESS, etc.)

## Tutorial Template Special Features

The tutorial template includes:

### Metadata Tracking

```yaml
rust_concepts_introduced:
  - "Concept: Description"
rust_concepts_reinforced:
  - "Concept (from Tutorial X)"
database_concepts_introduced:
  - "Concept: Why it matters"
database_concepts_reinforced:
  - "Concept (from Tutorial Y)"
```

### Step Structure

- Clear goal for each step
- Code tabs for multiple perspectives
- Concept explanations with familiar analogies
- Immediate testing with celebration
- Progressive complexity building

### Integration Points

- Links to tracking files (RUST-CONCEPTS-TAUGHT.md, etc.)
- Prerequisites from previous tutorials
- Next tutorial preview
- Real FerrisDB code comparison

## Related Guidelines

- [Tutorial Guidelines](../tutorials.md) - Comprehensive tutorial creation guide
- [Blogging Guidelines](../blogging.md) - Blog writing standards
- [Database Concepts](../database-concepts-articles.md) - Technical article standards
- [Rust by Example](../rust-by-example.md) - Rust learning article standards
- [Website Design](../website-design-starlight.md) - Overall design principles
