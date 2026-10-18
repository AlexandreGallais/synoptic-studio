//! The watchdog of a background task (`docs/tooling/metrics.md`, Watching a long task): run under
//! Claude Code's Monitor, it follows the task's output file and prints a line only when the agent
//! must look — the output stalled, came back, matched the end pattern, or the watch ran out of
//! time — so that a stuck task is seen within minutes instead of waited on. Its lines are those of
//! `watch-task.ts` (EN-107).

use std::io::Write;

use crate::format::{js_round, to_fixed};
use crate::transcript::is_js_space;

/// What the watchdog knows of the output file.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Watch {
    /// When the output last changed, epoch milliseconds.
    pub changed_at: i64,
    /// Whether a stall was already reported.
    pub is_stalled: bool,
    /// Size of the output, bytes; -1 before the first look.
    pub size: i64,
}

/// What to look for.
#[derive(Clone, Debug, PartialEq)]
pub struct WatchRules {
    /// Output silent this long, milliseconds: a stall.
    pub stall_ms: f64,
    /// A line starting with it ends the watch: plain text, not a pattern; empty for none.
    pub until: String,
}

/// A step of the watch: the new state, and the line to print, possibly empty.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WatchStep {
    /// The watch ends: the end pattern matched, or the task exited.
    pub is_done: bool,
    /// The line to print, empty for none.
    pub message: String,
    /// The state after this look.
    pub watch: Watch,
}

/// Milliseconds in a second.
const SECOND_MS: f64 = 1000.0;

/// The last line of a text that is not blank, trimmed; empty for none.
#[must_use]
pub fn last_line(text: &str) -> String {
    text.split('\n')
        .map(|line| line.trim_matches(is_js_space))
        .rfind(|line| !line.is_empty())
        .unwrap_or_default()
        .to_owned()
}

/// Whether a line is the one Claude Code appends to a background task's output when the task
/// ends: `[exited with code 0]`.
fn is_exit_line(line: &str) -> bool {
    line.strip_prefix("[exited with code ")
        .and_then(|rest| rest.strip_suffix(']'))
        .is_some_and(|code| !code.is_empty() && code.bytes().all(|byte| byte.is_ascii_digit()))
}

/// The step after the output changed: done when a new line matches the end pattern or says the
/// task exited, back after a stall.
fn changed_step(watch: Watch, now: i64, text: &str, until: &str) -> WatchStep {
    let from = usize::try_from(watch.size.max(0)).unwrap_or(0);
    let added =
        String::from_utf8_lossy(text.as_bytes().get(from..).unwrap_or_default()).into_owned();
    let end = added
        .split('\n')
        .find(|line| (!until.is_empty() && line.starts_with(until)) || is_exit_line(line))
        .unwrap_or_default();
    let is_done = !end.is_empty();
    let message = if is_done {
        format!("done: {end}")
    } else if watch.is_stalled {
        format!("resumed: {}", last_line(text))
    } else {
        String::new()
    };

    WatchStep {
        is_done,
        message,
        watch: Watch {
            changed_at: now,
            is_stalled: false,
            size: i64::try_from(text.len()).unwrap_or(i64::MAX),
        },
    }
}

/// One look at the output, at `now`.
#[must_use]
pub fn watch_step(watch: Watch, now: i64, text: &str, rules: &WatchRules) -> WatchStep {
    if i64::try_from(text.len()).unwrap_or(i64::MAX) != watch.size {
        return changed_step(watch, now, text, &rules.until);
    }

    // Times far below 2^53 milliseconds.
    #[allow(clippy::cast_precision_loss)]
    let silent_ms = (now - watch.changed_at) as f64;
    let is_new_stall = !watch.is_stalled && silent_ms >= rules.stall_ms;

    WatchStep {
        is_done: false,
        message: if is_new_stall {
            format!(
                "stalled: no output for {} s — last line: {}",
                to_fixed(js_round(silent_ms / SECOND_MS), 0),
                last_line(text)
            )
        } else {
            String::new()
        },
        watch: Watch {
            is_stalled: watch.is_stalled || is_new_stall,
            ..watch
        },
    }
}

/// What the watch reads and waits with: the clock, the output file, the pause between two looks.
pub trait Watching {
    /// Epoch milliseconds now.
    fn now(&mut self) -> i64;
    /// The output so far, empty while the file does not exist.
    fn read(&mut self) -> String;
    /// Waits until the next look.
    fn pause(&mut self);
}

/// Watches until the output ends or `end_at` passes, one look after each pause, writing each line
/// to print as soon as it is known; `watch over: <last line>` when the time ran out.
///
/// # Errors
///
/// The lines cannot be written.
pub fn watch_until(
    watching: &mut impl Watching,
    rules: &WatchRules,
    end_at: f64,
    out: &mut impl Write,
) -> std::io::Result<()> {
    let mut watch = Watch {
        changed_at: watching.now(),
        is_stalled: false,
        size: -1,
    };

    loop {
        watching.pause();

        let now = watching.now();
        let text = watching.read();
        let step = watch_step(watch, now, &text, rules);
        // Times far below 2^53 milliseconds; a time limit not a number never passes.
        #[allow(clippy::cast_precision_loss)]
        let is_over = now as f64 >= end_at && !step.is_done;

        watch = step.watch;
        if !step.message.is_empty() {
            writeln!(out, "{}", step.message)?;
        }
        if is_over {
            writeln!(out, "watch over: {}", last_line(&text))?;
        }
        out.flush()?;
        if is_over || step.is_done {
            return Ok(());
        }
    }
}

/// The value after an option, or the fallback when the option is absent or ends the line.
#[must_use]
pub fn option<'a>(options: &'a [String], name: &str, fallback: &'a str) -> &'a str {
    options
        .iter()
        .position(|option| option == name)
        .map_or(fallback, |at| {
            options.get(at + 1).map_or(fallback, String::as_str)
        })
}

/// A number of seconds as JavaScript's `Number` reads it, in milliseconds: not a number for a
/// text that is not one, so that `--stall x` never reports a stall.
#[must_use]
pub fn seconds_ms(text: &str) -> f64 {
    let trimmed = text.trim();

    if trimmed.is_empty() {
        return 0.0;
    }
    trimmed
        .parse::<f64>()
        .map_or(f64::NAN, |seconds| seconds * SECOND_MS)
}
