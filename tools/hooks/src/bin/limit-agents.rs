//! `PreToolUse` hook on `Agent` and `SendMessage` (ADR-0063, EN-047, EN-063): refuses a fourth
//! subagent at once, started or resumed, naming the three running; a message to the main
//! conversation or to a subagent running goes through, since it starts none. It counts them from the session's transcript, starting where the status line's
//! cache in `.git/metrics/` stopped. Any failure lets the call through: the hook never blocks work
//! by its own fault.

use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;
use std::process::Command;

use hooks::json::{self, Value};
use hooks::limit::{AGENT_LIMIT, deny_output, reaches_running, refusal, running_before};
use hooks::tasks::{Task, cached_tasks, pending_after};
use hooks::time::now_ms;

/// The status line's cache in the Git common directory, when there is one.
fn cache_file(cwd: &str) -> Option<PathBuf> {
    let output = Command::new("git")
        .args(["rev-parse", "--path-format=absolute", "--git-common-dir"])
        .current_dir(if cwd.is_empty() { "." } else { cwd })
        .output()
        .ok()?;

    output.status.success().then(|| {
        PathBuf::from(String::from_utf8_lossy(&output.stdout).trim())
            .join("metrics")
            .join("status-line-turn.json")
    })
}

/// Background tasks still running in a session, from its transcript's bytes.
fn pending_tasks(input: &Value, bytes: &[u8]) -> Vec<Task> {
    let transcript = input.get("transcript_path").text();
    let cache = cache_file(input.get("cwd").text())
        .and_then(|path| fs::read_to_string(path).ok())
        .and_then(|text| json::parse(&text))
        .unwrap_or(Value::Null);
    let (tasks, offset) = cached_tasks(&cache, transcript);
    let rest = bytes.get(offset..).unwrap_or(bytes);
    // Complete lines only: the last one may be still being written.
    let complete = rest
        .iter()
        .rposition(|byte| *byte == b'\n')
        .map_or(&rest[..0], |end| &rest[..=end]);

    pending_after(tasks, &String::from_utf8_lossy(complete))
}

fn main() {
    let mut text = String::new();

    if io::stdin().read_to_string(&mut text).is_err() {
        return;
    }

    let Some(input) = json::parse(&text) else {
        return;
    };
    let bytes = fs::read(input.get("transcript_path").text()).unwrap_or_default();
    let pending = pending_tasks(&input, &bytes);
    let running = running_before(&pending, input.get("tool_use_id").text());

    if input.get("tool_name").text() == "SendMessage"
        && reaches_running(
            input.get("tool_input").get("to").text(),
            &running,
            &String::from_utf8_lossy(&bytes),
        )
    {
        return;
    }

    if let Some(reason) = refusal(&running, AGENT_LIMIT, now_ms()) {
        print!("{}", deny_output(&reason));
    }
}
