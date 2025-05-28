//! Core types used throughout FerrisDB
//!
//! This module contains the fundamental data types that form the basis
//! of FerrisDB's data model and configuration.

use serde::{Deserialize, Serialize};

/// A key in the database, represented as a byte vector
pub type Key = Vec<u8>;

/// A value in the database, represented as a byte vector
pub type Value = Vec<u8>;

/// A monotonically increasing sequence number for ordering operations
pub type SequenceNumber = u64;

/// A timestamp for MVCC (Multi-Version Concurrency Control)
pub type Timestamp = u64;

/// The type of operation performed on a key
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Operation {
    /// Insert or update a key-value pair
    Put,
    /// Delete a key
    Delete,
}

/// A simple key-value pair
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyValue {
    /// The key
    pub key: Key,
    /// The value
    pub value: Value,
}

/// A key-value pair with timestamp and operation type for MVCC
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimestampedKeyValue {
    /// The key
    pub key: Key,
    /// The value (empty for Delete operations)
    pub value: Value,
    /// The timestamp when this operation occurred
    pub timestamp: Timestamp,
    /// The type of operation
    pub operation: Operation,
}

/// Compression algorithms supported by the storage engine
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompressionType {
    /// No compression
    None,
    /// LZ4 compression (fast, moderate compression ratio)
    Lz4,
    /// Snappy compression (very fast, lower compression ratio)
    Snappy,
}

/// Synchronization modes for write-ahead logging
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncMode {
    /// No synchronization (fastest, least durable)
    None,
    /// Normal synchronization (flush to OS buffer)
    Normal,
    /// Full synchronization (flush to disk)
    Full,
}
