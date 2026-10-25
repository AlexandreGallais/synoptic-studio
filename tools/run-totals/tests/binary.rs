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
    let mut command = test_support::command(program);

    command.current_dir(dir).env("HOME", home);
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

/// The text with each count of seconds, `0 s` or `1 s`, written `_ s`.
fn seconds_hidden(text: &str) -> String {
    let words: Vec<&str> = text.split(' ').collect();

    words
        .iter()
        .enumerate()
        .map(|(index, word)| {
            let is_count = !word.is_empty() && word.bytes().all(|byte| byte.is_ascii_digit());
            let unit = words.get(index + 1).copied().unwrap_or("");

            if is_count && (unit == "s" || unit.starts_with("s,")) {
                "_"
            } else {
                word
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
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
fn f20_ac4_opens_counts_shows_and_ends_a_run() {
    let (home, repo) = setup("run");
    let folder = projects(&home, &repo);

    fs::create_dir_all(folder.join("s1").join("subagents")).expect("projects");
    // A request after any start: 2099. 1 + 10 + 100 + 1 000 = 1 111 tokens.
    fs::write(
        folder.join("s1").join("subagents").join("agent-a.jsonl"),
        "{\"type\":\"assistant\",\"timestamp\":\"2099-01-01T00:00:00Z\",\"message\":{\"id\":\"a\",\"usage\":{\"input_tokens\":1,\"cache_creation_input_tokens\":10,\"cache_read_input_tokens\":100,\"output_tokens\":1000}}}\n",
    )
    .expect("transcript");

    // A second session, after a restart: 2 + 20 + 200 + 2 000 = 2 222; 3 333 in all.
    fs::write(
        folder.join("s2.jsonl"),
        "{\"type\":\"assistant\",\"timestamp\":\"2099-01-01T00:00:00Z\",\"message\":{\"id\":\"b\",\"usage\":{\"input_tokens\":2,\"cache_creation_input_tokens\":20,\"cache_read_input_tokens\":200,\"output_tokens\":2000}}}\n",
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
    // The duration may have reached a second between two commands: only its form is checked.
    assert_eq!(
        seconds_hidden(&stdout(&totals(&repo, &home, &["status"]))),
        "run _ s · 3,333 tokens · week 34 % used"
    );
    assert!(
        seconds_hidden(&stdout(&totals(&repo, &home, &["report"])))
            .contains("| _ s, open | 3,333 |")
    );

    // Ended now: the 2099 requests fall after the end.
    let ended = seconds_hidden(&stdout(&totals(&repo, &home, &["end"])));

    assert!(ended.contains("| _ s | 0 | 0 | 0 | 0 | 0 |"), "{ended}");
    assert_eq!(stdout(&totals(&repo, &home, &["status"])), "");
    assert!(!totals(&repo, &home, &["unknown"]).status.success());
}

#[test]
fn f20_ac6_should_stop_goes_on_at_any_usage_and_names_the_parked_stories_of_the_records() {
    let (home, repo) = setup("stop");
    let bin = home.join("bin");
    let tool = bin.join("run-totals");

    // A copy of the tool beside a `knowledge` script standing for the records (EN-154).
    fs::create_dir_all(&bin).expect("bin");
    fs::copy(env!("CARGO_BIN_EXE_run-totals"), &tool).expect("tool copied");

    let stop = || {
        clean(tool.to_str().expect("path"), &repo, &home)
            .arg("should-stop")
            .output()
            .expect("runs")
    };

    // No `knowledge` beside it: the run goes on, the parked stories unknown.
    let unread = stop();

    assert!(unread.status.success());
    assert!(
        stdout(&unread).starts_with(
            "go: the week's usage stops no run (2026-10-25)\nparked: unknown, the records cannot be read: "
        ),
        "{}",
        stdout(&unread)
    );

    fs::write(
        bin.join("knowledge"),
        "#!/bin/sh\nprintf 'id\\nEN-101\\nUS-020\\n'\n",
    )
    .expect("script");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        fs::set_permissions(bin.join("knowledge"), fs::Permissions::from_mode(0o755))
            .expect("script runs");
    }
    fs::create_dir_all(repo.join(".git/metrics")).expect("metrics");
    // The week all but spent: no stop any more.
    fs::write(
        repo.join(".git/metrics/usage.json"),
        "{\"sevenDay\":{\"usedPercent\":99.5,\"resetsAt\":\"x\"},\"time\":1}",
    )
    .expect("usage");

    let spent = stop();

    assert!(spent.status.success());
    assert_eq!(
        stdout(&spent),
        "go: the week's usage stops no run (2026-10-25)\nparked: EN-101, US-020\n"
    );
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

    let stop = clean(env!("CARGO_BIN_EXE_run-totals"), &home, &home)
        .env("GIT_CEILING_DIRECTORIES", home.parent().expect("parent"))
        .arg("should-stop")
        .output()
        .expect("runs");

    // 2, not 1: no answer is not a stop.
    assert_eq!(stop.status.code(), Some(2));
}
