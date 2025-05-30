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

As a CRUD developer, you've probably:

- Wondered why your queries are slow
- Hit mysterious deadlocks
- Struggled with database scaling
- Wanted to understand what's under the hood

**This project shows you exactly how databases work by building one.**

---

## What You'll Learn

### Core Concepts (Available Now)

✅ Why databases need Write-Ahead Logs (WAL) for crash recovery  
✅ How databases store data in memory with Skip Lists  
✅ The SSTable format - how databases organize data on disk  
✅ SSTable reader - efficiently reading sorted data files

### Currently Building

🚧 Basic Operations - Get/Put/Delete operations and batch writes  
🚧 Compaction - how databases merge files efficiently  
🚧 Bloom filters - probabilistic data structures for speed  
🚧 Block cache - keeping hot data in memory

### Coming Soon

⏳ ACID Transactions - MVCC foundation and snapshot isolation  
⏳ Range queries - efficiently querying data ranges  
⏳ Compression - making storage efficient  
⏳ Distribution layer - consensus and replication

---

## Explore The Code (2 minutes)

```bash
## Clone and explore
git clone https://github.com/ferrisdb/ferrisdb
cd ferrisdb

## See the components we're building
ls ferrisdb-storage/src/

## Run tests to see it working
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

**📖 Read the Blog** - Follow our daily progress and learnings  
[Start Reading →]({{ '/blog/' | relative_url }}){: .btn .btn-purple .btn-lg }

**🔧 Start Building** - Jump into the code  
[Get Started →]({{ '/getting-started/' | relative_url }}){: .btn .btn-outline .btn-lg }

**🤔 Database Concepts** - Understand the theory  
[Learn More →]({{ '/database-concepts/' | relative_url }}){: .btn .btn-outline .btn-lg }

---

## The Twist: AI Collaboration

This isn't just another database project. I'm learning WITH Claude AI:

1. **I assign tasks** → "Implement SSTable reader"
2. **Claude codes** → Implements with explanations
3. **I review** → "Why no error handling here?"
4. **We iterate** → Better code through questions

[See How We Work →]({{ '/how-we-work/' | relative_url }}){: .btn .btn-sm .btn-outline }

---

## Real Progress, Real Learning

**Day 2** of Rust development  
**3,694** lines of Rust code  
**44** tests written

No hiding failures. Every refactor, bug, and "aha!" moment is documented.

---

## Join Our Learning Journey

⭐ Star the repo to follow along  
💬 Join discussions on implementation choices  
🐛 Submit issues when something doesn't make sense

[⭐ Star on GitHub](https://github.com/ferrisdb/ferrisdb){: .btn .btn-primary .btn-lg }

---

> "This is the resource I wish existed when I started learning databases. Real code, real problems, real solutions."  
> — A fellow CRUD developer
