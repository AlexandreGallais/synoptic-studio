//! `run-totals start [session] | end | report | status` (`npm run run:start`, `run:end`,
//! `metrics:session -- --run`, the status line; EN-092). `start` opens the run's marker
//! `.git/metrics/run.json`, or adds the session to the run already open; the session is the
//! argument, else `CLAUDE_CODE_SESSION_ID`. `end` closes it and prints the report. `report` prints
//! the duration and the tokens by kind of the open or last run. `status` prints the status line's
//! part while a run is open, nothing otherwise. `should-stop` (`npm run run:should-stop`, EN-086)
//! prints `stop` and exits 1 once the week's usage reaches 90 %, else `go` and exits 0, then the
//! parked stories of the working tree's backlog; 2 outside a repository.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use calendar::now_ms;
use hooks::json::{Value, parse};
use run_totals::marker::{Run, closed, opened, parse_run, render_run, whole};
use run_totals::report::{report_table, status_text};
use run_totals::stop::{parked_story, stop_text};
use run_totals::tally::Tokens;
use run_totals::totals::{count_run, projects_dir};

/// The Git directory shared by every worktree, absolute.
fn common_dir() -> Result<PathBuf, String> {
    let output = Command::new("git")
        .args(["rev-parse", "--path-format=absolute", "--git-common-dir"])
        .output()
        .map_err(|error| format!("git: {error}"))?;

    if !output.status.success() {
        return Err("run-totals: not in a Git repository".to_owned());
    }

    Ok(PathBuf::from(
        String::from_utf8_lossy(&output.stdout).trim().to_owned(),
    ))
}

/// The run's tokens, the cache read before and written after.
fn counted(run: &Run, metrics: &Path, common: &Path) -> Tokens {
    let home = PathBuf::from(env::var("HOME").unwrap_or_default());

    count_run(run, metrics, &projects_dir(&home, common))
}

/// The week's usage in percent, from `usage.json`, when the status line wrote one.
fn week_percent(metrics: &Path) -> Option<i64> {
    let json = parse(&fs::read_to_string(metrics.join("usage.json")).ok()?)?;

    match json.get("sevenDay").get("usedPercent") {
        Value::Number(percent) => Some(whole(&Value::Number(percent.round()))),
        _ => None,
    }
}

/// The working tree's root, absolute: a worktree's own, so that its branch's stories are read.
fn top_level() -> Result<PathBuf, String> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .map_err(|error| format!("git: {error}"))?;

    if !output.status.success() {
        return Err("run-totals: not in a Git repository".to_owned());
    }

    Ok(PathBuf::from(
        String::from_utf8_lossy(&output.stdout).trim().to_owned(),
    ))
}

/// The ids of the parked stories of this working tree's backlog, sorted.
fn parked_stories(root: &Path) -> Vec<String> {
    let mut parked: Vec<String> = fs::read_dir(root.join("docs/backlog/stories"))
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|entry| fs::read_to_string(entry.path()).ok())
        .filter_map(|text| parked_story(&text))
        .collect();

    parked.sort();
    parked
}

/// `should-stop`: whether the run stops, from the week's usage, and the parked stories.
fn should_stop() -> Result<(bool, String), String> {
    let metrics = common_dir()?.join("metrics");
    let parked = parked_stories(&top_level()?);

    Ok(stop_text(week_percent(&metrics), &parked))
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
        Some("status") => Ok(match current {
            Some(open) if open.is_open() => {
                let tokens = counted(&open, &metrics, &common);

                status_text(open.duration_ms(now), &tokens, week_percent(&metrics))
            }
            _ => String::new(),
        }),
        _ => Err(
            "usage: run-totals start [session] | end | report | status | should-stop".to_owned(),
        ),
    }
}

fn main() -> ExitCode {
    let words: Vec<String> = env::args().skip(1).collect();
    let subcommand = words.first().map_or("", String::as_str);

    tool_use::run_or_poll(
        "run-totals",
        subcommand,
        run_totals::is_poll(subcommand),
        || answer(&words),
    )
}

/// Answers the subcommand; its exit code.
fn answer(words: &[String]) -> u8 {
    if words.first().map(String::as_str) == Some("should-stop") {
        return match should_stop() {
            Ok((stops, text)) => {
                print!("{text}");
                u8::from(stops)
            }
            Err(message) => {
                eprintln!("{message}");
                2
            }
        };
    }

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
