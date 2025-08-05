mod cli;
mod generator;
mod templates;

use anyhow::Result;
use clap::Parser;

/// Proact: A CLI that generates documentation for AI coding agents
///
/// This tool creates comprehensive documentation that instructs AI coding agents
/// to follow best practices, apply continuous improvement feedback, and utilize
/// tools like Playwright MCP for browser automation.
fn main() -> Result<()> {
    let args = cli::Args::parse();

    if args.verbose {
        eprintln!("Proact v{}", env!("CARGO_PKG_VERSION"));
        eprintln!("Target project: {}", args.target.display());
        eprintln!("Output directory: {}", args.output_dir.display());
    }

    // Validate target path exists
    if !args.target.exists() {
        anyhow::bail!("Target path does not exist: {}", args.target.display());
    }

    if !args.target.is_dir() {
        anyhow::bail!("Target path must be a directory: {}", args.target.display());
    }

    // Generate the documentation
    let doc_content = generator::generate_documentation(&args.target, args.verbose)?;

    // Create output directory if it doesn't exist
    std::fs::create_dir_all(&args.output_dir)?;

    // Determine output file path
    let output_file = args.output_dir.join("ai_agent_instructions.md");

    if args.verbose {
        eprintln!("Writing documentation to: {}", output_file.display());
    }

    // Write the documentation
    std::fs::write(&output_file, doc_content)?;

    println!("✅ AI agent documentation generated successfully!");
    println!("📄 Output: {}", output_file.display());

    if args.verbose {
        eprintln!("Documentation includes:");
        eprintln!("  • Development process guidelines");
        eprintln!("  • Continuous improvement practices");
        eprintln!("  • Playwright MCP setup instructions");
        eprintln!("  • Quality standards and testing requirements");
    }

    Ok(())
}
