//! Test runner for tutorial exercises
//!
//! Run with: cargo test --test exercises

// Include all challenge files as modules
#[path = "../exercises/challenge_01_delete.rs"]
mod challenge_01_delete;

#[path = "../exercises/challenge_02_ttl.rs"]
mod challenge_02_ttl;

#[path = "../exercises/challenge_03_case_insensitive.rs"]
mod challenge_03_case_insensitive;

#[path = "../exercises/challenge_04_prefix_scan.rs"]
mod challenge_04_prefix_scan;
