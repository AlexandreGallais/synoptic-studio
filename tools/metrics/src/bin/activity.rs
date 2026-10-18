//! `activity [count]` (`target/release/activity`, EN-107): what the agents are doing — the calls running
//! now (`.git/metrics/running/`), the latest calls of the main agent and of each subagent from the
//! hook's log (10 each, or `count`), and the last lines of each background command's output of the
//! last hour. Its lines are those of `activity.ts`.

use std::env;
use std::fs;
use std::process::{Command, ExitCode};
use std::time::SystemTime;

use metrics::activity::{activity_lines, logged_call, outputs_folder, recent_outputs};
use metrics::git::common_dir;
use metrics::json::json_lines;
use metrics::session::names_in;

/// The calls kept per agent: the count given, 10 without one, every call for 0 or a text that is
/// not a count.
fn kept(argument: Option<&String>) -> usize {
    argument.map_or(10, |text| {
        text.trim()
            .parse::<f64>()
            .ok()
            .filter(|count| count.is_finite() && *count >= 1.0)
            // A count of calls, far below `usize::MAX`.
            .map_or(0, |count| {
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                let whole = count.trunc() as usize;
                whole
            })
    })
}

/// The user's id, for Claude Code's folder of background outputs.
fn user_id() -> String {
    Command::new("id")
        .arg("-u")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_owned())
        .unwrap_or_default()
}

fn activity(arguments: &[String]) -> Result<String, String> {
    let metrics = common_dir()?.join("metrics");
    let running_folder = metrics.join("running");
    let running: Vec<String> = names_in(&running_folder)
        .iter()
        .map(|name| fs::read_to_string(running_folder.join(name)).unwrap_or_default())
        .collect();
    let calls: Vec<_> =
        json_lines(&fs::read_to_string(metrics.join("tool-calls.jsonl")).unwrap_or_default())
            .iter()
            .map(logged_call)
            .collect();
    let project = env::current_dir().map_err(|error| error.to_string())?;
    let outputs: Vec<(String, String)> =
        recent_outputs(&outputs_folder(&user_id(), &project), SystemTime::now())
            .into_iter()
            .map(|path| {
                let text = fs::read_to_string(&path).unwrap_or_default();

                (path.to_string_lossy().into_owned(), text)
            })
            .collect();

    Ok(activity_lines(
        &running,
        &calls,
        kept(arguments.first()),
        &outputs,
    ))
}

fn main() -> ExitCode {
    let arguments: Vec<String> = env::args().skip(1).collect();

    tool_use::run("activity", "", || match activity(&arguments) {
        Ok(text) => {
            print!("{text}");
            0
        }
        Err(message) => {
            eprintln!("{message}");
            1
        }
    })
}
