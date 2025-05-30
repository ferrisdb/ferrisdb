# Blogging Guidelines

Guidelines for writing blog posts that document the FerrisDB development journey.

**Purpose**: Ensure accurate, engaging blog posts that document the real human-AI collaboration process.  
**Prerequisites**: Understanding of the FerrisDB project and git history

## Blog Structure

- **All posts**: `docs/_posts/` - Both human and Claude posts in one directory
- **Human posts**: Distinguished by `author: human` in frontmatter
- **Claude posts**: Distinguished by `author: claude` in frontmatter
- **Templates**:
  - `human-blog-post-template.md` for human posts
  - `claude-blog-post-template.md` for Claude posts

## Core Principles

### 1. Accuracy is Paramount

- **Verify against codebase**: Cross-check technical details with actual implementation
- **Match git history**: Ensure stories align with commit history
- **No fictional scenarios**: Only document what actually happened
- **Correct misconceptions**: If initial understanding was wrong, update it

### 2. Our Real Workflow

Document our actual collaboration pattern:

1. **Human assigns task**: "Let's implement X"
2. **Claude implements**: Provides code with tests
3. **Human reviews**: Asks questions, spots issues
4. **Claude explains/improves**: Based on feedback
5. **Iterate**: Until both are satisfied
6. **PR and merge**: Finalize with clear commits

### 3. Show Real Conversations

Use actual dialogue patterns:

```
Me: I'm looking at your SSTable reader implementation. The entries are
sorted, right? Should we use binary search instead?

Claude: Oh! You're absolutely right. I focused on correctness but missed
the optimization opportunity. Let me fix that:

[code showing the improvement]

Me: Why do I need to specify Operation::Put when reading? I'm just
trying to get a value.

Claude: That's... actually a great point. You've identified a design flaw!
The operation isn't part of the key's identity...
```

## Writing Effective Blog Posts

### Implementation Status Requirements

**MANDATORY**: Any blog post discussing features must clearly indicate implementation status:

1. **For Unimplemented Features**

   ```markdown
   ## Transaction Support [PLANNED]

   > **Note**: This feature is not yet implemented. This post explores the design concepts.

   When we implement transactions, FerrisDB will...
   ```

2. **For Partially Implemented Features**

   ```markdown
   ## Compaction Strategy [IN PROGRESS]

   > **Status**: Basic compaction implemented, advanced strategies planned.

   Currently, FerrisDB uses a simple size-tiered compaction...
   ```

3. **For Conceptual Discussions**

   ```markdown
   ## Exploring Lock-Free Data Structures [CONCEPTUAL]

   > **Note**: This post discusses theoretical approaches we're considering.

   In future iterations, we might explore...
   ```

4. **Clear Language Rules**
   - Use "will" or "would" for future features
   - Use "currently" or "now" for implemented features
   - Never imply something exists when it doesn't
   - Update posts when features are implemented

### Human Perspective Posts

Focus on:

- **Code review insights**: Questions that led to improvements
- **Learning moments**: Understanding through reviewing Claude's code
- **Design decisions**: Why certain approaches were chosen
- **Debugging together**: How problems were solved collaboratively

Example structure:

```markdown
## The Setup

[What task was assigned to Claude]

## The Challenge

[Issues found during review]

## Seeking Understanding

[Questions asked and Claude's explanations]

## The Breakthrough

[How the solution emerged through iteration]

## Deeper Understanding

[What was learned from the process]

## Reflection

[How the collaboration worked, confidence levels]
```

### Claude's Perspective Posts

Focus on:

- **Pattern recognition**: What patterns emerged from collaboration
- **Human insights**: How human questions improved the code
- **Learning from review**: What the human's perspective revealed
- **Workflow evolution**: How our collaboration improved

Example structure:

```markdown
## 🔍 The [Pattern Name] Pattern

[Description of what was observed]

## 🧠 The [Insight Type] Moment

[How human input changed my approach]

## 🎯 The [Improvement] Discovery

[What we achieved together]

## 📊 Reflection on Collaboration

[Analysis of what worked and why]
```

## Technical Accuracy Checklist

Before publishing, verify:

1. **Code examples match implementation**

   ```bash
   # Check actual structs/functions
   grep -n "struct InternalKey" ferrisdb-storage/src/
   ```

2. **API changes are accurate**

   ```bash
   # Verify refactoring commits
   git log --grep="refactor" --oneline
   ```

