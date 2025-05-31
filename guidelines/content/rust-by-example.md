# Rust by Example: Database Edition Guidelines

Educational articles that teach Rust concepts through real FerrisDB code, comparing with other languages to help CRUD developers understand Rust.

**Purpose**: Create educational content that teaches Rust through practical database examples with language comparisons.  
**Prerequisites**: Proficiency in Rust and at least one other language (JavaScript, Python, Java, or Go)

## Purpose

These articles bridge the gap between CRUD development and systems programming by teaching Rust concepts through practical database engineering examples, with comparisons to familiar languages.

## Target Audience

Developers who know programming (especially CRUD development) but are new to Rust. Assumes familiarity with JavaScript, Python, Java, or Go.

## Article Structure (REQUIRED)

Every "Rust by Example" article must follow this exact structure:

1. **The Problem**: Clearly explain what we're solving in FerrisDB (business/technical requirement)
2. **Rust Solution**: Show actual FerrisDB code with detailed explanations for Rust newcomers
3. **Language Comparisons**: Implement the same solution in JavaScript, Python, Java, and Go
4. **Trade-off Analysis**: Honest assessment of where Rust excels and where it's harder
5. **Real Impact**: Concrete measurements/benefits in FerrisDB
6. **Try It**: Hands-on exercise for readers

## Language Comparison Requirements

- **JavaScript/TypeScript**: Focus on syntax simplicity vs runtime safety trade-offs
- **Python**: Emphasize readability vs performance, GIL limitations for concurrency
- **Java**: Compare verbosity, OOP patterns, garbage collection overhead
- **Go**: Compare simplicity vs safety, different concurrency models

## Code Quality Standards

- **Use actual FerrisDB code**: Never use toy examples - always reference real implementation
- **Include file references**: `// ferrisdb-storage/src/[component]/[file].rs:[line-range]`
- **Explain Rust concepts simply**: Assume zero Rust knowledge
- **Provide working code**: All code examples must compile and run
- **Accurate comparisons**: Don't oversimplify other languages or ignore their strengths
- **Mark unimplemented features**: Use [PLANNED] or [CONCEPTUAL] tags when discussing future work

### Implementation Status

**MANDATORY**: Be clear about what's implemented vs. planned:

1. **For Current Code**

   ```markdown
   Here's how FerrisDB handles concurrent writes using Arc:
   ```

2. **For Future Features**

   ```markdown
   ## Lock-Free Implementation [PLANNED]

   > **Note**: Currently using Arc with RwLock. Lock-free version planned.

   In the future, we plan to implement a lock-free skip list...
   ```

## Writing Guidelines

- **Conversational tone**: Write like explaining to a colleague over coffee
- **Practical focus**: Always connect concepts to database engineering problems
- **Honest trade-offs**: Don't oversell Rust - acknowledge where other languages are better
- **Progressive complexity**: Start with basics, build to advanced concepts
- **Concrete examples**: Use specific measurements, benchmarks, bug categories

## Topics to Cover

### Memory Management & Ownership

- Skip list node allocation and deallocation
- MemTable lifetime management during flush
- Smart pointers (Arc, Box) in concurrent data structures

### Safety & Error Handling

- Result types vs exceptions in WAL operations
- Option types vs null pointer exceptions
- Pattern matching vs switch statements

### Performance & Zero-Cost Abstractions

- Generic traits vs interface overhead
- Compile-time optimizations in binary serialization
- Memory layout optimization for cache performance

### Concurrency & Parallelism

- Lock-free programming in skip lists
- Message passing vs shared memory
- Atomic operations vs traditional locking

### Type System & Traits

- Trait system vs inheritance/interfaces
- Generic constraints vs runtime type checking
- Associated types vs generic parameters

### Systems Programming

- Binary format design and endianness
- Memory mapping for large files
- FFI integration with system libraries

## Article Naming Convention

