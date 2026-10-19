//! Report of one working session (`docs/tooling/metrics.md`): how long it took, how many tokens it
//! used and by which model (EN-066), how many tool calls and failures, which commits it made.
//! Without `--all`, the current turn only: from the prompt that started it.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use run_totals::tally::Tokens;

use crate::format::{duration_text, grouped};
use crate::transcript::{Events, Transcript, all_events};
use crate::work::active_ms;

/// Figures of one session.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SessionFigures {
    /// Active time, milliseconds.
    pub active_ms: i64,
    /// From its first moment to its last, milliseconds.
    pub wall_ms: i64,
    /// Tokens of every request, by kind.
    pub tokens: Tokens,
    /// Tokens per model that answered, the most first.
    pub tokens_by_model: Vec<(String, i64)>,
    /// Model requests.
    pub requests: usize,
    /// Tool calls answered.
    pub calls: usize,
    /// Tool calls that failed.
    pub failures: usize,
    /// One line per commit made meanwhile: `<short hash> <subject>`.
    pub commits: Vec<String>,
}

/// ANSI styles of the terminal report.
const BOLD: &str = "\u{1b}[1m";
const CYAN: &str = "\u{1b}[36m";
const DIM: &str = "\u{1b}[2m";
const GREEN: &str = "\u{1b}[32m";
const MAGENTA: &str = "\u{1b}[35m";
const RESET: &str = "\u{1b}[0m";

/// Spaces at least between a label and its value in the terminal report.
const GAP: usize = 4;

/// Commits listed by name in a report; beyond, only their count.
const COMMITS_LISTED: usize = 8;

/// The tokens each model used, by kind: Claude Code's count of the session when the events hold
/// it, else each request's, in the order they came.
fn counted_usage(events: &Events) -> Vec<(String, Tokens)> {
    if events.model_usage.is_empty() {
        events
            .requests
            .iter()
            .map(|request| (request.model.clone(), request.tokens))
            .collect()
    } else {
        events.model_usage.clone()
    }
}

/// Tokens of every kind per model that answered, the most first, a model without tokens left
/// out; models of equal tokens in the order they first answered. Claude Code's count of the
/// session when the events hold it (EN-120), else the requests' sums: a Sonnet subagent's
/// transcript holds a small part of what it wrote (CHK-021).
#[must_use]
pub fn tokens_by_model(events: &Events) -> Vec<(String, i64)> {
    let mut totals: Vec<(String, i64)> = Vec::new();

    for (model, tokens) in counted_usage(events) {
        match totals.iter_mut().find(|(known, _)| *known == model) {
            Some((_, total)) => *total += tokens.total(),
            None => totals.push((model, tokens.total())),
        }
    }

    totals.retain(|(_, total)| *total > 0);
    totals.sort_by_key(|(_, total)| std::cmp::Reverse(*total));
    totals
}

/// The tokens of each model on one line, its id without the `claude-` every one starts with:
/// `opus-5-5 1,253 · sonnet-5-5 279`; `unknown` for a request naming no model, `none` without a
/// request.
#[must_use]
pub fn model_tokens(totals: &[(String, i64)]) -> String {
    if totals.is_empty() {
        return "none".to_owned();
    }

    totals
        .iter()
        .map(|(model, total)| {
            let name = if model.is_empty() {
                "unknown"
            } else {
                model.strip_prefix("claude-").unwrap_or(model)
            };

            format!("{name} {}", grouped(*total))
        })
        .collect::<Vec<_>>()
        .join(" · ")
}

/// Figures of a session from its events and the commits made meanwhile.
#[must_use]
pub fn session_figures(events: &Events, commits: Vec<String>) -> SessionFigures {
    let times: Vec<i64> = events
        .requests
        .iter()
        .map(|request| request.time)
        .chain(events.calls.iter().map(|call| call.answer.end))
        .collect();
    let wall_ms = match (times.iter().min(), times.iter().max()) {
        (Some(first), Some(last)) => last - first,
        _ => 0,
    };

    SessionFigures {
        active_ms: active_ms(&times),
        wall_ms,
        tokens: counted_usage(events)
            .iter()
            .fold(Tokens::default(), |sum, (_, tokens)| sum.plus(tokens)),
        tokens_by_model: tokens_by_model(events),
        requests: events.requests.len(),
        calls: events.calls.len(),
        failures: events
            .calls
            .iter()
            .filter(|call| !call.answer.is_ok)
            .count(),
        commits,
    }
}

/// Rows of the report, label and value: duration, tokens, tokens per model, tokens written, tool
/// calls, commits.
#[must_use]
pub fn report_rows(figures: &SessionFigures) -> Vec<(&'static str, String)> {
    vec![
        (
            "Duration",
            format!(
                "{} (active {})",
                duration_text(figures.wall_ms),
                duration_text(figures.active_ms)
            ),
        ),
        ("Tokens", grouped(figures.tokens.total())),
        ("Per model", model_tokens(&figures.tokens_by_model)),
        ("Written", grouped(figures.tokens.output)),
        (
            "Tool calls",
            format!(
                "{} ({} failed)",
                grouped(i64::try_from(figures.calls).unwrap_or(i64::MAX)),
                grouped(i64::try_from(figures.failures).unwrap_or(i64::MAX))
            ),
        ),
        (
            "Commits",
            grouped(i64::try_from(figures.commits.len()).unwrap_or(i64::MAX)),
        ),
    ]
}

