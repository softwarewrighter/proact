//! Runtime helpers for the proact CLI

use anyhow::Result;
use chrono::Local;
use std::fs;
use std::path::Path;

use crate::cli;

/// Resolve output directory from args
pub fn resolve_output_dir(args: &cli::Args) -> std::path::PathBuf {
    if args.output_dir.is_absolute() {
        args.output_dir.clone()
    } else {
        args.target.join(&args.output_dir)
    }
}

/// Read learnings source content if it exists
pub fn read_learnings_source() -> Option<String> {
    let source = Path::new("docs/learnings.md");
    fs::read_to_string(source).ok()
}

/// Append content to existing learnings file
pub fn append_learnings(target: &Path, content: &str, verbose: bool) -> Result<()> {
    let timestamp = Local::now().format("%Y%m%dT%H%M%S").to_string();
    let separator = format!("\n\n---- Added {timestamp} ----\n\n");
    if verbose {
        eprintln!("append {} (+ {} bytes)", target.display(), content.len());
    }
    let existing = fs::read_to_string(target)?;
    fs::write(target, format!("{existing}{separator}{content}"))?;
    Ok(())
}

/// Write new learnings file
pub fn write_learnings(target: &Path, content: &str, verbose: bool) -> Result<()> {
    if verbose {
        eprintln!("write {} ({} bytes)", target.display(), content.len());
    }
    fs::write(target, content)?;
    Ok(())
}

/// Create output directory if needed
pub fn create_output_dir(dir: &Path, verbose: bool, dry_run: bool) -> Result<()> {
    if !dir.exists() {
        if verbose {
            eprintln!("mkdir -p {}", dir.display());
        }
        if !dry_run {
            fs::create_dir_all(dir)?;
        }
    }
    Ok(())
}

/// Write documentation file
pub fn write_doc_file(path: &Path, content: &str, verbose: bool, dry_run: bool) -> Result<()> {
    if verbose {
        eprintln!("write {} ({} bytes)", path.display(), content.len());
    }
    if !dry_run {
        fs::write(path, content)?;
    }
    Ok(())
}

/// Print completion messages
pub fn print_completion(file: &Path, dir: &Path, target: &Path, dry_run: bool) {
    let (prefix, verb) = if dry_run {
        ("DRY RUN completed - no files were created", "Would create")
    } else {
        ("AI agent documentation generated successfully!", "Created")
    };
    println!("{prefix}");
    println!("{verb}: {}", file.display());
    println!("{verb}: {}", dir.join("process.md").display());
    println!("{verb}: {}", dir.join("tools.md").display());
    println!("{verb}: {}", target.join("COPYRIGHT").display());
    println!("{verb}: {}", target.join("LICENSE").display());
}
