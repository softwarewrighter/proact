//! Tests for project metadata extraction

mod test_helpers;

use proact::metadata::{ProjectMetadata, generate_mit_license};
use std::fs;

#[test]
fn test_extract_metadata_from_cargo_toml() {
    let temp_dir = test_helpers::setup_test_dir("metadata_cargo");
    fs::write(
        temp_dir.join("Cargo.toml"),
        r#"[package]
name = "test"
license = "MIT"
"#,
    )
    .unwrap();

    let metadata = ProjectMetadata::extract(&temp_dir).unwrap();
    assert_eq!(metadata.license, "MIT");
}

#[test]
fn test_extract_metadata_from_package_json() {
    let temp_dir = test_helpers::setup_test_dir("metadata_npm");
    fs::write(
        temp_dir.join("package.json"),
        r#"{
  "name": "test",
  "license": "Apache-2.0"
}"#,
    )
    .unwrap();

    let metadata = ProjectMetadata::extract(&temp_dir).unwrap();
    assert_eq!(metadata.license, "Apache-2.0");
}

#[test]
fn test_copyright_string() {
    let temp_dir = test_helpers::setup_test_dir("metadata_copyright");
    fs::write(temp_dir.join("Cargo.toml"), "[package]").unwrap();

    let metadata = ProjectMetadata::extract(&temp_dir).unwrap();
    let copyright = metadata.copyright_string();
    assert!(copyright.starts_with("Copyright (c)"));
    assert!(copyright.contains(&metadata.current_year));
}

#[test]
fn test_generate_mit_license() {
    let temp_dir = test_helpers::setup_test_dir("metadata_license");
    fs::write(
        temp_dir.join("Cargo.toml"),
        r#"[package]
name = "test"
license = "MIT"
"#,
    )
    .unwrap();

    let metadata = ProjectMetadata::extract(&temp_dir).unwrap();
    let license = generate_mit_license(&metadata);
    assert!(license.contains("MIT License"));
    assert!(license.contains(&metadata.copyright_string()));
}

#[test]
fn test_default_license_placeholder() {
    let temp_dir = test_helpers::setup_test_dir("metadata_no_license");
    // No Cargo.toml or package.json

    let metadata = ProjectMetadata::extract(&temp_dir).unwrap();
    assert_eq!(metadata.license, "<license>");
}
