---
layout: post
title: "Day 2: The SSTable Strikes Back (And How Claude Saved My Sanity)"
subtitle: "Building persistent storage, optimizing with binary search, and why Operation doesn't belong in InternalKey"
date: 2025-05-28
day: 2
categories: [development, database, sstable, optimization]
tags: [ferrisdb, rust, lsm-tree, binary-search, architecture]
stats:
  [
    "📊 55 tests passing",
    "📄 5 technical PRs merged",
    "⏱️ Binary search optimization",
    "🔧 Major architectural refactor",
    "🏗️ SSTable writer & reader",
  ]
confidence: "Start: 6/10 ☕☕☕ | End: 8/10 ☕☕☕☕"
compilation_attempts: "34 (I'm getting better!)"
coffee_consumed: "12 cups (don't judge)"
---

## Day 2: When Tables Aren't Tables

*6:47 AM. Coffee #1.*

"Today we build SSTables!" I announced to my rubber duck with newfound confidence. After all, I'd conquered the WAL yesterday. How hard could writing some sorted data to disk be?

*Narrator: He would soon learn the 'T' in SSTable doesn't stand for 'Trivial'.*

## The SSTable Mystery (Coffee #2-3)

*Me:* "Claude, I'm ready for Super Saiyan Tables!"

*Claude:* "...it's Sorted String Table."

*Me:* "That's less exciting."

*Claude:* "But more accurate. Let's design the binary format."

*Me:* "Binary? Can't we just use JSON?"

*Claude:* *silence*

*Me:* "CSV?"

*Claude:* *more silence*

*Me:* "...binary it is."

## Designing the Format (Coffee #4-5)

Claude patiently explained that we needed an efficient on-disk format:

```rust
// What I thought we'd build:
struct SSTable {
    data: Vec<(Key, Value)>  // Just dump it all to disk!
}

// What Claude made us build:
// [Data Block 1][Data Block 2]...[Data Block N][Index Block][Footer]
//
// Each block: 4KB of sorted key-value pairs
// Index: First key of each block for fast lookups
// Footer: Metadata and magic numbers (actual magic, I think)
```

*Me:* "Why can't we just write all the data sequentially?"

*Claude:* "How would you find a specific key in a 1GB file?"

*Me:* "Read the whole thing into memory?"

*Claude:* "..."

*Me:* "Oh. That's bad, isn't it?"

## The Writer Implementation (Coffee #6-7)

