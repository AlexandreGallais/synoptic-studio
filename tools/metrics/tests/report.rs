//! Tests of the metrics report put together (EN-107): its sections in order, the exact durations
//! of the hook, the units of work, the findings per epic, and the transcripts' listing.

use std::collections::HashMap;
use std::env;
use std::fs;

use metrics::report::{ReportInputs, hook_durations, metrics_report, transcript_files};
use metrics::transcript::{Transcript, read_chunk};
use serde_json::json;

/// A session on the story branch of EN-107: a request at 10:00:00 of 1 000 tokens read and 10
/// written, its `npm run lint` call answered at 10:00:30 with two findings of one rule, then a
/// refusal of a hook at 10:01:00.
fn transcript() -> Transcript {
    let lines = [
        r#"{"type":"assistant","sessionId":"s1","gitBranch":"chore/en-107-x","timestamp":"2026-10-12T10:00:00Z","message":{"id":"m1","model":"claude-opus-5-5","usage":{"cache_read_input_tokens":1000,"output_tokens":10},"content":[{"type":"tool_use","id":"t1","name":"Bash","input":{"command":"npm run lint"}}]}}"#,
        r#"{"type":"user","sessionId":"s1","gitBranch":"chore/en-107-x","timestamp":"2026-10-12T10:00:30Z","message":{"content":[{"type":"tool_result","tool_use_id":"t1","is_error":true,"content":"src/a.ts:1:1 local/kind-limits x\nsrc/b.ts:2:1 local/kind-limits y"}]}}"#,
        r#"{"type":"assistant","sessionId":"s1","gitBranch":"chore/en-107-x","timestamp":"2026-10-12T10:00:40Z","message":{"id":"m2","usage":{"output_tokens":5},"content":[{"type":"tool_use","id":"t2","name":"Bash","input":{"command":"git add a && git commit"}}]}}"#,
        r#"{"type":"user","sessionId":"s1","gitBranch":"chore/en-107-x","timestamp":"2026-10-12T10:01:00Z","message":{"content":[{"type":"tool_result","tool_use_id":"t2","is_error":true,"content":"PreToolUse:Bash hook error: A worktree refuses a chained Git command: run it alone"}]}}"#,
        "",
    ];

    read_chunk(Transcript::default(), &lines.join("\n"))
}

/// The report's inputs around [`transcript`]: the hook timed `t1` at 25 s, a commit of EN-107.T1
/// at 10:02, a use of `format`, a saved turn, the backlog's files.
fn inputs(since: i64) -> ReportInputs {
    ReportInputs {
        transcripts: vec![transcript()],
        logged: vec![
            json!({"toolUseId": "t1", "durationMs": 25_000, "time": "2026-10-12T10:00:30Z", "tool": "Bash", "summary": "npm run lint", "isOk": false}),
        ],
        uses: vec![
            json!({"tool": "format", "subcommand": "--check", "start": "2026-10-12T10:00:10.000Z", "durationMs": 40, "exitCode": 0}),
        ],
        turns: Vec::new(),
        log: "abcdef1234\u{1f}2026-10-12T12:02:00+02:00\u{1f}feat(tooling): port\u{1f}Refs: EN-107.T1\u{1e}\n".to_owned(),
        stories: vec!["E17-F21-EN-107-metrics-tools-in-rust.md".to_owned()],
        features: vec!["E17-F21-agent-tools-in-rust.md".to_owned()],
        sources: "The library's uses\n".to_owned(),
        since,
    }
}

/// The report's sections, by their titles, each with its text.
fn sections(report: &str) -> HashMap<String, String> {
    report
        .split("\n## ")
        .skip(1)
        .map(|section| {
            let (title, body) = section.split_once("\n\n").unwrap_or((section, ""));

            (title.to_owned(), body.to_owned())
        })
        .collect()
}

#[test]
fn puts_the_sections_in_their_order_the_findings_before_the_sources() {
    let report = metrics_report(&inputs(0), &hook_durations(&inputs(0).logged));
    let titles: Vec<&str> = report
        .lines()
        .filter_map(|line| line.strip_prefix("## "))
        .collect();

    assert!(report.starts_with("# Metrics of the agent's work\n\n2 model requests, 2 tool calls (1 timed exactly by the hook;"));
    assert!(report.ends_with("## Sources\n\nThe library's uses\n"));
    assert_eq!(
        titles,
        [
            "Commands by count",
            "Commands by total time",
            "Slowest calls",
            "Sessions",
            "Features",
            "Stories and branches",
            "Tasks, by commit",
            "Turns",
            "Where the time of the latest turns went",
            "The agent's tools",
            "Tools",
            "Findings",
            "Sources",
        ]
    );
}

