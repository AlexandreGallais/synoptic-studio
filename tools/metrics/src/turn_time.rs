//! Where a turn's time goes (RET-002, `the records' conventions, area metrics`): for the main agent's calls whose
//! start the `track-tool-call` hook recorded, the time the tools ran, and the time between one
//! call's end and the next one's start — the model reading, thinking and writing the next call —
//! with the characters it wrote. Calls logged before RET-002 have no start and are left out.

use serde_json::Value;

use crate::format::{format_duration, grouped, js_round};
use crate::json::Json;
use crate::tables::markdown_table;
use calendar::iso_minute;

/// A main-agent call with its start, as the log holds it.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct StartedCall {
    /// How long it ran, milliseconds.
    pub duration_ms: f64,
    /// Characters the agent wrote for it.
    pub input_chars: f64,
    /// When it started, epoch milliseconds.
    pub started_at: f64,
    /// When its turn started, epoch milliseconds.
    pub turn_start: f64,
}

/// One turn's time, split between the tools and the model.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TurnTime {
    /// Its calls.
    pub calls: usize,
    /// Characters the agent wrote.
    pub chars_written: f64,
    /// The model between the calls, milliseconds.
    pub model_ms: f64,
    /// The tools running, milliseconds.
    pub tool_ms: f64,
    /// When it started, epoch milliseconds.
    pub turn_start: f64,
}

/// Gaps longer than this are a pause of the person, not the model at work: 5 minutes.
const PAUSE_MS: f64 = 300_000.0;

/// Percent of a whole.
const PERCENT: f64 = 100.0;

/// Turns shown in the time table.
pub const TURNS_SHOWN: usize = 10;

/// The main agent's calls of the log that have a start, background ones left out, oldest first.
#[must_use]
pub fn started_calls(records: &[Value]) -> Vec<StartedCall> {
    let mut calls: Vec<StartedCall> = records
        .iter()
        .filter(|record| {
            record.at("agent").text() == "main"
                && record.at("startedAt").number() > 0.0
                && record.at("turnStart").number() > 0.0
                && *record.at("isBackground") != Value::Bool(true)
        })
        .map(|record| StartedCall {
            duration_ms: record.at("durationMs").number(),
            input_chars: record.at("inputChars").number(),
            started_at: record.at("startedAt").number(),
            turn_start: record.at("turnStart").number(),
        })
        .collect();

    calls.sort_by(|left, right| left.started_at.total_cmp(&right.started_at));
    calls
}

/// The time of one turn: its tools, and the model between them, gaps over 5 minutes left out.
#[must_use]
pub fn turn_time(calls: &[StartedCall]) -> TurnTime {
    let model_ms = calls
        .windows(2)
        .map(|pair| pair[1].started_at - (pair[0].started_at + pair[0].duration_ms))
        .filter(|gap| *gap > 0.0 && *gap < PAUSE_MS)
        .sum();

    TurnTime {
        calls: calls.len(),
        chars_written: calls.iter().map(|call| call.input_chars).sum(),
        model_ms,
        tool_ms: calls.iter().map(|call| call.duration_ms).sum(),
        turn_start: calls.first().map_or(0.0, |call| call.turn_start),
    }
}

/// The table of the latest turns' time, newest turn last.
#[must_use]
pub fn time_table(records: &[Value], latest: usize) -> String {
    let calls = started_calls(records);
    let mut starts: Vec<f64> = Vec::new();

    for call in &calls {
        if !starts.contains(&call.turn_start) {
            starts.push(call.turn_start);
        }
    }

    let shown = &starts[starts.len().saturating_sub(latest)..];
    let rows: Vec<Vec<String>> = shown
        .iter()
        .map(|start| {
            let turn: Vec<StartedCall> = calls
                .iter()
                .filter(|call| call.turn_start.total_cmp(start).is_eq())
                .copied()
                .collect();
            let time = turn_time(&turn);
            let share = js_round(PERCENT * time.model_ms / (time.model_ms + time.tool_ms).max(1.0));

            // Epoch milliseconds are whole numbers far below 2^53.
            #[allow(clippy::cast_possible_truncation)]
            let start_ms = *start as i64;
            #[allow(clippy::cast_possible_truncation)]
            let chars = time.chars_written as i64;

            vec![
                iso_minute(start_ms),
                time.calls.to_string(),
                format_duration(time.tool_ms),
                format_duration(time.model_ms),
                format!("{share} %"),
                grouped(chars),
            ]
        })
        .collect();

    markdown_table(
        &[
            "Turn (UTC)",
            "Calls",
            "Tools ran",
            "Model between calls",
            "Model share",
            "Characters written",
        ],
        &rows,
    )
}
