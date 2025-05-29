---
layout: post
title: "Day 1: When a CRUD Developer Decided to Build a Database"
subtitle: "Or: How I learned to stop worrying and love the Rust compiler's 126 error messages"
description: "Follow a CRUD developer's journey into database internals with Claude AI. Day 1 covers building the foundation: WAL, MemTable, and surviving 126 Rust compiler errors."
date: 2025-05-27
day: 1
tags: [Architecture, Storage Engine, WAL, MemTable, Rust, Claude Code]
stats:
  [
    "📊 13 tests passing",
    "📄 8 technical PRs merged",
    "🏗️ WAL + MemTable implementation",
    "📖 Complete documentation site",
  ]
confidence: "Start: 3/10 ☕ | End: 6/10 ☕☕☕"
compilation_attempts: "47 (not counting the times I forgot semicolons)"
---

I stared at my terminal, coffee #1 steaming beside me. "I'm going to build a database," I announced to my rubber duck.

<!--more-->

## Table of contents

{: .no_toc .text-delta }

1. TOC
{:toc}

---

## The Morning That Changed Everything

The duck said nothing. It knew what was coming.

Look, I've been a CRUD developer for years. `SELECT * FROM users WHERE id = ?` is basically my native language. But last week, I read about LSM-trees and thought, "How hard could it be?"

_Narrator: It was, in fact, quite hard._

## The Brilliant Plan (Coffee #2)

"I'll build a distributed database from scratch!" I told myself. "Learn Rust and distributed systems at the same time! What could go wrong?"

**My qualifications:**

- Can write SQL queries ✅
- Once successfully used Redis ✅
- Watched a YouTube video about Raft consensus ✅
- Have strong opinions about MongoDB ✅

Clearly, I was ready.

I decided to call it **FerrisDB** (after Ferris, the Rust crab, who would soon become my therapist).

## Enter Claude: My AI Pair Programming Buddy

_Me:_ "Hey Claude, I want to build a distributed database. From scratch. In Rust. Which I've never used."

_Claude:_ "..."

_Me:_ "Claude?"

_Claude:_ "Let's start with architecture design. Have you considered—"

_Me:_ "WAIT. I already drew a diagram!" _shares napkin sketch_

_Claude:_ "That's... a box labeled 'database magic happens here'."

_Me:_ "Yes! The architecture!"

## The Real Architecture (Coffee #3-5)

After Claude gently suggested we might need more than one box, we designed a proper system inspired by FoundationDB:

- **Transaction Coordinator (TC)** - The boss that keeps everyone honest
- **Storage Servers (SS)** - Where data actually lives (not in "magic box")
- **Cluster Controller (CC)** - The responsible adult in the room
- **Client Library** - How normal people talk to our Frankenstein creation

## The Rust Workspace Adventure

_Me:_ "I'll just create a simple Rust project..."

_Rust:_ "Best I can do is 5 crates, a workspace, and 73 lifetime errors."

```
ferrisdb/
├── ferrisdb-core/       # Where types go to live
├── ferrisdb-storage/    # The actual database-y bits
├── ferrisdb-client/     # For people who want to use this thing
├── ferrisdb-server/     # The thing that serves the thing
└── ferrisdb/            # I honestly forgot what this one does
```

_Compilation attempt #1:_ 126 errors

_Me:_ "Claude, why does Rust hate me?"

_Claude:_ "It's not hate. It's tough love. Let's talk about borrowing..."

## Building a Storage Engine (Coffee #6-8)

_Me:_ "Let's use RocksDB!"

_Claude:_ "That would be sensible. But don't you want to understand how it works?"

_Me:_ "..."

_Claude:_ "Let's build an LSM-tree from scratch!"

_Me:_ "LSM? Like... Least Squares Method?"

_Claude:_ "Log-Structured Merge-tree."

_Me:_ "Right. That's what I meant."

### The Grand Design (As Explained by Claude While I Nodded)

```
Write Path: Write Request → WAL → MemTable → (Flush) → SSTable
            (Translation: Data goes places and eventually lands on disk)

Read Path:  Read Request → MemTable → SSTable (L0 → L1 → L2...)
            (Translation: Check RAM first, then dig through files)
```

**The Components I Pretended to Understand:**

- **WAL** - "Write-Ahead Log" (not "Wow, Another Log")
- **MemTable** - The speedy in-memory thing
- **SSTables** - Files that sound like database tables but aren't
- **Compaction** - Marie Kondo for databases

## WAL: My First Victory (Coffee #9)

_Compilation attempt #23:_ "Cannot borrow `self` as mutable because it is also borrowed as immutable"

_Me:_ "Claude, I'm borrowing myself. Is this an existential crisis?"

_Claude:_ "Let's focus on the Write-Ahead Log first."

After Claude explained that WAL wasn't a misspelling of WALL, we built this:

```rust
pub struct WALEntry {
    pub timestamp: u64,
    pub key: Vec<u8>,
    pub value: Option<Vec<u8>>, // None for deletes
    pub operation: Operation,
}
```

_Me:_ "Why do we need a log before writing data?"

_Claude:_ "What happens if the power goes out mid-write?"

_Me:_ "The user gets angry?"

_Claude:_ "Yes, but also data corruption."

_Me:_ "Oh. OH. The log is like a safety net!"

**Features I Actually Understood:**

- Binary encoding (computers like binary, apparently)
- CRC32 checksums (magic corruption-detection numbers)
- Atomic writes (all-or-nothing, like my cooking)

## The Skip List Saga (Coffee #10-12)

_Me:_ "What's a skip list?"

_Claude:_ "Imagine a linked list with express lanes."

_Me:_ "Like a highway?"

