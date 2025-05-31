# Website Design Guidelines - Astro Starlight

Design guidelines for the FerrisDB documentation website using Astro Starlight, focusing on educational value and user engagement.

## Migration Status

**Current State**: Migrated from Jekyll (Just the Docs) to Astro Starlight
**Location**: `/docs/` directory
**Framework**: Astro with Starlight theme and starlight-blog plugin

## Design Philosophy

- **Simplicity and Correctness First**: Clean, understandable implementations that teach concepts clearly. Optimize only after understanding trade-offs.

- **Developer-Skeptical Design**: Assume readers want proof, not persuasion. Provide immediate access to real code, honest limitations, and working examples.

- **Learning in Public**: Document real mistakes, breakthrough moments, and iteration cycles. Show authentic development process, not polished marketing.

- **Collaboration Transparency**: Analyze and share what actually works in human-AI partnerships. Focus on patterns and effectiveness, not hype.

- **Engineering Wisdom Over Trends**: Apply classic software engineering principles. Favor proven approaches over novel complexity.

## Developer-Focused Appeal Factors

### Core Design Principles

- **"Show Don't Tell"**: Real code must be visible within first screen interaction
- **"Honest Limitations"**: State what doesn't work before highlighting what does
- **"Engineering Wisdom"**: Classic patterns (simple first, optimize later) over trendy approaches
- **"Code Access Paths"**: Multiple direct routes to actual implementation
- **"Technical Depth on Demand"**: Easy navigation from overview to deep implementation details

### Content Strategy for Technical Audiences

- **Lead with Implementation**: Real code examples, not pseudocode or theory
- **Problem → Simple Solution → Optimization**: Show progression of understanding
- **Trade-offs Explicit**: Discuss why we chose X over Y with honest pros/cons
- **Verification Enabled**: Include file paths and links to actual implementation
- **No Production Claims**: Clear positioning as educational/learning project

### Navigation Priorities for Developers

**Primary Actions (Most Prominent)**:

- View GitHub repository
- Try hands-on tutorial
- See architecture documentation
- Read real implementation details

**Secondary Actions**:

- Follow development blog
- Browse database concepts
- Check current progress/status

**Tertiary Actions**:

- About the project
- FAQ and contributing
- Community links

### Developer Engagement Metrics

**Technical Engagement Indicators**:

- Time to GitHub repo access from homepage (target: <30 seconds)
- Tutorial completion rates (measures hands-on appeal)
- Code-focused navigation patterns
- Technical question quality in GitHub issues
- Developer retention through implementation phases

**Content Effectiveness Metrics**:

- GitHub stars/forks vs. site visits ratio
- Blog post engagement showing learning insights
- Architecture documentation page views
- Real code example interaction rates

## Visual Design

### Starlight Components

**IMPORTANT**: Use Starlight's built-in components instead of custom HTML/CSS.

Available components:

- `Card` and `CardGrid` - For feature boxes and grids
- `Tabs` and `TabItem` - For tabbed content
- `Steps` - For sequential instructions
- `Aside` - For callouts and notes (replaces Jekyll callouts)
- `Badge` - For status indicators
- `Icon` - For inline icons

Example usage:

````mdx
import { Card, CardGrid, Tabs, TabItem, Steps, Aside, Badge } from "@astrojs/starlight/components";

<Aside type="note">This replaces Jekyll callouts</Aside>

<Steps>1. First step 2. Second step 3. Third step</Steps>

<Tabs>
  <TabItem label="Rust">```rust // Rust code ```</TabItem>
  <TabItem label="CLI">```bash cargo run ```</TabItem>
</Tabs>
````

### Color Palette

Defined in `src/styles/custom.css`:

```css
:root {
  /* FerrisDB Brand Colors */
  --sl-color-accent: #ff6b35;
  --sl-color-accent-low: #ff8a5b;
  --sl-color-accent-high: #e55a2b;
}
```

Additional semantic colors available:

- `--sl-color-blue` - For human blog posts
- `--sl-color-purple` - For Claude blog posts
- `--sl-color-green` - For success states
- `--sl-color-orange` - For warnings

### Typography

Starlight provides built-in typography scales:

- Headings automatically styled with proper hierarchy
- Body text optimized for readability
- Code blocks with syntax highlighting
- No need for custom font utilities

### Diagram Guidelines

**CRITICAL**: Website content uses Mermaid diagrams for better visual presentation, but ALL Mermaid diagrams MUST be based on corresponding ASCII diagrams in the technical guidelines.

1. **Mermaid in MDX Files**

   ````mdx
   ```mermaid
   graph TD
       A[Client] --> B[Server]
       B --> C[Storage Engine]
       C --> D[MemTable]
       C --> E[SSTables]
       C --> F[WAL]
   ```
   ````

   ```

   ```

