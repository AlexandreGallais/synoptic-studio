//! Lock of the heavy commands (ADR-0063, EN-047): `check:all`, `npm run test`, `golden:check` and
//! `docs:build` run one at a time across every worktree. Each takes `heavy.lock` in the Git common
//! directory, a file holding its owner's process id; another heavy command waits and says who holds
//! it; a lock whose process no longer exists (after a force-quit) is taken over at once. A heavy
//! command started by the owner (`check:all` running the tests) runs inside its lock.
//!
//! `--lock <name>` takes another lock the same way: `story:finish` takes `merge.lock`, so stories
//! merge one at a time (ADR-0065, decision 5; EN-051). Each lock names its owner to the commands it
//! starts in a variable of its own: a command under the merge lock never runs inside the heavy one.

use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// The lock taken without `--lock`.
pub const HEAVY: &str = "heavy";

/// File of a lock in the Git common directory, `heavy.lock` for the heavy lock.
#[must_use]
pub fn lock_file(name: &str) -> String {
    format!("{name}.lock")
}

/// The lock's name from its file: `merge` for `…/merge.lock`.
#[must_use]
pub fn lock_name(path: &Path) -> String {
    path.file_stem()
        .map_or_else(String::new, |stem| stem.to_string_lossy().into_owned())
}

/// Variable naming the owner's process id to the commands it starts, `HEAVY_LOCK_PID` for the heavy
/// lock: they run inside its lock.
#[must_use]
pub fn owner_variable(name: &str) -> String {
    format!("{}_LOCK_PID", name.to_ascii_uppercase())
}

/// The lock and the command in the arguments: `--lock <name>` first chooses the lock, the heavy one
/// otherwise; `None` without a command, or for a name other than lower-case ASCII letters.
#[must_use]
pub fn lock_and_command(argv: &[String]) -> Option<(&str, &[String])> {
    let (name, command) = match argv {
        [flag, name, command @ ..] if flag == "--lock" => (name.as_str(), command),
        [flag, ..] if flag == "--lock" => return None,
        command => (HEAVY, command),
    };
    let valid = !name.is_empty() && name.bytes().all(|byte| byte.is_ascii_lowercase());

    (valid && !command.is_empty()).then_some((name, command))
}

/// `ESRCH`, "no such process", on macOS and Linux.
const NO_SUCH_PROCESS: i32 = 3;

/// Seconds in a minute.
const MINUTE: u64 = 60;

/// Seconds in an hour.
const HOUR: u64 = 60 * MINUTE;

/// Seconds in a day: longer than any heavy command.
const DAY: u64 = 24 * HOUR;

/// Who holds the lock, as written in it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LockOwner {
    /// The command it runs, e.g. `npm run check:all`.
    pub command: String,
    /// Folder it runs in: its worktree.
    pub cwd: String,
    /// Process id of the lock's owner.
    pub pid: u32,
    /// When it took the lock, seconds since the Unix epoch.
    pub start: u64,
}

impl LockOwner {
    /// The lock's text: one `key=value` line per field.
    #[must_use]
    pub fn to_text(&self) -> String {
        format!(
            "pid={}\ncommand={}\ncwd={}\nstart={}\n",
            self.pid,
            self.command.replace('\n', " "),
            self.cwd,
            self.start
        )
    }

    /// The owner written in a lock, `None` when it names no process (a partial write).
    #[must_use]
    pub fn parse(text: &str) -> Option<Self> {
        let value = |key: &str| {
            text.lines()
                .find_map(|line| line.strip_prefix(key)?.strip_prefix('='))
                .unwrap_or_default()
                .to_owned()
        };
        let pid = value("pid").parse::<u32>().ok().filter(|pid| *pid > 0)?;

        Some(Self {
            command: value("command"),
            cwd: value("cwd"),
            pid,
            start: value("start").parse().unwrap_or_default(),
        })
    }
}

/// The lock as found on disk.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Held {
    /// No lock.
    Free,
    /// A lock, with its owner when it names one; a folder (taken by hand with `mkdir`) names none.
    By(Option<LockOwner>),
}

/// What a heavy command does with the lock it finds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    /// Run inside the lock of the command that started it.
    Inside,
    /// Take a free lock.
    Take,
    /// Take over the lock of a process that no longer exists.
    TakeOver,
    /// Wait for the lock's owner to end.
    Wait,
}

