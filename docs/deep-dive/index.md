---
layout: page
title: Technical Deep Dives
subtitle: In-depth exploration of database internals through FerrisDB's implementation
permalink: /deep-dive/
---

Welcome to FerrisDB's technical deep dives! These articles explore fundamental database concepts through our actual implementation, providing both theoretical understanding and practical code examples.

## Storage Engine Fundamentals

<div class="article-grid">
  <div class="article-card">
    <h3><a href="{{ '/deep-dive/wal-crash-recovery/' | relative_url }}">WAL and Crash Recovery</a></h3>
    <p>Understand how Write-Ahead Logs ensure data durability and enable crash recovery. Learn about FerrisDB's WAL format, checksums, and recovery process.</p>
    <div class="article-tags">
      <span class="tag">Durability</span>
      <span class="tag">Recovery</span>
      <span class="tag">WAL</span>
    </div>
  </div>

  <div class="article-card">
    <h3><a href="{{ '/deep-dive/lsm-trees/' | relative_url }}">LSM-Trees Explained</a></h3>
    <p>Discover why LSM-trees revolutionized write performance in modern databases. Explore FerrisDB's implementation from MemTables to compaction.</p>
    <div class="article-tags">
      <span class="tag">LSM-Tree</span>
      <span class="tag">Performance</span>
      <span class="tag">Storage</span>
    </div>
  </div>

  <div class="article-card">
    <h3><a href="{{ '/deep-dive/sstable-design/' | relative_url }}">SSTable Format Design</a></h3>
    <p>Deep dive into FerrisDB's SSTable binary format, block structure, and how we achieve efficient lookups with binary search.</p>
    <div class="article-tags">
      <span class="tag">SSTable</span>
      <span class="tag">Binary Format</span>
      <span class="tag">Day 2</span>
    </div>
  </div>
</div>

## Concurrency and Performance

<div class="article-grid">
  <div class="article-card">
    <h3><a href="{{ '/deep-dive/concurrent-skip-list/' | relative_url }}">Lock-Free Skip Lists</a></h3>
    <p>Learn how FerrisDB uses concurrent skip lists for the MemTable, enabling lock-free reads while maintaining consistency.</p>
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

- **Explains the problem** the technology solves
- **Shows real code** from FerrisDB's implementation
- **Compares approaches** used by other databases
- **Includes performance analysis** where applicable
- **Provides hands-on examples** you can run

These aren't just theoretical explanations - they're based on actual code we've written and lessons we've learned building FerrisDB.

## Contributing

Have a topic you'd like us to cover? Found an error or have a suggestion? Please [open an issue](https://github.com/ferrisdb/ferrisdb/issues) or submit a PR!
