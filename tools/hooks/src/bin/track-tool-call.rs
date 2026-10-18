//! `PreToolUse` hook on every tool, in the background (`docs/tooling/metrics.md`, EN-104): writes
//! the tool call that starts to `.git/metrics/running/<id>.json`, so that the status line shows
//! what is running and for how long; the `log-tool-call` hook removes it when the call ends.

use std::fs;
use std::path::Path;
use std::process::ExitCode;

use calendar::now_ms;
use hooks::git::{common_folder, hook_input};
use hooks::json::{Value, number_text, write};
use hooks::record::running_call;

fn main() -> ExitCode {
    tool_use::run("track-tool-call", "", hook)
}

/// The call's id as JavaScript's `String()` wrote it: `undefined` when missing.
fn call_id(input: &Value) -> String {
    let Value::Object(members) = input else {
        return String::new();
    };

    match members.iter().find(|(name, _)| name == "tool_use_id") {
        None => "undefined".to_owned(),
        Some((_, Value::String(id))) => id.clone(),
        Some((_, Value::Number(number))) => number_text(*number),
        Some((_, other)) => write(other),
    }
}

/// Writes the call starting; exits 1 when the input is not JSON or Git finds no repository.
fn hook() -> u8 {
    let Some(input) = hook_input("track-tool-call") else {
        return 1;
    };
    let Some(common) = common_folder(Path::new(".")) else {
        return 1;
    };
    let folder = Path::new(&common).join("metrics").join("running");

    if fs::create_dir_all(&folder).is_err() {
        return 1;
    }
    let written = fs::write(
        folder.join(format!("{}.json", call_id(&input))),
        running_call(&input, now_ms()),
    );

    u8::from(written.is_err())
}
