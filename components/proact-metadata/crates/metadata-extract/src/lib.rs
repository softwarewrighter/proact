//! Metadata extraction helpers

use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

/// Extract git author name (from env var or git config)
pub fn get_git_author() -> Result<String> {
    if let Ok(owner) = std::env::var("PROACT_C_OWNER") {
        return Ok(owner.trim().to_string());
    }
    let output = Command::new("git")
        .args(["config", "user.name"])
        .output()
        .context("Failed to get git user.name")?;
    Ok(if output.status.success() {
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    } else {
        "<author>".to_string()
    })
}

/// Get the year of the first git commit in the project (env override: PROACT_C_START_YEAR)
pub fn get_first_commit_year(project_path: &Path) -> Option<String> {
    if let Ok(year) = std::env::var("PROACT_C_START_YEAR")
        && !year.trim().is_empty()
    {
        return Some(year.trim().to_string());
    }
    let dir = project_path.display().to_string();
    let args = [
        "-C",
        &dir,
        "log",
        "--reverse",
        "--format=%cd",
        "--date=format:%Y",
    ];
    let output = Command::new("git").args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let year = stdout.lines().next()?.trim().to_string();
    if year.is_empty() { None } else { Some(year) }
}

/// Extract license from project files
pub fn extract_license(project_path: &Path) -> String {
    if let Some(license) = extract_from_file(project_path, "Cargo.toml", "license", '=') {
        return license;
    }
    if let Some(license) = extract_from_file(project_path, "package.json", "\"license\"", ':') {
        return license;
    }
    "<license>".to_string()
}

/// Extract a field value from a project file
pub fn extract_from_file(path: &Path, file: &str, key: &str, sep: char) -> Option<String> {
    let content = std::fs::read_to_string(path.join(file)).ok()?;
    for line in content.lines() {
        let trimmed = line.trim();
        if (trimmed.starts_with(key) || trimmed.contains(key))
            && let Some(value) = trimmed.split(sep).nth(1)
        {
            let clean = value.trim().trim_matches(|c| c == '"' || c == ',').trim();
            if !clean.is_empty() && !clean.starts_with('{') {
                return Some(clean.to_string());
            }
        }
    }
    None
}
