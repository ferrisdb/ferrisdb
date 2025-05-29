---
layout: default
title: Deep Dives
nav_order: 3
has_children: true
permalink: /deep-dive/
---

# Technical Deep Dives
{: .no_toc }

In-depth exploration of database internals through FerrisDB's implementation
{: .fs-6 .fw-300 }

Welcome to FerrisDB's technical deep dives! These articles explore fundamental database concepts through our actual implementation, providing both theoretical understanding and practical code examples.

**Who are these articles for?**

- Developers curious about how databases work internally
- Engineers building data-intensive applications
- Anyone who's wondered "but how does it actually work?"

No PhD required! We explain complex concepts in plain English with real-world analogies.

## Article Difficulty Levels

- **Beginner**: Assumes only CRUD development experience. Concepts explained with everyday analogies.
- **Intermediate**: Some familiarity with Rust and concurrent programming helpful. Includes more complex code examples.
- **Advanced**: Solid understanding of systems programming concepts required. Discusses low-level implementation details.

## Storage Engine Fundamentals

<div class="article-grid">
  <div class="article-card">
    <h3><a href="{{ '/deep-dive/wal-crash-recovery/' | relative_url }}">WAL and Crash Recovery</a></h3>
    <p>Understand how Write-Ahead Logs ensure data durability and enable crash recovery. Learn about FerrisDB's WAL format, checksums, and recovery process.</p>
    <div class="article-meta">
      <span class="difficulty beginner">Beginner</span>
      <span class="reading-time">15 min read</span>
    </div>
    <div class="article-tags">
      <span class="tag">Durability</span>
      <span class="tag">Recovery</span>
      <span class="tag">WAL</span>
    </div>
  </div>

  <div class="article-card">
    <h3><a href="{{ '/deep-dive/lsm-trees/' | relative_url }}">LSM-Trees Explained</a></h3>
    <p>Discover why LSM-trees revolutionized write performance in modern databases. Explore FerrisDB's implementation from MemTables to compaction.</p>
    <div class="article-meta">
      <span class="difficulty beginner">Beginner</span>
      <span class="reading-time">15 min read</span>
    </div>
    <div class="article-tags">
      <span class="tag">LSM-Tree</span>
      <span class="tag">Performance</span>
      <span class="tag">Storage</span>
    </div>
  </div>

  <div class="article-card">
    <h3><a href="{{ '/deep-dive/sstable-design/' | relative_url }}">SSTable Format Design</a></h3>
    <p>Deep dive into FerrisDB's SSTable binary format, block structure, and how we achieve efficient lookups with binary search.</p>
    <div class="article-meta">
      <span class="difficulty intermediate">Intermediate</span>
      <span class="reading-time">20 min read</span>
    </div>
    <div class="article-tags">
      <span class="tag">SSTable</span>
      <span class="tag">Binary Format</span>
      <span class="tag">Performance</span>
    </div>
  </div>
</div>

## Concurrency and Performance

<div class="article-grid">
  <div class="article-card">
    <h3><a href="{{ '/deep-dive/concurrent-skip-list/' | relative_url }}">Lock-Free Skip Lists</a></h3>
    <p>Learn how FerrisDB uses concurrent skip lists for the MemTable, enabling lock-free reads while maintaining consistency.</p>
    <div class="article-meta">
      <span class="difficulty intermediate">Intermediate</span>
      <span class="reading-time">20 min read</span>
    </div>
    <div class="article-tags">
      <span class="tag">Concurrency</span>
      <span class="tag">Skip List</span>
      <span class="tag">Lock-Free</span>
    </div>
  </div>
</div>

## Coming Soon

<div class="coming-soon">
  <ul>
    <li><strong>Compaction Strategies</strong> - Level-based vs size-tiered compaction trade-offs</li>
    <li><strong>MVCC Implementation</strong> - Multi-version concurrency control for transactions</li>
    <li><strong>Bloom Filters</strong> - Probabilistic data structures for faster lookups</li>
    <li><strong>Block Cache Design</strong> - LRU cache implementation and memory management</li>
  </ul>
</div>

## About These Articles

Each deep dive article:

- **Explains the problem** the technology solves (why should you care?)
- **Shows real code** from FerrisDB's implementation (not pseudocode!)
- **Compares approaches** used by other databases (learn from the giants)
- **Includes performance analysis** where applicable (with actual numbers)
- **Provides hands-on examples** you can run (try it yourself!)

These aren't just theoretical explanations - they're based on actual code we've written and lessons we've learned building FerrisDB. We share our mistakes and "aha!" moments so you can learn alongside us.

## Contributing

Have a topic you'd like us to cover? Found an error or have a suggestion? Please [open an issue](https://github.com/ferrisdb/ferrisdb/issues) or submit a PR!
