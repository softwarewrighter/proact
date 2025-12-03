//! Shared test utilities for proact tests

use std::fs;
use std::path::PathBuf;

/// Get test output directory path
pub fn get_test_dir(test_name: &str) -> PathBuf {
    PathBuf::from("test-output").join(test_name)
}

/// Setup test directory - removes old artifacts if present
pub fn setup_test_dir(test_name: &str) -> PathBuf {
    let dir = get_test_dir(test_name);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}
