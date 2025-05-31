# Tutorial Guidelines

Comprehensive guidelines for creating FerrisDB's "Learn by Building" tutorial series, where CRUD developers learn database internals by building FerrisDB from scratch.

**Purpose**: Teach database concepts and Rust programming through hands-on, incremental tutorials that build real FerrisDB components.

## Tutorial Philosophy

### Core Principles

1. **One Component at a Time**: Each tutorial builds a complete, working component
2. **One Rust Concept per Step**: Never overwhelm with multiple new concepts
3. **Test Everything**: Each step includes tests to prove understanding
4. **Compare to Familiar Languages**: Use JS/Python/Java/Go analogies
5. **Build Real Code**: Final result must match actual FerrisDB implementation
6. **Celebrate Progress**: Make learning feel rewarding and achievable

### Target Audience

- **Primary**: CRUD developers (web developers) with no systems programming experience
- **Languages Known**: JavaScript, Python, Java, or Go (assume proficiency in at least one)
- **Database Knowledge**: Basic SQL, used ORMs, but never built a database
- **Goal**: Understand how databases work internally while learning Rust

## Tutorial Structure

### Required Sections

Every tutorial MUST include these sections in order:

#### 1. Opening Hook

```mdx
## What We're Building Today

[Clear, visual explanation with diagram if helpful]

### The Real-World Problem

[Relatable scenario from web development - e-commerce, sessions, etc.]

### What You'll Learn

<CardGrid>
  <Card title="🦀 New Rust Concepts">
    - List 3-5 concepts
  </Card>
  
  <Card title="📚 Database Knowledge">
    - List 2-3 concepts
  </Card>
</CardGrid>
```

#### 2. Prerequisites & Setup

```mdx
## Prerequisites

<Card title="Before You Start">

**Required Tutorials**: [List with links]

**Concepts You Already Know**:
- Rust: [From tracking system]
- Database: [From tracking system]

**Time Needed**: ~X minutes

</Card>

## Setup

[Simple setup instructions]
```

#### 3. Step-by-Step Building

Each step follows this pattern:

```mdx
### Step N: [Clear Goal]

[Explanation of what and why]

<Tabs>
  <TabItem label="Write This Code">
    ```rust
    // Code to write
    ```
  </TabItem>
  
  <TabItem label="Understanding the Code">
    // Line-by-line breakdown
  </TabItem>
  
  <TabItem label="If You Know JavaScript">
    ```javascript
    // JS equivalent
    ```
    
    **Key differences**:
    - [Difference 1]
    - [Difference 2]
  </TabItem>
  
  <TabItem label="If You Know Python">
    // Similar pattern
  </TabItem>
</Tabs>

<Aside type="note" title="🦀 New Rust Concept: [Name]">

[Clear explanation using web dev analogies]

**Think of it like**: [Familiar comparison]

**Why Rust does this**: [Brief benefit]

📖 **Learn more**: [Official Rust Book link]
</Aside>

#### Test What We Built

```rust
#[test]
fn test_step_n() {
    // Test code
}
```

Run it:
```bash
cargo test test_step_n
```

<Aside type="tip" title="✅ Success!">
[Celebration of what they accomplished]
</Aside>
```

#### 4. Real FerrisDB Comparison

```mdx
### Comparing with Real FerrisDB

<Tabs>
  <TabItem label="Our Tutorial Code">
    // Simplified version
  </TabItem>
  
  <TabItem label="Real FerrisDB Code">
    // ferrisdb-storage/src/[path]:[lines]
    // Actual implementation
  </TabItem>
  
  <TabItem label="Key Differences">
    // Explain what's added for production
  </TabItem>
</Tabs>
```

#### 5. Celebration & Next Steps

```mdx
## 🎉 Congratulations!

### What You Built
- ✅ [Specific achievement 1]
- ✅ [Specific achievement 2]

### Rust Concepts You Mastered
- 🦀 **[Concept]**: [What they can now do]

### Database Knowledge You Gained
- 📚 **[Concept]**: [Why it matters]

## Next Steps

<CardGrid>
  <Card title="Ready for More?">
    **Next Tutorial**: [Link]
  </Card>
  
  <Card title="Practice Challenges">
    1. [Challenge 1]
    2. [Challenge 2]
  </Card>
</CardGrid>
```

## Content Guidelines

### Language and Tone

