//! Metadata extraction for projects
//!
//! This module handles extracting metadata from project files
//! (Cargo.toml, package.json, etc.) and the git configuration.

use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

/// Project metadata extracted from various sources
#[derive(Debug, Clone)]
pub struct ProjectMetadata {
    pub current_year: String,
    pub author_name: String,
    #[allow(dead_code)] // Used for future features
    pub author_email: Option<String>,
    pub license: String,
    #[allow(dead_code)] // Used for future features
    pub repository: Option<String>,
}

impl ProjectMetadata {
    /// Extract metadata for a given project path
    pub fn extract(project_path: &Path) -> Result<Self> {
        let current_year = get_current_year();
        let (author_name, author_email) = get_git_author()?;
        let license = extract_license(project_path)?;
        let repository = extract_repository(project_path);

        Ok(Self {
            current_year,
            author_name,
            author_email,
            license,
            repository,
        })
    }

    /// Get formatted copyright string
    pub fn copyright_string(&self) -> String {
        format!("Copyright (c) {} {}", self.current_year, self.author_name)
    }

    /// Get author with email if available
    #[allow(dead_code)] // Used for future features
    pub fn author_with_email(&self) -> String {
        if let Some(email) = &self.author_email {
            format!("{} <{}>", self.author_name, email)
        } else {
            self.author_name.clone()
        }
    }
}

/// Get current year from system
fn get_current_year() -> String {
    chrono::Local::now().format("%Y").to_string()
}

/// Extract git author name and email
fn get_git_author() -> Result<(String, Option<String>)> {
    let name_output = Command::new("git")
        .args(["config", "user.name"])
        .output()
        .context("Failed to get git user.name")?;

    let email_output = Command::new("git")
        .args(["config", "user.email"])
        .output()
        .context("Failed to get git user.email")?;

    let name = if name_output.status.success() {
        String::from_utf8_lossy(&name_output.stdout)
            .trim()
            .to_string()
    } else {
        "<author>".to_string()
    };

    let email = if email_output.status.success() {
        let email_str = String::from_utf8_lossy(&email_output.stdout)
            .trim()
            .to_string();
        if email_str.is_empty() {
            None
        } else {
            Some(email_str)
        }
    } else {
        None
    };

    Ok((name, email))
}

/// Extract license from Cargo.toml, package.json, or return placeholder
fn extract_license(project_path: &Path) -> Result<String> {
    // Try Cargo.toml first
    let cargo_toml = project_path.join("Cargo.toml");
    if cargo_toml.exists()
        && let Ok(content) = std::fs::read_to_string(&cargo_toml)
        && let Some(license) = extract_license_from_toml(&content)
    {
        return Ok(license);
    }

    // Try package.json
    let package_json = project_path.join("package.json");
    if package_json.exists()
        && let Ok(content) = std::fs::read_to_string(&package_json)
        && let Some(license) = extract_license_from_json(&content)
    {
        return Ok(license);
    }

    // Default placeholder
    Ok("<license>".to_string())
}

/// Extract license from TOML content
fn extract_license_from_toml(content: &str) -> Option<String> {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("license")
            && let Some(value) = trimmed.split('=').nth(1)
        {
            let license = value.trim().trim_matches('"').trim();
            if !license.is_empty() {
                return Some(license.to_string());
            }
        }
    }
    None
}

/// Extract license from JSON content (simple extraction)
fn extract_license_from_json(content: &str) -> Option<String> {
    // Simple pattern matching for "license": "MIT" or "license": { "type": "MIT" }
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.contains("\"license\"")
            && let Some(value_part) = trimmed.split(':').nth(1)
        {
            let value = value_part
                .trim()
                .trim_end_matches(',')
                .trim_matches('"')
                .trim();
            if !value.is_empty() && !value.starts_with('{') {
                return Some(value.to_string());
            }
        }
    }
    None
}

/// Extract repository URL from project files
fn extract_repository(project_path: &Path) -> Option<String> {
    // Try Cargo.toml
    let cargo_toml = project_path.join("Cargo.toml");
    if cargo_toml.exists()
        && let Ok(content) = std::fs::read_to_string(&cargo_toml)
    {
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("repository")
                && let Some(value) = trimmed.split('=').nth(1)
            {
                let repo = value.trim().trim_matches('"').trim();
                if !repo.is_empty() {
                    return Some(repo.to_string());
                }
            }
        }
    }

    // Try package.json
    let package_json = project_path.join("package.json");
    if package_json.exists()
        && let Ok(content) = std::fs::read_to_string(&package_json)
    {
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.contains("\"repository\"")
                && let Some(value_part) = trimmed.split(':').nth(1)
            {
                let repo = value_part
                    .trim()
                    .trim_end_matches(',')
                    .trim_matches('"')
                    .trim();
                if !repo.is_empty() && !repo.starts_with('{') {
                    return Some(repo.to_string());
                }
            }
        }
    }

    None
}

/// Generate MIT LICENSE file content
pub fn generate_mit_license(metadata: &ProjectMetadata) -> String {
    format!(
        r#"MIT License

{copyright}

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
        copyright = metadata.copyright_string()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_current_year() {
        let year = get_current_year();
        assert!(year.len() == 4);
        assert!(year.parse::<u32>().is_ok());
    }

    #[test]
    fn test_extract_license_from_toml() {
        let content = r#"
[package]
name = "test"
license = "MIT"
"#;
        assert_eq!(extract_license_from_toml(content), Some("MIT".to_string()));
    }

    #[test]
    fn test_extract_license_from_json() {
        let content = r#"
{
  "name": "test",
  "license": "MIT"
}
"#;
        assert_eq!(extract_license_from_json(content), Some("MIT".to_string()));
    }

    #[test]
    fn test_copyright_string() {
        let metadata = ProjectMetadata {
            current_year: "2025".to_string(),
            author_name: "John Doe".to_string(),
            author_email: Some("john@example.com".to_string()),
            license: "MIT".to_string(),
            repository: None,
        };
        assert_eq!(metadata.copyright_string(), "Copyright (c) 2025 John Doe");
    }

    #[test]
    fn test_author_with_email() {
        let metadata = ProjectMetadata {
            current_year: "2025".to_string(),
            author_name: "John Doe".to_string(),
            author_email: Some("john@example.com".to_string()),
            license: "MIT".to_string(),
            repository: None,
        };
        assert_eq!(metadata.author_with_email(), "John Doe <john@example.com>");
    }

    #[test]
    fn test_generate_mit_license() {
        let metadata = ProjectMetadata {
            current_year: "2025".to_string(),
            author_name: "Test Author".to_string(),
            author_email: None,
            license: "MIT".to_string(),
            repository: None,
        };
        let license = generate_mit_license(&metadata);
        assert!(license.contains("MIT License"));
        assert!(license.contains("Copyright (c) 2025 Test Author"));
    }
}
