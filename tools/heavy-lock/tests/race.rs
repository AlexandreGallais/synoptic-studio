//! Races between commands taking the same lock (AUD-020, A4), each step played in turn: no sleep, no
//! thread. This file starts no process: a process started by another test would hold a copy of a
//! lock's open file until it runs its command, keeping that lock taken for a moment.
//! `[F18.AC3]` marks a test proving the feature's criterion.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use heavy_lock::{Attempt, Found, Held, Lock, LockOwner, attempt, open_lock, try_lock};

/// A command's owner, `pid` its process id: the lock checks no process id, any will do.
fn other(pid: u32) -> LockOwner {
    LockOwner {
        command: "npm run check:all".to_owned(),
        cwd: "/work/repo/.claude/worktrees/us-061".to_owned(),
        pid,
        start: 2_000,
    }
}

/// An owner as a heavy command writes it.
fn owner() -> LockOwner {
    LockOwner {
        command: "npm run test".to_owned(),
        cwd: "/work/repo/.claude/worktrees/en-047".to_owned(),
        pid: 4242,
        start: 1_000,
    }
}

/// A lock path in a new temporary folder of its own.
struct TempLock {
    folder: PathBuf,
    path: PathBuf,
}

impl TempLock {
    fn new() -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let folder = std::env::temp_dir().join(format!(
            "heavy-lock-race-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));

        fs::create_dir_all(&folder).expect("temporary folder");
        let path = folder.join("heavy.lock");

        Self { folder, path }
    }

    /// The files in its folder, sorted.
    fn files(&self) -> Vec<String> {
        let mut names: Vec<String> = fs::read_dir(&self.folder)
            .expect("folder")
            .map(|entry| {
                entry
                    .expect("entry")
                    .file_name()
                    .to_string_lossy()
                    .into_owned()
            })
            .collect();

        names.sort();
        names
    }
}

impl Drop for TempLock {
    fn drop(&mut self) {
        // Already gone is as good as removed.
        let _ = fs::remove_dir_all(&self.folder);
    }
}

/// The lock taken by `owner`, failing the test otherwise.
fn take(path: &Path, owner: &LockOwner) -> Lock {
    match try_lock(path, owner).expect("lock usable") {
        Attempt::Taken(lock) => lock,
        found => panic!("lock not taken: {found:?}"),
    }
}

/// What `owner` finds held, failing the test if it takes the lock.
fn held(path: &Path, owner: &LockOwner) -> Held {
    match try_lock(path, owner).expect("lock usable") {
        Attempt::Held(held) => held,
        found => panic!("lock not held: {found:?}"),
    }
}

#[test]
fn f18_ac3_two_commands_taking_over_the_same_left_lock_never_both_hold_it() {
    // AUD-020, A4: both open the lock left behind; the second takes it over first.
    let lock = TempLock::new();
    let dead = LockOwner {
        pid: 4243,
        ..owner()
    };

    fs::write(&lock.path, dead.to_text()).expect("lock left behind");

    let first = open_lock(&lock.path).expect("opened by the first");
    let second = open_lock(&lock.path).expect("opened by the second");
    let Attempt::TakenOver(taken, _) = attempt(&lock.path, second, &other(8)).expect("lock usable")
    else {
        panic!("not taken over by the second");
    };

    // The first now gets the lock of the file it opened, no longer the lock's: it looks again.
    assert!(matches!(
        attempt(&lock.path, first, &other(7)).expect("lock usable"),
        Attempt::Again
    ));
    assert_eq!(held(&lock.path, &other(7)), Held::By(Some(other(8))));
    assert_eq!(
        fs::read_to_string(&lock.path).expect("lock kept"),
        other(8).to_text()
    );
    drop(taken);
    assert!(matches!(
        try_lock(&lock.path, &other(7)).expect("lock usable"),
        Attempt::Taken(_)
    ));
}

#[test]
fn a_command_that_opened_a_lock_freed_since_looks_again() {
    let lock = TempLock::new();
    let taken = take(&lock.path, &owner());
    let opened = open_lock(&lock.path).expect("opened");

    drop(taken);

    assert!(matches!(
        attempt(&lock.path, opened, &other(7)).expect("lock usable"),
        Attempt::Again
    ));
    assert_eq!(lock.files(), Vec::<String>::new());
}

#[test]
fn a_command_that_found_the_lock_free_looks_again_when_another_takes_it_first() {
    let lock = TempLock::new();
    let _taken = take(&lock.path, &owner());

    assert!(matches!(
        attempt(&lock.path, Found::Missing, &other(7)).expect("lock usable"),
        Attempt::Again
    ));
    assert_eq!(lock.files(), ["heavy.lock"]);
}
