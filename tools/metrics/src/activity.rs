//! What the agents are doing (RET-002, `the records' tools`): the calls running now, the
//! latest calls of the main agent and of each subagent, and the end of the output of the
//! background commands of the last hour, for the Product Owner, who cannot see a subagent's work
//! until its final message. Its lines are those of `activity.ts` (EN-107).

use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use serde_json::Value;

use crate::format::{SECOND_MS, to_fixed};
use crate::json::Json;
use crate::session::names_in;
use crate::tables::js_prefix;

/// A tool call as the hook's log holds it, the fields shown here.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LoggedCall {
    /// `main` or the subagent's type.
    pub agent: String,
    /// The call's description, empty for none.
    pub description: String,
    /// How long it ran, milliseconds.
    pub duration_ms: f64,
    /// It did not fail.
    pub is_ok: bool,
    /// What it acted on.
    pub summary: String,
    /// When it ended, ISO 8601.
    pub time: String,
    /// The tool.
    pub tool: String,
}

/// Characters of a command shown when it has no description.
const COMMAND_SHOWN: usize = 80;

/// Characters of a running call's record shown.
pub const RECORD_SHOWN: usize = 160;

/// Lines of a background command's output shown.
const OUTPUT_LINES: usize = 5;

/// Background commands older than this are left out: one hour, in milliseconds.
const RECENT_MS: u128 = 3_600_000;

/// A logged call read from a line of the log: the agent `main` when it names none.
#[must_use]
pub fn logged_call(record: &Value) -> LoggedCall {
    let agent = record.at("agent").text();

    LoggedCall {
        agent: if agent.is_empty() { "main" } else { agent }.to_owned(),
        description: record.at("description").text().to_owned(),
        duration_ms: record.at("durationMs").number(),
        is_ok: *record.at("isOk") != Value::Bool(false),
        summary: record.at("summary").text().to_owned(),
        time: record.at("time").text().to_owned(),
        tool: record.at("tool").text().to_owned(),
    }
}

/// One line for a call: `  21:04:05 auditor Bash · List the files (1.2 s)`, `✗` first for a
/// failed one, the command's first 80 characters without a description.
#[must_use]
pub fn call_line(call: &LoggedCall) -> String {
    let what = if call.description.is_empty() {
        js_prefix(&call.summary, COMMAND_SHOWN)
    } else {
        call.description.clone()
    };
    let clock = call.time.get(11..19).unwrap_or_default();

    format!(
        "{} {clock} {} {} · {what} ({} s)",
        if call.is_ok { " " } else { "✗" },
        call.agent,
        call.tool,
        to_fixed(call.duration_ms / SECOND_MS, 1)
    )
}

/// The latest calls of each agent, the main one first even without calls, then in the order each
/// first called; every call for `kept` 0.
#[must_use]
pub fn latest_by_agent(calls: &[LoggedCall], kept: usize) -> Vec<(String, Vec<&LoggedCall>)> {
    let mut agents: Vec<String> = vec!["main".to_owned()];

    for call in calls {
        if !agents.contains(&call.agent) {
            agents.push(call.agent.clone());
        }
    }

    agents
        .into_iter()
        .map(|agent| {
            let own: Vec<&LoggedCall> = calls.iter().filter(|call| call.agent == agent).collect();
            // 0 keeps them all, as `slice(-0)` did.
            let from = if kept == 0 {
                0
            } else {
                own.len().saturating_sub(kept)
            };
            let latest = own[from..].to_vec();

            (agent, latest)
        })
        .collect()
}

/// When a file last changed, `None` when unknown.
fn modified(path: &Path) -> Option<SystemTime> {
    fs::metadata(path).and_then(|data| data.modified()).ok()
}

/// The output files of background commands written in the last hour, the newest first: the
/// `tasks/*.output` of each session's folder under `/private/tmp/claude-<uid>/<project>`, the
/// project being the folder's path, each `/` and space written `-`.
#[must_use]
pub fn recent_outputs(temporary: &Path, now: SystemTime) -> Vec<PathBuf> {
    let mut outputs: Vec<(PathBuf, SystemTime)> = names_in(temporary)
        .iter()
        .map(|session| temporary.join(session).join("tasks"))
        .filter(|folder| folder.is_dir())
        .flat_map(|folder| {
            names_in(&folder)
                .into_iter()
                .map(move |name| folder.join(name))
        })
        .filter(|path| path.to_string_lossy().ends_with(".output"))
        .filter_map(|path| modified(&path).map(|time| (path, time)))
        .filter(|(_, time)| {
            now.duration_since(*time)
                .map_or(true, |age| age.as_millis() < RECENT_MS)
        })
        .collect();

    outputs.sort_by_key(|(_, time)| std::cmp::Reverse(*time));
    outputs.into_iter().map(|(path, _)| path).collect()
}

/// Claude Code's folder of a project's background outputs: `<temporary>/claude-<uid>/<project>`.
#[must_use]
pub fn outputs_folder(uid: &str, project: &Path) -> PathBuf {
    let name: String = project
        .to_string_lossy()
        .chars()
        .map(|character| {
            if matches!(character, '/' | ' ') {
                '-'
            } else {
                character
            }
        })
        .collect();

    Path::new("/private/tmp")
        .join(format!("claude-{uid}"))
        .join(name)
}

/// What `activity` prints: the calls running, each agent's latest calls, the end of each recent
/// background output.
#[must_use]
pub fn activity_lines(
    running: &[String],
    calls: &[LoggedCall],
    kept: usize,
    outputs: &[(String, String)],
) -> String {
    let mut lines = vec!["Running now:".to_owned()];

    lines.extend(
        running
            .iter()
            .map(|record| format!("  {}", js_prefix(record, RECORD_SHOWN))),
    );
    for (agent, latest) in latest_by_agent(calls, kept) {
        lines.push(format!("Latest calls of {agent}:"));
        lines.extend(latest.iter().map(|call| format!("  {}", call_line(call))));
    }
    for (path, text) in outputs {
        let tail: Vec<&str> = text
            .trim_end_matches(crate::transcript::is_js_space)
            .split('\n')
            .collect();

        lines.push(format!("Background output {path}:"));
        lines.extend(
            tail[tail.len().saturating_sub(OUTPUT_LINES)..]
                .iter()
                .map(|line| format!("  {line}")),
        );
    }

    format!("{}\n", lines.join("\n"))
}
