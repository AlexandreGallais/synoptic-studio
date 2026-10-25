//! Tests of the status line, its tally and the `status-line` binary (EN-104, AUD-051): the turn
//! read from a transcript, a request counted from its last line, then only `working` or
//! `chat free` shown.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use hooks::json::{Value, parse, quote};
use hooks::status::{session_snapshot, status_text};
use hooks::tasks::{Kind, Task};
use hooks::tokens::{Tally, add_tokens, cache_json, cached_tally, empty_tally, turn_tokens};

/// 2026-10-13T08:00:00Z, 08:03:00Z and 09:00:00Z in epoch milliseconds: 09:00 is 31 h 33 min 05 s
/// before 2026-10-14T16:33:05Z, 1 791 995 585 000 (the TypeScript test's value), so
/// 1 791 995 585 000 − 113 585 000; 08:03 is 57 min, 3 420 000, before it; 08:00 is 3 min more.
const AT_0800: i64 = 1_791_878_400_000;
const AT_0803: i64 = 1_791_878_580_000;
const AT_0900: i64 = 1_791_882_000_000;

/// Transcript line of an assistant message: 1 input, 10 cache writes and 100 cache reads.
fn line(id: &str, output: i64) -> String {
    format!(
        r#"{{"message": {{"id": "{id}", "usage": {{"input_tokens": 1, "cache_creation_input_tokens": 10, "cache_read_input_tokens": 100, "output_tokens": {output}}}}}}}"#
    )
}

