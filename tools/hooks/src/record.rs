//! What the `track-tool-call` and `log-tool-call` hooks write of a tool call (RET-001,
//! `the records' conventions, area metrics`): the call starting, read back when it ends, and one line per call
//! ended in `.git/metrics/tool-calls.jsonl`, with the session's state the status line last saw.
//! Rewritten from `.claude/hooks/log-tool-call.ts` and `status-place.ts` (EN-104): the same fields in
//! the same order, written as `JSON.stringify` writes them.

use std::path::{Component, Path, PathBuf};

use crate::guard::is_space;
use crate::json::{Value, js_length, number_text, quote, write};
use crate::tokens::whole;

/// Longest summary kept for one call: a long heredoc is cut, its length is kept.
pub const SUMMARY_LENGTH: usize = 300;

/// Input keys naming what a tool acted on, in order of preference.
const SUBJECT_KEYS: [&str; 7] = [
    "command",
    "file_path",
    "notebook_path",
    "pattern",
    "url",
    "query",
    "skill",
];

/// The text of a member, or a fallback when it is missing or empty.
#[must_use]
pub fn text_or<'a>(value: &'a Value, key: &str, fallback: &'a str) -> &'a str {
    match value.get(key).text() {
        "" => fallback,
        found => found,
    }
}

/// A number member, 0 when it is missing or not a number.
#[must_use]
pub fn count(value: &Value, key: &str) -> f64 {
    match value.get(key) {
        Value::Number(number) => *number,
        _ => 0.0,
    }
}

/// A folder made absolute against the process's own and normalized, as Node.js's `path.resolve`.
fn resolved(path: &str) -> PathBuf {
    let joined = std::env::current_dir().unwrap_or_default().join(path);
    let mut normal = PathBuf::from("/");

    for component in joined.components() {
        match component {
            Component::ParentDir => {
                normal.pop();
            }
            Component::Normal(name) => normal.push(name),
            _ => {}
        }
    }
    normal
}

/// The path from one folder to another, as Node.js's `path.relative` writes it: `src/a.ts`,
/// `../b`, empty for the folder itself.
#[must_use]
pub fn relative(from: &str, to: &str) -> String {
    let from = resolved(from);
    let to = resolved(to);
    let from_parts: Vec<_> = from.components().skip(1).collect();
    let to_parts: Vec<_> = to.components().skip(1).collect();
    let common = from_parts
        .iter()
        .zip(&to_parts)
        .take_while(|(left, right)| left == right)
        .count();
    let mut parts: Vec<String> = vec!["..".to_owned(); from_parts.len() - common];

    parts.extend(
        to_parts[common..]
            .iter()
            .map(|part| part.as_os_str().to_string_lossy().into_owned()),
    );
    parts.join("/")
}

/// What a tool call acted on: its command, file, pattern, address, query or skill, the first not
/// empty, a file path relative to `cwd`; for a subagent, its type and task.
fn subject(tool: &str, input: &Value, cwd: &str) -> String {
    if tool == "Agent" {
        return format!(
            "{}: {}",
            input.get("subagent_type").text(),
            input.get("description").text()
        );
    }

    let value = SUBJECT_KEYS
        .iter()
        .map(|key| input.get(key).text())
        .find(|found| !found.is_empty())
        .unwrap_or_default();

    if !cwd.is_empty() && Path::new(value).is_absolute() {
        relative(cwd, value)
    } else {
        value.to_owned()
    }
}

/// A text's white space runs each made one space, as `replaceAll(/\s+/gu, " ")`.
fn one_line(text: &str) -> String {
    let mut line = String::with_capacity(text.len());
    let mut in_space = false;

    for character in text.chars() {
        if is_space(character) {
            if !in_space {
                line.push(' ');
            }
            in_space = true;
        } else {
            line.push(character);
            in_space = false;
        }
    }
    line
}

/// The first `units` UTF-16 code units of a text, as JavaScript's `slice(0, units)`; a character
/// beyond the basic plane that the cut would split is left out.
#[must_use]
pub fn js_prefix(text: &str, units: usize) -> &str {
    let mut taken = 0;

    for (index, character) in text.char_indices() {
        taken += character.len_utf16();
        if taken > units {
            return &text[..index];
        }
    }
    text
}

/// A short summary of what a tool call did, on one line: at most `SUMMARY_LENGTH` characters,
/// followed by the full length when cut.
#[must_use]
pub fn tool_summary(tool: &str, input: &Value, cwd: &str) -> String {
    let summary = one_line(&subject(tool, input, cwd));
    let length = js_length(&summary);

    if length > SUMMARY_LENGTH {
        format!(
            "{}… ({length} characters)",
            js_prefix(&summary, SUMMARY_LENGTH)
        )
    } else {
        summary
    }
}

/// Whether a running call's record is of the subagent `agent_id`, which has ended (`SubagentStop`,
/// EN-120): its `agentId` is that id, never empty.
#[must_use]
pub fn is_call_of(record: &Value, agent_id: &str) -> bool {
    !agent_id.is_empty() && record.get("agentId").text() == agent_id
}

