//! The run's marker, `.git/metrics/run.json` (EN-092, research note 0024): when the run started,
//! when it ended (0 while it is open), and the sessions whose transcripts it counts. A compaction
//! keeps a session's id, so one id covers a run across compactions; a run continued in a new
//! session adds that session's id.

use hooks::json::{Value, parse, quote};

/// A run, as its marker holds it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Run {
    /// When the run started, epoch milliseconds.
    pub start: i64,
    /// When the run ended, epoch milliseconds; 0 while it is open.
    pub end: i64,
    /// The sessions it counts, in the order they joined.
    pub sessions: Vec<String>,
}

impl Run {
    /// Whether the run is still going on.
    #[must_use]
    pub fn is_open(&self) -> bool {
        self.end == 0
    }

    /// The run's length: to its end, or to `now` while it is open.
    #[must_use]
    pub fn duration_ms(&self, now: i64) -> i64 {
        (if self.is_open() { now } else { self.end }) - self.start
    }
}

/// A whole number written by this tool, 0 for anything else.
#[must_use]
pub fn whole(value: &Value) -> i64 {
    match value {
        // Times, offsets and token counts are whole numbers far below 2^53.
        #[allow(clippy::cast_possible_truncation)]
        Value::Number(number) => *number as i64,
        _ => 0,
    }
}

/// Reads a marker, `None` when the text is not one (no start).
#[must_use]
pub fn parse_run(text: &str) -> Option<Run> {
    let json = parse(text)?;
    let start = whole(json.get("start"));

    (start > 0).then(|| Run {
        start,
        end: whole(json.get("end")),
        sessions: json
            .get("sessions")
            .items()
            .iter()
            .map(|session| session.text().to_owned())
            .filter(|session| !session.is_empty())
            .collect(),
    })
}

/// Writes a marker as JSON, one line.
#[must_use]
pub fn render_run(run: &Run) -> String {
    let sessions: Vec<String> = run.sessions.iter().map(|session| quote(session)).collect();

    format!(
        "{{\"start\":{},\"end\":{},\"sessions\":[{}]}}\n",
        run.start,
        run.end,
        sessions.join(",")
    )
}

/// The marker once `/run` starts in `session` at `now`: a new run when none is open, else the open
/// run with the session added (a run continued in a new session).
#[must_use]
pub fn opened(previous: Option<&Run>, session: &str, now: i64) -> Run {
    match previous {
        Some(run) if run.is_open() => {
            let mut sessions = run.sessions.clone();

            if !sessions.iter().any(|known| known == session) {
                sessions.push(session.to_owned());
            }

            Run {
                sessions,
                ..run.clone()
            }
        }
        _ => Run {
            start: now,
            end: 0,
            sessions: vec![session.to_owned()],
        },
    }
}

/// The marker once the run ends at `now`; a run already ended keeps its end.
#[must_use]
pub fn closed(run: &Run, now: i64) -> Run {
    Run {
        end: if run.is_open() { now } else { run.end },
        ..run.clone()
    }
}
