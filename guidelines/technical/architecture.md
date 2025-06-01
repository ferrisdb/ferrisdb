# Architecture Decisions

Key architectural decisions and design principles for FerrisDB.

**NOTE**: This document serves as the authoritative source of truth for all FerrisDB architectural decisions. Any technical content (blog posts, documentation, code) must align with the decisions documented here. If a better approach is discovered, update this document first before implementing changes.

**Prerequisites**: Basic understanding of distributed systems and database concepts  
**Related**: [Storage Engine](storage-engine.md), [System Invariants](invariants.md), [Performance](performance.md)

## System Architecture

### Overview

FerrisDB is a distributed, transactional key-value database with the following architecture:

```
┌─────────────────┐     ┌─────────────────┐
│ Client Library  │     │ Client Library  │
└────────┬────────┘     └────────┬────────┘
         │                       │
         └───────────┬───────────┘
                     │
            ┌────────▼────────┐
            │  Server (gRPC)  │
            └────────┬────────┘
                     │
            ┌────────▼────────┐
            │ Storage Engine  │
            │   (LSM-tree)    │
            └────────┬────────┘
                     │
          ┌──────────┼──────────┐
          │          │          │
     ┌────▼────┐ ┌───▼────┐ ┌───▼────┐
     │MemTable │ │SSTables│ │  WAL   │
     └─────────┘ └────────┘ └────────┘
```

### Component Boundaries

1. **ferrisdb-core**: Common types and traits
2. **ferrisdb-storage**: Storage engine implementation
3. **ferrisdb-server**: Network server and API
4. **ferrisdb-client**: Client library

## Design Principles

### 1. Educational First

- Code should be clear and understandable
- Prefer explicit over implicit
- Document why, not just what
- Show database concepts in action

### 2. Safety Over Performance

During the learning phase:

- Avoid `unsafe` code unless absolutely necessary
- Use safe abstractions even if slower
- Make performance optimizations explicit and documented

### 3. Modular Design

- Clear separation of concerns
- Well-defined interfaces between components
- Each crate should have a single responsibility
- Dependencies flow in one direction

## Key Decisions

### Storage Engine: LSM-Tree

**Why LSM-tree over B-tree:**

- Better write performance
- Natural fit for append-only operations
- Demonstrates compaction concepts
- Used by RocksDB, LevelDB, Cassandra

**Trade-offs:**

- More complex read path
- Requires background compaction
- Space amplification

### Concurrency: Lock-Free Skip List

**Why skip list for MemTable:**

- Lock-free concurrent operations
- Good cache locality
- Simpler than lock-free B-trees
- Educational value

**Implementation:**

- No `unsafe` code (learning focus)
- Arc-based node management
- Eventually optimize if needed

### Persistence: Write-Ahead Log

**Design choices:**

- Simple append-only format
- CRC32 checksums for integrity
- Synchronous writes by default
- Binary format for efficiency

### API: gRPC

**Why gRPC:**

- Language-agnostic clients
- Built-in code generation
- Streaming support
- Industry standard

## Future Considerations

### Distribution

When we add distribution:

- Raft for consensus
- Range-based sharding
- Learner replicas
- Multi-version concurrency control

### Transactions

Transaction design:

- Optimistic concurrency control
- Snapshot isolation
- Multi-key transactions
- Deterministic transaction ordering

## Trait-Based File Format Design

### Overview

FerrisDB uses a trait-based approach for designing extensible file formats. This pattern allows for modular composition of functionality while maintaining type safety and clear interfaces.

### Core Traits to Implement

When designing a new file format, implement these traits in order:

#### 1. Required Traits

```rust
// Define the basic structure
trait FileFormat {
    type Header: FileHeader;
    type Record;

    fn magic_bytes() -> &'static [u8];
    fn version() -> u32;
}

// Header operations
trait FileHeader {
    fn encode(&self) -> Result<Vec<u8>>;
    fn decode(bytes: &[u8]) -> Result<Self>;
    fn size() -> usize;
}

// File validation
trait ValidateFile {
    fn validate(&self) -> Result<()>;
    fn check_magic_bytes(&self, bytes: &[u8]) -> Result<()>;
    fn verify_version(&self, version: u32) -> Result<()>;
}
```

#### 2. Optional Enhancement Traits

Add these traits based on your file format's requirements:

```rust
// For checksummed headers
trait ChecksummedHeader: FileHeader {
    fn compute_checksum(&self) -> u32;
    fn verify_checksum(&self, expected: u32) -> Result<()>;
}

// For rich metadata
trait FileMetadata {
    fn created_at(&self) -> SystemTime;
    fn file_size(&self) -> u64;
    fn record_count(&self) -> usize;
    fn compression_type(&self) -> Option<CompressionType>;
}

// For indexable formats
trait IndexedFile {
    type IndexEntry;

    fn build_index(&self) -> Result<Vec<Self::IndexEntry>>;
    fn lookup(&self, key: &[u8]) -> Result<Option<u64>>;
}
```

### Trait Composition Pattern

Start with required traits and compose additional functionality:

```rust
// Example: SSTable file format
pub struct SSTableFormat;

impl FileFormat for SSTableFormat {
    type Header = SSTableHeader;
    type Record = KeyValue;

    fn magic_bytes() -> &'static [u8] { b"SSTB" }
    fn version() -> u32 { 1 }
}

// Compose multiple traits for the header
pub struct SSTableHeader {
    // fields...
}

impl FileHeader for SSTableHeader { /* ... */ }
impl ChecksummedHeader for SSTableHeader { /* ... */ }
impl FileMetadata for SSTableHeader { /* ... */ }

// Now you can use all trait methods on SSTableHeader
```

