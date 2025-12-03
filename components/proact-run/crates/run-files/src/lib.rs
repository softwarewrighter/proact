//! File operation helpers for proact

use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

/// Resolve output directory from target and output_dir paths
pub fn resolve_output_dir(target: &Path, output_dir: &Path) -> PathBuf {
    if output_dir.is_absolute() {
        output_dir.to_path_buf()
    } else {
        target.join(output_dir)
    }
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