2. **ASCII as Source of Truth**
   - Every Mermaid diagram must have an ASCII counterpart in technical guidelines
   - ASCII diagrams in `/guidelines/technical/` are authoritative
   - When updating diagrams, update ASCII first, then Mermaid

### Implementation Status in Content

**MANDATORY**: When creating any content that discusses features not yet implemented:

1. **Use Badge Components**

   ```mdx
   <Badge text="PLANNED" variant="caution" />
   <Badge text="IN PROGRESS" variant="note" />
   <Badge text="COMPLETED" variant="success" />
   ```

2. **Use Aside for Status**
   ```mdx
   <Aside type="caution" title="Planned Feature">
     This section describes planned functionality not yet implemented.
   </Aside>
   ```

## Content Structure

### File Organization

```
docs/src/
├── content/
│   ├── docs/           # Main documentation
│   │   ├── index.mdx   # Homepage
│   │   ├── getting-started.mdx
│   │   ├── guides/     # How-to guides
│   │   ├── concepts/   # Educational content
│   │   │   ├── database-internals/
│   │   │   └── rust-patterns/
│   │   ├── reference/  # Technical reference
│   │   └── project/    # About the project
│   └── blog/           # Blog posts
├── pages/              # Custom pages
│   └── blog/
│       ├── overview.astro    # Main blog page
│       ├── human.astro       # Human perspective
│       └── claude.astro      # Claude perspective
└── styles/
    └── custom.css      # Brand colors only
```

### Navigation Structure

Configured in `astro.config.mjs`:

```js
sidebar: [
  {
    label: "Start Here",
    items: [
      { label: "Introduction", slug: "index" },
      { label: "Getting Started", slug: "getting-started" },
    ],
  },
  {
    label: "Blog",
    items: [
      { label: "All Posts", link: "/blog" },
      { label: "Human Perspective", link: "/blog/human" },
      { label: "Claude Perspective", link: "/blog/claude" },
    ],
  },
  // ... other sections
];
```

## Blog System

### Blog Post Format

```yaml
---
title: "Day 1: Building from Scratch"
date: 2025-01-27
authors: [human] # or [claude]
tags: [development, rust, databases]
description: Brief description for SEO
excerpt: Short excerpt for blog listings
---
```

### Blog File Naming

Use SEO-friendly file names with author and title-derived slugs:

- Pattern: `day-N-author-seo-friendly-slug.md`
- Human example: `day-1-human-from-just-use-rocksdb-to-building-from-scratch.md`
- Claude example: `day-1-claude-how-i-learned-humans-say-build-but-mean-teach.md`

This generates URLs like:

- `/blog/day-1-human-from-just-use-rocksdb-to-building-from-scratch/`
- `/blog/day-1-claude-how-i-learned-humans-say-build-but-mean-teach/`

Benefits:

- Author is clear from the URL
- SEO-friendly with descriptive keywords
- Easy to identify perspective without clicking
- Maintains chronological ordering with day number

### Custom Blog Pages

Three custom blog views created in `src/pages/blog/`:

1. `overview.astro` - Combined view of all posts
2. `human.astro` - Human perspective only
3. `claude.astro` - Claude perspective only

Each uses:

- Author filtering
- Color-coded post borders (blue for human, purple for Claude)
- Consistent metadata display
- Proper CTAs ("Read the adventure" vs "Explore patterns")

## Component Patterns

### Cards with Starlight

```mdx
<CardGrid>
  <Card title="Feature Name" icon="rocket">
    Feature description with **markdown** support.
  </Card>
  <Card title="Another Feature" icon="setting">
    More content here.
  </Card>
</CardGrid>
```

### Progress Indicators

```mdx
<Badge text="COMPLETED" variant="success" /> Phase 1: Basic Functionality - [x] Simple WAL with
binary encoding - [x] Concurrent skip list MemTable

<Badge text="IN PROGRESS" variant="caution" /> Phase 2: Performance - [ ] Complete SSTable
implementation - [ ] Bloom filters
```

### Code Examples

````mdx
```rust title="ferrisdb-storage/src/memtable/mod.rs"
pub struct MemTable {
    skiplist: Arc<SkipList>,
    memory_usage: AtomicUsize,
    max_size: usize,
}
```
````

## Writing for Starlight

### MDX Features

- Import and use React/Astro components
- Inline JSX within markdown
- Dynamic content generation
- Better syntax highlighting with titles

### Frontmatter Options

```yaml
---
title: Page Title
description: SEO description
sidebar:
  order: 1 # Manual ordering
  badge: New # Optional badge
  hidden: false # Hide from sidebar
---
```

### Image Handling

Images go in `src/assets/` and are imported:

