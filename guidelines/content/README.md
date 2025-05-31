# Content Guidelines

Guidelines for creating educational and engaging content for the FerrisDB project.

**Purpose**: Index of all content creation guidelines for blogs, articles, and website documentation.

## 🎯 Content Strategy

### [Content Strategy](content-strategy.md) **← START HERE**

Our overarching content philosophy and principles. **Read this first** to understand:

- Core value propositions
- Content philosophy (journey over destination)
- Factual vs. aspirational content
- What we do and don't create
- Quality criteria and success metrics

## Content Types

### [Tutorial Guidelines](tutorials.md) 🆕 🔥

**"Learn by Building" tutorial series** where CRUD developers build FerrisDB from scratch, learning Rust and database internals incrementally.

**NEW HIGH BAR**: Every tutorial must have:

- Complete working implementation in `ferrisdb-tutorials/`
- Step-by-step tests + concurrent tests
- Performance benchmarks
- Exercises with solutions
- MANDATORY dogfooding process

Features one component per tutorial, one concept per step, with extensive testing and language comparisons.

### [Blogging Guidelines](blogging.md)

Guidelines for writing engaging blog posts that document the FerrisDB development journey, including format requirements, statistics gathering, and engagement techniques.

### [Claude's Blog Voice](claude-blog-voice.md)

Specific guidelines for Claude's AI perspective blog posts, focusing on pattern recognition, collaboration analysis, and meta-observations about human-AI development.

### [Database Concepts Articles](database-concepts-articles.md) **[BEING PHASED OUT]**

**Note**: We are transitioning away from separate concept articles. Database concepts will be integrated into the "Learn by Building" tutorials where they can be taught through hands-on implementation. Existing articles will be merged into relevant tutorials.

### [Rust by Example](rust-by-example.md)

Educational articles teaching Rust concepts through real FerrisDB code, with comparisons to JavaScript, Python, Java, and Go to help CRUD developers learn Rust.

### [Information Architecture](information-architecture.md) 🆕

Structure and organization of FerrisDB documentation to support our journey-focused mission. Defines navigation, content categories, and URL structure.

### [Website Design](website-design-starlight.md)

Design guidelines for the FerrisDB documentation website using Astro Starlight, covering visual design, content structure, and user experience principles aligned with developer-focused design.

## Key Principles

### Educational First

Every piece of content should help readers learn and understand database systems better.

### Honest and Transparent

- Show real progress, including failures and learning moments
- Accurately represent human vs AI contributions
- Don't claim features or performance that doesn't exist

### Engaging and Accessible

- Write for CRUD developers, not just systems programmers
- Use relatable analogies and examples
- Create a "page-turner" experience with personality

### Technically Accurate

- Use actual FerrisDB code, not toy examples
- Verify all performance claims with real measurements
- Test all code examples before publishing

## Content Creation Workflow

1. **Choose appropriate content type** based on your goal
2. **Use the appropriate template from the templates/ directory** for consistency
3. **Write with the target audience in mind**
4. **Include real code and measurements**
5. **Format with prettier** (`prettier --write "**/*.md"`)
6. **Submit PR with appropriate label**

## Tutorial Tracking System

### Concept Tracking Files

For the "Learn by Building" tutorial series, we maintain strict tracking of what's been taught:

- **[RUST-CONCEPTS-TAUGHT.md](RUST-CONCEPTS-TAUGHT.md)** - Tracks all Rust concepts introduced/reinforced per tutorial
- **[DATABASE-CONCEPTS-TAUGHT.md](DATABASE-CONCEPTS-TAUGHT.md)** - Tracks database concepts with real-world examples
- **[LEARNING-PROGRESS.md](LEARNING-PROGRESS.md)** - Dashboard showing tutorial dependencies and progress

These files are the **source of truth** for ensuring we never assume untaught knowledge.

## Templates

All content templates have been migrated to Starlight format and are located in the `templates/` directory:

### Blog Post Templates

- [Human Blog Post Template](templates/blog-post-human.mdx) - For human perspective blog posts
- [Claude Blog Post Template](templates/blog-post-claude.mdx) - For AI perspective blog posts

### Article Templates

- [Database Concept Template](templates/database-concept.mdx) - For deep dive technical articles
- [Rust by Example Template](templates/rust-by-example.mdx) - For Rust learning articles

### Tutorial Template

- [Tutorial Template](templates/tutorial.mdx) - For "Learn by Building" series with step-by-step component construction

**Note**: These templates follow our new design philosophy:

- Simplicity and correctness first
- Developer-skeptical design with immediate code access
- Learning in public with honest limitations
- Show don't tell approach

## Related Sections

- [Development Standards](../development/) - Code and documentation standards
- [Development Workflow](../workflow/) - Publishing processes
- [Website Maintenance](../workflow/website-maintenance-simple.md) - Keeping content updated
