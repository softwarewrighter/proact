//! Process and quality guideline templates

/// Returns process guidelines template
pub fn process_guidelines() -> &'static str {
    include_str!("../../../../../templates/process_guidelines.md")
}

/// Returns quality standards template
pub fn quality_standards() -> &'static str {
    include_str!("../../../../../templates/quality_standards.md")
}

/// Returns continuous improvement template
pub fn continuous_improvement() -> &'static str {
    include_str!("../../../../../templates/continuous_improvement.md")
}

/// Returns code metric gates and architecture guide template
pub fn code_metrics() -> &'static str {
    include_str!("../../../../../templates/code_metrics.md")
}

/// Returns summary template
pub fn summary() -> &'static str {
    include_str!("../../../../../templates/summary.md")
}
