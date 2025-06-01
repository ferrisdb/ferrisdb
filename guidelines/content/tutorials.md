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

### Critical Tutorial Design Principles

**MANDATORY**: These principles must guide ALL tutorial development:

1. **Keep It Simple, But Consistent**
   - Use FerrisDB's actual naming (Operation::Put, SyncMode::Normal)
   - Simplify implementation, NOT the interface
   - No complex trait gymnastics or overengineering
   - Goal: Familiarity when reading real FerrisDB code

2. **Gradual Step-by-Step Progression**
   - Each step should teach ONE thing
   - Build incrementally - no copy-paste-done
   - Show problems before solutions
   - Let students feel the pain before the fix

3. **CRUD Developers Are Smart, Not Systems Programmers**
   - They can learn binary serialization
   - They need step-by-step guidance
   - Show WHY before HOW
   - Relate to web development concepts

4. **Each Step Is a Complete Program**
   - Students can run and see results
   - Not just code snippets - working software
   - Include main.rs examples for tangible progress

5. **Teaching Through Discovery**
   ```
   Step 1: "Let's save to a file" → Simple text
   Step 2: "Oops, the program crashed!" → Show data loss
   Step 3: "How do we fix this?" → Introduce fsync
   Step 4: "It's so slow now!" → Benchmark it
   Step 5: "Let's make it configurable" → Sync modes
   Step 6: "Text files are huge!" → Show file size
   Step 7: "Let's use binary" → Teach serialization
   Step 8: "How do we detect corruption?" → Add checksums
   ```