After understanding why we need blocks (spoiler: so we don't read entire files), we built the writer:

```rust
pub struct SSTableWriter {
    file: File,
    current_block: Vec<SSTableEntry>,
    index: Vec<IndexEntry>,
    options: SSTableOptions,
}
```

*Compilation attempt #1:* "cannot move out of borrowed content"

*Me:* "It's complaining about borrowing again!"

*Claude:* "You're trying to move the file handle."

*Me:* "I didn't touch it!"

*Claude:* "Show me line 47."

*Me:* "...oh. `self.file = new_file`"

*Claude:* "That's a move."

*Me:* "But it's MY file!"

*Claude:* "Rust doesn't care about your feelings."

### Block Writing Adventures

```rust
impl SSTableWriter {
    pub fn add(&mut self, key: InternalKey, value: Value, operation: Operation) -> Result<()> {
        self.current_block.push(SSTableEntry { key, value, operation });
        
        if self.current_block_size() >= TARGET_BLOCK_SIZE {
            self.flush_block()?;  // Ship it to disk!
        }
        Ok(())
    }
}
```

*Test #1:* PASSED!

*Me:* "IT WORKS! I'M A DATABASE ENGINEER!"

*Claude:* "Great! Now let's build the reader."

*Me:* "There's more?"

## The Reader and The Linear Search Disaster (Coffee #8-9)

Building the reader seemed straightforward:

```rust
// My first attempt (don't judge)
pub fn get(&self, key: &InternalKey) -> Result<Option<Value>> {
    let block = self.read_block_for_key(key)?;
    
    // Just... look through everything?
    for entry in block.entries {
        if entry.key == *key {
            return Ok(Some(entry.value));
        }
    }
    Ok(None)
}
```

*Claude:* "This is O(n) complexity."

*Me:* "Is that bad?"

*Claude:* "How many entries per block?"

*Me:* "Like... 100?"

*Claude:* "So you might check 100 entries for each lookup?"

*Me:* "That's not... optimal?"

*Claude:* "The entries are sorted."

*Me:* "So?"

*Claude:* "What search algorithm works well with sorted data?"

*My brain:* *Windows XP shutdown sound*

## Binary Search Salvation (Coffee #10)

*Claude:* "Binary search. We need binary search."

*Me:* "Oh! Like finding a word in a dictionary!"

*Claude:* "Exactly!"

*Me:* "I never actually learned how that works."

*Claude:* "..."

### The Optimization

After Claude explained binary search using a phone book analogy (and I pretended to know what a phone book was), we optimized:

```rust
// Before: O(n) - checking every entry like a fool
for entry in entries {
    if entry.key == target_key {
        return Some(entry.value);
    }
}

// After: O(log n) - finding entries like a boss
match entries.binary_search_by(|entry| entry.key.cmp(&target_key)) {
    Ok(index) => Ok(Some(entries[index].value.clone())),
    Err(_) => Ok(None)
}
```

*Benchmark results:*
- Linear search: 50 comparisons average
- Binary search: 7 comparisons maximum

*Me:* "IT'S 7X FASTER!"

*Claude:* "For 100 entries. For 1000 entries it would be 50x faster."

*Me:* "UNLIMITED POWER!"

## The API Crisis (Coffee #11)

Then I noticed something weird:

```rust
// Why do I need to specify Operation::Put when reading?
reader.get(&InternalKey::new(key, ts, Operation::Put))?
```

*Me:* "Claude, why do I need to know the operation type to read data?"

*Claude:* "You don't. This API is confusing."

*Me:* "So let's fix it!"

*Claude:* "That means refactoring InternalKey."

*Me:* "How hard could it be?"

*Current test status:* 55 passing

*5 minutes later:* 0 passing, 127 compilation errors

## The Great Refactoring (Coffee #12-14)

*Me:* "CLAUDE, I BROKE EVERYTHING!"

*Claude:* "Let's think about this. What is an InternalKey?"

*Me:* "A key... that's internal?"

*Claude:* "What identifies a unique version of data?"

*Me:* "The key and... timestamp?"

*Claude:* "Right. Is the operation type part of the identity?"

*Me:* "No... it's more like metadata?"

*Claude:* "Exactly! Let's separate concerns."

### The Revelation

```rust
// Before: Confused design
pub struct InternalKey {
    pub user_key: Key,
    pub timestamp: Timestamp,
    pub operation: Operation,  // Why am I here?
}

// After: Clean separation
pub struct InternalKey {
    pub user_key: Key,
    pub timestamp: Timestamp,  // Just identity!
}

pub struct SSTableEntry {
    pub key: InternalKey,
    pub value: Value,
    pub operation: Operation,  // I belong here!
}
```

## Fixing the Universe (Coffee #15-17)

*Compilation attempt #15:* "expected 2 parameters, found 3"

*Me:* "It wants 2 parameters but I'm giving 3."

*Claude:* "The API changed. Update the call sites."

*Me:* "All... 47 of them?"

*Claude:* "All 47 of them."

*2 hours later...*

*Test results:* 55 passing!

*Me:* "WE DID IT! The refactoring worked!"

*Claude:* "How does the API look now?"

```rust
// Clean, intuitive, beautiful!
reader.get(&key, timestamp)?
writer.add(key, value, operation)?
```

*Me:* "It's... it's beautiful. 🥺"

## The Lessons (Coffee #18, Switching to Water)

### What Broke My Brain Today:

1. **Binary formats are hard** - But they're 100x more efficient than JSON (Claude showed me benchmarks)
2. **Binary search is magic** - Seriously, log(n) complexity is basically cheating
3. **API design matters** - Bad APIs hurt every time you use them
4. **Refactoring is scary** - But worth it when you see the clean result
5. **Rust still hates me** - But now it's more like tough love

### Claude's Wisdom Nuggets:

- "Separate identity from metadata"
- "Optimize the common path"
- "If the API feels wrong, it probably is"
- "The compiler errors are trying to help" (lies)

### What Actually Worked:

- Breaking the refactor into steps
- Writing tests first (Claude insisted)
- Using the compiler as a todo list
- Taking breaks (coffee doesn't count)

## The Stats That Matter

- **Lines of code written:** ~1,500
- **Lines of code kept:** ~800  
- **Compilation attempts:** 34
- **Tests written:** 25
- **Coffee consumed:** 12 cups
- **Rubber duck therapy sessions:** 4
- **Times I wanted to use `unsafe`:** 17
- **Times Claude let me:** 0

## What's Next?

We have working SSTables! They can:
- Write data in blocks ✅
- Read data efficiently ✅
- Not make me specify operations when reading ✅
- Make me feel like a real database engineer ✅

**Tomorrow's adventure:** Compaction (Claude says it's like "database Tetris")

**My concerns:**
- What if the blocks don't compact properly?
- What if I accidentally delete all the data?
- What if Rust makes me cry again?

**Claude's response:** "We'll handle it together."

## The Real Victory

Today I learned that building a database isn't just about making things work—it's about making them work *well*. The binary search optimization felt like unlocking a secret level in a game. The refactoring, while painful, made the code so much cleaner.

And yes, I finally understand why they're called Sorted String Tables and not Super Saiyan Tables (though I still think my name is cooler).

**Confidence level:** Started at 6/10, ended at 8/10. I'm no longer afraid of binary formats, and I can explain binary search without using Wikipedia!

---

_This is Day 2 of building FerrisDB with Claude. We turned 55 tests green, optimized the heck out of lookups, and refactored our way to a cleaner architecture._

**Human status:** Hydrated (finally), proud, ready for tomorrow
**AI status:** Still patient, secretly proud of the human's progress

*Next episode: "Day 3: The Compaction Strikes Back (Or: How I Learned to Love Background Threads)"*

**P.S.** - If you see someone at a coffee shop muttering about "binary search in logarithmic time," that's probably me. Say hi, but bring caffeine.