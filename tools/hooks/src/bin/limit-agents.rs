//! `PreToolUse` hook on `Agent` and `SendMessage` (ADR-0072, EN-047, EN-063): refuses a fourth
//! subagent at once, started or resumed, naming the three running; a message to the main
//! conversation or to a subagent running goes through, since it starts none. It counts them from the
//! session's whole transcript. Any failure lets the call through: the hook never blocks work
//! by its own fault.

use std::fs;
use std::io::{self, Read};

use calendar::now_ms;
use hooks::json;
use hooks::limit::{AGENT_LIMIT, deny_output, reaches_running, refusal, running_before};
use hooks::tasks::{Task, pending_after};

/// Background tasks still running in a session, from its transcript's bytes.
fn pending_tasks(bytes: &[u8]) -> Vec<Task> {
    // Complete lines only: the last one may be still being written.
    let complete = bytes
        .iter()
        .rposition(|byte| *byte == b'\n')
        .map_or(&bytes[..0], |end| &bytes[..=end]);

    pending_after(Vec::new(), &String::from_utf8_lossy(complete))
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
    let pending = pending_tasks(&bytes);
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
