---
layout: post
title: "Day 1: From 'Just Use RocksDB' to Building From Scratch"
subtitle: "From 'let's use RocksDB' to 'let's build it from scratch' - establishing our workflow"
description: "Starting FerrisDB with Claude. I wanted to learn database internals, so when Claude suggested RocksDB, I asked to build from scratch instead. Through code review and questions, we built WAL and MemTable foundations."
date: 2025-05-27
author: human
day: 1
tags: [ferrisdb, rust, database, learning, code-review]
permalink: /blog/human/day-1-from-just-use-rocksdb-to-building-from-scratch/
stats:
  [
    "📊 13 tests passing",
    "📄 First PR merged",
    "🏗️ WAL + MemTable implementation",
    "📝 CLAUDE.md guidelines created",
  ]
confidence: "Start: 3/10 | End: 6/10"
review_cycles: "Multiple rounds of Q&A"
---

Today I started building FerrisDB with Claude. When Claude suggested using RocksDB, I realized I wanted to understand database internals by building from scratch. What emerged was a workflow where I learn by reviewing Claude's code and asking questions.

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

I've always wanted to understand how databases work internally. Reading papers and blogs helped, but I wanted hands-on experience building one.

**Me**: I want to build a distributed database inspired by FoundationDB. Can you help me design the architecture?

**Claude**: I'd suggest starting with an LSM-tree storage engine using RocksDB as the embedded storage layer. This would give us:

- Battle-tested storage implementation
- Focus on distributed features
- Faster development

**Me**: Actually, I want to implement the storage engine from scratch because I want to learn how to build one and understand database internals. Can we start with the basic components?

**Claude**: Absolutely! Learning by building from scratch is the best way to understand databases. Let me design a basic LSM-tree architecture:

```
Components:
1. Write-Ahead Log (WAL) - Durability
2. MemTable - In-memory writes using skip list
3. SSTables - Persistent sorted files
4. Compaction - Background optimization

Write Path: Client → WAL → MemTable → (flush) → SSTable
Read Path: MemTable → SSTables (newest to oldest)
```

## The Challenge

Claude implemented the WAL, but I noticed something immediately.

**Me**: I'm looking at your WAL implementation. There are hardly any comments. Can you explain what each part does?

**Claude**: You're right, let me add documentation. [Adds detailed comments explaining each component]

**Me**: This is helpful, but I'm still confused about:

1. How does WAL ensure durability?
2. What happens during crash recovery?
3. Why serialize before writing?

**Claude**: Great questions! Let me explain:

1. **Durability**: WAL writes to disk with fsync() before acknowledging. Even if we crash after, the write is preserved.
2. **Recovery**: On startup, we replay the WAL to rebuild MemTable state
3. **Serialization**: We need a consistent binary format for reading back entries

This made me realize we need better documentation standards...

## Seeking Understanding

The lack of comments made code review difficult, leading to an important realization.

**Me**: We need documentation guidelines. Let me create CLAUDE.md with standards for comments, error handling, and testing.

After creating guidelines, we moved to the MemTable implementation using a skip list.

**Me**: I see you're using a skip list with unsafe code and ThreadRng. Do we really need unsafe code?

**Claude**: Good question! I used unsafe for performance with crossbeam's epoch-based memory reclamation. Here are our options:

1. Keep unsafe but add safety comments
2. Use safe alternatives with Arc/Mutex (some performance cost)
3. Use a different data structure entirely

**Me**: Let's explore option 2. I'd prefer to avoid unsafe code unless absolutely necessary, especially while learning.

**Claude**: Here's a safe implementation using StdRng instead of ThreadRng:

```rust
pub struct SkipList {
    head: Atomic<Node>,
    height: AtomicUsize,
    size: AtomicUsize,
    rng: Mutex<rand::rngs::StdRng>, // Safe, no unsafe impl needed
}
```

## The Breakthrough

Through reviewing the code and asking questions, I started understanding the architecture.

**Me**: How exactly does the WAL help with durability? I understand we write to disk, but how does recovery work?

**Claude**: Excellent question! Let me explain the complete flow:

1. **Write arrives**: `put(key, value)`
2. **WAL append**: Write entry to WAL file with checksum
3. **Fsync**: Force to disk (this is the durability guarantee)
4. **MemTable insert**: Only after WAL confirms
5. **Acknowledge**: Tell client write succeeded

On crash recovery:

- Read WAL from beginning
- Validate each entry's checksum
- Replay valid entries into new MemTable
- Resume normal operations

This explanation was so clear that it sparked an idea...

## Deeper Understanding

**Me**: Your explanation of WAL recovery is really informative. I think we should start a series of database concept articles explaining these concepts using our actual implementation.

**Claude**: That's a great idea! We could cover:

- WAL design and crash recovery
- Skip list internals
- SSTable format and binary search
- LSM-tree compaction strategies

**Me**: Let's do it. Also, we need more tests. Can you add tests for:

- Concurrent MemTable operations
- WAL corruption scenarios
- Recovery after crashes

**Claude**: [Implements comprehensive test suite]

## Reflection

Today established our workflow:

1. I request a feature or component
2. Claude implements with tests
3. I review and ask questions
4. Claude explains and improves based on feedback
5. We iterate until I understand and the code is solid

Starting confidence: 3/10 - "Can I really build a database?"
Ending confidence: 6/10 - "I'm learning by reviewing and questioning!"

The key insight: I don't need to know everything upfront. By reviewing Claude's code and asking questions, I learn naturally.

## Tomorrow's Adventure

Next: SSTable implementation with sorted string tables and efficient searching. I'm curious about the binary format and index design.

---

**Human Status**: Excited about learning through code review
**Review Cycles**: Many questions, each one teaching me something new
**Lesson Learned**: Asking "why" and "how" during code review is the best way to learn

_P.S. Creating CLAUDE.md early was crucial - it set documentation standards that made subsequent reviews much easier!_
