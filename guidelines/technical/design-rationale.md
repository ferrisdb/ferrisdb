# FerrisDB Design Rationale

This document explains the reasoning behind FerrisDB's technical decisions by comparing with established databases.

**Purpose**: Document why we made specific design choices  
**Prerequisites**: Basic database knowledge  
**Related**: [Common Standards](common-standards.md), [Architecture](architecture.md)

## Executive Summary

FerrisDB follows proven patterns from successful databases while making deliberate improvements:

- **Like LevelDB/RocksDB**: LSM-tree architecture, little-endian encoding
- **Like PostgreSQL**: MVCC, WAL-based durability, extensibility
- **Like SQLite**: Simplicity, embedded-first design
- **Like FoundationDB**: Rigorous testing, clear abstractions
- **Unique to FerrisDB**: Rust safety, standardized formats, educational focus

## Detailed Design Decisions

### 1. Little-Endian Everywhere

**Our Choice**: Consistent little-endian encoding

**Industry Analysis**:
```
Database        | Choice           | Rationale
----------------|------------------|----------------------------------
LevelDB/RocksDB | Little-endian    | x86 optimization
PostgreSQL      | Native/Mixed     | Platform flexibility  
SQLite          | Big-endian       | Platform independence
MySQL InnoDB    | Big-endian       | Historical (SPARC era)
FoundationDB    | Big-endian       | Order-preserving, distributed
FerrisDB        | Little-endian    | Modern CPU optimization
```

**Why We Chose Little-Endian**:
1. **Performance**: Zero-cost on x86/ARM (99% of deployments)
2. **Simplicity**: No special cases or byte swapping
3. **Validation**: Google/Meta made the same choice

**Trade-off**: Less readable in hex dumps (mitigated by tooling)

**FoundationDB Exception**: They chose big-endian for its order-preserving properties across heterogeneous systems in a distributed environment. For our single-node focus, little-endian is more efficient.

### 2. Magic Numbers Design

**Our Choice**: 8-byte ASCII with "FDB" prefix

**Comparison**:
```rust
// FerrisDB
FDBSST__ = 0x4644425353545F5F  // Clear component ID

// RocksDB  
0x88e241b785f4cff7             // Random-looking

// SQLite
"SQLite format 3\0"            // 16-byte string

// PostgreSQL
0xD098                         // Short hex
```

**Rationale**:
- ✅ Component identification at a glance
- ✅ Consistent 8-byte size
- ✅ ASCII readable (when big-endian interpreted)
- ✅ Namespace prevents collisions

### 3. Checksum Algorithm

**Our Choice**: CRC32C (Castagnoli)

**Industry Usage**:
- **CRC32C**: RocksDB, PostgreSQL 9.3+, ext4, Btrfs
- **xxHash**: RocksDB (optional), Kafka
- **CRC32**: Older PostgreSQL, ZIP files
- **None**: SQLite (relies on filesystem)

**Why CRC32C**:
1. **Hardware acceleration**: SSE4.2 instruction
2. **Error detection**: Better than CRC32 classic
3. **Performance**: ~20GB/s on modern CPUs
4. **Proven**: Battle-tested in production

### 4. Compression Strategy

**Our Default**: LZ4

**Database Defaults**:
```
Database      | Default      | Rationale
--------------|--------------|---------------------------
RocksDB       | Snappy       | Google ecosystem
PostgreSQL    | pglz/LZ4     | Backward compatibility
MongoDB       | Snappy       | Balance
Redis         | LZF          | Simplicity
FoundationDB  | None*        | Delegated to layers
FerrisDB      | LZ4          | SSD optimization
```
*FoundationDB delegates compression to higher layers

**Our Prioritization**:
1. **LZ4**: 10GB/s, 2.1x ratio - best for SSD
2. **Snappy**: 5GB/s, 2.0x ratio - compatibility  
3. **Zstd**: 1GB/s, 3.5x ratio - maximum compression

### 5. File Size Limits

**Our Choice**: 2GB default

**Industry Practices**:
- **2GB**: RocksDB SST, older filesystems
- **1GB**: PostgreSQL segments, cloud-friendly
- **4GB**: Modern filesystem limit
- **Unlimited**: SQLite (until filesystem limit)

**Why 2GB**:
- Fits in 32-bit signed integer
- Quick to transfer/backup
- Memory mappable on 32-bit systems
- Matches RocksDB for tool compatibility

### 6. Key/Value Size Limits

**Our Limits**:
- Key: 64KB max
- Value: 16MB max

