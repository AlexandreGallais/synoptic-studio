//! The metrics report of the agent's work (`the records' conventions, area metrics`): sessions, commands by count
//! and by time, the slowest calls, time and tokens per session, feature, story and task, the saved
//! turns, where the latest turns' time went, the agent's tools, each Rust tool's uses, the rules'
//! findings per epic, and the library's sources, as `metrics-report.ts` printed it, the findings
//! added (EN-107).

use std::collections::{HashMap, VecDeque};
use std::hash::BuildHasher;
use std::path::{Path, PathBuf};

use run_totals::tally::Tokens;
use serde_json::Value;

use crate::findings::findings_table;
use crate::format::{format_duration, grouped};
use crate::json::Json;
use crate::session::{is_jsonl, names_in};
use crate::tables::{TimedCall, call_stats, command_key, js_prefix, markdown_table, stats_table};
use crate::tools::{tool_uses_table, tools_table, uses_since};
use crate::transcript::{Call, Transcript, all_events};
use crate::turn_time::{TURNS_SHOWN, time_table};
use crate::turns::{TurnRecord, turns_table};
use crate::work::{Moment, WorkStats, commits_by_unit, task_commits, task_of, unit_of, work_stats};
use calendar::iso_date;

/// Rows shown in each ranked table.
const TOP: usize = 20;

/// Characters of a call's summary shown in the table of slowest calls.
const SUMMARY_SHOWN: usize = 80;

/// Characters of a session id shown, enough to find its transcript.
const SESSION_SHOWN: usize = 8;

/// What the report is made of, read by its command.
#[derive(Clone, Debug, Default)]
pub struct ReportInputs {
    /// The transcripts, in the order their files are listed.
    pub transcripts: Vec<Transcript>,
    /// The lines of the hook's log, `tool-calls.jsonl`.
    pub logged: Vec<Value>,
    /// The lines of the tools' uses, `tool-uses.jsonl`.
    pub uses: Vec<Value>,
    /// The saved turns, `turns.jsonl`.
    pub turns: Vec<TurnRecord>,
    /// `git log --all --format=%H%x1f%aI%x1f%s%x1f%b%x1e`.
    pub log: String,
    /// The names the backlog's story files had, `E01-F04-US-018-<topic>.md`, written by the
    /// records' [`STORY_NAMES`] since the backlog moved there (EN-154).
    pub stories: Vec<String>,
    /// The names the backlog's feature files had, by [`FEATURE_NAMES`].
    pub features: Vec<String>,
    /// The library's report of its uses.
    pub sources: String,
    /// The first moment covered, epoch milliseconds; 0 for everything.
    pub since: i64,
}

/// A tool call with its duration, exact (the hook's) or estimated (the transcript's times).
struct Timed<'a> {
    call: &'a Call,
    duration_ms: f64,
    is_exact: bool,
}

/// The transcripts of a folder and every folder below it, breadth first, each folder's names
/// sorted, as Node.js's recursive `readdirSync` lists them: the sessions, then their subagents.
#[must_use]
pub fn transcript_files(folder: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut folders = VecDeque::from([folder.to_path_buf()]);

    while let Some(current) = folders.pop_front() {
        for name in names_in(&current) {
            let path = current.join(&name);

            if path.symlink_metadata().is_ok_and(|data| data.is_dir()) {
                folders.push_back(path.clone());
            }
            if is_jsonl(&name) {
                files.push(path);
            }
        }
    }
    files
}

/// The first moment a report covers, from its `--since <day>` option: the day's start, 00:00
/// UTC; 0 without the option.
///
/// # Errors
///
/// The option without a date, or with a text that is not one.
pub fn since_start(options: &[String]) -> Result<i64, String> {
    let Some(at) = options.iter().position(|option| option == "--since") else {
        return Ok(0);
    };
    let day = options.get(at + 1).map_or("", String::as_str);

    crate::time::day_start_ms(day)
        .ok_or_else(|| format!("metrics: --since {day}: not a day written YYYY-MM-DD"))
}

/// The statement `knowledge sql` runs for the names each story's file had: its epic, its feature
/// when it has one, its id and its topic (EN-154).
pub const STORY_NAMES: &str = "SELECT epic || '-' || CASE WHEN feature <> '' THEN feature || '-' ELSE '' END || id || '-' || topic || '.md' FROM stories ORDER BY 1";

/// The statement for the names each feature's file had: its epic, its id and its topic.
pub const FEATURE_NAMES: &str =
    "SELECT epic || '-' || id || '-' || topic || '.md' FROM features ORDER BY 1";

