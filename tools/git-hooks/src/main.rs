//! `git-hooks <command>` (EN-108), run from a worktree's root:
//!
//! - `install`: sets `core.hooksPath` to `.githooks` once (`npm install`'s `prepare`,
//!   `worktree-new`);
//! - `pre-commit [--tasks <file>]`: formats, lints and scans the staged files, the repository's
//!   tasks or those of the file (`staged::parse_tasks`), and stages the fixes;
//! - `commit-msg <file>`: checks the message as commitlint did, then, on a story branch, its
//!   `Refs:` or `Closes:` footer;
//! - `commits --from <commit> --to <commit>`: checks the message of each commit of the range, as
//!   `commitlint --from --to` did in the CI;
//! - `branch [--name <name>]`: checks the current branch's name, or the one given (the CI's);
//! - `push-deletes-only`: exits 0 when the refs of the pre-push hook's standard input only delete
//!   branches, so that the hook runs no check, 1 otherwise (`push`);
//! - `post-merge [<squash flag>]` and `post-rewrite <amend | rebase>`: after a merge, a
//!   fast-forward or a rebase, rebuilds the release binaries when a Rust input changed
//!   (`rebuild`, EN-131); always exits 0, since a post hook cannot undo the pull.

use std::env;
use std::fs;
use std::io;
use std::process::{Command, ExitCode};

use git_hooks::branch::branch_refusal;
use git_hooks::footer::footer_refusal;
use git_hooks::install::{HOOKS_FOLDER, Installed, install};
use git_hooks::message::{format_report, lint, without_retired_scopes};
use git_hooks::pre_commit::pre_commit;
use git_hooks::push::deletes_only;
use git_hooks::rebuild::{AfterPull, after_pull, message};
use git_hooks::staged::{parse_tasks, repository_tasks};
use git_repository::stdout_of;

/// The usage, printed on a call without a known command.
const USAGE: &str = "usage: git-hooks install | pre-commit [--tasks <file>] | commit-msg <file> \
                     | commits --from <commit> --to <commit> | branch [--name <name>] \
                     | push-deletes-only | post-merge [<flag>] | post-rewrite <amend | rebase>";

/// Git's comment character when `core.commentChar` is not set.
const COMMENT: &str = "#";

/// A Git command's output, `None` when it fails.
fn git(arguments: &[&str]) -> Option<String> {
    stdout_of(Command::new("git").args(arguments)).ok()
}

/// Sets the hooks folder; the exit code.
fn run_install() -> u8 {
    match env::current_dir()
        .map_err(|error| error.to_string())
        .and_then(|folder| install(&folder))
    {
        Ok(Installed::Set) => {
            println!("git-hooks: core.hooksPath set to {HOOKS_FOLDER}");
            0
        }
        Ok(Installed::AlreadySet) => 0,
        Ok(Installed::NotARepository) => {
            println!("git-hooks: not in a Git work tree, no hook installed");
            0
        }
        Err(reason) => {
            eprintln!("git-hooks install: {reason}");
            1
        }
    }
}

/// Runs the tasks on the staged files; the exit code.
fn run_pre_commit(options: &[String]) -> u8 {
    let tasks = match options {
        [] => Ok(repository_tasks()),
        [flag, file] if flag == "--tasks" => fs::read_to_string(file)
            .map_err(|error| format!("{file}: {error}"))
            .and_then(|text| parse_tasks(&text)),
        _ => Err(USAGE.to_owned()),
    };

    match (tasks, env::current_dir()) {
        (Ok(tasks), Ok(folder)) => pre_commit(&folder, &tasks),
        (Err(reason), _) => {
            eprintln!("git-hooks pre-commit: {reason}");
            2
        }
        (_, Err(error)) => {
            eprintln!("git-hooks pre-commit: {error}");
            2
        }
    }
}

/// Checks a message file as `commitlint --edit` did, the comment lines of `core.commentChar` left
/// out, then the story footer of the current branch; the exit code.
fn run_commit_msg(options: &[String]) -> u8 {
    let [file] = options else {
        eprintln!("{USAGE}");
        return 2;
    };
    let text = match fs::read_to_string(file) {
        Ok(text) => format!("{text}\n"),
        Err(error) => {
            eprintln!("git-hooks commit-msg: {file}: {error}");
            return 2;
        }
    };

    if text.trim().is_empty() {
        eprintln!("✖   the commit message is empty [input]");
        return 1;
    }

    let comment = git(&["config", "core.commentChar"])
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| COMMENT.to_owned());
    let report = lint(&text, Some(&comment));
    let printed = report.as_ref().map(format_report).unwrap_or_default();

    print!("{printed}");

    if report.is_some_and(|report| !report.errors.is_empty()) {
        return 1;
    }

    let branch = git(&["symbolic-ref", "--short", "-q", "HEAD"]).unwrap_or_default();

    match footer_refusal(&text, branch.trim()) {
        Some(refusal) => {
            println!("{refusal}");
            1
        }
        None => 0,
    }
}

