//! Common traits and utilities for file formats in FerrisDB
//!
//! This module defines the traits that all file formats (WAL, SSTable, Manifest)
//! should implement to ensure consistent behavior across the storage engine.

use ferrisdb_core::{Error, Result};
use std::path::Path;

/// Core trait for all file formats with headers
pub trait FileFormat: Sized {
    /// Magic bytes identifying this file type
    const MAGIC: &'static [u8; 8];

    /// Human-readable name for error messages
    const FORMAT_NAME: &'static str;

    /// Current version of this format
    const CURRENT_VERSION: u16;

    /// Minimum supported version for reading
    const MIN_SUPPORTED_VERSION: u16;
}

/// Header operations for file formats
pub trait FileHeader: FileFormat {
    /// Size of the header in bytes
    const HEADER_SIZE: usize;

    /// Encode header to bytes
    fn encode(&self) -> Vec<u8>;

    /// Decode header from bytes
    fn decode(data: &[u8]) -> Result<Self>;

    /// Validate header integrity and version
    fn validate(&self) -> Result<()>;

    /// Get the magic bytes from this header
    fn magic(&self) -> &[u8; 8];

    /// Get version number
    fn version(&self) -> u16;

    /// Check if version is supported
    fn is_version_supported(&self) -> bool {
        let major = self.version() >> 8;
        let min_major = Self::MIN_SUPPORTED_VERSION >> 8;
        let current_major = Self::CURRENT_VERSION >> 8;

        major >= min_major && major <= current_major
    }
}

/// File validation operations
pub trait ValidateFile: FileHeader {
    /// Quickly validate file header without reading entire file
    fn validate_file_header(path: &Path) -> Result<()> {
        use std::fs::File;
        use std::io::Read;

        let mut file = File::open(path)?;
        let mut header_bytes = vec![0u8; Self::HEADER_SIZE];
        file.read_exact(&mut header_bytes)?;

        let header = Self::decode(&header_bytes)?;
        header.validate()?;

        Ok(())
    }

    /// Get file type from path (for error messages)
    fn identify_file(path: &Path) -> Result<String> {
        use std::fs::File;
        use std::io::Read;

        let mut file = File::open(path)?;
        let mut magic = [0u8; 8];
        file.read_exact(&mut magic)?;

        if &magic == Self::MAGIC {
            Ok(Self::FORMAT_NAME.to_string())
        } else {
            Err(Error::Corruption(format!(
                "Not a {} file (wrong magic bytes)",
                Self::FORMAT_NAME
            )))
        }
    }
}

/// Checksummed headers
pub trait ChecksummedHeader: FileHeader {
    /// Calculate checksum of header (excluding checksum field)
    fn calculate_checksum(&self) -> u32;

    /// Get stored checksum
    fn stored_checksum(&self) -> u32;

    /// Verify checksum matches
    fn verify_checksum(&self) -> Result<()> {
        let calculated = self.calculate_checksum();
        let stored = self.stored_checksum();

        if calculated != stored {
            Err(Error::Corruption(format!(
                "{} header checksum mismatch: expected {:#x}, got {:#x}",
                Self::FORMAT_NAME,
                stored,
                calculated
            )))
        } else {
            Ok(())
        }
    }
}

/// File creation metadata
pub trait FileMetadata {
    /// When file was created (unix timestamp microseconds)
    fn created_at(&self) -> u64;

    /// Unique identifier for this file
    fn file_id(&self) -> u64;

    /// Human-readable creation time
    fn created_at_string(&self) -> String {
        use std::time::{Duration, UNIX_EPOCH};
        let duration = Duration::from_micros(self.created_at());
        let _datetime = UNIX_EPOCH + duration;

        // Format as ISO 8601
        let secs = duration.as_secs();
        let micros = duration.subsec_micros();
        format!("{}.{:06}Z", secs, micros)
    }
}

/// Trait for files that can be compacted/merged
pub trait Compactable: FileFormat {
    /// Check if file should be compacted
    fn needs_compaction(&self) -> bool;

    /// Size threshold for compaction
    fn compaction_threshold() -> u64;
}

/// Trait for files with entries/records
pub trait EntryBasedFile: FileFormat {
    type Entry;

    /// Get number of entries in file
    fn entry_count(&self) -> u64;

    /// Estimate of average entry size
    fn avg_entry_size(&self) -> Option<u64>;
}

/// Trait for files that support key ranges
pub trait KeyRangeFile: FileFormat {
    /// Get minimum key in file
    fn min_key(&self) -> Option<&[u8]>;

    /// Get maximum key in file
    fn max_key(&self) -> Option<&[u8]>;

    /// Check if key might be in this file
    fn might_contain_key(&self, key: &[u8]) -> bool {
        match (self.min_key(), self.max_key()) {
            (Some(min), Some(max)) => key >= min && key <= max,
            _ => true, // Conservative: might contain
        }
    }
}
