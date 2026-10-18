//! Pre-push check (ADR-0065, decision 4; EN-050): refuses a worker's story branch whose commits
//! edit a shared file, naming each file. It reads the refs pushed on its standard input, one line
//! `<local ref> <local hash> <remote ref> <remote hash>` per ref (githooks, `pre-push`), and checks
//! the commits that no remote branch holds yet.

use std::io::{self, Read};
use std::process::{Command, ExitCode};

use shared_files::padding::padding_only;
use shared_files::shared::{Commit, refusal, shared_kind, worker_story};

/// The hash Git gives a deleted ref.
const DELETED: char = '0';

/// Output of a Git command.
fn git(arguments: &[&str]) -> io::Result<String> {
    let output = Command::new("git").args(arguments).output()?;

    if !output.status.success() {
        return Err(io::Error::other(format!(
            "git {}: {}",
            arguments.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        )));
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// Whether a commit changes a shared file only in the padding of its tables; false for a file it
/// adds or deletes, or a root commit.
fn padded(commit: &str, path: &str) -> bool {
    match (
        git(&["show", &format!("{commit}^:{path}")]),
        git(&["show", &format!("{commit}:{path}")]),
    ) {
        (Ok(old), Ok(new)) => padding_only(&old, &new),
        _ => false,
    }
}

/// The commits of a pushed hash that no remote branch holds yet.
fn new_commits(hash: &str) -> io::Result<Vec<Commit>> {
    git(&["rev-list", "--no-merges", hash, "--not", "--remotes"])?
        .lines()
        .map(|commit| {
            let files: Vec<String> =
                git(&["diff-tree", "--no-commit-id", "--name-only", "-r", commit])?
                    .lines()
                    .map(str::to_owned)
                    .collect();
            let padded_files = files
                .iter()
                .filter(|path| shared_kind(path).is_some() && padded(commit, path))
                .cloned()
                .collect();

            Ok(Commit {
                hash: commit.chars().take(8).collect(),
                message: git(&["log", "-1", "--format=%B", commit])?,
                files,
                padded: padded_files,
            })
        })
        .collect()
}

/// The refusals of every pushed ref, empty when none edits a shared file.
fn check(input: &str) -> io::Result<String> {
    let mut refusals = String::new();

    for line in input.lines() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        let [_, hash, remote_ref, _] = fields[..] else {
            continue;
        };

        if hash.chars().all(|digit| digit == DELETED) {
            continue;
        }

        let branch = remote_ref.strip_prefix("refs/heads/").unwrap_or(remote_ref);

        if let Some(story) = worker_story(branch) {
            refusals.push_str(&refusal(branch, &story, &new_commits(hash)?));
        }
    }

    Ok(refusals)
}

fn main() -> ExitCode {
    tool_use::run("refuse-shared-files", "", refuse)
}

/// Checks the pushed refs read on standard input; the exit code.
fn refuse() -> u8 {
    let mut input = String::new();

    if let Err(error) = io::stdin().read_to_string(&mut input) {
        eprintln!("refuse-shared-files: {error}");
        return 1;
    }

    match check(&input) {
        Ok(refusals) if refusals.is_empty() => 0,
        Ok(refusals) => {
            eprint!("{refusals}");
            1
        }
        Err(error) => {
            eprintln!("refuse-shared-files: {error}");
            1
        }
    }
}
