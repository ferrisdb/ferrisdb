# Website Design Guidelines

**⚠️ DEPRECATED as of 2025-05-31**: This file is being replaced by [website-design-starlight.md](website-design-starlight.md) as part of the migration to Astro Starlight. This file will be removed after the migration is complete. Please use the new Starlight-specific guidelines for all new work.

Design guidelines for the FerrisDB documentation website, focusing on educational value and user engagement.

## Design Philosophy

- **Educational First**: Every design decision should make learning easier
- **Page-Turner Experience**: Content should be engaging and flow naturally
- **Transparent & Honest**: Show real progress, real mistakes, real learning
- **Human-AI Balance**: Showcase collaboration without overselling either side

## Visual Design

**⚠️ IMPORTANT**: Use ONLY Just the Docs built-in utilities. No custom CSS allowed.

### Diagram Guidelines for Website

**CRITICAL**: Website content uses Mermaid diagrams for better visual presentation, but ALL Mermaid diagrams MUST be based on corresponding ASCII diagrams in the technical guidelines.

1. **Mermaid for Website Content**
   - Use Mermaid.js for blog posts, articles, and documentation pages
   - Provides cleaner rendering and better responsiveness
   - Supports interactive features and theming
2. **ASCII as Source of Truth**
   - Every Mermaid diagram must have an ASCII counterpart in technical guidelines
   - ASCII diagrams in `/guidelines/technical/` are authoritative
   - When updating diagrams, update ASCII first, then Mermaid
3. **Mermaid Syntax Examples**

   ```mermaid
   graph TD
       A[Client] --> B[Server]
       B --> C[Storage Engine]
       C --> D[MemTable]
       C --> E[SSTables]
       C --> F[WAL]
   ```

4. **Consistency Rules**
   - Node names must match between ASCII and Mermaid
   - Flow direction must be identical
   - All components shown in ASCII must appear in Mermaid
   - Labels and descriptions must align

### Implementation Status in Content

**MANDATORY**: When creating any content (blog posts, articles, documentation) that discusses features not yet implemented:

1. **Clear Status Markers**
   - Use **[PLANNED]**, **[CONCEPTUAL]**, or **[FUTURE]** tags
   - Place status at the beginning of sections
   - Use callout boxes for emphasis
2. **Example Patterns**

   ```markdown
   ## Transaction Support [PLANNED]

   > **Note**: This section describes planned functionality not yet implemented.

   In the future, FerrisDB will support ACID transactions...
   ```

3. **Avoid Misleading Claims**
   - Never present future features as current
   - Use future tense for unimplemented features
   - Be explicit about what exists vs. what's planned
4. **Update When Implemented**
   - Remove status markers when features land
   - Update tense from future to present
   - Add "Available since version X" notes

### Color Palette

Defined in `_sass/color_schemes/ferrisdb.scss` and applied automatically:

- **Primary**: Rust orange - use `.btn-primary` or `.text-purple-300`
- **Text**: High contrast dark gray - default text color
- **Backgrounds**: Use `.bg-grey-lt-000` for light sections

### Typography

Use Just the Docs utility classes:

- **Font Size**: `.fs-1` to `.fs-10` (9px to 48px)
- **Font Weight**: `.fw-300`, `.fw-400`, `.fw-500`, `.fw-700`
- **Small Text**: `.text-small` for meta information

### Layout

- **Spacing**: Use `.mb-*`, `.mt-*`, `.my-*` classes (1-8 scale)
- **Alignment**: Use `.text-center`, `.text-left`, `.text-right`
- **No custom containers**: Let Just the Docs handle max-width

## Content Structure

### Homepage Flow

1. **Hero with clear value proposition**
2. **Learning paths for different audiences**
3. **Why we built this (emotional connection)**
4. **What you'll learn (concrete benefits)**
5. **Progress tracking (transparency)**
6. **AI collaboration showcase**
7. **Educational resources**
8. **Call to action**

### Navigation

- **Dropdown menus for content grouping**
- **Learn menu groups all educational content**
- **Blog menu separates human and AI perspectives**
- **FAQ prominently featured**

## Component Patterns

### Cards

- Rounded corners (8-12px)
- Subtle shadows
- Hover effects

### Buttons

- **Primary**: Filled
- **Outline**: Bordered
- **Ghost**: Transparent

### Code Blocks

