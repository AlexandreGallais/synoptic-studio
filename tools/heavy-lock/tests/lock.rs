//! Tests of the heavy lock (EN-047); `[F18.AC3]` marks a test proving the feature's criterion.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use heavy_lock::{
    Action, Held, LockOwner, Request, acquire_reading, command_name, create_lock, held_for,
    is_running, lock_action, read_lock, remove_lock, run_locked, short_duration, take_over_line,
    waiting_line,
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
}

impl Drop for TempLock {
    fn drop(&mut self) {
        // Already gone is as good as removed.
        let _ = fs::remove_dir_all(&self.folder);
    }
}

/// Longest a run may take in a test: a lock never freed fails the test instead of hanging it.
const TIME_LIMIT: Duration = Duration::from_secs(10);

/// Runs a command under the lock at a path, as this test process, within `TIME_LIMIT`.
///
/// Returns its exit code and what it said.
fn run(path: &Path, owner_variable: Option<String>, argv: &[String]) -> (i32, String) {
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
        let mut said = Vec::new();
        let code = run_locked(&request, &argv, &mut said).expect("lock readable");

        send.send((code, String::from_utf8_lossy(&said).into_owned()))
            .expect("test waiting");
    });

    receive
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
fn knows_running_and_ended_processes() {
    assert!(is_running(std::process::id()));
    assert!(!is_running(dead_pid()));
    // Process 1 belongs to root: signalling it is refused, yet it exists.
    assert!(is_running(1));
    assert!(!is_running(u32::MAX));
}

#[test]
fn f18_ac3_takes_a_free_lock() {
    assert_eq!(lock_action(&Held::Free, None, |_| true), Action::Take);
}

#[test]
fn f18_ac3_waits_for_a_lock_held_by_a_running_process() {
    assert_eq!(
        lock_action(&Held::By(Some(owner())), None, |_| true),
        Action::Wait
    );
}

#[test]
fn f18_ac3_takes_over_a_lock_whose_process_no_longer_exists() {
    assert_eq!(
        lock_action(&Held::By(Some(owner())), None, |_| false),
        Action::TakeOver
    );
}

#[test]
fn waits_for_a_lock_naming_no_process() {
    assert_eq!(lock_action(&Held::By(None), None, |_| false), Action::Wait);
}

#[test]
fn runs_inside_the_lock_of_the_command_that_started_it_only() {
    let held = Held::By(Some(owner()));

    assert_eq!(lock_action(&held, Some("4242"), |_| true), Action::Inside);
    assert_eq!(lock_action(&held, Some("4243"), |_| true), Action::Wait);
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
    assert_eq!(
        waiting_line(Some(&owner()), Path::new("/repo/.git/heavy.lock"), 1_125),
        "heavy lock held by process 4242 (npm run test in en-047, for 2 min 05 s): waiting\n"
    );
    // Started 2 000 s from now: no age rather than a wrong one.
    assert_eq!(
        waiting_line(Some(&owner()), Path::new("/repo/.git/heavy.lock"), 0),
        "heavy lock held by process 4242 (npm run test in en-047): waiting\n"
    );
    assert_eq!(
        waiting_line(None, Path::new("/repo/.git/heavy.lock"), 0),
        "heavy lock /repo/.git/heavy.lock held without a process id (taken by hand): waiting; remove it with rmdir once its command ended\n"
    );
    assert_eq!(
        take_over_line(&owner()),
        "heavy lock of process 4242 (npm run test) taken over: that process no longer exists\n"
    );
}

#[test]
fn creates_the_lock_once_and_removes_it_by_its_owner_only() {
    let lock = TempLock::new();

    assert!(create_lock(&lock.path, &owner()).expect("created"));
    assert!(!create_lock(&lock.path, &LockOwner { pid: 7, ..owner() }).expect("refused"));
    assert_eq!(read_lock(&lock.path), Held::By(Some(owner())));

    remove_lock(&lock.path, 7);
    assert!(lock.path.exists());

    remove_lock(&lock.path, 4242);
    assert_eq!(read_lock(&lock.path), Held::Free);
    // The owner's draft, linked as the lock, is never left behind.
    assert_eq!(fs::read_dir(&lock.folder).expect("folder").count(), 0);
}