### Error Handling Best Practices

#### 1. Use Error::Corruption for File Format Errors

```rust
// Good: Specific corruption error with context
if header.magic != Self::magic_bytes() {
    return Err(Error::Corruption(format!(
        "Invalid magic bytes: expected {:?}, found {:?}",
        Self::magic_bytes(),
        header.magic
    )));
}

// Bad: Generic error
if header.magic != Self::magic_bytes() {
    return Err(Error::Other("Bad magic"));
}
```

#### 2. Include Context in Errors

```rust
impl FileHeader for MyHeader {
    fn decode(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < Self::size() {
            return Err(Error::Corruption(format!(
                "Header too small: expected {} bytes, got {}",
                Self::size(),
                bytes.len()
            )));
        }

        // Decode with context
        let version = u32::from_le_bytes(
            bytes[4..8].try_into()
                .map_err(|_| Error::Corruption(
                    "Failed to decode version field".to_string()
                ))?
        );

        // Continue decoding...
    }
}
```

#### 3. Validate During Decode

Always validate data during decoding, not as a separate step:

```rust
impl FileHeader for MyHeader {
    fn decode(bytes: &[u8]) -> Result<Self> {
        let header = Self {
            // ... decode fields ...
        };

        // Validate immediately
        header.validate()?;

        // Verify checksum if applicable
        if let Some(checksum) = header.checksum {
            header.verify_checksum(checksum)?;
        }

        Ok(header)
    }
}
```

### Implementation Example

Here's a complete example of implementing a simple indexed file format:

```rust
pub struct IndexedLogFormat;

impl FileFormat for IndexedLogFormat {
    type Header = IndexedLogHeader;
    type Record = LogEntry;

    fn magic_bytes() -> &'static [u8] { b"ILOG" }
    fn version() -> u32 { 1 }
}

pub struct IndexedLogHeader {
    magic: [u8; 4],
    version: u32,
    checksum: u32,
    created_at: u64,
    index_offset: u64,
    record_count: u32,
}

impl FileHeader for IndexedLogHeader {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut buf = Vec::with_capacity(Self::size());
        buf.extend_from_slice(&self.magic);
        buf.extend_from_slice(&self.version.to_le_bytes());
        // ... encode other fields ...
        Ok(buf)
    }

    fn decode(bytes: &[u8]) -> Result<Self> {
        // Decode and validate in one pass
        let header = Self {
            magic: bytes[0..4].try_into().unwrap(),
            version: u32::from_le_bytes(bytes[4..8].try_into().unwrap()),
            // ... decode other fields ...
        };

        // Validate magic bytes
        if &header.magic != IndexedLogFormat::magic_bytes() {
            return Err(Error::Corruption(format!(
                "Invalid magic bytes: {:?}", header.magic
            )));
        }

        // Verify checksum
        let computed = header.compute_checksum();
        if computed != header.checksum {
            return Err(Error::Corruption(format!(
                "Checksum mismatch: expected {}, computed {}",
                header.checksum, computed
            )));
        }

        Ok(header)
    }

    fn size() -> usize { 32 }
}

impl ChecksummedHeader for IndexedLogHeader {
    fn compute_checksum(&self) -> u32 {
        // CRC32 of header fields (excluding checksum itself)
        crc32::checksum_ieee(&self.encode_without_checksum())
    }

    fn verify_checksum(&self, expected: u32) -> Result<()> {
        let computed = self.compute_checksum();
        if computed != expected {
            return Err(Error::Corruption(format!(
                "Header checksum mismatch: expected {}, computed {}",
                expected, computed
            )));
        }
        Ok(())
    }
}
```

### Testing File Formats

Always test your trait implementations thoroughly:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_round_trip() {
        let header = MyHeader::new();
        let encoded = header.encode().unwrap();
        let decoded = MyHeader::decode(&encoded).unwrap();
        assert_eq!(header, decoded);
    }

    #[test]
    fn test_corruption_detection() {
        let header = MyHeader::new();
        let mut encoded = header.encode().unwrap();

        // Corrupt the magic bytes
        encoded[0] = 0xFF;

        let result = MyHeader::decode(&encoded);
        assert!(matches!(result, Err(Error::Corruption(_))));
    }

    #[test]
    fn test_checksum_verification() {
        let header = ChecksummedHeader::new();
        let checksum = header.compute_checksum();

        // Should succeed with correct checksum
        assert!(header.verify_checksum(checksum).is_ok());

        // Should fail with wrong checksum
        assert!(header.verify_checksum(checksum + 1).is_err());
    }
}
```

## Non-Goals

Things we explicitly don't optimize for:

1. **SQL compatibility** - We're a key-value store
2. **Embedded use** - We're building a server
3. **Maximum performance** - We prioritize learning
4. **Production readiness** - This is educational

## Invariants

See [System Invariants](invariants.md) for critical properties that must be maintained.

## References

- [Google's LevelDB Design](https://github.com/google/leveldb/blob/main/doc/impl.md)
- [RocksDB Architecture](https://github.com/facebook/rocksdb/wiki/RocksDB-Overview)
- [FoundationDB Design](https://apple.github.io/foundationdb/)
- [Apache Cassandra Architecture](https://cassandra.apache.org/doc/latest/architecture/)
