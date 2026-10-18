//! The agent's own tools in the metrics (RET-002, `docs/tooling/agent-tools.md`): how often each
//! one ran through npm, how long it took and how often it failed, from the hook's log; and each
//! Rust tool's own record of its uses, `tool-uses.jsonl` (EN-102, `docs/tooling/metrics.md`).

use std::cmp::Ordering;

use hooks::time::epoch_ms;
use serde_json::Value;

use crate::format::format_duration;
use crate::json::Json;
use crate::tables::{TimedCall, call_stats, markdown_table, stats_table};

/// The npm scripts that are or were the agent's tools, read in the hook's log: those EN-115
/// removed stay, since the logs of the runs before it name them; the agent now calls their
/// binaries, counted in `tool-uses.jsonl` ([`tool_uses_table`]).
const AGENT_TOOLS: [&str; 13] = [
    "activity",
    "backlog:status",
    "check:all",
    "epic:check",
    "golden:check",
    "golden:write",
    "metrics",
    "metrics:session",
    "records:save",
    "replace",
    "story:finish",
    "branches:clean",
    "library",
];

/// Whether a character may be part of an npm script's name here: a word character or `:`.
fn is_name_character(character: char) -> bool {
    character.is_ascii_alphanumeric() || character == '_' || character == ':'
}

/// The agent's tool a shell command runs, if any: the first `npm run [-s |--silent ]<name>`, a
/// word of its own, whose name is one of the agent's tools; empty otherwise.
#[must_use]
pub fn agent_tool(command: &str) -> String {
    let name = command.match_indices("npm run ").find_map(|(at, found)| {
        let is_word_start = command[..at]
            .chars()
            .next_back()
            .is_none_or(|before| !(before.is_ascii_alphanumeric() || before == '_'));
        let rest = &command[at + found.len()..];
        let after = ["-s ", "--silent "]
            .iter()
            .find_map(|flag| rest.strip_prefix(flag))
            .filter(|after| after.starts_with(is_name_character))
            .unwrap_or(rest);
        let name: String = after
            .chars()
            .take_while(|c| is_name_character(*c))
            .collect();

        (is_word_start && !name.is_empty()).then_some(name)
    });

    name.filter(|name| AGENT_TOOLS.contains(&name.as_str()))
        .unwrap_or_default()
}

/// What a recorded use is grouped under: the tool, then its subcommand when it has one.
#[must_use]
pub fn tool_use_key(record: &Value) -> String {
    [record.at("tool").text(), record.at("subcommand").text()]
        .into_iter()
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// The recorded uses that started from a moment on; every one for 0.
#[must_use]
pub fn uses_since(records: &[Value], since: i64) -> Vec<Value> {
    records
        .iter()
        .filter(|record| since == 0 || epoch_ms(record.at("start").text()) >= since)
        .cloned()
        .collect()
}

/// The primary weight of an ASCII character in the root collation of Unicode's CLDR, which
/// `localeCompare` follows: white space, then punctuation, symbols, digits and letters, a letter's
/// case left to a later level.
fn primary(character: char) -> (u8, u32) {
    const PUNCTUATION: &str = "_-,;:!?.'\"()[]{}@*/\\&#%`^";
    const SYMBOLS: &str = "+<=>|~$";

    if character.is_whitespace() {
        return (0, u32::from(character));
    }
    if let Some(at) = PUNCTUATION.find(character) {
        return (1, u32::try_from(at).unwrap_or(0));
    }
    if let Some(at) = SYMBOLS.find(character) {
        return (2, u32::try_from(at).unwrap_or(0));
    }
    if character.is_ascii_digit() {
        return (3, u32::from(character));
    }
    if character.is_ascii_alphabetic() {
        return (4, u32::from(character.to_ascii_lowercase()));
    }
    (5, u32::from(character))
}

/// Two texts compared as `localeCompare` does for the tools' names and subcommands: by their
/// characters' primary weights, then a lower-case letter before its upper case.
#[must_use]
pub fn locale_compare(left: &str, right: &str) -> Ordering {
    let weights = |text: &str| text.chars().map(primary).collect::<Vec<_>>();
    let cases = |text: &str| {
        text.chars()
            .map(|character| character.is_ascii_uppercase())
            .collect::<Vec<_>>()
    };

    weights(left)
        .cmp(&weights(right))
        .then_with(|| cases(left).cmp(&cases(right)))
}

/// The table of each tool's recorded uses (EN-102): calls, failures, median and slowest
/// durations; the most called first, then by name; a failure is an exit code other than 0.
#[must_use]
pub fn tool_uses_table(records: &[Value]) -> String {
    let uses: Vec<TimedCall> = records
        .iter()
        .map(|record| TimedCall {
            duration_ms: record.at("durationMs").number(),
            is_ok: record.at("exitCode").number() == 0.0,
            summary: tool_use_key(record),
        })
        .collect();
    let mut stats = call_stats(&uses, |use_| use_.summary.clone());

    stats.sort_by(|left, right| {
        right
            .count
            .cmp(&left.count)
            .then_with(|| locale_compare(&left.key, &right.key))
    });

    markdown_table(
        &["Tool", "Calls", "Failed", "Median", "Slowest"],
        &stats
            .iter()
            .map(|entry| {
                vec![
                    format!("`{}`", entry.key),
                    entry.count.to_string(),
                    entry.failures.to_string(),
                    format_duration(entry.median_ms),
                    format_duration(entry.max_ms),
                ]
            })
            .collect::<Vec<_>>(),
    )
}

/// The table of the agent's tools run through npm, from the hook's log: count, total, median,
/// longest, failed; the most used first.
#[must_use]
pub fn tools_table(records: &[Value]) -> String {
    let calls: Vec<TimedCall> = records
        .iter()
        .filter(|record| {
            record.at("tool").text() == "Bash"
                && !agent_tool(record.at("summary").text()).is_empty()
        })
        .map(|record| TimedCall {
            duration_ms: record.at("durationMs").number(),
            is_ok: *record.at("isOk") != Value::Bool(false),
            summary: record.at("summary").text().to_owned(),
        })
        .collect();
    let mut stats = call_stats(&calls, |call| agent_tool(&call.summary));

    stats.sort_by_key(|entry| std::cmp::Reverse(entry.count));
    stats_table(&stats)
}
