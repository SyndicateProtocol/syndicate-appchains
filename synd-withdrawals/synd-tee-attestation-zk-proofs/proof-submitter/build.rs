//! Build script to get the git commit hash at compile time.
use std::process::Command;

#[allow(clippy::expect_used)]
fn main() {
    // Get the git commit hash
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .expect("Failed to execute git command");

    let git_hash =
        String::from_utf8(output.stdout).expect("Invalid UTF-8 sequence").trim().to_string();

    // Make it available as an environment variable at compile time
    println!("cargo:rustc-env=GIT_HASH={git_hash}");

    // Tell Cargo to re-run this build script if the git HEAD changes
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs/heads/");
}
