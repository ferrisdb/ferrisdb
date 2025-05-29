//! Error types for FerrisDB
//!
//! This module defines the error types used throughout FerrisDB.

use thiserror::Error;

/// The main error type for FerrisDB operations
#[derive(Error, Debug)]
pub enum Error {
    /// An I/O error occurred
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// A serialization/deserialization error occurred
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// The requested key was not found
    #[error("Key not found")]
    KeyNotFound,

    /// Data corruption was detected
    #[error("Corruption detected: {0}")]
    Corruption(String),

    /// An invalid operation was attempted
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    /// A storage engine error occurred
    #[error("Storage engine error: {0}")]
    StorageEngine(String),

    /// MemTable is full and needs to be flushed
    #[error("MemTable is full")]
    MemTableFull,

    /// Invalid file or data format
    #[error("Invalid format: {0}")]
    InvalidFormat(String),

    /// Entry size exceeds the maximum allowed limit
    #[error("Entry size exceeds maximum: {size} > {max_size}")]
    EntrySizeExceeded { size: usize, max_size: usize },

    /// Attempted to use a resource that has already been consumed or finished
    #[error("Resource already consumed: {0}")]
    ResourceConsumed(String),

    /// Attempted to finish an operation with no data
    #[error("Operation has no data: {0}")]
    EmptyOperation(String),

    /// A transaction error occurred
    #[error("Transaction error: {0}")]
    Transaction(String),
}

/// A specialized Result type for FerrisDB operations
pub type Result<T> = std::result::Result<T, Error>;
