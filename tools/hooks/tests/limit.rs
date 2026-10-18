//! Tests of the subagent limit and of the `limit-agents` hook (EN-047).

use std::fs;
use std::io::Write;
use std::process::Stdio;

use hooks::json::{parse, quote};
use hooks::limit::{AGENT_LIMIT, deny_output, refusal, running_before};
use hooks::tasks::{Kind, Task};

/// A running task.
fn task(id: &str, kind: Kind) -> Task {
    Task {
        id: id.into(),
        kind,
        label: format!("general-purpose: {id}"),
        start: 0,
    }
}

#[test]
fn counts_the_subagents_started_before_the_call() {
    let pending = [
        task("a1", Kind::Agent),
        task("c1", Kind::Command),
        task("a2", Kind::Agent),
        task("a3", Kind::Agent),
    ];
    let ids = |running: Vec<&Task>| {
        running
            .iter()
            .map(|task| task.id.clone())
            .collect::<Vec<_>>()
    };

    // The call already in the transcript, with one more after it in the same message.
    assert_eq!(ids(running_before(&pending, "a2")), ["a1"]);
    // The call not in the transcript yet: every subagent running.
    assert_eq!(ids(running_before(&pending, "new")), ["a1", "a2", "a3"]);
}

#[test]
fn f18_ac2_refuses_a_fourth_subagent_naming_the_three_running() {
    let pending = [
        task("a1", Kind::Agent),
        task("a2", Kind::Agent),
        task("a3", Kind::Agent),
    ];
    let running = running_before(&pending, "a4");

    assert_eq!(AGENT_LIMIT, 3);
    assert_eq!(
        refusal(&running, AGENT_LIMIT, 125_000),
        Some("At most 3 subagents at once (ADR-0063): 3 running — general-purpose: a1 (2 min); general-purpose: a2 (2 min); general-purpose: a3 (2 min). Wait for one to end, then start or resume this one.".to_owned())
    );
    assert_eq!(refusal(&running[..2], AGENT_LIMIT, 0), None);
}

#[test]
fn answers_as_a_pre_tool_use_decision() {
    let output = parse(&deny_output("a \"b\"")).expect("JSON");
    let decision = output.get("hookSpecificOutput");

    assert_eq!(decision.get("hookEventName").text(), "PreToolUse");
    assert_eq!(decision.get("permissionDecision").text(), "deny");
    assert_eq!(decision.get("permissionDecisionReason").text(), "a \"b\"");
}

/// Runs the hook on an `Agent` call in a session whose transcript holds these lines.
fn hook(lines: &[String], id: &str) -> String {
    hook_on(lines, id, "Agent", "{}")
}

