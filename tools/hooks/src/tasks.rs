//! Background tasks of a session, read from its transcript: which tool calls start one — a subagent,
//! a monitor, a call run in the background — and which lines end one: its notification, or an error
//! result when it was refused before it started. The Rust twin of `.claude/hooks/status-tasks.ts`
//! and of `withTasks` in `status-tokens.ts`, until F19 ports the status line and drops those.

use crate::json::{self, Value};
use crate::time::epoch_ms;

/// Three hours: a background task older than this is taken as lost, its end never arrived.
pub const LOST_TASK_MS: i64 = 10_800_000;

/// Characters of a command kept as its label when it has no description.
const LABEL_LENGTH: usize = 60;

/// What a background task is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// A subagent.
    Agent,
    /// A shell command in the background, a monitor.
    Command,
    /// A message to a subagent whose result is still to come: a subagent if that result resumes one.
    Message,
}

/// A background task the agent started, still running.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Task {
    /// The id of the tool call that started it.
    pub id: String,
    pub kind: Kind,
    /// What it does: a subagent's type and task, a command's description.
    pub label: String,
    /// When it started, epoch milliseconds.
    pub start: i64,
}

/// The blocks of a transcript line's message, none for a text content.
fn blocks(line: &Value) -> &[Value] {
    line.get("message").get("content").items()
}

/// Whether a `tool_use` block may run in the background: every subagent and monitor, any call
/// asked to run there, and a message to a subagent, which resumes it when it had stopped.
fn is_background(block: &Value) -> bool {
    matches!(
        block.get("name").text(),
        "Agent" | "Monitor" | "SendMessage"
    ) || block.get("input").get("run_in_background") == &Value::Bool(true)
}

/// The text of a member, or a fallback when it is missing or empty.
fn text_or<'a>(value: &'a Value, key: &str, fallback: &'a str) -> &'a str {
    match value.get(key).text() {
        "" => fallback,
        found => found,
    }
}

/// What a background task does: a subagent's type and task, a message's summary, a command's
/// description.
fn label_of(name: &str, input: &Value) -> String {
    match name {
        "Agent" => format!(
            "{}: {}",
            text_or(input, "subagent_type", "agent"),
            input.get("description").text()
        ),
        "SendMessage" => format!(
            "resumed: {}",
            text_or(input, "summary", input.get("to").text())
        ),
        _ => {
            let command: String = input
                .get("command")
                .text()
                .chars()
                .take(LABEL_LENGTH)
                .collect();

            text_or(input, "description", &command).to_owned()
        }
    }
}

/// The background task a `tool_use` block starts.
fn task_of(block: &Value, start: i64) -> Task {
    let name = block.get("name").text();

    Task {
        id: block.get("id").text().to_owned(),
        kind: match name {
            "Agent" => Kind::Agent,
            "SendMessage" => Kind::Message,
            _ => Kind::Command,
        },
        label: label_of(name, block.get("input")),
        start,
    }
}

/// Background tasks a transcript line starts.
#[must_use]
pub fn started_tasks(line: &Value) -> Vec<Task> {
    let start = epoch_ms(line.get("timestamp").text());

    blocks(line)
        .iter()
        .filter(|block| block.get("type").text() == "tool_use" && is_background(block))
        .map(|block| task_of(block, start))
        .collect()
}

/// Ids of the calls a transcript line answers with an error: refused or failed before it started.
#[must_use]
pub fn failed_ids(line: &Value) -> Vec<String> {
    blocks(line)
        .iter()
        .filter(|block| {
            block.get("type").text() == "tool_result" && block.get("is_error") == &Value::Bool(true)
        })
        .map(|block| block.get("tool_use_id").text().to_owned())
        .collect()
}

/// Ids of the calls whose end a line notifies (`<task-notification>`, naming the call that started it).
#[must_use]
pub fn notified_ids(raw: &str) -> Vec<String> {
    if !raw.contains("<task-notification>") {
        return Vec::new();
    }

    raw.split("<tool-use-id>")
        .skip(1)
        .filter_map(|rest| {
            let id = rest.split("</tool-use-id>").next()?;

            id.chars()
                .all(|character| character.is_alphanumeric() || matches!(character, '_' | '-'))
                .then(|| id.to_owned())
        })
        .collect()
}

