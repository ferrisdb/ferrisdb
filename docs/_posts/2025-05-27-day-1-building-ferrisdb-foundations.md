---
layout: post
title: "Day 1: When a CRUD Developer Decided to Build a Database"
subtitle: "Or: How I learned to stop worrying and love the Rust compiler's 126 error messages"
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

## The Morning That Changed Everything

I stared at my terminal, coffee #1 steaming beside me. "I'm going to build a database," I announced to my rubber duck.

The duck said nothing. It knew what was coming.

Look, I've been a CRUD developer for years. `SELECT * FROM users WHERE id = ?` is basically my native language. But last week, I read about LSM-trees and thought, "How hard could it be?"

*Narrator: It was, in fact, quite hard.*

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

*Me:* "Hey Claude, I want to build a distributed database. From scratch. In Rust. Which I've never used."

*Claude:* "..."

*Me:* "Claude?"

*Claude:* "Let's start with architecture design. Have you considered—"

*Me:* "WAIT. I already drew a diagram!" *shares napkin sketch*

*Claude:* "That's... a box labeled 'database magic happens here'."

*Me:* "Yes! The architecture!"

## The Real Architecture (Coffee #3-5)

After Claude gently suggested we might need more than one box, we designed a proper system inspired by FoundationDB:

- **Transaction Coordinator (TC)** - The boss that keeps everyone honest
- **Storage Servers (SS)** - Where data actually lives (not in "magic box")
- **Cluster Controller (CC)** - The responsible adult in the room
- **Client Library** - How normal people talk to our Frankenstein creation

## The Rust Workspace Adventure

*Me:* "I'll just create a simple Rust project..."

*Rust:* "Best I can do is 5 crates, a workspace, and 73 lifetime errors."

```
ferrisdb/
├── ferrisdb-core/       # Where types go to live
├── ferrisdb-storage/    # The actual database-y bits
├── ferrisdb-client/     # For people who want to use this thing
├── ferrisdb-server/     # The thing that serves the thing
└── ferrisdb/            # I honestly forgot what this one does
```

*Compilation attempt #1:* 126 errors

*Me:* "Claude, why does Rust hate me?"

*Claude:* "It's not hate. It's tough love. Let's talk about borrowing..."

## Building a Storage Engine (Coffee #6-8)

*Me:* "Let's use RocksDB!"

*Claude:* "That would be sensible. But don't you want to understand how it works?"

*Me:* "..."

*Claude:* "Let's build an LSM-tree from scratch!"

*Me:* "LSM? Like... Least Squares Method?"

*Claude:* "Log-Structured Merge-tree."

*Me:* "Right. That's what I meant."

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

*Compilation attempt #23:* "Cannot borrow `self` as mutable because it is also borrowed as immutable"

*Me:* "Claude, I'm borrowing myself. Is this an existential crisis?"

*Claude:* "Let's focus on the Write-Ahead Log first."

After Claude explained that WAL wasn't a misspelling of WALL, we built this:

```rust
pub struct WALEntry {
    pub timestamp: u64,
    pub key: Vec<u8>,
    pub value: Option<Vec<u8>>, // None for deletes
    pub operation: Operation,
}
```

*Me:* "Why do we need a log before writing data?"

*Claude:* "What happens if the power goes out mid-write?"

*Me:* "The user gets angry?"

*Claude:* "Yes, but also data corruption."

*Me:* "Oh. OH. The log is like a safety net!"

**Features I Actually Understood:**
- Binary encoding (computers like binary, apparently)
- CRC32 checksums (magic corruption-detection numbers)
- Atomic writes (all-or-nothing, like my cooking)

## The Skip List Saga (Coffee #10-12)

*Me:* "What's a skip list?"

*Claude:* "Imagine a linked list with express lanes."

*Me:* "Like a highway?"

*Claude:* "More like a subway with express stops."

*Me:* "So... a skipway?"

*Claude:* "Let's just implement it."

```rust
pub struct MemTable {
    skiplist: Arc<SkipList>,  // The magic subway system
    size: AtomicUsize,        // How full is our train?
    size_limit: usize,        // When to kick passengers off
}
```

*Compilation attempt #38:* Success!

*Me:* "IT COMPILED! I'M A SYSTEMS PROGRAMMER!"

*Claude:* "Now let's make it concurrent."

*Me:* "The what now?"

### Making It Thread-Safe (Coffee #13-15)

Turns out "lock-free" doesn't mean "free locks at the hardware store." Claude patiently explained:

- **MVCC** = "Multi-Version Concurrency Control" (not "My Very Confusing Code")
- **Epoch-based reclamation** = Janitor for memory (knows when it's safe to clean up)
- **Atomic operations** = Things that happen all at once (unlike my understanding)

## The Bugs That Nearly Broke Me

### The Endianness Incident (Coffee #16)

*Test failure:* "Expected 42, got 704643072"

*Me:* "Claude, my database is doing math wrong."

*Claude:* "That's 42 in big-endian read as little-endian."

*Me:* "English, please?"

*Claude:* "You're reading the number backwards."

*Me:* "OH. Like reading manga?"

*Claude:* "...sure. Like reading manga."

### The MVCC Mystery (Coffee #17-18)

*Me:* "Why do we need timestamps on everything?"

*Claude:* "So we can have multiple versions of the same key."

*Me:* "Why would we want that?"

*Claude:* "What if two people update the same record?"

*Me:* "The last one wins?"

*Claude:* "What if they need to see different versions?"

*My brain:* *dial-up modem noises*

### Lock-Free Programming (Coffee #19-20)

*Me:* "I removed all the locks! It's lock-free now!"

*Claude:* "That's not what lock-free means."

*Test output:* "Segmentation fault"

*Me:* "Why is it segfaulting?"

*Claude:* "Remember when you removed all the locks?"

*Me:* "...oh."

## Making It Production-Ready™ (Coffee #21)

*Claude:* "Let's add documentation."

*Me:* "The code is self-documenting!"

*Claude:* "What does `xlmr_2` mean?"

*Me:* "eXtra... Large... Memory... Region... 2?"

*Claude:* "..."

*Me:* "Okay, we'll add docs."

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

### On AI Pair Programming:

**What I expected:** "Claude, build me a database!"

**What actually happened:** 
- Claude: "Let's understand what we're building first."
- Me: "But I want to code NOW!"
- Claude: "What's your data consistency model?"
- Me: "The... consistent one?"
- Claude: *patiently explains distributed systems*
- Me: *takes notes furiously*

### On Rust:

**Before:** "How hard can systems programming be?"

**After:** "The borrow checker is both my greatest enemy and my best friend."

**Real conversation:**
- Me: "Why can't I just use the variable?"
- Claude: "Because you moved it."
- Me: "I didn't touch my keyboard!"
- Claude: "...'move' has a specific meaning in Rust."
- Me: "Oh."

### The Actual Learnings:

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

*Next episode: "Day 2: The SSTable Strikes Back (And How Claude Saved My Sanity)"*