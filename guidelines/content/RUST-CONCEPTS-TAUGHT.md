# Rust Concepts Teaching Tracker

_Source of truth for what Rust concepts have been taught in FerrisDB tutorials_

**Purpose**: Track which Rust concepts have been introduced to ensure we don't assume knowledge that hasn't been taught yet.

## Concept Categories

### 🏗️ Basic Language Constructs

#### Variables & Types

- [ ] `let` bindings and immutability
- [ ] `mut` keyword
- [ ] Type annotations
- [ ] Type inference
- [ ] Shadowing

#### Primitive Types

- [ ] Integers (`i32`, `u64`, etc.)
- [ ] Floating point (`f32`, `f64`)
- [ ] Boolean (`bool`)
- [ ] Character (`char`)
- [ ] String slice (`&str`)

#### Compound Types

- [ ] Tuples
- [ ] Arrays `[T; N]`
- [ ] Slices `&[T]`
- [ ] Vectors `Vec<T>`
- [ ] Strings `String`
- [ ] HashMaps

### 🏛️ Structs & Enums

#### Structs

- [ ] Struct definition
- [ ] Struct instantiation
- [ ] Field access
- [ ] Tuple structs
- [ ] Unit structs
- [ ] Method syntax (`impl` blocks)
- [ ] Associated functions

#### Enums

- [ ] Basic enums
- [ ] Enums with data
- [ ] `Option<T>`
- [ ] `Result<T, E>`
- [ ] Pattern matching basics
- [ ] `match` expressions
- [ ] `if let`
- [ ] `while let`

### 🎯 Functions & Control Flow

#### Functions

- [ ] Function definition
- [ ] Parameters and return values
- [ ] Expressions vs statements
- [ ] Early returns

#### Control Flow

- [ ] `if`/`else`
- [ ] `loop`
- [ ] `while`
- [ ] `for` and ranges
- [ ] `break` and `continue`
- [ ] Loop labels

### 🔑 Ownership & Borrowing

#### Ownership

- [ ] Move semantics
- [ ] Copy trait
- [ ] Clone trait
- [ ] Drop trait

#### Borrowing

- [ ] Immutable references `&T`
- [ ] Mutable references `&mut T`
- [ ] Reference rules
- [ ] Slice references

#### Lifetimes

- [ ] Basic lifetime annotations
- [ ] Lifetime elision
- [ ] Static lifetime
- [ ] Struct lifetimes

### 🚨 Error Handling

- [ ] `Result<T, E>` type
- [ ] `unwrap()` and `expect()`
- [ ] `?` operator
- [ ] `map()` and `and_then()`
- [ ] Custom error types
- [ ] `From` trait for errors
- [ ] `panic!` macro

### 🧩 Traits & Generics

#### Traits

- [ ] Trait definition
- [ ] Implementing traits
- [ ] Derive macros
- [ ] Common traits (Debug, Clone, PartialEq)
- [ ] Trait bounds

#### Generics

- [ ] Generic functions
- [ ] Generic structs
- [ ] Generic enums
- [ ] Generic impl blocks
- [ ] Where clauses

### 🔄 Smart Pointers & Concurrency

#### Smart Pointers

- [ ] `Box<T>`
- [ ] `Rc<T>`
- [ ] `Arc<T>`
- [ ] `RefCell<T>`
- [ ] `Mutex<T>`
- [ ] `RwLock<T>`

#### Concurrency

- [ ] Threads with `std::thread`
- [ ] Message passing with channels
- [ ] Shared state with `Arc<Mutex<T>>`
- [ ] `Send` and `Sync` traits
- [ ] Atomic types

### 📦 Modules & Crates

- [ ] Module system (`mod`)
- [ ] Visibility (`pub`)
- [ ] Use statements
- [ ] External crates
- [ ] Cargo.toml basics

### 🛠️ Advanced Topics

#### Iterators

- [ ] Iterator trait
- [ ] Common iterator methods
- [ ] Collecting iterators
- [ ] Iterator adaptors

#### Closures

- [ ] Closure syntax
- [ ] Capturing variables
- [ ] Move closures
- [ ] Function traits (Fn, FnMut, FnOnce)

#### Unsafe Rust

- [ ] Raw pointers
- [ ] Unsafe functions
- [ ] Unsafe blocks
- [ ] FFI basics

## 📚 Concepts by Tutorial

### Tutorial 1: Building a Simple Key-Value Store

_Status: PUBLISHED_

**Introduced**:

- ✅ `let` bindings and immutability
- ✅ `mut` keyword
- ✅ Struct definition
- ✅ `impl` blocks and methods
- ✅ `HashMap` basics
- ✅ `Option<T>`
- ✅ `&self` vs `&mut self`

**File**: `/docs/src/content/docs/tutorials/01-key-value-store.mdx`

### Tutorial 2: Adding Persistence

_Status: [PLANNED]_

**Introduced**:

- [ ] `Result<T, E>` type
- [ ] `?` operator
- [ ] File I/O basics
- [ ] `use` statements
- [ ] External crates (serde)

**Reinforced**:

- [ ] Error handling patterns
- [ ] Struct methods

**File**: `tutorials/02-persistence.mdx`

### Tutorial 3: Write-Ahead Log

_Status: [PLANNED]_

**Introduced**:

- [ ] Custom error types
- [ ] `From` trait
- [ ] Binary file handling
- [ ] `Vec<u8>` for byte arrays

**Reinforced**:

- [ ] `Result<T, E>` handling
- [ ] File I/O patterns

**File**: `tutorials/03-write-ahead-log.mdx`

## 🔄 Maintenance Instructions

### When Writing a New Tutorial

1. **Before starting**: Check which concepts are already taught
2. **Plan concepts**: Decide which new concepts to introduce (aim for 3-5 per tutorial)
3. **During writing**: Mark each concept as introduced when first explained
4. **After completion**: Update this tracker with:
   - Concepts introduced (with ✅)
   - Concepts reinforced
   - Link to tutorial file

### Format for Updates

```markdown
### Tutorial N: [Title]

_Status: [PLANNED|DRAFT|PUBLISHED]_

**Introduced**:

- ✅ Concept name - Brief description of how it's used
- ✅ Another concept - Context of introduction

**Reinforced**:

- ✅ Previous concept (from Tutorial X) - How it's reinforced

**File**: `tutorials/NN-tutorial-name.mdx`
```

## 📊 Quick Reference Matrix

| Concept                              | Tutorial First Introduced | Tutorials Reinforced |
| ------------------------------------ | ------------------------- | -------------------- |
| `let` bindings                       | Tutorial 1                | All subsequent       |
| `Option<T>`                          | Tutorial 1                | 2, 3, 4, ...         |
| `Result<T, E>`                       | Tutorial 2                | 3, 4, 5, ...         |
| `?` operator                         | Tutorial 2                | 3, 4, 5, ...         |
| _...add as tutorials are created..._ |                           |                      |

## 🎯 Teaching Philosophy

- **Gradual Introduction**: Never use a concept before it's taught
- **Spaced Repetition**: Reinforce concepts in later tutorials
- **Practical Context**: Introduce concepts when they solve real problems
- **CRUD Developer Friendly**: Always relate to web development concepts when possible

---
_Last updated: 2025-06-01_
