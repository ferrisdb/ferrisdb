//! WAL file header implementation
//!
//! The WAL header is a 64-byte structure that appears at the beginning of every
//! WAL file. It provides file identification, versioning, and integrity checking.

use crate::format::{ChecksummedHeader, FileFormat, FileHeader, FileMetadata, ValidateFile};
use ferrisdb_core::{Error, Result};

use crc32fast::Hasher;

use std::time::{SystemTime, UNIX_EPOCH};

/// Magic number identifying WAL files
/// Format: "FDB_WAL\0" (7 chars + null terminator)
pub const WAL_MAGIC: &[u8; 8] = b"FDB_WAL\0";

/// Current WAL format version (1.0)
pub const WAL_CURRENT_VERSION: u16 = 0x0100;

/// Size of WAL header in bytes
pub const WAL_HEADER_SIZE: usize = 64;

/// WAL file header
///
/// The header is exactly 64 bytes (one cache line) and contains:
/// - File identification (magic number, version)
/// - Integrity check (CRC32 checksum)
/// - Metadata (creation time, file sequence)
/// - Reserved space for future extensions
///
/// ## Binary Layout
///
/// ```text
/// struct WALHeader {
///     magic: [u8; 8],           // offset 0:  "FDB_WAL\0"
///     version: u16,             // offset 8:  0x0100 (v1.0)
///     flags: u16,               // offset 10: 0x0000 (reserved)
///     header_size: u32,         // offset 12: 64
///     header_checksum: u32,     // offset 16: CRC32 of bytes 0-15,20-63
///     entry_start_offset: u32,  // offset 20: 64
///     created_at: u64,          // offset 24: microseconds since epoch
///     file_sequence: u64,       // offset 32: unique file ID
///     reserved: [u8; 24],       // offset 40: zeros (future use)
/// }  // Total: 64 bytes
/// ```
///
/// ## Version Scheme
///
/// Version is stored as a 16-bit value: `0xMMmm` where:
/// - `MM` = major version (incompatible changes)
/// - `mm` = minor version (compatible changes)
///
/// Current version: 0x0100 (v1.0)
///
/// ## Checksum Calculation
///
/// The header checksum is a CRC32 calculated over all header fields
/// EXCEPT the checksum field itself (bytes 16-19). This allows
/// detection of header corruption.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WALHeader {
    /// Magic bytes identifying this as a WAL file
    pub magic: [u8; 8],
    /// Version number (major.minor in high.low bytes)
    pub version: u16,
    /// Feature flags (currently unused, must be 0)
    pub flags: u16,
    /// Total size of header (64 for v1.0)
    pub header_size: u32,
    /// CRC32 checksum of header (excluding this field)
    pub header_checksum: u32,
    /// Offset where entries begin (64 for v1.0)
    pub entry_start_offset: u32,
    /// Creation timestamp in microseconds since Unix epoch
    pub created_at: u64,
    /// Unique sequence number for this file
    pub file_sequence: u64,
    /// Reserved for future use (must be zero)
    pub reserved: [u8; 24],
}

impl WALHeader {
    /// Create a new WAL header with the given file sequence
    pub fn new(file_sequence: u64) -> Self {
        let mut header = Self {
            magic: *WAL_MAGIC,
            version: WAL_CURRENT_VERSION,
            flags: 0,
            header_size: WAL_HEADER_SIZE as u32,
            header_checksum: 0,
            entry_start_offset: WAL_HEADER_SIZE as u32,
            created_at: current_timestamp_micros(),
            file_sequence,
            reserved: [0; 24],
        };

        // Calculate and set checksum
        header.header_checksum = header.calculate_checksum();
        header
    }
}

impl FileFormat for WALHeader {
    const MAGIC: &'static [u8; 8] = WAL_MAGIC;
    const FORMAT_NAME: &'static str = "WAL";
    const CURRENT_VERSION: u16 = WAL_CURRENT_VERSION;
    const MIN_SUPPORTED_VERSION: u16 = 0x0100; // v1.0
}

impl FileHeader for WALHeader {
    const HEADER_SIZE: usize = WAL_HEADER_SIZE;

    fn encode(&self) -> Vec<u8> {
        let mut buf = vec![0u8; Self::HEADER_SIZE];

        // Encode fields in order
        buf[0..8].copy_from_slice(&self.magic);
        buf[8..10].copy_from_slice(&self.version.to_le_bytes());
        buf[10..12].copy_from_slice(&self.flags.to_le_bytes());
        buf[12..16].copy_from_slice(&self.header_size.to_le_bytes());
        buf[16..20].copy_from_slice(&self.header_checksum.to_le_bytes());
        buf[20..24].copy_from_slice(&self.entry_start_offset.to_le_bytes());
        buf[24..32].copy_from_slice(&self.created_at.to_le_bytes());
        buf[32..40].copy_from_slice(&self.file_sequence.to_le_bytes());
        buf[40..64].copy_from_slice(&self.reserved);

        buf
    }

