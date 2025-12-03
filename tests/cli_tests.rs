//! Tests for CLI argument parsing

use clap::Parser;
use proact::cli::Args;
use std::path::PathBuf;

#[test]
fn test_cli_parsing() {
    let args = Args::parse_from(["proact", "../test-proj"]);
    assert_eq!(args.target, PathBuf::from("../test-proj"));
    assert_eq!(args.output_dir, PathBuf::from("docs"));
    assert!(!args.verbose);
    assert!(!args.dry_run);
}

#[test]
fn test_cli_with_verbose() {
    let args = Args::parse_from(["proact", "-v", "../test-proj"]);
    assert!(args.verbose);
}

#[test]
fn test_cli_with_output_dir() {
    let args = Args::parse_from(["proact", "-o", "./output", "../test-proj"]);
    assert_eq!(args.output_dir, PathBuf::from("./output"));
}

#[test]
fn test_cli_long_options() {
    let args = Args::parse_from([
        "proact",
        "--verbose",
        "--output-dir",
        "./custom",
        "../project",
    ]);
    assert!(args.verbose);
    assert_eq!(args.output_dir, PathBuf::from("./custom"));
    assert_eq!(args.target, PathBuf::from("../project"));
}

#[test]
fn test_cli_with_dry_run() {
    let args = Args::parse_from(["proact", "-n", "../test-proj"]);
    assert!(args.dry_run);
}

#[test]
fn test_cli_with_dry_run_long() {
    let args = Args::parse_from(["proact", "--dry-run", "../test-proj"]);
    assert!(args.dry_run);
}
