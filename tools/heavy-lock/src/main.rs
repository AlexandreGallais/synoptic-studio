//! `heavy-lock [--lock <name>] <command> [arguments…]`: runs a command under a lock of the Git
//! common directory, the heavy one by default (ADR-0063, EN-047), `merge` for `story:finish`
//! (ADR-0065, EN-051; `docs/tooling/commands.md`).

use std::env;
use std::io;
use std::path::PathBuf;
use std::process::{Command, ExitCode};
use std::time::Duration;

use git_repository::{Failure, stdout_of};
use heavy_lock::{
    LockOwner, Request, command_name, lock_and_command, lock_file, now, owner_variable, run_locked,
};

/// Time between two looks at a lock held by another command: a waiting command starts at most a
/// second after the lock frees (5 s until NAM-002, ADR-0069); a look opens a file and tries its
/// lock, nothing more.
const POLL: Duration = Duration::from_secs(1);

/// The Git common directory, shared by every worktree, as Git gives it: the lock's path must be
/// the one every worktree and `GIT_DIR` set by a hook agree on (`REF-GIT-REV-PARSE`).
fn git_common_dir() -> io::Result<PathBuf> {
    match stdout_of(Command::new("git").args([
        "rev-parse",
        "--path-format=absolute",
        "--git-common-dir",
    ])) {
        Ok(output) => Ok(PathBuf::from(output.trim())),
        Err(Failure::NotStarted { error, .. }) => Err(error),
        Err(Failure::Exited { .. }) => Err(io::Error::other("not inside a Git repository")),
    }
}

/// The usage, printed on `--help` or `-h` (REV-009).
const USAGE: &str = "usage: heavy-lock [--lock <name>] <command> [arguments…]";

fn main() -> ExitCode {
    if let Some(code) = tool_use::usage(&std::env::args().skip(1).collect::<Vec<_>>(), USAGE) {
        return code;
    }
    let argv: Vec<String> = env::args().skip(1).collect();
    // What it runs: the npm script that started it (`check:all`), else the command's program.
    let script = env::var("npm_lifecycle_event").unwrap_or_default();
    let subcommand = if script.is_empty() {
        lock_and_command(&argv)
            .and_then(|(_, command)| command.first())
            .map_or_else(String::new, Clone::clone)
    } else {
        script
    };

    tool_use::run("heavy-lock", &subcommand, || lock_and_run(&argv))
}

/// Runs the command under its lock; its exit code.
fn lock_and_run(argv: &[String]) -> u8 {
    let Some((name, command)) = lock_and_command(argv) else {
        eprintln!("usage: heavy-lock [--lock <name>] <command> [arguments…]");
        return 2;
    };

    let result = git_common_dir().and_then(|common| {
        let request = Request {
            path: &common.join(lock_file(name)),
            owner: LockOwner {
                command: command_name(command, env::var("npm_lifecycle_event").ok().as_deref()),
                cwd: env::current_dir()?.display().to_string(),
                pid: std::process::id(),
                start: now(),
            },
            owner_variable: env::var(owner_variable(name)).ok(),
            poll: POLL,
        };

        run_locked(&request, command, &mut io::stderr())
    });

    match result {
        Ok(code) => u8::try_from(code).unwrap_or(1),
        Err(error) => {
            eprintln!("heavy-lock: {error}");
            1
        }
    }
}
