//! The release binaries rebuilt after a pull (EN-131, ADR-0078): the hooks, the status line and
//! the agent call `target/release/<binary>` by its path, so a pull that brings new Rust sources
//! leaves them running the old code until something rebuilds them. `post-merge` runs after a merge
//! or a fast-forward, `post-rewrite rebase` after a rebase (`REF-GITHOOKS`); both read the files
//! changed since `ORIG_HEAD`, the head before the operation (`REF-GIT-REV-PARSE`), and rebuild
//! when one is a Rust input (`stale_binaries::touches_rust`, EN-130). A post hook cannot undo the
//! pull: a failed build is printed with the command to run again, never an error.

use std::path::Path;
use std::process::{Command, Stdio};
use std::time::SystemTime;

use git_repository::stdout_of;
use stale_binaries::{RELEASE_BUILD, binaries, binary_written, touches_rust};

/// The command a person runs again after a failed rebuild.
pub const AGAIN: &str = "cargo build --release --workspace --locked";

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
    stdout_of(
        Command::new("git")
            .args(["diff", "--name-only", "ORIG_HEAD", "HEAD", "--"])
            .current_dir(root),
    )
    .map(|output| output.lines().map(str::to_owned).collect())
    .unwrap_or_default()
}

/// When each release binary of the checkout was last written, `None` for a missing one, in the
/// order of `stale_binaries::binaries`.
#[must_use]
pub fn built_times(root: &Path) -> Vec<(String, Option<SystemTime>)> {
    binaries(root)
        .into_iter()
        .map(|binary| {
            let time = binary_written(root, &binary);

            (binary, time)
        })
        .collect()
}

/// The binaries of the second reading of [`built_times`] written since the first, in its order:
/// a time that changed, or a binary missing before; one missing after was not built.
#[must_use]
pub fn rebuilt(
    before: &[(String, Option<SystemTime>)],
    after: &[(String, Option<SystemTime>)],
) -> Vec<String> {
    after
        .iter()
        .filter(|(binary, time)| {
            time.is_some()
                && before
                    .iter()
                    .find(|(name, _)| name == binary)
                    .is_none_or(|(_, earlier)| earlier != time)
        })
        .map(|(binary, _)| binary.clone())
        .collect()
}

/// What the hook prints: the binaries rebuilt on one line, the failure with the command to run
/// again; nothing when nothing was rebuilt.
#[must_use]
pub fn message(outcome: &AfterPull) -> String {
    match outcome {
        AfterPull::Rebuilt(binaries) if !binaries.is_empty() => format!(
            "git-hooks: release binaries rebuilt after the pull: {}\n",
            binaries.join(", ")
        ),
        AfterPull::Failed(error) => format!(
            "git-hooks: the pull is done, but the rebuild of target/release failed; run `{AGAIN}` again:\n{}\n",
            error.trim_end()
        ),
        AfterPull::Unchanged | AfterPull::Rebuilt(_) => String::new(),
    }
}

/// Rebuilds the checkout's release binaries when the pull that just ended changed a Rust input;
/// a checkout without a Cargo workspace has none.
#[must_use]
pub fn after_pull(root: &Path) -> AfterPull {
    if !root.join("Cargo.toml").is_file() || !touches_rust(&pulled_files(root)) {
        return AfterPull::Unchanged;
    }

    let before = built_times(root);
    let built = Command::new("cargo")
        .args(RELEASE_BUILD)
        .current_dir(root)
        .stdin(Stdio::null())
        .output();

    match built {
        Ok(output) if output.status.success() => {
            AfterPull::Rebuilt(rebuilt(&before, &built_times(root)))
        }
        Ok(output) => AfterPull::Failed(String::from_utf8_lossy(&output.stderr).into_owned()),
        Err(error) => AfterPull::Failed(format!("cargo: {error}")),
    }
}