**Comparison**:
```
Database      | Max Key        | Max Value     | Notes
--------------|----------------|---------------|------------------
FerrisDB      | 64KB          | 16MB          | Practical limits
RocksDB       | 8MB*          | 3GB*          | Configurable
PostgreSQL    | 2.7KB (index) | 1GB           | TOAST for large
MongoDB       | 1KB           | 16MB          | Document limit
Redis         | 512MB         | 512MB         | Memory limited
FoundationDB  | 10KB          | 100KB         | Designed for small KV
```

**Rationale**:
- 64KB keys handle any practical use case
- 16MB values fit in memory comfortably
- Prevents pathological cases
- Encourages proper data modeling

### 7. Block/Page Size

**Our Choice**: 4KB default

**Industry Standards**:
- **4KB**: RocksDB default, OS page size
- **8KB**: PostgreSQL, better for OLAP
- **16KB**: MySQL InnoDB, reduced overhead
- **Variable**: SQLite (1-64KB)

**Why 4KB**:
- Matches OS page size (no waste)
- Good for point queries
- SSD-friendly alignment
- Configurable when needed

### 8. Timestamp Precision

**Our Choice**: Nanoseconds (u64)

**Database Approaches**:
```
Database    | Precision     | Storage  | Range
------------|---------------|----------|------------------
FerrisDB    | Nanosecond    | 8 bytes  | 584 years
PostgreSQL  | Microsecond   | 8 bytes  | 4713 BC - 294276 AD
MySQL       | Microsecond   | 4-8 bytes| 1970-2038/9999
Cassandra   | Microsecond   | 8 bytes  | Custom epoch
RocksDB     | User-defined  | Variable | Application-specific
```

**Why Nanoseconds**:
- Future-proof for high-frequency trading
- Sufficient for global ordering
- Hardware timestamps improving
- Simple implementation (one type)

### 9. Concurrency Model

**Our Approach**: MVCC with optimistic concurrency

**Similar To**:
- PostgreSQL: MVCC pioneers
- RocksDB: Snapshot isolation
- FoundationDB: Optimistic transactions

**Different From**:
- SQLite: Single writer
- Redis: Single-threaded
- Traditional: Lock-based

**Benefits**:
- Readers never block writers
- Natural with LSM architecture
- Scales with cores
- Snapshot consistency

**FoundationDB Insight**: Their distributed MVCC with strict serializability shows that optimistic concurrency can scale globally. We apply similar principles at the node level.

### 10. Error Philosophy

**Our Approach**: Explicit, structured errors

```rust
// FerrisDB style
#[derive(Error, Debug)]
pub enum WalError {
    #[error("Corruption at offset {offset}: {reason}")]
    Corruption { offset: u64, reason: String },
    // ...
}

// vs string errors
return Err("WAL corrupted")  // Bad!
```

**Inspired By**:
- **Rust**: Result<T, E> type system
- **Go**: Explicit error returns
- **PostgreSQL**: SQLSTATE codes

**Benefits**:
- Machine-parseable errors
- Contextual information
- Internationalization ready
- Better debugging

## Design Principles

### 1. **Steal Shamelessly** 
Learn from 40+ years of database research

### 2. **Standardize Aggressively**
One way to do things across components

### 3. **Optimize for the Common Case**
- x86/ARM processors
- SSD storage
- 4KB-64KB objects
- Sub-millisecond operations

### 4. **Make the Right Thing Easy**
- Safe by default
- Performance by default
- Debuggable by default

### 5. **Educational Clarity**
If it's not understandable, it's not finished

## Validation

Our design choices are validated by:

1. **Production Success**: Similar choices in RocksDB (Meta scale)
2. **Performance**: Benchmarks match or exceed alternatives
3. **Simplicity**: Fewer edge cases than mixed approaches
4. **Safety**: Rust prevents entire bug categories

## Future Considerations

As hardware evolves, we may revisit:

1. **Persistent Memory**: Different durability model
2. **GPU Acceleration**: Parallel compression/checksums
3. **Network Storage**: Cloud-native adaptations
4. **Quantum Storage**: Post-2030 considerations

But our fundamental standards provide a solid foundation.

## Conclusion

FerrisDB stands on the shoulders of giants:
- **Architecture** from LevelDB/RocksDB
- **Durability** from PostgreSQL  
- **Simplicity** from SQLite
- **Distribution insights** from FoundationDB
- **Safety** from Rust

We've made deliberate choices optimizing for modern hardware and use cases while maintaining clarity for educational purposes.

Every decision has trade-offs, but our choices form a coherent whole optimized for building a production-quality, educational database in Rust.