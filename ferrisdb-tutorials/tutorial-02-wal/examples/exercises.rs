//! Exercise test runner for Tutorial 2
//! 
//! Run with: cargo test --example exercises
//! 
//! This allows students to test their exercise implementations
//! without breaking CI (which ignores examples).

// Import exercise modules
mod exercises {
    #[path = "exercises/challenge_01_compression.rs"]
    pub mod challenge_01_compression;
    
    #[path = "exercises/challenge_02_rotation.rs"] 
    pub mod challenge_02_rotation;
    
    #[path = "exercises/challenge_03_concurrent.rs"]
    pub mod challenge_03_concurrent;
}

fn main() {
    println!("Run exercise tests with: cargo test --example exercises");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_exercise_compilation() {
        // This just ensures exercises compile
        // Students will implement actual tests in the exercise files
        println!("Exercises compile successfully!");
    }
}