    fn decode(data: &[u8]) -> Result<Self> {
        if data.len() < Self::HEADER_SIZE {
            return Err(Error::Corruption(format!(
                "WAL header too small: {} bytes (expected {})",
                data.len(),
                Self::HEADER_SIZE
            )));
        }

        let mut magic = [0u8; 8];
        magic.copy_from_slice(&data[0..8]);

        let version = u16::from_le_bytes([data[8], data[9]]);
        let flags = u16::from_le_bytes([data[10], data[11]]);
        let header_size = u32::from_le_bytes([data[12], data[13], data[14], data[15]]);
        let header_checksum = u32::from_le_bytes([data[16], data[17], data[18], data[19]]);
        let entry_start_offset = u32::from_le_bytes([data[20], data[21], data[22], data[23]]);
        let created_at = u64::from_le_bytes([
            data[24], data[25], data[26], data[27], data[28], data[29], data[30], data[31],
        ]);
        let file_sequence = u64::from_le_bytes([
            data[32], data[33], data[34], data[35], data[36], data[37], data[38], data[39],
        ]);

        let mut reserved = [0u8; 24];
        reserved.copy_from_slice(&data[40..64]);

        let header = Self {
            magic,
            version,
            flags,
            header_size,
            header_checksum,
            entry_start_offset,
            created_at,
            file_sequence,
            reserved,
        };

        // Validate immediately after decoding
        header.validate()?;

        Ok(header)
    }

    fn validate(&self) -> Result<()> {
        // Check magic number
        if &self.magic != Self::MAGIC {
            return Err(Error::Corruption(format!(
                "Invalid WAL magic: expected {:?}, found {:?}",
                Self::MAGIC,
                self.magic
            )));
        }

        // Check version compatibility
        if !self.is_version_supported() {
            return Err(Error::Corruption(format!(
                "Unsupported WAL version: {}.{} (supported: {}.x)",
                self.version >> 8,
                self.version & 0xFF,
                Self::CURRENT_VERSION >> 8
            )));
        }

        // Check header size
        if self.header_size != Self::HEADER_SIZE as u32 {
            return Err(Error::Corruption(format!(
                "Invalid WAL header size: {} (expected {})",
                self.header_size,
                Self::HEADER_SIZE
            )));
        }

        // Check flags (must be 0 for v1.0)
        if self.flags != 0 {
            return Err(Error::Corruption(format!(
                "Invalid WAL flags: {:#x} (must be 0)",
                self.flags
            )));
        }

        // Verify checksum
        self.verify_checksum()?;

        Ok(())
    }

    fn magic(&self) -> &[u8; 8] {
        &self.magic
    }

    fn version(&self) -> u16 {
        self.version
    }
}

impl ValidateFile for WALHeader {}

impl ChecksummedHeader for WALHeader {
    fn calculate_checksum(&self) -> u32 {
        let mut hasher = Hasher::new();

        // Hash all fields except the checksum itself
        hasher.update(&self.magic);
        hasher.update(&self.version.to_le_bytes());
        hasher.update(&self.flags.to_le_bytes());
        hasher.update(&self.header_size.to_le_bytes());
        // Skip header_checksum field
        hasher.update(&self.entry_start_offset.to_le_bytes());
        hasher.update(&self.created_at.to_le_bytes());
        hasher.update(&self.file_sequence.to_le_bytes());
        hasher.update(&self.reserved);

        hasher.finalize()
    }

    fn stored_checksum(&self) -> u32 {
        self.header_checksum
    }
}

impl FileMetadata for WALHeader {
    fn created_at(&self) -> u64 {
        self.created_at
    }

    fn file_id(&self) -> u64 {
        self.file_sequence
    }
}

/// Get current timestamp in microseconds since Unix epoch
fn current_timestamp_micros() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| {
            // Fallback for systems where time is before Unix epoch
            // Use a monotonic counter starting from process start
            std::time::Duration::from_secs(0)
        })
        .as_micros() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_preserves_all_header_fields() {
        let header = WALHeader::new(12345);
        let encoded = header.encode();
        let decoded = WALHeader::decode(&encoded).unwrap();

        assert_eq!(header, decoded);
    }

    #[test]
    fn validate_returns_error_for_incorrect_magic() {
        let mut header = WALHeader::new(12345);
        header.magic = *b"BADMAGIC";

        assert!(header.validate().is_err());
    }

    #[test]
    fn decode_returns_error_when_checksum_corrupted() {
        let header = WALHeader::new(12345);
        let encoded = header.encode();

        // Corrupt some data
        let mut corrupted = encoded;
        corrupted[25] ^= 0xFF;

        let result = WALHeader::decode(&corrupted);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::Corruption(msg) if msg.contains("checksum")));
    }

    #[test]
    fn validate_returns_error_for_unsupported_version() {
        let mut header = WALHeader::new(12345);
        header.version = 0x0200; // v2.0 - not supported

        let result = header.validate();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::Corruption(msg) if msg.contains("version")));
    }

    #[test]
    fn header_size_equals_64_bytes_cache_line() {
        assert_eq!(WAL_HEADER_SIZE, 64);
        assert_eq!(std::mem::size_of::<WALHeader>(), 64);
    }

    #[test]
    fn new_sets_file_sequence_and_current_timestamp() {
        let header = WALHeader::new(98765);

        assert_eq!(header.file_id(), 98765);
        assert!(header.created_at() > 0);

        // Check timestamp is reasonable (within last hour)
        let now = current_timestamp_micros();
        assert!(header.created_at() <= now);
        assert!(header.created_at() > now - 3600 * 1_000_000);
    }
}
