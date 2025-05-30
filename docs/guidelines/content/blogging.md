# Blogging Guidelines

Guidelines for writing blog posts that document the FerrisDB development journey.

## Regular Blog Posts

Blog posts document significant milestones, architectural decisions, and learning experiences:

- Located in `docs/_posts/` (team) and `docs/_claude_blog/` (Claude)
- Use descriptive titles and include practical insights
- Tag posts with relevant categories for easy discovery
- Write posts after major features, interesting debugging sessions, or collaboration insights
- Use templates: `docs/_posts/blog-post-template.md` for human posts, `docs/_claude_blog/blog-post-template.md` for Claude posts

## Blog Post Format Requirements

### 1. Excerpt Separator

Add `<!--more-->` after the opening paragraph to control excerpt:

```markdown
---
layout: post
title: "Your Title"
---

Opening paragraph that will appear in blog listing.

<!--more-->

## Table of contents

...
```

### 2. Metadata Display

- Use inline format for stats: `📊 55 tests • 📄 5 PRs • 🏗️ Feature built`
- Date format: `📅 Month Day, Year • 🏗️ Day N`
- Confidence levels: `☕ Confidence: Start: 3/10 | End: 6/10`
- Pattern metrics: `🔍 8 patterns • 🤝 Collaboration: 7/10`

### 3. SEO Requirements

- Always include `description:` field (150-160 characters)
- Use relevant tags and categories
- Keep titles descriptive but concise

### 4. Visual Consistency

- NO label badges - use inline text with emojis
- Consistent emoji usage throughout
- Clean, integrated appearance

## Blog Post Format (for main blog)

```yaml
---
layout: post
title: "Your Title Here"
subtitle: "Brief description of what was accomplished"
date: YYYY-MM-DD
day: N # Day number of development
tags: [tag1, tag2, tag3]
stats: ["📊 X tests passing", "📄 Y PRs merged", "⏱️ Key achievement"]
---
```

## Gathering Statistics for Blog Posts

Before writing a daily blog post, gather accurate statistics:

```bash
# Count total tests across all crates
cargo test --all --quiet 2>&1 | grep -E "test result:" | grep -oE "[0-9]+ passed" | awk '{sum += $1} END {print "Total tests: " sum}'

# List technical PRs merged on the day (adjust dates)
gh pr list --state merged --limit 50 --json number,title,mergedAt | jq -r '.[] | select(.mergedAt >= "2025-05-28T00:00:00Z" and .mergedAt < "2025-05-29T00:00:00Z") | "\(.number) - \(.title)"' | grep -E "(feat:|fix:|refactor:|perf:|test:)"

# Check current branch for recent commits
git log --oneline --since="1 day ago" --until="now"

# Verify feature completeness
grep -E "\[x\].*\(Day [0-9]+\)" TODO.md
```

## Stats Line Format

- First stat: Always include test count (e.g., "📊 55 tests passing")
- Second stat: Number of technical PRs merged (exclude docs-only PRs)
- Remaining stats: Key technical achievements of the day
- Be specific with numbers and achievements, not generic

## When to Write Blog Posts

- End of each development day (summarizing progress)
- After major architectural decisions
- When solving interesting technical challenges
- After significant refactoring or optimization work

## Making Blog Posts Engaging (Page-Turner Style)

### Create a Relatable Protagonist

You're a humble CRUD developer who never imagined building a database!

### Story Elements to Include

1. **The Hook**: Start with drama or a relatable problem

   - ❌ "Today we implemented SSTables"
   - ✅ "I stared at the failing tests, coffee cold, wondering if I'd bitten off more than I could chew..."

2. **The Struggle**: Show real challenges

   - "For three hours, I fought with Rust's borrow checker like it was my nemesis"
   - "The segfault appeared out of nowhere - my old CRUD reflexes were useless here"

3. **The AI Save**: Give Claude credit when deserved

   - "Then Claude dropped a knowledge bomb that changed everything"
   - "I was ready to give up when Claude suggested something I'd never considered"

4. **The Insight**: Share what you learned

   - "That's when it clicked - databases aren't magic, they're just really clever file management!"
   - "Who knew that 'eventual consistency' meant 'eventually I'd understand this'?"

5. **The Human Touch**: Address AI replacement fears
   - "Working with Claude proved my job is safe - AI amplifies developers, it doesn't replace them"
   - "Claude can write code, but only I can decide what code _should_ be written"

### Engagement Techniques

