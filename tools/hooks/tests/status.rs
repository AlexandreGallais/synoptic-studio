//! Tests of the status line, its tally and the `status-line` binary (EN-104): the cases of
//! `.claude/hooks/status-line.test.ts`, `status-place.test.ts` and `status-usage.test.ts` carried
//! over, then a request counted from its last line.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use hooks::json::{Value, parse, quote};
use hooks::status::{
    RunningCall, Status, background_lines, background_text, clock_duration, ended_calls, grouped,
    is_run_open, is_turn_done, place_of, read_running_call, running_line, session_snapshot,
    status_text, usage_snapshot,
};
use hooks::tasks::{Kind, Task};
use hooks::time::short_duration;
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

/// What the status line shows, before a test changes some of it.
fn status() -> Status {
    Status {
        background: Vec::new(),
        context_percent: 10.0,
        duration_ms: 60_000,
        model: "Opus 5.5".into(),
        place: "E14 › F12".into(),
        run: String::new(),
        is_running_long: false,
        running: String::new(),
        silent_ms: 1000,
        step: String::new(),
        tally: empty_tally(),
    }
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
    assert!(!is_turn_done(&ended));
    assert!(
        status_text(&Status {
            tally: ended.clone(),
            ..status()
        })
        .contains("◌ chat free · 1 agent, 1 command in the background")
    );
    let notified = add_tokens(
        ended,
        "{\"type\": \"queue-operation\", \"timestamp\": \"2026-10-13T08:30:00Z\", \"content\": \"<task-notification>\\n<tool-use-id>toolu_A</tool-use-id>\\n<tool-use-id>toolu_B</tool-use-id>\"}\n",
    );

    assert_eq!(notified.pending, []);
    assert!(is_turn_done(&notified));
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

    assert!(!is_turn_done(&running));
    let notified = add_tokens(
        running,
        "{\"type\": \"queue-operation\", \"timestamp\": \"2026-10-15T12:22:26Z\", \"content\": \"<task-notification>\\n<task-id>a93498ccc604c403b</task-id>\\n<tool-use-id>toolu_013f</tool-use-id>\\n<status>completed</status>\"}\n",
    );

    assert!(is_turn_done(&notified));
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
    assert_eq!(background_text(&tally), "1 agent, 2 commands");
    assert_eq!(background_text(&empty_tally()), "");
}

#[test]
fn f18_ac9_shows_a_worker_in_the_background_and_its_duration_never_waiting_nor_your_turn() {
    let tally = Tally {
        pending: vec![
            task("w", Kind::Agent, "resumed: EN-039 audit findings to fix", 0),
            task("m", Kind::Message, "resumed: other", 0),
        ],
        start: 1,
        ..empty_tally()
    };
    let background = background_lines(&tally.pending, &[], 900_000, short_duration);
    let shown = Status {
        background,
        tally,
        ..status()
    };
    let text = status_text(&shown);
    let lines: Vec<&str> = text.split('\n').collect();

    // 900 000 ms = 15 min, the time the resumed EN-039 worker ran; a message still unanswered is
    // neither a line nor counted.
    assert_eq!(lines.len(), 3);
    assert_eq!(lines[0], "\u{1b}[1mE14 › F12\u{1b}[0m");
    assert!(lines[1].contains("◌ chat free · 1 agent in the background"));
    assert_eq!(
        lines[2],
        "\u{1b}[2m↳ agent resumed: EN-039 audit findings to fix — 15 min\u{1b}[0m"
    );
    assert!(!text.contains("waiting") && !text.contains("your turn"));
    let snapshot = parse(&session_snapshot(&shown, 0)).expect("JSON");

    assert_eq!(snapshot.get("pendingAgents"), &Value::Number(1.0));
    assert_eq!(snapshot.get("pendingCommands"), &Value::Number(0.0));
}

/// A call running.
fn call(agent: &str, description: &str, start: i64, summary: &str, tool: &str) -> RunningCall {
    RunningCall {
        agent: agent.into(),
        agent_id: String::new(),
        description: description.into(),
        session: String::new(),
        start,
        summary: summary.into(),
        tool: tool.into(),
    }
}

