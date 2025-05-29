---
layout: post
title: "Day 1: When a CRUD Developer Decided to Build a Database"
subtitle: "Spoiler: The Rust compiler had other plans"
date: 2025-05-27
day: 1
tags: [Architecture, Storage Engine, WAL, MemTable, Rust, Claude Code]
stats:
  [
    "📊 13 tests passing",
    "📄 8 technical PRs merged",
    "☕ 5 cups consumed",
    "🤯 3 existential crises",
  ]
confidence: "Start: 3/10 ☕ | End: 6/10 ☕☕☕"
---

## The Morning That Started It All

I stared at my terminal, coffee #1 steaming beside me, and typed the most ambitious command of my career:

```bash
cargo new ferrisdb
```

"How hard could it be?" I thought. "I've built CRUD apps. I know databases. I use them every day!"

**Narrator**: He did not, in fact, know databases.

**Confidence Level: 3/10** ☕

## When Ambition Met Reality

My grand plan: Build a distributed database from scratch. Learn Rust. Understand distributed systems. What could possibly go wrong?

Claude watched silently as I confidently wrote:

```rust
// This should work, right?
let database = Database::new();
database.put("key", "value");
```

**Compilation Attempts:** |||| |||| |||| |

The Rust compiler laughed. Not a gentle chuckle, but a full-blown, error-message-filled cackle that scrolled past my terminal like credits in a Star Wars movie.

## The Struggle Is Real

For two hours, I battled with:

- 🤦 "What do you mean I can't just store a HashMap on disk?"
- 😤 "Why does everything need to be Send + Sync + 'static + Clone + Debug?"
- 😱 "Expected type `Result<T, E>`, found type `panic!`"

My CRUD brain kept screaming: "In Node.js, this would be ONE LINE!"

**Times I Googled "rust lifetime tutorial":** 11

## Enter Claude, Stage Left

Just when I was about to `rm -rf` the whole project and go back to building todo apps, I asked Claude for help...

💭 **Claude Says:** "Let's think about this systematically. Databases aren't just big HashMaps - they need durability, concurrency, and crash recovery. Let me show you something called a Write-Ahead Log..."

Wait, what? 🤯

## The Architecture Revelation

Claude walked me through what a real database needs:

```text
Write Path: Write Request → WAL → MemTable → (Flush) → SSTable
Read Path:  Read Request → MemTable → SSTable (L0 → L1 → L2...)
```

It was like seeing the Matrix for the first time. Suddenly, my `database.json` file approach seemed... inadequate.

**Key Learning:** Databases aren't magic - they're just really, really clever about managing files and memory!

## Building the Foundation (With Training Wheels)

### The Write-Ahead Log

Claude helped me build a WAL - basically a diary for your database that says "Dear diary, today someone wanted to store 'user:123' = 'Alice'..."

```rust
pub struct WALEntry {
    pub timestamp: u64,
    pub key: Vec<u8>,
    pub value: Option<Vec<u8>>, // None for deletes
    pub operation: Operation,
}
```

"But why not just write directly to the database?" I asked.

Claude patiently explained: "What happens when the power goes out mid-write?"

Oh. 💡

### The MemTable Mystery

Then came the MemTable - an in-memory structure that needs to be:
- Fast for writes
- Fast for reads
- Sorted
- Thread-safe
- Lock-free (whatever that meant)

My solution: "How about a Vec?"

Claude's solution: "How about a concurrent skip list?"

Three hours and 5 coffee cups later, I finally understood that a skip list is basically a linked list that went to the gym and grew multiple levels. Like a subway system for your data!

## Plot Twist: It Actually Worked!

By some miracle (and a lot of Claude's patient explanations), we had:

- ✅ A working WAL with CRC32 checksums
- ✅ A lock-free concurrent skip list (still not 100% sure how it works, but it does!)
- ✅ 13 tests passing (they're green, that's what matters!)
- ✅ Zero clippy warnings (after Claude caught my 47 style violations)

## The Human Truth

Working with Claude today proved something important: AI doesn't replace developers, it amplifies us.

- Claude knew about skip lists, but I decided we needed one
- Claude could write the atomic operations, but I chose the API design
- Claude explained endianness, but I decided on little-endian (still not sure why, but it sounded right)

**The Human-AI Score:** Humans 1, Robots 0 (but wow, what a teammate!)

## Tomorrow's Cliff-Hanger

With the foundation laid, tomorrow we tackle SSTables - apparently, they're not "Super Saiyan Tables" like I thought. Will my brain survive learning about bloom filters? Can I understand compaction without having an actual mental compaction?

Find out in Day 2: "The SSTable Strikes Back"

**Final Confidence Level: 6/10** ☕☕☕

---

**P.S.** If you're a CRUD developer thinking about systems programming, here's my advice: Do it. Yes, you'll feel lost. Yes, the Rust compiler will hurt your feelings. But when that first test passes? Pure magic.

**P.P.S.** Coffee consumed: 5 cups. Rust compiler errors: 126. Times I considered giving up: 3. Times Claude saved the day: countless.

**P.P.P.S.** Special thanks to Ferris the crab 🦀 for being the only thing that smiled at me today.