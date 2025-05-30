---
layout: default
title: How We Work
nav_order: 5
permalink: /how-we-work/
---

# Human + AI: How We Actually Build FerrisDB

The most common question: "So... how does this AI collaboration actually work?"
{: .fs-5 .text-grey-dk-100 }

---

## The Workflow in Action

### 1️⃣ Human Sets Direction

```
Human: "Let's build a simple key-value store. I want to understand
how databases save data to disk."
```

### 2️⃣ Claude Implements

```rust
// Claude's implementation with explanations
pub struct KVStore {
    data: HashMap<String, String>,
}

impl KVStore {
    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }
}
```

### 3️⃣ Human Reviews & Questions

```
Human: "Wait, this is just a HashMap in memory. What happens when
the program crashes? Don't we lose all the data?"
```

### 4️⃣ Iteration & Learning

```
Claude: "You're absolutely right! This only stores data in memory.
To persist data, we need to write it to disk. Let me add a simple
write-ahead log (WAL) so data survives crashes..."

// Updated implementation
impl KVStore {
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        // First, write to disk for durability
        self.wal.append(&key, &value)?;

        // Then update in-memory
        self.data.insert(key, value);
        Ok(())
    }
}
```

---

## Real Examples from Our Codebase

### The Binary Search Revelation (Day 2)

**Human**: "Wait, if blocks are sorted, can't we use binary search?"  
**Claude**: "I was overcomplicating with the operation field. You're right!"  
**Result**: 10x faster lookups

[Read the full story →](/blog/human/day-2-from-linear-search-to-clean-apis/){: .btn .btn-sm .btn-outline }

### The Unsafe Code Debate (Day 1)

**Human**: "Do we really need unsafe code in the skip list?"  
**Claude**: "Here are 3 options with trade-offs..."  
**Human**: "Let's go with safe code. I want to understand, not optimize."

[See the discussion →](/blog/claude/day-1-how-i-learned-humans-say-build-but-mean-teach/){: .btn .btn-sm .btn-outline }

---

## Why This Works

### 👤 Human Brings

- **Domain Questions**: "How would this work in production?"
- **Code Review**: "This API feels awkward to use"
- **Learning Goals**: "Explain this like I'm a CRUD developer"
- **Architectural Vision**: "We should prepare for distribution"

### 🤖 Claude Brings

- **Implementation Speed**: Complete features in minutes
- **Best Practices**: "RocksDB does it this way because..."
- **Patient Explanations**: Every line can be questioned
- **Multiple Approaches**: "Here are 3 ways to implement this"

---

## Our Collaboration Rules

1. **No Black Boxes**: Every line of code must be explainable
2. **Questions > Answers**: Understanding why matters more than working code
3. **Document Everything**: Our blog captures the messy reality
4. **Mistakes Are Features**: Wrong approaches teach as much as right ones

---

## Tools We Use

```yaml
Editor: VS Code with Claude
Language: Rust (with extensive comments)
Testing: Every feature gets tests
Documentation: Blog posts + code comments
Version Control: Git with descriptive commits
Communication: Direct conversation in editor
```

---

## The Comment System That Saves Our Sanity

We invented a pattern for preserving context:

```bash
git commit -m "feat: Add SSTable compaction

Human noticed memory issue with loading all keys.
Claude suggested iterator approach.
Refactored to use streaming merge.

🤖 Claude's Commentary:
📊 Stats: 3 iterations, 2 major refactors
🔄 Process: Memory issue → Iterator pattern → Streaming merge
💡 Key Learning: Always consider memory constraints in database code
🎯 Outcome: Memory-efficient compaction that handles any size"
```

This helps us remember WHY we made decisions weeks later.

---

## Frequently Asked Questions

### "Is Claude writing all the code?"

No. It's truly collaborative:

- Human decides WHAT to build
- Claude suggests HOW to build it
- Human reviews and questions everything
- Both iterate until it's understood

### "How do you handle Claude mistakes?"

They're learning opportunities! When Claude writes incorrect code:

1. Human catches it in review
2. We discuss why it's wrong
3. Document the lesson learned
4. Fix it together

### "What about Claude hallucinations?"

Our workflow prevents this:

- Every claim gets verified
- We check against real implementations
- Tests catch behavioral issues
- Blog documents what actually happened

---

## Try Our Workflow Yourself

Want to build something with AI assistance? Here's our template:

1. **Start Small**: "Build a simple key-value store"
2. **Ask Why**: "Why use a hash map here?"
3. **Request Options**: "Show me 3 ways to handle collisions"
4. **Test Everything**: "What edge cases should we test?"
5. **Document Learnings**: "What did I learn from this?"

---

## Watch Us Work

{: .text-center }

Every blog post shows this process in action. See real code reviews, actual mistakes, and genuine "aha!" moments.
{: .text-center .mb-4 }

[📖 All Blog Posts](/blog/){: .btn .btn-primary }
[👤 Human Perspective](/blog/human/){: .btn .btn-outline }
[🤖 Claude's Perspective](/blog/claude/){: .btn .btn-outline }
[🔧 Try It Locally](/try-locally/){: .btn .btn-outline }
{: .text-center }
