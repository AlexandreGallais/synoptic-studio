//! What the tests of the workspace share (EN-103). A test that starts `git`, or a binary that
//! runs it, starts it through [`command`]: the pre-push hook sets `GIT_DIR` before `check:all`
//! runs the tests, and a command inheriting it works on the real repository instead of the
//! test's own (a set-up wrote `core.bare=true` into the shared `.git/config`, 2026-10-17). The
//! rule binds the tests, not the binaries (`the records' conventions, area checks`, Tests that run Git).
//!
//! It also removes the variables naming the agent's real records, `AGENT_RECORDS_ROOT` and
//! `AGENT_KNOWLEDGE_SNAPSHOT`, which Claude Code exports to every process: a binary run in a test's
//! repository named `repo` opened `$AGENT_RECORDS_ROOT/repo.sqlite` in the real folder (AUD-051).
//! A test that needs records sets its own temporary folder after.

use std::ffi::OsStr;
use std::process::Command;

/// The variables naming the agent's real records, never passed to a test's command.
pub const RECORDS_VARIABLES: [&str; 2] = ["AGENT_RECORDS_ROOT", "AGENT_KNOWLEDGE_SNAPSHOT"];

/// A command for `program` without any `GIT_*` variable of the process starting it nor any of
/// [`RECORDS_VARIABLES`]; the caller sets those it needs after.
#[must_use]
pub fn command(program: impl AsRef<OsStr>) -> Command {
    let mut command = Command::new(program);

    for (name, _) in std::env::vars_os() {
        if name.to_string_lossy().starts_with("GIT_") {
            command.env_remove(name);
        }
    }
    for name in RECORDS_VARIABLES {
        command.env_remove(name);
    }
    command
}
