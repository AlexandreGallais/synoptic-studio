//! Tests of the `progress-page` command in a repository of its own, its transcripts under a home of
//! its own (EN-087). Every `GIT_*` variable is removed from the commands, and `HOME` points to the
//! test's folder, so that neither the real repository nor the real configuration is touched
//! (`.claude/rules/tooling.md`).

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const FEATURE: &str = "---\nid: F20\nepic: E17\ntitle: A context diet\nstatus: in-progress\n---\n\n\
| ID | Type | Title | Status |\n| --- | --- | --- | --- |\n\
| [EN-086](../stories/x.md) | Enabler | Parked questions | done |\n\
| [EN-087](../stories/y.md) | Enabler | A progress page | ready |\n";
const EPIC: &str = "---\nid: E17\ntitle: Fast tooling\nstatus: in-progress\n---\n";
const URL: &str = "https://claude.ai/code/artifact/5fbea6f3-1";

/// A command run in `dir` without the `GIT_*` variables, its home in `home`.
fn clean(program: &str, dir: &Path, home: &Path) -> Command {
    let mut command = Command::new(program);

    command.current_dir(dir).env("HOME", home);
    for (name, _) in env::vars_os() {
        if name.to_string_lossy().starts_with("GIT_") {
            command.env_remove(name);
        }
    }
    command.env("GIT_CONFIG_NOSYSTEM", "1");
    command
}

fn git(repo: &Path, home: &Path, words: &[&str]) -> String {
    let output = clean("git", repo, home)
        .args(words)
        .output()
        .expect("git runs");

    assert!(
        output.status.success(),
        "git {}: {}",
        words.join(" "),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).trim().to_owned()
}

