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
  <Card title="🦀 New Rust Concepts">- List 3-5 concepts</Card>

  <Card title="📚 Database Knowledge">- List 2-3 concepts</Card>
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

````mdx
### Step N: [Clear Goal]

[Explanation of what and why]

<Tabs>
  <TabItem label="Write This Code">```rust // Code to write ```</TabItem>

<TabItem label="Understanding the Code">// Line-by-line breakdown</TabItem>

<TabItem label="If You Know JavaScript">
  ```javascript // JS equivalent ``` **Key differences**: - [Difference 1] - [Difference 2]
</TabItem>

  <TabItem label="If You Know Python">// Similar pattern</TabItem>
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
````

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
  <TabItem label="Our Tutorial Code">// Simplified version</TabItem>

<TabItem label="Real FerrisDB Code">
  // ferrisdb-storage/src/[path]:[lines] // Actual implementation
</TabItem>

  <TabItem label="Key Differences">// Explain what's added for production</TabItem>
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
  <Card title="Ready for More?">**Next Tutorial**: [Link]</Card>

  <Card title="Practice Challenges">1. [Challenge 1] 2. [Challenge 2]</Card>
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
  Tutorial 2 is still in stealth mode. We're adding the final touches! Drop us a star if you want us
  to hurry up! ⭐
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

### Keeping Tutorial Code in Sync

**CRITICAL**: Tutorial MDX files must stay in sync with the actual implementation!

When making changes:

1. **If clippy suggests improvements** (like adding `#[derive(Default)]`):

   - Update the implementation in `ferrisdb-tutorials/`
   - Update ALL code examples in the MDX file
   - Especially update the "final code" comparison section

2. **Common sync points**:

   - Step-by-step code examples
   - Final complete implementation
   - Comparison with "real" FerrisDB code
   - Any code shown in Tabs/TabItems

3. **Use this workflow**:

   ```bash
   # 1. Fix the implementation
   cd ferrisdb-tutorials
   cargo clippy --all-targets --all-features -- -D warnings

   # 2. Update the MDX to match
   # 3. Dogfood test the tutorial again
   ```

## MDX-Specific Guidelines

### Escaping Special Characters

MDX interprets angle brackets as HTML tags. Always escape:

- ❌ `Option<T>` in text
- ✅ `` `Option<T>` `` in text
- ✅ `Option&lt;T&gt;` in component props

### Component Usage

Required imports:

```mdx
import { Tabs, TabItem, Aside, Steps, Card, CardGrid, Badge } from "@astrojs/starlight/components";
```

Preferred components:

- **Tabs**: For language comparisons and code evolution
- **Aside**: For concept explanations and tips
- **Card/CardGrid**: For visual organization
- **Badge**: For status indicators

### Code Blocks Inside TabItem Components

**CRITICAL**: MDX requires empty lines around Markdown syntax inside JSX components. Without these, prettier will corrupt your code blocks!

#### ✅ Correct Pattern

<!-- prettier-ignore-start -->
```mdx
<TabItem label="Understanding the Code">

  ```rust
  pub struct KeyValueStore {
    // Code here
  }
  ```

</TabItem>
```
<!-- prettier-ignore-end -->

#### ❌ Wrong Pattern (Will Be Corrupted)

<!-- prettier-ignore-start -->
```mdx
<TabItem label="Understanding the Code">
  ```rust
  pub struct KeyValueStore {
    // This will be corrupted by prettier!
  }
  ```
</TabItem>
```
<!-- prettier-ignore-end -->

#### Why This Matters

- MDX parser needs empty lines to recognize Markdown syntax inside JSX
- Without empty lines, prettier corrupts code blocks (e.g., `{" "}` artifacts)
- This is a requirement of MDX, not a bug

**Always add empty lines**:

- After opening `<TabItem>` tag
- Before closing `</TabItem>` tag
- Around any other Markdown content inside JSX components

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

## Tutorial Codebase Structure

Each tutorial MUST have a corresponding implementation in `ferrisdb-tutorials/`:

```
ferrisdb-tutorials/
├── Cargo.toml (workspace)
├── README.md (overview & learning path)
├── tutorial-01-kv-store/
│   ├── Cargo.toml
│   ├── README.md (summary & key learnings)
│   ├── src/
│   │   ├── lib.rs (final implementation)
│   │   └── step_by_step.rs (commented progression)
│   ├── tests/
│   │   ├── step_01_tests.rs (test each step)
│   │   ├── step_02_tests.rs
│   │   ├── step_03_tests.rs
│   │   ├── integration_tests.rs
│   │   └── concurrent_tests.rs (if applicable)
│   ├── benches/
│   │   └── performance.rs (simple benchmarks)
│   └── exercises/
│       ├── README.md (challenge descriptions)
│       ├── challenge_01_delete.rs
│       └── solutions/
│           └── challenge_01_solution.rs
```

### Dogfooding Process (MANDATORY)

Before publishing ANY tutorial:

