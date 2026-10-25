//! `run-totals start [session] | end | report` (by the agent and `session-report --run`; EN-092).
//! `start` opens the run's marker `.git/metrics/run.json`, or adds the session to the run already
//! open; the session is the argument, else `CLAUDE_CODE_SESSION_ID`. `end` closes it and prints the
//! report. `report` prints the duration and the tokens by kind of the open or last run. The status
//! line's part and `should-stop` went with the status line's run indicator and the stop at 90 % of
//! the week (AUD-051).

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use calendar::now_ms;
use git_repository::checkout_of;
use run_totals::marker::{Run, closed, opened, parse_run, render_run};
use run_totals::report::report_table;
use run_totals::tally::Tokens;
use run_totals::totals::{count_run, projects_dir};

/// The Git directory shared by every worktree, absolute, read from the files Git writes rather
/// than by starting Git (NAM-002).
fn common_dir() -> Result<PathBuf, String> {
    checkout_of(Path::new("."))
        .map(|found| found.common)
        .ok_or_else(|| "run-totals: not in a Git repository".to_owned())
}

/// The run's tokens, the cache read before and written after.
fn counted(run: &Run, metrics: &Path, common: &Path) -> Tokens {
    let home = PathBuf::from(env::var("HOME").unwrap_or_default());

    count_run(run, metrics, &projects_dir(&home, common))
}

fn run(words: &[String]) -> Result<String, String> {
    let common = common_dir()?;
    let metrics = common.join("metrics");
    let marker = metrics.join("run.json");
    let current = fs::read_to_string(&marker)
        .ok()
        .and_then(|text| parse_run(&text));
    let now = now_ms();

    match words.first().map(String::as_str) {
        Some("start") => {
            let session = words
                .get(1)
                .cloned()
                .or_else(|| env::var("CLAUDE_CODE_SESSION_ID").ok())
                .filter(|session| !session.is_empty())
                .ok_or("run-totals start: no session id (argument or CLAUDE_CODE_SESSION_ID)")?;
            let next = opened(current.as_ref(), &session, now);

            fs::create_dir_all(&metrics).map_err(|error| error.to_string())?;
            fs::write(&marker, render_run(&next)).map_err(|error| error.to_string())?;

            Ok(if next.start == now {
                format!("run opened, session {session}\n")
            } else {
                format!(
                    "run already open, session {session} counted ({} in all)\n",
                    next.sessions.len()
                )
            })
        }
        Some("end") => {
            let open = current.ok_or("run-totals end: no run to end")?;
            let ended = closed(&open, now);

            fs::write(&marker, render_run(&ended)).map_err(|error| error.to_string())?;

            let tokens = counted(&ended, &metrics, &common);

            Ok(report_table(ended.duration_ms(now), &tokens, false))
        }
        Some("report") => {
            let last = current.ok_or("run-totals report: no run recorded")?;
            let tokens = counted(&last, &metrics, &common);

            Ok(report_table(last.duration_ms(now), &tokens, last.is_open()))
        }
        _ => Err(USAGE.to_owned()),
    }
}

/// The usage, printed on `--help` or `-h` (REV-009).
const USAGE: &str = "usage: target/release/run-totals start [session] | end | report";

fn main() -> ExitCode {
    if let Some(code) = tool_use::usage(&std::env::args().skip(1).collect::<Vec<_>>(), USAGE) {
        return code;
    }
    let words: Vec<String> = env::args().skip(1).collect();
    let subcommand = words.first().map_or("", String::as_str);

    tool_use::run("run-totals", subcommand, || answer(&words))
}

/// Answers the subcommand; its exit code.
fn answer(words: &[String]) -> u8 {
    match run(words) {
        Ok(text) => {
            print!("{text}");
            0
        }
        Err(message) => {
            eprintln!("{message}");
            1
        }
    }
}
