# File Format Standards

Standardized file format design patterns for FerrisDB persistence layers.

**NOTE**: This document serves as the authoritative source of truth for all FerrisDB file format decisions. Any file format implementation must align with the standards documented here. If improvements are discovered, update this document first before implementing changes.

**Prerequisites**: Understanding of binary file formats, checksums, and endianness  
**Related**: [Storage Engine](storage-engine.md), [System Invariants](invariants.md), [Architecture](architecture.md)

## Core Principles

### 1. Self-Describing Formats

Every file must be self-contained and identifiable:

- Magic numbers for file type identification
- Version information for format evolution
- Metadata for validation and recovery

### 2. Integrity First

All persistent data must be verifiable:

- CRC32 checksums for all critical data
- Length prefixes for variable data
- Explicit record boundaries

### 3. Debuggability

Formats must be inspectable and understandable:

- Clear ASCII art documentation
- Offset tables for navigation
- Human-readable magic numbers

## Magic Number Pattern

All FerrisDB files MUST begin with an 8-byte magic number following this pattern:

```
FDB_XXX\0
```

Where:

- `FDB_` - Fixed prefix identifying FerrisDB files
- `XXX` - Three-character file type identifier
- `\0` - Null terminator (0x00)

### Standard Magic Numbers

```rust
// Core file types
const MAGIC_WAL: &[u8; 8] = b"FDB_WAL\0";    // Write-Ahead Log
const MAGIC_SST: &[u8; 8] = b"FDB_SST\0";    // Sorted String Table
const MAGIC_MAN: &[u8; 8] = b"FDB_MAN\0";    // Manifest file
const MAGIC_SNP: &[u8; 8] = b"FDB_SNP\0";    // Snapshot file

// Future file types
const MAGIC_IDX: &[u8; 8] = b"FDB_IDX\0";    // Index file
const MAGIC_BLM: &[u8; 8] = b"FDB_BLM\0";    // Bloom filter
const MAGIC_CMP: &[u8; 8] = b"FDB_CMP\0";    // Compaction state
```

### Magic Number Validation

```rust
fn validate_magic(data: &[u8], expected: &[u8; 8]) -> Result<(), Error> {
    if data.len() < 8 {
        return Err(Error::InvalidFormat("File too small for magic number"));
    }

    if &data[0..8] != expected {
        return Err(Error::InvalidFormat("Invalid magic number"));
    }

    Ok(())
}
```

## Header Design

All FerrisDB files use a standardized 64-byte header:

```
┌────────────────────────────────────────────────────────────────┐
│ Offset │ Size │ Field         │ Description                    │
├────────┼──────┼───────────────┼────────────────────────────────┤
│ 0      │ 8    │ magic         │ Magic number (FDB_XXX\0)       │
│ 8      │ 2    │ version_major │ Major version (breaking)       │
│ 10     │ 2    │ version_minor │ Minor version (compatible)     │
│ 12     │ 4    │ header_crc    │ CRC32 of bytes 0-11           │
│ 16     │ 8    │ created_at    │ Unix timestamp (microseconds)  │
│ 24     │ 8    │ file_size     │ Total file size in bytes       │
│ 32     │ 8    │ data_offset   │ Offset to first data record    │
│ 40     │ 8    │ data_size     │ Total size of data section     │
│ 48     │ 4    │ record_count  │ Number of records in file      │
│ 52     │ 4    │ flags         │ File-specific flags            │
│ 56     │ 4    │ reserved_1    │ Reserved for future use        │
│ 60     │ 4    │ data_crc      │ CRC32 of entire data section   │
└────────────────────────────────────────────────────────────────┘
```

### Header Implementation

```rust
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FileHeader {
    pub magic: [u8; 8],
    pub version_major: u16,
    pub version_minor: u16,
    pub header_crc: u32,
    pub created_at: u64,
    pub file_size: u64,
    pub data_offset: u64,
    pub data_size: u64,
    pub record_count: u32,
    pub flags: u32,
    pub reserved_1: u32,
    pub data_crc: u32,
}

impl FileHeader {
    pub const SIZE: usize = 64;

    pub fn validate(&self) -> Result<(), Error> {
        // Verify header CRC
        let computed_crc = self.compute_header_crc();
        if computed_crc != self.header_crc {
            return Err(Error::ChecksumMismatch);
        }

        // Validate offsets
        if self.data_offset < Self::SIZE as u64 {
            return Err(Error::InvalidFormat("Data offset inside header"));
        }

        Ok(())
    }
}
```

