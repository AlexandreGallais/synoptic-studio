//! `git-hooks <command>` (EN-108), run from a worktree's root:
//!
//! - `install`: sets `core.hooksPath` to `.githooks` once (`npm install`'s `prepare`,
//!   `worktree:new`).

use std::env;
use std::process::ExitCode;

use git_hooks::install::{HOOKS_FOLDER, Installed, install};

/// The usage, printed on a call without a known command.
const USAGE: &str = "usage: git-hooks install";

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

fn main() -> ExitCode {
    let arguments: Vec<String> = env::args().skip(1).collect();
    let command = arguments.first().map_or("", String::as_str);

    tool_use::run("git-hooks", command, || {
        if command == "install" {
            run_install()
        } else {
            eprintln!("{USAGE}");
            2
        }
    })
}
