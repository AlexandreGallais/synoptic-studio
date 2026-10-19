//! Tests of what a subagent carries at its first request (EN-065).

use std::fs;
use std::process::Command;

use hooks::context::{Start, first_request, page_tokens, pages_table, tables};
use hooks::json::parse;

/// A transcript as Claude Code writes one: the task, the attachments, then the first answer.
const TRANSCRIPT: &str = concat!(
    r#"{"type":"user","message":{"role":"user","content":"Do EN-065."}}"#,
    "\n",
    r#"{"type":"attachment","attachment":{"type":"skill_listing","content":"- run: x"}}"#,
    "\n",
    r#"{"type":"attachment","attachment":{"type":"instructions","files":[{"path":"/r/CLAUDE.md"},{"path":"/m/memory/MEMORY.md"}]}}"#,
    "\n",
    r#"{"type":"user","message":{"role":"user","content":"a later reminder"}}"#,
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
fn f20_ac1_reads_the_first_request_and_what_came_before_it() {
    let start = first_request(TRANSCRIPT, &meta());

    // 2 + 21 144 + 17 684 = 38 830: the first answer's usage, never the second's. The task is
    // the first user line, 10 characters, never the later reminder's 16.
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
fn counts_a_usage_field_missing_as_zero() {
    let transcript = concat!(
        r#"{"type":"user","message":{"role":"user","content":"Go."}}"#,
        "\n",
        r#"{"type":"assistant","message":{"usage":{"input_tokens":2,"cache_read_input_tokens":40}}}"#,
        "\n",
    );

    // No `cache_creation_input_tokens`: 2 + 0 + 40 = 42 (AUD-022, a mutation of the 0 survived).
    assert_eq!(
        first_request(transcript, &meta()).map(|start| start.tokens),
        Some(42)
    );
}

#[test]
fn reads_nothing_before_the_first_answer() {
    let unanswered = TRANSCRIPT.lines().take(3).collect::<Vec<_>>().join("\n");

    assert_eq!(first_request(&unanswered, &meta()), None);
    assert_eq!(first_request("", &meta()), None);
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
    // The workers' median, `tool_use::median`: (7 + 9) / 2 = 8 (AUD-035, F6).
    assert!(text.contains("| auditor | 1 | 4 |\n| worker | 2 | 8 |\n"));
    // One row per type, never one per agent.
    assert_eq!(text.matches("| worker | 2 | 8 |").count(), 1);
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

#[test]
fn f32_ac2_counts_a_page_tokens_at_the_ratio_measured() {
    // 230 characters at 2.30 per token: 100 tokens.
    assert_eq!(page_tokens(230), 100);
    // 23 characters: 10 tokens; 2 characters are 0.87 token, rounded up to 1; 1 is 0.43, down.
    assert_eq!(page_tokens(23), 10);
    assert_eq!(page_tokens(2), 1);
    assert_eq!(page_tokens(1), 0);
    assert_eq!(page_tokens(0), 0);
    // 8 characters are 3.48 tokens, rounded down to 3; 9 are 3.91, up to 4.
    assert_eq!(page_tokens(8), 3);
    assert_eq!(page_tokens(9), 4);
    // EN-097's pages: 170,731 characters at 2.30 are 74,230.87 tokens, 74,231 (74,118 measured).
    assert_eq!(page_tokens(170_731), 74_231);
}

#[test]
fn f32_ac2_writes_a_row_per_page_and_their_total() {
    let pages = [
        ("docs/a.md".to_owned(), "x".repeat(230)),
        // 23 characters, the accented one counted once: 10 tokens.
        ("docs/é.md".to_owned(), format!("é{}", "y".repeat(22))),
    ];

    // The total: 253 characters, 110 tokens.
    assert_eq!(
        pages_table(&pages),
        "| File | Characters | Tokens, at 2.30 characters per token |\n| --- | --- | --- |\n\
         | docs/a.md | 230 | 100 |\n| docs/é.md | 23 | 10 |\n| Total | 253 | 110 |\n"
    );
}

#[test]
fn f32_ac2_the_command_counts_the_files_named() {
    let folder = std::env::temp_dir().join(format!("context-size-files-{}", std::process::id()));
    let page = folder.join("page.md");
    let subagents = folder.join("session").join("subagents");

    fs::create_dir_all(&subagents).unwrap_or_default();
    fs::write(&page, "z".repeat(460)).unwrap_or_default();
    fs::write(subagents.join("agent-a1.jsonl"), TRANSCRIPT).unwrap_or_default();

    let output = |arguments: &[&std::path::Path]| {
        Command::new(env!("CARGO_BIN_EXE_context-size"))
            .args(arguments)
            .output()
            .map(|output| String::from_utf8_lossy(&output.stdout).into_owned())
            .unwrap_or_default()
    };
    let alone = output(&[&page]);
    let both = output(&[&folder.join("session"), &page]);

    fs::remove_dir_all(&folder).unwrap_or_default();
    // 460 characters: 200 tokens. A file alone prints no table of subagents.
    assert!(alone.contains(&format!("| {} | 460 | 200 |\n", page.display())));
    assert!(!alone.contains("Agent type"));
    assert!(both.contains("|  |  | 38830 | CLAUDE.md, MEMORY.md | 8 | 10 |"));
    assert!(both.contains("| Total | 460 | 200 |"));
}
