# Information Architecture Guidelines

**Purpose**: Define the structure and organization of FerrisDB documentation to support our journey-focused, educational mission.

## Core Principles

1. **Journey-Centric**: Content organized around the building process, not product features
2. **Progressive Disclosure**: Start with story, progressively reveal technical depth
3. **Clear Status Indicators**: Always distinguish built vs. planned
4. **Learning Pathways**: Obvious next steps for readers

## Primary Navigation Structure

```
FerrisDB
├── Start Here                    [Gateway - Why we exist]
│   ├── Our Story                 [Hook - Human+AI building together]
│   ├── Current Status            [Honesty - What actually works]
│   └── The Vision                [Inspiration - Where we're heading]
│
├── Learn by Building             [Core Value - Educational tutorials]
│   ├── Overview                  [What you'll build]
│   ├── Tutorial 01: KV Store     [Start simple]
│   ├── Tutorial 02: Add WAL      [Add persistence]
│   ├── Tutorial 03: Skip List    [Data structures]
│   ├── Tutorial 04: SSTables     [File formats]
│   ├── Tutorial 05: LSM Tree     [Architecture emerges]
│   ├── Tutorial 06: MVCC         [Transactions]
│   └── Tutorial 07: Distribution [Scale out]
│
├── The Journey                   [Story - How we're building]
│   ├── Development Blog          [Day-by-day progress]
│   │   ├── All Posts            [Chronological]
│   │   ├── Human Perspective    [Learning & reviewing]
│   │   └── AI Perspective       [Patterns & insights]
│   ├── Collaboration Insights    [Human-AI dynamics]
│   └── Technical Decisions       [Why we chose X]
│
├── Deep Dives                    [Optional - For the curious]
│   ├── Current Implementation    [How our code works now]
│   └── Future Architecture       [Research & ideas]
│
└── Get Involved                  [Action - Join us]
    ├── GitHub                    [See the code]
    ├── Contributing              [Help build]
    └── Community                 [Discuss & learn]
```

## Content Categories

### Essential (Must Read)

- **Start Here**: Hook readers with our unique story
- **Tutorial 01**: Immediate hands-on value
- **Current Status**: Honest assessment

### Recommended (Should Read)

- **Blog Posts**: Follow the journey
- **Subsequent Tutorials**: Continue building
- **Collaboration Insights**: Understand our process

### Optional (Can Read)

- **Deep Dives**: Technical details
- **Future Architecture**: Aspirational content
- **Research Notes**: Academic context

## Page Types and Templates

### 1. Story Pages

- **Purpose**: Connect emotionally, explain why
- **Examples**: Our Story, Collaboration Insights
- **Template**: Narrative-driven, personal tone

### 2. Tutorial Pages

- **Purpose**: Hands-on learning
- **Structure**: Problem → Concept → Build → Learn
- **Requirements**: Working code, tests, exercises

### 3. Status Pages

- **Purpose**: Radical transparency
- **Examples**: Current Status, Roadmap
- **Format**: Clear feature matrix, honest limitations

### 4. Blog Posts

- **Purpose**: Document the journey
- **Types**: Human perspective, AI perspective
- **Frequency**: Regular updates showing progress

### 5. Technical Deep Dives

- **Purpose**: Satisfy curious readers
- **Placement**: Optional/advanced sections
- **Note**: Always link back to journey context

## Navigation Principles

### Primary Navigation

- Maximum 5 top-level items
- Action-oriented labels ("Learn by Building" not "Tutorials")
- Most important items first

### Secondary Navigation

- Sidebar for section navigation
- Breadcrumbs for orientation
- "Next/Previous" for sequential content

### Cross-Linking Strategy

- Every technical page links to journey context
- Tutorials reference relevant blog posts
- Status pages link to implementation

## Status Indicators

### For Features

```markdown
✅ Complete - Fully implemented and tested
🚧 In Progress - Partially implemented
📋 Planned - Not yet started
❌ Not Planned - Explicitly not doing
```

### For Content

```markdown
[CURRENT] - Reflects actual implementation
[VISION] - Future goals and plans
[EXPERIMENTAL] - Research and exploration
[DEPRECATED] - Being phased out
```

## Anti-Patterns to Avoid

1. **Product Documentation Structure**

   - ❌ Installation → Configuration → API Reference
   - ✅ Story → Learn → Build → Contribute

2. **Feature-Focused Organization**

   - ❌ Transactions / Replication / Performance
   - ✅ The Journey / Tutorials / Current Status

3. **Theory Before Practice**

   - ❌ Concepts → Examples → Exercises
   - ✅ Problem → Build Solution → Understand Concept

4. **Hidden Status**
   - ❌ Mixing planned and built features
   - ✅ Clear "Current Status" page, status badges

## URL Structure

Keep URLs meaningful and stable:

```
/                           # Home - Start Here
/our-story/                 # Why we're building
/status/                    # What works today
/tutorials/                 # Learn by Building
/tutorials/01-kv-store/     # Individual tutorials
/blog/                      # All posts
/blog/human/               # Human perspective
/blog/ai/                  # AI perspective
/deep-dives/               # Technical details
/vision/                   # Future plans
```

## Search and Discovery

### Search Optimization

- Clear page titles with keywords
- Descriptive meta descriptions
- Structured headings (H1 → H2 → H3)

### Discovery Features

- "Start Here" for new visitors
- "Learning Path" for tutorials
- "Latest Updates" for returning readers
- Related content suggestions

## Mobile Considerations

- Collapsible navigation on mobile
- Touch-friendly tutorial navigation
- Readable code blocks with horizontal scroll
- Progressive enhancement approach

## Maintenance

### Regular Reviews

- Monthly: Update Current Status page
- Per Tutorial: Update learning progress trackers
- Per PR: Update relevant documentation

### Deprecation Process

1. Mark content as [DEPRECATED]
2. Add redirect or notice
3. Update navigation
4. Archive after 3 months

## Success Metrics

Good information architecture enables:

- New visitors understand our mission in < 1 minute
- CRUD developers find clear learning path
- Readers can distinguish built vs. planned
- Journey story is prominent throughout
- Technical depth available but not required

## Related Guidelines

- [Content Strategy](content-strategy.md) - Content philosophy
- [Website Design](website-design-starlight.md) - Visual presentation
- [Tutorial Guidelines](tutorials.md) - Tutorial structure

---

_Last updated: 2025-05-31_