### Version Format

Version numbers follow semantic versioning principles:

- **Major version**: Incremented for breaking format changes
- **Minor version**: Incremented for backward-compatible additions

```rust
pub fn is_compatible(file_version: (u16, u16), supported: (u16, u16)) -> bool {
    // Major version must match exactly
    if file_version.0 != supported.0 {
        return false;
    }

    // Minor version can be newer (backward compatible)
    file_version.1 <= supported.1
}
```

## Binary Encoding Standards

### Endianness

All multi-byte values MUST use **little-endian** encoding:

```rust
// Writing
writer.write_u32::<LittleEndian>(value)?;

// Reading
let value = reader.read_u32::<LittleEndian>()?;
```

### Variable-Length Data

All variable-length data MUST be prefixed with its length:

```
┌─────────────┬──────────────────┐
│ Length (4B) │ Data (N bytes)   │
└─────────────┴──────────────────┘
```

```rust
pub fn write_bytes(writer: &mut impl Write, data: &[u8]) -> io::Result<()> {
    writer.write_u32::<LittleEndian>(data.len() as u32)?;
    writer.write_all(data)?;
    Ok(())
}

pub fn read_bytes(reader: &mut impl Read) -> io::Result<Vec<u8>> {
    let len = reader.read_u32::<LittleEndian>()? as usize;
    let mut data = vec![0u8; len];
    reader.read_exact(&mut data)?;
    Ok(data)
}
```

### Self-Contained Records

Every record must be independently parseable:

```
┌────────────────────────────────────────────────────────┐
│ Record Header (16 bytes)                               │
├────────────┬────────────┬────────────┬────────────────┤
│ Type (2B)  │ Flags (2B) │ Length (4B)│ CRC32 (4B)     │
│ Timestamp  │            │            │ Reserved (4B)   │
│ (8B)       │            │            │                 │
├────────────┴────────────┴────────────┴────────────────┤
│ Record Data (Length bytes)                             │
└────────────────────────────────────────────────────────┘
```

## Documentation Requirements

### ASCII Diagrams

Every file format MUST include ASCII art diagrams showing:

1. **Overall file structure**
2. **Header layout with offsets**
3. **Record format details**
4. **Example with actual byte values**

Example:

```
WAL File Structure:
┌─────────────────────┐ 0x000
│   File Header (64B) │
├─────────────────────┤ 0x040
│   Record 1          │
├─────────────────────┤ 0x0A0
│   Record 2          │
├─────────────────────┤ 0x120
│   ...               │
└─────────────────────┘ EOF
```

### Offset Tables

Include detailed offset tables for navigation:

```
SSTable Index Block:
┌────────┬──────┬─────────────────┬──────────────────────┐
│ Offset │ Size │ Type            │ Description          │
├────────┼──────┼─────────────────┼──────────────────────┤
│ 0      │ 4    │ u32            │ Number of entries    │
│ 4      │ 4    │ u32            │ Block size           │
│ 8      │ N    │ IndexEntry[]   │ Index entries        │
│ 8+N    │ 4    │ u32            │ Index CRC32          │
└────────┴──────┴─────────────────┴──────────────────────┘
```

### Design Rationale

Document WHY each decision was made:

```markdown
## Design Decisions

### Why 64-byte headers?

- Fits in a single cache line on modern CPUs
- Provides room for future expansion
- Aligns well with typical disk sector sizes

### Why CRC32 instead of SHA256?

- Sufficient for detecting corruption (not security)
- Fast to compute
- Fixed 4-byte size
- Industry standard for database files

### Why little-endian?

- Most modern CPUs are little-endian
- Consistent with Rust's default byte order
- Matches RocksDB and LevelDB conventions
```

## Implementation Patterns

### File Writer Pattern