/// Checks the message of each commit after `--from` up to `--to`; the exit code.
fn run_commits(options: &[String]) -> u8 {
    let [from_flag, from, to_flag, to] = options else {
        eprintln!("{USAGE}");
        return 2;
    };

    if from_flag != "--from" || to_flag != "--to" {
        eprintln!("{USAGE}");
        return 2;
    }

    // As commitlint: a range without a common ancestor is a history cut short, never a pass.
    if git(&["merge-base", from, to]).is_none() {
        eprintln!("git-hooks commits: no merge base between {from} and {to}: fetch more history");
        return 1;
    }

    let Some(log) = git(&["log", "-z", "--format=%H%n%B", &format!("{from}..{to}")]) else {
        eprintln!("git-hooks commits: git log {from}..{to} failed");
        return 1;
    };
    let mut refused = 0;

    for record in log.split('\0').filter(|record| !record.is_empty()) {
        let (hash, message) = record.split_once('\n').unwrap_or((record, ""));

        if let Some(report) = lint(&format!("{message}\n"), None).map(without_retired_scopes) {
            let printed = format_report(&report);

            if !printed.is_empty() {
                println!("{hash}\n{printed}");
            }

            refused += usize::from(!report.errors.is_empty());
        }
    }

    u8::from(refused > 0)
}

/// Checks the current branch's name, or the one given; the exit code.
fn run_branch(options: &[String]) -> u8 {
    let name = match options {
        [] => git(&["symbolic-ref", "--short", "-q", "HEAD"]).map(|name| name.trim().to_owned()),
        [flag, name] if flag == "--name" => Some(name.clone()),
        _ => {
            eprintln!("{USAGE}");
            return 2;
        }
    };
    let Some(name) = name else {
        eprintln!("✖   no branch: HEAD is detached, or this is not a Git repository");
        return 1;
    };

    match branch_refusal(&name) {
        Some(refusal) => {
            eprintln!("{refusal}");
            1
        }
        None => 0,
    }
}

/// Reads the pushed refs on standard input: 0 when they only delete branches, 1 otherwise, 2 when
/// the input cannot be read, the hook then running its checks.
fn run_push_deletes_only() -> u8 {
    match io::read_to_string(io::stdin()) {
        Ok(refs) if deletes_only(&refs) => {
            println!("git-hooks: the push only deletes branches, no check run");
            0
        }
        Ok(_) => 1,
        Err(error) => {
            eprintln!("git-hooks push-deletes-only: {error}");
            2
        }
    }
}

/// Rebuilds the release binaries after a merge, a fast-forward or a rebase, and prints what it did;
/// an amend (`post-rewrite amend`) builds nothing. Always 0.
fn run_after_pull(hook: &str, options: &[String]) -> u8 {
    let rebuilds = hook == "post-merge" || options.first().is_some_and(|kind| kind == "rebase");

    // `post-rewrite` gets the rewritten commits on its standard input: read whole, so Git never
    // writes into a closed pipe. `post-merge` gets none, its input left untouched.
    if hook == "post-rewrite" {
        let _ = io::read_to_string(io::stdin());
    }

    if let (true, Ok(root)) = (rebuilds, env::current_dir()) {
        let outcome = after_pull(&root);
        let printed = message(&outcome);

        match outcome {
            AfterPull::Failed(_) => eprint!("{printed}"),
            _ => print!("{printed}"),
        }
    }
    0
}

fn main() -> ExitCode {
    if let Some(code) = tool_use::usage(&std::env::args().skip(1).collect::<Vec<_>>(), USAGE) {
        return code;
    }
    let arguments: Vec<String> = env::args().skip(1).collect();
    let command = arguments.first().map_or("", String::as_str);
    let options = arguments.get(1..).unwrap_or_default();

    tool_use::run("git-hooks", command, || match command {
        "install" => run_install(),
        "pre-commit" => run_pre_commit(options),
        "commit-msg" => run_commit_msg(options),
        "commits" => run_commits(options),
        "branch" => run_branch(options),
        "push-deletes-only" => run_push_deletes_only(),
        "post-merge" | "post-rewrite" => run_after_pull(command, options),
        _ => {
            eprintln!("{USAGE}");
            2
        }
    })
}
