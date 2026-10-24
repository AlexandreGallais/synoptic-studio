//! `session-report [session id] [--all] [--markdown] [--run]` (`target/release/session-report`, EN-107):
//! the report of one working session — its duration, its tokens and those of each model, its tool
//! calls and failures, its commits. Without an id, the session whose transcript changed last;
//! without `--all`, the current turn only, from the prompt that started it, saved as one line of
//! `.git/metrics/turns.jsonl`. With `--run`, the open or last run's duration and tokens by kind,
//! counted by `run-totals` (EN-092). The transcripts are those of the main checkout, in a worktree
//! too, read from where the last report left them (`.git/metrics/transcripts-cache.jsonl`).

use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;

use calendar::now_ms;
use calendar::temporal_instant_ms;
use metrics::cache::{CACHE_FILE, read_transcripts};
use metrics::git::{common_dir, git};
use metrics::session::{
    markdown_report, reported_events, session_figures, session_files, terminal_report,
};
use metrics::turns::{render_turn, turn_record};
use run_totals::marker::parse_run;
use run_totals::report::report_table;
use run_totals::totals::{count_run, projects_dir};

/// The report of the open or last run, as `run-totals report` prints it.
fn run_report(common: &std::path::Path, home: &std::path::Path) -> Result<String, String> {
    let metrics = common.join("metrics");
    let run = fs::read_to_string(metrics.join("run.json"))
        .ok()
        .and_then(|text| parse_run(&text))
        .ok_or("run-totals report: no run recorded")?;
    let tokens = count_run(&run, &metrics, &projects_dir(home, common));

    Ok(report_table(
        run.duration_ms(now_ms()),
        &tokens,
        run.is_open(),
    ))
}

/// The report the arguments ask for.
fn report(arguments: &[String]) -> Result<String, String> {
    let has = |option: &str| arguments.iter().any(|argument| argument == option);
    let common = common_dir()?;
    let home = PathBuf::from(env::var("HOME").unwrap_or_default());

    if has("--run") {
        return run_report(&common, &home);
    }

    let session = arguments
        .iter()
        .find(|argument| !argument.starts_with("--"))
        .map_or("", String::as_str);
    let is_all = has("--all");
    let files = session_files(&projects_dir(&home, &common), session);
    let transcripts = read_transcripts(&files, &common.join("metrics").join(CACHE_FILE));
    let events = reported_events(&transcripts, is_all);
    let start = events.requests.iter().map(|request| request.time).min();
    let since = format!("--since={}", temporal_instant_ms(start.unwrap_or(0)));
    let log = git(&["log", "--all", "--reverse", &since, "--format=%h %s"])?;
    let commits = log
        .lines()
        .filter(|line| !line.is_empty())
        .map(str::to_owned)
        .collect();
    let figures = session_figures(&events, commits);

    // Saved for the retrospective's comparison of turns (`target/release/metrics`), unless `--all`.
    if let Some(first) = start.filter(|_| !is_all) {
        let session = events
            .requests
            .first()
            .map_or("", |request| request.session.as_str());
        let metrics = common.join("metrics");
        let line = render_turn(&turn_record(&figures, first, session));

        fs::create_dir_all(&metrics)
            .and_then(|()| {
                OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(metrics.join("turns.jsonl"))
            })
            .and_then(|mut file| file.write_all(line.as_bytes()))
            .map_err(|error| format!("turns.jsonl: {error}"))?;
    }

    Ok(format!(
        "{}\n",
        if has("--markdown") {
            markdown_report(&figures)
        } else {
            terminal_report(&figures)
        }
    ))
}

/// The usage, printed on `--help` or `-h` (REV-009).
const USAGE: &str =
    "usage: target/release/session-report [session id] [--all] [--markdown] [--run]";

fn main() -> ExitCode {
    if let Some(code) = tool_use::usage(&std::env::args().skip(1).collect::<Vec<_>>(), USAGE) {
        return code;
    }
    let arguments: Vec<String> = env::args().skip(1).collect();
    let mode = ["--run", "--all"]
        .into_iter()
        .find(|option| arguments.iter().any(|argument| argument == option))
        .unwrap_or_default();

    tool_use::run("session-report", mode, || match report(&arguments) {
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
