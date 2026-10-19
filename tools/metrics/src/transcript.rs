//! Events of the agent's work read from Claude Code's transcripts (`docs/tooling/metrics.md`):
//! each model request with its tokens, each tool call with its start, end and outcome, and when
//! the current turn started. Every line of a transcript carries its time, session and Git branch.
//!
//! A transcript is read incrementally, as `run-totals` reads it (EN-092): [`read_chunk`] takes what
//! was read so far and the text written since, its complete lines only, so that a transcript of
//! hundreds of megabytes is read once and then from where it stopped (the cache, `cache.rs`).

use std::collections::HashMap;
use std::hash::BuildHasher;

use hooks::time::epoch_ms;
use run_totals::tally::Tokens;
use serde_json::Value;

use crate::findings::{Finding, result_findings};
use crate::json::Json;

/// The tokens of a request's `usage`, by kind.
#[must_use]
pub fn tokens_of(usage: &Value) -> Tokens {
    Tokens {
        input: usage.at("input_tokens").whole(),
        cache_write: usage.at("cache_creation_input_tokens").whole(),
        cache_read: usage.at("cache_read_input_tokens").whole(),
        output: usage.at("output_tokens").whole(),
    }
}

/// One model request, as its last line gives it: a request written over several lines (one per
/// content block) is counted once, by its message id, and a subagent's lines carry a growing
/// `output_tokens`, its last line the final count.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Request {
    /// `message.id`.
    pub id: String,
    /// The model that answered (`message.model`), empty when the line names none.
    pub model: String,
    /// `gitBranch`.
    pub branch: String,
    /// `sessionId`.
    pub session: String,
    /// The line's time, epoch milliseconds.
    pub time: i64,
    /// Its `usage`.
    pub tokens: Tokens,
}

/// A tool call asked for by the model (`tool_use`), answered or not yet.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ToolUse {
    /// The tool use id.
    pub id: String,
    /// The tool's name: `Bash`, `Read`.
    pub tool: String,
    /// What it acted on, on one line ([`call_summary`]).
    pub summary: String,
    /// `gitBranch` of its request's line.
    pub branch: String,
    /// `sessionId` of its request's line.
    pub session: String,
    /// The time of its request's line, epoch milliseconds.
    pub start: i64,
}

/// The result of a tool call: when it came, and whether it was not an error.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Answer {
    /// The time of the result's line, epoch milliseconds.
    pub end: i64,
    /// `is_error` was not `true`.
    pub is_ok: bool,
}

/// One answered tool call.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Call {
    /// The call as asked.
    pub asked: ToolUse,
    /// Its result.
    pub answer: Answer,
}

/// Every request and answered call of some transcripts.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Events {
    /// The requests, in the order their first line came.
    pub requests: Vec<Request>,
    /// The answered calls, in the order they were asked.
    pub calls: Vec<Call>,
    /// The tokens of each model that Claude Code counted for the sessions (`modelUsage`, EN-120),
    /// in the order each model first came; none when a session's count is missing or older than
    /// its last request.
    pub model_usage: Vec<(String, Tokens)>,
}

/// What a transcript holds, read up to `offset`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transcript {
    /// Bytes read: the end of the last complete line.
    pub offset: usize,
    /// The requests, in the order their first line came, each with its last line's values.
    pub requests: Vec<Request>,
    /// The tool calls asked, in order.
    pub uses: Vec<ToolUse>,
    /// The results, by tool use id, in the order they first came; a later result replaces one.
    pub answers: Vec<(String, Answer)>,
    /// Whether the agent ended its turn and waits for a prompt.
    pub is_waiting: bool,
    /// When the current turn started: the first prompt after the agent last ended its turn; 0
    /// without a prompt.
    pub turn_start: i64,
    /// The rules' findings met in its tool results, in order.
    pub findings: Vec<Finding>,
    /// The tokens of each model of its last `cost-state` line, the session's since it started;
    /// none without one, or once a request came after it (EN-120).
    pub model_usage: Vec<(String, Tokens)>,
}