/// The call starting, as the `track-tool-call` hook writes it for `log-tool-call`:
/// `{"agent":…,"agentId":…,"description":…,"session":…,"start":…,"summary":…,"tool":…}`.
#[must_use]
pub fn running_call(input: &Value, start: i64) -> String {
    let tool = input.get("tool_name").text();
    let tool_input = input.get("tool_input");

    format!(
        "{{\"agent\":{},\"agentId\":{},\"description\":{},\"session\":{},\"start\":{start},\"summary\":{},\"tool\":{}}}",
        quote(text_or(input, "agent_type", "main")),
        quote(input.get("agent_id").text()),
        quote(tool_input.get("description").text()),
        quote(input.get("session_id").text()),
        quote(&tool_summary(tool, tool_input, input.get("cwd").text())),
        quote(tool),
    )
}

/// Where and when a call ends, read by the hook beside its input.
#[derive(Debug, Clone, PartialEq)]
pub struct CallContext {
    /// The branch checked out.
    pub branch: String,
    /// `now.json`, written by the status line; `Null` when missing.
    pub snapshot: Value,
    /// When the call started, from its running file; 0 when unknown.
    pub started_at: f64,
    /// When the hook runs, epoch milliseconds.
    pub time: i64,
}

/// The session's state when a call ends, from the status line's last snapshot, as JSON members:
/// tokens of the turn, memory, background tasks, all 0 when the snapshot is missing.
#[must_use]
pub fn session_moment(snapshot: &Value, time: i64) -> String {
    // Its times are whole numbers, as the status line writes them.
    let turn_start = whole(snapshot, "turnStart");
    let taken = whole(snapshot, "time");
    let since = |moment: i64| if moment == 0 { 0 } else { time - moment };

    format!(
        "\"contextPercent\":{},\"model\":{},\"pendingAgents\":{},\"pendingCommands\":{},\"snapshotAgeMs\":{},\"turnElapsedMs\":{},\"turnOutputTokens\":{},\"turnStart\":{},\"turnTokens\":{}",
        number_text(count(snapshot, "contextPercent")),
        quote(snapshot.get("model").text()),
        number_text(count(snapshot, "pendingAgents")),
        number_text(count(snapshot, "pendingCommands")),
        since(taken),
        since(turn_start),
        number_text(count(snapshot, "turnOutputTokens")),
        turn_start,
        number_text(count(snapshot, "turnTokens")),
    )
}

/// Characters of a JSON value written out, 0 when it is missing.
fn size_of(value: Option<&Value>) -> usize {
    value.map_or(0, |found| js_length(&write(found)))
}

/// A member of an object, `None` when missing, `Some(Null)` when written `null`.
fn member<'a>(value: &'a Value, key: &str) -> Option<&'a Value> {
    match value {
        Value::Object(members) => members
            .iter()
            .find(|(name, _)| name == key)
            .map(|(_, found)| found),
        _ => None,
    }
}

/// The record of one tool call, from the hook input of `PostToolUse` or `PostToolUseFailure`: the
/// line appended to the metrics log, without its line feed.
#[must_use]
pub fn tool_call_record(input: &Value, context: &CallContext) -> String {
    let tool = input.get("tool_name").text();
    let tool_input = input.get("tool_input");
    let is_failure = input.get("hook_event_name").text() == "PostToolUseFailure";
    let duration = match input.get("duration_ms") {
        Value::Number(number) => format!("\"durationMs\":{},", number_text(*number)),
        _ => String::new(),
    };
    let error = if is_failure {
        input
            .get("error")
            .text()
            .split('\n')
            .next()
            .unwrap_or_default()
    } else {
        ""
    };
    let is_background = tool == "Agent"
        || tool == "Monitor"
        || tool_input.get("run_in_background") == &Value::Bool(true);

    format!(
        "{{{},\"agent\":{},\"agentId\":{},\"branch\":{},\"description\":{},{duration}\"error\":{},\"inputChars\":{},\"isBackground\":{is_background},\"isOk\":{},\"outputChars\":{},\"permissionMode\":{},\"session\":{},\"startedAt\":{},\"summary\":{},\"time\":\"{}\",\"tool\":{},\"toolUseId\":{}}}",
        session_moment(&context.snapshot, context.time),
        quote(text_or(input, "agent_type", "main")),
        quote(input.get("agent_id").text()),
        quote(&context.branch),
        quote(tool_input.get("description").text()),
        quote(error),
        size_of(member(input, "tool_input")),
        !is_failure,
        size_of(member(input, "tool_response")),
        quote(input.get("permission_mode").text()),
        quote(input.get("session_id").text()),
        number_text(context.started_at),
        quote(&tool_summary(tool, tool_input, input.get("cwd").text())),
        calendar::temporal_instant_ms(context.time),
        quote(tool),
        quote(input.get("tool_use_id").text()),
    )
}
