//! `heavy-lock <command> [arguments…]`: runs a heavy command under the lock of the Git common
//! directory (ADR-0063, EN-047, `docs/tooling/commands.md`).

use std::env;
use std::io;
use std::path::PathBuf;
use std::process::{Command, ExitCode};
use std::time::Duration;

use heavy_lock::{LOCK_NAME, LockOwner, OWNER_VARIABLE, Request, command_name, now, run_locked};

/// Time between two looks at a lock held by another command.
const POLL: Duration = Duration::from_secs(5);

/// The Git common directory, shared by every worktree.
fn git_common_dir() -> io::Result<PathBuf> {
    let output = Command::new("git")
        .args(["rev-parse", "--path-format=absolute", "--git-common-dir"])
        .output()?;

    if !output.status.success() {
        return Err(io::Error::other("not inside a Git repository"));
    }

    Ok(PathBuf::from(
        String::from_utf8_lossy(&output.stdout).trim(),
    ))
}

fn main() -> ExitCode {
    let argv: Vec<String> = env::args().skip(1).collect();

    if argv.is_empty() {
        eprintln!("usage: heavy-lock <command> [arguments…]");
        return ExitCode::from(2);
    }

    let result = git_common_dir().and_then(|common| {
        let request = Request {
            path: &common.join(LOCK_NAME),
            owner: LockOwner {
                command: command_name(&argv, env::var("npm_lifecycle_event").ok().as_deref()),
                cwd: env::current_dir()?.display().to_string(),
                pid: std::process::id(),
                start: now(),
            },
            owner_variable: env::var(OWNER_VARIABLE).ok(),
            poll: POLL,
        };

        run_locked(&request, &argv, &mut io::stderr())
    });

    match result {
        Ok(code) => ExitCode::from(u8::try_from(code).unwrap_or(1)),
        Err(error) => {
            eprintln!("heavy-lock: {error}");
            ExitCode::from(1)
        }
    }
}
