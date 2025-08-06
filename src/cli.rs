use clap::Parser;
use std::path::PathBuf;

/// CLI for generating AI coding agent documentation
///
/// This module handles command-line argument parsing for the Proact tool,
/// which generates comprehensive documentation for AI coding agents.
#[derive(Parser, Debug)]
#[command(
    name = "proact",
    about = "Generate documentation for AI coding agents",
    long_about = "Proact generates comprehensive documentation that instructs AI coding agents \
                  to follow best practices, apply continuous improvement, and use tools like \
                  Playwright MCP for browser automation.",
    version,
    author
)]
pub struct Args {
    /// Path to the target project directory
    ///
    /// This should be an existing directory where the AI agent documentation
    /// will be tailored for. The path can be absolute or relative.
    #[arg(value_name = "TARGET", help = "Path to an existing project directory")]
    pub target: PathBuf,

    /// Enable verbose output
    ///
    /// When enabled, provides detailed information about the generation process,
    /// including what content is being included and where files are being written.
    #[arg(short = 'v', long = "verbose", help = "Enable verbose output")]
    pub verbose: bool,

    /// Output directory for generated documentation
    ///
    /// The directory where the AI agent documentation will be written.
    /// Will be created if it doesn't exist.
    #[arg(
        short = 'o',
        long = "output-dir",
        value_name = "DIR",
        default_value = "docs",
        help = "Output directory for generated documentation"
    )]
    pub output_dir: PathBuf,

    /// Perform a dry run without creating files
    ///
    /// Shows all operations that would be performed without actually
    /// executing them. Implies verbose mode.
    #[arg(
        short = 'n',
        long = "dry-run",
        help = "Show what would be done without actually doing it"
    )]
    pub dry_run: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing() {
        let args = Args::parse_from(["proact", "../test-proj"]);
        assert_eq!(args.target, PathBuf::from("../test-proj"));
        assert_eq!(args.output_dir, PathBuf::from("docs"));
        assert!(!args.verbose);
        assert!(!args.dry_run);
    }

    #[test]
    fn test_cli_with_verbose() {
        let args = Args::parse_from(["proact", "-v", "../test-proj"]);
        assert!(args.verbose);
    }

    #[test]
    fn test_cli_with_output_dir() {
        let args = Args::parse_from(["proact", "-o", "./output", "../test-proj"]);
        assert_eq!(args.output_dir, PathBuf::from("./output"));
    }

    #[test]
    fn test_cli_long_options() {
        let args = Args::parse_from([
            "proact",
            "--verbose",
            "--output-dir",
            "./custom",
            "../project",
        ]);
        assert!(args.verbose);
        assert_eq!(args.output_dir, PathBuf::from("./custom"));
        assert_eq!(args.target, PathBuf::from("../project"));
    }

    #[test]
    fn test_cli_with_dry_run() {
        let args = Args::parse_from(["proact", "-n", "../test-proj"]);
        assert!(args.dry_run);
    }

    #[test]
    fn test_cli_with_dry_run_long() {
        let args = Args::parse_from(["proact", "--dry-run", "../test-proj"]);
        assert!(args.dry_run);
    }
}
