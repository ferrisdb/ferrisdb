# FerrisDB Common Standards

This document defines common standards that MUST be followed across all FerrisDB components to ensure consistency, maintainability, and interoperability.

**Purpose**: Establish standards before implementing new components  
**Prerequisites**: Understanding of binary formats and system design  
**Related**: [Magic Numbers](magic-numbers.md), [Architecture](architecture.md), [Storage Engine](storage-engine.md)

## Binary Encoding Standards

### Byte Order (Endianness)
- **Standard**: Little-endian for ALL binary data
- **No exceptions**: Including magic numbers, timestamps, lengths, etc.
- **Rationale**: Consistency and performance on x86/x64

### Integer Encoding
```rust
// Fixed-size integers
u8, u16, u32, u64    // Use these for known-size fields
i8, i16, i32, i64    // Signed variants when needed

// Variable-length integers (for space optimization)
// Use LEB128 encoding for:
// - Array lengths where most values are small
// - Optional field indicators
// - Forward compatibility
```

### String Encoding
- **Character encoding**: UTF-8 exclusively
- **Length prefix**: u32 little-endian (max 4GB strings)
- **No null terminators**: Length-prefixed instead

```rust
// Standard string format
[length: u32][utf8_bytes: length]
```

### Binary Data Encoding
- **Length prefix**: u32 or u64 depending on expected size
- **No padding**: Pack data tightly
- **Alignment**: Not required (handle in memory if needed)

## File Format Standards

### File Header Structure
Every FerrisDB file MUST start with:

```rust
struct StandardFileHeader {
    magic: u64,        // Component-specific magic
    version: u32,      // Format version number
    flags: u32,        // Feature flags
    header_size: u32,  // Size of full header (for forward compat)
    reserved: u32,     // Must be 0, for future use
    // Component-specific fields follow...
}
```

### Version Numbers
- **Format**: Major.Minor (e.g., 1.0, 2.1)
- **Encoding**: `(major << 16) | minor` as u32
- **Compatibility**: 
  - Same major = forward compatible
  - Different major = migration required

### Feature Flags
Standard flags across all components:
```rust
const FLAG_COMPRESSED: u32 = 1 << 0;    // Data is compressed
const FLAG_ENCRYPTED: u32 = 1 << 1;     // Data is encrypted
const FLAG_CHECKSUM: u32 = 1 << 2;      // Has checksums
const FLAG_EXPERIMENTAL: u32 = 1 << 31;  // Experimental feature
```

## Checksum Standards

### Algorithm
- **Primary**: CRC32C (hardware accelerated on modern CPUs)
- **Alternative**: xxHash64 for performance-critical paths
- **Cryptographic**: Blake3 when security matters

### Placement
- **Block level**: Every 4KB-64KB of data
- **Entry level**: Optional for critical data
- **File level**: In footer/trailer

### Format
```rust
struct ChecksummedBlock {
    data: Vec<u8>,
    checksum: u32,  // CRC32C of data
}
```

## Timestamp Standards

### Format
- **Type**: u64 nanoseconds since Unix epoch
- **Timezone**: Always UTC
- **Special values**:
  - 0: Unknown/not set
  - u64::MAX: Infinity/forever

### Usage
```rust
pub type Timestamp = u64;  // Nanoseconds since 1970-01-01 00:00:00 UTC

// Helper functions in ferrisdb_core
pub fn current_timestamp() -> Timestamp;
pub fn timestamp_to_rfc3339(ts: Timestamp) -> String;
```

## Key Encoding Standards

### User Keys
- **Format**: Raw bytes (Vec<u8>)
- **Ordering**: Lexicographic byte ordering
- **Max length**: 64KB (practical limit)

### Internal Keys
```rust
// Standard internal key format for MVCC
struct InternalKey {
    user_key: Vec<u8>,
    timestamp: Timestamp,  // For versioning
    sequence: u64,         // Tie-breaker
    key_type: KeyType,     // Put/Delete
}

// Encoding: [user_key][timestamp:8][sequence:8][type:1]
```

