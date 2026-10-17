//! `run-totals start [session] | end` (`npm run run:start`, `run:end`; EN-092). `start` opens the run's marker
//! `.git/metrics/run.json`, or adds the session to the run already open; the session is the
//! argument, else `CLAUDE_CODE_SESSION_ID`. `end` closes it.

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, ExitCode};

use hooks::time::now_ms;
use run_totals::marker::{closed, opened, parse_run, render_run};

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

            Ok(format!("run ended after {} ms\n", ended.duration_ms(now)))
        }
        _ => Err("usage: run-totals start [session] | end".to_owned()),
    }
}

fn main() -> ExitCode {
    let words: Vec<String> = env::args().skip(1).collect();

    match run(&words) {
        Ok(text) => {
            print!("{text}");
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}
