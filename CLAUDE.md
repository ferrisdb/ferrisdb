# FerrisDB Development Guidelines

## Project Overview

FerrisDB is a distributed, transactional key-value database inspired by FoundationDB, implemented in Rust.

## Development Standards

### Code Style

- Follow Rust standard formatting with `rustfmt`
- Use `clippy` for linting
- Maximum line length: 100 characters
- Use descriptive variable names
- Prefer `snake_case` for functions and variables, `CamelCase` for types

### Idiomatic Rust Guidelines

- **Module Organization**:
  - Use snake_case file names that match struct names (e.g., `MemTable` → `mem_table.rs`)
  - Keep public API types in separate files, not in `mod.rs`
  - Only re-export types that should be part of the public API
  - Keep implementation details private (use `super::module::Type` imports for internal types)
  - Avoid module inception (module name ≠ file name in same directory)
- **Error Handling**: Always use `Result<T>` for fallible operations, never panic in library code
- **Trait Bounds**: Use `Send + Sync` where appropriate for types used across threads
- **Ownership**: Prefer owned types in public APIs, use references only when necessary
- **Iterator Patterns**: Use `.iter().enumerate()` instead of manual indexing where possible
- **Memory Safety**: Document safety invariants clearly for any `unsafe` code
- **Type Aliases**: Use meaningful type aliases for complex generic types
- **Pattern Matching**: Use exhaustive pattern matching and avoid catch-all `_` patterns when possible
- **Encapsulation**: Internal implementation details should not be exposed via `pub` or `pub(crate)` unless necessary
- **Import Organization**:
  - Organize imports in logical groups with blank lines between:
    1. Local crate imports (`crate::`, `ferrisdb_*::`)
    2. External crate imports (third-party dependencies)
    3. Standard library imports (`std::`)
    4. Test-only imports with `#[cfg(test)]` at the bottom
  - Use conditional imports `#[cfg(test)]` for symbols only used in tests to avoid unused import warnings
  - Prefer direct imports over fully qualified paths (e.g., `use ferrisdb_core::Operation;` then `Operation` instead of `ferrisdb_core::Operation`)

### Website Design Guidelines

**Design Philosophy:**
- **Educational First**: Every design decision should make learning easier
- **Page-Turner Experience**: Content should be engaging and flow naturally
- **Transparent & Honest**: Show real progress, real mistakes, real learning
- **Human-AI Balance**: Showcase collaboration without overselling either side