- **Running Jokes**: Develop recurring themes

  - "My CRUD brain vs database reality"
  - "Coffee count: 7 cups and counting..."
  - "Rust compiler: 1, Me: 0 (but I'm learning!)"

- **Pop Culture References**: Make it relatable

  - "I felt like Neo seeing the Matrix for the first time"
  - "This bug was my white whale"
  - "Claude became my Yoda in the database arts"

- **Visual Breaks**: Use emojis and formatting

  - 🎉 for victories
  - 😱 for shocking discoveries
  - 💡 for "aha!" moments
  - 🤦 for facepalm mistakes

- **Mini-Cliffhangers**: Keep readers scrolling

  - "Little did I know, this simple change would cascade into..."
  - "The solution was right there, but I wouldn't see it for another hour"
  - "And that's when everything went sideways..."

- **Relatable Comparisons**:
  - "Building a database is like assembling IKEA furniture in the dark"
  - "Debugging this was like finding a specific grain of sand on a beach"
  - "The skip list finally clicked - it's just a subway system for data!"

## Realistic Prompting Style

Based on reader feedback, the blog should show authentic human-AI collaboration through realistic prompts and responses:

### What to Include

- **Actual prompts**: Real questions a developer would ask
- **Full AI responses**: Complete technical explanations with code
- **Follow-up questions**: Natural back-and-forth clarification
- **Real errors**: Actual compilation errors and debugging sessions
- **Learning progression**: How understanding deepens through interaction

### Example Format

Instead of theatrical dialogue:
```
Me: "I'm ready for Super Saiyan Tables!"
Claude: "...it's Sorted String Table."
```

Use realistic prompting:
```
Me: I need to implement SSTables next. Can you explain the basic structure 
and what components I need to build?

Claude: SSTables (Sorted String Tables) are immutable files that store 
sorted key-value pairs. Here's what you'll need to implement:
[detailed technical explanation with code]

Me: Why do we need blocks instead of just writing all data sequentially?

Claude: Good question! Sequential writing would be simpler, but...
[explains tradeoffs with specific examples]
```

### Benefits of This Style

- **Educational**: Readers learn actual prompting techniques
- **Authentic**: Shows real human-AI interaction patterns  
- **Practical**: Demonstrates effective collaboration with LLMs
- **Relatable**: Readers recognize their own AI interactions

### What to Keep from the Original Style

- **Your personality**: Still show frustration, excitement, confusion in narrative
- **Humor**: Keep the coffee count, funny observations between prompts
- **Real struggles**: Show actual errors and debugging
- **Story arc**: Maintain the journey from confusion to understanding

## Absolute Honesty About Contributions

CRITICAL: Always accurately represent who suggested what idea or solution:

- **Credit the human**: When the human suggests an optimization or finds a bug, they get credit
- **Credit Claude**: When Claude implements or explains something, Claude gets credit
- **No role reversal**: Never swap who did what for dramatic effect
- **Verify with Claude's blog**: Cross-check stories with Claude's perspective for accuracy
- **True collaboration**: Show the real back-and-forth, not a fictional version
- **Study collaboration effectiveness**: Accurate records help us understand what makes human-AI partnerships successful

Examples:

- ✅ "I suggested binary search and Claude implemented it"
- ❌ "Claude suggested binary search" (if the human actually suggested it)
- ✅ "I noticed the API was confusing, Claude helped refactor it"
- ❌ "Claude noticed the API issue" (if the human actually noticed it)

The goal is an honest, engaging story - not fiction. Readers should trust that while the tone is fun, the facts are real.

## Why Accuracy Matters for Both Blogs

Maintaining truthful records in both human and Claude blogs is essential because:

- **Research value**: Future teams studying human-AI collaboration need accurate data
- **Pattern recognition**: We can only identify effective collaboration patterns from true events
- **Trust building**: Readers rely on our honesty to understand real vs imagined capabilities
- **Learning opportunity**: Honest mistakes and corrections teach more than fictional successes

## Template Usage

- **Human blog template** (`docs/_posts/blog-post-template.md`): For daily development posts from the human perspective
- **Claude blog template** (`docs/_claude_blog/blog-post-template.md`): For Claude's pattern-recognition focused posts
- Templates ensure consistency in structure, metrics, and personality
- Modify templates as needed but maintain the core personality traits

## Publishing Process

1. Create post using appropriate template
2. Include actual statistics gathered from commands
3. Review for accuracy and engagement
4. Lint with prettier and markdownlint
5. Submit PR with "blog" label
