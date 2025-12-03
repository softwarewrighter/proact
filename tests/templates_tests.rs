//! Tests for documentation templates

use proact::templates;

#[test]
fn test_process_guidelines_not_empty() {
    assert!(!templates::process_guidelines().is_empty());
    assert!(templates::process_guidelines().contains("Process-Oriented Workflow"));
}

#[test]
fn test_quality_standards_not_empty() {
    assert!(!templates::quality_standards().is_empty());
    assert!(templates::quality_standards().contains("Quality-Oriented Development"));
}

#[test]
fn test_continuous_improvement_not_empty() {
    assert!(!templates::continuous_improvement().is_empty());
    assert!(templates::continuous_improvement().contains("Continuous Improvement"));
}

#[test]
fn test_playwright_setup_not_empty() {
    assert!(!templates::playwright_mcp_setup().is_empty());
    assert!(templates::playwright_mcp_setup().contains("Playwright MCP"));
}

#[test]
fn test_summary_not_empty() {
    assert!(!templates::summary().is_empty());
    assert!(templates::summary().contains("Expected Proactive Behaviors"));
}