### Key Prefixes
Reserved prefixes for system use:
- `_system:` - Internal system keys
- `_meta:` - Metadata keys
- `_temp:` - Temporary keys

## Error Handling Standards

### Error Types
```rust
// Every component defines specific errors
#[derive(Error, Debug)]
pub enum ComponentError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Corruption detected: {0}")]
    Corruption(String),
    
    #[error("Version mismatch: expected {expected}, got {actual}")]
    VersionMismatch { expected: u32, actual: u32 },
    
    // Component-specific errors...
}
```

### Error Context
- **Always** include relevant context (file path, offset, key, etc.)
- **Never** expose internal paths in production errors
- **Use** structured errors over string errors

## Compression Standards

### Algorithms
Priority order (configurable):
1. **LZ4**: Default for real-time (speed > ratio)
2. **Snappy**: Compatible with Google ecosystem
3. **Zstd**: When compression ratio matters
4. **None**: Always supported

### Implementation
```rust
pub enum CompressionType {
    None = 0,
    Lz4 = 1,
    Snappy = 2,
    Zstd = 3,
}

// Standard compression header
struct CompressedData {
    compression_type: u8,
    uncompressed_size: u32,
    compressed_data: Vec<u8>,
}
```

## File Naming Standards

### Data Files
```
<component>_<sequence>.<ext>
Examples:
- wal_000001.log
- sstable_000042.sst
- manifest_000003.mf
```

### Temporary Files
```
.<filename>.tmp.<random>
Example: .sstable_000042.sst.tmp.a3f8d9
```

### Backup Files
```
<filename>.backup.<timestamp>
Example: manifest_000003.mf.backup.20240115_120000
```

## Resource Limits

### Standard Limits
```rust
// File size limits
const MAX_FILE_SIZE: u64 = 2 * 1024 * 1024 * 1024;  // 2GB default
const MAX_KEY_SIZE: usize = 64 * 1024;              // 64KB
const MAX_VALUE_SIZE: usize = 16 * 1024 * 1024;     // 16MB
const MAX_BATCH_SIZE: usize = 100 * 1024 * 1024;    // 100MB

// Memory limits
const MAX_BLOCK_CACHE: usize = 1024 * 1024 * 1024;  // 1GB
const MAX_WRITE_BUFFER: usize = 64 * 1024 * 1024;   // 64MB

// Concurrency limits
const MAX_BACKGROUND_JOBS: usize = 4;
const MAX_OPEN_FILES: usize = 1000;
```

### Limit Enforcement
- **Validate** at API boundaries
- **Return** specific errors when exceeded
- **Make** limits configurable where sensible

## Metrics Standards

### Naming Convention
```
ferrisdb_<component>_<metric>_<unit>

Examples:
ferrisdb_wal_bytes_written_total
ferrisdb_sstable_reads_per_second
ferrisdb_compaction_duration_seconds
```

### Standard Metrics
Every component MUST expose:
- `*_operations_total` - Counter of operations
- `*_errors_total` - Counter of errors by type
- `*_duration_seconds` - Histogram of operation latency
- `*_bytes_total` - Counter of bytes processed

## Configuration Standards

### Format
- **Primary**: TOML for human-edited configs
- **Alternative**: JSON for programmatic configs
- **Environment**: `FERRISDB_` prefix for env vars

### Structure
```toml
[component]
# Required fields explicitly set
required_field = "value"

[component.tuning]
# Optional fields with defaults
optional_field = 100

[component.experimental]
# Feature flags
enable_new_feature = false
```

## API Versioning Standards

### Version Strategy
- **URL path**: `/api/v1/`, `/api/v2/`
- **Header**: `FerrisDB-API-Version: 1`
- **Deprecation**: 2 version support minimum

### Compatibility Rules
1. **Backward compatible**: Add fields, new endpoints
2. **Breaking changes**: New major version only
3. **Deprecation notices**: One version ahead

