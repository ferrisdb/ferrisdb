//! Sorted String Table (SSTable) implementation
//!
//! SSTables are immutable on-disk files that store sorted key-value pairs.
//! They are the persistent storage format for FerrisDB and are organized
//! in levels for efficient reads.
//!
//! # Format
//!
//! An SSTable consists of:
//! - **Data Blocks**: Sorted key-value pairs in fixed-size blocks
//! - **Index Blocks**: Pointers to data blocks for binary search
//! - **Bloom Filter**: Probabilistic filter to avoid unnecessary reads
//! - **Footer**: Metadata including index offset and magic number
//!
//! # Features
//!
//! - Block compression (LZ4, Snappy)
//! - Prefix compression for keys within blocks
//! - Checksums for corruption detection
//! - Bloom filters for existence checks

pub mod reader;
pub mod writer;
