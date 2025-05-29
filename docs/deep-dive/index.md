---
layout: default
title: Technical Deep Dives
nav_order: 3
has_children: true
permalink: /deep-dive/
---

In-depth exploration of database internals through FerrisDB's implementation
{: .fs-6 .fw-300 }

Welcome to FerrisDB's technical deep dives! These articles explore fundamental database concepts through our actual implementation, providing both theoretical understanding and practical code examples.

{: .note }
> **Who are these articles for?**
>
> - Developers curious about how databases work internally
> - Engineers building data-intensive applications
> - Anyone who's wondered "but how does it actually work?"
>
> No PhD required! We explain complex concepts in plain English with real-world analogies.

{: .important }
> **Article Difficulty Levels**
>
> - 📗 **Beginner**: Assumes only CRUD development experience. Concepts explained with everyday analogies.
> - 📙 **Intermediate**: Some familiarity with Rust and concurrent programming helpful. Includes more complex code examples.
> - 📕 **Advanced**: Solid understanding of systems programming concepts required. Discusses low-level implementation details.

## Storage Engine Fundamentals

### WAL and Crash Recovery

📗 **Beginner** • ⏱️ **15 min read**

Understand how Write-Ahead Logs ensure data durability and enable crash recovery. Learn about FerrisDB's WAL format, checksums, and recovery process.

**Topics:** Durability, Recovery, WAL

[Read Article →](/deep-dive/wal-crash-recovery/){: .btn .btn-purple .fs-5 }

---

### LSM-Trees Explained

📙 **Intermediate** • ⏱️ **20 min read**

Discover why LSM-trees revolutionized write performance in modern databases. Explore FerrisDB's implementation from MemTables to compaction.

**Topics:** LSM-Trees, MemTable, Compaction

[Read Article →](/deep-dive/lsm-trees/){: .btn .btn-purple .fs-5 }

---

### SSTable Format Design

📙 **Intermediate** • ⏱️ **18 min read**

Deep dive into efficient on-disk storage formats. Learn how FerrisDB organizes data for fast reads and space efficiency.

**Topics:** Storage Format, Binary Search, Checksums

[Read Article →](/deep-dive/sstable-design/){: .btn .btn-purple .fs-5 }

---

### Lock-Free Skip Lists

📕 **Advanced** • ⏱️ **25 min read**

Explore concurrent data structures that power FerrisDB's MemTable. Understanding lock-free programming through practical implementation.

**Topics:** Concurrency, Skip Lists, Lock-Free

[Read Article →](/deep-dive/concurrent-skip-list/){: .btn .btn-purple .fs-5 }

---

## Coming Soon

{: .note }
> **More articles in development:**
>
> - Compaction Strategies - How to merge SSTables efficiently
> - MVCC Transactions - Multi-version concurrency control
> - Distributed Consensus - Implementing Raft in FerrisDB
> - Query Planning - From SQL to storage operations

## How to Use These Articles

1. **Start with Beginner articles** if you're new to database internals
2. **Follow the suggested reading order** for a complete understanding
3. **Try the code examples** - all examples are from real FerrisDB implementation
4. **Join the discussion** on GitHub if you have questions

{: .highlight }
> **Learning Path Recommendation**
>
> For a complete understanding of FerrisDB's architecture:
>
> 1. [WAL and Crash Recovery](/deep-dive/wal-crash-recovery/) - Foundation
> 2. [LSM-Trees Explained](/deep-dive/lsm-trees/) - Core concept
> 3. [SSTable Format Design](/deep-dive/sstable-design/) - Storage details
> 4. [Lock-Free Skip Lists](/deep-dive/concurrent-skip-list/) - Advanced concurrency

Ready to start? Begin with [WAL and Crash Recovery](/deep-dive/wal-crash-recovery/) to understand the foundation of database durability.
