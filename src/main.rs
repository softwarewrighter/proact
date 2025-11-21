mod cli;
mod generator;
mod metadata;
mod templates;

use anyhow::Result;
use chrono::Local;
use clap::Parser;
use std::fs;
use std::path::Path;

/// Handles copying or appending learnings.md to the target directory
/// Returns Some(true) if appended, Some(false) if created new, None if skipped
fn handle_learnings_file(output_dir: &Path, dry_run: bool, verbose: bool) -> Result<Option<bool>> {
    // Path to our source learnings.md
    let source_learnings = Path::new("docs/learnings.md");

    if !source_learnings.exists() {
        if verbose {
            eprintln!("# No learnings.md file found in docs/, skipping");
        }
        return Ok(None);
    }

    let target_learnings = output_dir.join("learnings.md");
    let source_content = fs::read_to_string(source_learnings)?;

    // Check if target exists (even in dry-run, reading is OK)
    let target_exists = target_learnings.exists();

    if target_exists {
        // Will append to existing file with timestamp separator
        let timestamp = Local::now().format("%Y%m%dT%H%M%S").to_string();
        let separator = format!("\n\n---- Added {timestamp} ----\n\n");

        if verbose {
            let existing_size = fs::metadata(&target_learnings)
                .map(|m| m.len())
                .unwrap_or(0);
            eprintln!(
                "append {} (existing: {} bytes + separator + new: {} bytes)",
                target_learnings.display(),
                existing_size,
                source_content.len()
            );
        }

        if !dry_run {
            let existing_content = fs::read_to_string(&target_learnings)?;
            let combined = format!("{existing_content}{separator}{source_content}");
            fs::write(&target_learnings, combined)?;
        }
    } else {
        // Will copy as new file
        if verbose {
            eprintln!(
                "write {} ({} bytes)",
                target_learnings.display(),
                source_content.len()
            );
        }

        if !dry_run {
            fs::write(&target_learnings, source_content)?;
        }
    }

    Ok(Some(target_exists))
}

/// Print version information with copyright and license
fn print_version_info() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("{}", env!("CARGO_PKG_AUTHORS"));
    println!("License: {}", env!("CARGO_PKG_LICENSE"));
    if let Some(repo) = option_env!("CARGO_PKG_REPOSITORY") {
        println!("Repository: {}", repo);
    }
    println!(
        "Build: {} @ {} on {}",
        env!("GIT_SHORT_HASH"),
        env!("BUILD_TIMESTAMP"),
        env!("BUILD_HOST")
    );
}

/// Proact: A CLI that generates documentation for AI coding agents
///
/// This tool creates comprehensive documentation that instructs AI coding agents
/// to follow best practices, apply continuous improvement feedback, and utilize
/// tools like Playwright MCP for browser automation.
fn main() -> Result<()> {
    // Check for -V flag manually before parsing
    if std::env::args().any(|arg| arg == "-V") {
        print_version_info();
        return Ok(());
    }

    let args = cli::Args::parse();

    // Dry-run implies verbose
    let verbose = args.verbose || args.dry_run;

    // Validate target path exists
    if !args.target.exists() {
        anyhow::bail!("Target path does not exist: {}", args.target.display());
    }

    if !args.target.is_dir() {
        anyhow::bail!("Target path must be a directory: {}", args.target.display());
    }

    // Resolve output directory relative to target
    let output_dir = if args.output_dir.is_absolute() {
        args.output_dir.clone()
    } else {
        args.target.join(&args.output_dir)
    };

    if verbose {
        eprintln!("Proact v{}", env!("CARGO_PKG_VERSION"));
        eprintln!("Target project: {}", args.target.display());
        eprintln!("Output directory: {}", output_dir.display());
        if args.dry_run {
            eprintln!("Mode: DRY RUN (no files will be created)");
        }
    }

    // Generate the documentation
    let doc_content = generator::generate_documentation(&args.target, verbose)?;

    // Determine output file path
    let output_file = output_dir.join("ai_agent_instructions.md");

    // Create output directory if it doesn't exist
    if !output_dir.exists() {
        if verbose {
            eprintln!("mkdir -p {}", output_dir.display());
        }
        if !args.dry_run {
            std::fs::create_dir_all(&output_dir)?;
        }
    } else if verbose {
        eprintln!("# Directory already exists: {}", output_dir.display());
    }

    // Write the documentation
    if verbose {
        eprintln!(
            "write {} ({} bytes)",
            output_file.display(),
            doc_content.len()
        );
    }

    if !args.dry_run {
        std::fs::write(&output_file, doc_content)?;
    }

    // Copy template files (process.md, tools.md)
    generator::copy_templates(&output_dir, verbose, args.dry_run)?;

    // Generate COPYRIGHT and LICENSE files
    generator::generate_legal_files(&args.target, &output_dir, verbose, args.dry_run)?;

    // Copy or append learnings.md
    let learnings_action = handle_learnings_file(&output_dir, args.dry_run, verbose)?;

    if !args.dry_run {
        println!("✅ AI agent documentation generated successfully!");
        println!("📄 Created: {}", output_file.display());
        println!("📄 Created: {}", output_dir.join("process.md").display());
        println!("📄 Created: {}", output_dir.join("tools.md").display());
        println!("📄 Created: {}", args.target.join("COPYRIGHT").display());
        println!("📄 Created: {}", args.target.join("LICENSE").display());

        if let Some(appended) = learnings_action {
            let learnings_file = output_dir.join("learnings.md");
            if appended {
                println!("📄 Appended to: {}", learnings_file.display());
            } else {
                println!("📄 Created: {}", learnings_file.display());
            }
        }
    } else {
        println!("🔍 DRY RUN completed - no files were created");
        println!("📄 Would create: {}", output_file.display());
        println!(
            "📄 Would create: {}",
            output_dir.join("process.md").display()
        );
        println!("📄 Would create: {}", output_dir.join("tools.md").display());
        println!(
            "📄 Would create: {}",
            args.target.join("COPYRIGHT").display()
        );
        println!("📄 Would create: {}", args.target.join("LICENSE").display());
        if let Some(appended) = learnings_action {
            let learnings_file = output_dir.join("learnings.md");
            if appended {
                println!("📄 Would append to: {}", learnings_file.display());
            } else {
                println!("📄 Would create: {}", learnings_file.display());
            }
        }
    }

    if verbose {
        eprintln!("\nDocumentation includes:");
        eprintln!("  • AI agent instructions (ai_agent_instructions.md)");
        eprintln!("  • Development process guidelines (process.md)");
        eprintln!("  • Development tools reference (tools.md)");
        eprintln!("  • Copyright notice (COPYRIGHT)");
        eprintln!("  • MIT License file (LICENSE)");
        eprintln!("  • Continuous improvement practices");
        eprintln!("  • Playwright MCP setup instructions");
        eprintln!("  • Quality standards and testing requirements");
        eprintln!("  • Learnings from development issues");
    }

    Ok(())
}
