//! Tests of the `run-totals` command in a repository of its own, its transcripts under a home of
//! its own (EN-092). Every `GIT_*` variable is removed from the commands, and `HOME` points to the
//! test's folder, so that neither the real repository nor the real configuration is touched
//! (`.claude/rules/tooling.md`).

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// A command run in `dir` without the `GIT_*` variables, its home in `home`.
fn clean(program: &str, dir: &Path, home: &Path) -> Command {
    let mut command = Command::new(program);

    command.current_dir(dir).env("HOME", home);
    for (name, _) in env::vars_os() {
        if name.to_string_lossy().starts_with("GIT_") {
            command.env_remove(name);
        }
    }
    command
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env_remove("CLAUDE_CODE_SESSION_ID");
    command
}

/// A home and a fresh repository inside it.
fn setup(name: &str) -> (PathBuf, PathBuf) {
    let home = env::temp_dir().join(format!("run-totals-bin-{}-{name}", std::process::id()));

    let _ = fs::remove_dir_all(&home);
    fs::create_dir_all(home.join("repo")).expect("folders");

    let home = fs::canonicalize(&home).expect("home");
    let repo = home.join("repo");
    let init = clean("git", &repo, &home)
        .args(["init", "-q"])
        .output()
        .expect("git init");

    assert!(init.status.success());
    (home, repo)
}

fn totals(repo: &Path, home: &Path, words: &[&str]) -> Output {
    clean(env!("CARGO_BIN_EXE_run-totals"), repo, home)
        .args(words)
        .output()
        .expect("run-totals runs")
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

/// The transcripts folder Claude Code keeps for `repo`.
fn projects(home: &Path, repo: &Path) -> PathBuf {
    let name: String = repo
        .to_string_lossy()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect();

    home.join(".claude").join("projects").join(name)
}

#[test]
fn opens_counts_shows_and_ends_a_run() {
    let (home, repo) = setup("run");
    let folder = projects(&home, &repo);

    fs::create_dir_all(folder.join("s1").join("subagents")).expect("projects");
    // A request after any start: 2099. 1 + 10 + 100 + 1 000 = 1 111 tokens.
    fs::write(
        folder.join("s1").join("subagents").join("agent-a.jsonl"),
        "{\"type\":\"assistant\",\"timestamp\":\"2099-01-01T00:00:00Z\",\"message\":{\"id\":\"a\",\"usage\":{\"input_tokens\":1,\"cache_creation_input_tokens\":10,\"cache_read_input_tokens\":100,\"output_tokens\":1000}}}\n",
    )
    .expect("transcript");

    // No run yet: no status, no report, no end.
    assert_eq!(stdout(&totals(&repo, &home, &["status"])), "");
    assert!(!totals(&repo, &home, &["report"]).status.success());
    assert!(!totals(&repo, &home, &["end"]).status.success());
    // No session id: refused.
    assert!(!totals(&repo, &home, &["start"]).status.success());

    assert!(stdout(&totals(&repo, &home, &["start", "s1"])).starts_with("run opened"));
    assert!(stdout(&totals(&repo, &home, &["start", "s2"])).contains("(2 in all)"));

    let marker = fs::read_to_string(repo.join(".git/metrics/run.json")).expect("marker");

    assert!(marker.contains("\"sessions\":[\"s1\",\"s2\"]"));

    fs::write(
        repo.join(".git/metrics/usage.json"),
        "{\"sevenDay\":{\"usedPercent\":33.6,\"resetsAt\":\"x\"},\"time\":1}",
    )
    .expect("usage");
    assert_eq!(
        stdout(&totals(&repo, &home, &["status"])),
        "run 0 s · 1,111 tokens · week 34 % used"
    );
    assert!(stdout(&totals(&repo, &home, &["report"])).contains("| 0 s, open | 1,111 |"));

    // Ended now: the 2099 request falls after the end.
    let ended = stdout(&totals(&repo, &home, &["end"]));

    assert!(ended.contains("| 0 s | 0 | 0 | 0 | 0 | 0 |"), "{ended}");
    assert_eq!(stdout(&totals(&repo, &home, &["status"])), "");
    assert!(!totals(&repo, &home, &["unknown"]).status.success());
}

#[test]
fn refuses_outside_a_repository() {
    let home = env::temp_dir().join(format!("run-totals-bare-{}", std::process::id()));

    fs::create_dir_all(&home).expect("home");
    let home = fs::canonicalize(&home).expect("home");
    let output = clean(env!("CARGO_BIN_EXE_run-totals"), &home, &home)
        .env("GIT_CEILING_DIRECTORIES", home.parent().expect("parent"))
        .arg("status")
        .output()
        .expect("runs");

    assert!(!output.status.success());
}