## Testing Standards (Component-Specific)

### Required Test Types
1. **Corruption tests**: Inject bit flips, truncation
2. **Crash tests**: Kill at various points
3. **Compatibility tests**: Read old format versions
4. **Stress tests**: Max limits, concurrent access
5. **Fuzz tests**: Random inputs for parsers

### Test Data
```rust
// Standard test data generator
pub fn generate_test_data(
    key_prefix: &str,
    count: usize,
    value_size: usize,
) -> Vec<(Key, Value)>;
```

## Logging Standards

### Log Levels
Use standard Rust log levels consistently:
- **ERROR**: Unrecoverable errors requiring intervention
- **WARN**: Recoverable errors or concerning conditions  
- **INFO**: Important state changes or milestones
- **DEBUG**: Detailed operational information
- **TRACE**: Very detailed debugging information

### Log Message Format
```rust
// Include structured context
info!(
    component = "wal",
    file = %path.display(),
    size_bytes = file_size,
    "Created new WAL file"
);

// Errors MUST include context
error!(
    component = "sstable", 
    error = %e,
    offset = corrupt_offset,
    "Checksum validation failed"
);
```

### What to Log
- **Always**: Errors, lifecycle events, configuration
- **Info level**: State transitions, file operations, metrics
- **Debug level**: Operation details, cache hits/misses
- **Never**: Sensitive data, full keys/values, passwords

### Performance Considerations
- Use structured logging (tracing crate)
- Avoid string formatting in hot paths
- Use sampling for high-frequency events

## Documentation Standards

### Component Documentation Must Include
1. **File format specification**: Byte-level layout
2. **Configuration reference**: All options
3. **Metrics reference**: What to monitor
4. **Error reference**: All possible errors
5. **Example usage**: Common patterns
6. **Migration guide**: Version upgrades

## Security Standards

### Principles
1. **No trust**: Validate all inputs
2. **Fail closed**: Deny on any doubt
3. **Least privilege**: Minimal permissions
4. **Defense in depth**: Multiple checks

### Implementation
- **Bounds checking**: On every array access
- **Integer overflow**: Use checked arithmetic
- **Path traversal**: Validate all paths
- **Resource exhaustion**: Enforce limits

## Future-Proofing Standards

### Reserved Space
- **Headers**: Include `reserved` fields
- **Enums**: Reserve high values for future
- **Flags**: Reserve high bits

### Forward Compatibility
```rust
// Reading newer versions
if header.version > MY_VERSION {
    if header.version >> 16 == MY_VERSION >> 16 {
        // Same major version, try to read
        warn!("Reading newer minor version");
    } else {
        // Different major version
        return Err(Error::IncompatibleVersion);
    }
}
```

## Adoption Checklist

When implementing a new component:

- [ ] Follow binary encoding standards
- [ ] Include standard file header
- [ ] Implement checksum validation
- [ ] Use standard timestamp format
- [ ] Follow key encoding rules
- [ ] Define specific error types
- [ ] Support standard compression
- [ ] Follow file naming convention
- [ ] Enforce resource limits
- [ ] Export standard metrics
- [ ] Use standard configuration
- [ ] Version your APIs
- [ ] Write all required tests
- [ ] Document format and usage

## References

- [Protocol Buffers Encoding](https://developers.google.com/protocol-buffers/docs/encoding) - Variable-length integers
- [LevelDB Format](https://github.com/google/leveldb/blob/main/doc/table_format.md) - File structure patterns
- [RocksDB Format](https://github.com/facebook/rocksdb/wiki/Rocksdb-BlockBasedTable-Format) - Advanced features
- [Apache Parquet](https://parquet.apache.org/docs/file-format/) - Columnar format standards
- [FoundationDB Design](https://apple.github.io/foundationdb/design.html) - Distributed database architecture

## Questions?

For clarification on standards or proposing changes, open an issue with the `standards` label.