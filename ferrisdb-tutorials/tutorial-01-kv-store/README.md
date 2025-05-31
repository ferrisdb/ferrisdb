# Tutorial 01: Building a Key-Value Store

Welcome to your first FerrisDB tutorial! In this tutorial, you'll build a simple in-memory key-value store from scratch, learning fundamental Rust concepts along the way.

## 🎯 What You'll Build

A working key-value store that can:

- Store string key-value pairs
- Retrieve values by key
- Handle missing keys safely
- Count stored entries

## 🦀 Rust Concepts You'll Learn

- **Variables & Mutability**: `let` vs `let mut`
- **Structs & Methods**: Creating custom types
- **Ownership**: `&self` vs `&mut self`
- **Standard Collections**: Using `HashMap`
- **Option Type**: Safe null handling

## 📚 Database Concepts

- **Key-Value Model**: The simplest database abstraction
- **In-Memory Storage**: Trading durability for speed
- **Hash Tables**: O(1) average-case performance

## 🚀 Getting Started

### Running the Code

```bash
# Run all tests
cargo test

# Run specific step tests
cargo test step_01
cargo test step_02
cargo test step_03

# Run integration tests
cargo test integration

# Run concurrent tests (educational)
cargo test concurrent

# Run benchmarks
cargo bench
```

### Following the Tutorial

1. Start with the tutorial at [ferrisdb.org/learn](https://ferrisdb.org/learn)
2. Copy each code block as you progress
3. Run the corresponding step test to verify
4. Complete the full implementation
5. Try the exercises!

## 📁 Project Structure

```
tutorial-01-kv-store/
├── src/
│   └── lib.rs              # Final implementation
├── tests/
│   ├── step_01_tests.rs    # Test creating the struct
│   ├── step_02_tests.rs    # Test with HashMap field
│   ├── step_03_tests.rs    # Test set() method
│   ├── integration_tests.rs # Complete functionality
│   └── concurrent_tests.rs  # Thread safety demos
├── benches/
│   └── performance.rs       # Performance validation
└── exercises/
    ├── challenge_01_delete.rs
    └── solutions/
        └── challenge_01_solution.rs
```

## 🧪 Test-Driven Learning

Each step has corresponding tests:

**Step 1**: Create empty struct

```rust
cargo test step_01
```

**Step 2**: Add HashMap storage

```rust
cargo test step_02
```

**Step 3**: Implement set() method

```rust
cargo test step_03
```

**Complete**: All functionality with get()

```rust
cargo test integration
```

## 📊 Performance Characteristics

Run benchmarks to see HashMap's O(1) performance:

```bash
cargo bench
```

You'll see that performance remains constant regardless of the number of stored items (within reason).

## 🔍 Key Insights from Dogfooding

While creating this tutorial, we discovered:

1. **Ownership Confusion**: New Rustaceans often struggle with when to use `&str` vs `String`. We address this explicitly.

2. **Option Type**: The concept of `Option<T>` is foreign to many. We introduce it gradually with clear analogies.

3. **Mutability**: The distinction between `&self` and `&mut self` needs careful explanation.

4. **Testing**: Many developers don't know how to write Rust tests. We show the pattern clearly.

## 🎯 Exercises

After completing the tutorial, try the challenges in the `exercises/` directory:

1. **Delete Method**: Add ability to remove entries
2. **TTL Support**: Add expiration to entries
3. **Case-Insensitive**: Make keys case-insensitive
4. **Prefix Scan**: Find all keys with a prefix

## 🚧 Not Production Ready!

This is a learning implementation. A production key-value store would need:

- Thread safety (see Tutorial 4)
- Persistence (see Tutorial 2)
- Error handling (see Tutorial 3)
- Better performance (see Tutorial 5)

## 📈 Next Steps

Ready for more? Continue to:

- **Tutorial 2**: Add persistence to survive restarts
- **Tutorial 3**: Implement a write-ahead log
- **Tutorial 4**: Build a thread-safe MemTable

## 🤝 Found an Issue?

If something is confusing or broken:

1. Check you're using Rust 1.81.0 or later
2. Ensure you've run `cargo test` in this directory
3. Open an issue with your error message

Remember: The goal is to learn, not to build the fastest database. Take your time and understand each concept!

---

Happy learning! 🎉
