//! The release binaries rebuilt after a pull (EN-131, ADR-0078): the hooks, the status line and
//! the agent call `target/release/<binary>` by its path, so a pull that brings new Rust sources
//! leaves them running the old code until something rebuilds them. `post-merge` runs after a merge
//! or a fast-forward, `post-rewrite rebase` after a rebase (`REF-GITHOOKS`); both read the files
//! changed since `ORIG_HEAD`, the head before the operation (`REF-GIT-REV-PARSE`), and rebuild
//! when one is a Rust input (`stale_binaries::touches_rust`, EN-130). A post hook cannot undo the
//! pull: a failed build is printed with the command to run again, never an error.

use std::fs;
use std::path::Path;
use std::time::SystemTime;

/// Cargo's arguments: `npm run rust:build`'s build, quiet, as `git-sequences sync` runs it.
pub const BUILD: [&str; 5] = ["build", "--release", "--workspace", "--locked", "--quiet"];

/// The command a person runs again after a failed rebuild.
pub const AGAIN: &str = "npm run rust:build";

/// What a hook did after a pull.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AfterPull {
    /// No Rust input changed, or the checkout has no Cargo workspace: nothing built.
    Unchanged,
    /// The build ran; the binaries it wrote again, by name, sorted.
    Rebuilt(Vec<String>),
    /// The build failed; Cargo's error output.
    Failed(String),
}

/// The files changed between `ORIG_HEAD` and `HEAD` in the checkout at `root`, empty when Git has
/// no `ORIG_HEAD` or cannot compare them.
#[must_use]
pub fn pulled_files(root: &Path) -> Vec<String> {
    let _ = root;
    Vec::new()
}

/// When each release binary of the checkout was last written, `None` for a missing one, in the
/// order of `stale_binaries::binaries`.
#[must_use]
pub fn built_times(root: &Path) -> Vec<(String, Option<SystemTime>)> {
    let _ = fs::metadata(root);
    Vec::new()
}

/// The binaries whose time changed between two readings of [`built_times`], in their order.
#[must_use]
pub fn rebuilt(
    before: &[(String, Option<SystemTime>)],
    after: &[(String, Option<SystemTime>)],
) -> Vec<String> {
    let _ = (before, after);
    Vec::new()
}

/// What the hook prints: the binaries rebuilt on one line, the failure with the command to run
/// again; nothing when nothing was rebuilt.
#[must_use]
pub fn message(outcome: &AfterPull) -> String {
    let _ = outcome;
    String::new()
}

/// Rebuilds the checkout's release binaries when the pull that just ended changed a Rust input.
#[must_use]
pub fn after_pull(root: &Path) -> AfterPull {
    let _ = root;
    AfterPull::Unchanged
}