fn page(repo: &Path, home: &Path, words: &[&str]) -> Output {
    clean(env!("CARGO_BIN_EXE_progress-page"), repo, home)
        .args(words)
        .output()
        .expect("progress-page runs")
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

/// A home, and inside it a repository whose feature branch is on `origin` with one commit of
/// EN-087 and the done commit of EN-086.
fn setup(name: &str) -> (PathBuf, PathBuf) {
    let home = env::temp_dir().join(format!("progress-page-{}-{name}", std::process::id()));

    let _ = fs::remove_dir_all(&home);
    fs::create_dir_all(home.join("repo")).expect("folders");

    let home = fs::canonicalize(&home).expect("home");
    let repo = home.join("repo");

    git(&repo, &home, &["init", "-q", "-b", "develop"]);
    git(&repo, &home, &["config", "user.name", "Test"]);
    git(&repo, &home, &["config", "user.email", "test@example.com"]);
    git(&repo, &home, &["config", "commit.gpgsign", "false"]);
    fs::create_dir_all(repo.join("docs/backlog/features")).expect("features");
    fs::create_dir_all(repo.join("docs/backlog/epics")).expect("epics");
    fs::write(
        repo.join("docs/backlog/features/E17-F20-a-context-diet.md"),
        FEATURE,
    )
    .expect("feature");
    fs::write(repo.join("docs/backlog/epics/E17-fast-tooling.md"), EPIC).expect("epic");
    git(&repo, &home, &["add", "."]);
    git(
        &repo,
        &home,
        &[
            "commit",
            "-q",
            "-m",
            "docs(backlog): set EN-086 done",
            "-m",
            "Closes: EN-086",
        ],
    );
    git(
        &repo,
        &home,
        &[
            "commit",
            "-q",
            "--allow-empty",
            "-m",
            "feat(tooling): write the page",
            "-m",
            "Refs: EN-087.T1",
        ],
    );
    git(
        &repo,
        &home,
        &[
            "update-ref",
            "refs/remotes/origin/feature/f20-a-context-diet",
            "HEAD",
        ],
    );
    (home, repo)
}

/// The run's marker, its subagent's transcript and the usage file, as the hooks write them.
fn write_metrics(repo: &Path, home: &Path, start: i64) {
    let common = PathBuf::from(git(
        repo,
        home,
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    ));
    let metrics = common.join("metrics");
    let subagents = run_totals::totals::projects_dir(home, &common)
        .join("s1")
        .join("subagents");

    fs::create_dir_all(&metrics).expect("metrics");
    fs::create_dir_all(&subagents).expect("subagents");
    fs::write(
        metrics.join("run.json"),
        format!("{{\"start\":{start},\"end\":0,\"sessions\":[\"s1\"]}}\n"),
    )
    .expect("marker");
    fs::write(
        metrics.join("usage.json"),
        "{\"fiveHour\":{\"usedPercent\":15.4},\"sevenDay\":{\"usedPercent\":54.6}}",
    )
    .expect("usage");
    fs::write(
        subagents.join("agent-a1.meta.json"),
        "{\"agentType\":\"story-worker\",\"description\":\"EN-087 progress page\"}",
    )
    .expect("meta");
    // One request after the run's start: 1 + 20 + 300 + 4 = 325 tokens.
    fs::write(
        subagents.join("agent-a1.jsonl"),
        "{\"type\":\"user\",\"timestamp\":\"2099-01-01T00:00:00.000Z\"}\n\
{\"type\":\"assistant\",\"timestamp\":\"2099-01-01T00:00:01.000Z\",\"message\":{\"id\":\"m1\",\"stop_reason\":\"tool_use\",\"usage\":{\"input_tokens\":1,\"cache_creation_input_tokens\":20,\"cache_read_input_tokens\":300,\"output_tokens\":4}}}\n",
    )
    .expect("transcript");
}

#[test]
fn f20_ac5_writes_the_page_and_keeps_its_address_for_the_run() {
    let (home, repo) = setup("run");
    let html_path = repo.join(".git/metrics/progress.html");

    // The run started one hour ago, so the two commits made now are in it.
    let start = hooks::time::now_ms() - 3_600_000;

    write_metrics(&repo, &home, start);

    let first = page(&repo, &home, &[]);

    assert!(first.status.success(), "{}", text(&first.stderr));
    assert!(text(&first.stdout).contains("progress.html\npublish it as a new page"));

    let html = fs::read_to_string(&html_path).expect("page written");

    assert!(html.contains("<h2>E17 · Fast tooling</h2>"));
    assert!(html.contains("<h3>F20 · A context diet</h3><p class=\"muted\">1 of 2 stories closed"));
    // The worker runs, its transcript holds 325 tokens; the plan's usage is rounded.
    assert!(html.contains("<td>EN-087</td><td>story-worker</td><td>EN-087 progress page</td>"));
    assert!(html.contains("325"));
    assert!(html.contains("15 % of the five hours, 55 % of the week"));
    assert!(html.contains("<tr class=\"now\"><td>EN-087</td>"));
    assert!(html.contains("feat(tooling): write the page"));
    assert!(html.contains("docs(backlog): set EN-086 done"));

    // The address the main thread got, kept for this run, then given back.
    let kept = page(&repo, &home, &["address", URL]);

    assert!(kept.status.success(), "{}", text(&kept.stderr));
    assert!(text(&page(&repo, &home, &[]).stdout).contains(&format!("publish it again at {URL}")));

    // A new run: the address is another run's, so a new page.
    write_metrics(&repo, &home, start + 1);
    assert!(text(&page(&repo, &home, &[]).stdout).contains("publish it as a new page"));
}

#[test]
fn writes_a_page_without_a_run_and_refuses_wrong_words() {
    let (home, repo) = setup("norun");
    let output = page(&repo, &home, &[]);

    assert!(output.status.success(), "{}", text(&output.stderr));
    assert!(
        fs::read_to_string(repo.join(".git/metrics/progress.html"))
            .expect("page")
            .contains("No run recorded")
    );

    let wrong_address = page(&repo, &home, &["address", "https://example.com/x"]);

    assert!(!wrong_address.status.success());
    assert!(text(&wrong_address.stderr).contains("not a claude.ai address"));

    let wrong_word = page(&repo, &home, &["publish"]);

    assert!(!wrong_word.status.success());
    assert!(text(&wrong_word.stderr).starts_with("usage: progress-page"));

    let outside = page(&home, &home, &[]);

    assert!(!outside.status.success());
}