/// Runs the hook on a call of this tool, with this input, in a session whose transcript holds
/// these lines.
fn hook_on(lines: &[String], id: &str, tool: &str, tool_input: &str) -> String {
    let folder = std::env::temp_dir().join(format!("limit-agents-{}-{id}", std::process::id()));

    fs::create_dir_all(&folder).expect("folder");
    let transcript = folder.join("t.jsonl");

    fs::write(&transcript, lines.join("\n") + "\n").expect("transcript");

    let input = format!(
        r#"{{"tool_name":"{tool}","tool_input":{tool_input},"tool_use_id":"{id}","cwd":{},"transcript_path":{}}}"#,
        quote(&folder.display().to_string()),
        quote(&transcript.display().to_string())
    );
    let mut child = test_support::command(env!("CARGO_BIN_EXE_limit-agents"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("hook starts");

    child
        .stdin
        .take()
        .expect("stdin")
        .write_all(input.as_bytes())
        .expect("input");
    let output = child.wait_with_output().expect("hook ends");

    fs::remove_dir_all(&folder).expect("cleaned");
    assert!(output.status.success());
    String::from_utf8(output.stdout).expect("UTF-8")
}

/// A line starting one subagent.
fn started(id: &str) -> String {
    format!(
        r#"{{"type":"assistant","timestamp":"2026-10-16T20:00:00Z","message":{{"content":[{{"type":"tool_use","id":"{id}","name":"Agent","input":{{"subagent_type":"general-purpose","description":"{id}"}}}}]}}}}"#
    )
}

#[test]
fn f18_ac2_the_hook_refuses_a_fourth_subagent_and_lets_a_third_through() {
    let three = [started("a1"), started("a2"), started("a3")];
    let refused = parse(&hook(&three, "a4")).expect("a decision");
    let reason = refused
        .get("hookSpecificOutput")
        .get("permissionDecisionReason")
        .text()
        .to_owned();

    assert!(
        reason.contains("3 running — general-purpose: a1"),
        "{reason}"
    );
    assert!(reason.contains("general-purpose: a3"), "{reason}");
    assert_eq!(hook(&three[..2], "a3"), "");
    // A third already written in the transcript before its hook runs.
    assert_eq!(hook(&three, "a3"), "");
}

#[test]
fn the_hook_lets_the_call_through_when_it_cannot_read() {
    assert_eq!(hook(&["{".to_owned()], "a1"), "");

    let mut child = test_support::command(env!("CARGO_BIN_EXE_limit-agents"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("hook starts");

    child
        .stdin
        .take()
        .expect("stdin")
        .write_all(b"not JSON")
        .expect("input");
    let output = child.wait_with_output().expect("hook ends");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");
}

/// A line answering a subagent's start in the background, naming its agent id as on 2026-10-17.
fn launched(id: &str, agent: &str) -> String {
    format!(
        r#"{{"type":"user","timestamp":"2026-10-16T20:00:01Z","message":{{"content":[{{"type":"tool_result","tool_use_id":"{id}","content":[{{"type":"text","text":"Async agent launched successfully.\nagentId: {agent} (internal ID)"}}]}}]}},"toolUseResult":{{"status":"async_launched","isAsync":true}}}}"#
    )
}

/// Three subagents started in the background: `s1` named `writer`, agent ids `aaa1` to `aaa3`.
fn three_launched() -> Vec<String> {
    vec![
        started("s1").replace(
            r#""description":"s1""#,
            r#""description":"s1","name":"writer""#,
        ),
        launched("s1", "aaa1"),
        started("s2"),
        launched("s2", "aaa2"),
        started("s3"),
        launched("s3", "aaa3"),
    ]
}

#[test]
fn f18_ac2_the_hook_refuses_a_message_resuming_a_fourth_subagent() {
    let lines = three_launched();
    // `bbb4` is none of the three running: the message would resume a stopped fourth one.
    let refused = parse(&hook_on(
        &lines,
        "m1",
        "SendMessage",
        r#"{"to":"bbb4","summary":"fix the review","message":"…"}"#,
    ))
    .expect("a decision");
    let reason = refused
        .get("hookSpecificOutput")
        .get("permissionDecisionReason")
        .text()
        .to_owned();

    assert!(
        reason.contains("3 running — general-purpose: s1"),
        "{reason}"
    );
    assert!(reason.contains("start or resume"), "{reason}");
    // Two running: the resume of a third goes through.
    assert_eq!(
        hook_on(&lines[..4], "m2", "SendMessage", r#"{"to":"bbb4"}"#),
        ""
    );
}

#[test]
fn f18_ac2_the_hook_lets_a_message_to_a_running_subagent_or_the_main_thread_through() {
    let lines = three_launched();

    // By agent id, by the name its `Agent` call gave, and to the main conversation: none starts one.
    assert_eq!(hook_on(&lines, "m3", "SendMessage", r#"{"to":"aaa2"}"#), "");
    assert_eq!(
        hook_on(&lines, "m4", "SendMessage", r#"{"to":"writer"}"#),
        ""
    );
    assert_eq!(hook_on(&lines, "m5", "SendMessage", r#"{"to":"main"}"#), "");
    // A prefix of a running agent's id is another agent.
    assert_ne!(hook_on(&lines, "m6", "SendMessage", r#"{"to":"aaa"}"#), "");
}

#[test]
fn reads_the_agent_id_a_resume_names() {
    // A message `m0` resumed agent `ccc9` (the result of 2026-10-16); it now runs under `m0`.
    let resumed = r#"{"type":"user","message":{"content":[{"type":"tool_result","tool_use_id":"m0","content":[{"type":"text","text":"{\"success\":true,\"resumedAgentId\":\"ccc9\"}"}]}]}}"#;
    let running = [
        task("m0", Kind::Agent),
        task("s2", Kind::Agent),
        task("s3", Kind::Agent),
    ];
    let refs: Vec<&Task> = running.iter().collect();

    assert!(hooks::limit::reaches_running("ccc9", &refs, resumed));
    assert!(!hooks::limit::reaches_running("ccc", &refs, resumed));
    assert!(!hooks::limit::reaches_running("", &refs, resumed));
}