```rust
pub struct FileWriter<W: Write> {
    writer: BufWriter<W>,
    header: FileHeader,
    records_written: u32,
    data_crc: crc32::Digest,
}

impl<W: Write> FileWriter<W> {
    pub fn new(writer: W, file_type: &[u8; 8]) -> Result<Self, Error> {
        let mut header = FileHeader::default();
        header.magic.copy_from_slice(file_type);
        header.version_major = CURRENT_VERSION.0;
        header.version_minor = CURRENT_VERSION.1;
        header.created_at = timestamp_micros();
        header.data_offset = FileHeader::SIZE as u64;

        let mut file_writer = Self {
            writer: BufWriter::new(writer),
            header,
            records_written: 0,
            data_crc: crc32::Digest::new(),
        };

        // Reserve space for header
        file_writer.write_header_placeholder()?;

        Ok(file_writer)
    }

    pub fn write_record(&mut self, record: &[u8]) -> Result<(), Error> {
        // Update CRC
        self.data_crc.update(record);

        // Write record
        self.writer.write_all(record)?;
        self.records_written += 1;

        Ok(())
    }

    pub fn finish(mut self) -> Result<(), Error> {
        // Update header with final values
        self.header.record_count = self.records_written;
        self.header.data_crc = self.data_crc.finish();
        self.header.file_size = self.writer.stream_position()?;
        self.header.data_size = self.header.file_size - self.header.data_offset;

        // Rewind and write actual header
        self.writer.seek(SeekFrom::Start(0))?;
        self.write_header()?;

        self.writer.flush()?;
        Ok(())
    }
}
```

### File Reader Pattern

```rust
pub struct FileReader<R: Read + Seek> {
    reader: BufReader<R>,
    header: FileHeader,
}

impl<R: Read + Seek> FileReader<R> {
    pub fn new(mut reader: R, expected_magic: &[u8; 8]) -> Result<Self, Error> {
        // Read and validate header
        let header = Self::read_header(&mut reader)?;
        validate_magic(&header.magic, expected_magic)?;
        header.validate()?;

        Ok(Self {
            reader: BufReader::new(reader),
            header,
        })
    }

    pub fn records(&mut self) -> RecordIterator<R> {
        RecordIterator::new(self)
    }
}
```

## Testing Requirements

### Format Validation Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magic_number_validation() {
        let valid = b"FDB_WAL\0";
        assert!(validate_magic(valid, &MAGIC_WAL).is_ok());

        let invalid = b"INVALID\0";
        assert!(validate_magic(invalid, &MAGIC_WAL).is_err());
    }

    #[test]
    fn test_round_trip() {
        let mut buffer = Vec::new();

        // Write
        let writer = FileWriter::new(&mut buffer, &MAGIC_SST).unwrap();
        writer.write_record(b"test data").unwrap();
        writer.finish().unwrap();

        // Read
        let reader = FileReader::new(&buffer[..], &MAGIC_SST).unwrap();
        let records: Vec<_> = reader.records().collect();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0], b"test data");
    }
}
```

### Corruption Detection Tests

```rust
#[test]
fn test_corruption_detection() {
    let mut data = create_valid_file();

    // Corrupt a byte in the data section
    data[100] ^= 0xFF;

    let result = FileReader::new(&data[..], &MAGIC_WAL);
    assert!(matches!(result, Err(Error::ChecksumMismatch)));
}
```

## Migration Strategy

When file formats need to change:

1. **Minor version bump**: Add new fields using reserved space
2. **Major version bump**: Create new format, implement migration tool
3. **Document migration path**: Provide clear upgrade instructions

```rust
pub fn migrate_v1_to_v2(input_path: &Path, output_path: &Path) -> Result<(), Error> {
    let v1_reader = V1FileReader::new(File::open(input_path)?)?;
    let v2_writer = V2FileWriter::new(File::create(output_path)?)?;

    for record in v1_reader.records() {
        let v2_record = convert_record_v1_to_v2(record?)?;
        v2_writer.write_record(&v2_record)?;
    }

    v2_writer.finish()?;
    Ok(())
}
```

## References

- [LevelDB File Formats](https://github.com/google/leveldb/blob/main/doc/table_format.md)
- [RocksDB File Formats](https://github.com/facebook/rocksdb/wiki/Rocksdb-BlockBasedTable-Format)
- [Protocol Buffers Encoding](https://developers.google.com/protocol-buffers/docs/encoding)
- [Apache Parquet Format](https://parquet.apache.org/docs/file-format/)
