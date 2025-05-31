# Starlight Technical Reference

**Purpose**: Technical reference for working with Astro Starlight framework. Use this when you need to understand MDX components, build processes, or troubleshoot issues.

**Note**: For daily content updates, see [Website Maintenance - Simplified](website-maintenance-simple.md).

## MDX Components

### Available Components

```mdx
import { Aside, Card, CardGrid, Tabs, TabItem, Badge, Steps } from "@astrojs/starlight/components";

;
```

### Component Usage Examples

#### Aside (Callouts)

```mdx
<Aside type="note">Default informational note</Aside>
<Aside type="tip">Helpful suggestion</Aside>
<Aside type="caution">Warning message</Aside>
<Aside type="danger">Critical warning</Aside>
```

#### Cards

```mdx
<Card title="Feature" icon="rocket">
  Feature description here
</Card>

<CardGrid>
  <Card title="Card 1" icon="star">
    Content 1
  </Card>
  <Card title="Card 2" icon="moon">
    Content 2
  </Card>
</CardGrid>
```

#### Tabs

````mdx
<Tabs>
  <TabItem label="Rust">```rust let x = 42; ```</TabItem>
  <TabItem label="Python">```python x = 42 ```</TabItem>
</Tabs>
````

#### Steps (Ordered Process)

```mdx
<Steps>1. First step description 2. Second step description 3. Third step description</Steps>
```

**Warning**: Steps component expects a single ordered list as content.

#### Badges

```mdx
<Badge text="NEW" variant="note" />
<Badge text="BETA" variant="caution" />
<Badge text="DEPRECATED" variant="danger" />
<Badge text="STABLE" variant="success" />
```

## Project Structure

```
docs/
├── src/
│   ├── content/
│   │   ├── docs/          # Main documentation
│   │   └── config.ts      # Content configuration
│   ├── components/        # Custom Astro components
│   └── styles/           # Custom CSS
├── astro.config.mjs      # Astro configuration
└── package.json
```

## Build Commands

```bash
# Development server
cd docs
npm run dev

# Build for production
npm run build

# Preview production build
npm run preview

# Type checking
npm run astro check
```

## Content Configuration

### Sidebar Navigation

Edit `astro.config.mjs`:

```js
export default defineConfig({
  integrations: [
    starlight({
      sidebar: [
        {
          label: "Start Here",
          items: [
            { label: "Our Story", link: "/our-story/" },
            { label: "Current Status", link: "/status/" },
          ],
        },
        // Add more sections
      ],
    }),
  ],
});
```

### Frontmatter Options

```yaml
---
title: Page Title
description: SEO description
sidebar:
  order: 1 # Sidebar position
  label: Custom Label # Override title in sidebar
  badge:
    text: NEW
    variant: success
---
```

## Common Issues & Solutions

### MDX Parsing Errors

**Problem**: Code blocks inside TabItem components get corrupted by prettier.

**Solution**: Add empty lines around code blocks:

````mdx
<TabItem label="Example">

```rust
// Code here
```
````

</TabItem>
```

### Build Errors

**Problem**: "Entry was not found" errors.

**Solution**: Check that all linked files exist and frontmatter is valid.

### Component Not Rendering

**Problem**: MDX components show as plain text.

**Solution**: Ensure proper import statement at top of MDX file.

### Unescaped Characters

**Problem**: `<` or `>` in content causes parse errors.

**Solution**: Escape as `&lt;` and `&gt;`.

## Performance Optimization

### Image Handling

- Use Astro's Image component for optimization
- Store images in `src/assets/` for processing
- Public folder images aren't optimized

### Bundle Size

- Starlight is already optimized
- Avoid large client-side components
- Use dynamic imports for heavy features

## Deployment

### GitHub Pages

```yaml
# .github/workflows/deploy.yml
- name: Build
  run: npm run build

- name: Deploy to GitHub Pages
  uses: actions/deploy-pages@v1
```

### Environment Variables

```bash
# For different environments
PUBLIC_SITE_URL=https://ferrisdb.org
```

## Extending Starlight

### Custom Components

Create in `src/components/`:

```astro
---
// MyComponent.astro
export interface Props {
  title: string;
}
const { title } = Astro.props;
---

<div class="my-component">
  <h3>{title}</h3>
  <slot />
</div>
```

### Custom Styles

Add to `src/styles/custom.css`:

```css
/* Override Starlight variables */
:root {
  --sl-color-accent: #your-color;
}
```

## Related Guidelines

- [Website Maintenance - Simplified](website-maintenance-simple.md) - Daily content updates
- [Information Architecture](../content/information-architecture.md) - Content organization
- [Website Design](../content/website-design-starlight.md) - Design principles

---

_Last updated: 2025-05-31_
