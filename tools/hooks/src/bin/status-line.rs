//! The status line of Claude Code (`docs/tooling/metrics.md`, EN-104), refreshed every second: reads
//! the status line's JSON on stdin (`REF-CC-STATUSLINE`), the session's transcript from where its
//! cache stopped, the calls running and the backlog's file names; writes `now.json` for the
//! `log-tool-call` hook and `usage.json` for a run; prints the lines (`hooks::status`). A refresh
//! is a poll: its use is never recorded (`tool_use::run_or_poll`, EN-102).

use std::fs;
use std::path::Path;
use std::process::{Command, ExitCode, Stdio};
use std::time::UNIX_EPOCH;

use calendar::{SECOND_MS, now_ms};
use git_repository::{branch_of, checkout_of};
use hooks::git::hook_input;
use hooks::guard::is_space;
use hooks::json::{self, Value};
use hooks::record::count;
use hooks::status::{
    LONG_CALL_MS, RunningCall, Status, background_lines, is_run_open, is_turn_done, place_of,
    read_running_call, running_line, session_snapshot, status_text, usage_snapshot,
};
use hooks::time::short_duration;
use hooks::tokens::turn_tokens;

fn main() -> ExitCode {
    tool_use::run_or_poll("status-line", "", true, draw)
}

/// The trimmed text of a file, empty when it is missing.
fn text_of(path: &Path) -> String {
    fs::read_to_string(path)
        .unwrap_or_default()
        .trim_matches(is_space)
        .to_owned()
}

/// The names of a folder's entries, sorted as Node.js lists them; none when it cannot be read.
fn names(folder: &Path) -> Option<Vec<String>> {
    let mut names: Vec<String> = fs::read_dir(folder)
        .ok()?
        .flatten()
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();

    names.sort();
    Some(names)
}

/// The calls running, as the `track-tool-call` hook wrote them, each with its file's name; a file
/// that is not JSON (being written) is left for the next refresh.
fn running_calls(folder: &Path) -> Vec<(String, RunningCall)> {
    names(folder)
        .unwrap_or_default()
        .into_iter()
        .filter(|name| {
            Path::new(name)
                .extension()
                .is_some_and(|extension| extension == "json")
        })
        .filter_map(|name| {
            let record = json::parse(&fs::read_to_string(folder.join(&name)).ok()?)?;

            Some((name, read_running_call(&record)))
        })
        .collect()
}

/// The open run's part of the line, counted by the Rust tool `run-totals` (EN-092), its release
/// binary called by its path (ADR-0078); run only while a run is open; empty when it fails.
fn run_part(root: &Path, marker: &str) -> String {
    if !is_run_open(marker) {
        return String::new();
    }

    Command::new(root.join("target").join("release").join("run-totals"))
        .arg("status")
        .current_dir(root)
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| {
            String::from_utf8_lossy(&output.stdout)
                .trim_matches(is_space)
                .to_owned()
        })
        .unwrap_or_default()
}

/// Milliseconds since a file last changed, 0 when it is missing: Node.js's `mtimeMs` holds a
/// fraction, rounded up here, which leaves the comparison with 2 minutes and the minutes shown as
/// they were.
fn silent_ms(transcript: &str, now: i64) -> i64 {
    let modified = (!transcript.is_empty())
        .then(|| {
            fs::metadata(transcript)
                .and_then(|metadata| metadata.modified())
                .ok()
        })
        .flatten()
        .and_then(|modified| modified.duration_since(UNIX_EPOCH).ok());

    modified.map_or(0, |since| {
        let seconds = i64::try_from(since.as_secs()).unwrap_or(i64::MAX / SECOND_MS);

        now - (seconds * SECOND_MS + i64::from(since.subsec_nanos().div_ceil(1_000_000)))
    })
}

/// Draws the status line; exits 1 when the input is not JSON or the folder is no repository.
fn draw() -> u8 {
    let Some(input) = hook_input("status-line") else {
        return 1;
    };
    let Some(checkout) = checkout_of(Path::new(".")) else {
        eprintln!("status-line: not in a Git repository");
        return 1;
    };
    let folder = checkout.common.join("metrics");
    let root = checkout.top.clone();
    let backlog = root.join("docs").join("backlog");
    let (Some(stories), Some(features)) = (
        names(&backlog.join("stories")),
        names(&backlog.join("features")),
    ) else {
        eprintln!("status-line: the backlog's folders cannot be read");
        return 1;
    };
    let files: Vec<String> = stories.into_iter().chain(features).collect();
    let transcript = input.get("transcript_path").text();
    let now = now_ms();

    if fs::create_dir_all(&folder).is_err() {
        return 1;
    }

    let tally = turn_tokens(transcript, &folder.join("status-line-turn.json"));
    let running = folder.join("running");

    // A call whose result arrived is over, even if its end was never recorded (a scheduled wake-up).
    for id in &tally.finished {
        let _ = fs::remove_file(running.join(format!("{id}.json")));
    }
    // A main call started before the turn never finished (an interrupted turn): its file is
    // removed; a subagent's call may outlive the turn that launched the subagent.
    let turn_start = tally.start;

    for (name, call) in running_calls(&running) {
        if call.agent == "main" && call.start < turn_start {
            let _ = fs::remove_file(running.join(name));
        }
    }

    let all: Vec<RunningCall> = running_calls(&running)
        .into_iter()
        .map(|(_, call)| call)
        .collect();
    let calls: Vec<RunningCall> = if tally.is_waiting {
        Vec::new()
    } else {
        all.iter()
            .filter(|call| call.agent == "main" && call.start >= turn_start)
            .cloned()
            .collect()
    };
    let model = input.get("model").get("display_name");
    // Once the agent ended its turn, the clock stops at its last answer.
    let duration_ms = if tally.start == 0 {
        0
    } else if is_turn_done(&tally) && tally.end != 0 {
        tally.end - tally.start
    } else {
        now - tally.start
    };
    let is_running_long = calls.iter().any(|call| now - call.start > LONG_CALL_MS);
    let status = Status {
        background: background_lines(&tally.pending, &all, now, short_duration),
        context_percent: count(input.get("context_window"), "used_percentage"),
        duration_ms,
        model: match model {
            Value::String(name) => name.clone(),
            _ => String::new(),
        },
        place: place_of(&branch_of(&checkout), &files),
        run: run_part(&root, &text_of(&folder.join("run.json"))),
        is_running_long,
        running: running_line(&calls, now, short_duration),
        silent_ms: silent_ms(transcript, now),
        step: text_of(&folder.join("step")),
        tally,
    };

    let _ = fs::write(folder.join("now.json"), session_snapshot(&status, now));
    if let Some(usage) = usage_snapshot(&input, now) {
        let _ = fs::write(folder.join("usage.json"), usage);
    }
    print!("{}", status_text(&status));
    0
}
