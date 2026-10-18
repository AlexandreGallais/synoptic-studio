//! Tests of the `metrics` command in a repository of its own, its transcripts, logs and library
//! under a home of its own (EN-107, `common`).

mod common;

use std::fs;
use std::path::Path;
use std::process::Output;

use common::{clean, projects, setup, stdout};

fn metrics(dir: &Path, home: &Path, arguments: &[&str]) -> Output {
    clean(env!("CARGO_BIN_EXE_metrics"), dir, home)
        .args(arguments)
        .output()
        .expect("metrics runs")
}

#[test]
fn f21_ac2_reports_the_main_checkouts_work_from_a_worktree_its_findings_included() {
    let (home, repo, worktree) = setup("report");
    let folder = projects(&home, &repo);
    let lines = [
        r#"{"type":"assistant","sessionId":"s1","gitBranch":"chore/en-107-x","timestamp":"2026-10-12T10:00:00Z","message":{"id":"m1","usage":{"output_tokens":10},"content":[{"type":"tool_use","id":"t1","name":"Bash","input":{"command":"npm run lint"}}]}}"#,
        r#"{"type":"user","sessionId":"s1","gitBranch":"chore/en-107-x","timestamp":"2026-10-12T10:00:30Z","message":{"content":[{"type":"tool_result","tool_use_id":"t1","is_error":true,"content":"src/a.ts:1:1 local/kind-limits x"}]}}"#,
        "",
    ];

    fs::create_dir_all(folder.join("s1/subagents")).expect("projects");
    fs::write(folder.join("s1.jsonl"), lines.join("\n")).expect("transcript");
    fs::create_dir_all(repo.join(".git/metrics")).expect("metrics");
    fs::write(
        repo.join(".git/metrics/tool-calls.jsonl"),
        "{\"toolUseId\":\"t1\",\"durationMs\":1500,\"time\":\"2026-10-12T10:00:30Z\",\"tool\":\"Bash\",\"summary\":\"npm run lint\"}\n",
    )
    .expect("log");

    let output = metrics(&worktree, &home, &[]);
    let text = stdout(&output);

    assert!(output.status.success(), "{output:?}");
    assert!(text.starts_with("# Metrics of the agent's work\n\n1 model requests, 1 tool calls (1 timed exactly by the hook;"));
    assert!(text.contains("| 1.5 s | yes | Bash | chore/en-107-x | `npm run lint` |"));
    // Without the backlog's files, the story is its own epic.
    assert!(text.contains("| lint | `local/kind-limits` | 1 | EN-107 1 |"));
    assert!(text.contains("\n\n## Sources\n\n"));
    assert!(repo.join(".git/metrics/transcripts-cache.jsonl").is_file());

    // Read again from the cache, the same report.
    assert_eq!(stdout(&metrics(&worktree, &home, &[])), text);
    let _ = fs::remove_dir_all(&home);
}

#[test]
fn f18_ac11_refuses_a_since_that_is_not_a_day() {
    let (home, repo, _) = setup("since");
    let output = metrics(&repo, &home, &["--since", "yesterday"]);

    assert_eq!(output.status.code(), Some(1));
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "metrics: --since yesterday: not a day written YYYY-MM-DD\n"
    );

    let since = metrics(&repo, &home, &["--since", "2026-10-16"]);

    assert!(stdout(&since).contains("Since 2026-10-16 (UTC) only"));
    let _ = fs::remove_dir_all(&home);
}
