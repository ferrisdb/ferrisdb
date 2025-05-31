//! Challenge 1: Add compression to reduce WAL size
//! 
//! Goal: Implement optional compression for WAL entries using the flate2 crate.
//! 
//! Requirements:
//! 1. Add a compression flag to the WAL header
//! 2. Compress entry data before writing (after calculating checksum)
//! 3. Decompress when reading
//! 4. Make compression optional via the builder pattern
//! 
//! Hints:
//! - Add `flate2 = "1.0"` to Cargo.toml
//! - Use flate2::Compression::default() for balanced speed/size
//! - Compress the entire entry (except length prefix)
//! - Update the header version to indicate compression support

// TODO: Your implementation here!

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    #[ignore] // Remove this when you implement the solution
    fn test_compression() {
        // Test that compressed WAL is smaller than uncompressed
        // for repetitive data
    }
}