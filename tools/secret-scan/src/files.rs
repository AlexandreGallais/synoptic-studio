//! Which files the scan reads, as `secretlint` 13.0.7 chose them when `lint:secrets` and the
//! pre-commit hook passed it paths (`REF-SECRETLINT`): the paths given, the staged ones or the
//! tracked ones, less those it ignores — any under a `.git` or `node_modules` folder, the ignore
//! file itself, and those that the ignore file or a folder's `.gitignore` names, read with Git's
//! own rules by one `git check-ignore`. A path given that is a symbolic link is read through it,
//! as secretlint reads a path given; a path that no longer exists is skipped.

use std::collections::HashSet;
use std::io::Write as _;
use std::process::{Command, Stdio};

/// The scan's ignore file at the repository's root, `.gitignore`'s syntax.
pub const IGNORE_FILE: &str = ".secret-scan-ignore";

/// The paths a Git command prints, separated by NUL (`-z`).
///
/// # Errors
///
/// Git cannot start or fails, its message.
pub fn git_paths(arguments: &[&str]) -> Result<Vec<String>, String> {
    let output = Command::new("git")
        .args(arguments)
        .output()
        .map_err(|error| format!("git {}: {error}", arguments.join(" ")))?;

    if !output.status.success() {
        return Err(format!(
            "git {}: {}",
            arguments.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(split_nul(&output.stdout))
}

/// The paths of a NUL-separated output, the empty ones dropped.
#[must_use]
pub fn split_nul(output: &[u8]) -> Vec<String> {
    output
        .split(|byte| *byte == 0)
        .filter(|path| !path.is_empty())
        .map(|path| String::from_utf8_lossy(path).into_owned())
        .collect()
}

/// The tracked files, `git ls-files`.
///
/// # Errors
///
/// Git fails.
pub fn tracked() -> Result<Vec<String>, String> {
    git_paths(&["ls-files", "-z"])
}

/// The staged files that exist after the commit: added, copied, modified or renamed, as
/// `lint-staged` lists them (`--diff-filter=ACMR`).
///
/// # Errors
///
/// Git fails.
pub fn staged() -> Result<Vec<String>, String> {
    git_paths(&[
        "diff",
        "--cached",
        "--name-only",
        "--diff-filter=ACMR",
        "-z",
    ])
}

/// Whether a path is always ignored: under `.git` or `node_modules`, or the ignore file.
#[must_use]
pub fn is_built_in_ignore(path: &str) -> bool {
    let mut parts = path.split('/');

    parts.any(|part| part == ".git" || part == "node_modules")
        || path
            .rsplit('/')
            .next()
            .is_some_and(|name| name.starts_with(IGNORE_FILE))
}

/// The paths the ignore file or a `.gitignore` names, Git's rules applied to every path given,
/// tracked or not (`--no-index`).
///
/// # Errors
///
/// Git cannot start or fails otherwise than by finding none.
pub fn ignored_by_git(paths: &[String]) -> Result<Vec<String>, String> {
    if paths.is_empty() {
        return Ok(Vec::new());
    }
    let excludes = format!("core.excludesFile={IGNORE_FILE}");
    let mut child = Command::new("git")
        .args([
            "-c",
            &excludes,
            "check-ignore",
            "--no-index",
            "-z",
            "--stdin",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| format!("git check-ignore: {error}"))?;
    let input: Vec<u8> = paths
        .iter()
        .flat_map(|path| path.bytes().chain(std::iter::once(0)))
        .collect();

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(&input)
            .map_err(|error| format!("git check-ignore: {error}"))?;
    }
    let output = child
        .wait_with_output()
        .map_err(|error| format!("git check-ignore: {error}"))?;

    // 0: some are ignored; 1: none is.
    match output.status.code() {
        Some(0 | 1) => Ok(split_nul(&output.stdout)),
        _ => Err(format!(
            "git check-ignore: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        )),
    }
}

/// The paths to read: those given, less the ignored ones, in their order.
///
/// # Errors
///
/// Git fails.
pub fn to_scan(paths: Vec<String>) -> Result<Vec<String>, String> {
    let candidates: Vec<String> = paths
        .into_iter()
        .filter(|path| !is_built_in_ignore(path))
        .collect();
    let ignored: HashSet<String> = ignored_by_git(&candidates)?.into_iter().collect();

    Ok(candidates
        .into_iter()
        .filter(|path| !ignored.contains(path))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn built_in_ignores_are_git_node_modules_and_the_ignore_file() {
        assert!(is_built_in_ignore(".git/config"));
        assert!(is_built_in_ignore("a/node_modules/b.js"));
        assert!(is_built_in_ignore(".secret-scan-ignore"));
        assert!(is_built_in_ignore("d/.secret-scan-ignore.old"));
        assert!(!is_built_in_ignore("docs/.gitignore"));
        assert!(!is_built_in_ignore("node_modules.md"));
    }

    #[test]
    fn nul_separated_paths_are_split() {
        assert_eq!(split_nul(b"a\0b c\0\0"), ["a", "b c"]);
        assert_eq!(split_nul(b""), Vec::<String>::new());
    }
}
