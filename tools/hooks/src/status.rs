//! The status line of Claude Code (`REF-CC-STATUSLINE`): only whether the agent works — three dots
//! that move at each refresh while it thinks or a tool runs — or `chat free` once it ended its turn,
//! background tasks or not (decided 2026-10-25, AUD-051). Also the session's snapshot that each tool
//! call's record copies (`now.json`, read by the `log-tool-call` hook).

use calendar::SECOND_MS;

use crate::json::{number_text, quote};
use crate::tasks::Kind;
use crate::tokens::Tally;

/// ANSI colors of the status line.
const DIM: &str = "\u{1b}[2m";
const GREEN: &str = "\u{1b}[32m";
const RESET: &str = "\u{1b}[0m";

/// Frames of the working animation, one per second.
const FRAMES: i64 = 3;

/// The status line's text: `chat free` once the agent ended its turn, else `working` and one to
/// three dots, the count moving each second, padded so that the line keeps its width.
#[must_use]
pub fn status_text(tally: &Tally, now: i64) -> String {
    if tally.is_waiting {
        return format!("{DIM}chat free{RESET}");
    }

    let dots = usize::try_from(now.div_euclid(SECOND_MS).rem_euclid(FRAMES) + 1).unwrap_or(1);

    format!("{GREEN}working{:<3}{RESET}", ".".repeat(dots))
}

/// The session's state that each tool call's record copies (`now.json`, read by the
/// `log-tool-call` hook): written at each refresh, so that reading it costs the hook nothing.
#[must_use]
pub fn session_snapshot(tally: &Tally, context_percent: f64, model: &str, now: i64) -> String {
    let of = |kind: Kind| {
        tally
            .pending
            .iter()
            .filter(|task| task.kind == kind)
            .count()
    };

    format!(
        "{{\"contextPercent\":{},\"isWaiting\":{},\"model\":{},\"pendingAgents\":{},\"pendingCommands\":{},\"time\":{now},\"turnOutputTokens\":{},\"turnStart\":{},\"turnTokens\":{}}}",
        number_text(context_percent),
        tally.is_waiting,
        quote(model),
        of(Kind::Agent),
        of(Kind::Command),
        tally.output,
        tally.start,
        tally.total
    )
}