/// The feature of each story, and the epic of each story and feature, from the backlog's file
/// names: `E01-F04-US-018-….md`, `E17-RET-004-….md`, `E17-F21-….md`.
fn backlog_units(
    stories: &[String],
    features: &[String],
) -> (HashMap<String, String>, HashMap<String, String>) {
    let mut feature_of = HashMap::new();
    let mut epic_of = HashMap::new();
    let parts = |name: &str| -> Vec<String> { name.split('-').map(str::to_owned).collect() };
    let is_epic = |part: &str| {
        part.len() == 3 && part.starts_with('E') && part[1..].bytes().all(|b| b.is_ascii_digit())
    };
    let is_feature = |part: &str| {
        part.len() == 3 && part.starts_with('F') && part[1..].bytes().all(|b| b.is_ascii_digit())
    };
    let is_story = |kind: &str, number: &str| {
        !kind.is_empty()
            && kind.bytes().all(|b| b.is_ascii_uppercase())
            && number.len() == 3
            && number.bytes().all(|b| b.is_ascii_digit())
    };

    for name in stories {
        let part = parts(name);

        match part.as_slice() {
            [epic, feature, kind, number, _, ..]
                if is_epic(epic) && is_feature(feature) && is_story(kind, number) =>
            {
                let story = format!("{kind}-{number}");

                feature_of.insert(story.clone(), feature.clone());
                epic_of.insert(story, epic.clone());
            }
            [epic, kind, number, _, ..] if is_epic(epic) && is_story(kind, number) => {
                epic_of.insert(format!("{kind}-{number}"), epic.clone());
            }
            _ => {}
        }
    }
    for name in features {
        if let [epic, feature, ..] = parts(name).as_slice()
            && is_epic(epic)
            && is_feature(feature)
        {
            epic_of.insert(feature.clone(), epic.clone());
        }
    }
    (feature_of, epic_of)
}

/// The table of work totals, the longest active time first.
fn work_table(title: &str, mut stats: Vec<WorkStats>) -> String {
    stats.sort_by_key(|entry| std::cmp::Reverse(entry.active_ms));

    markdown_table(
        &[
            title,
            "Active time",
            "Output tokens",
            "All tokens",
            "Tool calls",
            "Tool time",
            "Failed",
        ],
        &stats
            .iter()
            .take(TOP)
            .map(|entry| {
                vec![
                    entry.key.clone(),
                    crate::format::duration_text(entry.active_ms),
                    grouped(entry.output_tokens),
                    grouped(entry.tokens),
                    entry.calls.to_string(),
                    format_duration(entry.tool_ms),
                    entry.failures.to_string(),
                ]
            })
            .collect::<Vec<_>>(),
    )
}

