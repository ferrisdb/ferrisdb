//! Write-Ahead Log (WAL) implementation
//!
//! The WAL provides durability by persisting all write operations to disk
//! before they are applied to the in-memory data structures.
//!
//! ## File Format Overview
//!
//! A WAL file consists of:
//! 1. A 64-byte header (see [`WALHeader`])
//! 2. Zero or more log entries (see [`WALEntry`])
//!
//! ```text
//! +----------------+
//! |   WAL Header   |  64 bytes - File identification and metadata
//! |    (64 bytes)  |
//! +----------------+
//! |   WAL Entry    |  Variable size - First operation
//! +----------------+
//! |   WAL Entry    |  Variable size - Second operation
//! +----------------+
//! |      ...       |
//! +----------------+
//! ```
//!
//! ## Header Format (64 bytes)
//!
//! The header provides file identification, versioning, and integrity checking:
//!
//! ```text
//! Offset  Size  Field              Description
//! ------  ----  -----              -----------
//! 0       8     magic              Magic bytes: "FDB_WAL\0"
//! 8       2     version            Format version (major.minor)
//! 10      2     flags              Feature flags (must be 0)
//! 12      4     header_size        Size of header (64)
//! 16      4     header_checksum    CRC32 of header (excluding this field)
//! 20      4     entry_start_offset Where entries begin (64)
//! 24      8     created_at         Creation time (µs since Unix epoch)
//! 32      8     file_sequence      Unique file identifier
//! 40      24    reserved           Reserved for future use (zeros)
//! ```
//!
//! ## Entry Format (Variable size)
//!
//! Each entry is self-contained with its own checksum:
//!
//! ```text
//! Offset  Size  Field         Description
//! ------  ----  -----         -----------
//! 0       4     length        Total entry size (including this field)
//! 4       4     checksum      CRC32 of all following fields
//! 8       8     timestamp     Operation timestamp (microseconds)
//! 16      1     operation     1=Put, 2=Delete
//! 17      4     key_len       Key length in bytes
//! 21      4     value_len     Value length in bytes (0 for Delete)
//! 25      var   key           Key data
//! 25+key  var   value         Value data (empty for Delete)
//! ```
//!
//! ## Design Rationale
//!
//! - **64-byte header**: Fits exactly in one CPU cache line
//! - **CRC32 checksums**: Fast corruption detection
//! - **Self-contained entries**: Allows partial recovery
//! - **Version field**: Enables format evolution
//! - **File sequence**: Prevents accidental file mixing
//!
//! ## Recovery and Durability
//!
//! The WAL ensures durability through:
//! - **Append-only writes**: New entries are always appended
//! - **Checksums**: Both header and entries have CRC32 checksums
//! - **Self-contained entries**: Each entry can be validated independently
//! - **Configurable sync modes**: Control fsync behavior for performance vs durability
//!
//! ## File Rotation
//!
//! WAL files have a size limit. When reached, a new file should be created.
//! The file sequence number in the header prevents accidental file mixing.
//!
//! # Examples
//!
//! ## Writing to WAL
//!
//! ```no_run
//! use ferrisdb_storage::wal::{WALWriter, WALEntry};
//! use ferrisdb_core::SyncMode;
//!
//! let writer = WALWriter::new(
//!     "path/to/wal.log",
//!     SyncMode::Normal,  // Flush to OS but don't fsync every write
//!     64 * 1024 * 1024   // 64MB size limit
//! )?;
//!
//! // Write a Put operation
//! let put_entry = WALEntry::new_put(
//!     b"user:123".to_vec(),
//!     b"{\"name\": \"Alice\"}".to_vec(),
//!     12345  // timestamp in microseconds
//! )?;
//! writer.append(&put_entry)?;
//!
//! // Write a Delete operation
//! let delete_entry = WALEntry::new_delete(
//!     b"user:456".to_vec(),
//!     12346
//! )?;
//! writer.append(&delete_entry)?;
//!
//! // Force sync to disk
//! writer.sync()?;
//! # Ok::<(), ferrisdb_core::Error>(())
//! ```
//!
//! ## Reading from WAL
//!
//! ```no_run
//! use ferrisdb_storage::wal::{WALReader, WALHeader};
//! use ferrisdb_core::Operation;
//!
//! let mut reader = WALReader::new("path/to/wal.log")?;
//!
//! // Check header information
//! let header = reader.header();
//! println!("WAL version: {:#x}", header.version);
//! println!("File sequence: {}", header.file_sequence);
//!
//! // Read all entries for recovery
//! let entries = reader.read_all()?;
//! for entry in entries {
//!     match entry.operation {
//!         Operation::Put => {
//!             println!("Put: {:?} = {:?}", entry.key, entry.value);
//!         }
//!         Operation::Delete => {
//!             println!("Delete: {:?}", entry.key);
//!         }
//!     }
//! }
//! # Ok::<(), ferrisdb_core::Error>(())
//! ```
//!
//! ## Using Iterator Interface
//!
//! ```no_run
//! use ferrisdb_storage::wal::WALReader;
//!
//! let reader = WALReader::new("path/to/wal.log")?;
//!
//! // Process entries one by one
//! for entry_result in reader {
//!     match entry_result {
//!         Ok(entry) => {
//!             // Process the entry
//!             println!("Entry at {}: {:?}", entry.timestamp, entry.operation);
//!         }
//!         Err(e) => {
//!             // Handle corruption or I/O errors
//!             eprintln!("Error reading entry: {}", e);
//!             break;
//!         }
//!     }
//! }
//! # Ok::<(), ferrisdb_core::Error>(())
//! ```

mod header;
mod log_entry;
mod metrics;
mod reader;
mod writer;

pub use header::{WALHeader, WAL_CURRENT_VERSION, WAL_HEADER_SIZE, WAL_MAGIC};
pub use log_entry::WALEntry;
pub use metrics::{TimedOperation, WALMetrics};
pub use reader::WALReader;
pub use writer::WALWriter;
