//! Tests of the `session-report` command in a repository of its own, its transcripts under a home
//! of its own (EN-107, `common`).

mod common;

use std::env;
use std::fs;
use std::path::Path;
use std::process::Output;

use common::{clean, setup, stdout, write_session};

fn report(dir: &Path, home: &Path, arguments: &[&str]) -> Output {
    clean(env!("CARGO_BIN_EXE_session-report"), dir, home)
        .args(arguments)
        .output()
        .expect("session-report runs")
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
