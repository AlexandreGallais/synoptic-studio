//! The `heavy-lock` program run from a linked worktree (AUD-020, V2): it takes the lock of the Git
//! common directory, shared by every worktree, never one of its own worktree.
//! `[F18.AC3]` marks a test proving the feature's criterion.

use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use heavy_lock::{Attempt, LockOwner, try_lock};

/// Longest a step may take: a program never waiting, or never ending, fails the test instead of
/// hanging it.
const TIME_LIMIT: Duration = Duration::from_secs(20);

/// A new temporary folder of its own, removed at the end of the test.
struct TempFolder(PathBuf);

impl Drop for TempFolder {
    fn drop(&mut self) {
        // Already gone is as good as removed.
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// A command run with none of the variables that would tie it to this repository or to a lock.
fn clean(program: &str) -> Command {
    let mut command = test_support::command(program);

    for (name, _) in env::vars() {
        if name.ends_with("_LOCK_PID") || name == "npm_lifecycle_event" {
            command.env_remove(name);
        }
    }
    command
}

/// Runs Git in a folder, failing the test if it fails.
fn git(folder: &Path, args: &[&str]) {
    let status = clean("git")
        .args([
            "-c",
            "user.name=Test",
            "-c",
            "user.email=test@example.com",
            "-c",
            "commit.gpgsign=false",
            "-c",
            "core.hooksPath=/dev/null",
        ])
        .args(args)
        .current_dir(folder)
        .stdout(Stdio::null())
        .status()
        .expect("git starts");

    assert!(status.success(), "git {args:?}");
}

#[test]
fn f18_ac3_waits_from_a_linked_worktree_for_the_lock_of_the_common_directory() {
    let folder = TempFolder(env::temp_dir().join(format!("heavy-lock-bin-{}", std::process::id())));
    let repo = folder.0.join("repo");

    fs::create_dir_all(&repo).expect("temporary repository");
    git(&repo, &["init", "--quiet"]);
    git(
        &repo,
        &["commit", "--quiet", "--allow-empty", "-m", "first"],
    );
    git(&repo, &["worktree", "add", "--quiet", "../us-061"]);

    let path = repo.join(".git").join("heavy.lock");
    let holder = LockOwner {
        command: "npm run test".to_owned(),
        cwd: repo.join(".claude/worktrees/en-047").display().to_string(),
        pid: std::process::id(),
        start: heavy_lock::now(),
    };
    let Attempt::Taken(held) = try_lock(&path, &holder).expect("lock usable") else {
        panic!("lock not taken");
    };
    let mut program = clean(env!("CARGO_BIN_EXE_heavy-lock"))
        .arg("true")
        .current_dir(folder.0.join("us-061"))
        .stderr(Stdio::piped())
        .spawn()
        .expect("heavy-lock starts");
    let stderr = program.stderr.take().expect("stderr piped");
    let (send, receive) = mpsc::channel();

    thread::spawn(move || {
        let mut line = String::new();

        BufReader::new(stderr)
            .read_line(&mut line)
            .expect("stderr readable");
        // The test may have stopped waiting: nothing to tell then.
        let _ = send.send(line);
    });

    let first_line = receive.recv_timeout(TIME_LIMIT);

    drop(held);

    let ended = program.wait().expect("heavy-lock ends");
    let first_line = first_line.expect("heavy-lock says it waits");

    assert!(
        first_line.starts_with(&format!(
            "heavy lock held by process {} (npm run test in en-047",
            holder.pid
        )),
        "{first_line:?}"
    );
    assert!(ended.success());
    // Its own lock freed: nothing left in the common directory.
    assert!(!path.exists());
}
