//! Core metadata types for proact

use anyhow::Result;
use std::path::Path;

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
        let author_name = metadata_extract::get_git_author()?;
        let license = metadata_extract::extract_license(project_path);
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

/// Generate MIT LICENSE file content
pub fn generate_mit_license(metadata: &ProjectMetadata) -> String {
    include_str!("mit_license.txt").replace("{copyright}", &metadata.copyright_string())
}