6. **One Clean API, Not Multiple**
   - Avoid parallel APIs or dual implementations
   - Use consistent types throughout
   - Evolution, not revolution

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
fn descriptive_test_name() {
    // Test code focused on happy path + key learning
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

### Style Consistency

#### Type References

- **In running text**: ALWAYS use backticks for types with angle brackets

  - ✅ "The `Option<T>` type handles nullable values"
  - ❌ "The Option<T> type handles nullable values" (MDX will break!)
  - ✅ "The `HashMap` stores key-value pairs"
  - ✅ "Using `Result<T, E>` for error handling"

- **In bullet points**: MUST use backticks for generic types
  - ✅ "- `Option<T>`: Safe handling of nullable values"
  - ❌ "- **Option<T>**: Safe handling of nullable values" (MDX parsing error!)
  - ❌ "- Option<T>: Safe handling of nullable values" (MDX parsing error!)

#### Formatting Rules

1. **Code elements**: Always use backticks for:

   - Function names: `new()`, `insert()`
   - Keywords: `mut`, `self`, `impl`
   - Variable names: `store`, `key`, `value`

2. **Emphasis patterns**:

   - Use **bold** for concept names
   - Use _italics_ sparingly for emphasis
   - Use CAPS only for acronyms (API, CRUD)

3. **Consistency within sections**:
   - If listing Rust concepts, format them all the same way
   - If showing language comparisons, use consistent structure
   - ALWAYS escape angle brackets with backticks in ALL contexts

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

### Content Flow Requirements

#### Progressive Code Evolution

1. **Never introduce unexplained features**:

   - If using `#[derive(Default)]`, add a step explaining it
   - If adding a trait implementation, explain why
   - If changing function signatures, explain the reason

2. **Show practical usage**:

   - After building core functionality, show it in action
   - Include a `main.rs` example demonstrating real usage
   - Let learners see their code actually run

3. **Address tooling feedback**:
   - If clippy suggests improvements, make it a learning opportunity
   - Explain WHY the tool suggests changes
   - Show the idiomatic Rust way

#### Example: Introducing #[derive(Default)]

````mdx
### Step 7: Making It Clippy-Compliant

Rust has a helpful tool called `clippy` that suggests improvements.
Let's follow one of its recommendations:

<Tabs>
  <TabItem label="Add Default Implementation">
    ```rust
    #[derive(Default)]  // Add this line
    pub struct KeyValueStore {
        data: HashMap<String, String>,
    }
    ```
  </TabItem>
  
  <TabItem label="Why This Matters">
    // Explain the Default trait and Rust conventions
  </TabItem>
</Tabs>
````

### Common Pitfalls to Avoid

#### ❌ Don't Do This:

- Introduce `Result<T, E>`, `?` operator, and custom errors in one step
- Use advanced Rust features without explanation
- Show final optimized code first
- Assume systems programming knowledge
- Use database jargon without definition
- Write generic types without backticks (e.g., Option<T> instead of `Option<T>`)

#### ✅ Do This Instead:

- Introduce `Result<T, E>` first, then `?` in next step
- Build up from simple to complex
- Start with working but simple code
- Relate everything to web development
- Define terms like "durability" with examples
- Always use backticks for generic types to prevent MDX parsing errors

### Avoiding Tutorial Overengineering

**CRITICAL**: Tutorials must teach, not showcase clever code!

#### Signs of Overengineering:

1. **Multiple Parallel APIs**
   ```rust
   // ❌ DON'T: Create dual APIs
   impl WAL {
       pub fn append(&mut self, op: Operation) { }      // String API
       pub fn append_entry(&mut self, entry: WALEntry) { } // Binary API
   }
   ```

2. **Unnecessary Trait Implementations**
   ```rust
   // ❌ DON'T: Add traits just to match internal code
   impl From<OperationType> for u8 { }
   impl TryFrom<u8> for OperationType { }
   ```

3. **Complex Type Gymnastics**
   ```rust
   // ❌ DON'T: Overcomplicate for "consistency"
   pub struct LogEntry { operation: Operation }
   pub struct WALEntry { operation: OperationType }
   // Two different operation types? Why?!
   ```

#### Keep It Teaching-Focused:

1. **One Clear API**
   ```rust
   // ✅ DO: Single, simple interface
   pub enum Operation {
       Put { key: String, value: String },
       Delete { key: String },
   }
   ```

2. **Progressive Enhancement**
   ```rust
   // Step 1: Start with strings
   pub struct WAL {
       file: File,
   }
   
   // Step N: Evolve to binary (explain why!)
   // Show the transformation, not both at once
   ```

3. **Explain Every Addition**
   - Why add this type?
   - What problem does it solve?
   - How does it help the student?

### Code Synchronization Requirements

**CRITICAL**: Tutorial MDX files must stay in sync with the actual implementation!

#### Mandatory Cross-Check Process

**Before every commit**, you MUST verify that tutorial content matches implementation:

1. **Automated Verification Commands**:

   ````bash
   # In ferrisdb-tutorials/tutorial-XX-name/

   # Extract all Rust code blocks from MDX
   grep -A 20 '```rust' ../../docs/src/content/docs/tutorials/XX-name.mdx

   # Compare with actual implementation
   diff <(cat src/lib.rs) <(extracted_code_from_mdx)

   # Verify all imports and method signatures match
   ````

2. **Critical Sync Points to Check**:

   - **Method signatures**: Parameter types, return types, mutability
   - **Struct definitions**: Field names, types, derives
   - **Import statements**: Ensure all `use` statements are consistent
   - **Code progression**: Each step's code must build correctly
   - **Final implementation**: Must be identical to `src/lib.rs`
   - **Test examples**: Function names and assertions must match actual tests

3. **Common Drift Scenarios**:

   ```rust
   // ❌ Tutorial shows this:
   pub fn get(&self, key: &str) -> Option<String> {
       self.data.get(key).cloned()
   }

   // ✅ But implementation has this:
   pub fn get(&self, key: &str) -> Option<String> {
       self.data.get(key).cloned()
   }

   pub fn len(&self) -> usize {
       self.data.len()  // Missing from tutorial!
   }
   ```

#### Synchronization Workflow

**When making ANY changes to tutorial implementations:**

1. **Update Implementation First**:

   ```bash
   cd ferrisdb-tutorials/tutorial-XX-name/

   # Make changes to src/lib.rs
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test --all
   ```

2. **Extract Exact Code for MDX**:

   ```bash
   # Copy EXACT method signatures and implementations
   # Don't paraphrase or summarize - use identical code
   ```

3. **Update ALL References in MDX**:

   - Step-by-step code examples
   - Complete implementation sections
   - Code comparison tabs
   - Method signature examples
   - Import statements

4. **Cross-Verification**:

   ```bash
   # Test that tutorial code actually compiles
   cd /tmp && mkdir tutorial-test
   cd tutorial-test

   # Follow your own tutorial step-by-step
   # Copy-paste each code block exactly as shown
   # Verify each step compiles and runs
   ```

#### Automated Sync Verification Tools

**Create these helper scripts in your tutorial directory**:

```bash
#!/bin/bash
# scripts/verify-sync.sh

# Extract struct definition from implementation
impl_struct=$(grep -A 10 "pub struct" src/lib.rs)

# Extract struct definition from MDX
mdx_struct=$(grep -A 10 "pub struct" ../../docs/src/content/docs/tutorials/01-key-value-store.mdx)

# Compare and highlight differences
diff <(echo "$impl_struct") <(echo "$mdx_struct") || echo "❌ Struct definitions don't match!"

# Check method signatures
echo "Verifying method signatures..."
impl_methods=$(grep "pub fn" src/lib.rs)
mdx_methods=$(grep "pub fn" ../../docs/src/content/docs/tutorials/01-key-value-store.mdx)

echo "Implementation methods:"
echo "$impl_methods"
echo "Tutorial methods:"
echo "$mdx_methods"
```

#### Sync Validation Checklist

Before committing tutorial changes:

- [ ] **Struct definitions identical**: Fields, types, derives match exactly
- [ ] **Method signatures identical**: Parameters, return types, mutability match
- [ ] **Import statements consistent**: All `use` statements match between tutorial and implementation
- [ ] **Progressive examples build correctly**: Each step compiles when following tutorial
- [ ] **Final code block identical**: Complete implementation matches `src/lib.rs`
- [ ] **Test examples match actual tests**: Function names and logic are identical
- [ ] **Code comments consistent**: Don't add extra comments in tutorial that aren't in implementation

#### Real-World Sync Issues We've Encountered

1. **Missing methods in tutorial**:

   - Implementation has `len()` and `is_empty()` methods
   - Tutorial forgot to include them
   - **Fix**: Update tutorial to show all public methods

2. **Outdated method signatures**:

   - Implementation changed from `&str` to `String` parameter
   - Tutorial still shows old signature
   - **Fix**: Systematic search-and-replace in MDX

3. **Missing derives**:

   - Implementation adds `#[derive(Default)]` for clippy
   - Tutorial doesn't show this
   - **Fix**: Add explanation step for derives

4. **Test naming mismatch**:
   - Implementation uses `new_creates_empty_store`
   - Tutorial shows `test_create_store`
   - **Fix**: Update tutorial to use exact test names

#### Integration with CI/CD

**Recommended CI check**:

```yaml
# .github/workflows/tutorial-sync.yml
- name: Verify Tutorial Code Sync
  run: |
    cd ferrisdb-tutorials
    for tutorial in tutorial-*/; do
      echo "Checking $tutorial"
      cd "$tutorial"
      # Run sync verification script
      bash scripts/verify-sync.sh
      cd ..
    done
```

#### Emergency Sync Recovery

**If you discover tutorial/implementation drift:**

1. **Identify source of truth**: Usually the implementation in `ferrisdb-tutorials/`
2. **Update tutorial systematically**: Go through each code block in MDX
3. **Re-dogfood the tutorial**: Follow it step by step to verify
4. **Add sync verification**: Implement checks to prevent future drift

**Remember**: A tutorial with wrong code is worse than no tutorial - it teaches incorrect patterns and breaks user trust!

## MDX-Specific Guidelines

<Aside type="caution" title="🚨 Most Common MDX Errors">

**Top MDX build failures in tutorials:**

1. **Writing `Option<T>` without backticks** - MDX interprets `<T>` as HTML. Use: `` `Option<T>` ``

2. **Inline comments in TabItem code blocks** - Use separate lines for comments in bash blocks inside JSX components

3. **Missing empty lines around Markdown in JSX** - Always add empty lines around code blocks, badges, etc. in TabItem/Card components

These cause 90% of Starlight build failures!

</Aside>

### Escaping Special Characters

**CRITICAL**: MDX interprets angle brackets as HTML tags. You MUST escape generic types!

#### In Running Text

- ❌ `Option<T>` (MDX will try to parse as HTML tag!)
- ✅ `` `Option<T>` `` (backticks escape the angle brackets)
- ✅ `Option&lt;T&gt;` (HTML entities, but prefer backticks)

#### In Component Props

- ❌ `<Card title="Option<T> type">` (will break!)
- ✅ `<Card title="Option&lt;T&gt; type">` (HTML escape)
- ✅ Better: Avoid generics in props when possible

#### In Bold/Italic Text

- ❌ `**Option<T>**` (MDX parses `<T>` as JSX tag!)
- ✅ `**Option&lt;T&gt;**` (HTML entities required)
- ✅ `**`Option<T>`**` (backticks inside bold)
- ✅ Better: Use backticks without bold: `` `Option<T>` ``

#### Common Generic Types to Escape

- `Result<T, E>` → `` `Result<T, E>` ``
- `Vec<String>` → `` `Vec<String>` ``
- `HashMap<K, V>` → `` `HashMap<K, V>` ``
- `Arc<Mutex<T>>` → `` `Arc<Mutex<T>>` ``
- `Option<&str>` → `` `Option<&str>` ``

### Component Usage

Required imports:

```mdx
import { Tabs, TabItem, Aside, Steps, Card, CardGrid, Badge } from "@astrojs/starlight/components";

;
```

Preferred components:

- **Tabs**: For language comparisons and code evolution
- **Aside**: For concept explanations and tips
- **Card/CardGrid**: For visual organization
- **Badge**: For status indicators

### Code Blocks Inside TabItem Components

**CRITICAL**: MDX requires empty lines around Markdown syntax inside JSX components. Without these, prettier will corrupt your code blocks!

#### ✅ Correct Pattern

````mdx
<TabItem label="Understanding the Code">

```rust
pub struct KeyValueStore {
  // Code here
}
```

</TabItem>
````

#### ❌ Wrong Pattern (Will Be Corrupted)

````mdx
<TabItem label="Understanding the Code">
  ```rust pub struct KeyValueStore{" "}
  {
    // This will be corrupted by prettier!
  }
  ```
</TabItem>
````

#### Why This Matters

- MDX parser needs empty lines to recognize Markdown syntax inside JSX
- Without empty lines, prettier corrupts code blocks (e.g., `{" "}` artifacts)
- This is a requirement of MDX, not a bug

**Always add empty lines**:

- After opening `<TabItem>` tag
- Before closing `</TabItem>` tag
- Around any other Markdown content inside JSX components

#### 🚨 Inline Comments in Code Blocks

**CRITICAL**: Never use inline comments in bash code blocks inside TabItem components!

##### ❌ Wrong Pattern (Will Break Build)

<!-- prettier-ignore-start -->
````mdx
<TabItem label="Commands">
  ```bash # This comment breaks MDX parsing
  cat file.rs # Another comment that corrupts the block
  cargo test # This also fails
  ```
</TabItem>
````
<!-- prettier-ignore-end -->

##### ✅ Correct Pattern

````mdx
<TabItem label="Commands">

```bash
# This comment is on its own line
cat file.rs

# Another comment, properly separated
cargo test

# This works correctly
```

</TabItem>
````

**Why this breaks**: MDX interprets inline comments in code blocks as JSX expressions when inside components, corrupting the entire code block structure.

**Always use**:

- Comments on separate lines in code blocks
- Empty lines around code blocks in JSX components
- Proper line separation for all bash commands

### Bullet Points in Card Components

**CRITICAL**: Bullet points inside Card components require proper line breaks to render correctly.

#### ✅ Correct Pattern

```mdx
<Card title="🦀 New Rust Concepts" icon="code">
  - **Variables**: How Rust handles data - **Mutability**: When data can change - **Structs**:
  Creating custom types - **HashMap**: Rust's key-value collection - **Option**: Handling missing
  values
</Card>
```

#### ❌ Wrong Pattern (Will Collapse)

```mdx
<Card title="🦀 New Rust Concepts" icon="code">
  - **Variables**: How Rust handles data - **Mutability**: When data can change - **Structs**:
  Creating custom types
</Card>
```

**Why This Happens**: Without line breaks, MDX treats the content as a single line, collapsing bullet points together.

### Steps Component Requirements

**CRITICAL**: Steps components in Starlight must contain only a single ordered list structure.

#### ✅ Correct Pattern

```mdx
<Steps>
1. **First Step**
   Content for step 1 with proper indentation

2. **Second Step**  
   Content for step 2

3. **Third Step**
   Content for step 3

{/* prettier-ignore */}
</Steps>
```

#### ❌ Wrong Pattern (Will Break Build)

```mdx
<Steps>

### Step 1: Heading Format

Content here

### Step 2: Another Heading

More content

</Steps>
```

**Common Issues**:

- Using `###` headings instead of numbered lists
- Indenting the closing `</Steps>` tag within a list item (prettier does this automatically!)
- Including multiple child elements instead of single ordered list
- Forgetting the `{/* prettier-ignore */}` comment before closing tag

**Why the prettier-ignore comment is required**:

- Prettier automatically indents the `</Steps>` closing tag to match list content
- This breaks MDX parsing because the tag is no longer at the correct scope level
- The `{/* prettier-ignore */}` comment prevents prettier from reformatting the closing tag

**Always verify** Steps components by running `npm run build` in docs!

### Tab Usage Strategy

#### When to Use Tabs

- **Language comparisons**: Show equivalent code in JS/Python/Go
- **Alternative perspectives**: "Write This Code" vs "Understanding the Code"
- **Before/after comparisons**: Show code evolution
- **Real vs tutorial code**: Compare simplified with production versions

#### When NOT to Use Tabs

- **Single code block**: Just show the code directly
- **Simple explanations**: Use an Aside instead
- **Early tutorial steps**: Reduce cognitive load for beginners

#### Alternative to Heavy Tab Usage

````mdx
// Instead of tabs for every explanation:

```rust
pub fn set(&mut self, key: String, value: String) {
    self.data.insert(key, value);
}
```
````

<Aside type="tip" title="Understanding &mut self" collapsible>
  In Rust, `&mut self` means we're borrowing `self` mutably...
</Aside>
```

This creates a cleaner reading flow while keeping explanations accessible.

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
│   │   └── main.rs (if tutorial includes one)
│   ├── tests/                        # CI tests these ✅
│   │   ├── step_01_tests.rs (test each step)
│   │   ├── step_02_tests.rs
│   │   ├── step_03_tests.rs
│   │   ├── integration_tests.rs
│   │   ├── concurrent_tests.rs (if applicable)
│   │   └── solutions.rs (runs all solutions)
│   ├── benches/
│   │   └── performance.rs (simple benchmarks)
│   ├── examples/                     # CI ignores these ❌
│   │   ├── exercises.rs (manual test runner)
│   │   └── exercises/
│   │       ├── README.md (challenge descriptions)
│   │       ├── challenge_01_delete.rs
│   │       ├── challenge_02_ttl.rs
│   │       ├── challenge_03_case_insensitive.rs
│   │       ├── challenge_04_prefix_scan.rs
│   │       └── solutions/
│   │           ├── challenge_01_solution.rs
│   │           ├── challenge_02_solution.rs
│   │           ├── challenge_03_solution.rs
│   │           └── challenge_04_solution.rs
│   └── scripts/
│       └── verify-sync.sh (code sync verification)
```

### CI-Friendly Structure

**Key Design**: Exercise templates are in `examples/` to avoid CI failures while remaining accessible to students.

#### Directory Purpose

- **`tests/`** - CI tests these automatically ✅

  - Step tests, integration tests, solution tests
  - Must pass with 100% success rate
  - No `todo!()` or stub implementations

- **`examples/`** - CI ignores these by default ❌
  - Exercise templates with `todo!()` for students
  - Run manually: `cargo test --example exercises`
  - Students can still run and debug their implementations

#### Student Experience

Students can run all the same commands as before:

```bash
# Test main implementation
cargo test --tests

# Test their exercise solutions
cargo test --example exercises

# Check exercise compilation
cargo check --example exercises
```

The structure is transparent to students while preventing CI failures.

### Exercise Design Guidelines

When creating tutorial exercises:

1. **100% Test Coverage**: Every public method must have comprehensive tests

   - Target 100% coverage - tutorials should exemplify best practices
   - Don't add methods without purpose or tests
   - Unused methods confuse learners and trigger warnings
   - Test all edge cases and error conditions

2. **Progressive Difficulty**: Order challenges from simple to complex

   - Challenge 1: Simple method addition (e.g., delete)
   - Challenge 2: Modify internal structure (e.g., TTL)
   - Challenge 3: Change behavior patterns (e.g., case-insensitive)
   - Challenge 4: Add complex features (e.g., prefix scanning)

3. **Exercise Template Standards**:

   **CRITICAL**: Exercise starter code must compile cleanly but may have warnings:

   ```rust
   // ✅ ACCEPTABLE: Compiles with unused warnings
   pub fn method_name(&mut self, key: &str) -> Option<String> {
       // TODO: Implement the delete method
       // Hint: HashMap has a remove() method that returns Option<V>
       todo!() // or return None
   }
   ```

   **Exercise Template Requirements**:

   - ✅ **Must compile**: `cargo build` succeeds
   - ✅ **Warnings OK**: Unused variables/imports expected in templates
   - ✅ **Include helpful TODOs**: Guide students with specific hints
   - ❌ **Must not panic**: Tests should fail gracefully with `todo!()` or sensible defaults
   - ❌ **Must not have compilation errors**: Templates with syntax errors are unhelpful

4. **Expected Compiler Warnings in Exercise Templates**:

   The following warnings are **expected and acceptable** in exercise starter code:

   ```rust
   warning: unused variable: `key`
   warning: unused import: `Duration`
   warning: field `data` is never read
   warning: function is never used
   ```

   **Why we allow these**:

   - Students will use these variables when implementing
   - Imports are provided for convenience
   - Struct fields guide the correct data structure
   - Warnings teach students about Rust's helpful compiler feedback

   **Never suppress these warnings** in exercise templates - they're educational!

5. **Test Runners**: Provide test files that include all exercises
   ```rust
   // tests/exercises.rs
   #[path = "../exercises/challenge_01_delete.rs"]
   mod challenge_01_delete;
   // ... include all challenges
   ```

### Naming Consistency

**CRITICAL**: Tutorial names must be consistent across all references:

1. **Directory name**: `tutorial-01-kv-store` (kebab-case)
2. **Package name in Cargo.toml**: `tutorial-01-kv-store`
3. **Crate name in Rust**: `tutorial_01_kv_store` (underscores)
4. **Import statements**: `use tutorial_01_kv_store::KeyValueStore;`

#### Example:

If your tutorial tells users to create:

```bash
cargo new --lib tutorial-01-kv-store
```

Then imports MUST use:

```rust
use tutorial_01_kv_store::KeyValueStore;  // Correct: underscores
// NOT: use tutorial-01-kv-store::...     // Wrong: hyphens in Rust
// NOT: use ferrisdb_tutorial_01::...     // Wrong: different name
```

### Dogfooding Process (MANDATORY)

Before publishing ANY tutorial:

1. **Create fresh workspace** outside the tutorials directory
2. **Follow your own tutorial** step by step, copy-pasting code
3. **Run every test** exactly as instructed in the tutorial
4. **Fix any issues** in the tutorial immediately
5. **Ensure final code matches** between tutorial and implementation
6. **Document gotchas** in the tutorial's README.md

#### Critical Dogfooding Requirements

**IMPORTANT**: Follow the tutorial EXACTLY as a learner would:

1. **Step-by-Step Progression**:

   - Start with Step 1 and modify code incrementally
   - DO NOT jump ahead or concatenate steps
   - Verify each step compiles and runs before proceeding

2. **Test at Each Stage**:

   - After each code modification, run `cargo build`
   - Run any tests mentioned in that specific step
   - Verify expected output matches documentation

3. **Common Issues to Check**:

   - Package name consistency (e.g., `tutorial-01-kv-store` vs `tutorial_01_kv_store`)
   - Import statements match actual crate names
   - All referenced files exist (including `main.rs` if mentioned)
   - MDX syntax issues (especially `<T>` in bold text needs escaping)

4. **Repository Completeness**:
   - Ensure tutorial repo includes ALL files referenced in tutorial
   - If tutorial shows `src/main.rs`, it MUST exist in the repo
   - Update `Cargo.toml` with `[[bin]]` section if needed

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

# Run clippy on main implementations
cargo clippy --all-targets --all-features -- -D warnings
```

**Important**: Tutorial implementations are held to the same quality standards as production code!

**Exception**: Exercise template files (e.g., `challenge_01_delete.rs`) are expected to have compiler warnings and should compile but not necessarily pass all clippy checks. This is by design - students will resolve these warnings as they implement solutions.

#### Test Naming Conventions

Tutorial tests MUST follow descriptive naming that explains behavior:

```rust
// ✅ GOOD: Describes what the test verifies
#[test]
fn get_returns_none_for_missing_key() { }

#[test]
fn len_returns_count_of_unique_keys() { }

#[test]
fn scan_prefix_returns_matching_keys_sorted() { }

// ❌ BAD: Generic names that don't describe behavior
#[test]
fn test_get() { }

#[test]
fn test_len_method() { }

#[test]
fn test_basic() { }
```

**Test Name Guidelines**:

- Describe the behavior being tested, not the method name
- Use format: `method_name_expected_behavior_under_condition`
- Avoid generic `test_` prefix - it adds no value
- Make tests self-documenting through their names

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
5. **Progressive Disclosure**: When adding derives or traits (like `#[derive(Default)]`), introduce them in a dedicated step with explanation

#### Pre-commit Checklist

Before committing tutorial changes:

- [ ] Run `cargo fmt --all` in `ferrisdb-tutorials/`
- [ ] Run `cargo clippy --all-targets --all-features -- -D warnings` on **main implementations** and fix ALL warnings
- [ ] Verify **exercise templates** compile but may have expected warnings (unused variables, etc.)
- [ ] Run `cargo test --all` to ensure solution tests pass
- [ ] Run `cargo check --example exercises` to verify exercise templates compile
- [ ] Run `cargo bench` to ensure benchmarks compile
- [ ] **CRITICAL: Verify code synchronization** between MDX and implementation:
  - [ ] Extract and compare all code blocks from tutorial
  - [ ] Verify method signatures match exactly
  - [ ] Check struct definitions and derives are identical
  - [ ] Confirm test names and logic match actual tests
- [ ] Update tutorial MDX if code structure changes (e.g., adding derives)
- [ ] Ensure all code additions are explained progressively (no surprise features)

### Testing Requirements

#### Test Selection for Tutorial Content

**CRITICAL**: Not every test should be featured in the tutorial content. Choose tests strategically to maximize learning without overwhelming students.

##### Tests to Include in Tutorial Content

**1. Happy Path Tests** - Show core functionality working:

```rust
#[test]
fn set_stores_value_and_get_retrieves_it() {
    let mut store = KeyValueStore::new();

    // Basic functionality
    store.set("user:1".to_string(), "Alice".to_string());
    assert_eq!(store.get("user:1"), Some("Alice".to_string()));
}
```

**2. Concept-Teaching Tests** - Demonstrate key Rust/database concepts:

```rust
#[test]
fn get_returns_none_for_missing_keys() {
    let store = KeyValueStore::new();

    // Teaches Option<T> and None handling
    assert_eq!(store.get("nonexistent"), None);
}
```

**3. Common Edge Cases** - Cases students will likely encounter:

```rust
#[test]
fn overwrite_updates_existing_key() {
    let mut store = KeyValueStore::new();

    store.set("key".to_string(), "value1".to_string());
    store.set("key".to_string(), "value2".to_string());

    // Shows update behavior (important for understanding)
    assert_eq!(store.get("key"), Some("value2".to_string()));
}
```

**4. Behavior Validation** - Tests that prove important guarantees:

```rust
#[test]
fn len_increases_with_new_entries() {
    let mut store = KeyValueStore::new();
    assert_eq!(store.len(), 0);

    store.set("key1".to_string(), "value1".to_string());
    assert_eq!(store.len(), 1);

    // Demonstrates counting behavior
}
```

##### Tests to Keep in Background Test Files Only

**❌ Don't include these in tutorial content:**

- **Exhaustive edge cases**: Empty strings, Unicode, etc. (important for coverage, not for learning)
- **Error condition variations**: Multiple ways the same error can occur
- **Implementation details**: Tests that verify internal structure rather than behavior
- **Repetitive patterns**: If you've shown the pattern once, don't repeat

**Example of background-only test:**

```rust
// Keep this in test files, not tutorial content
#[test]
fn handles_unicode_keys_and_values() {
    // Important for robustness, but doesn't teach core concepts
}
```

##### Tutorial Test Selection Strategy

**For each step, include 1-3 tests that:**

1. **Prove the step works** (happy path)
2. **Teach the most important concept** introduced in that step
3. **Show one interesting edge case** (if relevant)

**Example progression:**

- Step 1: `new_creates_empty_store()` - proves constructor works
- Step 2: `set_stores_value_and_get_retrieves_it()` - proves core functionality
- Step 3: `get_returns_none_for_missing_keys()` - teaches Option<T>
- Step 4: `len_increases_with_new_entries()` - shows utility methods

**Background test files contain:** All comprehensive coverage including Unicode, empty strings, concurrent access, performance validation, etc.

##### Test Presentation in Tutorial

When showing tests in tutorial content:

````mdx
#### Test What We Built

```rust
#[test]
fn new_creates_empty_store() {
    let store = KeyValueStore::new();
    assert!(store.is_empty());
    assert_eq!(store.len(), 0);
}
```
````

<Aside type="tip" title="What This Test Proves">
This test verifies our constructor creates an empty store. The `assert!` macro checks boolean conditions, while `assert_eq!` compares values for equality.
</Aside>
```

**Always explain WHY the test matters** - what concept it demonstrates or behavior it proves.

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

#### Exercise Testing Strategy

Exercise templates and solutions follow a dual testing approach:

1. **Exercise Templates (`challenge_*.rs`)**:

   - Should compile without errors
   - Tests will fail with `todo!()` or return defaults
   - Warnings about unused variables/imports are expected and educational
   - Purpose: Verify structure and guide implementation

2. **Solution Implementations (`solutions/challenge_*_solution.rs`)**:
   - Must pass all tests with 100% coverage
   - Must be clippy-clean (no warnings)
   - Demonstrate correct, idiomatic Rust code
   - Purpose: Verify students can achieve working solutions

**Example Exercise Test Pattern**:

```rust
// In challenge_01_delete.rs
#[test]
fn delete_existing_key_returns_old_value() {
    let mut store = KeyValueStore::new();
    store.set("key".to_string(), "value".to_string());

    // This will panic with todo!() until student implements
    assert_eq!(store.delete("key"), Some("value".to_string()));
}
```

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
  - [ ] No inline comments in bash code blocks inside TabItem components
  - [ ] Prettier runs without corrupting code
  - [ ] All special characters properly escaped
  - [ ] Component imports are correct
  - [ ] Exercises with solutions
  - [ ] Bullet points properly formatted in Cards (with line breaks)
  - [ ] No collapsed bullet point lists in Cards

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
  - [ ] Main implementations pass clippy without warnings
  - [ ] Exercise templates compile successfully (warnings expected)
  - [ ] Solution implementations pass all tests

- [ ] **Code Synchronization (CRITICAL)**

  - [ ] All code blocks in MDX match actual implementation exactly
  - [ ] Method signatures identical between tutorial and implementation
  - [ ] Struct definitions and derives match exactly
  - [ ] Import statements consistent across all examples
  - [ ] Test function names and logic match actual test files
  - [ ] Progressive code examples build correctly when followed step-by-step
  - [ ] Final complete implementation identical to `src/lib.rs`

- [ ] **Learning Flow**

  - [ ] Only one new Rust concept per step
  - [ ] Concepts build on previous tutorials
  - [ ] No unexplained terminology
  - [ ] Errors and fixes shown clearly

- [ ] **Test Selection in Tutorial Content**

  - [ ] Featured tests follow happy path + concept-teaching criteria
  - [ ] Each step shows 1-3 carefully chosen tests (not exhaustive)
  - [ ] Background tests provide comprehensive coverage separately
  - [ ] Test explanations clarify WHY each test matters
  - [ ] No repetitive or overly technical tests in tutorial content

- [ ] **Language Comparisons**

  - [ ] JS/Python examples in MDX are idiomatic
  - [ ] Comparisons highlight key differences
  - [ ] No language is portrayed negatively

- [ ] **External Links**

  - [ ] All Rust Book links are valid
  - [ ] Documentation links point to stable versions
  - [ ] No broken links to external resources

- [ ] **Content Flow**

  - [ ] All code additions explained when introduced
  - [ ] No features appear without explanation (e.g., #[derive(Default)])
  - [ ] Practical usage examples shown (main.rs examples)
  - [ ] Progressive disclosure maintained throughout

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

- [ ] **`docs/astro.config.mjs`**
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

- [ ] **`docs/src/content/docs/tutorials/index.mdx`**
  - Update tutorial list
  - Remove any "coming soon" placeholders
  - Add brief description of new tutorial
- [ ] **`docs/src/content/docs/index.mdx`** (home page)
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
   grep -n "Tutorial $NUMBER" docs/astro.config.mjs

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
- **Published Example**: [Tutorial 1: Key-Value Store](/docs/src/content/docs/tutorials/01-key-value-store.mdx)

## Related Guidelines

- [Website Design](website-design-starlight.md) - Overall documentation structure
- [Blogging](blogging.md) - For development journey posts
- [Markdown Standards](../development/markdown-standards.md) - Formatting rules

---

_Remember: We're teaching CRUD developers to build databases. Every decision should make that journey clearer and more achievable._
