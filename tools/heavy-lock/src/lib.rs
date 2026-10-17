//! Lock of the heavy commands (ADR-0063, EN-047): `check:all`, `npm run test`, `golden:check` and
//! `docs:build` run one at a time across every worktree. Each takes `heavy.lock` in the Git common
//! directory and holds the kernel's advisory lock on it (`flock(2)`, `REF-MACOS-FLOCK`) for the
//! command's life; another heavy command waits and says who holds it, from the owner written in the
//! file. The kernel frees the lock when its holder ends, even after a force-quit: a file left behind
//! that no process holds is taken over at once, whatever process id it names. A heavy command started
//! by the owner (`check:all` running the tests) runs inside its lock.
//!
//! The file is only ever replaced or removed by the command holding its lock, and a command that
//! locks a file checks it is still the one at the lock's path: two commands taking over the same
//! file left behind never both run (AUD-020, A4).
//!
//! `--lock <name>` takes another lock the same way: `story:finish` takes `merge.lock`, so stories
//! merge one at a time (ADR-0065, decision 5; EN-051). Each lock names its owner to the commands it
//! starts in a variable of its own: a command under the merge lock never runs inside the heavy one.

use std::fs::{self, File, OpenOptions, TryLockError};
use std::io::{self, Read, Write};
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
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

    /// The owner written in a lock, `None` when it names no process.
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

/// A lock held by another command, as found at the lock's path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Held {
    /// A file whose lock a running process holds, with the owner it names; `None` for a file naming
    /// none, locked by another tool.
    By(Option<LockOwner>),
    /// A folder: the lock taken by hand with `mkdir`, an old habit no process holds.
    Folder,
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

/// Line saying who holds the lock, `now` in seconds since the Unix epoch; for a lock naming no
/// owner, how to remove it: `rmdir` for a folder, `rm` for a file.
#[must_use]
pub fn waiting_line(held: &Held, path: &Path, now: u64) -> String {
    let name = lock_name(path);

    match held {
        Held::Folder => format!(
            "{name} lock {} is a folder (taken by hand with mkdir): waiting; remove it with rmdir once its command ended\n",
            path.display()
        ),
        Held::By(None) => format!(
            "{name} lock {} held by a process naming none: waiting; remove it with rm once its command ended\n",
            path.display()
        ),
        Held::By(Some(owner)) => format!(
            "{name} lock held by process {} ({} in {}{}): waiting\n",
            owner.pid,
            owner.command,
            Path::new(&owner.cwd)
                .file_name()
                .map_or_else(String::new, |folder| folder.to_string_lossy().into_owned()),
            held_for(owner.start, now).map_or_else(String::new, |age| format!(", for {age}"))
        ),
    }
}

/// Line saying a lock left behind, which no process holds any more, was taken over.
#[must_use]
pub fn take_over_line(left_by: Option<&LockOwner>, path: &Path) -> String {
    match left_by {
        Some(owner) => format!(
            "{} lock of process {} ({}) taken over: that process no longer holds it\n",
            lock_name(path),
            owner.pid,
            owner.command
        ),
        None => format!(
            "{} lock {} naming no process taken over: no process holds it\n",
            lock_name(path),
            path.display()
        ),
    }
}

/// Takes the kernel's exclusive advisory lock on an open file without waiting (`File::try_lock`,
/// `flock(2)` with `LOCK_EX | LOCK_NB` on Unix, `REF-RUST-FILE-LOCK`, `REF-MACOS-FLOCK`): `true`
/// when taken, `false` when another open file holds it. The lock lasts until the file is closed, by
/// this process or by its end.
///
/// # Errors
///
/// When the system refuses the lock for another reason than its being held.
pub fn lock_exclusive(file: &File) -> io::Result<bool> {
    match file.try_lock() {
        Ok(()) => Ok(true),
        Err(TryLockError::WouldBlock) => Ok(false),
        Err(TryLockError::Error(error)) => Err(error),
    }
}

/// Whether an open file is still the one at a path.
fn is_at(file: &File, path: &Path) -> bool {
    match (file.metadata(), fs::symlink_metadata(path)) {
        (Ok(open), Ok(named)) => open.dev() == named.dev() && open.ino() == named.ino(),
        _ => false,
    }
}

/// The owner written in an open lock file, `None` when it names none or cannot be read.
fn owner_in(mut file: &File) -> Option<LockOwner> {
    let mut text = String::new();

    file.read_to_string(&mut text).ok()?;
    LockOwner::parse(&text)
}

/// A lock this command holds. Dropping it removes its file, then closes it, which frees it.
#[derive(Debug)]
pub struct Lock {
    file: File,
    path: PathBuf,
}

impl Drop for Lock {
    fn drop(&mut self) {
        // Removed while still locked, and only if still this lock's file: a command that opened it
        // meanwhile finds it gone once it gets its lock, and looks again.
        if is_at(&self.file, &self.path) {
            // Already gone is as good as removed.
            let _ = fs::remove_file(&self.path);
        }
    }
}

