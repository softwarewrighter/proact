//! Legal file generation and template copying

use anyhow::Result;
use chrono::Local;
use std::fs;
use std::path::Path;

const PROCESS_MD: &str = include_str!("../../../../../templates/process.md");
const TOOLS_MD: &str = include_str!("../../../../../templates/tools.md");
const YOUTUBE_MD: &str = include_str!("../../../../../templates/youtube.md");
const CODE_METRICS_MD: &str = include_str!("../../../../../templates/code_metrics.md");

/// Copy or append template files to output directory
pub fn copy_templates(output_dir: &Path, verbose: bool, dry_run: bool) -> Result<()> {
    for (content, name) in [
        (PROCESS_MD, "process.md"),
        (TOOLS_MD, "tools.md"),
        (YOUTUBE_MD, "youtube.md"),
        (CODE_METRICS_MD, "code_metrics.md"),
    ] {
        let dest = output_dir.join(name);
        let sep = format!(
            "\n\n---\n\n**Added by Proact on {}**\n\n",
            Local::now().format("%Y-%m-%d %H:%M:%S")
        );
        let text = if dest.exists() {
            if verbose {
                eprintln!("append {}", dest.display());
            }
            format!("{}{sep}{content}", fs::read_to_string(&dest)?)
        } else {
            if verbose {
                eprintln!("write {}", dest.display());
            }
            content.to_string()
        };
        if !dry_run {
            fs::write(dest, text)?;
        }
    }
    Ok(())
}

/// Generate COPYRIGHT and LICENSE files
pub fn generate_legal_files(target: &Path, verbose: bool, dry_run: bool) -> Result<()> {
    let meta = metadata_core::ProjectMetadata::extract(target)?;
    let license_content = if meta.license == "MIT" || meta.license == "<license>" {
        metadata_core::generate_mit_license(&meta)
    } else {
        String::new()
    };
    let files = [
        (target.join("COPYRIGHT"), meta.copyright_string()),
        (target.join("LICENSE"), license_content),
    ];
    for (path, content) in files {
        if content.is_empty() {
            continue;
        }
        if verbose {
            eprintln!("write {}", path.display());
        }
        if !dry_run {
            fs::write(path, content)?;
        }
    }
    Ok(())
}