#[test]
fn f18_ac3_waits_for_a_lock_released_right_after_it_was_read_then_takes_it() {
    let lock = TempLock::new();
    let holder = LockOwner {
        pid: std::process::id(),
        ..owner()
    };
    let request = Request {
        path: &lock.path,
        owner: LockOwner { pid: 7, ..owner() },
        owner_variable: None,
        poll: Duration::ZERO,
    };
    let mut reads = 0;
    let mut said = Vec::new();

    fs::write(&lock.path, holder.to_text()).expect("held lock");

    // The holder ends just after the first reading: the release lands between the waiter's look and
    // anything it does next.
    let action = acquire_reading(&request, &mut said, |path| {
        let held = read_lock(path);

        reads += 1;
        if reads == 1 {
            fs::remove_file(path).expect("released");
        }
        held
    })
    .expect("lock readable");
    let said = String::from_utf8_lossy(&said);

    assert_eq!(action, Action::Take);
    assert_eq!(reads, 2);
    assert_eq!(
        said,
        waiting_line(Some(&holder), &lock.path, heavy_lock::now())
    );
    assert_eq!(read_lock(&lock.path), Held::By(Some(request.owner.clone())));
}

#[test]
fn looks_again_at_once_when_another_command_takes_the_free_lock_first() {
    let lock = TempLock::new();
    let first = LockOwner {
        pid: dead_pid(),
        ..owner()
    };
    let request = Request {
        path: &lock.path,
        owner: LockOwner {
            pid: std::process::id(),
            ..owner()
        },
        owner_variable: None,
        poll: Duration::ZERO,
    };
    let mut reads = 0;
    let mut said = Vec::new();

    // Seen free, then taken by an ended process before this command creates it; that process is then seen
    // gone and its lock taken over.
    let action = acquire_reading(&request, &mut said, |path| {
        reads += 1;
        if reads == 1 {
            create_lock(path, &first).expect("taken first");
            return Held::Free;
        }
        read_lock(path)
    })
    .expect("lock readable");

    assert_eq!(action, Action::Take);
    assert_eq!(reads, 2);
    assert_eq!(String::from_utf8_lossy(&said), take_over_line(&first));
}

#[test]
fn takes_a_folder_as_a_lock_held_by_hand() {
    let lock = TempLock::new();

    fs::create_dir(&lock.path).expect("folder");

    assert_eq!(read_lock(&lock.path), Held::By(None));
    assert!(!create_lock(&lock.path, &owner()).expect("refused"));
}

#[test]
fn fails_to_create_a_lock_in_a_missing_folder() {
    let lock = TempLock::new();

    assert!(create_lock(&lock.path.join("heavy.lock"), &owner()).is_err());
}

#[test]
fn f18_ac3_runs_the_command_under_the_lock_frees_it_and_returns_its_code() {
    let lock = TempLock::new();

    assert_eq!(run(&lock.path, None, &exit_with(3)), (3, String::new()));
    assert_eq!(read_lock(&lock.path), Held::Free);
}

#[test]
fn names_the_owner_to_the_command_it_starts() {
    let lock = TempLock::new();
    let script = format!("test \"$HEAVY_LOCK_PID\" = {}", std::process::id());
    let argv = ["sh", "-c", &script].map(str::to_owned);

    assert_eq!(run(&lock.path, None, &argv).0, 0);
}

#[test]
fn returns_1_for_a_command_that_cannot_start() {
    let lock = TempLock::new();
    let (code, said) = run(&lock.path, None, &["/missing/command".to_owned()]);

    assert_eq!(code, 1);
    assert!(said.starts_with("/missing/command: "), "{said}");
    assert_eq!(read_lock(&lock.path), Held::Free);
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
        (0, take_over_line(&dead))
    );
    assert_eq!(read_lock(&lock.path), Held::Free);
}

#[test]
fn f18_ac3_waits_for_a_running_owner_says_who_once_then_runs() {
    let lock = TempLock::new();
    // This test process holds it, from another command than the one asking.
    let holder = LockOwner {
        pid: std::process::id(),
        ..owner()
    };

    fs::write(&lock.path, holder.to_text()).expect("held lock");

    let path = lock.path.clone();
    let release = thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        fs::remove_file(path).expect("released");
    });
    let (code, said) = run(&lock.path, None, &exit_with(0));

    release.join().expect("release thread");

    assert_eq!(code, 0);
    assert_eq!(said.lines().count(), 1, "{said}");
    assert!(
        said.starts_with(&format!("heavy lock held by process {}", holder.pid)),
        "{said}"
    );
}

#[test]
fn runs_inside_the_lock_of_the_command_that_started_it_and_leaves_it() {
    let lock = TempLock::new();
    let pid = std::process::id();

    fs::write(&lock.path, LockOwner { pid, ..owner() }.to_text()).expect("held lock");

    assert_eq!(run(&lock.path, Some(pid.to_string()), &exit_with(0)).0, 0);
    assert!(lock.path.exists());
}
