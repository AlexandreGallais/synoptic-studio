//! `SessionStart` hook (ADR-0021, EN-104): prints the project state derived from Git and the
//! backlog in the records (`knowledge sql`, EN-154), so that a new or compacted session starts
//! from facts rather than from memory; records that cannot be read leave the features out;
//! removes the files of calls left running for a day, which an interrupted or denied call leaves.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::SystemTime;

use git_repository::{branch_of, checkout_of};
use hooks::git::git;
use hooks::session::{FEATURE_STORIES, feature_line, feature_stories, is_stale, state_text};

fn main() -> ExitCode {
    tool_use::run("session-state", "", hook)
}

/// The repository the session works in: Claude Code's `CLAUDE_PROJECT_DIR`, else the checkout of
/// the current folder.
fn root() -> Option<PathBuf> {
    env::var("CLAUDE_PROJECT_DIR")
        .ok()
        .filter(|folder| !folder.is_empty())
        .map(PathBuf::from)
        .or_else(|| checkout_of(Path::new(".")).map(|checkout| checkout.top))
}

/// Removes the running files last written over a day ago (`is_stale`): a call interrupted or denied never
/// reaches the `log-tool-call` hook, which removes its file.
fn remove_stale_calls(folder: &Path, now: SystemTime) {
    for entry in fs::read_dir(folder).into_iter().flatten().flatten() {
        let age = entry
            .metadata()
            .and_then(|metadata| metadata.modified())
            .ok()
            .and_then(|modified| now.duration_since(modified).ok());

        if age.is_some_and(|age| is_stale(age.as_millis())) {
            let _ = fs::remove_file(entry.path());
        }
    }
}

/// Prints the state; exits 1 when Git or the feature files cannot be read.
fn hook() -> u8 {
    let Some(root) = root() else {
        return 1;
    };
    let Some(checkout) = checkout_of(&root) else {
        eprintln!(
            "session-state: {} is not in a Git repository",
            root.display()
        );
        return 1;
    };

    remove_stale_calls(
        &checkout.common.join("metrics").join("running"),
        SystemTime::now(),
    );

    let branch = branch_of(&checkout);
    let (Some(status), Some(last)) = (
        git(&root, &["status", "--porcelain"]),
        git(&root, &["log", "-1", "--format=%h %s"]),
    ) else {
        return 1;
    };
    let changed = status.split('\n').filter(|line| !line.is_empty()).count();
    let rows = knowledge_call::rows(&root, FEATURE_STORIES).unwrap_or_else(|reason| {
        eprintln!("session-state: the records cannot be read, the features left out: {reason}");
        Vec::new()
    });
    let features: Vec<String> = feature_stories(&rows)
        .iter()
        .filter_map(|(feature, stories)| feature_line(feature, stories))
        .collect();

    print!(
        "{}",
        state_text(
            &branch,
            changed,
            &last,
            &features,
            &stale_binaries::stale_binaries(&root)
        )
    );
    0
}
