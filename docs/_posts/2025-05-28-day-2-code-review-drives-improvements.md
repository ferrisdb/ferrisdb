---
layout: post
title: "Day 2: From Linear Search to Clean APIs - Learning Through Code Review"
subtitle: "How asking 'why' during code review led to binary search optimization and architectural improvements"
description: "Building SSTables with Claude revealed the power of code review. My questions about defensive checks, linear search, and API design led to significant improvements and deeper understanding."
date: 2025-05-28
author: human
day: 2
categories: [development, database, collaboration]
tags: [ferrisdb, rust, sstable, optimization, api-design]
permalink: /blog/human/day-2-code-review-drives-improvements/
stats:
  [
    "📊 55 tests passing",
    "📄 5 PRs merged",
    "⏱️ 7x search performance improvement",
    "🔧 Major API refactoring",
    "🛡️ Defensive checks added",
  ]
confidence: "Start: 6/10 | End: 8/10"
review_cycles: "5 major iterations"
---

Day 2 showed how simple questions during code review can lead to major improvements. From asking about sorted data to questioning API design, each question deepened my understanding and improved our code.

<!--more-->

## Table of contents

{: .no_toc .text-delta }

<!-- prettier-ignore-start -->

1. TOC
{:toc}
<!-- prettier-ignore-end -->

---

<!-- Before publishing: Verify technical details against codebase! -->

## The Setup

**Me**: What should we work on next?

**Claude**: Let me check our TODO list... We should implement SSTables next. These are the on-disk sorted string tables that store data persistently after it's flushed from the MemTable.

**Me**: Sounds good! Can you implement the SSTable writer and reader?

**Claude**: I'll implement a block-based format for efficiency. Here's my design:

```rust
// SSTable structure:
// [Data Block 1][Data Block 2]...[Index Block][Footer]
//
// Each data block: ~4KB of sorted entries
// Index block: First key of each data block
// Footer: Metadata and magic number
```

## The Challenge

Claude implemented the writer and reader. During review, I noticed something in the comments.

**Me**: I see you commented that the `add` method won't check key order. Why don't we add defensive checks here?

**Claude**: Good question! I avoided checks for performance - assuming the caller (MemTable flush) provides sorted data. But you're right about defensive programming. There's a trade-off:

- No checks: Better performance, but silent corruption if ordering violated
- With checks: Slight overhead, but fail-fast on bugs

**Me**: Can you research how other storage engines like RocksDB handle this?

**Claude**: I researched it. RocksDB and LevelDB both validate key ordering during SSTable construction. They follow the principle: "Fail fast and loud rather than corrupt data silently." The performance cost is minimal compared to the disk I/O.

**Me**: Let's add defensive checks then. Better to catch bugs early than debug corruption later.

## Seeking Understanding

After implementing defensive checks, I continued reviewing the reader implementation.

**Me**: I'm looking at your block search code:

```rust
// Linear search through block
for entry in block.entries {
    if entry.key == *key {
        return Ok(Some(entry.value));
    }
}
```

Are the blocks always sorted?

**Claude**: Yes! The blocks are always sorted because:

1. MemTable maintains sorted order (skip list)
2. We flush in order during SSTable creation
3. The defensive checks now ensure this

The entries are sorted by user_key (ascending), then timestamp (descending).

**Me**: Wait, if they're sorted, can't we use binary search instead of linear search?

## The Breakthrough

**Claude**: Hmm, I'm hesitant about binary search because InternalKey contains an operation field. I'm not sure how that impacts sorting...

**Me**: But I see InternalKey implements Ord, and looking at the implementation, it only compares user_key and timestamp, not operation. So binary search should work, right?

**Claude**: You're absolutely right! I was overcomplicating it. Since InternalKey already implements Ord correctly, we can use binary search straightforwardly:

```rust
// Binary search using partition_point
let pos = block.entries.partition_point(|e| e.key < *search_key);

if pos < block.entries.len() && block.entries[pos].key == *search_key {
    return Ok(Some(block.entries[pos].value.clone()));
}
```

**Me**: Great! Please implement the binary search.

**Claude**: Done! All tests pass. With 100 entries per block, we go from ~50 comparisons (linear) to ~7 comparisons (binary) - about 7x improvement!

## Deeper Understanding

After the binary search improvement, something bothered me about the API.

**Me**: I'm looking at how we use the reader:

```rust
// To read a value, I have to do this:
let key = InternalKey::new(b"user:123", timestamp, Operation::Put);
let value = reader.get(&key)?;
```

Why do I need to specify `Operation::Put` when reading? I'm just trying to get a value. What are your thoughts about removing operation from InternalKey?

**Claude**: That's an excellent observation! Let me investigate...

You're right - this is a design flaw. The operation isn't part of the key's identity, it's metadata about what to do with that key. I suggest:

```rust
// Current design - operation mixed into identity
pub struct InternalKey {
    pub user_key: Key,
    pub timestamp: Timestamp,
    pub operation: Operation,  // Doesn't belong here!
}

// Better design - clean separation
pub struct InternalKey {
    pub user_key: Key,
    pub timestamp: Timestamp,
}

pub struct SSTableEntry {
    pub key: InternalKey,
    pub value: Value,
    pub operation: Operation,  // Metadata, not identity!
}
```

**Me**: That makes so much more sense! Let's refactor it.

**Claude**: This will touch a lot of code, but it's worth it for the cleaner API. Let me work through all the compiler errors...

[After some time fixing compilation errors]

Done! All tests pass. The API is much cleaner now:

```rust
// Before
let key = InternalKey::new(b"user:123", timestamp, Operation::Put);
let value = reader.get(&key)?;

// After
let key = InternalKey::new(b"user:123", timestamp);
let value = reader.get(&key)?;
```

## Reflection

Today demonstrated the power of asking "why" during code review:

1. **"Why no defensive checks?"** → Better error detection
2. **"Are blocks sorted?"** → Understanding the data model
3. **"Can't we use binary search?"** → 7x performance improvement
4. **"Why specify operation when reading?"** → Cleaner architecture

Starting confidence: 6/10 - "I can review database code"
Ending confidence: 8/10 - "My questions lead to real improvements!"

The biggest lesson: Don't accept awkward APIs or inefficient code. Question everything, and often you'll find better solutions.

## Tomorrow's Adventure

Next: Compaction! Claude mentioned it's like "database Tetris" - merging SSTables while removing old versions. I'm already preparing questions about triggering strategies and handling concurrent reads.

---

**Human Status**: Proud of catching both performance and design issues
**Review Cycles**: 5 major improvements from questions
**Lesson Learned**: Simple questions often reveal big opportunities

_P.S. The moment I realized we were doing linear search on sorted data was when I truly understood the value of code review. Always question the obvious!_
