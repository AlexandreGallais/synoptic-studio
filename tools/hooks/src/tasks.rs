//! Background tasks of a session, read from its transcript: which tool calls start one — a subagent,
//! a monitor, a call run in the background, a message that resumes a subagent — and which lines end
//! one: its notification, an error result when it was refused before it started, or a subagent's
//! result when it ran in the foreground. Read by `limit-agents`; rewritten from
//! `.claude/hooks/status-tasks.ts` (EN-104), its only version.

use crate::json::{self, Value, write};
use crate::record::js_prefix;
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

/// The blocks of a line of one type: `tool_use`, `tool_result`.
fn blocks_of<'a>(line: &'a Value, kind: &'a str) -> impl Iterator<Item = &'a Value> {
    blocks(line)
        .iter()
        .filter(move |block| block.get("type").text() == kind)
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
/// description or the start of its command.
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
        _ => text_or(
            input,
            "description",
            js_prefix(input.get("command").text(), LABEL_LENGTH),
        )
        .to_owned(),
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

    blocks_of(line, "tool_use")
        .filter(|block| is_background(block))
        .map(|block| task_of(block, start))
        .collect()
}

/// The id a tool result answers.
fn answered(block: &Value) -> String {
    block.get("tool_use_id").text().to_owned()
}

/// Ids of the calls a transcript line answers with an error: refused or failed before it started.
#[must_use]
pub fn failed_ids(line: &Value) -> Vec<String> {
    blocks_of(line, "tool_result")
        .filter(|block| block.get("is_error") == &Value::Bool(true))
        .map(answered)
        .collect()
}