impl Default for Transcript {
    fn default() -> Self {
        Transcript {
            offset: 0,
            requests: Vec::new(),
            uses: Vec::new(),
            answers: Vec::new(),
            is_waiting: true,
            turn_start: 0,
            findings: Vec::new(),
            model_usage: Vec::new(),
        }
    }
}

impl Transcript {
    /// The requests and the calls answered so far.
    #[must_use]
    pub fn events(&self) -> Events {
        let answers: HashMap<&str, Answer> = self
            .answers
            .iter()
            .map(|(id, answer)| (id.as_str(), *answer))
            .collect();

        Events {
            requests: self.requests.clone(),
            calls: self
                .uses
                .iter()
                .filter_map(|asked| {
                    answers.get(asked.id.as_str()).map(|answer| Call {
                        asked: asked.clone(),
                        answer: *answer,
                    })
                })
                .collect(),
            model_usage: Vec::new(),
        }
    }
}

/// The tokens of each model a `cost-state` line holds (`modelUsage`, seen 2026-10-19 in 14 sessions'
/// transcripts): Claude Code's count for the whole session, its subagents' included, written when
/// the session is left and again, grown, when a resumed one is; by kind, `thinkingTokens` left out;
/// none for another line.
#[must_use]
pub fn model_usage(line: &Value) -> Vec<(String, Tokens)> {
    let _ = line;
    Vec::new()
}

/// Input keys naming what a tool acted on, in order of preference.
const SUBJECT_KEYS: [&str; 7] = [
    "command",
    "file_path",
    "pattern",
    "url",
    "query",
    "skill",
    "description",
];

/// Whether a character is white space for JavaScript's `\s`: Unicode's white space and the byte
/// order mark, without the next-line control `U+0085`.
#[must_use]
pub fn is_js_space(character: char) -> bool {
    (character.is_whitespace() && character != '\u{85}') || character == '\u{feff}'
}

/// What a tool call acted on, on one line: its command, file, pattern, address, query, skill or
/// task, the first not empty, each run of white space written as one space.
#[must_use]
pub fn call_summary(input: &Value) -> String {
    let subject = SUBJECT_KEYS
        .iter()
        .map(|key| input.at(key).text())
        .find(|value| !value.is_empty())
        .unwrap_or_default();
    let mut summary = String::with_capacity(subject.len());
    let mut in_space = false;

    for character in subject.chars() {
        if is_js_space(character) {
            if !in_space {
                summary.push(' ');
            }
            in_space = true;
        } else {
            summary.push(character);
            in_space = false;
        }
    }
    summary
}

/// The content blocks of a line's message, none when it is not a list.
fn blocks(line: &Value) -> &[Value] {
    line.at("message").at("content").items()
}

/// Whether a `user` line comes from the person: not a reminder, nor the summary written after the
/// conversation is compacted, nor a background task's notification.
fn is_from_person(line: &Value) -> bool {
    line.at("type").text() == "user"
        && *line.at("isMeta") != Value::Bool(true)
        && *line.at("isCompactSummary") != Value::Bool(true)
        && line.at("promptSource").text() != "system"
}

/// Whether a line is a prompt the person typed, not a tool result nor a reminder.
fn is_prompt(line: &Value) -> bool {
    is_from_person(line)
        && (matches!(line.at("message").at("content"), Value::String(_))
            || blocks(line)
                .iter()
                .any(|block| block.at("type").text() == "text"))
}

/// Where each request and tool call of a transcript sits, for the lines read next.
struct Places {
    requests: HashMap<String, usize>,
    answers: HashMap<String, usize>,
    /// The tool of each call asked, by its id.
    tools: HashMap<String, String>,
}

/// The text of a tool result: its content, a string or the texts of its blocks.
fn result_text(block: &Value) -> String {
    match block.at("content") {
        Value::String(text) => text.clone(),
        content => content
            .items()
            .iter()
            .filter(|item| item.at("type").text() == "text")
            .map(|item| item.at("text").text())
            .collect::<Vec<_>>()
            .join("\n"),
    }
}

