//! Tests of the background tasks read from a transcript, the cases `limit-agents` counts.

use hooks::json::parse;
use hooks::tasks::{
    Kind, LOST_TASK_MS, Task, after_line, completed_ids, failed_ids, notified_ids, pending_after,
    started_tasks,
};

/// A transcript line where the agent starts tool calls, given as JSON blocks.
fn uses(time: &str, blocks: &str) -> String {
    format!(r#"{{"type":"assistant","timestamp":"{time}","message":{{"content":[{blocks}]}}}}"#)
}

/// A `tool_use` block of an `Agent` call.
fn agent(id: &str, kind: &str, description: &str) -> String {
    format!(
        r#"{{"type":"tool_use","id":"{id}","name":"Agent","input":{{"subagent_type":"{kind}","description":"{description}"}}}}"#
    )
}

/// The notification of a background task's end.
fn notification(id: &str) -> String {
    format!(
        r#"{{"type":"user","timestamp":"2026-10-16T21:00:00Z","message":{{"content":"<task-notification>\n<task-id>a1</task-id>\n<tool-use-id>{id}</tool-use-id>\n<status>completed</status>"}}}}"#
    )
}

/// The result of an `Agent` call run in the background, as the transcripts write it (2026-10-09).
fn launched(id: &str) -> String {
    format!(
        r#"{{"type":"user","message":{{"content":[{{"tool_use_id":"{id}","type":"tool_result","content":[{{"type":"text","text":"Async agent launched successfully. (This tool result is internal metadata.)\nagentId: add0a203ccde364b0"}}]}}]}},"toolUseResult":{{"isAsync":true,"status":"async_launched","agentId":"add0a203ccde364b0"}}}}"#
    )
}

#[test]
fn starts_a_task_for_each_subagent_monitor_and_background_call() {
    let line = uses(
        "2026-10-16T20:00:00Z",
        &[
            agent("t1", "general-purpose", "EN-047 worker"),
            r#"{"type":"tool_use","id":"t2","name":"Agent","input":{"description":"x"}}"#.to_owned(),
            r#"{"type":"tool_use","id":"t3","name":"Monitor","input":{"description":"watch"}}"#.to_owned(),
            r#"{"type":"tool_use","id":"t4","name":"Bash","input":{"command":"npm run check:all","run_in_background":true}}"#.to_owned(),
            r#"{"type":"tool_use","id":"t5","name":"Bash","input":{"command":"ls"}}"#.to_owned(),
        ]
        .join(","),
    );
    let tasks = started_tasks(&parse(&line).expect("JSON"));
    let start = 1_792_180_800_000;

    assert_eq!(
        tasks,
        vec![
            Task {
                id: "t1".into(),
                kind: Kind::Agent,
                label: "general-purpose: EN-047 worker".into(),
                start
            },
            Task {
                id: "t2".into(),
                kind: Kind::Agent,
                label: "agent: x".into(),
                start
            },
            Task {
                id: "t3".into(),
                kind: Kind::Command,
                label: "watch".into(),
                start
            },
            Task {
                id: "t4".into(),
                kind: Kind::Command,
                label: "npm run check:all".into(),
                start
            },
        ]
    );
}

#[test]
fn reads_the_ids_a_line_ends() {
    assert_eq!(
        notified_ids(&notification("toolu_01-x")),
        vec!["toolu_01-x"]
    );
    assert_eq!(
        notified_ids(r#"{"content":"<tool-use-id>t1</tool-use-id>"}"#),
        Vec::<String>::new()
    );
    assert_eq!(
        notified_ids("<task-notification><tool-use-id>a b</tool-use-id>"),
        Vec::<String>::new()
    );

    let refused = parse(
        r#"{"message":{"content":[{"type":"tool_result","tool_use_id":"t1","is_error":true},{"type":"tool_result","tool_use_id":"t2"}]}}"#,
    )
    .expect("JSON");

    assert_eq!(failed_ids(&refused), vec!["t1"]);
}

#[test]
fn f18_ac2_follows_the_tasks_running_through_a_transcript() {
    let lines = [
        uses("2026-10-16T20:00:00Z", &agent("t1", "general-purpose", "one")),
        uses("2026-10-16T20:01:00Z", &[agent("t2", "Explore", "two"), agent("t3", "Plan", "three")].join(",")),
        // A launch in the background is no end: a subagent answers at once that it started.
        launched("t1"),
        notification("t2"),
        r#"{"type":"user","message":{"content":[{"type":"tool_result","tool_use_id":"t3","is_error":true}]}}"#.to_owned(),
        "not JSON {".to_owned(),
    ]
    .join("\n");
    let pending = pending_after(Vec::new(), &lines);

    assert_eq!(
        pending
            .iter()
            .map(|task| task.id.as_str())
            .collect::<Vec<_>>(),
        ["t1"]
    );
}

#[test]
fn drops_a_task_older_than_three_hours() {
    let task = Task {
        id: "t1".into(),
        kind: Kind::Agent,
        label: "x".into(),
        start: 1_792_180_800_000 - LOST_TASK_MS,
    };
    let line = uses("2026-10-16T20:00:00Z", &agent("t2", "x", "y"));

    assert_eq!(after_line(vec![task.clone()], &line).len(), 1);
    assert_eq!(
        after_line(
            vec![Task {
                start: task.start + 1,
                ..task
            }],
            &line
        )
        .len(),
        2
    );
}

#[test]
fn f18_ac9_counts_a_subagent_resumed_by_a_message_until_its_notification() {
    // The lines that showed "your turn" for 15 minutes while a worker ran (2026-10-15, 12:07 to
    // 12:22 UTC): a message resumed the stopped EN-039 worker, its end notified under the
    // message's call id.
    let send = |id: &str| {
        uses(
            "2026-10-15T12:07:38Z",
            &format!(
                r#"{{"type":"tool_use","id":"{id}","name":"SendMessage","input":{{"to":"a93498ccc604c403b","summary":"EN-039 audit findings to fix","message":"fix"}}}}"#
            ),
        )
    };
    let result = |id: &str, text: &str| {
        format!(
            r#"{{"type":"user","message":{{"content":[{{"tool_use_id":"{id}","type":"tool_result","content":[{{"type":"text","text":"{text}"}}]}}]}}}}"#
        )
    };
    let resumed = result(
        "toolu_013f",
        r#"{\"success\":true,\"message\":\"Resuming agent a93498c\",\"resumedAgentId\":\"a93498ccc604c403b\"}"#,
    );
    let running = pending_after(Vec::new(), &[send("toolu_013f"), resumed].join("\n"));

    assert_eq!(
        running,
        vec![Task {
            id: "toolu_013f".into(),
            kind: Kind::Agent,
            label: "resumed: EN-039 audit findings to fix".into(),
            start: 1_792_066_058_000,
        }]
    );
    assert_eq!(after_line(running, &notification("toolu_013f")), Vec::new());

    // A message to a subagent still running is only queued: it resumes nothing.
    let asked = after_line(Vec::new(), &send("toolu_01Wc"));

    assert_eq!(asked[0].kind, Kind::Message);
    assert_eq!(
        after_line(
            asked,
            &result(
                "toolu_01Wc",
                r#"{\"success\":true,\"message\":\"Message queued for delivery to a6ab5a436c6a3dc67 at its next tool round.\"}"#
            )
        ),
        Vec::new()
    );
}

#[test]
fn f18_ac2_ends_a_subagent_run_in_the_foreground_with_its_result() {
    let start = uses(
        "2026-10-17T01:00:00Z",
        &[
            agent("fg", "Explore", "in the foreground"),
            agent("bg", "Explore", "in the background"),
            agent("text", "Explore", "launch text only"),
            agent("flag", "Explore", "launch flags only"),
            r#"{"type":"tool_use","id":"cmd","name":"Bash","input":{"command":"npm run test","run_in_background":true}}"#.to_owned(),
        ]
        .join(","),
    );
    // A foreground subagent's result: `status: "completed"` (the hooks reference), never notified.
    let completed = r#"{"type":"user","message":{"content":[{"tool_use_id":"fg","type":"tool_result","content":[{"type":"text","text":"The report."}]}]},"toolUseResult":{"status":"completed","content":[]}}"#;
    // The launch text without `toolUseResult` (a line of 2026-10-16), and the flags without the text.
    let text_only = r#"{"type":"user","message":{"content":[{"tool_use_id":"text","type":"tool_result","content":[{"type":"text","text":"Async agent launched successfully."}]}]}}"#;
    let flags_only = r#"{"type":"user","message":{"content":[{"tool_use_id":"flag","type":"tool_result","content":"started"}]},"toolUseResult":{"isAsync":true}}"#;
    // A command in the background answers at once too: only its notification ends it.
    let command = r#"{"type":"user","message":{"content":[{"tool_use_id":"cmd","type":"tool_result","content":"Command running in background with ID: b1"}]}}"#;
    let lines = [
        start,
        completed.to_owned(),
        launched("bg"),
        text_only.to_owned(),
        flags_only.to_owned(),
        command.to_owned(),
    ]
    .join("\n");

    assert_eq!(
        pending_after(Vec::new(), &lines)
            .iter()
            .map(|task| task.id.as_str())
            .collect::<Vec<_>>(),
        ["bg", "text", "flag", "cmd"]
    );
    assert_eq!(
        completed_ids(&parse(completed).unwrap()),
        vec!["fg".to_owned()]
    );
    assert_eq!(
        completed_ids(&parse(&launched("bg")).unwrap()),
        Vec::<String>::new()
    );
}
