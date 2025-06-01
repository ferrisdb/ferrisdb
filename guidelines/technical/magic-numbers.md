# FerrisDB Magic Number Standard

This document defines the standard for magic numbers (file signatures) used across FerrisDB file formats.

**Purpose**: Ensure reliable file type identification and prevent accidental processing of wrong file types  
**Prerequisites**: Understanding of file formats and binary data  
**Related**: [Storage Engine](storage-engine.md), [Architecture](architecture.md)

## Why Magic Numbers?

Magic numbers serve critical purposes in database systems:

1. **File Type Identification**: Instantly identify if a file belongs to FerrisDB
2. **Component Differentiation**: Distinguish between WAL, SSTable, and other file types
3. **Corruption Detection**: Early detection if file header is corrupted
4. **Version Compatibility**: Prevent processing incompatible file formats
5. **Operational Safety**: Avoid accidentally processing wrong files

## FerrisDB Magic Number Convention

### Standard Format

All new FerrisDB components should follow this pattern:

```
FDB<component>_
```

- **Prefix**: "FDB" (FerrisDB identifier)
- **Component**: 3-5 character component identifier
- **Padding**: Underscores to reach exactly 8 bytes
- **Encoding**: ASCII characters stored as 64-bit little-endian integer

### Examples

```rust
// Standard format - all components use FDB prefix
SSTABLE_MAGIC    = 0x4644425353545F5F  // "FDBSST__"
WAL_MAGIC        = 0x4644425741_4C5F5F  // "FDBWAL__"
MANIFEST_MAGIC   = 0x464442_4D414E4946  // "FDBMANIF"
CHECKPOINT_MAGIC = 0x464442_434B50545F  // "FDBCKPT_"
```

## Endianness Decision

FerrisDB uses **little-endian** byte order throughout all file formats, including magic numbers. This decision is based on:

1. **Consistency**: All other fields in FerrisDB use little-endian
2. **Performance**: No byte swapping on x86/x64 architectures 
3. **Simplicity**: One encoding rule for everything
4. **Industry precedent**: LevelDB/RocksDB follow this pattern

While this means magic numbers won't appear as readable ASCII in raw hex dumps, it maintains consistency across the entire codebase.

## Implementation Guidelines

### 1. File Header Structure

Every FerrisDB file should start with:

```rust
struct FileHeader {
    magic: u64,      // 8 bytes: Magic number
    version: u32,    // 4 bytes: Format version
    flags: u32,      // 4 bytes: Feature flags
    // Component-specific fields...
}
```

### 2. Magic Number Validation

Always validate magic numbers when opening files:

```rust
use ferrisdb_core::{validate_magic, WAL_MAGIC};

fn open_wal_file(path: &Path) -> Result<File> {
    let mut file = File::open(path)?;
    let mut header = [0u8; 8];
    file.read_exact(&mut header)?;
    
    validate_magic(&header, WAL_MAGIC)?;
    // Continue processing...
}
```

### 3. Error Messages

Include human-readable format in error messages:

```rust
// Good error message:
"Invalid magic number: expected 0x4644425741_4C5F5F (FDBWAL__), got 0x12345678 (...)"

// Not just:
"Invalid magic number: 0x12345678"
```

## Component-Specific Magic Numbers

### WAL (Write-Ahead Log)
- **Magic**: `FDBWAL__` (0x4644425741_4C5F5F)
- **Usage**: WAL segment files
- **Location**: First 8 bytes of file

### SSTable
- **Magic**: `FDBSST__` (0x4644425353545F5F)
- **Usage**: Sorted String Table files
- **Location**: Last 8 bytes of footer (offset: filesize - 8)
- **Note**: Follows standard FDB prefix convention

### Manifest
- **Magic**: `FDBMANIF` (0x464442_4D414E4946)
- **Usage**: Database manifest tracking SSTable levels
- **Location**: First 8 bytes of file

### Future Components

Reserved magic numbers for future use:

```rust
CHECKPOINT_MAGIC = "FDBCKPT_"  // Checkpoints
CONFIG_MAGIC     = "FDBCONF_"  // Binary configs
BACKUP_MAGIC     = "FDBBKUP_"  // Backup files
JOURNAL_MAGIC    = "FDBJRNL_"  // Transaction journals
```

## Best Practices

### DO:
- ✅ Use the standard FDB prefix for new components
- ✅ Validate magic numbers before processing
- ✅ Include version numbers separate from magic
- ✅ Document magic numbers in file format specs
- ✅ Use little-endian encoding (consistent with all FerrisDB formats)
- ✅ Make magic numbers exactly 8 bytes

### DON'T:
- ❌ Create magic numbers shorter than 8 bytes
- ❌ Use random hex values without ASCII meaning
- ❌ Put version info in the magic number itself
- ❌ Skip validation "for performance"
- ❌ Reuse magic numbers across components

## Migration Strategy

### Adding Magic Numbers to Existing Formats

For components without magic numbers (like current WAL):

1. **Detection Phase**: Check if file starts with magic
2. **Compatibility Mode**: If no magic, assume v0 format
3. **Migration**: Add magic on next file rotation
4. **Deprecation**: Eventually require magic numbers

```rust
fn open_with_migration(path: &Path) -> Result<Reader> {
    let mut file = File::open(path)?;
    let mut probe = [0u8; 8];
    
    if file.read_exact(&mut probe).is_ok() {
        file.seek(SeekFrom::Start(0))?;
        
        if is_valid_magic(&probe) {
            // New format with magic
            return open_v1(file);
        }
    }
    
    // Legacy format without magic
    file.seek(SeekFrom::Start(0))?;
    open_v0(file)
}
```

## Testing Requirements

### Unit Tests
```rust
#[test]
fn magic_number_is_correct_length() {
    assert_eq!(magic_to_ascii(WAL_MAGIC).len(), 8);
}

#[test]
fn magic_number_is_unique() {
    // Ensure no duplicates across components
}

#[test]
fn magic_validation_detects_wrong_type() {
    // Test rejection of wrong file types
}
```

### Integration Tests
- Open SSTable with WAL reader → Should fail with clear error
- Corrupt magic number → Should fail validation
- Missing magic (v0 format) → Should handle gracefully

## Security Considerations

1. **No Sensitive Data**: Magic numbers are not secret
2. **Tamper Detection**: Not cryptographic protection
3. **File Disclosure**: Magic reveals file type to attackers

## References

- [SQLite File Format](https://www.sqlite.org/fileformat.html) - Application ID concept
- [List of file signatures](https://en.wikipedia.org/wiki/List_of_file_signatures) - Common magic numbers
- [PostgreSQL WAL Format](https://www.postgresql.org/docs/current/wal-internals.html) - XLOG_PAGE_MAGIC usage

## Implementation Status

| Component | Magic Number | Status | Version |
|-----------|-------------|---------|----------|
| SSTable | FDBSST__ | ✅ Implemented | v1 |
| WAL | FDBWAL__ | 📋 Planned | v1 |
| Manifest | FDBMANIF | 🔮 Future | - |
| Checkpoint | FDBCKPT_ | 🔮 Future | - |

## Questions?

For questions about magic numbers or file formats, consult the storage team or open an issue.