//! Each use of a tool of the workspace, one line of `.git/metrics/tool-uses.jsonl` in the main
//! checkout's Git folder, a linked worktree's uses included (EN-102, `docs/tooling/metrics.md`):
//! the tool, its subcommand, the worktree, its start, its duration and its exit code, so that
//! `npm run metrics` lists each tool's calls, failures and times.
//!
//! A tool's `main` hands its work to [`run`], which times it and appends the line. The record never
//! changes what the tool prints nor its exit code: a line that cannot be written (no Git folder, a
//! read-only folder) is dropped silently. Only a release build records, as `package.json`, the Git
//! hooks and `.claude/settings.json` run every tool: a debug build is a test's, whose calls are not
//! uses. The standard library appends the line, no crate (research note 0025, The crates).

use std::env;
use std::fmt::Write as _;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

/// The file of the uses, in the Git common folder.
const USES_FILE: &str = "tool-uses.jsonl";

/// The worktree named for the main checkout, whose Git folder is not a linked worktree's.
pub const MAIN_CHECKOUT: &str = "main";

/// Milliseconds in a day.
const DAY_MS: i64 = 86_400_000;

/// One use of a tool, as its line holds it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolUse {
    /// The binary's name: `git-sequences`, `check-handback`.
    pub tool: String,
    /// What the call asked of it (`sync`, `--check`), empty for a tool of one mode.
    pub subcommand: String,
    /// The linked worktree's name, [`MAIN_CHECKOUT`] for the main checkout.
    pub worktree: String,
    /// When it started, epoch milliseconds.
    pub start_ms: i64,
    /// How long its work took, the record left out.
    pub duration_ms: u64,
    /// The code it exited with.
    pub exit_code: u8,
}

/// Where a folder's uses are written: the uses file of its Git common folder, and its worktree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Checkout {
    /// `<common folder>/metrics/tool-uses.jsonl`.
    pub uses_file: PathBuf,
    /// The linked worktree's name, [`MAIN_CHECKOUT`] for the main checkout.
    pub worktree: String,
}

/// Runs a tool's work and records its use, in a release build only; returns the work's exit code
/// unchanged, recorded or not.
pub fn run(tool: &str, subcommand: &str, work: impl FnOnce() -> u8) -> ExitCode {
    run_or_poll(tool, subcommand, false, work)
}

/// Runs a tool's work, its use recorded as [`run`] does unless the call is a poll: a subcommand
/// another program calls on a timer, such as the status line's `run-totals status` once a second
/// during a run, whose thousands of lines would say nothing of the tool's use (EN-102).
pub fn run_or_poll(
    tool: &str,
    subcommand: &str,
    is_poll: bool,
    work: impl FnOnce() -> u8,
) -> ExitCode {
    let folder = if cfg!(debug_assertions) {
        None
    } else {
        env::current_dir().ok()
    };

    ExitCode::from(run_in(folder.as_deref(), tool, subcommand, is_poll, work))
}

/// Runs a tool's work and, given the folder it runs in, appends its use to that folder's checkout,
/// unless the call is a poll; a use that cannot be written is dropped. Returns the work's exit code.
pub fn run_in(
    folder: Option<&Path>,
    tool: &str,
    subcommand: &str,
    is_poll: bool,
    work: impl FnOnce() -> u8,
) -> u8 {
    let start_ms = epoch_ms_now();
    let clock = Instant::now();
    let exit_code = work();
    let duration_ms = u64::try_from(clock.elapsed().as_millis()).unwrap_or(u64::MAX);

    if let Some(checkout) = folder.filter(|_| !is_poll).and_then(checkout) {
        let tool_use = ToolUse {
            tool: tool.to_owned(),
            subcommand: subcommand.to_owned(),
            worktree: checkout.worktree,
            start_ms,
            duration_ms,
            exit_code,
        };
        // A use not written leaves the tool as it is: its output and its code are all that count.
        let _ = append(&checkout.uses_file, &json_line(&tool_use));
    }
    exit_code
}

/// Appends one line to a file of the uses, creating the file and its `metrics` folder, never the
/// Git folder above them. One `write` of the whole line to a file opened to append: two processes
/// appending at once never mix their lines (`O_APPEND`, each write lands at the file's end).
///
/// # Errors
///
/// The folder or the file cannot be created or written.
pub fn append(file: &Path, line: &str) -> io::Result<()> {
    if let Some(folder) = file.parent() {
        if folder.parent().is_some_and(|git| !git.is_dir()) {
            return Err(io::Error::new(io::ErrorKind::NotFound, "no Git folder"));
        }
        fs::create_dir_all(folder)?;
    }

    OpenOptions::new()
        .create(true)
        .append(true)
        .open(file)?
        .write_all(line.as_bytes())
}

