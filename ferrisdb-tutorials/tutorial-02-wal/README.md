# Tutorial 2: Building a Write-Ahead Log

Welcome to Tutorial 2! Today we're building a Write-Ahead Log (WAL) - the component that makes databases durable.

## What You'll Learn

**Rust Concepts:**
- Binary file I/O with `std::fs::File`
- Error handling with `Result<T, E>` and `?`
- Binary serialization with `byteorder`
- Checksums with `crc32fast`
- Builder pattern for configuration
- Custom error types with `thiserror`

**Database Concepts:**
- Write-Ahead Logging (WAL) fundamentals
- Durability and crash recovery
- Binary formats vs text formats
- Checksums for data integrity
- fsync and durability guarantees
- Performance vs safety trade-offs

## Tutorial Structure

```
src/
├── lib.rs           # WAL implementation
└── main.rs          # Demo application

tests/
├── step_01_tests.rs # Basic append/read
├── step_02_tests.rs # Binary format
├── step_03_tests.rs # Checksums
├── step_04_tests.rs # Recovery
├── step_05_tests.rs # Sync modes
└── integration_tests.rs

exercises/
├── challenge_01_compression.rs
├── challenge_02_snapshots.rs
├── challenge_03_replication.rs
└── solutions/
```

## Running the Tutorial

```bash
# Run all tests
cargo test

# Run specific step
cargo test step_01

# Run the demo
cargo run

# Run benchmarks
cargo bench
```

## The Story

In Tutorial 1, we built an in-memory key-value store. But there's a problem - when your program crashes, all data is lost! Today we'll fix that by building a Write-Ahead Log, the same technique used by PostgreSQL, SQLite, and most production databases.

Ready? Let's make your data survive anything! 🚀