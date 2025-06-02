# Markdown Standards

Comprehensive guidelines for markdown documentation in FerrisDB, including quality standards, formatting rules, and tool configuration.

**Purpose**: Ensure consistent, high-quality markdown documentation across the project.  
**Prerequisites**: Basic markdown knowledge, prettier installed

## 🚨 MANDATORY: Format Before Commit

**CRITICAL**: You MUST run prettier after ANY markdown or MDX changes:

```bash
# REQUIRED: Format all markdown after making changes
prettier --write "**/*.md" "**/*.mdx"

# Verify formatting (CI will check this)
prettier --check "**/*.md" "**/*.mdx"
```

**Why this is mandatory**:

- CI will fail if markdown is not properly formatted
- Prevents merge conflicts from formatting differences
- Ensures consistent documentation across the codebase
- Required for MDX files to prevent build failures

**Never skip this step** - it's as important as `cargo fmt` for Rust code!

## 🚨 MANDATORY: Starlight Build Verification

**CRITICAL**: When modifying ANY Starlight site files (docs/), you MUST build before committing:

```bash
cd docs
npm run build
```

**Why this is mandatory**:

- Prevents MDX syntax errors that break the documentation site
- Validates Steps components and other Starlight-specific syntax
- Ensures the documentation site can actually be deployed
- Catches build issues early rather than in CI

**Rule**: If you touch ANY file in `docs/`, you MUST run the build to verify it works!

## Tool Configuration

### Prettier Handles Everything

**Prettier** handles all markdown formatting:

- Indentation and spacing
- Line breaks and wrapping
- List formatting
- Table alignment
- Consistent style

No more conflicting tools or complex configurations!

### Jekyll/kramdown Compatibility

Jekyll uses kramdown syntax that prettier doesn't understand. We handle this with prettier-ignore:

```markdown
<!-- prettier-ignore-start -->

1. TOC
{:toc}
<!-- prettier-ignore-end -->
```

### Configuration Files

- **`.prettierrc`** - Prettier formatting rules
- That's it! One tool, one config file

## Writing Standards

### Headers

- Use ATX-style (`#` not underlines)
- Include space after `#`
- Surround with blank lines
- Use sentence case
- Don't skip levels (h1 → h2 → h3)

### Lists

- Use `-` for unordered lists
- Use `1.` for all ordered items
- Indent nested lists with 2 spaces
- Include blank lines around lists

### Code Blocks

````markdown
```rust
// Always specify language
let x = 42;
```
````

- Use triple backticks (not tildes)
- Always specify language
- Include blank lines before/after

### Links

```markdown
// Good
[Descriptive text](path/to/file.md)

// Bad
[Click here](path/to/file.md)
```

- Use relative paths for internal links
- Use descriptive link text
- Check for broken links regularly

### Tables

| Column 1 | Column 2 | Column 3 |
| -------- | -------- | -------- |
| Data 1   | Data 2   | Data 3   |

- Use pipes for all borders
- Align with spaces for readability
- Keep simple (consider lists for complex data)

### Front Matter (Starlight/MDX)

```yaml
---
title: "Page Title"
description: "Keep under 160 characters for SEO"
sidebar:
  order: 1
  badge: "New" # Optional
---
```

## Common Issues and Solutions

### 1. MDX Component Formatting

**Problem**: Prettier may format MDX components unexpectedly  
**Solution**: Use prettier-ignore comments around complex components

### 2. MDX Syntax Issues

**Problem**: Prettier indents `{:toc}` breaking Jekyll  
**Solution**: Use prettier-ignore comments

### 3. Line Length Warnings

**Problem**: Long lines in paragraphs  
**Solution**: Let prettier handle wrapping; focus on code blocks

### 4. Blank Line Errors

**Problem**: Missing blank lines around elements  
**Solution**: Add blank lines around headers, lists, code blocks

## Editor Integration

### VS Code

1. Install extension:

   - Prettier - Code formatter

2. Add to settings.json:
   ```json
   {
     "[markdown]": {
       "editor.formatOnSave": true,
       "editor.defaultFormatter": "esbenp.prettier-vscode"
     }
   }
   ```

### Other Editors

- **Vim**: ALE plugin with prettier
- **IntelliJ**: Built-in markdown support + prettier plugin
- **Sublime**: JsPrettier package

## Workflow Integration

### Pre-commit

```bash
# Add to your workflow
prettier --write "**/*.md"
```

### CI Integration

Prettier runs automatically in CI. PRs fail if:

- Prettier would make changes

### Quick Commands

```bash
# Format all markdown
prettier --write "**/*.md"

# Check without changing
prettier --check "**/*.md"

# Format specific file
prettier --write "path/to/file.md"

# Format and check (typical workflow)
prettier --write "**/*.md" && prettier --check "**/*.md"
```

## Best Practices

1. **Write first, format later** - Focus on content
2. **Use templates** - Start from existing files
3. **Preview locally** - Check rendered output
4. **Be consistent** - Follow existing patterns
5. **Keep it simple** - Avoid complex formatting

## Troubleshooting

### Formatting looks wrong?

1. Run prettier to auto-fix: `prettier --write file.md`
2. Use prettier-ignore for special cases
3. Compare with well-formatted files
4. Ask in PR review

### Starlight build failing?

1. Check for MDX syntax errors (unescaped `<>` in text)
2. Verify front matter YAML
3. Test locally: `cd docs && npm run build`
4. Check [Starlight Technical Reference](../workflow/starlight-technical-reference.md)

## Related Guidelines

- [Documentation](documentation.md) - Writing technical documentation
- [Visualization](visualization.md) - Creating diagrams and tables
- [Website Design](../content/website-design-starlight.md) - Starlight-specific requirements

---
_Last updated: 2025-06-01_
