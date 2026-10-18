//! Turns of work kept for later comparison (`docs/tooling/metrics.md`): each session report of the
//! current turn is saved as one line of `.git/metrics/turns.jsonl`, and the metrics report lists
//! them all, so that a retrospective compares the turns of an epic.

use std::collections::HashMap;

use hooks::json::quote;
use serde_json::Value;

use crate::format::{duration_text, grouped};
use crate::json::Json;
use crate::session::SessionFigures;
use crate::tables::{js_prefix, markdown_table};
use crate::transcript::upsert;
use calendar::iso_minute;

/// Characters of a session id shown.
const SESSION_SHOWN: usize = 8;

/// One turn of work, as saved.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TurnRecord {
    /// Active time, milliseconds.
    pub active_ms: i64,
    /// Tool calls.
    pub calls: i64,
    /// Commits made meanwhile.
    pub commits: i64,
    /// Tool calls that failed.
    pub failures: i64,
    /// Tokens the model wrote.
    pub output_tokens: i64,
    /// The session's id.
    pub session: String,
    /// When the turn started, epoch milliseconds.
    pub start: i64,
    /// Tokens of every kind.
    pub tokens: i64,
    /// From its first moment to its last, milliseconds.
    pub wall_ms: i64,
}

/// The record of a turn from its figures.
#[must_use]
pub fn turn_record(figures: &SessionFigures, start: i64, session: &str) -> TurnRecord {
    TurnRecord {
        active_ms: figures.active_ms,
        calls: i64::try_from(figures.calls).unwrap_or(i64::MAX),
        commits: i64::try_from(figures.commits.len()).unwrap_or(i64::MAX),
        failures: i64::try_from(figures.failures).unwrap_or(i64::MAX),
        output_tokens: figures.tokens.output,
        session: session.to_owned(),
        start,
        tokens: figures.tokens.total(),
        wall_ms: figures.wall_ms,
    }
}

/// The line saved for a turn, its keys in the order the TypeScript report wrote them.
#[must_use]
pub fn render_turn(turn: &TurnRecord) -> String {
    format!(
        "{{\"activeMs\":{},\"calls\":{},\"commits\":{},\"failures\":{},\"outputTokens\":{},\"session\":{},\"start\":{},\"tokens\":{},\"wallMs\":{}}}\n",
        turn.active_ms,
        turn.calls,
        turn.commits,
        turn.failures,
        turn.output_tokens,
        quote(&turn.session),
        turn.start,
        turn.tokens,
        turn.wall_ms
    )
}

/// A saved turn read back, a missing number read as 0 and a missing session as empty.
#[must_use]
pub fn read_turn(record: &Value) -> TurnRecord {
    TurnRecord {
        active_ms: record.at("activeMs").whole(),
        calls: record.at("calls").whole(),
        commits: record.at("commits").whole(),
        failures: record.at("failures").whole(),
        output_tokens: record.at("outputTokens").whole(),
        session: record.at("session").text().to_owned(),
        start: record.at("start").whole(),
        tokens: record.at("tokens").whole(),
        wall_ms: record.at("wallMs").whole(),
    }
}

/// The table of the turns saved, oldest first; a turn saved twice keeps its last figures where
/// it first came: start, session, duration, active time, tokens, written, calls, failed, commits.
#[must_use]
pub fn turns_table(records: &[TurnRecord]) -> String {
    let mut places: HashMap<String, usize> = HashMap::new();
    let mut turns: Vec<&TurnRecord> = Vec::new();

    for record in records {
        let key = format!("{} {}", record.session, record.start);

        upsert(&mut turns, &mut places, &key, record);
    }
    turns.sort_by_key(|turn| turn.start);

    markdown_table(
        &[
            "Started (UTC)",
            "Session",
            "Duration",
            "Active",
            "Tokens",
            "Written",
            "Calls",
            "Failed",
            "Commits",
        ],
        &turns
            .iter()
            .map(|turn| {
                vec![
                    iso_minute(turn.start),
                    js_prefix(&turn.session, SESSION_SHOWN),
                    duration_text(turn.wall_ms),
                    duration_text(turn.active_ms),
                    grouped(turn.tokens),
                    grouped(turn.output_tokens),
                    turn.calls.to_string(),
                    turn.failures.to_string(),
                    turn.commits.to_string(),
                ]
            })
            .collect::<Vec<_>>(),
    )
}
