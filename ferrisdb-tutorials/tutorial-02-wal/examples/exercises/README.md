# Tutorial 2: Practice Challenges

These exercises will help you extend the WAL implementation with advanced features.

## Running Exercises

```bash
# Test your implementations
cargo test --example exercises

# Check if your code compiles
cargo check --example exercises
```

## Challenges

### Challenge 1: Compression
**File**: `challenge_01_compression.rs`

Add optional compression to reduce WAL file size. This is especially useful for repetitive data.

**Requirements**:
- Use the `flate2` crate for gzip compression
- Make compression optional via the builder pattern
- Compress after calculating checksums
- Update the file header to indicate compression

### Challenge 2: Log Rotation
**File**: `challenge_02_rotation.rs`

Implement log rotation when files reach a size limit. This prevents unbounded file growth.

**Requirements**:
- Rotate when `max_file_size` is reached
- Use numbered suffixes: `data.wal`, `data.wal.1`, `data.wal.2`
- Recovery must read all rotated files in order
- Handle rotation failures gracefully

### Challenge 3: Concurrent Access
**File**: `challenge_03_concurrent.rs`

Allow multiple readers while writing continues. This improves performance in multi-threaded applications.

**Requirements**:
- Use `Arc<RwLock<WAL>>` for thread-safe access
- Implement snapshot reads that don't block writers
- Ensure read consistency during concurrent writes
- Test with multiple threads

## Solutions

Complete solutions are available in the `solutions/` directory. Try to solve the challenges yourself first!

## Tips

- Start with the simplest challenge first
- Write tests to verify your implementation
- Think about error handling - what can go wrong?
- Consider performance implications of your design