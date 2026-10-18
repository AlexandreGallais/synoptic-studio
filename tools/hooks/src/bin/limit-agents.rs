//! `PreToolUse` hook on `Agent` and `SendMessage` (ADR-0063, EN-047, EN-063): refuses a fourth
//! subagent at once, started or resumed, naming the three running; a message to the main
//! conversation or to a subagent running goes through, since it starts none. It counts them from the session's transcript, starting where the status line's
//! cache in `.git/metrics/` stopped. Any failure lets the call through: the hook never blocks work
//! by its own fault.

use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use calendar::now_ms;
use git_repository::common_folder;
use hooks::json::{self, Value};
use hooks::limit::{AGENT_LIMIT, deny_output, reaches_running, refusal, running_before};
use hooks::tasks::{Task, cached_tasks, pending_after};

/// The status line's cache in the Git common directory, when there is one: read from the files Git
/// writes, as the other hooks find it, rather than by starting Git (NAM-002).
fn cache_file(cwd: &str) -> Option<PathBuf> {
    common_folder(Path::new(if cwd.is_empty() { "." } else { cwd })).map(|common| {
        PathBuf::from(common)
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

fn main() -> std::process::ExitCode {
    tool_use::run("limit-agents", "", || {
        hook();
        0
    })
}

/// Reads the hook's input and prints its refusal, if any.
fn hook() {
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
