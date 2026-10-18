//! What the reports ask Git: the folder every worktree shares, and the log.

use std::path::PathBuf;
use std::process::Command;

/// What `git` prints for these arguments, run in the current folder; an error naming the command
/// when it cannot run or fails.
///
/// # Errors
///
/// `git` is missing, or exits with a code other than 0.
pub fn git(arguments: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .args(arguments)
        .output()
        .map_err(|error| format!("git: {error}"))?;

    if !output.status.success() {
        return Err(format!(
            "git {}: {}",
            arguments.first().copied().unwrap_or_default(),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// The Git folder shared by every worktree, absolute: `.git/metrics/` and the transcripts' folder
/// are read from the main checkout's, in a worktree too.
///
/// # Errors
///
/// Outside a Git repository.
pub fn common_dir() -> Result<PathBuf, String> {
    git(&["rev-parse", "--path-format=absolute", "--git-common-dir"])
        .map(|text| PathBuf::from(text.trim()))
}
