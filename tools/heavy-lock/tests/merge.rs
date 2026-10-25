//! Tests of a lock other than the heavy one, the merge lock of `story-finish` (EN-051);
//! `[F18.AC7]` marks a test proving the feature's criterion.

use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

use heavy_lock::{
    Attempt, HEAVY, Held, Lock, LockOwner, Request, lock_and_command, lock_file, lock_name,
    owner_variable, run_locked, take_over_line, try_lock, waiting_line,
};

/// Longest a run may take in a test: a lock never freed fails the test instead of hanging it.
const TIME_LIMIT: Duration = Duration::from_secs(10);

/// Arguments as a command line gives them.
fn args(words: &[&str]) -> Vec<String> {
    words.iter().map(|word| (*word).to_owned()).collect()
}

/// An owner as `story-finish` writes it.
fn owner() -> LockOwner {
    LockOwner {
        command: "target/release/story-finish develop".to_owned(),
        cwd: "/work/repo/.claude/worktrees/en-051".to_owned(),
        pid: 4242,
        start: 1_000,
    }
}

/// A new temporary folder of its own, removed at the end of the test.
struct TempFolder(PathBuf);

impl TempFolder {
    fn new(name: &str) -> Self {
        let folder = std::env::temp_dir().join(format!("merge-lock-{}-{name}", std::process::id()));

        fs::create_dir_all(&folder).expect("temporary folder");
        Self(folder)
    }
}

impl Drop for TempFolder {
    fn drop(&mut self) {
        // Already gone is as good as removed.
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// The lock taken by `owner`, failing the test otherwise.
fn take(path: &Path, owner: &LockOwner) -> Lock {
    match try_lock(path, owner).expect("lock usable") {
        Attempt::Taken(lock) => lock,
        found => panic!("lock not taken: {found:?}"),
    }
}

/// What a run says, signalling its first line: the test frees the lock only once the run waits.
struct Said {
    text: Vec<u8>,
    first: Option<Sender<()>>,
}

impl Write for Said {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.text.extend_from_slice(bytes);
        if let Some(first) = self.first.take() {
            // The test may have stopped waiting: nothing to tell then.
            let _ = first.send(());
        }
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// Runs a command under the lock at a path as the process `pid`, within `TIME_LIMIT`.
fn run(path: &Path, pid: u32, argv: &[String]) -> (i32, String) {
    start(path, pid, argv, mpsc::channel().0)
        .recv_timeout(TIME_LIMIT)
        .expect("the run ends in time")
}

/// Starts a command under the lock at a path as the process `pid`, signalling `first` at its first
/// line. Receives its exit code and what it said.
fn start(path: &Path, pid: u32, argv: &[String], first: Sender<()>) -> Receiver<(i32, String)> {
    let (path, argv) = (path.to_owned(), argv.to_vec());
    let (send, receive) = mpsc::channel();

    thread::spawn(move || {
        let request = Request {
            path: &path,
            owner: LockOwner { pid, ..owner() },
            owner_variable: None,
            poll: Duration::from_millis(5),
        };
        let mut said = Said {
            text: Vec::new(),
            first: Some(first),
        };
        let code = run_locked(&request, &argv, &mut said).expect("lock usable");

        send.send((code, String::from_utf8_lossy(&said.text).into_owned()))
            .expect("test waiting");
    });

    receive
}

#[test]
fn takes_the_heavy_lock_without_a_lock_named() {
    let argv = args(&["vitest", "run"]);

    assert_eq!(lock_and_command(&argv), Some((HEAVY, &argv[..])));
}

#[test]
fn takes_the_lock_named_by_lock_before_the_command() {
    let argv = args(&[
        "--lock",
        "merge",
        "cargo",
        "run",
        "--bin",
        "story-finish",
        "feature/f18",
    ]);

    assert_eq!(lock_and_command(&argv), Some(("merge", &argv[2..])));
}

#[test]
fn refuses_a_lock_without_a_command_or_with_a_name_outside_lower_case_letters() {
    for words in [
        &[][..],
        &["--lock"][..],
        &["--lock", "merge"][..],
        &["--lock", "", "true"][..],
        &["--lock", "Merge", "true"][..],
        &["--lock", "../x", "true"][..],
        &["--lock", "merge-2", "true"][..],
    ] {
        assert_eq!(lock_and_command(&args(words)), None, "{words:?}");
    }
}

#[test]
fn names_each_lock_its_file_and_its_owner_variable() {
    assert_eq!(lock_file("merge"), "merge.lock");
    assert_eq!(lock_file(HEAVY), "heavy.lock");
    assert_eq!(lock_name(Path::new("/repo/.git/merge.lock")), "merge");
    assert_eq!(owner_variable("merge"), "MERGE_LOCK_PID");
    assert_eq!(owner_variable(HEAVY), "HEAVY_LOCK_PID");
}

#[test]
fn says_which_lock_it_waits_for() {
    let path = Path::new("/repo/.git/merge.lock");

    assert_eq!(
        waiting_line(&Held::By(Some(owner())), path, 1_065),
        "merge lock held by process 4242 (target/release/story-finish develop in en-051, for 1 min 05 s): waiting\n"
    );
    assert_eq!(
        take_over_line(Some(&owner()), path),
        "merge lock of process 4242 (target/release/story-finish develop) taken over: that process no longer holds it\n"
    );
}

#[test]
fn f18_ac7_a_second_story_finish_waits_for_the_merge_lock_of_the_first() {
    let folder = TempFolder::new("wait");
    let path = folder.0.join(lock_file("merge"));
    let holder = LockOwner {
        pid: std::process::id(),
        ..owner()
    };

    let held = take(&path, &holder);
    let (first, waiting) = mpsc::channel();
    // Another process than this one asks: the lock is not the waiter's own.
    let ended = start(&path, holder.pid + 1, &args(&["true"]), first);

    // Freed only once the second says it waits.
    waiting.recv_timeout(TIME_LIMIT).expect("the second waits");
    drop(held);

    let (code, said) = ended
        .recv_timeout(TIME_LIMIT)
        .expect("the second ends in time");

    assert_eq!(code, 0);
    assert!(
        said.starts_with(&format!("merge lock held by process {}", holder.pid)),
        "{said}"
    );
}

#[test]
fn names_its_owner_to_its_commands_in_the_merge_variable_never_the_heavy_one() {
    let folder = TempFolder::new("variable");
    let path = folder.0.join(lock_file("merge"));
    let pid = std::process::id();
    let check =
        format!("test \"$MERGE_LOCK_PID\" = {pid} && test \"${{HEAVY_LOCK_PID:-none}}\" != {pid}");

    assert_eq!(run(&path, pid, &args(&["sh", "-c", &check])).0, 0);
}

#[test]
fn a_held_merge_lock_does_not_hold_a_heavy_command() {
    let folder = TempFolder::new("apart");
    let merge = folder.0.join(lock_file("merge"));
    let pid = std::process::id();

    let _held = take(&merge, &LockOwner { pid, ..owner() });

    assert_eq!(
        run(&folder.0.join(lock_file(HEAVY)), pid + 1, &args(&["true"])),
        (0, String::new())
    );
    assert!(merge.exists());
}
