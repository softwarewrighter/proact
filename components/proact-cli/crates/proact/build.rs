use std::process::Command;

fn main() {
    let git_hash = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                String::from_utf8(o.stdout).ok()
            } else {
                None
            }
        })
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let git_short = if git_hash.len() >= 7 {
        git_hash[..7].to_string()
    } else {
        git_hash.clone()
    };

    let host = hostname::get()
        .ok()
        .and_then(|h| h.into_string().ok())
        .unwrap_or_else(|| "unknown".to_string());

    let timestamp = chrono::Local::now()
        .format("%Y-%m-%dT%H:%M:%S%:z")
        .to_string();

    for (k, v) in [
        ("GIT_HASH", &git_hash),
        ("GIT_SHORT_HASH", &git_short),
        ("BUILD_HOST", &host),
        ("BUILD_TIMESTAMP", &timestamp),
    ] {
        println!("cargo:rustc-env={k}={v}");
    }
    println!("cargo:rerun-if-changed=.git/HEAD");
}
