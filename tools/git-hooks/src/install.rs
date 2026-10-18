//! The hooks installed (EN-108, in place of husky 9.1.7, `REF-HUSKY-INSTALL`): `core.hooksPath` set
//! once to the tracked folder `.githooks`, by `npm install`'s `prepare` and by `worktree:new`.
//!
//! The path is relative, so Git resolves it in each worktree's own root: a linked worktree runs the
//! hooks of its own checkout without anything written in it (EN-052). husky wrote a stub per hook
//! name into the ignored `.husky/_` and put `node_modules/.bin` on the `PATH`; the hooks here are
//! tracked files that call the binaries by their path, so neither is needed.

use std::path::Path;
use std::process::Command;

use git_repository::{Failure, stdout_of};

/// The hooks folder, relative to a worktree's root.
pub const HOOKS_FOLDER: &str = ".githooks";

/// What `install` found and did.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Installed {
    /// `core.hooksPath` was set to [`HOOKS_FOLDER`].
    Set,
    /// It already named [`HOOKS_FOLDER`]: nothing written.
    AlreadySet,
    /// The folder is not inside a Git work tree (an archive, a package's tarball): nothing to do.
    NotARepository,
}

/// Output of a Git command run in `folder`, trimmed, `None` when it fails; an error when Git
/// cannot start.
fn git(folder: &Path, arguments: &[&str]) -> Result<Option<String>, String> {
    match stdout_of(Command::new("git").args(arguments).current_dir(folder)) {
        Ok(output) => Ok(Some(output.trim().to_owned())),
        Err(Failure::Exited { .. }) => Ok(None),
        Err(failure @ Failure::NotStarted { .. }) => Err(failure.to_string()),
    }
}

/// Sets `core.hooksPath` to [`HOOKS_FOLDER`] for the repository holding `folder`, unless it already
/// names it.
///
/// # Errors
///
/// Git cannot be started, or refuses to write the setting.
pub fn install(folder: &Path) -> Result<Installed, String> {
    if git(folder, &["rev-parse", "--is-inside-work-tree"])?.as_deref() != Some("true") {
        return Ok(Installed::NotARepository);
    }

    if git(folder, &["config", "--get", "core.hooksPath"])?.as_deref() == Some(HOOKS_FOLDER) {
        return Ok(Installed::AlreadySet);
    }

    git(folder, &["config", "core.hooksPath", HOOKS_FOLDER])?
        .map(|_| Installed::Set)
        .ok_or_else(|| "git config core.hooksPath: refused".to_owned())
}