_Claude:_ "More like a subway with express stops."

_Me:_ "So... a skipway?"

_Claude:_ "Let's just implement it."

```rust
pub struct MemTable {
    skiplist: Arc<SkipList>,  // The magic subway system
    size: AtomicUsize,        // How full is our train?
    size_limit: usize,        // When to kick passengers off
}
```

_Compilation attempt #38:_ Success!

_Me:_ "IT COMPILED! I'M A SYSTEMS PROGRAMMER!"

_Claude:_ "Now let's make it concurrent."

_Me:_ "The what now?"

### Making It Thread-Safe (Coffee #13-15)

Turns out "lock-free" doesn't mean "free locks at the hardware store." Claude patiently explained:

- **MVCC** = "Multi-Version Concurrency Control" (not "My Very Confusing Code")
- **Epoch-based reclamation** = Janitor for memory (knows when it's safe to clean up)
- **Atomic operations** = Things that happen all at once (unlike my understanding)

## The Bugs That Nearly Broke Me

### The Endianness Incident (Coffee #16)

_Test failure:_ "Expected 42, got 704643072"

_Me:_ "Claude, my database is doing math wrong."

_Claude:_ "That's 42 in big-endian read as little-endian."

_Me:_ "English, please?"

_Claude:_ "You're reading the number backwards."

_Me:_ "OH. Like reading manga?"

_Claude:_ "...sure. Like reading manga."

### The MVCC Mystery (Coffee #17-18)

_Me:_ "Why do we need timestamps on everything?"

_Claude:_ "So we can have multiple versions of the same key."

_Me:_ "Why would we want that?"

_Claude:_ "What if two people update the same record?"

_Me:_ "The last one wins?"

_Claude:_ "What if they need to see different versions?"

_My brain:_ _dial-up modem noises_

### Lock-Free Programming (Coffee #19-20)

_Me:_ "I removed all the locks! It's lock-free now!"

_Claude:_ "That's not what lock-free means."

_Test output:_ "Segmentation fault"

_Me:_ "Why is it segfaulting?"

_Claude:_ "Remember when you removed all the locks?"

_Me:_ "...oh."

## Making It Production-Ready™ (Coffee #21)

_Claude:_ "Let's add documentation."

_Me:_ "The code is self-documenting!"

_Claude:_ "What does `buf` mean?"

_Me:_ "Buffer? Buffering? Buffy the Vampire Slayer?"

_Claude:_ "..."

_Me:_ "Okay, we'll add docs."

**The Quality Checklist:**

- ✅ Documentation (Claude made me)
- ✅ 13 tests passing (only took 47 tries)
- ✅ Error handling (no more `unwrap()` everywhere)
- ✅ Zero Clippy warnings (Clippy is scarier than the borrow checker)
- ✅ Formatted code (rustfmt is my new best friend)

## What's Next? (Coffee #22, switching to tea)

We have a working WAL and MemTable! I can:

- Write data (it goes somewhere!)
- Read data (it comes back!)
- Not crash (mostly!)

**Tomorrow's adventures:**

1. **SSTable Implementation** - Claude says these are "Super Saiyan Tables" (I think he's lying)
2. **Compaction** - Apparently we need to squish our data sometimes
3. **Bloom Filters** - Not a coffee filter, sadly
4. **Integration Tests** - Where we find out what's really broken
5. **Benchmarks** - Where we find out how slow it really is

## Day 1 Lessons: What I Learned (Besides Humility)

### On AI Pair Programming

**What I expected:** "Claude, build me a database!"

**What actually happened:**

- Claude: "Let's understand what we're building first."
- Me: "But I want to code NOW!"
- Claude: "What's your data consistency model?"
- Me: "The... consistent one?"
- Claude: _patiently explains distributed systems_
- Me: _takes notes furiously_

### On Rust

**Before:** "How hard can systems programming be?"

**After:** "The borrow checker is both my greatest enemy and my best friend."

**Real conversation:**

- Me: "Why can't I just use the variable?"
- Claude: "Because you moved it."
- Me: "I didn't touch my keyboard!"
- Claude: "...'move' has a specific meaning in Rust."
- Me: "Oh."

### The Actual Learnings

1. **Rust's ownership model** is like a strict parent - annoying but keeps you safe
2. **Type systems** catch bugs I didn't even know I was writing
3. **Documentation first** sounds boring but saves hours of confusion
4. **AI assistants** are like patient teachers who never judge your 47th compilation error

## The Unexpected Win

At the end of Day 1, I have:

- A working WAL (still not sure that's the right acronym)
- A MemTable with a skip list (the subway thing)
- 13 passing tests (ignore the 34 that didn't pass)
- A new respect for database engineers
- An unhealthy relationship with the Rust compiler

**Confidence level:** Started at 3/10 ("How hard could it be?"), ended at 6/10 ("I know enough to know I know nothing")

## Want to Join This Madness?

I'm building this in the open because misery loves company... I mean, learning is better together!

- **Code**: [GitHub repository]({{ site.project.repo_url }}) (PRs welcome, especially if you understand lifetimes)
- **Design docs**: Written by Claude while I nodded knowingly
- **Daily blogs**: Watch me stumble through database internals!

Tomorrow: SSTable implementation. Claude promises it's "just writing sorted data to disk." I've learned to be suspicious when he says "just."

---

_This is Day 1 of building FerrisDB with Claude. Follow along as we turn a CRUD developer into a systems programmer, one compilation error at a time._

**Human status:** Caffeinated, confused, but committed
**AI status:** Patient, helpful, probably questioning its life choices

_Tomorrow: More adventures in database building, assuming the Rust compiler doesn't make me cry first._
