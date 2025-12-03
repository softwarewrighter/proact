//! Print functions for CLI output

use std::path::Path;

/// Print completion messages
pub fn print_completion(file: &Path, dir: &Path, target: &Path, dry_run: bool) {
    let verb = if dry_run { "Would create" } else { "Created" };
    if dry_run {
        println!("DRY RUN completed - no files were created");
    } else {
        println!("AI agent documentation generated successfully!");
    }
    let paths = [
        file.to_path_buf(),
        dir.join("process.md"),
        dir.join("tools.md"),
        target.join("COPYRIGHT"),
        target.join("LICENSE"),
    ];
    for p in paths {
        println!("{verb}: {}", p.display());
    }
}

/// Print learnings action
pub fn print_learnings(dir: &Path, appended: bool, dry_run: bool) {
    let action = match (dry_run, appended) {
        (true, true) => "Would append to",
        (true, false) => "Would create",
        (false, true) => "Appended to",
        (false, false) => "Created",
    };
    println!("{action}: {}", dir.join("learnings.md").display());
}
