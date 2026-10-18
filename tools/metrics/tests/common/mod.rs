//! What the tests of the binaries share: a repository and a home of their own. Every `GIT_*`
//! variable is removed from the commands (`test_support::command`), and `HOME` points to the
//! test's folder, so that neither the real repository nor the real transcripts are touched
//! (`.claude/rules/tooling.md`).

// Each test binary uses a part of these: the rest would be reported unused in it.
#![allow(dead_code)]

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// A command run in `dir` without the `GIT_*` variables, its home in `home`, the library a file
/// of the home.
pub fn clean(program: &str, dir: &Path, home: &Path) -> Command {
    let mut command = test_support::command(program);

    command
        .current_dir(dir)
        .env("HOME", home)
        .env("AGENT_LIBRARY_FILE", home.join("library.sqlite"))
        .env("GIT_CONFIG_NOSYSTEM", "1");
    command
}

/// Runs `git` in `dir`, which must succeed, its commits dated 2026-10-12T10:00:30Z.
pub fn git(dir: &Path, home: &Path, arguments: &[&str]) {
    let output = clean("git", dir, home)
        .args(arguments)
        .env("GIT_AUTHOR_DATE", "2026-10-12T10:00:30Z")
        .env("GIT_COMMITTER_DATE", "2026-10-12T10:00:30Z")
        .output()
        .expect("git runs");

    assert!(
        output.status.success(),
        "git {arguments:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

/// A home, a repository inside it with one commit, and a linked worktree of it on a story's
/// branch.
pub fn setup(name: &str) -> (PathBuf, PathBuf, PathBuf) {
    let home = env::temp_dir().join(format!("metrics-bin-{}-{name}", std::process::id()));

    let _ = fs::remove_dir_all(&home);
    fs::create_dir_all(home.join("repo")).expect("folders");

    let home = fs::canonicalize(&home).expect("home");
    let repo = home.join("repo");

    git(&repo, &home, &["init", "-q", "-b", "main"]);
    git(&repo, &home, &["config", "user.email", "test@example.com"]);
    git(&repo, &home, &["config", "user.name", "Test"]);
    git(
        &repo,
        &home,
        &["commit", "-q", "--allow-empty", "-m", "docs: first"],
    );
    git(
        &repo,
        &home,
        &["worktree", "add", "-q", "-b", "chore/en-107-x", "wt"],
    );
    (home.clone(), repo.clone(), repo.join("wt"))
}

/// The transcripts folder Claude Code keeps for the main checkout `repo`.
pub fn projects(home: &Path, repo: &Path) -> PathBuf {
    run_totals::totals::projects_dir(home, &repo.join(".git"))
}

/// A session's transcript: a prompt, then one request of 1 000 tokens read and 10 written.
pub fn write_session(home: &Path, repo: &Path) {
    let folder = projects(home, repo);
    let lines = [
        r#"{"type":"user","sessionId":"s1","timestamp":"2026-10-12T10:00:00Z","message":{"content":"go"}}"#,
        r#"{"type":"assistant","sessionId":"s1","timestamp":"2026-10-12T10:00:10Z","message":{"id":"m1","model":"claude-opus-5-5","usage":{"cache_read_input_tokens":1000,"output_tokens":10}}}"#,
        "",
    ];

    fs::create_dir_all(&folder).expect("projects");
    fs::write(folder.join("s1.jsonl"), lines.join("\n")).expect("transcript");
}

/// What a command printed.
pub fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}