/// What a heavy command does with the lock as it finds it.
///
/// `owner_variable` is `HEAVY_LOCK_PID` in its environment; `is_alive` tells whether a process exists.
pub fn lock_action(
    held: &Held,
    owner_variable: Option<&str>,
    is_alive: impl Fn(u32) -> bool,
) -> Action {
    match held {
        Held::Free => Action::Take,
        Held::By(None) => Action::Wait,
        Held::By(Some(owner)) if owner_variable == Some(owner.pid.to_string().as_str()) => {
            Action::Inside
        }
        Held::By(Some(owner)) if is_alive(owner.pid) => Action::Wait,
        Held::By(Some(_)) => Action::TakeOver,
    }
}

/// Whether a process still exists: false only when the system says there is no such process.
#[must_use]
pub fn is_running(pid: u32) -> bool {
    unsafe extern "C" {
        fn kill(pid: i32, signal: i32) -> i32;
    }

    let Ok(pid) = i32::try_from(pid) else {
        return false;
    };

    // SAFETY: signal 0 sends nothing; the call only checks that the process exists.
    if unsafe { kill(pid, 0) } == 0 {
        return true;
    }

    // `EPERM`: the process exists but belongs to another user.
    io::Error::last_os_error().raw_os_error() != Some(NO_SUCH_PROCESS)
}

/// A duration in seconds, written short: `45 s`, `2 min 05 s`, `1 h 02 min`.
#[must_use]
pub fn short_duration(seconds: u64) -> String {
    if seconds < MINUTE {
        format!("{seconds} s")
    } else if seconds < HOUR {
        format!("{} min {:02} s", seconds / MINUTE, seconds % MINUTE)
    } else {
        format!("{} h {:02} min", seconds / HOUR, seconds % HOUR / MINUTE)
    }
}

/// How long a lock taken at `start` has been held at `now`, written short; `None` when the start is
/// missing, later than now, or over a day ago, which no heavy command lasts: a clock or a lock gone
/// wrong.
#[must_use]
pub fn held_for(start: u64, now: u64) -> Option<String> {
    now.checked_sub(start)
        .filter(|age| start > 0 && *age <= DAY)
        .map(short_duration)
}

/// The command as the person typed it: `npm run check:all` from its npm script, else the command.
#[must_use]
pub fn command_name(argv: &[String], script: Option<&str>) -> String {
    match script {
        Some(script) if !script.is_empty() => format!("npm run {script}"),
        _ => argv.join(" "),
    }
}

/// Line saying who holds the lock, `now` in seconds since the Unix epoch.
#[must_use]
pub fn waiting_line(owner: Option<&LockOwner>, path: &Path, now: u64) -> String {
    match owner {
        None => format!(
            "{} lock {} held without a process id (taken by hand): waiting; remove it with rmdir once its command ended\n",
            lock_name(path),
            path.display()
        ),
        Some(owner) => format!(
            "{} lock held by process {} ({} in {}{}): waiting\n",
            lock_name(path),
            owner.pid,
            owner.command,
            Path::new(&owner.cwd)
                .file_name()
                .map_or_else(String::new, |name| name.to_string_lossy().into_owned()),
            held_for(owner.start, now).map_or_else(String::new, |age| format!(", for {age}"))
        ),
    }
}

/// Line saying a dead owner's lock was taken over.
#[must_use]
pub fn take_over_line(owner: &LockOwner, path: &Path) -> String {
    format!(
        "{} lock of process {} ({}) taken over: that process no longer exists\n",
        lock_name(path),
        owner.pid,
        owner.command
    )
}

/// The lock as found on disk.
#[must_use]
pub fn read_lock(path: &Path) -> Held {
    match fs::read_to_string(path) {
        Ok(text) => Held::By(LockOwner::parse(&text)),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Held::Free,
        Err(_) => Held::By(None),
    }
}

/// Creates the lock, only if none exists: an exclusive creation, which one command only wins.
///
/// The owner is written to a file of its own first, then linked as the lock: a waiter never reads a
/// lock still empty or half written.
///
/// # Errors
///
/// When the lock cannot be written for another reason than an existing one.
pub fn create_lock(path: &Path, owner: &LockOwner) -> io::Result<bool> {
    let draft = path.with_extension(format!("lock.{}", owner.pid));

    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&draft)?
        .write_all(owner.to_text().as_bytes())?;

    let linked = fs::hard_link(&draft, path);

    // Gone already is as good as removed.
    let _ = fs::remove_file(&draft);

    match linked {
        Ok(()) => Ok(true),
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => Ok(false),
        Err(error) => Err(error),
    }
}

