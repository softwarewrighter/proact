//! Project-specific and MCP templates

/// Returns Playwright MCP setup template
pub fn playwright_mcp_setup() -> &'static str {
    include_str!("../../../../../templates/playwright_mcp_setup.md")
}

/// Returns project-specific notes based on project type
pub fn project_notes(project_type: &str) -> &'static str {
    match project_type {
        "rust" => include_str!("../../../../../templates/rust_notes.md"),
        "javascript" => include_str!("../../../../../templates/javascript_notes.md"),
        "python" => include_str!("../../../../../templates/python_notes.md"),
        _ => "",
    }
}
