//! `PostToolUse` and `PostToolUseFailure` hook on every tool, in the background (RET-001,
//! `docs/tooling/metrics.md`, EN-104): appends one line per tool call to
//! `.git/metrics/tool-calls.jsonl` — what ran, who ran it, when it started and ended, the size of
//! what it read and returned, whether it failed, and the session's state at that moment as the
//! status line last saw it — then removes the call's running file. `target/release/metrics` reads it.
//! On `SubagentStop`, of any agent type, it removes the running files the subagent left and logs
//! nothing (EN-120).

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use calendar::now_ms;
use git_repository::{branch_of, checkout_of};
use hooks::git::hook_input;
use hooks::json::{self, Value};
use hooks::record::{CallContext, count, is_call_of, tool_call_record};

fn main() -> ExitCode {
    tool_use::run("log-tool-call", "", hook)
}

/// The parsed JSON of a file, `Null` when it is missing or unreadable.
fn json_of(path: &Path) -> Value {
    fs::read_to_string(path)
        .ok()
        .and_then(|text| json::parse(&text))
        .unwrap_or(Value::Null)
}

/// Removes the running files of a subagent that stopped: a call refused before it ran, or cut by
/// the subagent's end, gets no `PostToolUse` to remove its file (EN-120).
fn remove_calls_of(running: &Path, agent_id: &str) {
    for entry in fs::read_dir(running).into_iter().flatten().flatten() {
        if is_call_of(&json_of(&entry.path()), agent_id) {
            let _ = fs::remove_file(entry.path());
        }
    }
}

/// Appends the call's record, or on a subagent's stop removes its calls still shown running;
/// exits 1 when the input is not JSON. Outside a repository, nothing.
fn hook() -> u8 {
    let Some(input) = hook_input("log-tool-call") else {
        return 1;
    };
    let cwd = match input.get("cwd").text() {
        "" => PathBuf::from("."),
        folder => PathBuf::from(folder),
    };
    let Some(checkout) = checkout_of(&cwd) else {
        return 0;
    };
    let folder = checkout.common.join("metrics");

    if input.get("hook_event_name").text() == "SubagentStop" {
        remove_calls_of(&folder.join("running"), input.get("agent_id").text());
        return 0;
    }
    let running = folder
        .join("running")
        .join(format!("{}.json", input.get("tool_use_id").text()));
    let step = fs::read_to_string(folder.join("step")).unwrap_or_default();
    let context = CallContext {
        branch: branch_of(&checkout),
        snapshot: json_of(&folder.join("now.json")),
        started_at: count(&json_of(&running), "start"),
        step: step.split('\n').next().unwrap_or_default().to_owned(),
        time: now_ms(),
    };
    let line = format!("{}\n", tool_call_record(&input, &context));
    let appended = fs::create_dir_all(&folder).and_then(|()| {
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(folder.join("tool-calls.jsonl"))?
            .write_all(line.as_bytes())
    });

    // The call has ended: the status line no longer shows it (track-tool-call hook).
    let _ = fs::remove_file(&running);
    u8::from(appended.is_err())
}
