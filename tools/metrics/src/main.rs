//! `metrics [--since <YYYY-MM-DD>]` (`target/release/metrics`, EN-107): the Markdown report of the
//! agent's work, from Claude Code's transcripts of the main checkout (read from where the last
//! report left them, `.git/metrics/transcripts-cache.jsonl`), the hook's log of each call, the
//! uses each Rust tool records, the saved turns, the Git log, the names the backlog's files had,
//! read from the records (EN-154), and the library's uses; with `--since`, what happened from that
//! day on, 00:00 UTC.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use library::store::{library_file_of_process, open_library};
use library::usage::usage_report;
use metrics::cache::{CACHE_FILE, read_transcripts};
use metrics::git::{common_dir, git};
use metrics::json::json_lines;
use metrics::report::{
    FEATURE_NAMES, ReportInputs, STORY_NAMES, hook_durations, metrics_report, since_start,
    transcript_files,
};
use metrics::turns::read_turn;
use run_totals::totals::projects_dir;

/// The lines of a JSON Lines file of `.git/metrics/`, none when it is missing.
fn metrics_lines(metrics: &Path, file: &str) -> Vec<serde_json::Value> {
    json_lines(&fs::read_to_string(metrics.join(file)).unwrap_or_default())
}

/// The library's report of its uses, every one since its start, or what kept it from being read.
fn sources() -> String {
    let Some(file) = library_file_of_process() else {
        return "No library file: set AGENT_LIBRARY_FILE or HOME".to_owned();
    };

    open_library(&file)
        .and_then(|database| usage_report(&database, 0))
        .unwrap_or_else(|error| format!("Error: {error}"))
}

/// The report the options ask for.
fn report(options: &[String]) -> Result<String, String> {
    let since = since_start(options)?;
    let common = common_dir()?;
    let metrics = common.join("metrics");
    let root = PathBuf::from(git(&["rev-parse", "--show-toplevel"])?.trim());
    let home = PathBuf::from(env::var("HOME").unwrap_or_default());
    let files = transcript_files(&projects_dir(&home, &common));
    let logged = metrics_lines(&metrics, "tool-calls.jsonl");
    let exact = hook_durations(&logged);
    let inputs = ReportInputs {
        transcripts: read_transcripts(&files, &metrics.join(CACHE_FILE)),
        uses: metrics_lines(&metrics, "tool-uses.jsonl"),
        turns: metrics_lines(&metrics, "turns.jsonl")
            .iter()
            .map(read_turn)
            .collect(),
        log: git(&["log", "--all", "--format=%H%x1f%aI%x1f%s%x1f%b%x1e"])?,
        stories: recorded_names(&root, STORY_NAMES),
        features: recorded_names(&root, FEATURE_NAMES),
        sources: sources(),
        since,
        logged,
    };

    Ok(metrics_report(&inputs, &exact))
}

/// The usage, printed on `--help` or `-h` (REV-009).
const USAGE: &str = "usage: target/release/metrics [--since <YYYY-MM-DD>]";

/// The names a statement of the records gives, each row's first cell; none when the records
/// cannot be read: the report then counts no feature nor epic.
fn recorded_names(root: &Path, statement: &str) -> Vec<String> {
    knowledge_call::rows(root, statement)
        .map(|rows| {
            rows.into_iter()
                .filter_map(|row| row.into_iter().next())
                .collect()
        })
        .unwrap_or_default()
}

fn main() -> ExitCode {
    if let Some(code) = tool_use::usage(&std::env::args().skip(1).collect::<Vec<_>>(), USAGE) {
        return code;
    }
    let options: Vec<String> = env::args().skip(1).collect();
    let mode = if options.iter().any(|option| option == "--since") {
        "--since"
    } else {
        ""
    };

    tool_use::run("metrics", mode, || match report(&options) {
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
