//! Tests of the `session-report` command in a repository of its own, its transcripts under a home
//! of its own (EN-107). Every `GIT_*` variable is removed from the commands, and `HOME` points to
//! the test's folder, so that neither the real repository nor the real transcripts are touched
//! (`.claude/rules/tooling.md`, `test_support::command`).

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// A command run in `dir` without the `GIT_*` variables, its home in `home`.
fn clean(program: &str, dir: &Path, home: &Path) -> Command {
    let mut command = test_support::command(program);

    command
        .current_dir(dir)
        .env("HOME", home)
        .env("GIT_CONFIG_NOSYSTEM", "1");
    command
}

/// Runs `git` in `dir`, which must succeed.
fn git(dir: &Path, home: &Path, arguments: &[&str]) {
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

/// A home, a repository inside it with one commit, and a linked worktree of it.
fn setup(name: &str) -> (PathBuf, PathBuf, PathBuf) {
    let home = env::temp_dir().join(format!("metrics-session-{}-{name}", std::process::id()));

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
fn projects(home: &Path, repo: &Path) -> PathBuf {
    let name: String = repo
        .to_string_lossy()
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character
            } else {
                '-'
            }
        })
        .collect();

    home.join(".claude").join("projects").join(name)
}

/// A session's transcript: a prompt, then one request of 1 000 tokens read and 10 written.
fn write_session(home: &Path, repo: &Path) {
    let folder = projects(home, repo);
    let lines = [
        r#"{"type":"user","sessionId":"s1","timestamp":"2026-10-12T10:00:00Z","message":{"content":"go"}}"#,
        r#"{"type":"assistant","sessionId":"s1","timestamp":"2026-10-12T10:00:10Z","message":{"id":"m1","model":"claude-opus-5-5","usage":{"cache_read_input_tokens":1000,"output_tokens":10}}}"#,
        "",
    ];

    fs::create_dir_all(&folder).expect("projects");
    fs::write(folder.join("s1.jsonl"), lines.join("\n")).expect("transcript");
}

fn report(dir: &Path, home: &Path, arguments: &[&str]) -> Output {
    clean(env!("CARGO_BIN_EXE_session-report"), dir, home)
        .args(arguments)
        .output()
        .expect("session-report runs")
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[test]
fn reads_the_main_checkouts_transcripts_from_a_worktree_and_saves_the_turn() {
    let (home, repo, worktree) = setup("worktree");

    write_session(&home, &repo);
    let output = report(&worktree, &home, &["--markdown"]);

    // One request at 10:00:10, the turn from the prompt: 1 010 tokens, the commit at 10:00:30.
    assert!(output.status.success(), "{output:?}");
    assert_eq!(
        stdout(&output).lines().nth(2),
        Some("| 0 ms (active 0 ms) | 1,010 | opus-5-5 1,010 | 10 | 0 (0 failed) | 1 |")
    );
    assert!(stdout(&output).contains("docs: first`"));

    let turns = fs::read_to_string(repo.join(".git/metrics/turns.jsonl")).expect("turn saved");

    // 2026-10-12T10:00:10Z: 20 738 days, 10 h and 10 s.
    assert_eq!(
        turns,
        "{\"activeMs\":0,\"calls\":0,\"commits\":1,\"failures\":0,\"outputTokens\":10,\"session\":\"s1\",\"start\":1791799210000,\"tokens\":1010,\"wallMs\":0}\n"
    );
    assert!(repo.join(".git/metrics/transcripts-cache.jsonl").is_file());

    // With --all the session is reported, not saved as a turn.
    let all = report(&repo, &home, &["s1", "--all"]);

    assert!(stdout(&all).contains("Session report"));
    assert_eq!(
        fs::read_to_string(repo.join(".git/metrics/turns.jsonl"))
            .expect("turns")
            .lines()
            .count(),
        1
    );
    let _ = fs::remove_dir_all(&home);
}

#[test]
fn reports_an_empty_session_when_there_is_no_transcript() {
    let (home, repo, _) = setup("none");
    let output = report(&repo, &home, &["--markdown"]);

    assert!(output.status.success(), "{output:?}");
    assert!(stdout(&output).contains("| 0 ms (active 0 ms) | 0 | none | 0 | 0 (0 failed) |"));
    assert!(!repo.join(".git/metrics/turns.jsonl").exists());
    let _ = fs::remove_dir_all(&home);
}

#[test]
fn gives_the_runs_totals_with_run() {
    let (home, repo, _) = setup("run");
    let none = report(&repo, &home, &["--run"]);

    assert_eq!(none.status.code(), Some(1));
    assert_eq!(
        String::from_utf8_lossy(&none.stderr),
        "run-totals report: no run recorded\n"
    );

    write_session(&home, &repo);
    fs::create_dir_all(repo.join(".git/metrics")).expect("metrics");
    // A run from 10:00:00 to 10:01:00 of session s1.
    fs::write(
        repo.join(".git/metrics/run.json"),
        "{\"start\":1791799200000,\"end\":1791799260000,\"sessions\":[\"s1\"]}\n",
    )
    .expect("marker");
    let run = report(&repo, &home, &["--run"]);

    assert!(run.status.success(), "{run:?}");
    // 1 000 read from the cache and 10 written, as `run-totals report` prints them.
    assert_eq!(
        stdout(&run).lines().nth(2),
        Some("| 1 min | 1,010 | 0 | 0 | 1,000 | 10 |")
    );
    let _ = fs::remove_dir_all(&home);
}

#[test]
fn fails_outside_a_repository() {
    let home = env::temp_dir().join(format!("metrics-session-{}-outside", std::process::id()));

    fs::create_dir_all(&home).expect("folder");
    let output = clean(env!("CARGO_BIN_EXE_session-report"), &home, &home)
        .env("GIT_CEILING_DIRECTORIES", env::temp_dir())
        .output()
        .expect("runs");

    assert_eq!(output.status.code(), Some(1));
    let _ = fs::remove_dir_all(&home);
}