/// The commits of a report: each one when few, else none (the count is in the table).
fn listed_commits(commits: &[String]) -> &[String] {
    if commits.len() <= COMMITS_LISTED {
        commits
    } else {
        &[]
    }
}

/// The report as a one-row Markdown table, for a chat message, then the commits when they are few.
#[must_use]
pub fn markdown_report(figures: &SessionFigures) -> String {
    let rows = report_rows(figures);
    let mut lines = vec![
        format!(
            "| {} |",
            rows.iter()
                .map(|(label, _)| *label)
                .collect::<Vec<_>>()
                .join(" | ")
        ),
        format!("| {} |", vec!["---"; rows.len()].join(" | ")),
        format!(
            "| {} |",
            rows.iter()
                .map(|(_, value)| value.as_str())
                .collect::<Vec<_>>()
                .join(" | ")
        ),
    ];
    let commits = listed_commits(&figures.commits);

    if !commits.is_empty() {
        lines.push(String::new());
        lines.extend(commits.iter().map(|commit| format!("- `{commit}`")));
    }

    lines.join("\n")
}

/// The report for a terminal, framed and colored.
#[must_use]
pub fn terminal_report(figures: &SessionFigures) -> String {
    let rows = report_rows(figures);
    let width = rows
        .iter()
        .map(|(label, value)| label.chars().count() + value.chars().count())
        .max()
        .unwrap_or(0)
        + GAP;
    let rule = format!("{DIM}{}{RESET}", "─".repeat(width));
    let mut lines = vec![
        format!("{MAGENTA}{BOLD}Session report{RESET}"),
        rule.clone(),
    ];

    lines.extend(rows.iter().map(|(label, value)| {
        let space = width - label.chars().count() - value.chars().count();

        format!(
            "{CYAN}{label}{RESET}{}{BOLD}{value}{RESET}",
            " ".repeat(space)
        )
    }));
    lines.push(rule);
    lines.extend(
        listed_commits(&figures.commits)
            .iter()
            .map(|commit| format!("{GREEN}●{RESET} {commit}")),
    );
    lines.join("\n")
}

/// The time a file was last changed, the oldest possible when unknown.
fn modified(path: &Path) -> SystemTime {
    fs::metadata(path)
        .and_then(|data| data.modified())
        .unwrap_or(SystemTime::UNIX_EPOCH)
}

/// The transcript of a session, then those of its subagents: of the transcripts whose name starts
/// with `session`, the one changed last; any session's when `session` is empty. None when the
/// folder holds none.
#[must_use]
pub fn session_files(folder: &Path, session: &str) -> Vec<PathBuf> {
    let mut mains: Vec<PathBuf> = names_in(folder)
        .into_iter()
        .filter(|name| is_jsonl(name) && name.starts_with(session))
        .map(|name| folder.join(name))
        .collect();

    // The newest first; of equal times, the first by name, as the TypeScript report's stable sort.
    mains.sort_by_key(|path| std::cmp::Reverse(modified(path)));

    let Some(main) = mains.into_iter().next() else {
        return Vec::new();
    };
    let subagents = main.with_extension("").join("subagents");
    let mut files = vec![main];

    files.extend(
        names_in(&subagents)
            .into_iter()
            .filter(|name| is_jsonl(name))
            .map(|name| subagents.join(name)),
    );
    files
}

/// Whether a file name is a JSON Lines transcript's, `.jsonl` in lower case as Claude Code writes it.
#[must_use]
pub fn is_jsonl(name: &str) -> bool {
    Path::new(name)
        .extension()
        .is_some_and(|extension| extension == "jsonl")
}

/// The names in a folder, sorted, none when it cannot be read.
#[must_use]
pub fn names_in(folder: &Path) -> Vec<String> {
    let mut names: Vec<String> = fs::read_dir(folder)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .map(|entry| entry.file_name().to_string_lossy().into_owned())
                .collect()
        })
        .unwrap_or_default();

    names.sort();
    names
}

/// Events a report counts: every turn of the session with `--all`, else the current turn only,
/// from the prompt that started it (the main transcript's last turn).
#[must_use]
pub fn reported_events(transcripts: &[Transcript], is_all: bool) -> Events {
    let from = match transcripts.first() {
        Some(main) if !is_all => main.turn_start,
        _ => 0,
    };
    let events = all_events(transcripts);

    Events {
        requests: events
            .requests
            .into_iter()
            .filter(|request| request.time >= from)
            .collect(),
        calls: events
            .calls
            .into_iter()
            .filter(|call| call.asked.start >= from)
            .collect(),
        // The session's count holds every turn: a turn alone counts its requests.
        model_usage: if is_all {
            events.model_usage
        } else {
            Vec::new()
        },
    }
}
