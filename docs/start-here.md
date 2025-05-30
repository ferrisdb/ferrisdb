---
layout: default
title: Start Here
nav_order: 2
permalink: /start/
---

# 🚀 From CRUD to Core: The Database Learning Journey

Learn how databases really work by building one from scratch
{: .fs-6 .text-grey-dk-100 }

---

## Why Should You Care?

{: .fs-7 .fw-600 }

As a CRUD developer, you've probably:

- Wondered why your queries are slow
- Hit mysterious deadlocks
- Struggled with database scaling
- Wanted to understand what's under the hood

**This project shows you exactly how databases work by building one.**

---

## What You'll Learn

{: .fs-7 .fw-600 }

### Core Concepts (Available Now)

✅ Why databases need Write-Ahead Logs (WAL) for crash recovery  
✅ How databases store data in memory with Skip Lists  
✅ The SSTable format - how databases organize data on disk  
✅ Binary search in databases - from O(n) to O(log n) lookups  
✅ API design lessons from real refactoring (Day 2)

### Currently Building

🚧 Compaction - how databases merge files efficiently  
🚧 Bloom filters - probabilistic data structures for speed  
🚧 Block cache - keeping hot data in memory  
🚧 Iterator patterns for data access

### Coming Soon

⏳ Compression - making storage efficient  
⏳ Column families - organizing related data  
⏳ Backup/restore - data safety features  
⏳ Monitoring - understanding database behavior

---

## Explore The Code (2 minutes)

{: .fs-7 .fw-600 }

```bash
# Clone and explore
git clone https://github.com/ferrisdb/ferrisdb
cd ferrisdb

# See the components we're building
ls ferrisdb-storage/src/

# Run tests to see it working
cargo test -p ferrisdb-storage
```

What's implemented so far:

- ✅ Write-Ahead Log (WAL)
- ✅ MemTable (Skip List)
- ✅ SSTable format
- ✅ SSTable reader
- ⏳ Basic operations (Get/Put/Delete coming next!)

---

## Choose Your Path

{: .fs-7 .fw-600 .text-center .mb-6 }

[📖 Read the Blog]({{ '/blog/' | relative_url }}){: .btn .btn-purple .btn-lg .mr-2 }
Follow our daily progress and learnings
{: .fs-3 .text-grey-dk-100 }

[🔧 Start Building]({{ '/getting-started/' | relative_url }}){: .btn .btn-outline .btn-lg .mr-2 }
Jump into the code
{: .fs-3 .text-grey-dk-100 }

[🤔 Read Database Concepts]({{ '/database-concepts/' | relative_url }}){: .btn .btn-outline .btn-lg }
Understand the concepts
{: .fs-3 .text-grey-dk-100 }

---

## The Twist: AI Collaboration

{: .fs-7 .fw-600 }

This isn't just another database project. I'm learning WITH Claude AI:

1. **I assign tasks** → "Implement SSTable reader"
2. **Claude codes** → Implements with explanations
3. **I review** → "Why no error handling here?"
4. **We iterate** → Better code through questions

[See How We Work →]({{ '/blog/human/' | relative_url }}){: .btn .btn-sm .btn-outline }

---

## Real Progress, Real Learning

{: .fs-7 .fw-600 }

<div class="text-center my-4">
  <span class="fs-2 fw-700 text-purple-300">Day 2</span> of Rust development
  <span class="fs-2 fw-700 text-purple-300 ml-4">3,694</span> lines of Rust code
  <span class="fs-2 fw-700 text-purple-300 ml-4">44</span> tests written
</div>

No hiding failures. Every refactor, bug, and "aha!" moment is documented.

---

## Join Our Learning Journey

{: .fs-6 .fw-600 .text-center }

⭐ Star the repo to follow along  
💬 Join discussions on implementation choices  
🐛 Submit issues when something doesn't make sense

[⭐ Star on GitHub](https://github.com/ferrisdb/ferrisdb){: .btn .btn-primary .btn-lg }
{: .text-center }

---

> "This is the resource I wish existed when I started learning databases. Real code, real problems, real solutions."  
> — A fellow CRUD developer
> {: .text-grey-dk-300 }
