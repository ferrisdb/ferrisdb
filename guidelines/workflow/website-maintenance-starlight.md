# Website Maintenance Workflow - Astro Starlight

Step-by-step guide for maintaining the FerrisDB documentation website using Astro Starlight, including daily updates, content management, and deployment.

**Purpose**: Provide systematic workflows for keeping the FerrisDB Starlight website accurate and up-to-date.  
**Prerequisites**: Access to the repository, understanding of Astro/Starlight, and familiarity with ROADMAP.md

## Daily Maintenance Tasks

### 1. Update Statistics (All Pages)

Create a reusable statistics component or update the cached values:

```bash
# Use the cached statistics function from commands.md
eval "$(get_cached_stats)"
echo "Update these stats: Days: $DAYS, Lines: $LINES, Tests: $TESTS, Posts: $POSTS"

# Update the Stats component if using one
cd ferrisdb-docs
# Update src/components/Stats.astro or similar
```

### 2. Update Progress from ROADMAP.md

**Critical**: ALL progress information MUST come from ROADMAP.md

Update these files:

- `/src/content/docs/index.mdx` - Homepage progress sections
- `/src/content/docs/project/roadmap.mdx` - Roadmap page
- `/src/content/docs/project/faq.mdx` - FAQ architecture status

Use Starlight components for better presentation:

```mdx
<Tabs>
  <TabItem label="✅ Completed">- List completed items from ROADMAP.md</TabItem>
  <TabItem label="🚧 In Progress">- List current work</TabItem>
  <TabItem label="⏳ Planned">- List future items</TabItem>
</Tabs>
```

### 3. Update Collaboration Metrics

Track and update these metrics in homepage:

- **Pattern recognitions**: Count Claude's insights in recent blog posts
- **Human intuition saves**: Times human review prevented issues
- **Collaboration score**: Subjective assessment (1-10) of recent work quality
- **Tests passing**: `grep -r "#\[test\]" --include="*.rs" . | wc -l`

### 4. Blog Management

Add new blog posts:

```bash
# Create new blog post
cd ferrisdb-docs/src/content/blog
# Name format: day-N-author-seo-friendly-slug.md
# Human example: day-4-human-why-compaction-matters-for-performance.md
# Claude example: day-4-claude-patterns-in-human-performance-questions.md
```

Ensure proper frontmatter:

```yaml
---
title: "Day 4: Your Title Here"
date: 2025-01-30
authors: [human] # or [claude]
tags: [development, rust, databases]
description: Brief description
excerpt: Short excerpt for listings
---
```

**Blog Post Checklist**:

- [ ] Title follows compelling hook pattern
- [ ] Companion post exists (human/Claude pair)
- [ ] Cross-references included
- [ ] Code examples verified against implementation

## Weekly Maintenance Tasks

### 1. Link Validation

```bash
# Build the site and check for broken links
cd ferrisdb-docs
npm run build

# The build process will report any broken internal links
# For external links, use a link checker tool
npx broken-link-checker http://localhost:4321 --recursive
```

### 2. Content Sync Check

Verify all pages reflect current state:

- Components match implementation
- No outdated feature claims
- Progress indicators accurate
- Blog cross-references work

### 3. Component Updates

Check if any reusable components need updates:

- `Stats.astro` - Project statistics
- Custom blog pages - Author filtering working
- Navigation - All links functional

## Monthly Maintenance Tasks

### 1. Dependency Updates

```bash
cd ferrisdb-docs
# Check for updates
npm outdated

# Update dependencies carefully
npm update @astrojs/starlight
npm update astro

# Test thoroughly after updates
npm run build
npm run preview
```

### 2. Performance Audit

```bash
# Build and analyze bundle size
npm run build

# Check build output for large assets
# Look for warnings about image sizes

# Run Lighthouse on preview
npm run preview
# Then use Chrome DevTools Lighthouse
```

### 3. Content Reorganization

- Review navigation structure in `astro.config.mjs`
- Check if sections need reordering
- Ensure sidebar categories make sense
- Update slug/link references if needed

## Deployment Workflow

### Local Testing

```bash
cd ferrisdb-docs

# Development server (hot reload)
npm run dev

# Production build
npm run build

# Preview production build
npm run preview
```

### GitHub Actions Deployment

The site deploys automatically on push to main. Monitor the Actions tab for:

- Build success/failure
- Deployment status
- Error messages if any

### Manual Deployment Check

After deployment:

1. Visit <https://ferrisdb.org>
2. Check latest changes are live
3. Test navigation and search
4. Verify blog posts display correctly

## FAQ-Specific Maintenance

### Daily FAQ Updates

1. **Statistics Section in FAQ.mdx**

   ```mdx
   **Current Progress**: Day {stats.days} of development with {stats.lines} lines of Rust code, {stats.tests} tests, and {stats.posts} blog posts
   ```

2. **Architecture Status**

   - Update checkmarks ✅ for completed components
   - Must match ROADMAP.md exactly
   - Use Badge components for status

3. **Link Verification**
   - All internal links work
   - GitHub links are current
   - Anchor links function

### Weekly FAQ Updates

1. **Next Steps Section**
   - Priorities must match ROADMAP.md
   - No fictional features
   - Update when items complete

## Common Maintenance Commands

