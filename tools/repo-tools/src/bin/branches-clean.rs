//! `branches-clean` (`npm run branches:clean`, ADR-0038, AUD-020, EN-103), in the checkout it is
//! started from, after each merged feature pull request and each release: stale remote branches
//! pruned, local branches merged into `develop` or deleted on GitHub removed with their worktrees,
//! a story just started and a worktree a running process works in kept, then what is left reported.

use std::collections::HashSet;
use std::process::{Command, ExitCode, Stdio};

use git_repository::{Failure, stdout_of};
use repo_tools::branches_clean::{
    BranchFields, DEVELOP, Left, LocalBranch, base_of, is_in_use, read_branch, report,
    stale_branches, stale_worktrees, working_folders, worktrees_of,
};

/// The output of a Git command, its standard error shown; an error when it fails.
fn git_text(words: &[&str]) -> Result<String, String> {
    match stdout_of(
        Command::new("git")
            .args(words)
            .stdin(Stdio::null())
            .stderr(Stdio::inherit()),
    ) {
        Ok(output) => Ok(output),
        Err(failure @ Failure::NotStarted { .. }) => Err(failure.to_string()),
        Err(Failure::Exited { words, .. }) => Err(format!("{words} failed")),
    }
}

/// The non-empty lines of a Git command's output, each trimmed.
fn git(words: &[&str]) -> Result<Vec<String>, String> {
    Ok(git_text(words)?
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_owned)
        .collect())
}

/// A branch's commits beyond its base, 0 when Git cannot count them: such a branch is kept.
fn own_commits(fields: &BranchFields) -> u64 {
    Command::new("git")
        .args([
            "rev-list",
            "--count",
            &format!("{}..{}", base_of(fields), fields.name),
        ])
        .stdin(Stdio::null())
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|output| String::from_utf8_lossy(&output.stdout).trim().parse().ok())
        .unwrap_or(0)
}

/// Removes a worktree, unless a running process works in it or it holds changes not committed:
/// the reason it was kept, or `None` once removed.
fn remove_worktree(path: &str, folders: &[String]) -> Option<String> {
    if is_in_use(path, folders) {
        return Some("a running process works in it".to_owned());
    }

    let output = Command::new("git")
        .args(["worktree", "remove", path])
        .stdin(Stdio::null())
        .output();

    match output {
        Ok(output) if output.status.success() => None,
        Ok(output) => Some(
            String::from_utf8_lossy(&output.stderr)
                .trim()
                .replace('\n', " "),
        ),
        Err(error) => Some(error.to_string()),
    }
}

fn clean() -> Result<String, String> {
    git(&["fetch", "--prune", "origin"])?;
    git(&["worktree", "prune"])?;

    let merged: HashSet<String> =
        git(&["branch", "--format=%(refname:short)", "--merged", DEVELOP])?
            .into_iter()
            .collect();
    let branches: Vec<LocalBranch> = git(&[
        "for-each-ref",
        "--format=%(refname:short)|%(upstream:track)|%(upstream:short)",
        "refs/heads",
    ])?
    .iter()
    .map(|line| {
        let fields = BranchFields::of_line(line);

        read_branch(&fields, merged.contains(&fields.name), own_commits(&fields))
    })
    .collect();
    let current = git(&["branch", "--show-current"])?
        .into_iter()
        .next()
        .unwrap_or_default();
    let stale = stale_branches(&branches, &current);
    let worktrees = worktrees_of(&git_text(&["worktree", "list", "--porcelain"])?);
    // lsof exits with 1 when it cannot read some process: its output is still read.
    let folders = Command::new("lsof")
        .args(["-d", "cwd", "-Fn"])
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .map(|output| working_folders(&String::from_utf8_lossy(&output.stdout)))
        .unwrap_or_default();
    let names: Vec<String> = branches.iter().map(|branch| branch.name.clone()).collect();
    let kept: Vec<(String, String)> = stale_worktrees(&worktrees, &stale, &names)
        .into_iter()
        .filter_map(|path| remove_worktree(&path, &folders).map(|reason| (path, reason)))
        .collect();

    git(&["worktree", "prune"])?;

    // A branch still checked out in a worktree kept cannot be deleted.
    let kept_branches: Vec<&str> = worktrees
        .iter()
        .filter(|tree| kept.iter().any(|(path, _)| *path == tree.path))
        .map(|tree| tree.branch.as_str())
        .collect();
    let deleted: Vec<String> = stale
        .into_iter()
        .filter(|name| !kept_branches.contains(&name.as_str()))
        .collect();

    for name in &deleted {
        git(&["branch", "-D", name])?;
    }

    let left = Left {
        local: git(&["branch", "--format=%(refname:short)"])?,
        remote: git(&["branch", "-r", "--format=%(refname:short)"])?,
        stashes: git(&["stash", "list"])?.len(),
        worktrees: git(&["worktree", "list"])?.len(),
    };

    Ok(report(&deleted, &kept, &left))
}

fn work() -> u8 {
    match clean() {
        Ok(report) => {
            println!("{report}");
            0
        }
        Err(error) => {
            eprintln!("branches-clean: {error}");
            1
        }
    }
}

fn main() -> ExitCode {
    tool_use::run("branches-clean", "", work)
}