#[test]
fn times_a_call_by_the_hook_else_by_the_transcript() {
    let report = metrics_report(&inputs(0), &hook_durations(&inputs(0).logged));
    let slowest = &sections(&report)["Slowest calls"];

    // t1: the hook's 25 s, exact; t2: 10:00:40 to 10:01:00 from the transcript, 20 s.
    assert!(slowest.contains("| 25.0 s | yes | Bash | chore/en-107-x | `npm run lint` |"));
    assert!(
        slowest.contains("| 20.0 s | no | Bash | chore/en-107-x | `git add a && git commit` |")
    );
    assert!(sections(&report)["Commands by count"].contains("| `npm run lint` | 1 | 25.0 s |"));
}

#[test]
fn gives_the_work_to_its_story_feature_and_task() {
    let sections = sections(&metrics_report(&inputs(0), &HashMap::new()));

    // Moments at 10:00:00, 10:00:30, 10:00:40 and 10:01:00: 60 s active; 1 010 + 5 tokens.
    assert!(sections["Features"].contains("| F21 | 1 min 00 s | 15 | 1,015 | 2 |"));
    assert!(sections["Stories and branches"].contains("| EN-107 | 1 min 00 s |"));
    assert!(
        sections["Tasks, by commit"]
            .contains("| EN-107.T1 abcdef1 feat(tooling): port | 1 min 00 s |")
    );
    assert!(sections["Sessions"].contains("| 2026-10-12 s1 | 1 min 00 s |"));
    assert!(sections["Tools"].contains("| `format --check` | 1 | 0 | 40 ms | 40 ms |"));
}

#[test]
fn counts_the_findings_of_each_rule_per_epic() {
    let sections = sections(&metrics_report(&inputs(0), &HashMap::new()));
    let findings = &sections["Findings"];

    // The story EN-107 is of E17, from its file's name.
    assert!(findings.contains("| lint | `local/kind-limits` | 2 | E17 2 |"));
    assert!(findings.contains(
        "| hook | `PreToolUse:Bash: A worktree refuses a chained Git command` | 1 | E17 1 |"
    ));
}

#[test]
fn f18_ac11_keeps_what_happened_from_the_since_day_on() {
    // 2026-10-13T00:00Z: after every moment of the session.
    let report = metrics_report(&inputs(1_791_849_600_000), &HashMap::new());

    assert!(report.contains("0 model requests, 0 tool calls (0 timed exactly by the hook;"));
    assert!(report.contains(
        "each capped at 5 minutes. Since 2026-10-13 (UTC) only; the sources from the start."
    ));
    assert_eq!(sections(&report)["Findings"].lines().count(), 4);
}

#[test]
fn lists_the_transcripts_breadth_first_each_folders_names_sorted() {
    let root = env::temp_dir().join(format!("metrics-files-{}", std::process::id()));

    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("b/subagents")).expect("folders");
    fs::create_dir_all(root.join("a/subagents")).expect("folders");
    for file in [
        "b.jsonl",
        "a.jsonl",
        "a/subagents/z.jsonl",
        "a/subagents/y.jsonl",
        "b/subagents/x.jsonl",
        "a/notes.md",
    ] {
        fs::write(root.join(file), "").expect("file");
    }

    let listed: Vec<String> = transcript_files(&root)
        .iter()
        .map(|path| {
            path.strip_prefix(&root)
                .expect("inside")
                .to_string_lossy()
                .into_owned()
        })
        .collect();

    assert_eq!(
        listed,
        [
            "a.jsonl",
            "b.jsonl",
            "a/subagents/y.jsonl",
            "a/subagents/z.jsonl",
            "b/subagents/x.jsonl"
        ]
    );
    assert_eq!(
        transcript_files(&root.join("missing")),
        [] as [std::path::PathBuf; 0]
    );
    let _ = fs::remove_dir_all(&root);
}
