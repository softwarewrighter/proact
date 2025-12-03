use anyhow::Result;
use clap::Parser;
use proact::{cli, generator, run};
use std::path::Path;

/// Handle learnings.md file - copy or append
fn handle_learnings(output_dir: &Path, dry_run: bool, verbose: bool) -> Result<Option<bool>> {
    let Some(content) = run::read_learnings_source() else {
        return Ok(None);
    };
    let target = output_dir.join("learnings.md");
    let exists = target.exists();
    if !dry_run {
        if exists {
            run::append_learnings(&target, &content, verbose)?;
        } else {
            run::write_learnings(&target, &content, verbose)?;
        }
    }
    Ok(Some(exists))
}

/// Print version information
fn print_version() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("Copyright: {}", env!("CARGO_PKG_AUTHORS"));
    println!("License: {}", env!("CARGO_PKG_LICENSE"));
    if let Some(repo) = option_env!("CARGO_PKG_REPOSITORY") {
        println!("Repository: {}", repo);
    }
    println!("Build Commit: {}", env!("GIT_SHORT_HASH"));
    println!("Build Time: {}", env!("BUILD_TIMESTAMP"));
    println!("Build Host: {}", env!("BUILD_HOST"));
}

fn main() -> Result<()> {
    if std::env::args().any(|arg| arg == "-V" || arg == "--version") {
        print_version();
        return Ok(());
    }

    let args = cli::Args::parse();
    let verbose = args.verbose || args.dry_run;

    if !args.target.exists() {
        anyhow::bail!("Target path does not exist: {}", args.target.display());
    }
    if !args.target.is_dir() {
        anyhow::bail!("Target path must be a directory: {}", args.target.display());
    }

    let output_dir = run::resolve_output_dir(&args);
    let doc = generator::generate_documentation(&args.target, verbose)?;
    let output_file = output_dir.join("ai_agent_instructions.md");

    run::create_output_dir(&output_dir, verbose, args.dry_run)?;
    run::write_doc_file(&output_file, &doc, verbose, args.dry_run)?;
    generator::copy_templates(&output_dir, verbose, args.dry_run)?;
    generator::generate_legal_files(&args.target, &output_dir, verbose, args.dry_run)?;

    let learnings = handle_learnings(&output_dir, args.dry_run, verbose)?;

    run::print_completion(&output_file, &output_dir, &args.target, args.dry_run);
    if let Some(appended) = learnings {
        let action = if args.dry_run {
            if appended {
                "Would append to"
            } else {
                "Would create"
            }
        } else if appended {
            "Appended to"
        } else {
            "Created"
        };
        println!("{action}: {}", output_dir.join("learnings.md").display());
    }

    Ok(())
}
