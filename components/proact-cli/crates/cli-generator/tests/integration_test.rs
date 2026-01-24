//! Integration tests for cli-generator

use std::fs;
use std::path::PathBuf;

fn test_output_dir() -> PathBuf {
    // Navigate from crate dir to repo root, then to test-output
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../../test-output")
}

#[test]
fn generates_template_files() {
    let output_dir = test_output_dir();
    fs::create_dir_all(&output_dir).expect("create test-output dir");

    cli_generator::copy_templates(&output_dir, false, false).expect("copy templates");

    // Verify youtube.md exists with expected content
    let youtube = fs::read_to_string(output_dir.join("youtube.md")).expect("read youtube.md");
    assert!(youtube.contains("@SoftwareWrighter playlists"));
    assert!(youtube.contains("https://www.youtube.com/@SoftwareWrighter/playlists"));

    // Verify other templates exist
    assert!(output_dir.join("process.md").exists());
    assert!(output_dir.join("tools.md").exists());
}
