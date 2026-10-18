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
use std::time::Instant;

use calendar::{iso_instant, now_ms};

/// The file of the uses, in the Git common folder.
const USES_FILE: &str = "tool-uses.jsonl";

/// The worktree named for the main checkout, whose Git folder is not a linked worktree's.
pub const MAIN_CHECKOUT: &str = "main";

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
    let start_ms = now_ms();
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

/// The checkout a folder belongs to, as `git_repository::checkout_of` reads it from the files Git
/// writes: the main checkout's, or a linked worktree's, named after its Git folder; a submodule,
/// whose Git folder is its own common folder, counts as a main checkout. `None` outside a Git
/// repository, or for a `.git` file that cannot be read.
#[must_use]
pub fn checkout(folder: &Path) -> Option<Checkout> {
    let found = git_repository::checkout_of(folder)?;
    let worktree = if found.git_dir == found.common {
        MAIN_CHECKOUT.to_owned()
    } else {
        found.git_dir.file_name()?.to_string_lossy().into_owned()
    };

    Some(Checkout {
        uses_file: found.common.join("metrics").join(USES_FILE),
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
