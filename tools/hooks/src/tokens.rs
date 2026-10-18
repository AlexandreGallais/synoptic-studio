//! Exact tokens of the current turn for the status line (`docs/tooling/metrics.md`): from the
//! person's prompt to the agent's last answer, like the session report, and the background tasks
//! the agent waits for. Summed from the transcript incrementally, a small cache in the Git folder
//! keeping the byte offset already read. Rewritten from `.claude/hooks/status-tokens.ts` (EN-104).
//!
//! A request is counted once, from its last line: a subagent's transcript repeats a request's
//! usage on each of its lines with a growing `output_tokens` (EN-092's note), so a line of the
//! request counted last replaces what its earlier lines added.

use std::fs;
use std::io::{Read, Seek, SeekFrom};

use crate::json::{self, Value, number_text, quote};
use crate::tasks::{Kind, Task, after_parsed_line};
use crate::time::epoch_ms;

/// Tokens of the current turn, and where the reading of the transcript stopped.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Tally {
    /// When the agent ended its turn, epoch milliseconds; 0 while it works.
    pub end: i64,
    /// Ids of the tool calls whose result or end arrived in the lines just read.
    pub finished: Vec<String>,
    /// Whether the agent ended its turn: the next prompt starts a new one.
    pub is_waiting: bool,
    /// The message id of the request counted last.
    pub last_id: String,
    /// Bytes of the transcript read.
    pub offset: usize,
    /// Tokens written by the agent in the turn.
    pub output: i64,
    /// Background tasks still running: the agent waits for them even after its turn ended.
    pub pending: Vec<Task>,
    /// When the turn started, epoch milliseconds; 0 before the first prompt.
    pub start: i64,
    /// Every token of the turn: input, cache writes and reads, output.
    pub total: i64,
    /// What the request counted last added to `output`, replaced by its next line.
    pub last_output: i64,
    /// What the request counted last added to `total`, replaced by its next line.
    pub last_total: i64,
}

/// A tally before any line.
#[must_use]
pub fn empty_tally() -> Tally {
    Tally {
        is_waiting: true,
        ..Tally::default()
    }
}

/// A whole number member, 0 when missing or not a number.
#[must_use]
pub fn whole(value: &Value, key: &str) -> i64 {
    match value.get(key) {
        // Tokens, offsets and times are whole numbers.
        #[allow(clippy::cast_possible_truncation)]
        Value::Number(number) => *number as i64,
        _ => 0,
    }
}

/// When a transcript line was written, epoch milliseconds, 0 when it has no time.
fn time_of(line: &Value) -> i64 {
    epoch_ms(line.get("timestamp").text())
}

/// Whether a transcript line holds text, not only tool results.
fn has_text(line: &Value) -> bool {
    match line.get("message").get("content") {
        Value::String(_) => true,
        content => content
            .items()
            .iter()
            .any(|block| block.get("type").text() == "text"),
    }
}

/// Whether a transcript line is a prompt typed by the person, not a tool result, a reminder, nor
/// the summary written after the conversation memory is compacted, nor a background task's
/// notification: they continue the turn.
fn is_prompt(line: &Value) -> bool {
    line.get("type").text() == "user"
        && line.get("isMeta") != &Value::Bool(true)
        && line.get("isCompactSummary") != &Value::Bool(true)
        && line.get("promptSource").text() != "system"
        && has_text(line)
}

/// A message's id as JavaScript's `String()` wrote it, `undefined` when missing.
fn message_id(message: &Value) -> String {
    match message {
        Value::Object(members) => match members.iter().find(|(name, _)| name == "id") {
            None => "undefined".to_owned(),
            Some((_, Value::String(id))) => id.clone(),
            Some((_, Value::Number(number))) => number_text(*number),
            Some((_, other)) => json::write(other),
        },
        _ => "undefined".to_owned(),
    }
}

/// The tally after a request's usage, counted from its last line: a line of the request counted
/// last replaces what the earlier ones added.
fn with_usage(mut tally: Tally, message: &Value) -> Tally {
    let usage = match message {
        Value::Object(members) => members.iter().find(|(name, _)| name == "usage"),
        _ => None,
    };
    let Some((_, usage)) = usage else {
        return tally;
    };
    let id = message_id(message);
    let output = whole(usage, "output_tokens");
    let all = output
        + whole(usage, "input_tokens")
        + whole(usage, "cache_creation_input_tokens")
        + whole(usage, "cache_read_input_tokens");

    if id == tally.last_id {
        tally.output += output - tally.last_output;
        tally.total += all - tally.last_total;
    } else {
        tally.output += output;
        tally.total += all;
        tally.last_id = id;
    }
    tally.last_output = output;
    tally.last_total = all;
    tally
}

/// The tally once the agent works again after the end of its turn: a new turn at a prompt, the
/// same turn resumed at an answer that no prompt asked (a background task's notification woke it).
fn resumed(mut tally: Tally, line: &Value) -> Tally {
    if is_prompt(line) {
        tally.end = 0;
        tally.is_waiting = false;
        tally.output = 0;
        tally.start = time_of(line);
        tally.total = 0;
    } else if line.get("type").text() == "assistant" {
        tally.end = 0;
        tally.is_waiting = false;
    }
    tally
}

