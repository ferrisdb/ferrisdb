# FerrisDB Core

Core types, traits, and standards for the FerrisDB storage engine.

## Overview

This crate provides the fundamental building blocks used throughout FerrisDB:
- Common error types and result handling
- Basic data types (Key, Value, Operation)
- Magic numbers and file signatures
- Configuration types

## Design Decisions & Industry Comparison

FerrisDB's design choices are informed by analyzing successful databases. Here's how our standards compare:

### Binary Encoding: Little-Endian Everywhere

| Database | Endianness | Notes |
|----------|------------|-------|
| **FerrisDB** | Little-endian | Consistent across all formats |
| **LevelDB/RocksDB** | Little-endian | Same choice, proven at scale |
| **PostgreSQL** | Native (varies) | Platform-dependent |
| **SQLite** | Big-endian headers | Mixed approach |
| **MySQL InnoDB** | Big-endian | Historical choice |
| **FoundationDB** | Big-endian | Order-preserving encoding |

**Our Rationale**: 
- ✅ **Performance**: No byte swapping on x86/x64 (99% of servers)
- ✅ **Simplicity**: One rule, no special cases
- ✅ **Industry Validation**: Google (LevelDB) and Meta (RocksDB) made the same choice

**Note**: FoundationDB chose big-endian for order-preserving properties in distributed systems, but we optimize for single-node performance.

### File Format Structure

```rust
// FerrisDB standard header (all files)
struct StandardFileHeader {
    magic: u64,        // Component identifier
    version: u32,      // Format version
    flags: u32,        // Feature flags
    header_size: u32,  // Forward compatibility
    reserved: u32,     // Future use
}
```

**Comparison**:
- **RocksDB**: Uses footer with magic number (0x88e241b785f4cff7)
- **PostgreSQL**: 24-byte page header with version field
- **SQLite**: 100-byte header with "SQLite format 3\0"
- **FoundationDB**: Decoupled architecture, format details abstracted

**Our Approach**: Standardized header across all components for consistency.

### Key Design Principles

#### 1. **Checksums: CRC32C**
- **FerrisDB**: CRC32C (hardware accelerated)
- **RocksDB**: CRC32C 
- **PostgreSQL**: Optional CRC32C (since 9.3)
- **SQLite**: No built-in checksums

We follow RocksDB's lead using CRC32C for its hardware acceleration on modern CPUs.

#### 2. **Compression: LZ4 Default**
- **FerrisDB**: LZ4 > Snappy > Zstd
- **RocksDB**: Supports all, Snappy default
- **PostgreSQL**: Optional (TOAST compression)
- **SQLite**: No built-in compression

LZ4 offers the best speed/ratio trade-off for modern SSDs.

#### 3. **String Encoding: Length-Prefixed UTF-8**
```rust
[length: u32][utf8_bytes: length]
```
- **FerrisDB**: Length-prefixed, no nulls
- **RocksDB**: Length-prefixed
- **PostgreSQL**: Varlena format (similar)
- **SQLite**: Null-terminated
- **FoundationDB**: Tuple encoding with type codes

Length prefixes enable O(1) skipping and prevent buffer overruns.

#### 4. **Timestamp Format: Nanoseconds**
```rust
pub type Timestamp = u64; // Nanos since Unix epoch
```
- **FerrisDB**: u64 nanoseconds UTC
- **RocksDB**: Microseconds typically
- **PostgreSQL**: Microseconds 
- **SQLite**: Various formats

Nanoseconds provide future-proof precision for distributed systems.

### Resource Limits

| Limit | FerrisDB | RocksDB | PostgreSQL | FoundationDB |
|-------|----------|---------|------------|--------------|
| Max Key | 64KB | 8MB* | 2.7KB (index) | 10KB |
| Max Value | 16MB | 3GB* | 1GB | 100KB |
| Max File | 2GB | 2GB | 1GB (segment) | N/A** |

*Configurable, defaults shown
**FoundationDB abstracts storage details

Our limits balance practical needs with memory efficiency.

### Magic Numbers

FerrisDB uses 8-byte ASCII magic numbers with "FDB" prefix:
- `FDBSST__` - SSTable files
- `FDBWAL__` - Write-ahead logs
- `FDBMANIF` - Manifest files

This provides:
1. Clear component identification
2. ASCII readability in hex dumps
3. Consistent 8-byte size
4. Namespace isolation

## Standards Documentation

For complete standards, see:
- [Common Standards](/guidelines/technical/common-standards.md) - Comprehensive guide
- [Quick Reference](/guidelines/technical/standards-quick-reference.md) - One-page summary
- [Magic Numbers](/guidelines/technical/magic-numbers.md) - File signatures

## Usage Example

```rust
use ferrisdb_core::{Key, Value, Operation, Result};
use ferrisdb_core::{SSTABLE_MAGIC, validate_magic};

// Basic types
let key: Key = b"user:123".to_vec();
let value: Value = b"John Doe".to_vec();
let op = Operation::Put;

// Magic number validation
let file_header = read_header()?;
validate_magic(&file_header, SSTABLE_MAGIC)?;
```

## Industry Validation

Our design choices align with production databases:

1. **Little-endian** (like LevelDB/RocksDB) for x86 performance
2. **CRC32C checksums** (industry standard) for corruption detection
3. **LZ4 compression** (fastest) matching modern SSD characteristics
4. **Length-prefixed strings** (safer than null-terminated)
5. **Standardized headers** (cleaner than format detection)

## Philosophy

> "Stand on the shoulders of giants, but don't be afraid to improve."

We've studied PostgreSQL's robustness, RocksDB's performance, and SQLite's simplicity. FerrisDB combines proven patterns with modern Rust safety guarantees.

## Contributing

When adding new types or standards:
1. Follow the established patterns
2. Document the rationale
3. Compare with industry practices
4. Add comprehensive tests