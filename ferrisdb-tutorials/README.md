# FerrisDB Tutorials - Learn by Building

Welcome to the FerrisDB tutorial series! This is where CRUD developers learn to build a database from scratch, one component at a time.

## 🎯 Our Mission

Make database internals accessible to every developer. If you can build a web app, you can understand how databases work!

## 📚 Tutorial Series

| Tutorial                                     | Component             | Key Concepts                          | Status         |
| -------------------------------------------- | --------------------- | ------------------------------------- | -------------- |
| [01: Key-Value Store](tutorial-01-kv-store/) | Basic HashMap storage | Rust basics, ownership, testing       | ✅ Ready       |
| 02: Persistence                              | File I/O              | Result, error handling, serialization | 🚧 Coming Soon |
| 03: Write-Ahead Log                          | Durability            | Binary files, crash recovery          | 📋 Planned     |
| 04: MemTable                                 | Concurrent storage    | Arc, RwLock, concurrency              | 📋 Planned     |
| 05: Skip Lists                               | Ordered storage       | Generics, unsafe basics               | 📋 Planned     |
| 06: SSTables                                 | On-disk format        | Binary encoding, iterators            | 📋 Planned     |
| 07: Concurrency                              | Thread safety         | Send/Sync, atomics                    | 📋 Planned     |
| 08: Compaction                               | Background tasks      | Async, channels                       | 📋 Planned     |
| 09: Storage Engine                           | Full integration      | API design, modules                   | 📋 Planned     |
| 10: Performance                              | Optimization          | Benchmarking, profiling               | 📋 Planned     |

## 🚀 Getting Started

### Prerequisites

- Rust installed ([rustup.rs](https://rustup.rs))
- Basic programming knowledge (any language)
- Enthusiasm to learn!

### How to Use These Tutorials

1. **Start with Tutorial 01** - Each builds on the previous
2. **Run the tests** - Every step has tests to verify your understanding
3. **Try the exercises** - Challenge yourself with the practice problems
4. **Check the benchmarks** - See the performance characteristics

### Running a Tutorial

```bash
# Clone the repository
git clone https://github.com/ferrisdb/ferrisdb.git
cd ferrisdb/ferrisdb-tutorials

# Run tutorial 01
cd tutorial-01-kv-store
cargo test
cargo bench
```

## 🧪 Quality Standards

Every tutorial in this series:

- ✅ Has been dogfooded (we completed it ourselves)
- ✅ Includes comprehensive tests for each step
- ✅ Contains concurrent safety tests where applicable
- ✅ Includes performance benchmarks
- ✅ Provides exercises with solutions
- ✅ Maps directly to the tutorial content

## 📖 Learning Path

### Phase 1: Foundation (Tutorials 1-3)

Learn basic Rust and simple storage concepts. You'll build confidence with Rust's ownership model and basic I/O.

### Phase 2: Core Components (Tutorials 4-8)

Build the real database structures. You'll tackle concurrency, data structures, and performance.

### Phase 3: Integration (Tutorials 9-10)

Put it all together into a working storage engine. You'll see how the pieces fit together.

## 🤝 Contributing

Found an issue? Have a suggestion? We want these tutorials to be as clear as possible!

- Open an issue for bugs or confusion
- Submit a PR for improvements
- Share your learning experience

## 📊 Success Metrics

How do we know these tutorials work?

- 🎯 **Completion Rate**: Track how many developers finish
- ⏱️ **Time to Complete**: Stay within estimated times
- 💡 **Concept Mastery**: Learners can explain back
- 🚀 **Confidence Growth**: Ready for the next tutorial

## 🎉 Your Journey Starts Here!

Ready to understand how databases really work? Start with [Tutorial 01: Key-Value Store](tutorial-01-kv-store/) and build your way up to a complete storage engine!

Remember: Every expert was once a beginner. The difference is they started.

---

_Built with ❤️ by the FerrisDB community_
