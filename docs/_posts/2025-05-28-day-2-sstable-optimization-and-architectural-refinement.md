---
layout: post
title: "Day 2: The SSTable Strikes Back (And How Claude Saved My Sanity)"
subtitle: "A tale of binary search, architectural epiphanies, and why 'Super Saiyan Tables' aren't a thing"
date: 2025-05-28
day: 2
categories: [development, database, sstable, optimization]
tags: [ferrisdb, rust, lsm-tree, binary-search, architecture]
stats:
  [
    "📊 55 tests passing",
    "📄 5 technical PRs merged",
    "☕ 8 cups consumed",
    "🧠 1 major epiphany",
  ]
confidence: "Start: 6/10 ☕☕☕ | End: 8/10 ☕☕☕☕"
---

## The Morning After

I opened my laptop, yesterday's skip list victory still fresh in my mind. Coffee #1 in hand, I confidently declared:

"Today, we build SSTables! How hard could it be? It's just... tables... on disk... right?"

**Narrator**: It was not just tables on disk.

**Confidence Level: 6/10** ☕☕☕

## The Great SSTable Mystery

First, I had to Google what SSTable actually stood for. "Sorted String Table" - okay, that sounds manageable. Like a CSV file but fancier?

💭 **Claude Says:** "Actually, it's a carefully designed binary format with blocks, indexes, checksums, and—"

"Wait, what? Binary? I thought we were past the dark ages!"

## Building My First "Table"

My initial approach was... optimistic:

```rust
// This should work!
let mut file = File::create("data.sstable")?;
for (key, value) in memtable.iter() {
    writeln!(file, "{},{}", key, value)?;
}
```

**Compilation Attempts:** | (it actually compiled! But...)

Claude watched me create what was essentially a glorified CSV file and gently suggested: "What happens when you need to find a specific key in a 10GB file?"

Oh. Right. Performance. 🤦

## The Binary Format Revelation

For three hours, I battled with:

- 🤦 "Why can't I just use JSON?"
- 😤 "What do you mean 'byte order matters'?"
- 😱 "CRC32? I thought that was a Star Wars droid!"

**Times I Googled "what is little endian":** 7

Claude patiently explained that databases need structure - specific byte layouts, checksums for corruption detection, and indexes for fast lookups. It was like learning that IKEA furniture actually comes with instructions, not just a pile of wood and hope.

## Enter Binary Search (My New Best Friend)

After implementing the basic SSTable writer, we had a working reader. But then I tested it with 1000 entries...

```rust
// My first attempt: Linear search
for entry in block.entries {
    if entry.key == target {
        return Some(entry.value);
    }
}
// Time: 45ms per lookup 😱
```

💭 **Claude Says:** "You know, since the entries are sorted, you could use binary search..."

Binary search! Of course! It's like having a phone book and actually using alphabetical order instead of reading every name from A to Z!

```rust
// After optimization
match entries.binary_search_by(|e| e.key.cmp(&target)) {
    Ok(idx) => Some(entries[idx].value.clone()),
    Err(_) => None,
}
// Time: 3ms per lookup 🚀
```

The feeling when those benchmarks improved? Better than coffee. (Almost.)

## The Architectural Plot Twist

Just when I thought we were done, I noticed something weird in our API:

```rust
// Why do I need to specify Operation::Put when reading?
reader.get(&InternalKey::new(key, ts, Operation::Put))?
```

It was like needing to know if someone was married to look up their phone number. Made no sense!

That's when the epiphany hit (with Claude's help): Operation isn't part of the key's identity - it's metadata about what happened to that key. Mind. Blown. 🤯

## The Great Refactor of Day 2

What followed was a cascade of changes:

1. Ripped `Operation` out of `InternalKey`
2. Created `SSTableEntry` to hold the operation
3. Updated the binary format
4. Fixed approximately 73 broken tests
5. Questioned all my life choices
6. Had another coffee
7. Fixed the remaining 42 tests

But when it was done? *Chef's kiss* 👨‍🍳

```rust
// Beautiful, clean API
reader.get(&key, timestamp)?  // No more Operation nonsense!
```

## Plot Twist: Everything Is Connected

Halfway through the refactor, I realized this change would ripple through EVERYTHING. The skip list, the WAL, future components I hadn't even built yet...

My CRUD brain: "In JavaScript, I'd just add a field and call it a day!"
My emerging systems brain: "But this is better. This is *right*."

## The Human Truth Continues

Today reinforced what I learned yesterday: Claude is like having a senior engineer who never gets tired of my questions.

- I decided we needed fast lookups; Claude showed me binary search
- I felt the API was wrong; Claude helped me understand why
- I wanted to give up during the refactor; Claude kept the big picture in view

**The Human-AI Score:** Humans still leading, but it's a team sport!

## Tomorrow's Cliff-Hanger

With SSTables conquered and our architecture refined, tomorrow we face the boss battle: Compaction. 

Will I understand why we need to merge files? Can my brain handle bloom filters? (Still not sure if they're related to flowers.) And what the heck is "level-based compaction"?

Find out in Day 3: "The Compaction Strikes Back" (or "How I Learned to Stop Worrying and Love Background Threads")

**Final Confidence Level: 8/10** ☕☕☕☕

---

**P.S.** To all the CRUD developers out there: Yes, binary formats are scary. Yes, you'll miss JSON. But when you see those lookup times drop from O(n) to O(log n)? Pure. Magic. ✨

**P.P.S.** Coffee consumed: 8 cups. Tests broken: 115. Tests fixed: 115. Architectural epiphanies: 1 (but what an epiphany!).

**P.P.P.S.** Current status: Starting to dream in Rust. Send help (or more coffee).