- Light theme (#f6f8fa background)
- Clear syntax highlighting

### Progress Items

- Visual indicators (✅/🚧/⏳) with clear status

### Metrics

- Large numbers with descriptive labels

## Writing for the Web

### Scannable Content

- Use headers, bullets, and short paragraphs
- Progressive disclosure: Overview → Details → Deep Dive
- Multiple entry points: Different CTAs for different audiences
- Clear signposting: Tell readers where they are and where to go next

### Visual Consistency

- **Emoji indicators**: 📗📙📕 for difficulty, ⏱️ for time, 📊📄🏗️ for stats
- **No label badges**: Use inline text formatting
- **Consistent spacing**: Follow Just the Docs utilities

## Jekyll & Just the Docs Configuration

### Theme Settings

- **Theme Version**: Just the Docs 0.10.1 (keep updated)
- **No Custom CSS**: Use only Just the Docs built-in components and utilities
- **Accessibility**: Maintain WCAG AA color contrast standards

### Essential Plugins

```ruby
group :jekyll_plugins do
  gem "jekyll-feed"
  gem "jekyll-sitemap"
  gem "jekyll-seo-tag"
  gem "jekyll-include-cache"
  gem "jekyll-paginate"
end
```

### Blog Configuration

- **Pagination**: 5 posts per page
- **Blog listings**: Use HTML format (`blog/index.html`, `claude-blog/index.html`)
- **No manual loops**: Let pagination handle post listing

### Google Analytics

- **Tracking ID**: G-JPW5LY247F
- **IP Anonymization**: Enabled for GDPR compliance

## Content Organization

### Learn Menu Structure

- **Getting Started**: First steps with FerrisDB
- **Database Concepts**: Comprehensive technical explanations
- **Rust by Example**: Language comparisons and learning
- **Architecture**: System design documentation

### Blog Menu Structure

- **Development Blog**: Human perspective on building FerrisDB
- **Claude's Blog**: AI perspective on patterns and collaboration

### Resources Menu

- **FAQ**: Common questions and answers
- **Contributing**: How to get involved
- **API Reference**: Generated documentation

## Design Principles

### Clarity Over Cleverness

- Simple, clear navigation
- Obvious CTAs
- Predictable layouts

### Educational Focus

- Code examples front and center
- Clear difficulty indicators
- Estimated reading times
- Prerequisites clearly stated

### Mobile Responsiveness

- Content readable on all devices
- Navigation accessible on mobile
- Code blocks scrollable horizontally

### Performance

- Optimize images and assets
- Minimize JavaScript usage
- Fast page load times
- Good Core Web Vitals scores

## Brand Voice

### Professional but Approachable

- Technical accuracy without intimidation
- Conversational tone
- Honest about complexities

### Community-Focused

- Welcoming to newcomers
- Celebrate contributions
- Share learning journey

### Transparent

- Show work in progress
- Admit limitations
- Document failures and learnings

## Implementation Guidelines

### Use Just the Docs Components

- Leverage built-in callouts for warnings/notes
- Use native navigation features
- Apply utility classes for spacing

### Maintain Consistency

- Follow established patterns
- Use templates for new content
- Review design checklist before publishing

### Accessibility First

- Proper heading hierarchy
- Alt text for images
- Sufficient color contrast
- Keyboard navigation support

## ROADMAP.md as Source of Truth

The `ROADMAP.md` file is the SINGLE source of truth for ALL project progress displayed on the website. Never add progress information that isn't tracked in ROADMAP.md.

**ROADMAP.md Structure**:

- **Storage Engine Foundation**: Core database components (WAL, MemTable, SSTable, etc.)
- **Basic Operations**: CRUD operations and queries
- **ACID Transactions**: Transaction support and MVCC
- **Distribution Layer**: Distributed system features
- **Performance & Monitoring**: Metrics and benchmarking
- **Operations & Management**: Admin tools and utilities
- **Client & API**: Protocol and client libraries
- **Production Readiness**: Reliability features
- **Documentation & Examples**: Docs and learning materials
- **Educational Content**: Tutorials and guides

**Mapping ROADMAP.md to Website Content**:

1. ✅ Items marked `[x]` → "Available Now" / "What's Working" / "Core Concepts (Available Now)"
2. 🚧 Items marked `[ ]` in current focus areas → "Currently Building" / "In Progress"
3. ⏳ Items in later sections → "Coming Soon" / "Coming Next"

**ALL Progress Sections Must Update from ROADMAP.md**:

- `/index.md` → "Latest Progress" section (lines 102-124) and "Development Progress" table (lines 97-108)
- `/start-here.md` → "What You'll Learn" section (lines 32-50) and "What's implemented so far" section (lines 68-74)
- Any other page showing feature status

## Regular Maintenance Tasks

For detailed maintenance workflows, see [Website Maintenance](../workflow/website-maintenance.md).

### Quick Reference

- **Daily**: Update statistics and progress sections
- **Weekly**: Validate links and check progress accuracy
- **Monthly**: Full content review and navigation testing

### Update ALL Progress Sections from ROADMAP.md

**⚠️ CRITICAL**: ALL progress information on the website MUST be derived from ROADMAP.md. Never add features that aren't tracked there.

#### 1. Update `/start.md` Progress Sections

**"What You'll Learn" section (lines 32-50)**

1. Check ROADMAP.md for completed/in-progress items
2. Update three subsections:
   - **Core Concepts (Available Now)**: Completed items from "Storage Engine Foundation"
   - **Currently Building**: In-progress items from current focus areas
   - **Coming Soon**: Items from later ROADMAP.md sections
3. Transform technical terms into learner-friendly language:
   - "Write-Ahead Log (WAL)" → "Why databases need Write-Ahead Logs (WAL) for crash recovery"
   - "MemTable (Skip List)" → "How databases store data in memory with Skip Lists"
   - "SSTable format" → "The SSTable format - how databases organize data on disk"
   - "Compaction" → "Compaction - how databases merge files efficiently"
   - "Bloom filters" → "Bloom filters - probabilistic data structures for speed"

**"What's implemented so far" section (lines 68-74)**

1. List completed items from ROADMAP.md with checkmarks
2. Show current work in progress
3. Mention what's coming next (interactive REPL when "CLI client" is ready)

#### 2. Update `/index.md` Progress Sections

**"Development Progress" table (lines 97-108)**:

1. Check ROADMAP.md for completed/in-progress items
2. Update table rows:
   - ✅ Mark completed sections as "Complete"
   - 🚧 Mark sections with active work as "In Progress"
   - ⏳ Mark future sections as "Planned"
3. Include brief technical descriptions for each component

**"Latest Progress" section (if exists)**:

1. Check ROADMAP.md for completed/in-progress items
2. Update three subsections:
   - **✅ What's Working**: All items marked `[x]` in ROADMAP.md
   - **🚧 Currently Building**: Items marked `[ ]` in current focus areas (usually Storage Engine)
   - **⏳ Coming Next**: Next logical items from ROADMAP.md
3. Use technical descriptions (more detailed than start.md):
   - Include implementation details: "MemTable with concurrent Skip List"
   - Mention optimizations: "SSTable reader with binary search optimization"
   - Reference architectural decisions: "Clean API design after Day 2 refactoring"

#### 3. Check for Other Progress Sections

- Search for any other pages mentioning features or progress
- Ensure they all reference ROADMAP.md items only
- Update as needed to maintain consistency

### Language Guidelines for Progress Updates

**For `/start.md` (learner-focused)**

- Use "why" and "how" framing
- Explain the purpose of each feature
- Make it accessible to CRUD developers
- Example: "Why databases need Write-Ahead Logs" not just "WAL implemented"

**For `/index.md` (technically accurate)**

- Use precise technical terms
- Include implementation details
- Mention specific algorithms or data structures
- Example: "MemTable with concurrent Skip List" not just "in-memory storage"

### Daily Homepage Checklist

- [ ] **Development Progress table** (lines 101-108): Reflects current ROADMAP.md status
- [ ] **Latest Collaboration Metrics** (lines 127-133): Update from actual collaboration data
  - Pattern recognitions: Count Claude's insights in recent blog posts
  - Human intuition saves: Times human review prevented issues
  - Collaboration score: Subjective assessment of recent work quality
  - Tests passing: `grep -r "#\[test\]" --include="*.rs" . | wc -l`
- [ ] **Educational Resources** (lines 134-157): Keep links current and mark "Coming Soon" items
  - Update database concepts article links to real URLs
  - Mark new articles as available vs "Coming Soon"
  - Update Rust by Example progress
- [ ] **No production claims**: Keep messaging as educational/experimental
- [ ] **CTA buttons work**: All links go to existing pages

### Per Blog Post

- [ ] Ensure title follows compelling hook pattern
- [ ] Update permalink to match title
- [ ] Check companion post links work
- [ ] Verify code examples match actual implementation

### When New Features Are Ready

- [ ] **When Binary/REPL is Available**: Update "Try It Now" sections

  - Homepage: Replace "See The Code In Action" with actual REPL example
  - Start page: Add interactive playground example
  - Example format:

    ```bash
    # Docker example (when available)
    docker run -it ferrisdb/playground

    ferris> set key "value"
    OK
    ferris> get key
    "value"
    ```

- [ ] **When Examples are Added**: Update with `cargo run --example`
- [ ] **When Server is Ready**: Add client connection examples

### All Maintenance Tasks

See [Website Maintenance](../workflow/website-maintenance.md) for:

- Daily/weekly/monthly checklists
- Statistics update commands
- FAQ maintenance procedures
- Link validation workflows
- Content accuracy reviews

## Quality Checklist

- [ ] Follows Just the Docs conventions
- [ ] Mobile responsive
- [ ] Accessible (WCAG AA)
- [ ] Fast loading
- [ ] Clear navigation
- [ ] Consistent visual language
- [ ] Educational value prioritized
- [ ] No custom CSS hacks
- [ ] All progress sections match ROADMAP.md
- [ ] FAQ.md statistics current and accurate
- [ ] All FAQ links functional

## Related Guidelines

- **Maintenance**: [Website Maintenance](../workflow/website-maintenance.md) - Step-by-step workflows
- **Commands**: [Common Commands](../workflow/commands.md#website-maintenance-commands) - Statistics scripts
- **Content Types**: [Blogging](blogging.md), [Database Concepts](database-concepts-articles.md), [Rust by Example](rust-by-example.md)
- **Formatting**: [Markdown Standards](../development/markdown-standards.md) - Jekyll compatibility
