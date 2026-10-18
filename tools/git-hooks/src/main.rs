//! `git-hooks <command>` (EN-108), run from a worktree's root:
//!
//! - `install`: sets `core.hooksPath` to `.githooks` once (`npm install`'s `prepare`,
//!   `worktree:new`);
//! - `pre-commit [--tasks <file>]`: formats, lints and scans the staged files, the repository's
//!   tasks or those of the file (`staged::parse_tasks`), and stages the fixes.

use std::env;
use std::fs;
use std::process::ExitCode;

use git_hooks::install::{HOOKS_FOLDER, Installed, install};
use git_hooks::pre_commit::pre_commit;
use git_hooks::staged::{parse_tasks, repository_tasks};

/// The usage, printed on a call without a known command.
const USAGE: &str = "usage: git-hooks install | pre-commit [--tasks <file>]";

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

fn main() -> ExitCode {
    let arguments: Vec<String> = env::args().skip(1).collect();
    let command = arguments.first().map_or("", String::as_str);
    let options = arguments.get(1..).unwrap_or_default();

    tool_use::run("git-hooks", command, || match command {
        "install" => run_install(),
        "pre-commit" => run_pre_commit(options),
        _ => {
            eprintln!("{USAGE}");
            2
        }
    })
}
