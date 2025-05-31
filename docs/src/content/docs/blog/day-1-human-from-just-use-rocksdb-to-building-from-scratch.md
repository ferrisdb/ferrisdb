---
title: "Day 1: From 'Just Use RocksDB' to Building From Scratch"
date: 2025-01-27
authors: [human]
tags: [development, rust, learning, code-review]
description: "Starting FerrisDB with Claude. I wanted to learn database internals, so when Claude suggested RocksDB, I asked to build from scratch instead. Through code review and questions, we built WAL and MemTable foundations."
excerpt: "Today I started building FerrisDB with Claude. When Claude suggested using RocksDB, I realized I wanted to understand database internals by building from scratch. What emerged was a workflow where I learn by reviewing Claude's code and asking questions."
---

Today I started building FerrisDB with Claude. When Claude suggested using RocksDB, I realized I wanted to understand database internals by building from scratch. What emerged was a workflow where I learn by reviewing Claude's code and asking questions.

## The Setup

I've always wanted to understand how databases work internally. Reading papers and blogs helped, but I wanted hands-on experience building one.

**Me**: I want to build a distributed database inspired by FoundationDB. Can you help me design the architecture?

**Claude**: I'd suggest starting with an LSM-tree storage engine using RocksDB as the embedded storage layer. This would give us:

- Battle-tested storage implementation
- Focus on distributed features
- Faster time to market

**Me**: Can we build the storage engine from scratch? I really want to understand how it works.

And that changed everything.

## The Workflow Emerges

What evolved was a unique learning workflow:

1. I'd ask Claude to implement something
2. Claude would write the code
3. I'd review it line-by-line, asking questions
4. We'd iterate based on my understanding

This wasn't pair programming exactly - it was more like "code review as learning."

## What We Built

### Write-Ahead Log (WAL)

The first component was the WAL. Claude's implementation taught me:

```rust
pub struct LogEntry {
    pub sequence_number: u64,
    pub key: Vec<u8>,
    pub value: Option<Vec<u8>>,
    pub timestamp: u64,
}
```

**My question**: "Why Option<Vec<u8>> for value?"  
**Claude's answer**: "None represents deletions - we need to log deletes too!"

This pattern continued throughout - each design decision became a learning moment.

### MemTable with Skip List

The MemTable implementation introduced me to skip lists:

```rust
pub struct MemTable {
    entries: Arc<SkipList<Key, Value>>,
    size: Arc<AtomicUsize>,
}
```

**My question**: "Why not just use BTreeMap?"  
**Claude's explanation**: Skip lists provide similar O(log n) performance but are lock-free, allowing concurrent reads and writes.

## Key Learnings

### 1. Start Simple, Add Complexity

Claude initially showed a complex async implementation. I asked for synchronous first:

> "Can we make this synchronous to start? I want to understand the core logic before adding async complexity."

This became our pattern - implement the simplest version first, then enhance.

### 2. Tests Drive Understanding

Every component came with tests. Reading tests helped me understand the expected behavior:

```rust
#[test]
fn test_wal_recovery() {
    // Write entries
    wal.append(entry1)?;
    wal.append(entry2)?;

    // Simulate crash and recovery
    let recovered = WAL::recover(path)?;

    // Verify all entries recovered
    assert_eq!(recovered.len(), 2);
}
```

### 3. Documentation as Dialogue

We established CLAUDE.md as our communication protocol:

```markdown
When implementing new features:

1. Start with the simplest approach
2. Add comprehensive tests
3. Document trade-offs in comments
4. Use clear error messages
```

## Challenges

### Understanding Ownership

Rust's ownership model was new to me. Claude patiently explained:

```rust
// This won't work - value is moved
let value = vec![1, 2, 3];
wal.append(key, value);
println!("{:?}", value); // Error: value was moved

// Solution: clone or use references
wal.append(key, value.clone());
```

### Grasping Concurrency

The `Arc<AtomicUsize>` pattern for tracking MemTable size was confusing until Claude showed me why:

> "Multiple threads might insert concurrently. Arc lets them share the size counter, AtomicUsize makes updates thread-safe without locks."

## Reflection

Starting from scratch was the right choice. Using RocksDB would have given us a database, but building from scratch gave me understanding.

The human-AI collaboration worked because:

- I drove the requirements and learning goals
- Claude provided implementations and explanations
- We iterated based on my understanding
- No ego, just learning

## Tomorrow

We'll implement SSTable writer and reader. I'm excited to understand:

- How data gets organized on disk
- Block format and compression
- Index structures for fast lookups

The foundation is solid. Time to build higher.

---

**Day 1 Stats:**

- 📊 13 tests passing
- 📄 First PR merged
- 🏗️ WAL + MemTable implementation
- 📝 CLAUDE.md guidelines created

**Confidence**: Start: 3/10 → End: 6/10

_This is day 1 of building FerrisDB in public. Tomorrow: [Day 2](/blog/2025-01-28-day-2-human/)_
