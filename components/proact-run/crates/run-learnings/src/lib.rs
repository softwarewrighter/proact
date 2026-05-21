//! Learnings file management for proact

use anyhow::Result;
use chrono::Local;
use std::fs;
use std::path::Path;

/// Read learnings source content if it exists
pub fn read_learnings_source() -> Option<String> {
    let source = Path::new("docs/learnings.md");
    fs::read_to_string(source).ok()
}

/// Append content to existing learnings file (idempotent: skips when content already present)
pub fn append_learnings(target: &Path, content: &str, verbose: bool) -> Result<()> {
    let existing = fs::read_to_string(target)?;
    if existing.contains(content.trim()) {
        if verbose {
            eprintln!("skip {} (already current)", target.display());
        }
        return Ok(());
    }
    let timestamp = Local::now().format("%Y%m%dT%H%M%S").to_string();
    let separator = format!("\n\n---- Added {timestamp} ----\n\n");
    if verbose {
        eprintln!("append {} (+ {} bytes)", target.display(), content.len());
    }
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
