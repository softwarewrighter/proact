//! Templates for AI agent documentation

/// Returns the process guidelines template
pub fn process_guidelines() -> &'static str {
    include_str!("../templates/process_guidelines.md")
}

/// Returns the quality standards template
pub fn quality_standards() -> &'static str {
    include_str!("../templates/quality_standards.md")
}

/// Returns the continuous improvement template
pub fn continuous_improvement() -> &'static str {
    include_str!("../templates/continuous_improvement.md")
}

/// Returns the Playwright MCP setup instructions
pub fn playwright_mcp_setup() -> &'static str {
    include_str!("../templates/playwright_mcp_setup.md")
}

/// Returns the summary section
pub fn summary() -> &'static str {
    include_str!("../templates/summary.md")
}

/// Returns project-specific notes for the given project type
pub fn project_notes(project_type: &str) -> &'static str {
    match project_type {
        "rust" => include_str!("../templates/rust_notes.md"),
        "javascript" => include_str!("../templates/javascript_notes.md"),
        "python" => include_str!("../templates/python_notes.md"),
        _ => "",
    }
}
