# Markdown Tooling Configuration

Guidelines for markdown formatting and linting in FerrisDB.

## Current Setup

We use two tools for markdown quality:

1. **Prettier** - For formatting (making code look consistent)
2. **markdownlint** - For linting (catching style issues and best practices)

## The Challenge: Jekyll/kramdown Compatibility

Jekyll uses kramdown, which has special syntax that prettier doesn't understand:

```markdown
{:toc} # Table of contents generation
{: .no_toc .text-delta } # CSS class application
```

When prettier sees these, it indents them, breaking Jekyll's processing.

## Our Solution

### 1. Prettier-ignore for kramdown syntax

We wrap kramdown-specific syntax with prettier-ignore comments:

```markdown
<!-- prettier-ignore-start -->

1. TOC
{:toc}
<!-- prettier-ignore-end -->
```

### 2. Disabled markdownlint rules

We've disabled formatting-related rules in `.markdownlint.json` to avoid conflicts:

- **MD007**: Unordered list indentation (let prettier handle this)
- **MD009**: Trailing spaces (prettier removes these)
- **MD010**: Hard tabs (prettier converts to spaces)
- **MD012**: Multiple consecutive blank lines (prettier normalizes these)
- **MD013**: Line length (already disabled - we don't enforce this)
- **MD030**: Spaces after list markers (prettier handles this)
- **MD031**: Fenced code blocks surrounded by blank lines (conflicts with prettier)
- **MD032**: Lists surrounded by blank lines (conflicts with prettier)
- **MD033**: Inline HTML (already disabled - we use HTML for Jekyll)
- **MD036**: Emphasis used instead of heading (already disabled)
- **MD040**: Fenced code blocks should have language (already disabled)
- **MD042**: No empty links (already disabled)
- **MD052**: Reference links and images should use label (already disabled)

### 3. Division of responsibilities

- **Prettier**: Handles all formatting (indentation, spacing, line breaks)
- **markdownlint**: Handles best practices and non-formatting issues

## Alternative Approaches Considered

1. **markdownfmt**: Go-based formatter, but doesn't understand kramdown
2. **mdformat**: Python-based, CommonMark only, no kramdown support
3. **Custom kramdown formatter**: Would require significant development

## Running the Tools

```bash
# Format with prettier
prettier --write "**/*.md"

# Lint with markdownlint
markdownlint-cli2 "**/*.md"

# Both are run automatically in CI
```

## Adding New Markdown Files

When creating new markdown files with Jekyll front matter:

1. Use the appropriate template from `docs/_posts/`
2. The TOC section already has prettier-ignore comments
3. Run both tools before committing

## Future Improvements

If kramdown support becomes critical, we could:

1. Write a prettier plugin for kramdown syntax
2. Switch to a Jekyll-specific markdown formatter
3. Use kramdown itself as a formatter (with custom tooling)

For now, our prettier-ignore approach works well and maintains compatibility.