/// The tally after one transcript line: a prompt after the agent ended its turn starts a new turn;
/// messages sent while the agent works stay inside the turn; an answer after the end of the turn
/// without a prompt resumes that turn.
fn with_line(tally: Tally, line: &Value, raw: &str) -> Tally {
    let message = line.get("message");
    let mut started = if tally.is_waiting {
        resumed(tally, line)
    } else {
        tally
    };
    let (pending, ended) = after_parsed_line(std::mem::take(&mut started.pending), line, raw);

    started.pending = pending;
    started.finished.extend(ended);

    let mut counted = with_usage(started, message);

    if message.get("stop_reason").text() == "end_turn" {
        counted.end = time_of(line);
        counted.is_waiting = true;
    }
    counted
}

/// The tally updated with new complete transcript lines, its offset moved past their bytes.
#[must_use]
pub fn add_tokens(tally: Tally, lines: &str) -> Tally {
    let start = Tally {
        finished: Vec::new(),
        offset: tally.offset + lines.len(),
        ..tally
    };

    lines
        .split('\n')
        .filter(|raw| raw.starts_with('{'))
        .fold(start, |sum, raw| match json::parse(raw) {
            Some(line) => with_line(sum, &line, raw),
            None => sum,
        })
}

/// The tally kept by the previous call, when it was for the same transcript; an empty one from the
/// start of the file otherwise.
#[must_use]
pub fn cached_tally(cached: &Value, transcript: &str) -> Tally {
    if cached.get("transcript") != &Value::String(transcript.to_owned()) {
        return empty_tally();
    }

    let tally = cached.get("tally");

    Tally {
        end: whole(tally, "end"),
        finished: Vec::new(),
        is_waiting: tally.get("isWaiting") != &Value::Bool(false),
        last_id: tally.get("lastId").text().to_owned(),
        offset: usize::try_from(whole(tally, "offset")).unwrap_or(0),
        output: whole(tally, "output"),
        pending: tally
            .get("pending")
            .items()
            .iter()
            .map(|task| Task {
                id: task.get("id").text().to_owned(),
                kind: Kind::read(task.get("kind").text()),
                label: task.get("label").text().to_owned(),
                start: whole(task, "start"),
            })
            .collect(),
        start: whole(tally, "start"),
        total: whole(tally, "total"),
        last_output: whole(tally, "lastOutput"),
        last_total: whole(tally, "lastTotal"),
    }
}

/// The cache's text: the tally and its transcript, members in the order the TypeScript status
/// line wrote them, the last request's counts after them.
#[must_use]
pub fn cache_json(tally: &Tally, transcript: &str) -> String {
    let list = |items: Vec<String>| items.join(",");
    let finished = list(tally.finished.iter().map(|id| quote(id)).collect());
    let pending = list(
        tally
            .pending
            .iter()
            .map(|task| {
                format!(
                    "{{\"id\":{},\"kind\":\"{}\",\"label\":{},\"start\":{}}}",
                    quote(&task.id),
                    task.kind.name(),
                    quote(&task.label),
                    task.start
                )
            })
            .collect(),
    );

    format!(
        "{{\"tally\":{{\"end\":{},\"finished\":[{finished}],\"isWaiting\":{},\"lastId\":{},\"offset\":{},\"output\":{},\"pending\":[{pending}],\"start\":{},\"total\":{},\"lastOutput\":{},\"lastTotal\":{}}},\"transcript\":{}}}",
        tally.end,
        tally.is_waiting,
        quote(&tally.last_id),
        tally.offset,
        tally.output,
        tally.start,
        tally.total,
        tally.last_output,
        tally.last_total,
        quote(transcript)
    )
}

/// The bytes of a transcript after an offset, the bytes before it never read: a session's
/// transcript grows to hundreds of megabytes (217 MB, SP-023). None when it is missing.
fn unread(transcript: &str, offset: usize) -> Vec<u8> {
    let mut rest = Vec::new();

    if !transcript.is_empty() {
        let _ = fs::File::open(transcript).and_then(|mut file| {
            file.seek(SeekFrom::Start(u64::try_from(offset).unwrap_or(u64::MAX)))?;
            file.read_to_end(&mut rest)
        });
    }
    rest
}

/// The tally of a transcript, read from where the cache stopped, the cache written again; only
/// complete lines are read, the last one may be still being written.
#[must_use]
pub fn turn_tokens(transcript: &str, cache_file: &std::path::Path) -> Tally {
    let cached = fs::read_to_string(cache_file)
        .ok()
        .and_then(|text| json::parse(&text))
        .unwrap_or(Value::Null);
    let start = cached_tally(&cached, transcript);
    let rest = unread(transcript, start.offset);
    let complete = rest
        .iter()
        .rposition(|byte| *byte == b'\n')
        .map_or(&rest[..0], |end| &rest[..=end]);
    let tally = add_tokens(start, &String::from_utf8_lossy(complete));
    let _ = fs::write(cache_file, cache_json(&tally, transcript));

    tally
}