- **Friendly and Encouraging**: "Let's build", "Great job!", "You've successfully..."
- **Clear and Direct**: No jargon without explanation
- **Acknowledge Difficulty**: "This might seem complex, but..."
- **Celebrate Small Wins**: Emphasize progress at each step

### Navigation and Cross-References

#### Connecting Tutorials

When creating a new tutorial, you MUST:

1. **Update Previous Tutorial**: Change its "Next Steps" section to link to your new tutorial
2. **Update Navigation**: Add to astro.config.mjs under "Learn by Building"
3. **Handle Future References**: Use playful messages for unwritten tutorials

#### Handling Unwritten Tutorials

When referencing tutorials that don't exist yet, be playful and engaging:

```mdx
<Card title="You Found Our Secret! 🤫" icon="puzzle">
  Tutorial 2 is still in stealth mode. We're adding the final touches!
  
  Drop us a star if you want us to hurry up! ⭐
</Card>
```

Other playful variations:
- "Caught us mid-build!"
- "You're ahead of us!" 
- "Still brewing in our code kitchen"
- "Coming soon to a codebase near you"

**Never** use boring placeholders like "coming soon" or "TBD" - keep the energy high!

#### Navigation Update Checklist

When publishing a new tutorial:

- [ ] Update previous tutorial's "Next Steps" with actual link
- [ ] Replace playful "secret" message with real navigation
- [ ] Add your tutorial to astro.config.mjs
- [ ] Update LEARNING-PROGRESS.md status
- [ ] Check all tutorials that might reference yours

### Code Examples

1. **Start Simple**: First version should be minimal
2. **Evolve Gradually**: Show progression, not final form
3. **Real File References**: Always include `// ferrisdb-storage/src/...`
4. **Test Everything**: Every code block should be runnable

### Concept Introduction

1. **One at a Time**: Never introduce multiple new concepts in one step
2. **Familiar First**: Always relate to JS/Python/Java/Go concepts
3. **Practical Context**: Introduce when solving real problems
4. **Official Docs**: Always link to Rust Book or official docs

### Common Pitfalls to Avoid

#### ❌ Don't Do This:
- Introduce `Result<T, E>`, `?` operator, and custom errors in one step
- Use advanced Rust features without explanation
- Show final optimized code first
- Assume systems programming knowledge
- Use database jargon without definition

#### ✅ Do This Instead:
- Introduce `Result<T, E>` first, then `?` in next step
- Build up from simple to complex
- Start with working but simple code
- Relate everything to web development
- Define terms like "durability" with examples

## MDX-Specific Guidelines

### Escaping Special Characters

MDX interprets angle brackets as HTML tags. Always escape:

- ❌ `Option<T>` in text
- ✅ `` `Option<T>` `` in text
- ✅ `Option&lt;T&gt;` in component props

### Component Usage

Required imports:
```mdx
import { Tabs, TabItem, Aside, Steps, Card, CardGrid, Badge } from '@astrojs/starlight/components';
```

Preferred components:
- **Tabs**: For language comparisons and code evolution
- **Aside**: For concept explanations and tips
- **Card/CardGrid**: For visual organization
- **Badge**: For status indicators

## Tracking System Integration

### Before Writing

1. Check `RUST-CONCEPTS-TAUGHT.md` for already-taught concepts
2. Check `DATABASE-CONCEPTS-TAUGHT.md` for covered database topics
3. Plan which new concepts to introduce (aim for 3-5 Rust, 2-3 database)

### After Writing

1. Update `RUST-CONCEPTS-TAUGHT.md` with:
   - Concepts introduced (mark with ✅)
   - Concepts reinforced from previous tutorials
   
2. Update `DATABASE-CONCEPTS-TAUGHT.md` with:
   - Database concepts introduced
   - Real-world examples used

3. Update `LEARNING-PROGRESS.md` with:
   - Tutorial status (change to Published)
   - Progress bars for concept coverage

### Metadata Format

```yaml
# Tracking metadata - MUST be kept up to date
rust_concepts_introduced:
  - "`let` bindings and immutability"
  - "`mut` keyword for mutability"
rust_concepts_reinforced:
  - "`Option<T>` (from Tutorial 1)"
database_concepts_introduced:
  - "Write-Ahead Logging: durability before performance"
database_concepts_reinforced:
  - "Key-value model (from Tutorial 1)"
```

## Tutorial Progression Plan

