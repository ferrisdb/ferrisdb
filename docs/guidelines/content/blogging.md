# Blogging Guidelines

Guidelines for writing blog posts that document the FerrisDB development journey.

## Blog Structure

- **Human posts**: `docs/_posts/` - Daily development from human perspective
- **Claude posts**: `docs/_claude_blog/` - AI perspective on patterns and collaboration
- **Templates**: Use provided templates for consistency

## Writing Effective Blog Posts

### 1. Start with a Hook

Open with drama or a relatable problem that draws readers in:

- ✅ "I stared at the failing tests, coffee cold, wondering if I'd bitten off more than I could chew..."
- ❌ "Today we implemented SSTables"

### 2. Show Real Human-AI Collaboration

Use authentic prompting patterns that readers can learn from:

```
Me: I need to implement SSTables next. Can you explain the basic structure 
and what components I need to build?

Claude: SSTables (Sorted String Tables) are immutable files that store 
sorted key-value pairs. Here's what you'll need to implement:

1. File format with these components:
   - Data blocks (4KB each with sorted entries)
   - Index block (for binary search across blocks)
   - Footer (metadata and checksums)

[code example]

Me: Why do we need blocks instead of just writing all data sequentially?

Claude: Good question! Sequential writing would be simpler, but it creates 
problems at scale:
- To find a key in a 1GB file, you'd need to scan the entire file
- With blocks, you only read ~4KB after finding the right block
- This changes lookup from O(n) to O(log n)
```

### 3. Maintain Your Personality

Between technical exchanges, add your colorful commentary:

- Running gags (coffee count, compiler battles)
- Pop culture references that fit
- Honest reactions and emotions
- Relatable comparisons

### 4. Document the Journey

- Show compilation errors and debugging sessions
- Include "aha!" moments when concepts click
- Track confidence levels throughout
- Share what you learned in plain language

## Technical Requirements

### Frontmatter Format

```yaml
---
layout: post
title: "Day N: Catchy Title That Describes the Achievement"
subtitle: "Brief context or humor"
description: "SEO description 150-160 chars"
date: YYYY-MM-DD
day: N
tags: [tag1, tag2, tag3, tag4, tag5]
stats: ["📊 X tests passing", "📄 Y PRs merged", "🏗️ Key achievement"]
confidence: "Start: X/10 ☕ | End: Y/10 ☕☕"
compilation_attempts: "XX (optional funny note)"
---
```

### Required Elements

1. **Excerpt separator**: Add `<!--more-->` after opening paragraph
2. **Table of contents**: Include TOC after excerpt
3. **Statistics**: Gather accurate numbers before writing
4. **Attribution**: Credit ideas accurately to human or AI

## Gathering Statistics

```bash
# Count tests
cargo test --all --quiet 2>&1 | grep -E "test result:" | grep -oE "[0-9]+ passed"

# List PRs merged today
gh pr list --state merged --limit 50 --json number,title,mergedAt | \
  jq -r '.[] | select(.mergedAt >= "YYYY-MM-DD") | "\(.number) - \(.title)"'

# Recent commits
git log --oneline --since="1 day ago"
```

## Engagement Techniques

### Visual Elements
- 🎉 Victories
- 😱 Shocking discoveries  
- 💡 "Aha!" moments
- 🤦 Facepalm mistakes
- ☕ Coffee count tracker

### Narrative Devices
- Mini-cliffhangers between sections
- Relatable analogies (e.g., "like IKEA furniture assembly")
- Running jokes that develop over time
- Honest vulnerability about struggles

### Educational Value
- Show effective prompting techniques
- Explain concepts in accessible terms
- Include working code examples
- Document real errors and fixes

## Absolute Accuracy

Always maintain truthful attribution:

- If human suggested it → human gets credit
- If Claude implemented it → Claude gets credit  
- If collaboration → explain who did what
- Cross-reference with Claude's blog for consistency

This matters because:
- Builds reader trust
- Provides research value
- Shows real collaboration patterns
- Teaches authentic lessons

## Publishing Checklist

1. ✅ Used appropriate template
2. ✅ Gathered accurate statistics
3. ✅ Included realistic AI interactions
4. ✅ Maintained engaging narrative voice
5. ✅ Credited contributions accurately
6. ✅ Added SEO description
7. ✅ Ran markdown linters
8. ✅ Created PR with "blog" label

## Remember

You're writing for:
- Developers curious about AI collaboration
- People learning database internals
- Readers who enjoy technical journeys
- Future researchers studying human-AI work

Make it educational, honest, and fun!