/// A new file holding the owner, already locked, beside the lock: it becomes the lock under its
/// name, so a waiter never reads a lock still empty or half written.
fn locked_draft(path: &Path, owner: &LockOwner) -> io::Result<(File, PathBuf)> {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let draft = path.with_extension(format!(
        "lock.{}.{}",
        owner.pid,
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        // A draft left by an ended process of the same id is written over.
        .create(true)
        .truncate(true)
        .open(&draft)?;

    let written = file
        .write_all(owner.to_text().as_bytes())
        .and_then(|()| lock_exclusive(&file));

    match written {
        Ok(true) => Ok((file, draft)),
        Ok(false) => {
            let _ = fs::remove_file(&draft);
            Err(io::Error::other("a new lock file is locked already"))
        }
        Err(error) => {
            // Gone already is as good as removed.
            let _ = fs::remove_file(&draft);
            Err(error)
        }
    }
}

/// What is at the lock's path when a command opens it.
#[derive(Debug)]
pub enum Found {
    /// Nothing: the lock is free.
    Missing,
    /// A folder, taken by hand.
    Folder,
    /// A file, opened, not yet locked.
    File(File),
}

/// Opens what is at the lock's path.
///
/// # Errors
///
/// When it cannot be opened for another reason than its absence.
pub fn open_lock(path: &Path) -> io::Result<Found> {
    match File::open(path) {
        Ok(file) if file.metadata()?.is_dir() => Ok(Found::Folder),
        Ok(file) => Ok(Found::File(file)),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(Found::Missing),
        Err(error) => Err(error),
    }
}

/// What one attempt at the lock leads to.
#[derive(Debug)]
pub enum Attempt {
    /// The lock was free: this command holds it.
    Taken(Lock),
    /// A file no process held any more, with the owner it named: this command holds it now.
    TakenOver(Lock, Option<LockOwner>),
    /// Held by another command: wait.
    Held(Held),
    /// The lock changed since it was opened: try again at once.
    Again,
}

/// One attempt at the lock from what was opened at its path.
///
/// # Errors
///
/// When the lock cannot be read or written.
pub fn attempt(path: &Path, found: Found, owner: &LockOwner) -> io::Result<Attempt> {
    let file = match found {
        Found::Folder => return Ok(Attempt::Held(Held::Folder)),
        Found::Missing => {
            let (file, draft) = locked_draft(path, owner)?;
            // Linked only if no lock exists: one command only wins.
            let linked = fs::hard_link(&draft, path);

            // Gone already is as good as removed.
            let _ = fs::remove_file(&draft);

            return match linked {
                Ok(()) => Ok(Attempt::Taken(Lock {
                    file,
                    path: path.to_owned(),
                })),
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => Ok(Attempt::Again),
                Err(error) => Err(error),
            };
        }
        Found::File(file) => file,
    };

    if !lock_exclusive(&file)? {
        return Ok(Attempt::Held(Held::By(owner_in(&file))));
    }

    // Locked, but freed and removed, or taken over, since it was opened.
    if !is_at(&file, path) {
        return Ok(Attempt::Again);
    }

    // Left behind: its holder ended without removing it. This command holds its lock, so none other
    // replaces it meanwhile.
    let left_by = owner_in(&file);
    let (draft_file, draft) = locked_draft(path, owner)?;

    if let Err(error) = fs::rename(&draft, path) {
        // Gone already is as good as removed.
        let _ = fs::remove_file(&draft);
        return Err(error);
    }

    Ok(Attempt::TakenOver(
        Lock {
            file: draft_file,
            path: path.to_owned(),
        },
        left_by,
    ))
}

/// One attempt at the lock at a path.
///
/// # Errors
///
/// When the lock cannot be read or written.
pub fn try_lock(path: &Path, owner: &LockOwner) -> io::Result<Attempt> {
    attempt(path, open_lock(path)?, owner)
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

/// How a command may run.
#[derive(Debug)]
pub enum Acquired {
    /// Inside the lock of the command that started it.
    Inside,
    /// Under its own lock.
    Taken(Lock),
}

/// Waits until the command may run, trying the lock with `try_at`, saying who holds it each time the
/// holder changes.
///
/// # Errors
///
/// When the lock cannot be read or written.
pub fn acquire_trying(
    request: &Request,
    say: &mut impl Write,
    mut try_at: impl FnMut(&Path, &LockOwner) -> io::Result<Attempt>,
) -> io::Result<Acquired> {
    let mut said = None;

    loop {
        let held = match try_at(request.path, &request.owner)? {
            Attempt::Taken(lock) => return Ok(Acquired::Taken(lock)),
            Attempt::TakenOver(lock, left_by) => {
                say.write_all(take_over_line(left_by.as_ref(), request.path).as_bytes())?;
                return Ok(Acquired::Taken(lock));
            }
            Attempt::Again => continue,
            Attempt::Held(Held::By(Some(owner)))
                if request.owner_variable.as_deref() == Some(owner.pid.to_string().as_str()) =>
            {
                return Ok(Acquired::Inside);
            }
            Attempt::Held(held) => held,
        };

        // One line per holder, not per look.
        if said.as_ref() != Some(&held) {
            say.write_all(waiting_line(&held, request.path, now()).as_bytes())?;
            said = Some(held);
        }

        thread::sleep(request.poll);
    }
}

/// Waits until the command may run, saying who holds the lock each time the holder changes.
///
/// # Errors
///
/// When the lock cannot be read or written.
pub fn acquire(request: &Request, say: &mut impl Write) -> io::Result<Acquired> {
    acquire_trying(request, say, try_lock)
}

/// Runs a heavy command under the lock: waits for it, runs the command, then frees it.
///
/// Returns the command's exit code, 1 when it could not start or was stopped by a signal. A force-quit
/// of this process frees the lock with it; its file, left behind, is taken over by the next command.
///
/// # Errors
///
/// When the lock cannot be read or written.
pub fn run_locked(request: &Request, argv: &[String], say: &mut impl Write) -> io::Result<i32> {
    let acquired = acquire(request, say)?;
    let Some((command, rest)) = argv.split_first() else {
        return Ok(1);
    };
    let mut child = Command::new(command);

    child.args(rest);

    // Inside its owner's lock, the owner stays the one named to the commands it starts.
    if matches!(acquired, Acquired::Taken(_)) {
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

    drop(acquired);

    Ok(code)
}
