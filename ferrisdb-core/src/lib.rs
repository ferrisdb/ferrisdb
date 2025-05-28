//! Core types and traits for FerrisDB
//!
//! This crate contains the fundamental types and error handling used throughout
//! the FerrisDB project. It provides:
//!
//! - Common error types with [`Error`] and [`Result`]
//! - Basic data types like [`Key`], [`Value`], and [`Operation`]
//! - Configuration types for storage and synchronization
//!
//! # Example
//!
//! ```
//! use ferrisdb_core::{Key, Value, Operation};
//!
//! let key: Key = b"user:123".to_vec();
//! let value: Value = b"John Doe".to_vec();
//! let op = Operation::Put;
//! ```

pub mod error;
pub mod types;

pub use error::{Error, Result};
pub use types::*;