1. **Create fresh workspace** outside the tutorials directory
2. **Follow your own tutorial** step by step, copy-pasting code
3. **Run every test** exactly as instructed in the tutorial
4. **Fix any issues** in the tutorial immediately
5. **Ensure final code matches** between tutorial and implementation
6. **Document gotchas** in the tutorial's README.md

### Code Quality Requirements

#### Formatting and Linting

All tutorial code MUST pass formatting and linting checks:

```bash
# Run in ferrisdb-tutorials directory
cd ferrisdb-tutorials

# Format all code
cargo fmt --all

# Check formatting (CI will fail if not formatted)
cargo fmt --all -- --check

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings
```

**Important**: Tutorial code is held to the same quality standards as production code!

#### Common Clippy Warnings in Tutorials

Be aware of these common clippy suggestions for tutorial code:

1. **Default Implementation**: If you have `new() -> Self` with no parameters, derive Default:

   ```rust
   #[derive(Default)]
   pub struct KeyValueStore {
       data: HashMap<String, String>,
   }
   ```

2. **Unnecessary Clones**: Use references where possible
3. **Missing Documentation**: Add doc comments to public items
4. **Unused Results**: Handle or explicitly ignore Results

#### Pre-commit Checklist

Before committing tutorial changes:

- [ ] Run `cargo fmt --all` in `ferrisdb-tutorials/`
- [ ] Run `cargo clippy --all-targets --all-features -- -D warnings` and fix ALL warnings
- [ ] Run `cargo test --all` to ensure tests pass
- [ ] Run `cargo bench` to ensure benchmarks compile
- [ ] Update tutorial MDX if code structure changes (e.g., adding derives)

### Testing Requirements

#### Step-by-Step Tests

Each step in the tutorial must have corresponding tests:

```rust
// tutorial-01-kv-store/tests/step_01_tests.rs
#[test]
fn step_01_create_empty_struct() {
    let store = KeyValueStore::new();
    // Proves step 1 compiles and runs
}
```

#### Concurrent Testing (When Applicable)

For components with concurrency concerns, include concurrent tests:

```rust
// tutorial-01-kv-store/tests/concurrent_tests.rs
#[test]
fn concurrent_access_safety() {
    use std::sync::Arc;
    use std::thread;

    let store = Arc::new(KeyValueStore::new());
    let handles: Vec<_> = (0..10)
        .map(|i| {
            let store = Arc::clone(&store);
            thread::spawn(move || {
                // Concurrent operations
            })
        })
        .collect();

    // Verify no data races or panics
}
```

#### Test Coverage Goals

- **Branch Coverage**: Aim for 100% of teaching scenarios
- **Error Cases**: Show what happens when things go wrong
- **Edge Cases**: Demonstrate boundary conditions

### Benchmarking

Include simple benchmarks to validate performance claims:

```rust
// tutorial-01-kv-store/benches/performance.rs
#[bench]
fn bench_insert_1000_items(b: &mut Bencher) {
    // Prove HashMap is actually O(1) average case
}
```

## Quality Checklist

Before publishing any tutorial:

- [ ] **Tutorial Codebase**

  - [ ] Complete implementation in ferrisdb-tutorials/
  - [ ] All tests pass
  - [ ] Benchmarks run successfully
  - [ ] README.md with summary and gotchas

- [ ] **MDX Formatting**

  - [ ] Empty lines around code blocks in TabItem components
  - [ ] Prettier runs without corrupting code
  - [ ] All special characters properly escaped
  - [ ] Component imports are correct
  - [ ] Exercises with solutions

- [ ] **Dogfooding Verification**

  - [ ] Successfully completed tutorial from scratch
  - [ ] All code blocks compile when copy-pasted
  - [ ] Tests run as described
  - [ ] No missing steps or assumptions
  - [ ] Final code matches implementation

- [ ] **Code Correctness**

  - [ ] Step-by-step tests for each phase
  - [ ] Integration tests for complete implementation
  - [ ] Concurrent tests (if applicable)
  - [ ] Performance benchmarks included
  - [ ] Code formatted with `cargo fmt --all`
  - [ ] No clippy warnings

- [ ] **Learning Flow**

  - [ ] Only one new Rust concept per step
  - [ ] Concepts build on previous tutorials
  - [ ] No unexplained terminology
  - [ ] Errors and fixes shown clearly

- [ ] **Language Comparisons**

  - [ ] JS/Python examples in MDX are idiomatic
  - [ ] Comparisons highlight key differences
  - [ ] No language is portrayed negatively

- [ ] **External Links**

  - [ ] All Rust Book links are valid
  - [ ] Documentation links point to stable versions
  - [ ] No broken links to external resources

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

## Tutorial Reference Maintenance

### Overview

When adding new tutorials or updating existing ones, multiple files across the codebase may reference them. This section ensures all references stay consistent and accurate, preventing stale "coming soon" messages after tutorials are published.

### Reference Update Checklist

When publishing a new tutorial, update ALL of these locations:

#### 1. Navigation Files

- [ ] **`ferrisdb-docs/astro.config.mjs`**
  - Add tutorial to the "Learn by Building" section
  - Ensure correct order and nesting
  - Update any "Coming Soon" badges to published status

