# Website Maintenance Workflow

Step-by-step guide for maintaining the FerrisDB documentation website, including daily updates, FAQ maintenance, and progress tracking.

**Purpose**: Provide systematic workflows for keeping the FerrisDB website accurate and up-to-date.  
**Prerequisites**: Access to the repository, understanding of Jekyll, and familiarity with ROADMAP.md

## Daily Maintenance Tasks

### 1. Update Statistics (All Pages)

Use the cached statistics function from [Commands](commands.md#statistics-and-metrics):

```bash
# Source the function from commands.md first, then:
eval "$(get_cached_stats)"
echo "Update these stats: Days: $DAYS, Lines: $LINES, Tests: $TESTS, Posts: $POSTS"
```

See [Commands > Statistics and Metrics](commands.md#statistics-and-metrics) for the full function definition.

### 2. Update Progress from ROADMAP.md

**Critical**: ALL progress information MUST come from ROADMAP.md

1. Check ROADMAP.md for current status
2. Update these sections:
   - `/index.md` - Development Progress table
   - `/start-here.md` - What You'll Learn sections
   - `/faq.md` - Architecture status

### 3. Update Collaboration Metrics

On `/index.md`, update:

- Pattern recognitions (from recent blog posts)
- Human intuition saves (from PR reviews)
- Collaboration score (subjective assessment)
- Test count (use cached stats)

## Weekly Maintenance Tasks

### 1. Link Validation

```bash
# Check for broken internal links
find docs -name "*.md" -exec grep -l "\[.*\](/" {} \; | while read file; do
    echo "Checking $file"
    # Extract and verify each link
done

# Verify external links work
grep -r "https://" docs/ --include="*.md" | cut -d'"' -f2 | sort -u | while read url; do
    curl -s -o /dev/null -w "%{http_code} $url\n" "$url"
done
```

### 2. Progress Accuracy Check

Compare all progress sections against ROADMAP.md:

- Remove features not in ROADMAP.md
- Add new completed items
- Update status of in-progress items

### 3. Blog Structure Validation

```bash
# Check companion post links
for post in docs/_posts/*human*.md; do
    day=$(grep "day:" "$post" | cut -d' ' -f2)
    claude_post=$(find docs/_posts -name "*claude*day-$day-*.md")
    if [ -z "$claude_post" ]; then
        echo "Missing Claude post for day $day"
    fi
done
```

## Monthly Maintenance Tasks

### 1. Content Accuracy Review

- Technical claims match implementation
- Learning paths still make sense
- Difficulty indicators (📗📙📕) are accurate
- No outdated information

### 2. Navigation Testing

Test all user journeys:

- New visitor → Getting started
- CRUD developer → Database concepts
- Contributor → Development setup
- Learner → Rust by example

### 3. Performance Check

```bash
# Check page sizes
find docs -name "*.md" -exec wc -c {} \; | sort -n | tail -10

# Check image sizes
find docs/assets/images -type f -exec ls -lh {} \; | awk '{print $5, $9}' | sort -h
```

## FAQ-Specific Maintenance

### Daily FAQ Updates

1. **Statistics Section**
   ```markdown
   **Current Progress**: Day N of development with X lines of Rust code, Y tests, and Z blog posts
   ```
2. **Link Verification**
   - All internal links work
   - GitHub links are current
   - Navigation anchors function

### Weekly FAQ Updates

1. **Architecture Section**
   - Update ✅ for completed components
   - Match ROADMAP.md exactly
2. **Next Steps Section**
   - Priorities match ROADMAP.md
   - No fictional features

## Common Maintenance Commands

```bash
# Update all statistics at once
eval "$(get_cached_stats)"
sed -i "s/Day [0-9]* of development/Day $DAYS of development/g" docs/index.md docs/start-here.md docs/faq.md

# Find all progress sections
grep -n "Progress\|What's Working\|Currently Building\|Coming" docs/*.md

# Check for [PLANNED] tags that might need updating
grep -r "\[PLANNED\]\|\[CONCEPTUAL\]\|\[FUTURE\]" docs/ --include="*.md"

# Verify Jekyll builds successfully
cd docs && bundle exec jekyll build

# Count various metrics
echo "Database concepts: $(ls docs/database-concepts/*.md | grep -v template | wc -l)"
echo "Rust examples: $(ls docs/rust-by-example/*.md | grep -v template | wc -l)"
echo "Blog posts: $(find docs/_posts -name "*.md" | grep -v template | wc -l)"
```

## Maintenance Checklist Template

Copy and use for each maintenance session:

```markdown
## Daily Maintenance - [Date]

- [ ] Update statistics on all pages (use cached function)
- [ ] Verify progress sections match ROADMAP.md
- [ ] Update collaboration metrics on homepage
- [ ] Check for new completed features
- [ ] Verify all CTAs work

## Weekly Maintenance - [Week of]

- [ ] Validate all internal links
- [ ] Check external links (GitHub, etc.)
- [ ] Verify blog post cross-references
- [ ] Update FAQ architecture section
- [ ] Test main user journeys

## Monthly Maintenance - [Month]

- [ ] Full content accuracy review
- [ ] Update difficulty indicators
- [ ] Check page/image sizes
- [ ] Review navigation structure
- [ ] Update any [PLANNED] → implemented
```

## Troubleshooting

### Statistics don't match?

1. Clear cache: `rm /tmp/ferrisdb_stats_cache.txt`
2. Recompute: `get_cached_stats`
3. Update all pages with same values

### Jekyll build fails?

1. Check recent markdown changes
2. Verify front matter YAML
3. Look for kramdown syntax issues
4. Run: `bundle exec jekyll build --trace`

### Links broken after reorganization?

1. Use find/replace across all markdown
2. Update both source and target
3. Don't forget about anchor links (#section)

## Related Guidelines

- [Commands](commands.md) - Development commands reference
- [Website Design](../content/website-design.md) - Design standards
- [Blogging](../content/blogging.md) - Blog post guidelines