/// Ids of the calls whose end a line notifies (`<task-notification>`, naming the call that started
/// it): each `<tool-use-id>([\w-]+)</tool-use-id>`.
#[must_use]
pub fn notified_ids(raw: &str) -> Vec<String> {
    if !raw.contains("<task-notification>") {
        return Vec::new();
    }

    raw.split("<tool-use-id>")
        .skip(1)
        .filter_map(|rest| {
            let (id, _) = rest.split_once("</tool-use-id>")?;

            (!id.is_empty()
                && id
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-')))
            .then(|| id.to_owned())
        })
        .collect()
}

/// Ids of the tool calls a line ends: its tool results, then the calls its notification names.
#[must_use]
pub fn ended_ids(line: &Value, raw: &str) -> Vec<String> {
    blocks_of(line, "tool_result")
        .map(answered)
        .chain(notified_ids(raw))
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

/// A tool result's content written as JSON, `""` when it has none: the text the transcripts are
/// searched in for the marks of a launch or a resume.
fn content_json(block: &Value) -> String {
    match block.get("content") {
        Value::Null => "\"\"".to_owned(),
        content => write(content),
    }
}

/// Whether a tool result launches a subagent in the background, as every `Agent` result of the
/// transcripts did up to 2026-10-17: `toolUseResult` holds `status: "async_launched"` and
/// `isAsync: true`, and the text opens with "Async agent launched successfully" (one line of
/// 2026-10-16 holds the text without `toolUseResult`).
fn launched_in_background(line: &Value, block: &Value) -> bool {
    let result = line.get("toolUseResult");

    result.get("status").text() == "async_launched"
        || result.get("isAsync") == &Value::Bool(true)
        || content_json(block).contains("Async agent launched")
}

/// Ids of the calls a line answers with a result that is neither an error nor a launch in the
/// background: a subagent run in the foreground ends with its result (`status: "completed"`, the
/// hooks reference), never with a notification (AUD-020).
#[must_use]
pub fn completed_ids(line: &Value) -> Vec<String> {
    blocks_of(line, "tool_result")
        .filter(|block| {
            block.get("is_error") != &Value::Bool(true) && !launched_in_background(line, block)
        })
        .map(answered)
        .collect()
}

/// Ids of the messages a line answers by resuming a subagent: a subagent that had stopped runs
/// again in the background, its end notified under the message's call id (seen 2026-10-15:
/// "Resuming agent …", then the notification 15 minutes later); a message to a subagent still
/// running is only queued, its result resuming nothing.
#[must_use]
pub fn resumed_ids(line: &Value) -> Vec<String> {
    blocks_of(line, "tool_result")
        .filter(|block| content_json(block).contains("resumedAgentId"))
        .map(answered)
        .collect()
}

/// Background tasks still running after one parsed transcript line: those it starts added, those
/// notified as ended or answered with an error removed, a subagent answered with its result in the
/// foreground removed, those older than `LOST_TASK_MS` at the line's time dropped; a message to a
/// subagent, once answered, becomes that subagent when its result resumes it, or is dropped.
fn after_parsed_line(pending: Vec<Task>, line: &Value, raw: &str) -> Vec<Task> {
    let ended = ended_ids(line, raw);
    let mut notified = if raw.contains("<task-notification>") {
        ended.clone()
    } else {
        Vec::new()
    };

    notified.extend(failed_ids(line));

    let completed = completed_ids(line);
    let resumed = resumed_ids(line);
    let time = epoch_ms(line.get("timestamp").text());

    pending
        .into_iter()
        .chain(started_tasks(line))
        .filter(|task| {
            !notified.contains(&task.id) && (time == 0 || time - task.start < LOST_TASK_MS)
        })
        .filter(|task| task.kind != Kind::Agent || !completed.contains(&task.id))
        .filter_map(|task| {
            if task.kind != Kind::Message || !ended.contains(&task.id) {
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

/// Background tasks still running after one transcript line, as [`after_parsed_line`] gives them.
/// With no task pending, only a line starting a call can change them: the others are not parsed.
#[must_use]
pub fn after_line(pending: Vec<Task>, raw: &str) -> Vec<Task> {
    if pending.is_empty() && !raw.contains("\"tool_use\"") {
        return pending;
    }

    match json::parse(raw) {
        Some(line) => after_parsed_line(pending, &line, raw),
        None => pending,
    }
}

/// Background tasks still running after the complete lines of a transcript's text.
#[must_use]
pub fn pending_after(pending: Vec<Task>, lines: &str) -> Vec<Task> {
    lines
        .split('\n')
        .filter(|line| line.starts_with('{'))
        .fold(pending, after_line)
}

/// The id following `marker` in a text, made of letters and digits, empty when the marker is absent.
fn id_after<'a>(text: &'a str, marker: &str) -> &'a str {
    text.split_once(marker).map_or("", |(_, rest)| {
        let end = rest
            .find(|character: char| !character.is_ascii_alphanumeric())
            .unwrap_or(rest.len());

        &rest[..end]
    })
}

/// The names the subagents started or resumed by these calls answer to in a `SendMessage`: the
/// `name` an `Agent` call gave, and the agent id its result names, `agentId: …` when launched
/// (seen 2026-10-17) or `"resumedAgentId":"…"` when a message resumed it (seen 2026-10-16).
#[must_use]
pub fn agent_names(lines: &str, ids: &[&str]) -> Vec<String> {
    lines
        .lines()
        .filter(|raw| raw.starts_with('{') && ids.iter().any(|id| raw.contains(id)))
        .filter_map(json::parse)
        .flat_map(|line| {
            blocks(&line)
                .iter()
                .filter_map(|block| match block.get("type").text() {
                    "tool_use" if ids.contains(&block.get("id").text()) => {
                        Some(block.get("input").get("name").text().to_owned())
                    }
                    "tool_result" if ids.contains(&block.get("tool_use_id").text()) => {
                        let text = result_text(block);
                        let launched = id_after(&text, "agentId: ");

                        Some(
                            if launched.is_empty() {
                                id_after(&text, "\"resumedAgentId\":\"")
                            } else {
                                launched
                            }
                            .to_owned(),
                        )
                    }
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .filter(|name| !name.is_empty())
        .collect()
}