#[test]
fn shows_each_background_task_a_subagent_with_the_call_it_makes_now() {
    let tasks = [
        task("a", Kind::Agent, "auditor: Review EN-030", 0),
        task("b", Kind::Command, "Push", 60_000),
    ];
    let calls = [
        call("auditor", "", 100, "src/a.ts", "Read"),
        call("auditor", "List the files", 200, "", "Bash"),
        call("main", "", 300, "", "Edit"),
    ];

    // 240 000 ms = 4 min; 180 000 ms = 3 min.
    assert_eq!(
        background_lines(&tasks, &calls, 240_000, short_duration),
        [
            "↳ agent auditor: Review EN-030 — 4 min · now Bash · List the files",
            "↳ command Push — 3 min"
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

#[test]
fn writes_durations_to_the_second() {
    assert_eq!(clock_duration(45_900), "45 s");
    assert_eq!(clock_duration(12 * 60_000 + 5000), "12 min 05 s");
    assert_eq!(clock_duration(125 * 60_000 + 9000), "2 h 05 min 09 s");
    assert_eq!(clock_duration(-1500), "-2 s");
}

#[test]
fn groups_thousands_as_en_us_does() {
    assert_eq!(grouped(0), "0");
    assert_eq!(grouped(999), "999");
    assert_eq!(grouped(1234), "1,234");
    assert_eq!(grouped(5_678_901), "5,678,901");
    assert_eq!(grouped(-1_000), "-1,000");
}

#[test]
fn shows_the_place_and_step_the_turn_and_the_call_running() {
    let base = Status {
        context_percent: 34.4,
        place: "E01 › RET-001 retrospective e01".into(),
        tally: Tally {
            output: 1234,
            total: 5_678_901,
            ..empty_tally()
        },
        ..status()
    };
    let active = status_text(&Status {
        step: "RET-001 · metrics".into(),
        tally: Tally {
            is_waiting: false,
            ..base.tally.clone()
        },
        ..base.clone()
    });
    let silent = status_text(&Status {
        is_running_long: true,
        running: "▶ Bash · Run the checks — 12 s".into(),
        silent_ms: 180_000,
        ..base.clone()
    });

    assert!(active.contains("E01 › RET-001 retrospective e01"));
    assert!(active.contains("RET-001 · metrics"));
    assert!(active.contains("● working"));
    assert!(active.contains("⏱ turn 1 min 00 s"));
    assert!(active.contains("5,678,901 tokens this turn (1,234 written)"));
    assert!(active.contains("conversation memory 34 % full"));
    assert_eq!(active.split('\n').count(), 2);
    assert_eq!(
        silent.split('\n').next(),
        Some("\u{1b}[1mE01 › RET-001 retrospective e01\u{1b}[0m")
    );
    assert!(silent.contains("◌ silent for 3 min"));
    assert_eq!(
        silent.split('\n').nth(2),
        Some("\u{1b}[31m▶ Bash · Run the checks — 12 s\u{1b}[0m")
    );
    let done = status_text(&Status {
        tally: Tally {
            start: 1,
            ..base.tally.clone()
        },
        ..base.clone()
    });

    assert!(done.contains("◌ your turn"));
    assert!(done.contains("⏱ last turn 1 min 00 s"));
    let working = status_text(&Status {
        background: vec!["↳ command Push — 3 min".into()],
        running: "▶ Read · a — 1 s".into(),
        tally: Tally {
            is_waiting: false,
            pending: vec![task("b", Kind::Command, "Push", 0)],
            ..base.tally.clone()
        },
        ..base
    });
    let lines: Vec<&str> = working.split('\n').collect();

    assert!(lines[1].contains("● working · 1 command in the background"));
    assert_eq!(lines[2], "\u{1b}[36m▶ Read · a — 1 s\u{1b}[0m");
    assert_eq!(lines[3], "\u{1b}[2m↳ command Push — 3 min\u{1b}[0m");
}

#[test]
fn shows_the_open_run_beside_the_turn_and_nothing_of_a_run_outside_one() {
    let run = "run 2 h 05 · 43,100,000 tokens";
    let shown = status_text(&Status {
        run: run.into(),
        ..status()
    });

    assert!(
        shown
            .split('\n')
            .nth(1)
            .unwrap_or_default()
            .contains(&format!(
                "tokens this turn (0 written)\u{1b}[2m · \u{1b}[0m\u{1b}[35m{run}\u{1b}[0m"
            ))
    );
    assert!(!status_text(&status()).contains("run "));
}

#[test]
fn tells_an_open_run_s_marker_from_a_closed_or_missing_one() {
    assert!(is_run_open(r#"{"start":5,"end":0,"sessions":["a"]}"#));
    assert!(!is_run_open(r#"{"start":5,"end":9,"sessions":["a"]}"#));
    assert!(!is_run_open(r#"{"end":0}"#));
    assert!(!is_run_open(""));
}

/// Backlog files, as the status line lists them.
fn files() -> Vec<String> {
    [
        "E01-F04-US-018-turn-a-shape.md",
        "E01-RET-001-retrospective-e01.md",
        "E14-F11-code-base-ready-for-curves.md",
        "E17-F21-EN-104-x.md",
        "E02-F05-.md",
    ]
    .map(String::from)
    .to_vec()
}

#[test]
fn names_the_epic_feature_and_story_of_a_branch() {
    assert_eq!(
        place_of("feat/us-018-turn-a-shape", &files()),
        "E01 › F04 › US-018 turn a shape"
    );
    assert_eq!(
        place_of("docs/ret-001-retrospective-e01", &files()),
        "E01 › RET-001 retrospective e01"
    );
    assert_eq!(
        place_of("feature/f11-clean-up", &files()),
        "E14 › F11 › F11 code base ready for curves"
    );
    assert_eq!(
        place_of("chore/en-104-hooks", &files()),
        "E17 › F21 › EN-104 x"
    );
}

#[test]
fn keeps_the_branch_when_it_names_no_backlog_item() {
    assert_eq!(place_of("develop", &files()), "develop");
    assert_eq!(
        place_of("feat/us-099-unknown", &files()),
        "feat/us-099-unknown"
    );
    // An id inside a word, or of four digits, is none.
    assert_eq!(place_of("feat/xus-018", &files()), "feat/xus-018");
    assert_eq!(place_of("feat/us-0180", &files()), "feat/us-0180");
}

#[test]
fn reads_a_backlog_file_name_as_the_regular_expression_did() {
    // A feature file without a slug after its feature: the feature read as the slug, its hyphen
    // a space.
    let names = vec!["E02-F05-.md".to_owned()];

    assert_eq!(place_of("feature/f05-x", &names), "E02 › F05 F05 ");
}

#[test]
fn reads_back_a_call_starting_and_shows_the_oldest() {
    let record = parse(
        r#"{"agent":"main","description":"Run the checks","start":1000,"summary":"npm run check","tool":"Bash"}"#,
    )
    .expect("JSON");

    assert_eq!(
        read_running_call(&record),
        call("main", "Run the checks", 1000, "npm run check", "Bash")
    );
    assert_eq!(read_running_call(&Value::Null), call("main", "", 0, "", ""));
    let calls = [
        call("main", "", 5000, "ls", "Bash"),
        call("main", "Run the checks", 1000, "npm run check", "Bash"),
    ];
    let seconds = |milliseconds: i64| format!("{} s", milliseconds / 1000);

    assert_eq!(
        running_line(&calls, 13_000, seconds),
        "▶ Bash · Run the checks — 12 s"
    );
    assert_eq!(
        running_line(&calls[..1], 6000, seconds),
        "▶ Bash · ls — 1 s"
    );
    assert_eq!(running_line(&[], 0, seconds), "");
}

#[test]
fn f32_ac1_reads_back_the_subagent_and_the_session_of_a_call() {
    let record = parse(
        r#"{"agent":"story-worker","agentId":"a1","description":"","session":"s","start":7,"summary":"ls","tool":"Bash"}"#,
    )
    .expect("JSON");

    assert_eq!(
        read_running_call(&record),
        RunningCall {
            agent_id: "a1".into(),
            session: "s".into(),
            ..call("story-worker", "", 7, "ls", "Bash")
        }
    );
}

/// A call of a subagent of session `s`, its file named after its id.
fn of_agent(name: &str, agent: &str, start: i64, session: &str) -> (String, RunningCall) {
    (
        format!("{name}.json"),
        RunningCall {
            agent_id: format!("id-{name}"),
            session: session.into(),
            ..call(agent, "", start, "git log", "Bash")
        },
    )
}

#[test]
fn f32_ac1_finds_the_calls_left_by_subagents_that_ended() {
    // 2026-10-18: no agent ran, yet 169 files of `story-worker` calls stayed in `running/`, the
    // latest shown as a story worker running for 9 min 20 (C5). Here one subagent runs, launched at
    // 1 000; calls of ended subagents started at 100 and 900, one of them in a file older than
    // EN-120 (no session); one of this subagent at 1 500; one of another session at 200; the main
    // agent's at 50.
    let tasks = [
        task("a", Kind::Agent, "story-worker: EN-120", 1000),
        task("b", Kind::Command, "Push", 10),
    ];
    let calls = [
        of_agent("old", "story-worker", 100, "s"),
        of_agent("legacy", "story-worker", 900, ""),
        of_agent("live", "story-worker", 1500, "s"),
        of_agent("other", "story-worker", 200, "t"),
        ("main.json".to_owned(), call("main", "", 50, "ls", "Bash")),
    ];

    assert_eq!(
        ended_calls(&calls, &tasks, "s"),
        ["old.json", "legacy.json"]
    );
    // No subagent runs: every subagent call of the session has ended; a command is no subagent.
    assert_eq!(
        ended_calls(&calls, &tasks[1..], "s"),
        ["old.json", "legacy.json", "live.json"]
    );
    // Launched exactly when the call started: the call may be its, it is kept.
    assert_eq!(
        ended_calls(&calls[2..3], &[task("c", Kind::Agent, "x", 1500)], "s"),
        [] as [&str; 0]
    );
    // A resumed subagent counts as running from its resume.
    assert_eq!(
        ended_calls(
            &calls,
            &[task("r", Kind::Agent, "resumed: EN-120", 950)],
            "s"
        ),
        ["old.json", "legacy.json"]
    );
}

#[test]
fn keeps_the_weekly_and_five_hour_windows_with_their_reset_times() {
    let input = parse(
        r#"{"rate_limits": {"five_hour": {"used_percentage": 23.5, "resets_at": 1738425600}, "seven_day": {"used_percentage": 91, "resets_at": 1738857600}}}"#,
    )
    .expect("JSON");

    assert_eq!(
        usage_snapshot(&input, 5).as_deref(),
        Some(
            r#"{"fiveHour":{"resetsAt":"2025-02-01T16:00:00Z","usedPercent":23.5},"sevenDay":{"resetsAt":"2025-02-06T16:00:00Z","usedPercent":91},"time":5}"#
        )
    );
    let weekly =
        parse(r#"{"rate_limits": {"seven_day": {"used_percentage": 40, "resets_at": 0}}}"#)
            .expect("JSON");

    assert_eq!(
        usage_snapshot(&weekly, 5).as_deref(),
        Some(r#"{"sevenDay":{"resetsAt":"1970-01-01T00:00:00Z","usedPercent":40},"time":5}"#)
    );
    // No rate limits, or a window without its reset time: nothing, the last file kept.
    for text in [
        r#"{"model": {"display_name": "Opus 5.5"}}"#,
        r#"{"rate_limits": {"five_hour": {"used_percentage": 10}}}"#,
    ] {
        assert_eq!(usage_snapshot(&parse(text).expect("JSON"), 5), None);
    }
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

/// A repository on a story's branch, with its backlog's files and a transcript.
fn project(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("status-line-{name}-{}", std::process::id()));

    let _ = fs::remove_dir_all(&root);
    for (folder, file) in [
        ("docs/backlog/stories", "E17-F21-EN-104-hooks-in-rust.md"),
        ("docs/backlog/features", "E17-F21-agent-tools.md"),
    ] {
        fs::create_dir_all(root.join(folder)).expect("folder");
        fs::write(root.join(folder).join(file), "").expect("file");
    }
    let status = clean("git")
        .args(["init", "-q", "-b", "chore/en-104-hooks-in-rust"])
        .current_dir(&root)
        .status()
        .expect("git runs");

    assert!(status.success());
    root
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
fn draws_the_status_line_and_writes_its_files() {
    let root = project("draw");
    let metrics = root.join(".git/metrics");
    let transcript = root.join("t.jsonl");

    fs::create_dir_all(metrics.join("running")).expect("running");
    fs::write(metrics.join("step"), "  EN-104 · T3\n").expect("step");
    // A main call started before the turn, one in it, a subagent's, a file being written.
    for (name, start, agent) in [
        ("old", AT_0800 - 1, "main"),
        ("now", AT_0800 + 1, "main"),
        ("sub", 1, "auditor"),
    ] {
        fs::write(
            metrics.join(format!("running/{name}.json")),
            format!(r#"{{"agent":"{agent}","description":"","start":{start},"summary":"{name}","tool":"Bash"}}"#),
        )
        .expect("running file");
    }
    fs::write(metrics.join("running/partial.json"), "{").expect("partial");
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
    let lines: Vec<&str> = shown.split('\n').collect();

    assert!(output.status.success());
    assert_eq!(
        lines[0],
        "\u{1b}[1mE17 › F21 › EN-104 hooks in rust\u{1b}[0m\u{1b}[2m · \u{1b}[0mEN-104 · T3"
    );
    // 34.5 % rounded half up; the last line, being written, not read yet.
    assert!(
        lines[1].contains("116 tokens this turn (5 written)"),
        "{shown}"
    );
    assert!(lines[1].contains("conversation memory 35 % full"));
    assert!(lines[2].contains("▶ Bash · now — "), "{shown}");
    assert_eq!(lines.len(), 3);
    assert!(!metrics.join("running/old.json").exists());
    assert!(metrics.join("running/sub.json").exists());
    let snapshot =
        parse(&fs::read_to_string(metrics.join("now.json")).expect("now")).expect("JSON");

    assert_eq!(snapshot.get("turnTokens"), &Value::Number(116.0));
    assert_eq!(
        snapshot.get("turnStart"),
        &Value::Number(1_791_878_400_000.0)
    );
    assert!(
        fs::read_to_string(metrics.join("usage.json"))
            .expect("usage")
            .starts_with(
                r#"{"sevenDay":{"resetsAt":"1970-01-01T00:00:00Z","usedPercent":60},"time":"#
            )
    );
    // The cache kept the offset of the complete lines: a second draw reads nothing again.
    let cached = turn_tokens(
        &transcript.to_string_lossy(),
        &metrics.join("status-line-turn.json"),
    );

    assert_eq!(cached.total, 116);
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