/// The sections about commands: by count, by total time, the slowest calls.
fn command_sections(calls: &[Timed<'_>]) -> Vec<String> {
    let shell: Vec<TimedCall> = calls
        .iter()
        .filter(|timed| timed.call.asked.tool == "Bash")
        .map(|timed| TimedCall {
            duration_ms: timed.duration_ms,
            is_ok: timed.call.answer.is_ok,
            summary: timed.call.asked.summary.clone(),
        })
        .collect();
    let by_command = call_stats(&shell, |call| command_key(&call.summary));
    let mut by_count = by_command.clone();
    let mut by_time = by_command;
    let mut slowest: Vec<&Timed<'_>> = calls.iter().collect();

    by_count.sort_by_key(|entry| std::cmp::Reverse(entry.count));
    by_time.sort_by(|left, right| right.total_ms.total_cmp(&left.total_ms));
    slowest.sort_by(|left, right| right.duration_ms.total_cmp(&left.duration_ms));
    by_count.truncate(TOP);
    by_time.truncate(TOP);
    slowest.truncate(TOP);

    vec![
        "## Commands by count".to_owned(),
        stats_table(&by_count),
        "## Commands by total time".to_owned(),
        stats_table(&by_time),
        "## Slowest calls".to_owned(),
        markdown_table(
            &["Duration", "Exact", "Tool", "Branch", "What"],
            &slowest
                .iter()
                .map(|timed| {
                    vec![
                        format_duration(timed.duration_ms),
                        (if timed.is_exact { "yes" } else { "no" }).to_owned(),
                        timed.call.asked.tool.clone(),
                        timed.call.asked.branch.clone(),
                        format!("`{}`", js_prefix(&timed.call.asked.summary, SUMMARY_SHOWN)),
                    ]
                })
                .collect::<Vec<_>>(),
        ),
    ]
}

/// The tokens Claude Code counted for each session (`modelUsage`, EN-120), its transcripts' counts
/// summed, a transcript's session being its requests'; only for a session whose first request
/// came in the period, since the count holds the whole session and cannot be cut (AUD-034).
#[must_use]
pub fn counted_sessions(transcripts: &[Transcript], since: i64) -> HashMap<String, Tokens> {
    let mut firsts: HashMap<&str, i64> = HashMap::new();
    let mut counts: HashMap<String, Tokens> = HashMap::new();

    for transcript in transcripts {
        for request in &transcript.requests {
            let first = firsts.entry(&request.session).or_insert(request.time);

            *first = (*first).min(request.time);
        }
        if let Some(request) = transcript.requests.first() {
            let count = counts.entry(request.session.clone()).or_default();

            *count = transcript
                .model_usage
                .iter()
                .fold(*count, |sum, (_, tokens)| sum.plus(tokens));
        }
    }

    counts.retain(|session, count| {
        count.total() > 0
            && firsts
                .get(session.as_str())
                .is_some_and(|first| *first >= since)
    });
    counts
}

/// The sections about units of work: sessions, features, stories and branches, tasks. A session's
/// tokens are Claude Code's count when [`counted_sessions`] holds it, the requests' otherwise; the
/// other units' are always the requests', a count being the session's, not a story's.
fn work_sections(
    moments: &[Moment],
    inputs: &ReportInputs,
    feature_of: &HashMap<String, String>,
) -> Vec<String> {
    let commits = task_commits(&inputs.log);
    let by_unit = commits_by_unit(&commits);
    let counted = counted_sessions(&inputs.transcripts, inputs.since);
    let mut starts: HashMap<&str, i64> = HashMap::new();

    for moment in moments {
        starts.entry(&moment.session).or_insert(moment.time);
    }

    let sessions: Vec<WorkStats> = work_stats(moments, |moment| moment.session.clone())
        .into_iter()
        .map(|stats| {
            let count = counted.get(&stats.key);

            WorkStats {
                key: format!(
                    "{} {}",
                    iso_date(starts.get(stats.key.as_str()).copied().unwrap_or(0)),
                    js_prefix(&stats.key, SESSION_SHOWN)
                ),
                output_tokens: count.map_or(stats.output_tokens, |tokens| tokens.output),
                tokens: count.map_or(stats.tokens, Tokens::total),
                ..stats
            }
        })
        .collect();

    vec![
        "## Sessions".to_owned(),
        work_table("Started, session", sessions),
        "## Features".to_owned(),
        work_table(
            "Feature",
            work_stats(moments, |moment| {
                let unit = unit_of(&moment.branch);

                feature_of.get(&unit).cloned().unwrap_or(unit)
            }),
        ),
        "## Stories and branches".to_owned(),
        work_table(
            "Unit",
            work_stats(moments, |moment| unit_of(&moment.branch)),
        ),
        "## Tasks, by commit".to_owned(),
        work_table(
            "Task",
            work_stats(moments, |moment| task_of(moment, &by_unit)),
        ),
    ]
}

/// The lines of the hook's log for calls that ended from a moment on.
#[must_use]
pub fn logged_since(records: &[Value], since: i64) -> Vec<Value> {
    records
        .iter()
        .filter(|record| hooks::time::epoch_ms(record.at("time").text()) >= since)
        .cloned()
        .collect()
}

/// The sections on turns and tools: each saved turn, where the latest turns' time went, the
/// agent's tools run through npm, each Rust tool's recorded uses.
fn turn_sections(inputs: &ReportInputs) -> Vec<String> {
    let calls = logged_since(&inputs.logged, inputs.since);
    let turns: Vec<TurnRecord> = inputs
        .turns
        .iter()
        .filter(|turn| turn.start >= inputs.since)
        .cloned()
        .collect();

    vec![
        "## Turns".to_owned(),
        turns_table(&turns),
        "## Where the time of the latest turns went".to_owned(),
        "Main agent only, calls timed by the hooks since RET-002: the tools running, and the model between two calls (reading, thinking, writing the next one; gaps over 5 minutes left out).".to_owned(),
        time_table(&calls, TURNS_SHOWN),
        "## The agent's tools".to_owned(),
        "Each tool of `the records' tools` run through npm: how often, how long, how often it failed.".to_owned(),
        tools_table(&calls),
        "## Tools".to_owned(),
        "Each Rust tool of the workspace, as it records its own uses in `.git/metrics/tool-uses.jsonl`, whoever called it (EN-102): by tool and subcommand, a failure being an exit code other than 0.".to_owned(),
        tool_uses_table(&uses_since(&inputs.uses, inputs.since)),
    ]
}

/// The section of the rules' findings, per epic (EN-107).
fn findings_section(inputs: &ReportInputs, epic_of: &HashMap<String, String>) -> Vec<String> {
    let findings: Vec<(String, String, String, i64)> = inputs
        .transcripts
        .iter()
        .flat_map(|transcript| &transcript.findings)
        .filter(|finding| finding.time >= inputs.since)
        .map(|finding| {
            let unit = unit_of(&finding.branch);

            (
                epic_of.get(&unit).cloned().unwrap_or(unit),
                finding.kind.clone(),
                finding.rule.clone(),
                finding.count,
            )
        })
        .collect();

    vec![
        "## Findings".to_owned(),
        "The project's rules met in the tool results, by rule (EN-093's count, EN-107): each hook's refusals by its event and reason (`hook`); in a shell command's output, each rule of ESLint, oxlint, the project's linter and markdownlint (`lint`), each TypeScript error (`type`), each Clippy lint (`clippy`) and each failed check of `check:all` (`check`); per epic, from the result's branch.".to_owned(),
        findings_table(&findings),
    ]
}

/// The report, its sections joined by a blank line, ended by a line break.
#[must_use]
pub fn metrics_report<S: BuildHasher>(
    inputs: &ReportInputs,
    exact: &HashMap<String, f64, S>,
) -> String {
    let events = all_events(&inputs.transcripts);
    let requests: Vec<_> = events
        .requests
        .iter()
        .filter(|request| request.time >= inputs.since)
        .collect();
    let calls: Vec<Timed<'_>> = events
        .calls
        .iter()
        .filter(|call| call.asked.start >= inputs.since)
        .map(|call| {
            let known = exact.get(&call.asked.id).copied();

            // Times far below 2^53 milliseconds.
            #[allow(clippy::cast_precision_loss)]
            let estimated = (call.answer.end - call.asked.start) as f64;

            Timed {
                call,
                duration_ms: known.unwrap_or(estimated),
                is_exact: known.is_some(),
            }
        })
        .collect();
    let mut moments: Vec<Moment> = requests
        .iter()
        .map(|request| Moment {
            branch: request.branch.clone(),
            output_tokens: request.tokens.output,
            session: request.session.clone(),
            time: request.time,
            tokens: request.tokens.total(),
            ..Moment::default()
        })
        .chain(calls.iter().map(|timed| Moment {
            branch: timed.call.asked.branch.clone(),
            duration_ms: timed.duration_ms,
            is_failure: !timed.call.answer.is_ok,
            is_tool_call: true,
            session: timed.call.asked.session.clone(),
            time: timed.call.answer.end,
            ..Moment::default()
        }))
        .collect();

    moments.sort_by_key(|moment| moment.time);

    let exact_count = calls.iter().filter(|timed| timed.is_exact).count();
    let since = if inputs.since == 0 {
        String::new()
    } else {
        format!(
            "Since {} (UTC) only; the sources from the start.",
            iso_date(inputs.since)
        )
    };
    let summary = [
        format!(
            "{} model requests, {} tool calls",
            requests.len(),
            calls.len()
        ),
        format!(
            "({exact_count} timed exactly by the hook; the others from transcript times, which also"
        ),
        "count the time spent writing the command). Active time: the gaps between moments of work,"
            .to_owned(),
        "each capped at 5 minutes.".to_owned(),
        since,
    ]
    .join(" ")
    .trim()
    .to_owned();
    let (feature_of, epic_of) = backlog_units(&inputs.stories, &inputs.features);
    let sections: Vec<String> = ["# Metrics of the agent's work".to_owned(), summary]
        .into_iter()
        .chain(command_sections(&calls))
        .chain(work_sections(&moments, inputs, &feature_of))
        .chain(turn_sections(inputs))
        .chain(findings_section(inputs, &epic_of))
        .chain([
            "## Sources".to_owned(),
            inputs.sources.trim_end().to_owned(),
        ])
        .collect();

    format!("{}\n", sections.join("\n\n"))
}

/// The exact durations the hook wrote, by tool use id.
#[must_use]
pub fn hook_durations(records: &[Value]) -> HashMap<String, f64> {
    records
        .iter()
        .filter(|record| record.at("durationMs").is_number())
        .map(|record| {
            (
                record.at("toolUseId").text().to_owned(),
                record.at("durationMs").number(),
            )
        })
        .collect()
}