/// Transcript line of a prompt typed by the person.
fn prompt(time: &str) -> String {
    format!(r#"{{"type": "user", "timestamp": "{time}", "message": {{"content": "go on"}}}}"#)
}

/// A `tool_use` block of an `Agent` call, its id also its description.
fn agent(id: &str) -> String {
    format!(
        r#"{{"type": "tool_use", "id": "{id}", "name": "Agent", "input": {{"subagent_type": "Explore", "description": "{id}"}}}}"#
    )
}

/// Transcript line of the agent ending its turn.
const END: &str = r#"{"message": {"id": "z", "stop_reason": "end_turn"}}"#;

/// Lines joined, each ended by a line feed.
fn text(lines: &[&str]) -> String {
    lines.iter().flat_map(|line| [*line, "\n"]).collect()
}

/// The ids of the tasks pending.
fn ids(tally: &Tally) -> Vec<&str> {
    tally.pending.iter().map(|task| task.id.as_str()).collect()
}

/// A task of a kind.
fn task(id: &str, kind: Kind, label: &str, start: i64) -> Task {
    Task {
        id: id.into(),
        kind,
        label: label.into(),
        start,
    }
}

#[test]
fn counts_a_request_written_on_two_lines_once_and_moves_past_the_text_read() {
    let lines = text(&[
        &line("a", 5),
        &line("a", 5),
        r#"{"type": "user"}"#,
        &line("b", 7),
    ]);
    let tally = add_tokens(empty_tally(), &lines);

    // a: 1 + 10 + 100 + 5 = 116; b: 1 + 10 + 100 + 7 = 118; 234 in all, 12 written.
    assert_eq!(
        (
            tally.last_id.as_str(),
            tally.offset,
            tally.output,
            tally.total
        ),
        ("b", lines.len(), 12, 234)
    );
}

#[test]
fn continues_a_tally_without_counting_again_the_request_it_ended_on() {
    let tally = add_tokens(empty_tally(), &text(&[&line("a", 5)]));

    // a once, 116, then c: 1 + 10 + 100 + 1 = 112.
    assert_eq!(
        add_tokens(tally, &text(&[&line("a", 5), &line("c", 1)])).total,
        116 + 112
    );
}

#[test]
fn f21_ac1_counts_a_subagent_s_request_once_from_its_last_line() {
    // A subagent's transcript repeats a request's usage with a growing `output_tokens` (EN-092).
    let lines = text(&[&line("a", 5), &line("a", 9), &line("b", 2), &line("b", 30)]);
    let tally = add_tokens(empty_tally(), &lines);

    // a from its last line: 1 + 10 + 100 + 9 = 120; b: 111 + 30 = 141; written 9 + 30.
    assert_eq!((tally.output, tally.total), (39, 261));
    // Read in two pieces through the cache, the last line of `b` read second: the same.
    let cut = text(&[&line("a", 5), &line("a", 9), &line("b", 2)]);
    let first = add_tokens(empty_tally(), &cut);
    let cached = parse(&cache_json(&first, "t.jsonl")).expect("JSON");
    let second = add_tokens(cached_tally(&cached, "t.jsonl"), &text(&[&line("b", 30)]));

    assert_eq!((second.output, second.total), (39, 261));
}

#[test]
fn counts_the_current_turn_only_from_the_prompt_after_the_agent_ended_its_turn() {
    let first = text(&[&prompt("2026-10-13T08:00:00Z"), &line("a", 5), END]);
    let second = text(&[
        &prompt("2026-10-13T09:00:00Z"),
        &line("b", 7),
        &prompt("2026-10-13T09:05:00Z"),
        &line("c", 1),
    ]);
    let tally = add_tokens(add_tokens(empty_tally(), &first), &second);

    // b and c only: 118 + 112; the prompt sent while the agent works stays in the turn.
    assert_eq!(
        (tally.is_waiting, tally.output, tally.start, tally.total),
        (false, 8, AT_0900, 230)
    );
}

#[test]
fn stops_the_turn_s_clock_at_the_agent_s_last_answer_and_restarts_it_at_the_next_prompt() {
    let ended = add_tokens(
        empty_tally(),
        &text(&[
            &prompt("2026-10-13T08:00:00Z"),
            &line("a", 5),
            r#"{"timestamp": "2026-10-13T08:03:00Z", "message": {"id": "z", "stop_reason": "end_turn"}}"#,
        ]),
    );

    assert_eq!((ended.end, ended.is_waiting), (AT_0803, true));
    let next = add_tokens(ended, &text(&[&prompt("2026-10-13T09:00:00Z")]));

    assert_eq!((next.end, next.is_waiting), (0, false));
}

#[test]
fn resumes_the_turn_when_the_agent_answers_a_notification_after_its_end() {
    let ended = add_tokens(
        empty_tally(),
        &text(&[
            &prompt("2026-10-13T08:00:00Z"),
            &line("a", 5),
            r#"{"timestamp": "2026-10-13T08:03:00Z", "message": {"id": "z", "stop_reason": "end_turn"}}"#,
        ]),
    );
    let tally = add_tokens(
        ended,
        &text(&[
            r#"{"type": "user", "promptSource": "system", "message": {"content": "done"}}"#,
            r#"{"type": "assistant", "message": {"id": "b", "usage": {"input_tokens": 1, "output_tokens": 7}}}"#,
        ]),
    );

    // Its clock and tokens run on: 5 + 7 written, from 08:00.
    assert_eq!(
        (tally.end, tally.is_waiting, tally.output, tally.start),
        (0, false, 12, AT_0800)
    );
}

#[test]
fn does_not_start_a_turn_at_a_compaction_summary_nor_at_a_task_notification() {
    let tally = add_tokens(
        empty_tally(),
        &text(&[
            END,
            r#"{"type": "user", "isCompactSummary": true, "message": {"content": "summary"}}"#,
            r#"{"type": "user", "promptSource": "system", "message": {"content": "done"}}"#,
            r#"{"type": "user", "isMeta": true, "message": {"content": "reminder"}}"#,
            r#"{"type": "user", "message": {"content": [{"type": "tool_result", "tool_use_id": "x"}]}}"#,
        ]),
    );

    assert_eq!((tally.is_waiting, tally.start), (true, 0));
}

#[test]
fn waits_for_background_tasks_after_the_end_of_the_turn() {
    let launch = r#"{"type": "assistant", "timestamp": "2026-10-13T08:01:00Z", "message": {"id": "a", "content": [{"type": "tool_use", "id": "toolu_A", "name": "Agent", "input": {}}, {"type": "tool_use", "id": "toolu_B", "name": "Bash", "input": {"run_in_background": true}}, {"type": "tool_use", "id": "toolu_C", "name": "Bash", "input": {}}]}}"#;
    let results = r#"{"type": "user", "message": {"content": [{"type": "tool_result", "tool_use_id": "toolu_A", "content": [{"type": "text", "text": "Async agent launched successfully."}]}, {"type": "tool_result", "tool_use_id": "toolu_C"}]}}"#;
    let ended = add_tokens(
        empty_tally(),
        &text(&[
            &prompt("2026-10-13T08:00:00Z"),
            launch,
            results,
            r#"{"timestamp": "2026-10-13T08:02:00Z", "message": {"id": "z", "stop_reason": "end_turn"}}"#,
        ]),
    );

    // A subagent and a command in the background run on; their immediate results end nothing.
    assert_eq!(ids(&ended), ["toolu_A", "toolu_B"]);
    assert_eq!(ended.finished, ["toolu_A", "toolu_C"]);
    assert!(ended.is_waiting);
    assert_eq!(status_text(&ended, 0), "\u{1b}[2mchat free\u{1b}[0m");
    let notified = add_tokens(
        ended,
        "{\"type\": \"queue-operation\", \"timestamp\": \"2026-10-13T08:30:00Z\", \"content\": \"<task-notification>\\n<tool-use-id>toolu_A</tool-use-id>\\n<tool-use-id>toolu_B</tool-use-id>\"}\n",
    );

    assert_eq!(notified.pending, []);
    assert!(notified.is_waiting);
}

#[test]
fn f18_ac2_ends_a_subagent_run_in_the_foreground_with_its_result_never_one_launched_in_the_background()
 {
    let agents: Vec<String> = ["fg", "bg", "text", "flag"]
        .iter()
        .map(|id| agent(id))
        .collect();
    let launch = format!(
        r#"{{"type": "assistant", "timestamp": "2026-10-17T01:00:00Z", "message": {{"id": "a", "content": [{}, {{"type": "tool_use", "id": "cmd", "name": "Bash", "input": {{"command": "npm run test", "run_in_background": true}}}}]}}}}"#,
        agents.join(", ")
    );
    let tally = add_tokens(
        empty_tally(),
        &text(&[
            &launch,
            // A foreground subagent's result: `status: "completed"` (the hooks reference).
            r#"{"type": "user", "message": {"content": [{"tool_use_id": "fg", "type": "tool_result", "content": [{"type": "text", "text": "The report."}]}]}, "toolUseResult": {"status": "completed", "content": []}}"#,
            // As the transcripts write a launch in the background (2026-10-09): text and flags.
            r#"{"type": "user", "message": {"content": [{"tool_use_id": "bg", "type": "tool_result", "content": [{"type": "text", "text": "Async agent launched successfully. (This tool result is internal metadata.)"}]}]}, "toolUseResult": {"isAsync": true, "status": "async_launched"}}"#,
            // The launch text without `toolUseResult` (a line of 2026-10-16), the flags without the text.
            r#"{"type": "user", "message": {"content": [{"tool_use_id": "text", "type": "tool_result", "content": [{"type": "text", "text": "Async agent launched successfully."}]}]}}"#,
            r#"{"type": "user", "message": {"content": [{"tool_use_id": "flag", "type": "tool_result", "content": "started"}]}, "toolUseResult": {"isAsync": true}}"#,
            // A command in the background answers at once too: only its notification ends it.
            r#"{"type": "user", "message": {"content": [{"tool_use_id": "cmd", "type": "tool_result", "content": "Command running in background with ID: b1"}]}}"#,
        ]),
    );

    assert_eq!(ids(&tally), ["bg", "text", "flag", "cmd"]);
}

#[test]
fn ends_a_background_call_answered_with_an_error() {
    // The line that kept the status line waiting for three hours (2026-10-14): a push in the
    // background refused because the safety check gave no verdict.
    let launch = r#"{"type": "assistant", "timestamp": "2026-10-14T16:33:05Z", "message": {"id": "a", "content": [{"type": "tool_use", "id": "toolu_P", "name": "Bash", "input": {"command": "git push", "description": "Push", "run_in_background": true}}]}}"#;
    let refused = r#"{"type": "user", "message": {"content": [{"type": "tool_result", "is_error": true, "tool_use_id": "toolu_P", "content": "The server-side auto mode classifier gave no verdict"}]}}"#;
    let started = add_tokens(
        empty_tally(),
        &text(&[&prompt("2026-10-14T16:33:00Z"), launch]),
    );

    assert_eq!(
        started.pending,
        [task("toolu_P", Kind::Command, "Push", 1_791_995_585_000)]
    );
    assert_eq!(add_tokens(started, &text(&[refused, END])).pending, []);
}

#[test]
fn f18_ac9_keeps_the_turn_open_while_a_resumed_subagent_runs() {
    // The lines that showed "your turn" for 15 minutes while a worker ran (2026-10-15, 12:07 to
    // 12:22 UTC): a message resumed the stopped EN-039 worker, its end notified under the
    // message's call id. The task itself, its label, start and end, and a message only queued,
    // are `tasks.rs`' (NAM-002): here, what the tally of a transcript adds, the turn kept open.
    let send = r#"{"type": "assistant", "timestamp": "2026-10-15T12:07:38Z", "message": {"id": "m", "content": [{"type": "tool_use", "id": "toolu_013f", "name": "SendMessage", "input": {"to": "a93498ccc604c403b", "summary": "EN-039 audit findings to fix", "message": "EN-039 audit findings: fix …"}}]}}"#;
    let resumed = r#"{"type": "user", "timestamp": "2026-10-15T12:07:40Z", "message": {"content": [{"tool_use_id": "toolu_013f", "type": "tool_result", "content": [{"type": "text", "text": "{\"success\":true,\"message\":\"Resuming agent a93498c\",\"resumedAgentId\":\"a93498ccc604c403b\"}"}]}]}}"#;
    let running = add_tokens(
        empty_tally(),
        &text(&[
            &prompt("2026-10-15T12:07:30Z"),
            send,
            resumed,
            r#"{"timestamp": "2026-10-15T12:07:46Z", "message": {"id": "z", "stop_reason": "end_turn"}}"#,
        ]),
    );

    assert_eq!(running.pending.len(), 1);
    let notified = add_tokens(
        running,
        "{\"type\": \"queue-operation\", \"timestamp\": \"2026-10-15T12:22:26Z\", \"content\": \"<task-notification>\\n<task-id>a93498ccc604c403b</task-id>\\n<tool-use-id>toolu_013f</tool-use-id>\\n<status>completed</status>\"}\n",
    );

    assert_eq!(notified.pending, []);
}

#[test]
fn counts_a_monitor_and_a_subagent_as_background_tasks_with_what_they_do() {
    let launch = r#"{"type": "assistant", "timestamp": "2026-10-14T16:00:00Z", "message": {"id": "a", "content": [{"type": "tool_use", "id": "toolu_M", "name": "Monitor", "input": {"description": "push F10", "command": "until …"}}, {"type": "tool_use", "id": "toolu_A", "name": "Agent", "input": {"subagent_type": "auditor", "description": "Review EN-030"}}, {"type": "tool_use", "id": "toolu_B", "name": "Bash", "input": {"command": "npm run check:all", "run_in_background": true}}]}}"#;
    let tally = add_tokens(empty_tally(), &text(&[launch]));
    let labels: Vec<(Kind, &str)> = tally
        .pending
        .iter()
        .map(|task| (task.kind, task.label.as_str()))
        .collect();

    assert_eq!(
        labels,
        [
            (Kind::Command, "push F10"),
            (Kind::Agent, "auditor: Review EN-030"),
            (Kind::Command, "npm run check:all")
        ]
    );
}

#[test]
fn drops_a_background_task_whose_end_never_came_after_three_hours() {
    let launch = r#"{"type": "assistant", "timestamp": "2026-10-13T08:00:00Z", "message": {"id": "a", "content": [{"type": "tool_use", "id": "toolu_A", "name": "Agent", "input": {}}]}}"#;

    assert_eq!(
        add_tokens(
            empty_tally(),
            &text(&[launch, &prompt("2026-10-13T11:00:01Z")])
        )
        .pending,
        []
    );
    assert_eq!(
        add_tokens(
            empty_tally(),
            &text(&[launch, &prompt("2026-10-13T10:59:59Z")])
        )
        .pending
        .len(),
        1
    );
}

#[test]
fn keeps_a_cached_tally_only_for_the_same_transcript() {
    let cache = parse(
        r#"{"transcript": "t.jsonl", "tally": {"end": 7, "isWaiting": false, "lastId": "a", "offset": 9, "output": 2, "pending": [{"id": "toolu_A", "start": 4}], "start": 5, "total": 3}}"#,
    )
    .expect("JSON");

    assert_eq!(
        cached_tally(&cache, "t.jsonl"),
        Tally {
            end: 7,
            is_waiting: false,
            last_id: "a".into(),
            offset: 9,
            output: 2,
            pending: vec![task("toolu_A", Kind::Command, "", 4)],
            start: 5,
            total: 3,
            ..empty_tally()
        }
    );
    assert_eq!(cached_tally(&cache, "other.jsonl"), empty_tally());
}

#[test]
fn writes_the_cache_as_the_typescript_status_line_did_then_the_last_request_s_counts() {
    let tally = Tally {
        finished: vec!["x".into()],
        pending: vec![task("a", Kind::Message, "resumed: \"b\"", 4)],
        last_output: 9,
        last_total: 120,
        ..empty_tally()
    };

    assert_eq!(
        cache_json(&tally, "t.jsonl"),
        r#"{"tally":{"end":0,"finished":["x"],"isWaiting":true,"lastId":"","offset":0,"output":0,"pending":[{"id":"a","kind":"message","label":"resumed: \"b\"","start":4}],"start":0,"total":0,"lastOutput":9,"lastTotal":120},"transcript":"t.jsonl"}"#
    );
}

/// A command without the `GIT_*` variables of the process running the tests: under the pre-push
/// hook, `GIT_DIR` names the repository itself (`.claude/rules/tooling.md`); started through the
/// shared `test_support::command` (EN-103), with a home of its own.
fn clean(program: &str) -> Command {
    let mut command = test_support::command(program);
    let home = std::env::temp_dir().join(format!("status-home-{}", std::process::id()));

    fs::create_dir_all(&home).expect("home");
    command.env("HOME", home).env("GIT_CONFIG_NOSYSTEM", "1");
    command
}

/// The binary run in a folder on an input.
fn draw(folder: &Path, input: &str) -> Output {
    let mut child = clean(env!("CARGO_BIN_EXE_status-line"))
        .current_dir(folder)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("status line runs");

    child
        .stdin
        .take()
        .expect("stdin")
        .write_all(input.as_bytes())
        .expect("input written");
    child.wait_with_output().expect("status line ends")
}

#[test]
fn aud_051_ends_the_turn_when_the_person_interrupts_the_agent() {
    // The two lines Claude Code writes on Escape (transcripts of 2026-10-11): no `end_turn` follows.
    for interruption in [
        r#"{"type": "user", "timestamp": "2026-10-13T08:03:00Z", "message": {"content": [{"type": "text", "text": "[Request interrupted by user for tool use]"}]}}"#,
        r#"{"type": "user", "timestamp": "2026-10-13T08:03:00Z", "message": {"content": "[Request interrupted by user]"}}"#,
    ] {
        let tally = add_tokens(
            empty_tally(),
            &text(&[&prompt("2026-10-13T08:00:00Z"), &line("a", 5), interruption]),
        );

        assert_eq!(
            (tally.is_waiting, tally.end),
            (true, AT_0803),
            "{interruption}"
        );
        assert_eq!(status_text(&tally, 0), "\u{1b}[2mchat free\u{1b}[0m");
    }
    // The same words quoted inside a prompt's text, not at its start, keep the agent working.
    let quoted = add_tokens(
        empty_tally(),
        &text(&[
            &prompt("2026-10-13T08:00:00Z"),
            r#"{"type": "user", "message": {"content": "why [Request interrupted by user]?"}}"#,
        ]),
    );

    assert!(!quoted.is_waiting);
}

#[test]
fn aud_051_shows_three_moving_dots_while_the_agent_works_and_chat_free_once_its_turn_ended() {
    let working = add_tokens(
        empty_tally(),
        &text(&[&prompt("2026-10-13T08:00:00Z"), &line("a", 5)]),
    );

    // One dot at second 0, two at 1, three at 2, one again at 3: 0, 1 999 and 3 000 ms; padded to
    // three characters so that the line keeps its width.
    assert_eq!(status_text(&working, 0), "\u{1b}[32mworking.  \u{1b}[0m");
    assert_eq!(
        status_text(&working, 1_999),
        "\u{1b}[32mworking.. \u{1b}[0m"
    );
    assert_eq!(
        status_text(&working, 2_000),
        "\u{1b}[32mworking...\u{1b}[0m"
    );
    assert_eq!(
        status_text(&working, 3_000),
        "\u{1b}[32mworking.  \u{1b}[0m"
    );
    // Before any prompt, and once the turn ended with a worker still in the background: free.
    let ended = add_tokens(working, &text(&[&agent_launch(), END]));

    assert_eq!(ended.pending.len(), 1);
    for tally in [empty_tally(), ended] {
        let shown = status_text(&tally, 5_000);

        assert_eq!(shown, "\u{1b}[2mchat free\u{1b}[0m");
        assert!(!shown.contains("token") && !shown.contains("agent"));
    }
}

/// An assistant line launching a subagent in the background, then the result saying so.
fn agent_launch() -> String {
    text(&[
        &format!(
            r#"{{"type": "assistant", "message": {{"id": "b", "content": [{}]}}}}"#,
            agent("w")
        ),
        r#"{"type": "user", "message": {"content": [{"tool_use_id": "w", "type": "tool_result", "content": "Async agent launched successfully."}]}, "toolUseResult": {"isAsync": true}}"#,
    ])
    .trim_end()
    .to_owned()
}

#[test]
fn writes_the_session_s_snapshot_for_each_tool_call_s_record() {
    let tally = Tally {
        output: 5,
        pending: vec![
            task("a", Kind::Agent, "auditor: Review", 0),
            task("b", Kind::Command, "Push", 0),
            task("c", Kind::Command, "Test", 0),
            task("m", Kind::Message, "resumed: other", 0),
        ],
        start: AT_0800,
        total: 116,
        ..empty_tally()
    };

    assert_eq!(
        session_snapshot(&tally, 34.5, "Opus \"5\"", 9),
        r#"{"contextPercent":34.5,"isWaiting":true,"model":"Opus \"5\"","pendingAgents":1,"pendingCommands":2,"time":9,"turnOutputTokens":5,"turnStart":1791878400000,"turnTokens":116}"#
    );
}

/// A repository with a transcript.
fn project(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("status-line-{name}-{}", std::process::id()));

    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).expect("folder");
    let status = clean("git")
        .args(["init", "-q", "-b", "chore/en-104-hooks-in-rust"])
        .current_dir(&root)
        .status()
        .expect("git runs");

    assert!(status.success());
    root
}