### Phase 1: Foundation (T1-T3)
Focus: Basic Rust, simple storage concepts

### Phase 2: Core Components (T4-T8)  
Focus: Real database structures, intermediate Rust

### Phase 3: Integration (T9-T10)
Focus: Putting it together, optimization

See `LEARNING-PROGRESS.md` for detailed curriculum.

## Quality Checklist

Before publishing any tutorial:

- [ ] **Code Correctness**
  - [ ] All code examples compile and run
  - [ ] Tests pass
  - [ ] Final code matches real FerrisDB patterns
  
- [ ] **Learning Flow**
  - [ ] Only one new Rust concept per step
  - [ ] Concepts build on previous tutorials
  - [ ] No unexplained terminology
  
- [ ] **Language Comparisons**
  - [ ] JS/Python examples are idiomatic
  - [ ] Comparisons highlight key differences
  - [ ] No language is portrayed negatively
  
- [ ] **Accessibility**
  - [ ] Real-world problem is relatable
  - [ ] Explanations use web dev analogies
  - [ ] Success is celebrated
  
- [ ] **External Links**
  - [ ] All Rust Book links are valid (test each one)
  - [ ] Documentation links point to stable versions
  - [ ] No broken links to external resources
  - [ ] Links use HTTPS where available
  
- [ ] **Tracking**
  - [ ] Metadata is complete
  - [ ] Tracking files updated
  - [ ] Prerequisites accurate

### Link Validation Tips

Common external links to verify:
- Rust Book: `https://doc.rust-lang.org/book/`
- Rust by Example: `https://doc.rust-lang.org/rust-by-example/`
- Std library docs: `https://doc.rust-lang.org/std/`
- Rustup: `https://rustup.rs`

Use a link checker tool or manually verify each link works and points to the intended content.

## Testing with Readers

### Early Feedback Process

1. **Find CRUD Developers**: 2-3 volunteers per tutorial
2. **Observe Completion**: Watch them go through tutorial
3. **Note Confusion Points**: Where do they get stuck?
4. **Iterate**: Update based on feedback

### Success Metrics

- **Completion Rate**: >80% should finish
- **Time to Complete**: Within estimated time ±20%
- **Concept Understanding**: Can explain back to you
- **Confidence Growth**: Feel ready for next tutorial

## Example Analysis: Tutorial 1 Success

What made Tutorial 1 effective:

1. **Relatable Problem**: E-commerce site needs fast data (Redis-like)
2. **Gradual Introduction**: Variables → Structs → Methods → HashMap → Option
3. **Multiple Perspectives**: JS/Python comparisons for each concept
4. **Immediate Testing**: Prove each step works
5. **Clear Progress**: From empty struct to working KV store
6. **Production Connection**: Show real FerrisDB code at end

## Common Questions

### "How much should we simplify?"

Start with the simplest possible working version. For example:
- Tutorial version: `HashMap<String, String>`
- Real version: `Arc<SkipList>` with `Vec<u8>`

Show the progression at the end.

### "What if prerequisites are too complex?"

Break into smaller tutorials. Better to have 10 clear tutorials than 5 confusing ones.

### "How do we handle errors?"

- Tutorial 1-2: Use `.unwrap()` with explanation
- Tutorial 3+: Introduce `Result<T, E>` and `?`
- Later: Custom error types

### "When to introduce concurrency?"

Not until Tutorial 7. Build solid foundation first.

## Template and Resources

- **Tutorial Template**: [templates/tutorial.mdx](templates/tutorial.mdx)
- **Tracking Files**: 
  - [RUST-CONCEPTS-TAUGHT.md](RUST-CONCEPTS-TAUGHT.md)
  - [DATABASE-CONCEPTS-TAUGHT.md](DATABASE-CONCEPTS-TAUGHT.md)
  - [LEARNING-PROGRESS.md](LEARNING-PROGRESS.md)
- **Published Example**: [Tutorial 1: Key-Value Store](/ferrisdb-docs/src/content/docs/tutorials/01-key-value-store.mdx)

## Related Guidelines

- [Website Design](website-design-starlight.md) - Overall documentation structure
- [Blogging](blogging.md) - For development journey posts
- [Markdown Standards](../development/markdown-standards.md) - Formatting rules

---

*Remember: We're teaching CRUD developers to build databases. Every decision should make that journey clearer and more achievable.*