/// Removes the lock if it is still held by a given process.
pub fn remove_lock(path: &Path, pid: u32) {
    if matches!(read_lock(path), Held::By(Some(owner)) if owner.pid == pid) {
        // Already gone is as good as removed.
        let _ = fs::remove_file(path);
    }
}

/// Seconds since the Unix epoch.
#[must_use]
pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_secs())
}

/// A heavy command asking for the lock.
#[derive(Clone, Debug)]
pub struct Request<'a> {
    /// File of the lock in the Git common directory.
    pub path: &'a Path,
    /// This command, as written in the lock.
    pub owner: LockOwner,
    /// The lock's owner variable in its environment, `HEAVY_LOCK_PID` for the heavy lock.
    pub owner_variable: Option<String>,
    /// Time between two looks at a held lock.
    pub poll: Duration,
}

/// What one look at the lock leads to.
enum Look {
    /// The command may run.
    Run(Action),
    /// The lock is held by this owner, `None` for a lock taken by hand: wait.
    Wait(Option<LockOwner>),
    /// Another command took the lock first: look again at once.
    Again,
}

/// One look at the lock, from one reading of it: a lock gone since that reading is seen at the next.
fn look(request: &Request, held: Held, say: &mut impl Write) -> io::Result<Look> {
    match lock_action(&held, request.owner_variable.as_deref(), is_running) {
        Action::Inside => Ok(Look::Run(Action::Inside)),
        Action::Wait => Ok(Look::Wait(match held {
            Held::By(owner) => owner,
            Held::Free => None,
        })),
        action => {
            if let (Action::TakeOver, Held::By(Some(dead))) = (action, &held) {
                say.write_all(take_over_line(dead, request.path).as_bytes())?;
                // Removed only if still the dead owner's: another waiter may have taken it over already.
                remove_lock(request.path, dead.pid);
            }

            Ok(if create_lock(request.path, &request.owner)? {
                Look::Run(Action::Take)
            } else {
                Look::Again
            })
        }
    }
}

/// Waits until the command may run, reading the lock with `read`, saying who holds it each time the
/// holder changes.
///
/// # Errors
///
/// When the lock cannot be read or written.
pub fn acquire_reading(
    request: &Request,
    say: &mut impl Write,
    mut read: impl FnMut(&Path) -> Held,
) -> io::Result<Action> {
    let mut said = None;

    loop {
        let owner = match look(request, read(request.path), say)? {
            Look::Run(action) => return Ok(action),
            Look::Again => continue,
            Look::Wait(owner) => owner,
        };

        // One line per holder, not per look.
        if said.as_ref() != Some(&owner) {
            say.write_all(waiting_line(owner.as_ref(), request.path, now()).as_bytes())?;
            said = Some(owner);
        }

        thread::sleep(request.poll);
    }
}

/// Waits until the command may run, saying who holds the lock each time the holder changes.
///
/// # Errors
///
/// When the lock cannot be read or written.
pub fn acquire(request: &Request, say: &mut impl Write) -> io::Result<Action> {
    acquire_reading(request, say, read_lock)
}

/// Runs a heavy command under the lock: waits for it, runs the command, then frees it.
///
/// Returns the command's exit code, 1 when it could not start or was stopped by a signal. A force-quit
/// of this process leaves the lock to a dead process id: the next heavy command takes it over.
///
/// # Errors
///
/// When the lock cannot be read or written.
pub fn run_locked(request: &Request, argv: &[String], say: &mut impl Write) -> io::Result<i32> {
    let action = acquire(request, say)?;
    let Some((command, rest)) = argv.split_first() else {
        return Ok(1);
    };
    let mut child = Command::new(command);

    child.args(rest);

    // Inside its owner's lock, the owner stays the one named to the commands it starts.
    if action == Action::Take {
        child.env(
            owner_variable(&lock_name(request.path)),
            request.owner.pid.to_string(),
        );
    }

    let code = match child.status() {
        Ok(status) => status.code().unwrap_or(1),
        Err(error) => {
            say.write_all(format!("{command}: {error}\n").as_bytes())?;
            1
        }
    };

    if action == Action::Take {
        remove_lock(request.path, request.owner.pid);
    }

    Ok(code)
}
