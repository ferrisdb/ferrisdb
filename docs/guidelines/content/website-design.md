# Website Design Guidelines

Design guidelines for the FerrisDB documentation website, focusing on educational value and user engagement.

## Design Philosophy

- **Educational First**: Every design decision should make learning easier
- **Page-Turner Experience**: Content should be engaging and flow naturally
- **Transparent & Honest**: Show real progress, real mistakes, real learning
- **Human-AI Balance**: Showcase collaboration without overselling either side

## Visual Design

### Color Palette

- **Primary**: Rust orange (#ce422b)
- **Secondary**: Darker orange (#f74c00)
- **Accent**: Bright orange (#ffa500)
- **Text**: Dark gray (#2d3748)
- **Muted**: Medium gray (#718096)

### Typography

- **Body**: Inter font family, 18px base size, 1.7-1.8 line height
- **Code**: JetBrains Mono, 0.95rem in blocks
- **Headers**: 600-700 weight for emphasis

### Layout

- **Article content**: 750px max width for optimal reading
- \*\*Consistent card-based design for featured content
- \*\*Generous whitespace for breathing room

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
- **Deep Dive Articles**: Comprehensive technical explanations
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

## Quality Checklist

- [ ] Follows Just the Docs conventions
- [ ] Mobile responsive
- [ ] Accessible (WCAG AA)
- [ ] Fast loading
- [ ] Clear navigation
- [ ] Consistent visual language
- [ ] Educational value prioritized
- [ ] No custom CSS hacks
