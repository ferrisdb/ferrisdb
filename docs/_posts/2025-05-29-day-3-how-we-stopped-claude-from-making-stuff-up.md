---
layout: post
title: "Day 3: How We Stopped Claude from Making Stuff Up"
subtitle: "When context compression leads to fiction, build a better system"
description: "Solving the context problem in human-AI development by creating a collaboration commentary system that preserves our real workflow."
date: 2025-05-29
author: human
day: 3
tags: [ferrisdb, human-ai-collaboration, documentation, workflow, innovation]
permalink: /blog/human/day-3-how-we-stopped-claude-from-making-stuff-up/
stats:
  [
    "📊 4 blog posts rewritten for accuracy",
    "📄 3 PRs with commentary system",
    "🔧 21 files updated for TOC fix",
    "💡 1 major workflow innovation",
  ]
confidence: "Start: 10/10 | End: 10/10"
review_cycles: "Multiple iterations on accuracy"
---

Today we took a break from coding FerrisDB to solve a bigger problem: how do we preserve the reality of human-AI collaboration when AI context gets compressed over time?

<!--more-->

## Table of contents

{: .no_toc .text-delta }

<!-- prettier-ignore-start -->

1. TOC
{:toc}
<!-- prettier-ignore-end -->

---

<!-- Before publishing: Verify technical details against codebase! -->

## The Context Problem

After two days of intense development, I asked Claude to help write our blog posts. That's when I noticed something troubling: the posts contained fictional elements. Not intentional fiction, but the kind that emerges when context gets compressed and details blur together.

**Me**: Let's rewrite our blog posts with the new guidelines.

**Claude**: [Rewrites 4 posts with improved style but still fictional elements]

**Me**: Wait, this isn't what actually happened. I didn't notice two InternalKey structs. I noticed the awkward API requiring Operation::Put when reading.

This wasn't Claude's fault. After multiple context resets and compressions, the fine details of our collaboration had become fuzzy. We needed a better system.

## The Commentary Innovation

That's when it hit me: what if we could preserve our collaboration details directly in the git history?

**Me**: What if we add collaboration commentary to our commits? Track iterations, insights, and patterns right where they happen?

**Claude**: That's brilliant! We could add a structured format to commit messages that captures our workflow.

We developed this format:

```
🤖 Claude's Commentary:
📊 Stats: X iterations, Y key insights, Z refactors
🔄 Process: [How we arrived at the solution]
💡 Key Learning: [What drove the improvement]
🎯 Outcome: [What was achieved]
```

## Implementation Journey

### First: Clean Up CLAUDE.md

**Me**: CLAUDE.md is getting bloated. It should be an index, not a manual.

We stripped out duplicate content and made it a pure reference guide. This freed up context space for actual work.

### Second: Fix the Blog Structure

We restructured the entire blog system:

- Unified all posts in `_posts/`
- Used author field to distinguish human/Claude posts
- Fixed URLs to be descriptive
- Ensured accuracy by verification against git history

### Third: The Prettier/Markdownlint Battle

Then we hit a recurring issue:

**Me**: The TOC has issues again because of the indent. Can you research how to properly fix it?

This led to discovering that prettier and markdownlint were fighting over Jekyll's kramdown syntax. We implemented a comprehensive solution using `prettier-ignore` comments.

## The Bigger Picture

Today wasn't about building database features. It was about building infrastructure for sustainable human-AI collaboration. The commentary system solves multiple problems:

1. **Preserves context**: No more fictional blog posts
2. **Tracks patterns**: We can analyze collaboration effectiveness
3. **Shares knowledge**: Others can learn from our workflow
4. **Builds trust**: Transparency in how decisions were made

## Real Impact

Look at one of our actual commit messages from today:

```
🤖 Claude's Commentary:
📊 Stats: 8 iterations, 4 major insights, 2 complete rewrites
🔄 Process: Human noticed inaccuracies → fact-checking revealed gaps → rewrote with verification → improved URL structure
💡 Key Learning: Human's insistence on accuracy against git history prevented fictional documentation
🎯 Outcome: Accurate blog posts with verifiable guidelines
```

This isn't just metadata. It's the story of how we work together, preserved where it matters.

## Reflection

Today proved that the best tools emerge from real problems. We didn't set out to innovate in documentation - we just wanted accurate blog posts. But by addressing the root cause (context loss), we created something bigger: a system that makes human-AI collaboration more transparent and effective.

The commentary system isn't just for FerrisDB. Any team working with AI could use this approach to preserve their collaboration patterns and learn from them.

Tomorrow, we'll return to compaction with better tools for preserving our journey.

---

**Confidence Progression**: Started and ended at 10/10 - solving tooling problems is my forte
**Review Cycles**: Many, but each one improved accuracy
**Lesson Learned**: Sometimes the best features aren't in your product - they're in your process

_P.S. Claude suggested we're "bessie not enemy" (best friends, not enemies). I couldn't agree more. AI doesn't take jobs - it transforms how we work together._