/// The transcript once one parsed line, with its time, is read.
fn read_line(
    mut transcript: Transcript,
    places: &mut Places,
    line: &Value,
    time: i64,
) -> Transcript {
    let branch = line.at("gitBranch").text();
    let session = line.at("sessionId").text();
    let message = line.at("message");

    if message.at("stop_reason").text() == "end_turn" {
        transcript.is_waiting = true;
    } else if transcript.is_waiting && is_prompt(line) {
        transcript.is_waiting = false;
        transcript.turn_start = time;
    }

    if line.at("type").text() == "assistant" {
        let id = message.at("id").text();
        let request = Request {
            id: id.to_owned(),
            model: message.at("model").text().to_owned(),
            branch: branch.to_owned(),
            session: session.to_owned(),
            time,
            tokens: tokens_of(message.at("usage")),
        };

        if !id.is_empty() {
            upsert(&mut transcript.requests, &mut places.requests, id, request);
        }

        for block in blocks(line)
            .iter()
            .filter(|block| block.at("type").text() == "tool_use")
        {
            let asked = ToolUse {
                id: block.at("id").text().to_owned(),
                tool: block.at("name").text().to_owned(),
                summary: call_summary(block.at("input")),
                branch: branch.to_owned(),
                session: session.to_owned(),
                start: time,
            };

            places.tools.insert(asked.id.clone(), asked.tool.clone());
            transcript.uses.push(asked);
        }
    }

    for block in blocks(line)
        .iter()
        .filter(|block| block.at("type").text() == "tool_result")
    {
        let id = block.at("tool_use_id").text();
        let answer = Answer {
            end: time,
            is_ok: *block.at("is_error") != Value::Bool(true),
        };
        let is_shell = places.tools.get(id).is_some_and(|tool| tool == "Bash");

        transcript.findings.extend(
            result_findings(&result_text(block), !answer.is_ok, is_shell)
                .into_iter()
                .map(|(kind, rule, count)| Finding {
                    time,
                    branch: branch.to_owned(),
                    kind,
                    rule,
                    count,
                }),
        );
        upsert(
            &mut transcript.answers,
            &mut places.answers,
            id,
            (id.to_owned(), answer),
        );
    }

    transcript
}

/// An item put in its list: in place of the one of the same id, kept where that one came first,
/// else at the end.
pub fn upsert<T, S: BuildHasher>(
    list: &mut Vec<T>,
    places: &mut HashMap<String, usize, S>,
    id: &str,
    item: T,
) {
    if let Some(&at) = places.get(id) {
        list[at] = item;
    } else {
        places.insert(id.to_owned(), list.len());
        list.push(item);
    }
}

/// The transcript once the text written after its offset is read: its complete lines only, the
/// last one kept for the next reading when no line break ends it yet. A line that is not a JSON
/// object, or has no time, is left out.
#[must_use]
pub fn read_chunk(transcript: Transcript, chunk: &str) -> Transcript {
    let complete = chunk.rfind('\n').map_or(0, |at| at + 1);
    let mut places = Places {
        requests: transcript
            .requests
            .iter()
            .enumerate()
            .map(|(at, request)| (request.id.clone(), at))
            .collect(),
        answers: transcript
            .answers
            .iter()
            .enumerate()
            .map(|(at, (id, _))| (id.clone(), at))
            .collect(),
        tools: transcript
            .uses
            .iter()
            .map(|asked| (asked.id.clone(), asked.tool.clone()))
            .collect(),
    };
    let mut next = Transcript {
        offset: transcript.offset + complete,
        ..transcript
    };

    for line in chunk[..complete]
        .split('\n')
        .filter(|line| line.starts_with('{'))
    {
        let Ok(json) = serde_json::from_str::<Value>(line) else {
            continue;
        };
        let stamp = json.at("timestamp").text();

        if stamp.is_empty() {
            continue;
        }
        let time = epoch_ms(stamp);

        next = read_line(next, &mut places, &json, time);
    }

    next
}

/// The events of several transcripts, in their order.
#[must_use]
pub fn all_events(transcripts: &[Transcript]) -> Events {
    transcripts
        .iter()
        .map(Transcript::events)
        .fold(Events::default(), |mut all, events| {
            all.requests.extend(events.requests);
            all.calls.extend(events.calls);
            all
        })
}
