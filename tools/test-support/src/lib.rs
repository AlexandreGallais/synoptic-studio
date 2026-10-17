//! What the tests of the workspace share (EN-103). A test that starts `git`, or a binary that
//! runs it, starts it through [`command`]: the pre-push hook sets `GIT_DIR` before `check:all`
//! runs the tests, and a command inheriting it works on the real repository instead of the
//! test's own (a set-up wrote `core.bare=true` into the shared `.git/config`, 2026-10-17). The
//! rule binds the tests, not the binaries (`docs/tooling/commands.md`, Tests that run Git).

use std::ffi::OsStr;
use std::process::Command;

/// A command for `program` without any `GIT_*` variable of the process starting it; the caller
/// sets those it needs after.
#[must_use]
pub fn command(program: impl AsRef<OsStr>) -> Command {
    let mut command = Command::new(program);

    for (name, _) in std::env::vars_os() {
        if name.to_string_lossy().starts_with("GIT_") {
            command.env_remove(name);
        }
    }
    command
}
