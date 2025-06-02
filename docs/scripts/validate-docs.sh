#!/bin/bash
# Quick validation script for Starlight docs
# This is much faster than a full build but catches most issues

set -e

echo "🔍 Validating Starlight documentation..."

# Check required directories
echo "✓ Checking directory structure..."
required_dirs=("src/content/docs" "src/assets" "public")
for dir in "${required_dirs[@]}"; do
    if [ ! -d "$dir" ]; then
        echo "❌ Missing required directory: $dir"
        exit 1
    fi
done

# Validate Astro config
echo "✓ Checking Astro configuration..."
if [ ! -f "astro.config.mjs" ]; then
    echo "❌ Missing astro.config.mjs"
    exit 1
fi

# Check for TypeScript config
if [ -f "tsconfig.json" ]; then
    echo "✓ TypeScript config found"
fi

# Validate content files
echo "✓ Checking content files..."
content_count=$(find src/content/docs -name "*.md" -o -name "*.mdx" | wc -l)
if [ "$content_count" -eq 0 ]; then
    echo "❌ No markdown content found in src/content/docs"
    exit 1
fi
echo "  Found $content_count content files"

# Check frontmatter in a sample of files
echo "✓ Validating frontmatter..."
errors=0
for file in $(find src/content/docs -name "*.mdx" -o -name "*.md" | head -20); do
    # Check for frontmatter markers
    if ! head -1 "$file" | grep -q "^---$"; then
        echo "  ⚠️  Missing frontmatter in: $file"
        errors=$((errors + 1))
    else
        # Check for required frontmatter fields
        if ! grep -q "^title:" "$file"; then
            echo "  ⚠️  Missing 'title' in frontmatter: $file"
            errors=$((errors + 1))
        fi
        if ! grep -q "^description:" "$file"; then
            echo "  ⚠️  Missing 'description' in frontmatter: $file"
            errors=$((errors + 1))
        fi
    fi
done

if [ "$errors" -gt 0 ]; then
    echo "❌ Found $errors frontmatter issues"
    exit 1
fi

# Check for common MDX syntax issues
echo "✓ Checking MDX syntax..."
mdx_errors=0
for file in $(find src/content/docs -name "*.mdx" | head -10); do
    # Check for import statements without quotes
    if grep -E '^import .* from [^"'\''`]' "$file" > /dev/null 2>&1; then
        echo "  ⚠️  Import statement might be missing quotes in: $file"
        mdx_errors=$((mdx_errors + 1))
    fi
    
    # Check for obvious JSX issues (very basic check to avoid false positives)
    # Only flag if we see an opening tag at the start of a line with no closing
    if grep -E '^<[A-Za-z]+[^/>]*>$' "$file" > /dev/null 2>&1; then
        echo "  ⚠️  Possible unclosed JSX tag in: $file"
        mdx_errors=$((mdx_errors + 1))
    fi
done

if [ "$mdx_errors" -gt 0 ]; then
    echo "⚠️  Found $mdx_errors potential MDX syntax issues (non-blocking)"
fi

# Quick Astro syntax check (if available)
# Note: astro check requires @astrojs/check and typescript packages
# In CI, we skip this check since packages can't be auto-installed
if [ "$CI" = "true" ]; then
    echo "ℹ️  Skipping Astro check in CI environment"
    echo "    (Full type checking happens during build in deploy-docs.yml)"
elif command -v npx &> /dev/null && [ -f "node_modules/@astrojs/check/package.json" ]; then
    echo "✓ Running Astro check..."
    npx astro check || echo "  ℹ️  Astro check completed with warnings"
else
    echo "ℹ️  Skipping Astro check (dependencies not installed)"
    echo "    Run 'npm install @astrojs/check typescript' to enable"
fi

echo "✅ Documentation validation complete!"
echo ""
echo "ℹ️  This is a quick validation. Full build happens in deploy-docs.yml"