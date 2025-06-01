# FerrisDB Standards Quick Reference

A one-page reference for the most commonly needed standards.

## 🔢 Binary Encoding

```rust
// ALWAYS little-endian
value.to_le_bytes()
u32::from_le_bytes(...)

// Standard string format
[length: u32][utf8_bytes: length]

// NO null terminators, NO padding
```

## 📁 File Headers

```rust
struct StandardFileHeader {
    magic: u64,        // e.g., 0x4644425353545F5F
    version: u32,      // (major << 16) | minor
    flags: u32,        // Feature flags
    header_size: u32,  // For forward compatibility
    reserved: u32,     // Must be 0
}
```

## 🔑 Keys & Timestamps

```rust
// Timestamps: u64 nanoseconds UTC
pub type Timestamp = u64;

// Internal key encoding
[user_key][timestamp:8][sequence:8][type:1]

// Max sizes
MAX_KEY_SIZE = 64KB
MAX_VALUE_SIZE = 16MB
```

## ✓ Checksums

```rust
// Default: CRC32C (hardware accelerated)
use crc32fast::Hasher;

// Every 4KB-64KB block
struct ChecksummedBlock {
    data: Vec<u8>,
    checksum: u32,
}
```

## 🗜️ Compression

```rust
pub enum CompressionType {
    None = 0,
    Lz4 = 1,      // Default (fast)
    Snappy = 2,   // Google compat
    Zstd = 3,     // Best ratio
}
```

## 📏 Resource Limits

```rust
MAX_FILE_SIZE     = 2GB
MAX_KEY_SIZE      = 64KB  
MAX_VALUE_SIZE    = 16MB
MAX_BATCH_SIZE    = 100MB
MAX_BLOCK_CACHE   = 1GB
MAX_WRITE_BUFFER  = 64MB
MAX_OPEN_FILES    = 1000
```

## 🏷️ Naming Conventions

```
Files:     component_sequence.ext
Temp:      .filename.tmp.random
Backup:    filename.backup.timestamp
Metrics:   ferrisdb_component_metric_unit
Env vars:  FERRISDB_*
```

## ❌ Error Handling

```rust
#[derive(Error, Debug)]
pub enum ComponentError {
    #[error("Corruption at offset {offset}: {reason}")]
    Corruption { offset: u64, reason: String },
    
    // ALWAYS include context!
}
```

## 🔄 Version Compatibility

```rust
// Same major version = forward compatible
if new_version >> 16 == my_version >> 16 {
    // Can read with warnings
} else {
    // Need migration
}
```

## 📝 Required Documentation

Every component needs:
1. File format spec (byte-level)
2. Configuration reference
3. Metrics reference  
4. Error reference
5. Example usage
6. Migration guide

## 🚫 Common Mistakes

❌ Using big-endian anywhere  
❌ Null-terminated strings  
❌ Trusting input without validation  
❌ Missing checksums on critical data  
❌ Hardcoded limits without constants  
❌ Paths in error messages  
❌ Skipping forward compatibility  

## ✅ Always Remember

✓ Little-endian everywhere  
✓ Length-prefixed strings  
✓ Validate everything  
✓ Checksum critical data  
✓ Define limit constants  
✓ Context in errors  
✓ Reserve fields for future  

---

Full details: [Common Standards](common-standards.md)