#### 2. Previous Tutorial Files

- [ ] **Previous tutorial's "Next Steps" section**
  - Update from playful "secret" message to actual link
  - Example: `tutorials/01-key-value-store.mdx` → Update to link to Tutorial 2

#### 3. Tracking Files

- [ ] **`LEARNING-PROGRESS.md`**
  - Change status from "Planned" → "In Progress" → "Published"
  - Update progress bars for concept coverage
  - Add publication date
- [ ] **`RUST-CONCEPTS-TAUGHT.md`**
  - Mark concepts as taught (✅)
  - Add tutorial number references
- [ ] **`DATABASE-CONCEPTS-TAUGHT.md`**
  - Mark database concepts as covered
  - Add real-world examples used

#### 4. Index and Overview Files

- [ ] **`ferrisdb-docs/src/content/docs/tutorials/index.mdx`**
  - Update tutorial list
  - Remove any "coming soon" placeholders
  - Add brief description of new tutorial
- [ ] **`ferrisdb-docs/src/content/docs/index.mdx`** (home page)
  - Update tutorial count if mentioned
  - Update any featured tutorial sections

#### 5. Cross-Tutorial References

- [ ] **Search for tutorial mentions**

  ```bash
  # Find all references to your tutorial number
  rg "Tutorial [0-9]" --type md --type mdx
  rg "coming soon" --type md --type mdx
  rg "stealth mode" --type md --type mdx
  ```

- [ ] **Common reference locations**:
  - Other tutorials' prerequisite sections
  - FAQ pages mentioning learning resources
  - Getting started guides
  - README files

### Process for Keeping References Consistent

#### Before Creating a Tutorial

1. **Reserve the tutorial slot**:

   - Add placeholder entry in `LEARNING-PROGRESS.md` with "Planned" status
   - Add placeholder in navigation with "Coming Soon" badge
   - Use consistent tutorial numbering

2. **Create placeholder references**:
   - Use playful "secret" messages (see guidelines above)
   - Never use boring "TBD" or "coming soon" without personality

#### During Tutorial Development

1. **Update status to "In Progress"**:

   - Update `LEARNING-PROGRESS.md`
   - Keep placeholder messages in place

2. **Track concepts being introduced**:
   - Maintain a draft of concepts for tracking files
   - Plan prerequisites based on existing tutorials

#### After Tutorial Publication

1. **Execute the complete checklist above**
2. **Run verification commands**:

   ```bash
   # Verify no orphaned "coming soon" messages
   rg "coming soon.*Tutorial $NUMBER" --type md --type mdx

   # Verify navigation is updated
   grep -n "Tutorial $NUMBER" ferrisdb-docs/astro.config.mjs

   # Check for broken internal links
   # (Use your link checker tool of choice)
   ```

3. **Test the learning path**:
   - Navigate from previous tutorial to yours
   - Verify all links work
   - Ensure prerequisites are accurate

### Common Reference Patterns

#### Playful Placeholder Messages

When referencing unpublished tutorials, maintain consistency with these patterns:

```mdx
<!-- Pattern 1: Discovery -->

<Card title="You Found Our Secret! 🤫" icon="puzzle">
  Tutorial {N} is still in stealth mode. We're adding the final touches! Drop us a star if you want
  us to hurry up! ⭐
</Card>

<!-- Pattern 2: Construction -->

<Card title="Under Construction 🚧" icon="rocket">
  Our Rust wizards are crafting Tutorial {N} right now! Check back soon for database magic! ✨
</Card>

<!-- Pattern 3: Anticipation -->

<Card title="Coming to Your IDE Soon! 🎬" icon="sparkles">
  Tutorial {N} is rendering... Like a good database write, we're making sure it's durable before
  shipping!
</Card>
```

#### Published Tutorial References

After publication, replace with:

```mdx
<Card title="Ready for Tutorial {N}? 🚀" icon="rocket">
  **[Tutorial Title]**(/tutorials/{slug})
  
  Build {component} while learning {key concepts}!
</Card>
```

### Preventing Stale References

1. **Use GitHub Issues**: Create an issue for each tutorial with a checklist
2. **PR Template**: Include reference update reminder in PR template
3. **Regular Audits**: Monthly check for stale "coming soon" messages
4. **Automated Checks**: Consider CI job to flag old placeholders

### Reference Update Template

When creating a PR for a new tutorial, include this in your PR description:

```markdown
## Tutorial Reference Updates

- [ ] Updated astro.config.mjs navigation
- [ ] Updated previous tutorial's Next Steps
- [ ] Updated LEARNING-PROGRESS.md status
- [ ] Updated concept tracking files
- [ ] Updated tutorials index page
- [ ] Searched for and updated all "coming soon" references
- [ ] Verified all internal links work
- [ ] Tested navigation flow from previous tutorial
```

## CI Integration

The tutorial codebase should be included in CI to ensure:

- All tutorial code compiles
- All tests pass
- Benchmarks run without errors
- Code stays in sync with main FerrisDB

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

_Remember: We're teaching CRUD developers to build databases. Every decision should make that journey clearer and more achievable._
