# FerrisDB Content Strategy

**Purpose**: Define our content philosophy, principles, and structure to align with our core value propositions.

## Core Value Propositions

1. **Building a database from scratch** - A human and an AI learning database internals and Rust together
2. **Showcasing human-AI collaboration** - Pushing limits no CRUD dev imagined possible
3. **Sharing insights discovered** - Through our collaborative journey via blog posts
4. **Teaching through tutorials** - Anyone, even CRUD devs, can follow along and build a distributed database

## Content Philosophy

### Journey Over Destination

We are documenting the **process of building**, not selling a finished database. Every piece of content should connect to this journey.

### Radical Transparency

- Show real code, real mistakes, real breakthroughs
- Clearly distinguish between what's built vs. what's planned
- Document actual human-AI conversations and iterations

### Educational First

- Assume CRUD developer background, build up from there
- Learn by building, not by reading theory
- Each tutorial builds on previous knowledge

### Story-Driven Technical Content

Technical content without narrative context is boring. Every article should answer:

- Why are we building this?
- What problem does it solve?
- What did we learn along the way?

## Content Principles

### 1. Factual vs. Aspirational

**Factual Content** (must reflect reality):

- Blog posts - actual development progress
- Tutorials - working code in the repo
- Implementation deep dives - current codebase
- Status updates - what works today

**Aspirational Content** (clearly marked as future):

- Architecture vision - where we're heading
- Future explorations - what excites us
- Research notes - papers we're reading

### 2. No Premature Documentation

We do NOT create:

- Installation guides (nothing to install)
- Configuration references (nothing to configure)
- API documentation (no API yet)
- Operational guides (not deployable)
- Performance benchmarks (incomplete system)

### 3. Tutorial-Driven Learning

Instead of separate "concepts" and "tutorials", we merge them:

- Each tutorial teaches concepts through building
- Theory is introduced only when needed to solve a problem
- Real-world comparisons show how production databases work

### 4. Authentic Voice

- **Human posts**: Show learning, confusion, breakthroughs
- **AI posts**: Pattern recognition, meta-observations
- **Technical content**: Clear, engaging, never condescending

## Content Structure

### Primary Categories

1. **Start Here**

   - Our Story (why we're building this)
   - Current Status (what actually works)
   - The Vision (where we're heading)

2. **Learn by Building** (Tutorials)

   - Progressive series building a complete database
   - Each tutorial includes concepts inline
   - Hands-on with real code

3. **The Journey** (Blog & Insights)

   - Development blog (both perspectives)
   - Collaboration insights
   - Technical decisions and debates

4. **Deep Dives** (Optional Reading)

   - Current implementation details
   - Future architecture ideas
   - Research and experiments

5. **Get Involved**
   - Contributing guide
   - Community discussions

### Content Hierarchy

```
Essential (Must Read)
├── Start Here → Our Story
├── Current Status
└── Tutorial 01

Recommended (Should Read)
├── Blog Posts (The Journey)
├── Next Tutorials
└── Collaboration Insights

Optional (Can Read)
├── Deep Dives
├── Future Architecture
└── Research Notes
```

## Quality Criteria

Every piece of content must:

1. **Connect to the journey** - How does this fit in our building story?
2. **Be honest about status** - What works, what doesn't, what's planned
3. **Teach something valuable** - Database concepts, Rust, or collaboration
4. **Engage the reader** - Would a CRUD dev find this interesting?
5. **Show real code** - From our actual codebase, not toy examples

## What Makes Us Unique

Not competing with PostgreSQL or MongoDB. We're offering:

> "The only place where you can watch a CRUD developer and AI build a distributed database from scratch, learning together through real code, real mistakes, and real breakthroughs."

## Content Review Checklist

Before publishing any content:

- [ ] Does it align with our value propositions?
- [ ] Is it factual (if blog/tutorial) or clearly marked as vision?
- [ ] Does it connect to the larger journey?
- [ ] Would a CRUD developer understand it?
- [ ] Does it show real code/progress?
- [ ] Is the human-AI collaboration visible?
- [ ] Does it teach something valuable?

## Anti-Patterns to Avoid

1. **Documentation without implementation** - Don't document features that don't exist
2. **Theory without practice** - Always connect concepts to real code
3. **Perfection theater** - Show the messy reality, not an idealized version
4. **Jargon walls** - Explain systems concepts in accessible terms
5. **Missing context** - Every technical piece needs journey context

## Success Metrics

Good content will:

- Attract developers interested in learning, not using
- Generate discussions about the process, not the product
- Inspire others to build their own databases
- Showcase unprecedented human-AI collaboration
- Make database internals accessible to CRUD developers

## Related Guidelines

- [Tutorial Guidelines](tutorials.md) - Detailed tutorial structure
- [Blogging Guidelines](blogging.md) - Blog post standards
- [Website Design](website-design-starlight.md) - Visual presentation

---

_Last updated: 2025-05-31_

---
_Last updated: 2025-06-01_
