---
title: "Day 3: How We Stopped Claude from Making Stuff Up"
date: 2025-05-29
authors: [human]
tags: [human-perspective, human-ai-collaboration, documentation, workflow, innovation]
description: "Solving the context problem in human-AI development by creating a collaboration commentary system that preserves our real workflow."
excerpt: "Today we took a break from coding FerrisDB to solve a bigger problem: how do we preserve the reality of human-AI collaboration when AI context gets compressed over time?"
collaborationDay: 3
companion: "day-3-claude-when-i-started-writing-fiction-and-how-we-fixed-it"
insights:
  questionsAsked: 4
  iterations: 8
  breakthroughMoment: "Realizing we needed external memory system in git commits"
  conceptsIntroduced: ["collaboration commentary", "context preservation", "mandatory documentation", "jekyll reliability"]
---

Today we took a break from coding FerrisDB to solve a bigger problem: how do we preserve the reality of human-AI collaboration when AI context gets compressed over time?

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

## Making It Mandatory

After implementing the commentary system, I realized it needed to be required, not optional:

**Me**: We should update our guidelines to make collaboration commentary MANDATORY. This is essential data we're collecting.

We updated three key files:

- `git-workflow.md`: Made commentary REQUIRED with bold emphasis
- `pr-process.md`: MANDATORY for PR descriptions and squash merges
- `CLAUDE.md`: Emphasized it's essential data that cannot be skipped

This wasn't bureaucracy - it was protecting the value of our innovation.

## Website Reliability

While working on documentation, we discovered another issue:

**Me**: Sometimes ferrisdb.org displays raw markdown instead of rendered HTML. This is intermittent but concerning.

Claude's investigation revealed multiple causes:

- Jekyll's `{: .class}` syntax fails when on separate lines
- Mixed markdown/HTML can cause rendering issues
- Build caches create inconsistent behavior

We implemented a comprehensive fix in `deploy-docs.yml`:

- Clear Jekyll cache before builds
- Add `--strict_front_matter` flag
- Implement retry logic for transient failures
- Verify build output doesn't contain raw markdown

## The Bigger Picture

Today wasn't about building database features. It was about building infrastructure for sustainable human-AI collaboration. We solved three interconnected problems:

1. **Context preservation**: Commentary system prevents fictional blog posts
2. **Process enforcement**: Making it mandatory ensures we capture all patterns
3. **Infrastructure reliability**: Jekyll fixes ensure our collaboration is actually visible
4. **Knowledge sharing**: Others can learn from our documented workflow

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