```bash
# Update all statistics at once
cd ferrisdb-docs
eval "$(get_cached_stats)"
# Update Stats component or find/replace in MDX files
find src/content/docs -name "*.mdx" -exec sed -i "s/Day [0-9]* of development/Day $DAYS of development/g" {} \;

# Find all progress sections
grep -r "Progress\|Working\|Building\|Coming" src/content/docs --include="*.mdx"

# Check for outdated status badges
grep -r "PLANNED\|CONCEPTUAL\|FUTURE" src/content/docs --include="*.mdx"

# Find all Mermaid diagrams to verify against ASCII
grep -r "mermaid" src/content/docs --include="*.mdx"

# Verify companion blog posts
for post in src/content/blog/day-*-human-*.md; do
    # Extract day number
    day=$(echo $post | sed 's/.*day-\([0-9]*\)-human-.*/\1/')
    # Look for corresponding Claude post
    claude_post=$(ls src/content/blog/day-$day-claude-*.md 2>/dev/null)
    if [ -z "$claude_post" ]; then
        echo "Missing Claude post for day $day"
    fi
done

# Also check for Claude posts without human companions
for post in src/content/blog/day-*-claude-*.md; do
    # Extract day number
    day=$(echo $post | sed 's/.*day-\([0-9]*\)-claude-.*/\1/')
    # Look for corresponding human post
    human_post=$(ls src/content/blog/day-$day-human-*.md 2>/dev/null)
    if [ -z "$human_post" ]; then
        echo "Missing human post for day $day"
    fi
done

# Count content metrics
echo "Database concepts: $(ls src/content/docs/concepts/database-internals/*.mdx | wc -l)"
echo "Rust patterns: $(ls src/content/docs/concepts/rust-patterns/*.mdx | wc -l)"
echo "Blog posts: $(ls src/content/blog/*.md | wc -l)"
```

## Specific Update Locations

### Homepage (`src/content/docs/index.mdx`)

- **Development Progress**: Update `<Tabs>` component with completed/in-progress/planned
- **Collaboration Metrics**: Update metrics in dedicated section
- **Educational Resources**: Update `<CardGrid>` with new articles

### Getting Started (`src/content/docs/getting-started.mdx`)

- **Prerequisites**: Keep current with actual requirements
- **Quick Start**: Update commands as needed
- **Learning Paths**: Update `<CardGrid>` when new content available

### FAQ (`src/content/docs/project/faq.mdx`)

- **Statistics**: Update "Current Progress" line
- **Architecture**: Update component status badges
- **Common Questions**: Add new Q&As as they arise

### Roadmap (`src/content/docs/project/roadmap.mdx`)

- **Progress bars**: Update percentages
- **Timeline**: Adjust dates if needed
- **Component status**: Match ROADMAP.md exactly

## MDX-Specific Maintenance

### Component Import Management

Ensure consistent imports across MDX files:

```mdx
import { Card, CardGrid, Tabs, TabItem, Aside, Badge } from "@astrojs/starlight/components";

;
```

### Image Management

```bash
# Check image sizes
find src/assets -name "*.png" -o -name "*.jpg" | xargs ls -lh

# Optimize images if needed
# Consider using .webp format for better performance
```

### Code Block Maintenance

Ensure code blocks have proper language tags and optional titles:

````mdx
```rust title="src/main.rs"
// Code here
```
````

## Troubleshooting

### Build Failures

```bash
# Clear cache and rebuild
rm -rf .astro
rm -rf dist
npm run build
```

### Component Errors

- Check import statements
- Verify component props
- Ensure MDX syntax is valid
- Look for unclosed tags

### Search Not Working

```bash
# Rebuild search index
npm run build
# Search index is generated during build
```

### Blog Not Updating

- Check frontmatter date format
- Verify authors array syntax
- Ensure file is in correct directory
- Clear browser cache

## Maintenance Checklist Template

```markdown
## Daily Maintenance - [Date]

- [ ] Update statistics on all pages (use cached function)
- [ ] Verify progress sections match ROADMAP.md
- [ ] Update collaboration metrics on homepage
  - [ ] Pattern recognitions count
  - [ ] Human intuition saves
  - [ ] Collaboration score (1-10)
  - [ ] Test count
- [ ] Check for new completed features
- [ ] Verify all CTAs work
- [ ] Update FAQ statistics section
- [ ] Add new blog posts if any
- [ ] Push changes to trigger deployment

## Weekly Maintenance - [Week of]

- [ ] Validate all internal links
- [ ] Check external links (GitHub, etc.)
- [ ] Verify blog post cross-references
- [ ] Check companion posts (human/Claude pairs)
- [ ] Update FAQ architecture section
- [ ] Test main user journeys:
  - [ ] New visitor → Getting started
  - [ ] CRUD developer → Database concepts
  - [ ] Contributor → Development setup
  - [ ] Learner → Rust by example
- [ ] Review recent deployments

## Monthly Maintenance - [Month]

- [ ] Full content accuracy review
- [ ] Update difficulty indicators (📗📙📕) if still used
- [ ] Check page/image sizes
- [ ] Review navigation structure
- [ ] Update any [PLANNED] → implemented
- [ ] Update dependencies carefully
- [ ] Run performance audit
- [ ] Full accessibility check
- [ ] Verify all code examples against codebase
```

## Migration from Jekyll

If maintaining both sites during transition:

```bash
# Jekyll site (backup)
cd docs
bundle exec jekyll build

# Starlight site (primary)
cd ferrisdb-docs
npm run build
```

Keep Jekyll site as reference until fully transitioned.

## Related Guidelines

- [Website Design - Starlight](../content/website-design-starlight.md) - Design standards
- [Commands](commands.md) - Development commands reference
- [Markdown Standards](../development/markdown-standards.md) - MDX compatibility