/// The checkout a folder belongs to: the nearest `.git` above it, a folder for the main checkout, a
/// file `gitdir: <path>` for a linked worktree, whose `commondir` names the common folder. `None`
/// outside a Git repository, or for a `.git` file that cannot be read.
#[must_use]
pub fn checkout(folder: &Path) -> Option<Checkout> {
    let dot_git = folder
        .ancestors()
        .map(|ancestor| ancestor.join(".git"))
        .find(|candidate| candidate.exists())?;

    let (common, worktree) = if dot_git.is_dir() {
        (dot_git, MAIN_CHECKOUT.to_owned())
    } else {
        let text = fs::read_to_string(&dot_git).ok()?;
        let git_dir = dot_git.parent()?.join(text.strip_prefix("gitdir:")?.trim());
        // A submodule's Git folder has no `commondir`: it is its own common folder.
        match fs::read_to_string(git_dir.join("commondir")) {
            Ok(relative) => (
                git_dir.join(relative.trim()),
                git_dir.file_name()?.to_string_lossy().into_owned(),
            ),
            Err(_) => (git_dir, MAIN_CHECKOUT.to_owned()),
        }
    };

    Some(Checkout {
        uses_file: common.join("metrics").join(USES_FILE),
        worktree,
    })
}

/// The line of a use, JSON ended by a line feed:
/// `{"tool":"format","subcommand":"--check","worktree":"main","start":"2026-10-16T00:00:00.123Z","durationMs":41,"exitCode":0}`.
#[must_use]
pub fn json_line(tool_use: &ToolUse) -> String {
    format!(
        "{{\"tool\":{},\"subcommand\":{},\"worktree\":{},\"start\":\"{}\",\"durationMs\":{},\"exitCode\":{}}}\n",
        json_string(&tool_use.tool),
        json_string(&tool_use.subcommand),
        json_string(&tool_use.worktree),
        iso_instant(tool_use.start_ms),
        tool_use.duration_ms,
        tool_use.exit_code,
    )
}

/// A JSON string: quotes, backslashes and control characters escaped (RFC 8259, section 7).
#[must_use]
pub fn json_string(text: &str) -> String {
    let mut quoted = String::with_capacity(text.len() + 2);

    quoted.push('"');
    for character in text.chars() {
        match character {
            '"' => quoted.push_str("\\\""),
            '\\' => quoted.push_str("\\\\"),
            '\n' => quoted.push_str("\\n"),
            '\r' => quoted.push_str("\\r"),
            '\t' => quoted.push_str("\\t"),
            control if u32::from(control) < 0x20 => {
                // Writing into a `String` never fails.
                let _ = write!(quoted, "\\u{:04x}", u32::from(control));
            }
            other => quoted.push(other),
        }
    }
    quoted.push('"');
    quoted
}

/// A UTC instant written as ISO 8601 with milliseconds, `2026-10-16T00:00:00.123Z`, from epoch
/// milliseconds; before 1970 too.
#[must_use]
pub fn iso_instant(epoch_ms: i64) -> String {
    let days = epoch_ms.div_euclid(DAY_MS);
    let in_day = epoch_ms.rem_euclid(DAY_MS);
    let (year, month, day) = civil_from_days(days);

    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}.{:03}Z",
        in_day / 3_600_000,
        in_day / 60_000 % 60,
        in_day / 1000 % 60,
        in_day % 1000,
    )
}

/// The date of the proleptic Gregorian calendar a number of days after 1970-01-01 falls on, as
/// year, month and day (`REF-HINNANT-DATES`, `civil_from_days`, the inverse of `hooks`'
/// `days_from_civil`); the one copy of the workspace.
#[must_use]
pub fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let shifted = days + 719_468;
    let era = shifted.div_euclid(146_097);
    let day_of_era = shifted - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_from_march = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_from_march + 2) / 5 + 1;
    let month = if month_from_march < 10 {
        month_from_march + 3
    } else {
        month_from_march - 9
    };
    let year = year_of_era + era * 400 + i64::from(month <= 2);

    (year, month, day)
}

/// Epoch milliseconds now, 0 for a clock before 1970.
fn epoch_ms_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| i64::try_from(elapsed.as_millis()).unwrap_or(0))
}
