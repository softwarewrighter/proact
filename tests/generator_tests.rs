//! Tests for documentation generator

mod test_helpers;

use proact::generator;
use std::fs;

#[test]
fn test_generate_documentation() {
    let temp_dir = test_helpers::setup_test_dir("generate_doc");

    let result = generator::generate_documentation(&temp_dir, false);
    assert!(result.is_ok());

    let doc = result.unwrap();
    assert!(doc.contains("AI Coding Agent Development Process Guidelines"));
    assert!(doc.contains("Quality-Oriented Development"));
    assert!(doc.contains("Continuous Improvement"));
    assert!(doc.contains("Playwright MCP"));
}

#[test]
fn test_rust_project_detection() {
    let temp_dir = test_helpers::setup_test_dir("rust_detection");
    fs::write(temp_dir.join("Cargo.toml"), "[package]").unwrap();

    let doc = generator::generate_documentation(&temp_dir, false).unwrap();
    assert!(doc.contains("Rust Development"));
}

#[test]
fn test_javascript_project_detection() {
    let temp_dir = test_helpers::setup_test_dir("js_detection");
    fs::write(temp_dir.join("package.json"), "{}").unwrap();

    let doc = generator::generate_documentation(&temp_dir, false).unwrap();
    assert!(doc.contains("JavaScript/Node.js Development"));
}

#[test]
fn test_python_project_detection() {
    let temp_dir = test_helpers::setup_test_dir("python_detection");
    fs::write(temp_dir.join("requirements.txt"), "").unwrap();

    let doc = generator::generate_documentation(&temp_dir, false).unwrap();
    assert!(doc.contains("Python Development"));
}
