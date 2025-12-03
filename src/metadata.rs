//! Metadata extraction for projects

use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

/// Project metadata extracted from various sources
#[derive(Debug, Clone)]
pub struct ProjectMetadata {
    pub current_year: String,
    pub author_name: String,
    pub license: String,
}

impl ProjectMetadata {
    /// Extract metadata for a given project path
    pub fn extract(project_path: &Path) -> Result<Self> {
        let current_year = chrono::Local::now().format("%Y").to_string();
        let author_name = get_git_author()?;
        let license = extract_license(project_path);

        Ok(Self {
            current_year,
            author_name,
            license,
        })
    }

    /// Get formatted copyright string
    pub fn copyright_string(&self) -> String {
        format!("Copyright (c) {} {}", self.current_year, self.author_name)
    }
}

/// Extract git author name (from env var or git config)
fn get_git_author() -> Result<String> {
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

/// Extract license from project files
fn extract_license(project_path: &Path) -> String {
    // Try Cargo.toml
    if let Some(license) = extract_from_file(project_path, "Cargo.toml", "license", '=') {
        return license;
    }
    // Try package.json
    if let Some(license) = extract_from_file(project_path, "package.json", "\"license\"", ':') {
        return license;
    }
    "<license>".to_string()
}

/// Extract a field value from a project file
fn extract_from_file(path: &Path, file: &str, key: &str, sep: char) -> Option<String> {
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

/// Generate MIT LICENSE file content
pub fn generate_mit_license(metadata: &ProjectMetadata) -> String {
    format!(
        r#"MIT License

{}

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
"#,
        metadata.copyright_string()
    )
}
