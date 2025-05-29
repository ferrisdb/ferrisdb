# Markdown Quality Guidelines

Ensuring consistent, high-quality markdown documentation across FerrisDB.

## Markdown Quality Requirements (REQUIRED before commit)

1. **Format first**: `prettier --write "**/*.md"`
2. **Lint second**: `markdownlint-cli2 "**/*.md"`
3. **Fix any remaining issues manually** - prettier doesn't catch everything
4. **Verify clean**: Run linter again to ensure no errors

## Common Issues Prettier Might Miss

- Lists need blank lines before and after
- Code blocks need blank lines before and after
- Headers need blank lines before and after

**Note**: Consider adding `.prettierrc` configuration to ensure consistent formatting that aligns with markdownlint rules.

## Markdown Linting Commands

```bash
# Lint markdown files
markdownlint-cli2 "**/*.md" "!target/**" "!**/target/**"

# Auto-fix markdown formatting
prettier --write "**/*.md"
```

## Markdown Standards

### Headers

- Use ATX-style headers (`#` not underlines)
- Include space after `#` symbols
- Surround headers with blank lines
- Use sentence case for headers
- Don't skip header levels (no h1 → h3)

### Lists

- Use `-` for unordered lists (not `*` or `+`)
- Use `1.` for all ordered list items (auto-numbering)
- Indent nested lists with 2 spaces
- Include blank lines before and after lists
- Don't mix ordered and unordered list markers at same level

### Code Blocks

- Always specify language for syntax highlighting
- Use triple backticks (```) not tildes (~~~)
- Include blank lines before and after code blocks
- Use `bash` for shell commands, not `sh` or `shell`

````markdown
# Good

```rust
let x = 42;
```

# Bad

```
let x = 42;
```
````

### Links

- Use reference-style links for repeated URLs
- Use descriptive link text (not "click here")
- Check for broken links regularly
- Use relative paths for internal documentation links

### Emphasis

- Use `**bold**` for strong emphasis
- Use `*italic*` for regular emphasis
- Don't use underscores for emphasis
- Don't nest emphasis styles

### Line Length

- No hard line length limit for paragraphs
- Let prettier handle line wrapping
- Keep code examples under 100 characters when possible
- Use line breaks for readability in complex lists

### Tables

- Use pipes (`|`) for all table borders
- Align columns with spaces for readability
- Include header separator row
- Keep tables simple (consider lists for complex data)

### Images

- Include alt text for all images
- Use relative paths for project images
- Store images in `docs/assets/images/`
- Use descriptive filenames

### Front Matter

- Required for all Jekyll pages
- Use proper YAML syntax
- Include all required fields for page type
- Keep descriptions under 160 characters

### Common Markdownlint Rules

- **MD001**: Header levels should only increment by one
- **MD003**: Header style should be consistent (ATX)
- **MD004**: Unordered list style should be consistent (`-`)
- **MD007**: Unordered list indentation (2 spaces)
- **MD009**: No trailing spaces
- **MD010**: No hard tabs (use spaces)
- **MD012**: No multiple consecutive blank lines
- **MD013**: Line length (disabled for paragraphs)
- **MD014**: Dollar signs before commands without output
- **MD018**: No space after hash on atx style header
- **MD022**: Headers should be surrounded by blank lines
- **MD023**: Headers must start at beginning of line
- **MD024**: No duplicate header text at same level
- **MD025**: Single h1 per document
- **MD026**: No trailing punctuation in headers
- **MD029**: Ordered list prefix (use `1.` for all)
- **MD030**: Spaces after list markers
- **MD031**: Fenced code blocks surrounded by blank lines
- **MD032**: Lists surrounded by blank lines
- **MD033**: No inline HTML (with exceptions)
- **MD034**: No bare URLs (use angle brackets)
- **MD036**: Emphasis used instead of headers
- **MD037**: Spaces inside emphasis markers
- **MD038**: Spaces inside code span elements
- **MD039**: Spaces inside link text
- **MD040**: Fenced code blocks should have language
- **MD041**: First line should be top level header
- **MD042**: No empty links
- **MD045**: Images should have alt text
- **MD046**: Code block style (fenced)
- **MD047**: Files should end with single newline
- **MD048**: Code fence style (backticks)

## Documentation Site Formatting

### Jekyll-Specific Requirements

- Use `{: .class-name }` for custom classes
- Include `<!--more-->` for post excerpts
- Use liquid tags properly (spaces around filters)
- Test locally with `bundle exec jekyll serve`

### Just the Docs Theme

- Use built-in utility classes
- Don't add custom CSS
- Follow theme's component patterns
- Maintain accessibility standards

## Workflow Integration

### Pre-Commit Checks

1. Run prettier to format
2. Run markdownlint to validate
3. Fix any remaining issues
4. Verify all checks pass

### CI Integration

- CI runs both prettier and markdownlint
- PRs blocked if markdown quality checks fail
- Fix locally before pushing updates

### Editor Integration

- Install markdownlint extension for your editor
- Configure prettier for markdown files
- Enable format-on-save for consistency
- Use editor's markdown preview

## Best Practices

1. **Write first, format later**: Focus on content, then clean up
2. **Preview before committing**: Check rendered output
3. **Be consistent**: Follow patterns in existing docs
4. **Keep it simple**: Avoid complex formatting
5. **Test links**: Ensure all links work
6. **Use templates**: Start from existing templates
7. **Review diffs**: Check formatting changes in PRs

## Quick Reference

```bash
# Format all markdown files
prettier --write "**/*.md"

# Check for linting issues
markdownlint-cli2 "**/*.md"

# Format and lint (typical workflow)
prettier --write "**/*.md" && markdownlint-cli2 "**/*.md"

# Preview Jekyll site locally
bundle exec jekyll serve

# Check specific file
markdownlint-cli2 "path/to/file.md"
````

## Troubleshooting

### Common Issues

1. **Prettier and markdownlint conflicts**:

   - Run prettier first, then markdownlint
   - Manually fix any remaining issues
   - Consider prettier config adjustments

2. **Line length warnings**:

   - Ignored for paragraph text
   - Fix for code blocks and tables
   - Use line breaks for readability

3. **Blank line errors**:

   - Add blank lines around blocks
   - Check lists, headers, code blocks
   - Remove multiple consecutive blanks

4. **Front matter errors**:
   - Ensure valid YAML syntax
   - Check required fields
   - Use quotes for special characters

### Getting Help

- Check markdownlint rule documentation
- Review existing well-formatted files
- Ask in PR reviews for guidance
- Use editor's linting feedback
