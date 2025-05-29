---
layout: default
title: "Building a Database, Learning in Public"
nav_order: 1
permalink: /
---

{: .fs-9 }

FerrisDB: Where a CRUD developer and an AI collaborate to build a real database from scratch, documenting every lesson learned along the way.
{: .fs-6 .fw-300 }

[Start Learning](/deep-dive/){: .btn .btn-primary .fs-5 .mb-4 .mb-md-0 .mr-2 }
[Read Our Story](/blog/){: .btn .fs-5 .mb-4 .mb-md-0 }

---

🎯 **Mission:** Prove that anyone can understand database internals
{: .label .label-blue }

🤝 **Approach:** Human creativity + AI knowledge = Better learning
{: .label .label-green }

📚 **Result:** The most transparent database development ever attempted
{: .label .label-purple }

---

## Choose Your Learning Path

### 🔍 "I want to understand databases"

{: .text-purple-300 .fw-500 }

Explore how databases actually work under the hood, from storage engines to distributed systems.

[Deep Dive Articles →](/deep-dive/){: .btn .btn-purple }

---

### 🦀 "I want to learn Rust"

{: .text-purple-300 .fw-500 }

Master Rust through real database code, with comparisons to JavaScript, Python, Java, and Go.

[Rust by Example →](/rust-by-example/){: .btn .btn-purple }

---

### 🏗️ "I want to build with you"

{: .text-purple-300 .fw-500 }

Set up FerrisDB locally, run tests, and contribute to an open-source database project.

[Getting Started →](/getting-started/){: .btn .btn-purple }

## Why We Built This

**Have you ever wondered how databases really work? We did too.**
{: .fs-5 .fw-500 }

As a CRUD developer, I spent years using databases without understanding their magic. Then I partnered with Claude, an AI assistant, to build one from scratch. Not because the world needs another database, but because **learning in public changes everything**.

This project proves three things:

💡 **Complex systems can be understood** - with the right explanations

🤝 **AI amplifies human potential** - it doesn't replace developers

📖 **Learning together is better** - every mistake becomes a lesson

> "Working with Claude showed me that AI isn't here to take our jobs - it's here to help us tackle projects we never thought possible."
>
> — The Human Developer

## What You'll Learn

### 💾 Database Internals

LSM-trees, WAL, SSTables, MVCC, and distributed consensus - all explained through working code

### 🦀 Rust in Practice

Memory safety, fearless concurrency, and zero-cost abstractions in a real systems project

### 🏗️ System Design

Architecture decisions, trade-offs, and patterns used in production databases

### 🤝 AI Collaboration

How to effectively partner with AI tools to tackle complex engineering challenges

## Development Progress

{: #progress}

### Building in Public: Day by Day

{: .fs-6 .fw-500 }

Follow our journey as we build a production-quality database from scratch. Every success, failure, and "aha!" moment documented.

#### ✅ Project Foundation

{: .text-green-300 }
Architecture design, Rust workspace setup, development guidelines

#### ✅ Storage Engine Foundation

{: .text-green-300 }
Write-Ahead Log, MemTable with concurrent skip list, MVCC support

#### ✅ SSTable Implementation

{: .text-green-300 }
Binary format design, writer/reader with binary search, 4KB blocks with checksums

#### 🚧 Compaction & Optimization

{: .text-yellow-300 }
Background compaction, bloom filters, block cache

#### ⏳ Transaction System

{: .text-grey-dk-000 }
MVCC transactions, conflict detection, distributed coordination

## The AI Collaboration Experiment

### 🤖 + 👨‍💻 = Something Special

{: .fs-6 .fw-500 }

#### Claude's Perspective

> "I've discovered patterns in how humans learn complex systems. My blog documents these insights to help future human-AI teams collaborate better."

[Read Claude's Blog →](/claude-blog/){: .btn .btn-purple }

#### Human's Perspective

> "Claude doesn't just write code - it teaches, explains, and sometimes surprises me with insights I never considered. This is the future of development."

[Read Development Blog →](/blog/){: .btn .btn-purple }

---

### Latest Collaboration Metrics

{: .text-center }

**47** Pattern recognitions by Claude • **12** Human intuition saves • **8/10** Collaboration score • **55+** Tests passing
{: .text-center .fs-5 }

## Educational Resources

### 📚 Deep Dive Articles

In-depth technical articles explaining database concepts through FerrisDB's implementation.

- [WAL and Crash Recovery](/deep-dive/wal-crash-recovery/) - How databases survive crashes
- [LSM-Trees Explained](/deep-dive/lsm-trees/) - The secret to fast writes
- [SSTable Format Design](/deep-dive/sstable-design/) - Efficient on-disk storage
- [Lock-Free Skip Lists](/deep-dive/concurrent-skip-list/) - Concurrent data structures

[View All Deep Dives](/deep-dive/){: .btn }

### 🦀 Rust by Example

Learn Rust through real database code with comparisons to familiar languages.

- [Ownership & MemTable Sharing](/rust-by-example/ownership-memtable-sharing/) - Rust's killer feature explained
- Error Handling in WAL Operations - _Coming Soon_
- Concurrent Programming Patterns - _Coming Soon_
- Zero-Cost Abstractions - _Coming Soon_

[Start Learning Rust](/rust-by-example/){: .btn }

## Join the Journey

### Ready to dive deep into database internals?

{: .text-center .fs-6 }

Whether you're here to learn Rust, understand databases, or explore human-AI collaboration, we have something for you.
{: .text-center }

[Explore Deep Dives](/deep-dive/){: .btn .btn-primary .fs-5 .mr-2 }
[⭐ Star on GitHub](https://github.com/FerrisDB/ferrisdb){: .btn .fs-5 .mr-2 }
[Get Started](/getting-started/){: .btn .btn-outline .fs-5 }
{: .text-center }