#[test]
fn draws_the_status_line_and_writes_its_files() {
    let root = project("draw");
    let metrics = root.join(".git/metrics");
    let transcript = root.join("t.jsonl");

    fs::write(
        &transcript,
        text(&[
            &prompt("2026-10-13T08:00:00Z"),
            &line("a", 5),
            "{\"partial\":",
        ])
        .trim_end(),
    )
    .expect("transcript");
    let input = format!(
        r#"{{"transcript_path":{},"model":{{"display_name":"Opus 5.5"}},"context_window":{{"used_percentage":34.5}},"rate_limits":{{"seven_day":{{"used_percentage":60,"resets_at":0}}}}}}"#,
        quote(&transcript.to_string_lossy())
    );
    let output = draw(&root, &input);
    let shown = String::from_utf8_lossy(&output.stdout);

    // One line, the agent working; nothing of the place, the tokens, the run nor the usage.
    assert!(output.status.success());
    assert!(shown.starts_with("\u{1b}[32mworking."), "{shown}");
    assert!(!shown.contains('\n'));
    assert!(!metrics.join("usage.json").exists());
    let snapshot =
        parse(&fs::read_to_string(metrics.join("now.json")).expect("now")).expect("JSON");

    // The last line, being written, not read yet: 116 tokens of the request `a`.
    assert_eq!(snapshot.get("turnTokens"), &Value::Number(116.0));
    assert_eq!(snapshot.get("contextPercent"), &Value::Number(34.5));
    assert_eq!(
        snapshot.get("turnStart"),
        &Value::Number(1_791_878_400_000.0)
    );
    // The cache kept the offset of the complete lines: a second draw reads nothing again.
    let cached = turn_tokens(
        &transcript.to_string_lossy(),
        &metrics.join("status-line-turn.json"),
    );

    assert_eq!(cached.total, 116);
    fs::write(
        &transcript,
        text(&[&prompt("2026-10-13T08:00:00Z"), &line("a", 5), END]),
    )
    .expect("transcript");
    assert_eq!(
        String::from_utf8_lossy(&draw(&root, &input).stdout),
        "\u{1b}[2mchat free\u{1b}[0m"
    );
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn fails_on_an_input_that_is_not_json_or_outside_a_repository() {
    let root = project("fail");
    let output = draw(&root, "not json");

    assert_eq!(output.status.code(), Some(1));
    assert_eq!(output.stdout, [] as [u8; 0]);
    fs::remove_dir_all(root.join(".git")).expect("no repository");
    let output = clean(env!("CARGO_BIN_EXE_status-line"))
        .env("GIT_CEILING_DIRECTORIES", std::env::temp_dir())
        .current_dir(&root)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            child.stdin.take().expect("stdin").write_all(b"{}")?;
            child.wait_with_output()
        })
        .expect("status line runs");

    assert_eq!(output.status.code(), Some(1));
    let _ = fs::remove_dir_all(&root);
}
