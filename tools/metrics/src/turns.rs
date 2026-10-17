//! Turns of work kept for later comparison (`docs/tooling/metrics.md`): each session report of the
//! current turn is saved as one line of `.git/metrics/turns.jsonl`, and the metrics report lists
//! them all, so that a retrospective compares the turns of an epic.

use hooks::json::quote;

use crate::session::SessionFigures;

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
