use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Get test output directory path
fn get_test_dir(test_name: &str) -> PathBuf {
    PathBuf::from("test-output")
        .join("integration")
        .join(test_name)
}

/// Setup test directory - removes old artifacts if present
fn setup_test_dir(test_name: &str) -> PathBuf {
    let dir = get_test_dir(test_name);
    // Clean up any previous test artifacts
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn test_dry_run_does_not_create_files() {
    let test_dir = setup_test_dir("dry_run_no_create");

    // Create a target project directory
    let target_dir = test_dir.join("test_project");
    fs::create_dir_all(&target_dir).unwrap();
    fs::write(target_dir.join("Cargo.toml"), "[package]").unwrap();

    // Run the CLI in dry-run mode
    let output = Command::new("cargo")
        .args(["run", "--", "-n", target_dir.to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    // Check that the command succeeded
    assert!(
        output.status.success(),
        "Command failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Check that output directory was NOT created
    let output_dir = target_dir.join("docs");
    assert!(
        !output_dir.exists(),
        "Output directory should not exist in dry-run mode"
    );

    // Check that the output mentions dry run
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("DRY RUN"), "Output should mention dry run");

    // No cleanup - leave for inspection
}

#[test]
fn test_verbose_creates_files() {
    let test_dir = setup_test_dir("verbose_creates");

    // Create a dummy project directory
    let project_dir = test_dir.join("dummy_project");
    fs::create_dir_all(&project_dir).unwrap();
    fs::write(project_dir.join("Cargo.toml"), "[package]\nname = \"test\"").unwrap();

    // Run the CLI in verbose mode (not dry-run) - output goes to project_dir/docs
    let output = Command::new("cargo")
        .args(["run", "--", "-v", project_dir.to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    // Check that the command succeeded
    assert!(
        output.status.success(),
        "Command failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Check that output directory WAS created IN THE PROJECT directory
    let actual_output_dir = project_dir.join("docs");
    assert!(
        actual_output_dir.exists(),
        "Output directory should exist in project directory"
    );

    // Check that the output file was created
    let output_file = actual_output_dir.join("ai_agent_instructions.md");
    assert!(output_file.exists(), "Output file should exist");

    // Check that the stderr mentions mkdir and write operations
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("mkdir -p") || stderr.contains("Directory already exists"),
        "Verbose output should show directory operations"
    );
    assert!(
        stderr.contains("write"),
        "Verbose output should show write operations"
    );

    // No cleanup - leave for inspection
}

#[test]
fn test_dry_run_with_custom_output() {
    let test_dir = setup_test_dir("dry_run_custom");

    // Create a target project directory
    let target_dir = test_dir.join("target_project");
    fs::create_dir_all(&target_dir).unwrap();
    fs::write(target_dir.join("package.json"), "{}").unwrap();

    // Run the CLI in dry-run mode with custom output directory
    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "--dry-run",
            "-o",
            "custom-docs",
            target_dir.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    // Check that the command succeeded
    assert!(
        output.status.success(),
        "Command failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Check that custom output directory was NOT created
    let expected_output_dir = target_dir.join("custom-docs");
    assert!(
        !expected_output_dir.exists(),
        "Custom output directory should not exist in dry-run mode"
    );

    // Check that stderr shows the mkdir command that would be run
    let stderr = String::from_utf8_lossy(&output.stderr);
    let expected_mkdir = format!("mkdir -p {}", expected_output_dir.display());
    assert!(
        stderr.contains(&expected_mkdir),
        "Dry-run should show mkdir command for custom directory: {stderr}"
    );

    // Check that stdout shows the file that would be created
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains(&format!(
            "Would create: {}/ai_agent_instructions.md",
            expected_output_dir.display()
        )),
        "Dry-run should show the file that would be created"
    );

    // No cleanup - leave for inspection
}

#[test]
fn test_output_goes_to_target_directory() {
    let test_dir = setup_test_dir("output_to_target");

    // Create a target project directory
    let target_dir = test_dir.join("my_project");
    fs::create_dir_all(&target_dir).unwrap();
    fs::write(target_dir.join("package.json"), "{}").unwrap();

    // Run without -o flag - should create docs in TARGET directory
    let output = Command::new("cargo")
        .args(["run", "--", target_dir.to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    assert!(
        output.status.success(),
        "Command failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Check that docs directory was created IN THE TARGET directory
    let expected_docs_dir = target_dir.join("docs");
    assert!(
        expected_docs_dir.exists(),
        "docs directory should be created in target directory, not current directory"
    );

    let expected_file = expected_docs_dir.join("ai_agent_instructions.md");
    assert!(
        expected_file.exists(),
        "ai_agent_instructions.md should exist in target/docs directory"
    );

    // Also check that learnings.md was copied
    let expected_learnings = expected_docs_dir.join("learnings.md");
    assert!(
        expected_learnings.exists(),
        "learnings.md should be copied to target/docs directory"
    );

    // No cleanup - leave for inspection
}

#[test]
fn test_custom_output_dir_relative_to_target() {
    let test_dir = setup_test_dir("custom_output_relative");

    // Create a target project directory
    let target_dir = test_dir.join("another_project");
    fs::create_dir_all(&target_dir).unwrap();
    fs::write(target_dir.join("Cargo.toml"), "[package]").unwrap();

    // Run with -o flag - should create custom dir in TARGET directory
    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "-o",
            "documentation",
            target_dir.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(
        output.status.success(),
        "Command failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Check that custom directory was created IN THE TARGET directory
    let expected_docs_dir = target_dir.join("documentation");
    assert!(
        expected_docs_dir.exists(),
        "documentation directory should be created in target directory"
    );

    let expected_file = expected_docs_dir.join("ai_agent_instructions.md");
    assert!(
        expected_file.exists(),
        "ai_agent_instructions.md should exist in target/documentation directory"
    );

    // Check learnings.md in custom location
    let expected_learnings = expected_docs_dir.join("learnings.md");
    assert!(
        expected_learnings.exists(),
        "learnings.md should be copied to target/documentation directory"
    );

    // No cleanup - leave for inspection
}

#[test]
fn test_learnings_append_to_existing() {
    let test_dir = setup_test_dir("learnings_append");

    // Create a target project directory with existing learnings.md
    let target_dir = test_dir.join("existing_project");
    let docs_dir = target_dir.join("docs");
    fs::create_dir_all(&docs_dir).unwrap();
    fs::write(target_dir.join("package.json"), "{}").unwrap();

    // Create existing learnings.md
    let existing_content = "# Existing Learnings\n\nSome existing content here.\n";
    fs::write(docs_dir.join("learnings.md"), existing_content).unwrap();

    // Run the CLI
    let output = Command::new("cargo")
        .args(["run", "--", target_dir.to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    assert!(
        output.status.success(),
        "Command failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Check that learnings.md was appended, not replaced
    let learnings_content = fs::read_to_string(docs_dir.join("learnings.md")).unwrap();
    assert!(
        learnings_content.contains("Existing Learnings"),
        "Original content should be preserved"
    );
    assert!(
        learnings_content.contains("---- Added"),
        "Should have separator with timestamp"
    );
    assert!(
        learnings_content.contains("Learnings from Development Issues"),
        "New content should be appended"
    );

    // No cleanup - leave for inspection
}