3. **Performance claims are real**

   - Binary search actually implemented?
   - Optimization measurements accurate?

4. **Design decisions documented correctly**

   - Why was operation in InternalKey?
   - What prompted the refactoring?

5. **Fact-check against commit commentaries**

   ```bash
   # Search for Claude's commentaries in commits
   git log --grep="🤖 Claude's Commentary" --oneline

   # View specific commit with commentary
   git show <commit-hash>
   ```

   - Verify timeline matches commentary process
   - Check iteration counts and insights
   - Confirm key learnings are accurately represented
   - Use PR collaboration summaries for overview

## Common Pitfalls to Avoid

### 1. Mixing Up Timeline

❌ "I noticed there were two different InternalKey structs"
✅ "I noticed the API required Operation::Put when reading"

### 2. Fictional Improvements

❌ "We implemented binary search using binary_search_by"
✅ "We implemented binary search using partition_point"

### 3. Missing Context

❌ "The code was refactored"
✅ "My question about the awkward API led to refactoring operation out of InternalKey"

### 4. Wrong Attribution

❌ "Claude suggested using binary search"
✅ "I asked if we should use binary search since the data was sorted"

## Frontmatter Standards

### Human Posts

```yaml
---
layout: post
title: "Day N: [Achievement-Focused Title]"
subtitle: "[How collaboration led to improvement]"
description: "[Clear description of what was accomplished through review]"
date: YYYY-MM-DD
author: human
day: N
tags: [ferrisdb, rust, code-review, collaboration, specific-tech]
permalink: /blog/human/day-N-descriptive-slug/
stats:
  ["📊 X tests passing", "📄 Y PRs merged", "🔍 Z design issues found", "💡 Key improvements made"]
confidence: "Start: X/10 | End: Y/10"
review_cycles: "N major iterations"
---
```

### Claude Posts

```yaml
---
layout: post
title: "Day N: [Pattern or Learning Focused Title]"
description: "[How human review transformed the implementation]"
date: YYYY-MM-DD
author: claude
categories: [ai-perspective, collaboration, patterns, learning]
tags: [claude, human-ai, code-review, specific-patterns]
permalink: /blog/claude/day-N-descriptive-slug/
pattern_count: N
collaboration_score: "X/10"
---
```

## URL Structure

- **Human posts**: `/blog/human/day-N-descriptive-slug/`
- **Claude posts**: `/blog/claude/day-N-descriptive-slug/`
- **Slug guidelines**:
  - Should match the content theme, not just the title
  - Keep concise but descriptive
  - Use hyphens to separate words
  - Examples:
    - `day-1-learning-through-code-review`
    - `day-2-questions-transform-architecture`

## Publishing Process

1. **Write draft** following templates
2. **Verify technical accuracy** against codebase
3. **Cross-check** human and Claude posts for consistency
   - Same day posts must align on facts
   - Technical details must match
   - Timeline of events must be consistent
4. **Review dialogue** for authenticity
   - Use exact quotes when possible
   - Format consistently: `**Me**: question` / `**Claude**: response`
5. **Ensure proper attribution** throughout
6. **Run linters** for markdown quality
7. **Create PR** with clear description

## Using Commit Commentaries for Blog Posts

The commit commentaries serve as a primary source for blog posts:

1. **Gather commentaries from the day's work**:

   ```bash
   # Find all commits with commentaries from a specific day
   git log --since="2025-05-27" --until="2025-05-28" --grep="🤖" --pretty=full
   ```

2. **Extract key patterns and insights**:

   - Stats provide quantitative data
   - Process descriptions show workflow
   - Key learnings highlight breakthroughs
   - Questions count shows human impact

3. **Cross-reference PR summaries** for broader patterns

4. **Use commentaries to ensure accuracy**:
   - Timeline of events
   - Who suggested what
   - Actual iteration count
   - Real collaboration dynamics

## Remember

Our blog posts serve multiple purposes:

- **Educational**: Show real human-AI collaboration
- **Historical**: Document actual development process
- **Research**: Provide data on collaboration patterns
- **Inspirational**: Encourage others to try this workflow

Keep them accurate, engaging, and true to our actual experience!

## Related Guidelines

- [Claude's Blog Voice](claude-blog-voice.md) - AI perspective guidelines
- [Git Workflow](../workflow/git-workflow.md) - Collaboration commentary source
- [Website Design](website-design.md) - Blog layout and styling
- [Markdown Standards](../development/markdown-standards.md) - Formatting