```mdx
import ferrisdbLogo from "../../assets/ferrisdb_logo.svg";

<img src={ferrisdbLogo.src} alt="FerrisDB Logo" />
```

## Astro/Starlight Configuration

### Key Configuration Files

1. `astro.config.mjs` - Main configuration
2. `src/styles/custom.css` - Brand colors only
3. `package.json` - Dependencies

### Essential Features

- **Search**: Built-in with Pagefind
- **Dark Mode**: Automatic with theme toggle
- **Mobile Navigation**: Responsive by default
- **Syntax Highlighting**: Shiki with multiple themes
- **SEO**: Automatic meta tags and sitemap

## Content Templates

All content templates have been migrated to Starlight MDX format and are located in:

- `/guidelines/content/templates/`

Available templates:

- **blog-post-human.mdx** - Human perspective blog posts
- **blog-post-claude.mdx** - AI perspective blog posts
- **database-concept.mdx** - Deep dive technical articles
- **rust-by-example.mdx** - Rust learning articles

These templates incorporate:

- Developer-focused design principles
- Starlight component usage
- Proper frontmatter format
- MDX imports and features

## Content Migration from Jekyll

### Conversion Patterns

| Jekyll               | Starlight                   |
| -------------------- | --------------------------- |
| `{: .fs-6 .fw-300 }` | Remove (use default styles) |
| `{: .no_toc }`       | Remove (not needed)         |
| Liquid loops         | Astro components with JS    |
| `callout` blocks     | `<Aside>` component         |
| Custom HTML          | Starlight components        |

### Blog Migration

- Convert Jekyll frontmatter to starlight-blog format
- Update internal links to new paths
- Replace custom HTML with components
- Ensure proper author attribution

## ROADMAP.md as Source of Truth

The `ROADMAP.md` file remains the SINGLE source of truth for ALL project progress displayed on the website.

### Mapping ROADMAP.md to Website Content

**Exact mapping rules**:

1. ✅ Items marked `[x]` → "Available Now" / "What's Working" / "Core Concepts (Available Now)"
2. 🚧 Items marked `[ ]` in current focus areas → "Currently Building" / "In Progress"
3. ⏳ Items in later sections → "Coming Soon" / "Coming Next"

### Language Guidelines for Progress Updates

**For learner-focused pages (e.g., Getting Started)**:

- Use "why" and "how" framing
- Explain the purpose of each feature
- Make it accessible to CRUD developers
- Transform technical terms:
  - "Write-Ahead Log (WAL)" → "Why databases need Write-Ahead Logs (WAL) for crash recovery"
  - "MemTable (Skip List)" → "How databases store data in memory with Skip Lists"
  - "SSTable format" → "The SSTable format - how databases organize data on disk"
  - "Compaction" → "Compaction - how databases merge files efficiently"
  - "Bloom filters" → "Bloom filters - probabilistic data structures for speed"

**For technically accurate pages (e.g., Homepage, Architecture)**:

- Use precise technical terms
- Include implementation details
- Mention specific algorithms or data structures
- Examples:
  - "MemTable with concurrent Skip List" not just "in-memory storage"
  - "SSTable reader with binary search optimization"
  - "Clean API design after Day 2 refactoring"

### Displaying Progress in Starlight

Use components for better visual presentation:

```mdx
<Tabs>
  <TabItem label="✅ Completed">
    - Write-Ahead Log (WAL) - MemTable with Skip List - Basic SSTable structure
  </TabItem>
  <TabItem label="🚧 In Progress">- SSTable reader implementation - Compaction strategy</TabItem>
  <TabItem label="⏳ Planned">- Bloom filters - Block cache - Transactions</TabItem>
</Tabs>
```

## Regular Maintenance Tasks

### Daily Homepage Checklist

**Update these specific sections in `src/content/docs/index.mdx`**:

- [ ] **Development Progress section**: Update Tabs component with current ROADMAP.md status
- [ ] **Collaboration Metrics**: Track and update:
  - Pattern recognitions: Count Claude's insights in recent blog posts
  - Human intuition saves: Times human review prevented issues
  - Collaboration score: Subjective assessment (1-10) of recent work quality
  - Tests passing: `grep -r "#\[test\]" --include="*.rs" . | wc -l`
- [ ] **Educational Resources**: Verify all CardGrid links work
- [ ] **No production claims**: Keep messaging as educational/experimental
- [ ] **CTA buttons**: Test all action links

### Statistics Component

Create a reusable component for statistics:

```astro
---
// src/components/Stats.astro
const stats = await getProjectStats(); // Implement based on commands
---

<div class="stats">
  Day {stats.days} • {stats.lines} lines • {stats.tests} tests
</div>
```

### Google Analytics

Configure in `astro.config.mjs`:

```js
export default defineConfig({
  integrations: [
    starlight({
      // ... other config
      head: [
        {
          tag: "script",
          attrs: {
            src: "https://www.googletagmanager.com/gtag/js?id=G-JPW5LY247F",
            async: true,
          },
        },
        {
          tag: "script",
          content: `
            window.dataLayer = window.dataLayer || [];
            function gtag(){dataLayer.push(arguments);}
            gtag('js', new Date());
            gtag('config', 'G-JPW5LY247F', {
              anonymize_ip: true // GDPR compliance
            });
          `,
        },
      ],
    }),
  ],
});
```

## When New Features Are Ready

### When Binary/REPL is Available

Update "Try It Now" sections across the site:

**Homepage** - Replace placeholder with actual REPL example:

````mdx
<Tabs>
  <TabItem label="Docker">
    ```bash docker run -it ferrisdb/playground ferris> set key "value" OK ferris> get key "value"
    ```
  </TabItem>
  <TabItem label="Local">
    ```bash cargo install ferrisdb ferrisdb-repl ferris> set key "value" OK ```
  </TabItem>
</Tabs>
````

### When Examples are Added

Update with `cargo run --example`:

````mdx
<Card title="Run Examples" icon="rocket">
  ```bash
  # Basic operations
  cargo run --example basic_operations
  
  # Concurrent access
  cargo run --example concurrent_writes
````

</Card>
```

### When Server is Ready

Add client connection examples:

````mdx
<Tabs>
  <TabItem label="Rust">
    ```rust let client = FerrisClient::connect("localhost:50051").await?; client.set("key",
    "value").await?; ```
  </TabItem>
  <TabItem label="gRPC">```bash grpcurl -plaintext localhost:50051 ferrisdb.v1.KV/Set ```</TabItem>
</Tabs>
````

## Blog Post Maintenance

### Title Requirements

- Use compelling hooks that promise learning value
- Follow pattern: "Day N: [Hook] - [Learning Outcome]"
- Examples:
  - ✅ "Day 1: How I Learned Humans Say 'Build' But Mean 'Teach'"
  - ❌ "Day 1: Building FerrisDB"

### Companion Post Verification

Each day should have both perspectives with author-prefixed SEO-friendly names:

- Human: `/src/content/blog/day-N-human-seo-title.md`
- Claude: `/src/content/blog/day-N-claude-seo-title.md`

Example for Day 4:

- Human: `day-4-human-why-compaction-matters-for-performance.md`
- Claude: `day-4-claude-patterns-in-human-performance-questions.md`

Cross-reference in each post:

```mdx
> 🤖 **Claude's perspective**: [Day N: Title](/blog/day-N-claude-seo-slug/)
> 👤 **Human's perspective**: [Day N: Title](/blog/day-N-human-seo-slug/)
```

### Code Example Verification

All code blocks must:

1. Match actual implementation in the codebase
2. Include file paths when referencing real code
3. Be tested before publishing

## Deployment

### GitHub Actions Workflow

Located in `.github/workflows/deploy-docs.yml`:

```yaml
name: Deploy Docs
on:
  push:
    branches: [main]
    paths:
      - "docs/**"
      - ".github/workflows/deploy-docs.yml"
```

### Build Commands

```bash
# Development
cd docs
npm install
npm run dev

# Production build
npm run build

# Preview production build
npm run preview
```

## Quality Checklist

- [ ] Uses Starlight components (no custom HTML)
- [ ] MDX files properly formatted
- [ ] All imports working
- [ ] Navigation properly configured
- [ ] Blog posts have correct frontmatter
- [ ] Custom blog pages rendering correctly
- [ ] Mobile responsive (automatic with Starlight)
- [ ] Search working
- [ ] Dark mode functional
- [ ] Build passes without errors
- [ ] All progress matches ROADMAP.md
- [ ] Links are correct for new structure

## Benefits Over Jekyll

1. **Modern Stack**: Faster builds, better DX
2. **Component-Based**: Reusable, maintainable
3. **MDX Support**: Rich interactive content
4. **Better Performance**: Static site with optimizations
5. **Type Safety**: TypeScript support
6. **Active Development**: Regular updates

## Migration Notes

- Old Jekyll site remains in `/docs/` as backup
- New Starlight site in `/docs/`
- Can switch GitHub Pages source when ready
- All content migrated and enhanced
- Blog system improved with custom views

## Related Guidelines

- **Content Types**: [Blogging](blogging.md), [Database Concepts](database-concepts-articles.md), [Rust by Example](rust-by-example.md)
- **Maintenance**: [Website Maintenance](../workflow/website-maintenance.md)
- **Commands**: [Common Commands](../workflow/commands.md#website-maintenance-commands)
- **Markdown**: [Markdown Standards](../development/markdown-standards.md)
