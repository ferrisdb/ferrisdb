//! Magic numbers and file signatures for FerrisDB
//!
//! This module defines standard magic numbers used across FerrisDB file formats
//! to enable reliable file type identification and version detection.
//!
//! # Design Principles
//!
//! 1. **Human Readable**: Magic numbers spell out component names in ASCII
//! 2. **8 Bytes**: All magic numbers are 64-bit for consistency and uniqueness
//! 3. **Component Specific**: Each file type has its own magic number
//! 4. **Version Agnostic**: Version info stored separately from magic number
//! 5. **Little-Endian**: Consistent with all other FerrisDB binary formats
//!
//! # Magic Number Format
//!
//! All FerrisDB magic numbers follow this pattern:
//! - Start with "FDB" (FerrisDB identifier)
//! - Follow with component identifier
//! - Pad with underscores to reach 8 bytes
//! - Stored as little-endian u64
//!
//! Example: "FDBSST__" for SSTable files
//!
//! Note: The hex constants are written in source as big-endian ASCII values
//! for readability, but are stored in files using little-endian byte order.

/// SSTable magic number: "FDBSST__"
/// 
/// Used to identify SSTable files and stored in the footer for validation.
pub const SSTABLE_MAGIC: u64 = 0x4644425353545F5F; // "FDBSST__"

/// Write-Ahead Log magic number: "FDBWAL__"
/// 
/// Used to identify WAL segment files and verify they haven't been corrupted
/// or accidentally processed as the wrong file type.
pub const WAL_MAGIC: u64 = 0x4644425741_4C5F5F; // "FDBWAL__"

/// Manifest file magic number: "FDBMANIF"
/// 
/// Used for the MANIFEST file that tracks SSTable levels and metadata.
pub const MANIFEST_MAGIC: u64 = 0x464442_4D414E4946; // "FDBMANIF"

/// Checkpoint file magic number: "FDBCKPT_"
/// 
/// Used for checkpoint files that capture consistent database state.
pub const CHECKPOINT_MAGIC: u64 = 0x464442_434B50545F; // "FDBCKPT_"

/// Configuration file magic number: "FDBCONF_"
/// 
/// Used for binary configuration files (if we move away from text config).
pub const CONFIG_MAGIC: u64 = 0x464442_434F4E465F; // "FDBCONF_"

/// Backup file magic number: "FDBBKUP_"
/// 
/// Used for backup file formats.
pub const BACKUP_MAGIC: u64 = 0x464442_424B55505F; // "FDBBKUP_"

/// Journal file magic number: "FDBJRNL_"
/// 
/// Reserved for future journal/transaction log files.
pub const JOURNAL_MAGIC: u64 = 0x464442_4A524E4C5F; // "FDBJRNL_"

/// Converts a magic number to its ASCII representation for debugging
/// 
/// Note: Uses little-endian to match FerrisDB's consistent use of 
/// little-endian throughout all file formats.
pub fn magic_to_ascii(magic: u64) -> String {
    let bytes = magic.to_le_bytes();
    String::from_utf8_lossy(&bytes).to_string()
}

/// Validates that a file starts with the expected magic number
/// 
/// Note: Uses little-endian to match FerrisDB's consistent use of 
/// little-endian throughout all file formats.
pub fn validate_magic(data: &[u8], expected: u64) -> crate::Result<()> {
    if data.len() < 8 {
        return Err(crate::Error::InvalidFormat(
            "File too small to contain magic number".to_string(),
        ));
    }

    let magic = u64::from_le_bytes(data[0..8].try_into().unwrap());
    if magic != expected {
        return Err(crate::Error::InvalidFormat(format!(
            "Invalid magic number: expected {} ({}), got {} ({})",
            expected,
            magic_to_ascii(expected),
            magic,
            magic_to_ascii(magic),
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magic_to_ascii() {
        assert_eq!(magic_to_ascii(SSTABLE_MAGIC), "FDBSST__");
        assert_eq!(magic_to_ascii(WAL_MAGIC), "FDBWAL__");
        assert_eq!(magic_to_ascii(MANIFEST_MAGIC), "FDBMANIF");
        assert_eq!(magic_to_ascii(CHECKPOINT_MAGIC), "FDBCKPT_");
    }

    #[test]
    fn test_validate_magic_success() {
        let mut data = vec![0u8; 16];
        data[0..8].copy_from_slice(&WAL_MAGIC.to_le_bytes());
        
        assert!(validate_magic(&data, WAL_MAGIC).is_ok());
    }

    #[test]
    fn test_validate_magic_wrong_magic() {
        let mut data = vec![0u8; 16];
        data[0..8].copy_from_slice(&SSTABLE_MAGIC.to_le_bytes());
        
        let result = validate_magic(&data, WAL_MAGIC);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid magic number"));
    }

    #[test]
    fn test_validate_magic_too_small() {
        let data = vec![0u8; 4];
        
        let result = validate_magic(&data, WAL_MAGIC);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("too small"));
    }

    #[test]
    fn test_all_magics_are_8_bytes() {
        // Ensure all magic numbers are exactly 8 bytes
        assert_eq!(magic_to_ascii(SSTABLE_MAGIC).len(), 8);
        assert_eq!(magic_to_ascii(WAL_MAGIC).len(), 8);
        assert_eq!(magic_to_ascii(MANIFEST_MAGIC).len(), 8);
        assert_eq!(magic_to_ascii(CHECKPOINT_MAGIC).len(), 8);
        assert_eq!(magic_to_ascii(CONFIG_MAGIC).len(), 8);
        assert_eq!(magic_to_ascii(BACKUP_MAGIC).len(), 8);
        assert_eq!(magic_to_ascii(JOURNAL_MAGIC).len(), 8);
    }

    #[test]
    fn test_magic_uniqueness() {
        let magics = vec![
            SSTABLE_MAGIC,
            WAL_MAGIC,
            MANIFEST_MAGIC,
            CHECKPOINT_MAGIC,
            CONFIG_MAGIC,
            BACKUP_MAGIC,
            JOURNAL_MAGIC,
        ];

        // Ensure all magic numbers are unique
        let unique_count = magics.iter().collect::<std::collections::HashSet<_>>().len();
        assert_eq!(unique_count, magics.len());
    }
}