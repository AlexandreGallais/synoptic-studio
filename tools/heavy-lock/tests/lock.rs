//! Tests of the heavy lock (EN-047, AUD-020); `[F18.AC3]` marks a test proving the feature's
//! criterion.

use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::Duration;

use heavy_lock::{
    Acquired, Attempt, Held, Lock, LockOwner, Request, acquire_trying, command_name, held_for,
    lock_exclusive, run_locked, short_duration, take_over_line, try_lock, waiting_line,
};

/// An owner as a heavy command writes it.
fn owner() -> LockOwner {
    LockOwner {
        command: "npm run test".to_owned(),
        cwd: "/work/repo/.claude/worktrees/en-047".to_owned(),
        pid: 4242,
        start: 1_000,
    }
}

/// Another command's owner, `pid` its process id.
fn other(pid: u32) -> LockOwner {
    LockOwner {
        command: "npm run check:all".to_owned(),
        cwd: "/work/repo/.claude/worktrees/us-061".to_owned(),
        pid,
        start: 2_000,
    }
}

/// Process id of a process that has ended.
fn dead_pid() -> u32 {
    let mut child = Command::new("true").spawn().expect("true starts");

    child.wait().expect("true ends");
    child.id()
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
            "heavy-lock-{}-{}",
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

/// Longest a run may take in a test: a lock never freed fails the test instead of hanging it.
const TIME_LIMIT: Duration = Duration::from_secs(10);

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

/// Starts a command under the lock at a path, as this test process, signalling `first` at its first
/// line. Receives its exit code and what it said.
fn start(
    path: &Path,
    owner_variable: Option<String>,
    argv: &[String],
    first: Sender<()>,
) -> mpsc::Receiver<(i32, String)> {
    let (path, argv) = (path.to_owned(), argv.to_vec());
    let (send, receive) = mpsc::channel();

    thread::spawn(move || {
        let request = Request {
            path: &path,
            owner: LockOwner {
                pid: std::process::id(),
                ..owner()
            },
            owner_variable,
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

/// Runs a command under the lock at a path, as this test process, within `TIME_LIMIT`.
fn run(path: &Path, owner_variable: Option<String>, argv: &[String]) -> (i32, String) {
    start(path, owner_variable, argv, mpsc::channel().0)
        .recv_timeout(TIME_LIMIT)
        .expect("the run ends in time")
}

/// A shell command ending with a given exit code.
fn exit_with(code: i32) -> Vec<String> {
    vec!["sh".to_owned(), "-c".to_owned(), format!("exit {code}")]
}

#[test]
fn reads_the_owner_written_in_the_lock() {
    assert_eq!(LockOwner::parse(&owner().to_text()), Some(owner()));
}

#[test]
fn finds_no_owner_in_a_partial_or_foreign_lock() {
    for text in [
        "",
        "pid=",
        "pid=0",
        "pid=-3",
        "pid=abc",
        "command=x",
        "{\"pid\": 4242}",
    ] {
        assert_eq!(LockOwner::parse(text), None, "{text:?}");
    }
}

#[test]
fn keeps_a_command_on_one_line() {
    let written = LockOwner {
        command: "a\nb".to_owned(),
        ..owner()
    }
    .to_text();

    assert_eq!(
        LockOwner::parse(&written).map(|found| found.command),
        Some("a b".to_owned())
    );
}

#[test]
fn names_the_command_by_its_npm_script() {
    let argv = ["vitest".to_owned(), "run".to_owned()];

    assert_eq!(command_name(&argv, Some("test")), "npm run test");
    assert_eq!(command_name(&argv, Some("")), "vitest run");
    assert_eq!(command_name(&argv, None), "vitest run");
}

#[test]
fn writes_durations_short() {
    assert_eq!(short_duration(0), "0 s");
    assert_eq!(short_duration(59), "59 s");
    // 125 s = 2 × 60 + 5.
    assert_eq!(short_duration(125), "2 min 05 s");
    // 3 599 s = 59 × 60 + 59.
    assert_eq!(short_duration(3_599), "59 min 59 s");
    // 3 720 s = 1 × 3 600 + 2 × 60.
    assert_eq!(short_duration(3_720), "1 h 02 min");
}

#[test]
fn tells_the_age_of_a_lock_only_when_its_start_makes_sense() {
    assert_eq!(held_for(1_000, 1_125), Some("2 min 05 s".to_owned()));
    assert_eq!(held_for(1_000, 1_000), Some("0 s".to_owned()));
    // A day exactly, 86 400 s = 24 × 3 600, is still told; a second more is not.
    assert_eq!(held_for(1_000, 87_400), Some("24 h 00 min".to_owned()));
    assert_eq!(held_for(1_000, 87_401), None);
    // A start in the future, or none written (0, the Unix epoch).
    assert_eq!(held_for(1_126, 1_125), None);
    assert_eq!(held_for(0, 125), None);
}

#[test]
fn f18_ac3_names_the_process_holding_the_lock() {
    let path = Path::new("/repo/.git/heavy.lock");

    assert_eq!(
        waiting_line(&Held::By(Some(owner())), path, 1_125),
        "heavy lock held by process 4242 (npm run test in en-047, for 2 min 05 s): waiting\n"
    );
    // Started 2 000 s from now: no age rather than a wrong one.
    assert_eq!(
        waiting_line(&Held::By(Some(owner())), path, 0),
        "heavy lock held by process 4242 (npm run test in en-047): waiting\n"
    );
    assert_eq!(
        take_over_line(Some(&owner()), path),
        "heavy lock of process 4242 (npm run test) taken over: that process no longer holds it\n"
    );
    assert_eq!(
        take_over_line(None, path),
        "heavy lock /repo/.git/heavy.lock naming no process taken over: no process holds it\n"
    );
}

#[test]
fn tells_rmdir_for_a_folder_and_rm_for_a_file() {
    let path = Path::new("/repo/.git/heavy.lock");

    assert_eq!(
        waiting_line(&Held::Folder, path, 0),
        "heavy lock /repo/.git/heavy.lock is a folder (taken by hand with mkdir): waiting; remove it with rmdir once its command ended\n"
    );
    assert_eq!(
        waiting_line(&Held::By(None), path, 0),
        "heavy lock /repo/.git/heavy.lock held by a process naming none: waiting; remove it with rm once its command ended\n"
    );
}

#[test]
fn f18_ac3_takes_a_free_lock_writes_its_owner_and_removes_it_when_dropped() {
    let lock = TempLock::new();
    let taken = take(&lock.path, &owner());

    assert_eq!(
        fs::read_to_string(&lock.path).expect("lock written"),
        owner().to_text()
    );
    // The owner's draft, linked as the lock, is never left behind.
    assert_eq!(lock.files(), ["heavy.lock"]);

    drop(taken);
    assert_eq!(lock.files(), Vec::<String>::new());
}

#[test]
fn f18_ac3_waits_for_a_lock_held_by_another_command_and_names_it() {
    let lock = TempLock::new();
    let _taken = take(&lock.path, &owner());

    assert_eq!(held(&lock.path, &other(7)), Held::By(Some(owner())));
    // Still the holder's.
    assert_eq!(
        fs::read_to_string(&lock.path).expect("lock kept"),
        owner().to_text()
    );
}

#[test]
fn f18_ac3_takes_over_a_lock_left_by_an_ended_process() {
    let lock = TempLock::new();
    let dead = LockOwner {
        pid: dead_pid(),
        ..owner()
    };

    fs::write(&lock.path, dead.to_text()).expect("lock left behind");

    let Attempt::TakenOver(taken, left_by) = try_lock(&lock.path, &other(7)).expect("lock usable")
    else {
        panic!("not taken over");
    };

    assert_eq!(left_by, Some(dead));
    assert_eq!(
        fs::read_to_string(&lock.path).expect("lock written"),
        other(7).to_text()
    );
    assert_eq!(held(&lock.path, &owner()), Held::By(Some(other(7))));
    drop(taken);
    assert_eq!(lock.files(), Vec::<String>::new());
}

#[test]
fn takes_over_a_lock_naming_a_live_process_that_does_not_hold_it() {
    // A process id used again by another process: this test process is alive, yet holds no lock.
    let lock = TempLock::new();
    let reused = LockOwner {
        pid: std::process::id(),
        ..owner()
    };

    fs::write(&lock.path, reused.to_text()).expect("lock left behind");

    assert!(matches!(
        try_lock(&lock.path, &other(7)).expect("lock usable"),
        Attempt::TakenOver(_, Some(left_by)) if left_by == reused
    ));
}

#[test]
fn takes_over_an_empty_lock_file_no_process_holds() {
    let lock = TempLock::new();

    fs::write(&lock.path, "").expect("empty lock");

    assert!(matches!(
        try_lock(&lock.path, &owner()).expect("lock usable"),
        Attempt::TakenOver(_, None)
    ));
}

#[test]
fn waits_for_an_empty_lock_file_another_tool_holds() {
    let lock = TempLock::new();

    fs::write(&lock.path, "").expect("empty lock");
    let other_tool = File::open(&lock.path).expect("opened");
    assert!(lock_exclusive(&other_tool).expect("locked"));

    assert_eq!(held(&lock.path, &owner()), Held::By(None));
}

#[test]
fn waits_for_a_folder_taken_by_hand() {
    let lock = TempLock::new();

    fs::create_dir(&lock.path).expect("folder");

    assert_eq!(held(&lock.path, &owner()), Held::Folder);
    assert!(lock.path.is_dir());
}

#[test]
fn fails_to_take_a_lock_in_a_missing_folder() {
    let lock = TempLock::new();

    assert!(try_lock(&lock.path.join("heavy.lock"), &owner()).is_err());
}

#[test]
fn f18_ac3_waits_for_a_lock_released_right_after_it_was_tried_then_takes_it() {
    let lock = TempLock::new();
    let holder = take(&lock.path, &other(7));
    let mut holder = Some(holder);
    let request = Request {
        path: &lock.path,
        owner: owner(),
        owner_variable: None,
        poll: Duration::ZERO,
    };
    let mut tries = 0;
    let mut said = Vec::new();

    // The holder ends just after the first try: the release lands between the waiter's look and
    // anything it does next.
    let acquired = acquire_trying(&request, &mut said, |path, asking| {
        let found = try_lock(path, asking);

        tries += 1;
        holder = None;
        found
    })
    .expect("lock usable");

    assert!(matches!(acquired, Acquired::Taken(_)));
    assert_eq!(tries, 2);
    assert_eq!(
        String::from_utf8_lossy(&said),
        waiting_line(&Held::By(Some(other(7))), &lock.path, heavy_lock::now())
    );
    assert_eq!(
        fs::read_to_string(&lock.path).expect("lock written"),
        owner().to_text()
    );
}

#[test]
fn f18_ac3_runs_the_command_under_the_lock_frees_it_and_returns_its_code() {
    let lock = TempLock::new();

    assert_eq!(run(&lock.path, None, &exit_with(3)), (3, String::new()));
    assert_eq!(lock.files(), Vec::<String>::new());
}

#[test]
fn names_the_owner_to_the_command_it_starts() {
    let lock = TempLock::new();
    let script = format!("test \"$HEAVY_LOCK_PID\" = {}", std::process::id());
    let argv = ["sh", "-c", &script].map(str::to_owned);

    assert_eq!(run(&lock.path, None, &argv).0, 0);
}

#[test]
fn holds_the_lock_while_its_command_runs() {
    let lock = TempLock::new();
    let script = format!("test -f '{}'", lock.path.display());
    let argv = ["sh", "-c", &script].map(str::to_owned);

    assert_eq!(run(&lock.path, None, &argv).0, 0);
}

#[test]
fn returns_1_for_a_command_that_cannot_start() {
    let lock = TempLock::new();
    let (code, said) = run(&lock.path, None, &["/missing/command".to_owned()]);

    assert_eq!(code, 1);
    assert!(said.starts_with("/missing/command: "), "{said}");
    assert_eq!(lock.files(), Vec::<String>::new());
}

#[test]
fn f18_ac3_takes_over_at_once_the_lock_of_a_process_that_no_longer_exists() {
    let lock = TempLock::new();
    let dead = LockOwner {
        pid: dead_pid(),
        ..owner()
    };

    fs::write(&lock.path, dead.to_text()).expect("dead lock");

    assert_eq!(
        run(&lock.path, None, &exit_with(0)),
        (0, take_over_line(Some(&dead), &lock.path))
    );
    assert_eq!(lock.files(), Vec::<String>::new());
}

#[test]
fn f18_ac3_waits_for_a_running_owner_says_who_once_then_runs() {
    let lock = TempLock::new();
    let holder = take(&lock.path, &other(7));
    let (first, waiting) = mpsc::channel();
    let ended = start(&lock.path, None, &exit_with(0), first);

    // Freed only once the run says it waits.
    waiting.recv_timeout(TIME_LIMIT).expect("the run waits");
    drop(holder);

    let (code, said) = ended
        .recv_timeout(TIME_LIMIT)
        .expect("the run ends in time");

    assert_eq!(code, 0);
    assert_eq!(said.lines().count(), 1, "{said}");
    assert!(
        said.starts_with("heavy lock held by process 7 (npm run check:all in us-061"),
        "{said}"
    );
}

#[test]
fn runs_inside_the_lock_of_the_command_that_started_it_and_leaves_it() {
    let lock = TempLock::new();
    let pid = std::process::id();
    let _held = take(&lock.path, &LockOwner { pid, ..owner() });

    assert_eq!(run(&lock.path, Some(pid.to_string()), &exit_with(0)).0, 0);
    assert_eq!(lock.files(), ["heavy.lock"]);
}

#[test]
fn waits_inside_a_lock_another_command_holds_whatever_its_variable() {
    let lock = TempLock::new();
    let holder = take(&lock.path, &other(7));
    let (first, waiting) = mpsc::channel();
    let ended = start(&lock.path, Some("8".to_owned()), &exit_with(0), first);

    waiting.recv_timeout(TIME_LIMIT).expect("the run waits");
    drop(holder);

    assert_eq!(ended.recv_timeout(TIME_LIMIT).expect("ends").0, 0);
}
