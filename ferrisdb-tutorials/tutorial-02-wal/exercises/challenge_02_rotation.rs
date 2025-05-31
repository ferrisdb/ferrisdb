//! Challenge 2: Implement log rotation
//! 
//! Goal: Rotate to a new file when the current WAL reaches max_file_size.
//! 
//! Requirements:
//! 1. When max_file_size is reached, close current file
//! 2. Rename it to filename.wal.1 (or .2, .3, etc.)
//! 3. Create a new filename.wal
//! 4. Recovery should read all rotated files in order
//! 
//! Hints:
//! - Keep track of current file size
//! - Use a naming scheme like: data.wal, data.wal.1, data.wal.2
//! - Recovery should sort files by their suffix number
//! - Consider what happens if rotation fails mid-write

// TODO: Your implementation here!

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    #[ignore] // Remove this when you implement the solution
    fn test_rotation() {
        // Test that files rotate when size limit is reached
        // Test that recovery reads all rotated files
    }
}