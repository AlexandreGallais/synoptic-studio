//! `SessionStart` hook (ADR-0021, EN-104): prints the project state derived from Git and the
//! backlog files, so that a new or compacted session starts from facts rather than from memory;
//! removes the files of calls left running for a day, which an interrupted or denied call leaves.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::SystemTime;

use hooks::git::{branch_of, checkout_of, git};
use hooks::session::{STALE_MS, feature_line, is_feature_file, state_text, story_rows};

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

/// Removes the running files last written over `STALE_MS` ago: a call interrupted or denied never
/// reaches the `log-tool-call` hook, which removes its file.
fn remove_stale_calls(folder: &Path, now: SystemTime) {
    for entry in fs::read_dir(folder).into_iter().flatten().flatten() {
        let age = entry
            .metadata()
            .and_then(|metadata| metadata.modified())
            .ok()
            .and_then(|modified| now.duration_since(modified).ok());

        if age.is_some_and(|age| age.as_millis() > STALE_MS) {
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
        eprintln!("session-state: {} is not in a Git repository", root.display());
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
    let features_folder = root.join("docs").join("backlog").join("features");
    let Ok(entries) = fs::read_dir(&features_folder) else {
        eprintln!(
            "session-state: {} cannot be read",
            features_folder.display()
        );
        return 1;
    };
    // Node.js lists a folder sorted by name.
    let mut files: Vec<String> = entries
        .flatten()
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|name| is_feature_file(name))
        .collect();

    files.sort();
    let features: Vec<String> = files
        .iter()
        .filter_map(|file| {
            let text = fs::read_to_string(features_folder.join(file)).unwrap_or_default();

            feature_line(file, &story_rows(&text))
        })
        .collect();

    print!("{}", state_text(&branch, changed, &last, &features));
    0
}
