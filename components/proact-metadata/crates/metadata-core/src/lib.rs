//! Core metadata types for proact

use anyhow::Result;
use std::path::Path;

/// Project metadata extracted from various sources
#[derive(Debug, Clone)]
pub struct ProjectMetadata {
    pub start_year: String,
    pub current_year: String,
    pub author_name: String,
    pub license: String,
}

impl ProjectMetadata {
    /// Extract metadata for a given project path
    pub fn extract(project_path: &Path) -> Result<Self> {
        let current_year = chrono::Local::now().format("%Y").to_string();
        let start_year = metadata_extract::get_first_commit_year(project_path)
            .unwrap_or_else(|| current_year.clone());
        let author_name = metadata_extract::get_git_author()?;
        let license = metadata_extract::extract_license(project_path);
        Ok(Self {
            start_year,
            current_year,
            author_name,
            license,
        })
    }

    /// Get formatted copyright string (year range when work spans years)
    pub fn copyright_string(&self) -> String {
        let year = if self.start_year == self.current_year {
            self.current_year.clone()
        } else {
            format!("{}-{}", self.start_year, self.current_year)
        };
        format!("Copyright (c) {} {}", year, self.author_name)
    }
}

/// Generate MIT LICENSE file content
pub fn generate_mit_license(metadata: &ProjectMetadata) -> String {
    include_str!("mit_license.txt").replace("{copyright}", &metadata.copyright_string())
}
