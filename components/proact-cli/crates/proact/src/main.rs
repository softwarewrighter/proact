//! proact - CLI tool for generating AI coding agent documentation

use anyhow::Result;
use clap::Parser;
use std::path::Path;

fn handle_learnings(dir: &Path, dry_run: bool, verbose: bool) -> Result<Option<bool>> {
    let Some(content) = run_learnings::read_learnings_source() else {
        return Ok(None);
    };
    let target = dir.join("learnings.md");
    let exists = target.exists();
    if !dry_run {
        if exists {
            run_learnings::append_learnings(&target, &content, verbose)?;
        } else {
            run_learnings::write_learnings(&target, &content, verbose)?;
        }
    }
    Ok(Some(exists))
}

fn print_version() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("Copyright: {}", env!("CARGO_PKG_AUTHORS"));
    println!("License: {}", env!("CARGO_PKG_LICENSE"));
    option_env!("CARGO_PKG_REPOSITORY").map(|r| println!("Repository: {r}"));
    println!("Build Commit: {}", env!("GIT_SHORT_HASH"));
    println!("Build Time: {}", env!("BUILD_TIMESTAMP"));
    println!("Build Host: {}", env!("BUILD_HOST"));
}

fn validate_target(target: &Path) -> Result<()> {
    if !target.exists() {
        anyhow::bail!("Target path does not exist: {}", target.display());
    }
    if !target.is_dir() {
        anyhow::bail!("Target path must be a directory: {}", target.display());
    }
    Ok(())
}

fn main() -> Result<()> {
    if std::env::args().any(|a| a == "-V" || a == "--version") {
        print_version();
        return Ok(());
    }
    let args = cli_args::Args::parse();
    let verbose = args.verbose || args.dry_run;
    validate_target(&args.target)?;
    let output_dir = run_files::resolve_output_dir(&args.target, &args.output_dir);
    let doc = cli_generator::generate_documentation(&args.target, verbose)?;
    let output_file = output_dir.join("ai_agent_instructions.md");
    run_files::create_output_dir(&output_dir, verbose, args.dry_run)?;
    run_files::write_doc_file(&output_file, &doc, verbose, args.dry_run)?;
    cli_generator::copy_templates(&output_dir, verbose, args.dry_run)?;
    cli_generator::generate_legal_files(&args.target, verbose, args.dry_run)?;
    let learnings = handle_learnings(&output_dir, args.dry_run, verbose)?;
    cli_output::print_completion(&output_file, &output_dir, &args.target, args.dry_run);
    if let Some(appended) = learnings {
        cli_output::print_learnings(&output_dir, appended, args.dry_run);
    }
    Ok(())
}
