//! Pre-push check (ADR-0065, decision 4; EN-050): refuses a worker's story branch whose commits
//! edit a shared file, naming each file. It reads the refs pushed on its standard input, one line
//! `<local ref> <local hash> <remote ref> <remote hash>` per ref (githooks, `pre-push`), and checks
//! the commits that no remote branch holds yet. In the story's `done` commit, a log passes when it
//! only gains the rows and cards that commit removes from the story file, both read at the commit
//! and at its parent (ADR-0091, AUD-025).

use std::io::{self, Read};
use std::process::{Command, ExitCode};

use git_repository::stdout_of;
use shared_files::backlog::{STORIES, file_of};
use shared_files::padding::padding_only;
use shared_files::rows::{Change, LOGS, only_moved_rows};
use shared_files::shared::{Commit, closes, refusal, shared_kind, worker_story};

/// The hash Git gives a deleted ref.
const DELETED: char = '0';

/// Output of a Git command.
fn git(arguments: &[&str]) -> io::Result<String> {
    stdout_of(Command::new("git").args(arguments)).map_err(io::Error::other)
}

/// A file's text in a commit's parent and in the commit; `None` for a file it adds or deletes, or
/// a root commit.
fn texts(commit: &str, path: &str) -> Option<(String, String)> {
    let before = git(&["show", &format!("{commit}^:{path}")]).ok()?;
    let after = git(&["show", &format!("{commit}:{path}")]).ok()?;

    Some((before, after))
}

/// Whether a commit changes a shared file only in the padding of its tables; false for a file it
/// adds or deletes, or a root commit.
fn padded(commit: &str, path: &str) -> bool {
    texts(commit, path).is_some_and(|(old, new)| padding_only(&old, &new))
}

/// The logs a story's `done` commit changes only by adding the rows and cards it removes from the
/// story file; none when the commit holds no story file of that id.
fn moved_logs(commit: &str, story: &str, files: &[String]) -> Vec<String> {
    let folder = format!("{STORIES}/");
    let names: Vec<String> = git(&["ls-tree", "--name-only", commit, &folder])
        .unwrap_or_default()
        .lines()
        .filter_map(|path| path.strip_prefix(&folder))
        .map(str::to_owned)
        .collect();
    let Some((story_before, story_after)) =
        file_of(&names, story).and_then(|name| texts(commit, &format!("{folder}{name}")))
    else {
        return Vec::new();
    };
    let story_change = Change {
        before: &story_before,
        after: &story_after,
    };

    files
        .iter()
        .filter(|path| {
            LOGS.iter()
                .find(|log| log.path == path.as_str())
                .zip(texts(commit, path))
                .is_some_and(|(log, (before, after))| {
                    only_moved_rows(
                        *log,
                        Change {
                            before: &before,
                            after: &after,
                        },
                        story_change,
                    )
                })
        })
        .cloned()
        .collect()
}

/// The commits of a pushed hash that no remote branch holds yet, those of a story's branch.
fn new_commits(hash: &str, story: &str) -> io::Result<Vec<Commit>> {
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
            let mut read = Commit {
                hash: commit.chars().take(8).collect(),
                message: git(&["log", "-1", "--format=%B", commit])?,
                files,
                padded: padded_files,
                moved: Vec::new(),
            };

            if closes(&read, story) {
                read.moved = moved_logs(commit, story, &read.files);
            }
            Ok(read)
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
            refusals.push_str(&refusal(branch, &story, &new_commits(hash, &story)?));
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