- File: `rust-by-example/[concept-slug].md`
- Permalink: `/rust-by-example/[concept-slug]/`
- Title: "[Rust Concept]: [Brief Description]"
- Last updated: Manually update date when content changes (don't use dynamic dates)

## Difficulty Levels

Since these articles teach Rust concepts to non-Rust developers:

### Beginner

- First Rust concepts (ownership, borrowing, Result types)
- Comparisons focus on syntax differences
- Simple code examples without advanced features
- Topics: Basic ownership, error handling, simple structs

### Intermediate

- Rust patterns and idioms (traits, generics)
- Comparisons include performance implications
- Code examples show real-world usage
- Topics: Trait systems, iterators, smart pointers

### Advanced

- Complex Rust features (lifetimes, unsafe, macros)
- Deep performance and memory comparisons
- Code examples from actual FerrisDB implementation
- Topics: Lock-free programming, async, advanced lifetimes

## Example Comparison Structure

### The Problem

"We need to safely share the MemTable between multiple reader threads while one writer thread updates it."

### Rust Solution

```rust
// ferrisdb-storage/src/memtable/mod.rs:45-67
use std::sync::Arc;
use parking_lot::RwLock;

pub struct MemTable {
    inner: Arc<RwLock<SkipList>>,
}
```

[Detailed explanation of Arc, RwLock, and why this is safe]

### JavaScript Comparison

```javascript
// No built-in solution - would need external library
// or risk race conditions with shared state
class MemTable {
  constructor() {
    this.data = new Map(); // Not thread-safe!
  }
}
```

[Explain lack of threading model and safety issues]

### Python Comparison

```python
import threading

class MemTable:
    def __init__(self):
        self.lock = threading.RLock()
        self.data = {}  # GIL provides some safety, but...
```

[Discuss GIL limitations and performance impact]

### Java Comparison

```java
import java.util.concurrent.ConcurrentSkipListMap;
import java.util.concurrent.locks.ReadWriteLock;

public class MemTable {
    private final ReadWriteLock lock = new ReentrantReadWriteLock();
    private final ConcurrentSkipListMap<Key, Value> data;
}
```

[Compare verbosity and runtime overhead]

### Go Comparison

```go
type MemTable struct {
    mu   sync.RWMutex
    data *skiplist.SkipList // Assume third-party
}
```

[Discuss simplicity vs compile-time guarantees]

## Quality Checklist

- [ ] Uses actual FerrisDB code (not toy examples)
- [ ] Explains Rust concepts for complete beginners
- [ ] Includes working code in all 4 comparison languages
- [ ] Provides honest trade-off analysis
- [ ] References specific file locations in FerrisDB
- [ ] Includes hands-on exercise
- [ ] Follows template structure exactly
- [ ] Code examples are tested and working

## Common Pitfalls to Avoid

- Don't create strawman comparisons - show each language at its best
- Avoid Rust evangelism - be honest about learning curve
- Don't skip error handling in examples - it's core to Rust
- Include real performance numbers when available
- Show idiomatic code for each language

## Template Usage

- **ALWAYS** use the [Rust by Example Template](templates/rust-by-example.mdx)
- Template now uses Starlight MDX format with component imports
- Fill in all sections - never leave template placeholders
- Include actual performance measurements when possible
- Provide working code examples in all languages
- Use Starlight components (Card, Tabs, Aside) for comparisons

## Publishing Process

1. Create article using template
2. Test all code examples in all languages
3. Verify FerrisDB code references are accurate
4. Review for balanced language comparisons
5. Lint with prettier and markdownlint
6. Submit PR with "rust-by-example" label

## Related Guidelines

- [Idiomatic Rust](../development/idiomatic-rust.md) - Rust best practices
- [Database Concepts](database-concepts-articles.md) - Technical depth
- [Documentation](../development/documentation.md) - Writing standards
- [Code Style](../development/code-style.md) - Code examples formatting
