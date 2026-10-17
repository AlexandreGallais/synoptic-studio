//! Tests of what a subagent carries at its first request (EN-065).

use std::fs;
use std::process::Command;

use hooks::context::{Start, first_request, median, tables};
use hooks::json::parse;

/// A transcript as Claude Code writes one: the task, the attachments, then the first answer.
const TRANSCRIPT: &str = concat!(
    r#"{"type":"user","message":{"role":"user","content":"Do EN-065."}}"#,
    "\n",
    r#"{"type":"attachment","attachment":{"type":"skill_listing","content":"- run: x"}}"#,
    "\n",
    r#"{"type":"attachment","attachment":{"type":"instructions","files":[{"path":"/r/CLAUDE.md"},{"path":"/m/memory/MEMORY.md"}]}}"#,
    "\n",
    r#"{"type":"user","message":{"role":"user","content":"a reminder"}}"#,
    "\n",
    r#"{"type":"assistant","message":{"usage":{"input_tokens":2,"cache_creation_input_tokens":21144,"cache_read_input_tokens":17684}}}"#,
    "\n",
    r#"{"type":"assistant","message":{"usage":{"input_tokens":5,"cache_creation_input_tokens":1,"cache_read_input_tokens":1}}}"#,
    "\n",
);

fn meta() -> hooks::json::Value {
    parse(r#"{"agentType":"story-worker","description":"EN-065 | worker"}"#)
        .unwrap_or(hooks::json::Value::Null)
}

#[test]
fn reads_the_first_request_and_what_came_before_it() {
    let start = first_request(TRANSCRIPT, &meta());

    // 2 + 21 144 + 17 684 = 38 830: the first answer's usage, never the second's.
    assert_eq!(
        start,
        Some(Start {
            agent_type: "story-worker".into(),
            description: "EN-065 | worker".into(),
            tokens: 38_830,
            instructions: vec!["CLAUDE.md".into(), "MEMORY.md".into()],
            listing: 8,
            task: 10,
        })
    );
}

#[test]
fn reads_nothing_before_the_first_answer() {
    let unanswered = TRANSCRIPT.lines().take(3).collect::<Vec<_>>().join("\n");

    assert_eq!(first_request(&unanswered, &meta()), None);
    assert_eq!(first_request("", &meta()), None);
}

#[test]
fn takes_the_lower_middle_as_median() {
    assert_eq!(median(&[]), 0);
    assert_eq!(median(&[7]), 7);
    // 3 values: the middle one once sorted.
    assert_eq!(median(&[30, 10, 20]), 20);
    // 4 values: the lower of the two middles.
    assert_eq!(median(&[40, 10, 30, 20]), 20);
}

#[test]
fn lists_each_agent_then_each_type() {
    let start = |agent_type: &str, tokens| Start {
        agent_type: agent_type.into(),
        description: "a | b".into(),
        tokens,
        instructions: vec![],
        listing: 0,
        task: 3,
    };
    let text = tables(&[start("worker", 9), start("auditor", 4), start("worker", 7)]);

    assert!(text.contains("| worker | a \\| b | 9 |  | 0 | 3 |\n"));
    assert!(text.contains("| auditor | 1 | 4 |\n| worker | 2 | 7 |\n"));
}

#[test]
fn the_command_reads_a_session_folder() {
    let folder = std::env::temp_dir().join(format!("context-size-{}", std::process::id()));
    let subagents = folder.join("subagents");

    fs::create_dir_all(&subagents).unwrap_or_default();
    fs::write(subagents.join("agent-a1.jsonl"), TRANSCRIPT).unwrap_or_default();
    fs::write(
        subagents.join("agent-a1.meta.json"),
        r#"{"agentType":"researcher","description":"SP-022"}"#,
    )
    .unwrap_or_default();
    fs::write(subagents.join("notes.txt"), "not a transcript").unwrap_or_default();

    let output = Command::new(env!("CARGO_BIN_EXE_context-size"))
        .arg(&folder)
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).into_owned())
        .unwrap_or_default();

    fs::remove_dir_all(&folder).unwrap_or_default();
    assert!(output.contains("| researcher | SP-022 | 38830 | CLAUDE.md, MEMORY.md | 8 | 10 |"));
    assert!(output.contains("| researcher | 1 | 38830 |"));
}
