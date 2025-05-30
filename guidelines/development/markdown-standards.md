# Markdown Standards

Comprehensive guidelines for markdown documentation in FerrisDB, including quality standards, formatting rules, and tool configuration.

**Purpose**: Ensure consistent, high-quality markdown documentation across the project.  
**Prerequisites**: Basic markdown knowledge, prettier and markdownlint installed

## Quick Start

Before committing any markdown:

```bash
# 1. Format with prettier
prettier --write "**/*.md"

# 2. Lint with markdownlint
markdownlint-cli2 "**/*.md"

# Both must pass before committing!
```

## Tool Configuration

### Division of Responsibilities

1. **Prettier** - Handles all formatting:

   - Indentation and spacing
   - Line breaks and wrapping
   - List formatting
   - Table alignment

2. **markdownlint** - Handles best practices:
   - Header structure
   - Link validity
   - Emphasis consistency
   - Code block languages

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
- **`.markdownlint.json`** - Disabled formatting rules to avoid conflicts with prettier
- Both configured to work together without conflicts

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

### Front Matter (Jekyll)

```yaml
---
layout: post
title: "Required for Jekyll pages"
description: "Keep under 160 characters"
---
```

## Common Issues and Solutions

### 1. Prettier/markdownlint Conflicts

**Problem**: Both tools want different formatting  
**Solution**: Run prettier first, then markdownlint

### 2. kramdown Syntax Breaking

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

1. Install extensions:

   - Prettier - Code formatter
   - markdownlint

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

- **Vim**: ALE plugin with prettier/markdownlint
- **IntelliJ**: Built-in markdown support + prettier plugin
- **Sublime**: MarkdownLinting + JsPrettier packages

## Workflow Integration

### Pre-commit

```bash
# Add to your workflow
prettier --write "**/*.md" && markdownlint-cli2 "**/*.md"
```

### CI Integration

Both tools run automatically in CI. PRs fail if:

- Prettier would make changes
- markdownlint finds issues

### Quick Commands

```bash
# Format all markdown
prettier --write "**/*.md"

# Check without changing
prettier --check "**/*.md"

# Lint all markdown
markdownlint-cli2 "**/*.md"

# Lint specific file
markdownlint-cli2 "path/to/file.md"

# Format and lint (typical workflow)
prettier --write "**/*.md" && markdownlint-cli2 "**/*.md"
```

## Best Practices

1. **Write first, format later** - Focus on content
2. **Use templates** - Start from existing files
3. **Preview locally** - Check rendered output
4. **Be consistent** - Follow existing patterns
5. **Keep it simple** - Avoid complex formatting

## Troubleshooting

### Can't find the issue?

1. Check the specific rule in markdownlint output
2. Look up rule at [markdownlint rules](https://github.com/DavidAnson/markdownlint/blob/main/doc/Rules.md)
3. Compare with well-formatted files
4. Ask in PR review

### Jekyll build failing?

1. Check for broken kramdown syntax
2. Verify front matter YAML
3. Test locally: `bundle exec jekyll serve`
4. Check for special characters in titles

## Related Guidelines

- [Documentation](documentation.md) - Writing technical documentation
- [Visualization](visualization.md) - Creating diagrams and tables
- [Website Design](../content/website-design.md) - Jekyll-specific requirements