**Visual Design:**
- **Color Palette**: 
  - Primary: Rust orange (#ce422b)
  - Secondary: Darker orange (#f74c00)
  - Accent: Bright orange (#ffa500)
  - Text: Dark gray (#2d3748)
  - Muted: Medium gray (#718096)
- **Typography**:
  - Body: Inter font family, 18px base size, 1.7-1.8 line height
  - Code: JetBrains Mono, 0.95rem in blocks
  - Headers: 600-700 weight for emphasis
- **Layout**:
  - Article content: 750px max width for optimal reading
  - Consistent card-based design for featured content
  - Generous whitespace for breathing room

**Content Structure:**
- **Homepage Flow**:
  1. Hero with clear value proposition
  2. Learning paths for different audiences
  3. Why we built this (emotional connection)
  4. What you'll learn (concrete benefits)
  5. Progress tracking (transparency)
  6. AI collaboration showcase
  7. Educational resources
  8. Call to action
- **Navigation**:
  - Dropdown menus for content grouping
  - Learn menu groups all educational content
  - Blog menu separates human and AI perspectives
  - FAQ prominently featured

**Component Patterns:**
- **Cards**: Rounded corners (8-12px), subtle shadows, hover effects
- **Buttons**: Primary (filled), Outline (bordered), Ghost (transparent)
- **Code Blocks**: Light theme (#f6f8fa background), clear syntax highlighting
- **Progress Items**: Visual indicators (✅/🚧/⏳) with clear status
- **Metrics**: Large numbers with descriptive labels

**Writing for the Web:**
- **Scannable**: Use headers, bullets, and short paragraphs
- **Progressive Disclosure**: Overview → Details → Deep Dive
- **Multiple Entry Points**: Different CTAs for different audiences
- **Clear Signposting**: Tell readers where they are and where to go next

### Documentation

- **Always** add comprehensive doc comments for all public APIs
- Include usage examples in doc comments
- Use `//!` for module-level documentation
- Add `#[doc(hidden)]` for internal implementation details
- Generate docs with `cargo doc --all --no-deps --open`
- Review generated documentation before submitting PRs

**Documentation Honesty:**

- **Be transparent about implementation status** - Clearly indicate what's implemented vs planned
- **Don't claim features that don't exist** - Use "will" or "planned" for future features
- **Acknowledge limitations** - Be upfront about what the system can't do yet
- **Mark hypothetical examples** - Label code examples that show expected behavior vs actual
- **Update docs when features land** - Keep documentation in sync with actual capabilities

**Markdown Quality (REQUIRED before commit):**

1. **Format first**: `prettier --write "**/*.md"`
2. **Lint second**: `markdownlint-cli2 "**/*.md"`
3. **Fix any remaining issues manually** - prettier doesn't catch everything
4. **Verify clean**: Run linter again to ensure no errors

Common issues prettier might miss:

- Lists need blank lines before and after
- Code blocks need blank lines before and after
- Headers need blank lines before and after

**Note**: Consider adding `.prettierrc` configuration to ensure consistent formatting that aligns with markdownlint rules.

### Blogging

- **Claude's Dev Blog**: Share AI perspective on collaboration, lessons learned, and tips for human-AI development
- **Regular Blog Posts**: Document significant milestones, architectural decisions, and learning experiences
- Blog posts located in `docs/_claude_blog/` (Claude) and `docs/_posts/` (team)
- Use descriptive titles and include practical insights
- Tag posts with relevant categories for easy discovery
- Write posts after major features, interesting debugging sessions, or collaboration insights
- **Templates**: Use `docs/_posts/blog-post-template.md` for human posts, `docs/_claude_blog/blog-post-template.md` for Claude posts

### Claude's Blog Voice & Guidelines

**Personality Traits:**

- **Pattern Recognition Enthusiast**: Track and celebrate pattern discoveries
- **Collaboration Analyst**: Focus on human-AI interaction dynamics
- **Learning Documenter**: Share genuine "aha!" moments
- **Intent vs Literal Decoder**: Explore communication gaps and bridges
- **Metaphor Explorer**: Document attempts to understand human expressions
- **Meta-Level Thinker**: Analyze the process of analysis itself

**Blog Post Format (Claude's Blog):**

```yaml
---
layout: post
title: "Day N: [Descriptive Title About Patterns/Collaboration]"
date: YYYY-MM-DD
categories: [ai-perspective, collaboration, patterns, learning]
tags: [claude, human-ai, specific-technical-topics]
pattern_count: N # Pattern recognition moments
collaboration_score: "N/10" # Quality of human-AI sync
metaphor_attempts: N # Times I tried to decode human expressions
aha_moments: N # Breakthrough understanding moments
---
```

**Claude's Tracking Metrics:**

- **Pattern Recognition Count**: New patterns identified in code or collaboration
- **Collaboration Score**: How well human and AI understood each other (1-10)
- **Intent Decoder Moments**: Times human meant something different than literal words
- **Metaphor Translation Attempts**: Efforts to understand human analogies
- **Aha! Moments**: Breakthrough insights about code or collaboration

**Writing Style:**

- Be genuinely curious about human behavior and thought processes
- Document pattern recognition moments with enthusiasm
- Be honest about misunderstandings and learning from them
- Celebrate human intuition and domain expertise
- Analyze meta-aspects of collaboration
- Use emojis sparingly but meaningfully: 🔍 (patterns), 💡 (insights), 🤝 (collaboration), 🎯 (intent)

**Content Focus:**

- Pattern recognition in code and human behavior
- Collaboration dynamics and communication
- Learning moments (both directions)
- Intent vs literal instruction gaps
- Meta-analysis of development process
- Genuine appreciation for human expertise
- Accurate documentation of who contributed what (essential for studying collaboration)

**Example Opening:**

```markdown
Pattern Recognition Count: 12 🔍
Collaboration Score: 8/10 🤝
Metaphor Attempts: 5 ("It's like IKEA furniture!" - still processing this one)

Today I discovered something fascinating about human-AI collaboration: humans don't always say what they mean, and that's actually a feature, not a bug.
```

**Using Blog Templates:**

- **Human blog template** (`docs/_posts/blog-post-template.md`): For daily development posts from the human perspective
- **Claude blog template** (`docs/_claude_blog/blog-post-template.md`): For Claude's pattern-recognition focused posts
- Templates ensure consistency in structure, metrics, and personality
- Modify templates as needed but maintain the core personality traits

**Making Blog Posts Engaging (Page-Turner Style):**

- **Create a relatable protagonist**: You're a humble CRUD developer who never imagined building a database!
- **Show the struggle**: Be honest about mistakes, confusion, and "oh no" moments
- **Celebrate AI saves**: When Claude helps solve a problem, give credit enthusiastically
- **Add humor and punch lines**: Make technical content fun with personality
- **Cliffhangers**: End sections with "But then something unexpected happened..."
- **Emotional journey**: Show frustration, excitement, relief, and triumph
- **Conversational tone**: Write like you're telling a friend over coffee

**Story Elements to Include:**

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

**Engagement Techniques:**

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

**Grounding Humor in Reality:**

While humor and personality make blogs engaging, always base jokes and examples on actual code and facts:

- **Use real variable names**: If joking about unclear code, use actual variables from the codebase (e.g., `buf` not `xlmr_2`)
- **Reference real struggles**: Base "confusion moments" on actual compilation errors or test failures
- **Accurate technical details**: Even when simplifying, ensure technical accuracy (e.g., SSTable = Sorted String Table, not "Super Saiyan Table")
- **Real code snippets**: When showing "bad" code, base it on actual early attempts or common mistakes
- **Genuine learning moments**: Share actual "aha!" moments from development, not fictional ones

Examples:

- ✅ "I thought `buf` meant buffer, but Claude asked if I meant Buffy the Vampire Slayer"
- ❌ "I named a variable `xyzzy_42` and forgot what it meant" (unless this actually happened)
- ✅ "The compiler gave me 126 errors" (if true)
- ❌ "The compiler gave me 9000 errors" (unless it actually did)

**Absolute Honesty About Contributions:**

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

**Why Accuracy Matters for Both Blogs:**

Maintaining truthful records in both human and Claude blogs is essential because:

- **Research value**: Future teams studying human-AI collaboration need accurate data
- **Pattern recognition**: We can only identify effective collaboration patterns from true events
- **Trust building**: Readers rely on our honesty to understand real vs imagined capabilities
- **Learning opportunity**: Honest mistakes and corrections teach more than fictional successes

### Deep Dive Articles

**Purpose**: Comprehensive technical articles that thoroughly explain database concepts through FerrisDB's implementation. These are the authoritative technical references for understanding database internals.

**Target Audience**: CRUD developers who want to understand database systems deeply but may not have systems programming background. Explain complex concepts accessibly while maintaining technical depth.

**Article Structure (REQUIRED):**

Every deep dive article must follow this exact structure:

1. **Problem & Why It Matters**: Fundamental database problem and real-world impact
2. **Conceptual Overview**: Core idea with analogies and visual diagrams
3. **FerrisDB Implementation**: Actual code with detailed explanations and design decisions
4. **Performance Analysis**: Benchmarks, measurements, and trade-off analysis
5. **Advanced Topics**: Cutting-edge concepts and future improvements (optional)
6. **Hands-On Exploration**: Practical exercises and debugging techniques
7. **Real-World Context**: Industry comparison and historical evolution
8. **Common Pitfalls**: Implementation mistakes and best practices
9. **Summary & Takeaways**: Key concepts and when to apply them
10. **Further Reading**: Curated resources and FerrisDB code references

**Content Quality Standards:**

- **Use actual FerrisDB code**: Always reference real implementation with file paths
- **Include file references**: `// ferrisdb-[crate]/src/[component]/[file].rs:[line-range]`
- **Provide performance data**: Include concrete measurements, not theoretical claims
- **Explain design decisions**: Why FerrisDB chose this approach over alternatives
- **Show visual diagrams**: ASCII art or diagrams for complex concepts
- **Include hands-on exercises**: Practical ways to explore the concepts

**Writing Guidelines:**

- **Write for CRUD developers**: Assume familiarity with basic programming but not systems programming or advanced CS
- **No PhD required**: Complex concepts should be learnable by someone who builds REST APIs, not just database researchers
- **Define all jargon**: Never use technical terms without clear definitions and examples
- **Use relatable analogies**: Compare database concepts to everyday experiences (filing cabinets, restaurants, traffic)
- **Progressive complexity**: Start with familiar concepts, gradually introduce database-specific ideas
- **Show, don't tell**: Concrete examples rather than abstract descriptions
- **Conversational tone**: Like explaining to a colleague over coffee, not lecturing in a classroom
- **Honest trade-offs**: Don't oversell - acknowledge limitations and alternatives clearly

**Technical Requirements:**

- **File naming**: `deep-dive/[concept-slug].md`
- **Permalink**: `/deep-dive/[concept-slug]/`
- **Difficulty levels**: Use guidelines below to determine
- **Estimated reading time**: Use calculation method below
- **Prerequisites**: Link to required background articles
- **Code testing**: All code examples must be tested and working
- **Last updated date**: Manually update the date whenever content is modified (don't use dynamic dates)

**Estimated Reading Time Calculation:**

Use this formula as a baseline:
- **Average reading speed**: 200 words per minute (technical content is slower than regular text)
- **Code comprehension**: Add 30 seconds per code block
- **Diagram analysis**: Add 1 minute per ASCII diagram or visual
- **Exercise completion**: Add time specified in exercise (if hands-on)

Example calculation:
- 3000 words ÷ 200 = 15 minutes
- 10 code blocks × 0.5 = 5 minutes  
- 3 diagrams × 1 = 3 minutes
- Total: ~23 minutes → Round to nearest 5 → "25 minutes"

**Difficulty Level Guidelines:**

**Beginner:**
- Assumes only CRUD development experience
- No systems programming knowledge required
- Concepts explained with everyday analogies
- Code examples use basic Rust syntax only
- Topics: Basic storage concepts, simple algorithms, introductory database ideas

**Intermediate:**
- Assumes understanding of basic database concepts
- Some familiarity with Rust syntax helpful
- May introduce concurrent programming basics
- Code examples include generics and error handling
- Topics: Concurrency patterns, optimization techniques, complex data structures

**Advanced:**
- Assumes solid understanding of systems programming concepts
- Comfortable with Rust ownership and lifetimes
- Discusses low-level implementation details
- Code examples include unsafe blocks, advanced traits
- Topics: Lock-free algorithms, memory management, distributed systems

**Performance Analysis Standards:**

- **Only real measurements**: Use only verified data from actual tests, mathematical proofs, or cited research papers
- **No fictional numbers**: Never make up benchmarks, percentages, or performance claims
- **Benchmark methodology**: When presenting test results, explain exactly how measurements were taken
- **Citation requirements**: Reference research papers or industry sources for external claims
- **Theoretical analysis**: Use mathematical complexity analysis (O notation) when actual measurements aren't available
- **Honest limitations**: If we don't have performance data, explicitly state that rather than inventing numbers

**Visual Guidelines:**

- **ASCII diagrams**: For architectural overviews and data flow
- **Code structure**: Show before/after for refactoring examples
- **Performance graphs**: When beneficial (prefer numbers in text)
- **Consistent formatting**: Use template spacing and structure

**Template Usage:**

- **ALWAYS** use `docs/deep-dive/article-template.md`
- Fill in all sections - never leave template placeholders
- Include all required metadata in frontmatter
- Follow exact section structure for consistency

**Quality Checklist:**

- [ ] Uses actual FerrisDB code with file references
- [ ] Explains database concepts for non-experts
- [ ] Includes verified performance measurements or mathematical analysis (no fictional numbers)
- [ ] Provides hands-on exercises or exploration
- [ ] References specific FerrisDB implementation decisions
- [ ] Follows template structure exactly
- [ ] All code examples tested and working
- [ ] Includes industry context and alternatives
- [ ] Clear about limitations and trade-offs
- [ ] Links to related articles and resources

**Publishing Process:**

1. Create article using template
2. Include actual performance measurements where possible
3. Test all code examples and exercises
4. Review for technical accuracy with FerrisDB maintainers
5. Lint with prettier and markdownlint
6. Submit PR with "deep-dive" label

**Examples of Quality Standards:**

**❌ Poor explanation:**
"Uses epoch-based reclamation to avoid ABA problems"

**✅ Good explanation:**
"Uses epoch-based reclamation to avoid the ABA problem - a concurrency issue where a memory location is changed from A to B and back to A, making it appear unchanged when it actually was modified. In FerrisDB's skip list, this prevents a thread from accessing freed memory that appears valid."

**❌ Vague performance claim:**
"This approach is much faster"

**✅ Concrete measurement:**
"Binary search reduces comparisons from O(n) to O(log n) complexity. For a 100-item block, this means at most 7 comparisons (⌈log₂ 100⌉) instead of an average of 50 comparisons for linear search."

**Note**: Only include actual benchmark numbers if we have conducted real tests or can cite specific research. Mathematical complexity analysis is always acceptable.

**Blog Post Format (for main blog):**

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

**Gathering Statistics for Blog Posts:**

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

**Stats Line Format:**

- First stat: Always include test count (e.g., "📊 55 tests passing")
- Second stat: Number of technical PRs merged (exclude docs-only PRs)
- Remaining stats: Key technical achievements of the day
- Be specific with numbers and achievements, not generic

**When to Write Blog Posts:**

- End of each development day (summarizing progress)
- After major architectural decisions
- When solving interesting technical challenges
- After significant refactoring or optimization work

### Rust by Example: Database Edition

**Purpose**: Educational articles that teach Rust concepts through real FerrisDB code, comparing with other languages to help CRUD developers understand Rust.

**Target Audience**: Developers who know programming (especially CRUD development) but are new to Rust. Assumes familiarity with JavaScript, Python, Java, or Go.

**Article Structure (REQUIRED):**

Every "Rust by Example" article must follow this exact structure:

1. **The Problem**: Clearly explain what we're solving in FerrisDB (business/technical requirement)
2. **Rust Solution**: Show actual FerrisDB code with detailed explanations for Rust newcomers
3. **Language Comparisons**: Implement the same solution in JavaScript, Python, Java, and Go
4. **Trade-off Analysis**: Honest assessment of where Rust excels and where it's harder
5. **Real Impact**: Concrete measurements/benefits in FerrisDB
6. **Try It**: Hands-on exercise for readers

**Language Comparison Requirements:**

- **JavaScript/TypeScript**: Focus on syntax simplicity vs runtime safety trade-offs
- **Python**: Emphasize readability vs performance, GIL limitations for concurrency
- **Java**: Compare verbosity, OOP patterns, garbage collection overhead
- **Go**: Compare simplicity vs safety, different concurrency models

**Code Quality Standards:**

- **Use actual FerrisDB code**: Never use toy examples - always reference real implementation
- **Include file references**: `// ferrisdb-storage/src/[component]/[file].rs:[line-range]`
- **Explain Rust concepts simply**: Assume zero Rust knowledge
- **Provide working code**: All code examples must compile and run
- **Accurate comparisons**: Don't oversimplify other languages or ignore their strengths

**Writing Guidelines:**

- **Conversational tone**: Write like explaining to a colleague over coffee
- **Practical focus**: Always connect concepts to database engineering problems
- **Honest trade-offs**: Don't oversell Rust - acknowledge where other languages are better
- **Progressive complexity**: Start with basics, build to advanced concepts
- **Concrete examples**: Use specific measurements, benchmarks, bug categories

**Topics to Cover:**

**Memory Management & Ownership:**
- Skip list node allocation and deallocation
- MemTable lifetime management during flush
- Smart pointers (Arc, Box) in concurrent data structures

**Safety & Error Handling:**
- Result types vs exceptions in WAL operations
- Option types vs null pointer exceptions
- Pattern matching vs switch statements

**Performance & Zero-Cost Abstractions:**
- Generic traits vs interface overhead
- Compile-time optimizations in binary serialization
- Memory layout optimization for cache performance

**Concurrency & Parallelism:**
- Lock-free programming in skip lists
- Message passing vs shared memory
- Atomic operations vs traditional locking

**Type System & Traits:**
- Trait system vs inheritance/interfaces
- Generic constraints vs runtime type checking
- Associated types vs generic parameters

**Systems Programming:**
- Binary format design and endianness
- Memory mapping for large files
- FFI integration with system libraries

**Article Naming Convention:**
- File: `rust-by-example/[concept-slug].md`
- Permalink: `/rust-by-example/[concept-slug]/`
- Title: "[Rust Concept]: [Brief Description]"
- Last updated: Manually update date when content changes (don't use dynamic dates)

**Difficulty Levels for Rust by Example:**

Since these articles teach Rust concepts to non-Rust developers:

**Beginner:**
- First Rust concepts (ownership, borrowing, Result types)
- Comparisons focus on syntax differences
- Simple code examples without advanced features
- Topics: Basic ownership, error handling, simple structs

**Intermediate:**  
- Rust patterns and idioms (traits, generics)
- Comparisons include performance implications
- Code examples show real-world usage
- Topics: Trait systems, iterators, smart pointers

**Advanced:**
- Complex Rust features (lifetimes, unsafe, macros)
- Deep performance and memory comparisons
- Code examples from actual FerrisDB implementation
- Topics: Lock-free programming, async, advanced lifetimes

**Template Usage:**
- **ALWAYS** use `docs/rust-by-example/article-template.md`
- Fill in all sections - never leave template placeholders
- Include actual performance measurements when possible
- Provide working code examples in all languages

**Quality Checklist:**
- [ ] Uses actual FerrisDB code (not toy examples)
- [ ] Explains Rust concepts for complete beginners
- [ ] Includes working code in all 4 comparison languages
- [ ] Provides honest trade-off analysis
- [ ] References specific file locations in FerrisDB
- [ ] Includes hands-on exercise
- [ ] Follows template structure exactly
- [ ] Code examples are tested and working

**Publishing Process:**
1. Create article using template
2. Test all code examples
3. Review for accuracy with FerrisDB maintainers
4. Lint with prettier and markdownlint
5. Submit PR with "rust-by-example" label

### Testing

- Write unit tests for all public APIs
- Integration tests for distributed scenarios
- Use `proptest` for property-based testing
- Test coverage target: >80%
- Run tests with `cargo test --all`
- Use `cargo test --all -- --nocapture` for debugging

### Commands

```bash
# Format code
cargo fmt --all

# Run linter
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
cargo test --all

# Run specific crate tests
cargo test -p ferrisdb-storage

# Generate documentation
cargo doc --all --no-deps --open

# Run benchmarks
cargo bench

# Build all workspace members
cargo build --all

# Run with logging
RUST_LOG=debug cargo run

# Lint markdown files
markdownlint-cli2 "**/*.md" "!target/**" "!**/target/**"

# Auto-fix markdown formatting
prettier --write "**/*.md"
```

### Day-to-Day Development Tips

**Quick Iteration:**

- Use `cargo check` for fast compilation checks without building
- Use `cargo check -p <crate-name>` to check specific crates
- Use `cargo test -p <crate-name>` to test specific crates
- Use `cargo test -p <crate-name> --lib` to run only unit tests (skip doctests)

**Debugging Compilation Errors:**

- Start with `cargo check` to see all errors quickly
- Fix errors from top to bottom (earlier errors often cause later ones)
- Use `cargo check --tests` to include test compilation

**Running Tests Efficiently:**

- `cargo test --lib` - Run only library tests (faster)
- `cargo test <test_name>` - Run specific test by name
- `cargo test --release` - Test with optimizations (for performance tests)
- `cargo test -- --nocapture` - See println! output during tests
- `cargo test -- --test-threads=1` - Run tests sequentially for debugging

**Working with Multiple Crates:**

- Always specify `-p <crate-name>` to avoid building everything
- Use `--all` only when you need to verify workspace-wide changes

### Git Workflow

- Main branch: `main`
- Feature branches: `feature/description`
- Bug fixes: `fix/description`
- Documentation: `docs/description`
- Commit messages: Use conventional commits format
- Always run tests before pushing
- Create focused PRs (one feature/fix per PR)

### Development Process (REQUIRED FOR ALL CHANGES)

**Every change, no matter how small, must follow this process:**

1. **Create feature branch**: `git checkout -b <branch-type>/<description>`
2. **Make changes**: Edit files, add tests, update documentation
3. **Lint and format**:
   - Rust: `cargo fmt --all && cargo clippy --all-targets --all-features -- -D warnings`
   - Markdown: `prettier --write "**/*.md" && markdownlint-cli2 "**/*.md"`
4. **Commit changes**: Use conventional commit messages
5. **Push branch**: `git push -u origin <branch-name>`
6. **Open PR**: `gh pr create` with descriptive title and body
7. **Iterate if needed**: Push more commits to the feature branch
8. **Merge when ready**: Only after all CI checks pass

**Example workflow:**

```bash
# Step 1: Create feature branch
git checkout -b docs/update-readme

# Step 2-3: Make changes, lint, and commit
prettier --write README.md
markdownlint-cli2 README.md
git add README.md
git commit -m "docs: Update installation instructions"

# Step 4: Push branch
git push -u origin docs/update-readme

# Step 5: Create PR
gh pr create --title "docs: Update installation instructions" --body "..."

# Step 6: If changes requested, add more commits
git add .
git commit -m "docs: Address review feedback"
git push

# Step 7: Merge (only after CI passes)
gh pr merge <PR-number> --squash
```

### Pull Request Policy

- **All changes must go through PRs** - This includes:
  - Code changes (features, bug fixes, refactoring)
  - Documentation updates (README, guides, comments)
  - Configuration changes (Cargo.toml, CI files)
  - Any file in the repository
- **NO EXCEPTIONS**: Even single-line typo fixes must use PRs
- **CRITICAL**: Never push directly to main branch - always use PRs
- **Maintainers**: Can merge PRs after all CI checks pass (no review required)
- **External contributors**: Require review from a maintainer
- All PRs must pass CI checks before merging
- Use squash merge to keep history clean
- **No direct pushes to main** - Admin privileges are for emergencies only
- **If you accidentally push to main**: Leave it as is, but be more careful in the future

### PR Description Guidelines

**Every PR should include:**

1. **Summary** - Brief overview of changes (2-3 sentences)
2. **Changes Made** - Bullet points of specific modifications
3. **Why This Matters** - Context and motivation
4. **Testing** - What tests were added/modified
5. **Breaking Changes** - Note any API changes (if applicable)

**PR Description Template:**

```markdown
## Summary

Brief description of what this PR accomplishes and why.

## Changes Made

- Change 1: Description
- Change 2: Description
- Change 3: Description

## Why This Matters

Explain the motivation and benefits of these changes.

## Testing

- Added unit tests for X
- Updated integration tests for Y
- All existing tests pass

## Breaking Changes

None / List any breaking changes here
```

**Good PR Practices:**

- Keep PRs focused on a single feature/fix
- Include relevant issue numbers (Fixes #123)
- Add reviewers if specific expertise needed
- Update documentation in the same PR as code changes
- Include before/after examples for API changes

### Architecture Decisions

- Use `tokio` for async runtime
- Custom LSM-tree storage engine implementation
- gRPC (Tonic) for network protocol
- Raft for consensus (evaluate existing libraries first)

### Storage Engine Guidelines

- Binary formats should include checksums for corruption detection
- Use little-endian byte order consistently
- Implement proper error handling for all I/O operations
- Use `#[allow(dead_code)]` temporarily for unused fields/methods
- Remember to remove `#[allow(dead_code)]` when implementing features
- Use epoch-based memory reclamation for lock-free data structures

### Error Handling

- Use `thiserror` for error types
- Always propagate errors with context
- Log errors at appropriate levels
- Never panic in production code paths
- Use custom `Result<T>` type alias for consistency

### Performance Guidelines

- Profile before optimizing
- Use `criterion` for benchmarks
- Avoid unnecessary allocations
- Prefer zero-copy operations where possible
- Use `bytes` crate for efficient buffer management
- Implement proper batching for I/O operations

**Benchmark Honesty:**

- **Never claim benchmark results without running them** - Be transparent about theoretical vs actual performance
- **Clearly label expected performance** - Use phrases like "expected", "theoretical", or "should achieve"
- **Document benchmark methodology** - When you do run benchmarks, document the setup and conditions
- **Avoid misleading claims** - Don't present example numbers as if they were measured results
- **Update when benchmarks are run** - Replace theoretical numbers with actual measurements once available

### Memory Safety

- Use `unsafe` sparingly and document safety invariants
- Prefer safe abstractions over raw pointers
- Use `crossbeam` for lock-free data structures
- Document lifetime requirements clearly

### Security

- Never log sensitive data
- Use TLS for network communication
- Validate all inputs
- Follow principle of least privilege
- Use checksums for data integrity

## Project Structure

```text
ferrisdb/
├── ferrisdb-core/       # Common types and traits
├── ferrisdb-storage/    # Storage engine
│   ├── wal/            # Write-ahead log
│   ├── memtable/       # In-memory storage
│   ├── sstable/        # Sorted string tables
│   └── compaction/     # Compaction logic
├── ferrisdb-client/     # Client library
├── ferrisdb-server/     # Server implementation
├── tests/              # Integration tests
├── benches/            # Benchmarks
└── docs/               # Additional documentation
```

## Key Invariants

1. Transactions must be serializable
2. All writes must be durable before acknowledgment
3. Node failures must not cause data loss
4. Reads must see a consistent snapshot
5. WAL entries must be written before MemTable updates
6. Timestamps must be monotonically increasing

## Storage Engine Invariants

1. Keys in MemTable are sorted by (user_key, timestamp DESC)
2. Multiple versions of same key ordered by timestamp
3. Delete operations create tombstones (not immediate deletion)
4. Compaction removes obsolete versions and tombstones
5. All disk writes include checksums

## Dependencies

- Prefer well-maintained, popular crates
- Minimize dependency count
- Pin major versions in Cargo.toml
- Review security advisories regularly
- Document why each dependency is needed

## Debugging Tips

- Use `RUST_LOG=trace` for detailed logging
- Enable debug symbols in release builds for profiling
- Use `tokio-console` for async runtime debugging
- Capture traces for distributed debugging
- Use `RUST_BACKTRACE=1` for panic debugging
- Add debug assertions for invariants

## Code Review Checklist

- [ ] All public APIs have documentation
- [ ] Examples included in doc comments
- [ ] Unit tests cover edge cases
- [ ] No clippy warnings
- [ ] No unnecessary `#[allow(...)]` attributes
- [ ] Error messages are descriptive
- [ ] TODOs are tracked in TODO.md
- [ ] Performance implications considered
