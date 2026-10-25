//! The status line of Claude Code (`REF-CC-STATUSLINE`, AUD-051), refreshed every second: reads its
//! JSON on stdin and the session's transcript from where its cache stopped (`hooks::tokens`, the
//! cache the `limit-agents` hook reads too), writes `now.json` for the `log-tool-call` hook, and
//! prints whether the agent works or the chat is free (`hooks::status`). A refresh is a poll: its
//! use is never recorded (`tool_use::run_or_poll`, EN-102).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use calendar::now_ms;
use git_repository::common_folder;
use hooks::git::hook_input;
use hooks::json::Value;
use hooks::record::count;
use hooks::status::{session_snapshot, status_text};
use hooks::tokens::turn_tokens;

fn main() -> ExitCode {
    tool_use::run_or_poll("status-line", "", true, draw)
}

/// Draws the status line; exits 1 when the input is not JSON or the folder is no repository.
fn draw() -> u8 {
    let Some(input) = hook_input("status-line") else {
        return 1;
    };
    let Some(common) = common_folder(Path::new(".")) else {
        eprintln!("status-line: not in a Git repository");
        return 1;
    };
    let folder = PathBuf::from(common).join("metrics");

    if fs::create_dir_all(&folder).is_err() {
        return 1;
    }

    let tally = turn_tokens(
        input.get("transcript_path").text(),
        &folder.join("status-line-turn.json"),
    );
    let now = now_ms();
    let model = match input.get("model").get("display_name") {
        Value::String(name) => name.as_str(),
        _ => "",
    };
    let context_percent = count(input.get("context_window"), "used_percentage");

    let _ = fs::write(
        folder.join("now.json"),
        session_snapshot(&tally, context_percent, model, now),
    );
    print!("{}", status_text(&tally, now));
    0
}