/// The text of a tool result's content: a string, or its text blocks joined.
fn result_text(block: &Value) -> String {
    let content = block.get("content");

    match content {
        Value::String(text) => text.clone(),
        _ => content
            .items()
            .iter()
            .map(|item| item.get("text").text())
            .collect(),
    }
}

/// Ids of the calls a line answers with a result, and of those whose result resumes a subagent: a
/// subagent that had stopped runs again in the background, its end notified under the message's
/// call id (seen 2026-10-15: "Resuming agent …", then the notification 15 minutes later); a
/// message to a subagent still running is only queued.
#[must_use]
pub fn answered_ids(line: &Value) -> (Vec<String>, Vec<String>) {
    let results: Vec<&Value> = blocks(line)
        .iter()
        .filter(|block| block.get("type").text() == "tool_result")
        .collect();
    let id = |block: &&Value| block.get("tool_use_id").text().to_owned();

    (
        results.iter().map(id).collect(),
        results
            .iter()
            .filter(|block| result_text(block).contains("\"resumedAgentId\""))
            .map(id)
            .collect(),
    )
}

/// Background tasks still running after one transcript line: those it starts added, those notified
/// as ended or answered with an error removed, those older than `LOST_TASK_MS` dropped; a message
/// to a subagent, once answered, becomes that subagent when its result resumes it, or is dropped.
#[must_use]
pub fn after_line(pending: Vec<Task>, raw: &str) -> Vec<Task> {
    let awaits_message = pending.iter().any(|task| task.kind == Kind::Message);

    // Only lines that start, end or refuse a call change the tasks, and the results of messages:
    // the others are not parsed.
    let matters = raw.contains("\"tool_use\"")
        || raw.contains("<task-notification>")
        || raw.contains("is_error")
        || (awaits_message && raw.contains("\"tool_result\""));

    if !matters {
        return pending;
    }

    let Some(line) = json::parse(raw) else {
        return pending;
    };
    let mut ended = notified_ids(raw);

    ended.extend(failed_ids(&line));

    let time = epoch_ms(line.get("timestamp").text());
    let (answered, resumed) = answered_ids(&line);

    pending
        .into_iter()
        .chain(started_tasks(&line))
        .filter(|task| !ended.contains(&task.id) && (time == 0 || time - task.start < LOST_TASK_MS))
        .filter_map(|task| {
            if task.kind != Kind::Message || !answered.contains(&task.id) {
                Some(task)
            } else if resumed.contains(&task.id) {
                Some(Task {
                    kind: Kind::Agent,
                    ..task
                })
            } else {
                None
            }
        })
        .collect()
}

/// Background tasks still running after the complete lines of a transcript's text.
#[must_use]
pub fn pending_after(pending: Vec<Task>, lines: &str) -> Vec<Task> {
    lines
        .lines()
        .filter(|line| line.starts_with('{'))
        .fold(pending, after_line)
}

/// The tasks kept in the status line's cache (`.git/metrics/status-line-turn.json`) for a
/// transcript, and the byte offset they were read to; none and 0 for another transcript.
#[must_use]
pub fn cached_tasks(cache: &Value, transcript: &str) -> (Vec<Task>, usize) {
    if cache.get("transcript").text() != transcript {
        return (Vec::new(), 0);
    }

    let tally = cache.get("tally");
    let number = |value: &Value| match value {
        // Offsets and times are whole numbers written by the status line.
        #[allow(clippy::cast_possible_truncation)]
        Value::Number(number) => *number as i64,
        _ => 0,
    };
    let tasks = tally
        .get("pending")
        .items()
        .iter()
        .map(|task| Task {
            id: task.get("id").text().to_owned(),
            kind: match task.get("kind").text() {
                "agent" => Kind::Agent,
                "message" => Kind::Message,
                _ => Kind::Command,
            },
            label: task.get("label").text().to_owned(),
            start: number(task.get("start")),
        })
        .collect();

    (
        tasks,
        usize::try_from(number(tally.get("offset"))).unwrap_or(0),
    )
}
